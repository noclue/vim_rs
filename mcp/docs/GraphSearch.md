# GraphSearch — enhancing the vim_rs MCP server with GraphRAG-style retrieval

> **Status:** Working draft / thinking document. Not a spec, not a plan of record.
> The point of this file is to (a) write down what we actually have today, (b) name the
> gaps honestly, and (c) spend most of our energy on the *open questions* we need to
> answer before we commit to a data model. Design proposals here are speculation and
> are labelled as such. Do not treat any section below "Design speculations" as decided.

Reference for the technique family: <https://graphrag.com/> — the general idea being to
combine a knowledge graph (entities + typed relationships) with vector and lexical
retrieval, plus graph traversal / community summarization, instead of relying on vector
similarity alone.

---

## 1. Goal (restated in our terms)

We want an LLM agent to reliably answer questions of the shape:

1. **"Where in the API is X?"** — find the relevant managed object / method / struct /
   field / enum for a concept ("EVC mode", "NIC MAC address", "datastore free space").
2. **"How do I get to X?"** — given a target type or field, what are the concrete access
   routes: which property accessor, which method return value, which method parameter,
   and what is the field path (including downcasts) from an entry point?
3. **"How do I query X generically?"** — express the access as a `PropertyCollector`
   spec / `vim_retrievable!` path when that is the right tool, vs. calling a specific
   method.

Today (1) is served by semantic search, (2) partially by precomputed truncated paths,
and (3) by the property-tree tools (which walk scalar/struct reads but stop at array and
polymorphic boundaries). The three are not unified and the underlying graph is not
queryable.

---

## 2. What we have today (findings from the code)

### 2.1 The MCP server surface (`mcp/server/src/main.rs`)

Seven tools:

| Tool | Backing data | Notes |
|------|--------------|-------|
| `get_starter_guide` | embedded `STARTER_GUIDE` string | static |
| `search` | embeddings (BGE-small-en-v1.5, 384-dim) | **semantic only**, brute-force dot product over all items via Rayon |
| `get` | `ApiDatabase.items` (IndexMap by ID) | exact-ID lookup; substring fallback only for "did you mean" |
| `list_property_collector_root_types` | hard-coded 15 inventory types | see `property_collector::get_managed_object_types` |
| `get_property_path` | `field_data.rs` PHF map | one path level + children |
| `get_property_tree` | `field_data.rs` PHF map | flat dotted paths, depth 1–5, optional regex filter |

Observations:
- `search` is the only discovery entry point and it is **purely vector**. There is no
  lexical / exact / BM25 / FTS path. `get`'s substring match is a fallback for typos,
  not a search tool.
- Filtering in `search` is post-hoc by item type only. No filter by managed object,
  subsystem, "is this reachable via PropertyCollector", etc.
- The property tools and the search tools draw on **two different data
  representations** that were built by different generators (see 2.3 and 2.4).

### 2.2 The item model (`mcp/api_database/src/lib.rs`)

- `ApiDatabase { items: IndexMap<String, ApiItemEntry>, embeddings: Option<Vec<Vec<f32>>> }`.
- `embeddings[i]` aligns positionally with `items[i]`. Serialized with `bincode` into
  `mcp/data/api_database.bin` (**~76 MB** on disk — dominated by the denormalized
  markdown docs stored per item plus the float vectors), embedded into the binary.
- Item variants: `ManagedObject`, `Method`, `Structure`, `Field`, `Enum`, `Trait`,
  `Example`. Methods and fields are **exploded into their own top-level items** (id
  like `VirtualMachine::power_on_vm_task`, `VirtualHardware::device`) so a single field
  is both an item and part of its struct's `fields`.
- Each item knows how to render three things: `embedding_text()`, `search_summary()`,
  `detailed_document()`. This is nice: the "what gets embedded" and "what gets shown"
  are explicit and per-type.

Graph-ish data already carried on items:
- `StructureEntry`: `parent`, `children`, `all_descendants`, `inheritance_chain`,
  `implements_traits`, `related_types`, and **`paths: Vec<ApiTypePath>`**.
- `TraitEntry`: `parent_trait`, `implementing_types`, `all_descendants`, `deref_target`.
- `FieldItemData`: carries a copy of the parent struct's `paths` (to answer "how do I
  reach this field").

So a lot of graph edges are *present as denormalized lists on nodes* — but there is no
edge index, and nothing lets you traverse or query them at runtime beyond what a given
item already stores.

### 2.3 The path system (`vim_build/src/vim_model/paths.rs`)

This is the closest thing we have to graph reachability, and it's the thing the user
flagged as "a fixed set of paths… incomplete… truncated… not easy to query." Confirmed:

- `compute_paths` does a BFS from every managed object's methods:
  - `GET` method → `PropertyAccessor` origin (property read).
  - `POST` method output → `MethodOutput` origin.
  - `POST` request-type fields → `MethodInput` origin (one per parameter).
- From each origin it walks struct fields recursively, recording a `TypePath`
  (`origin` + `Vec<PathStep>`), where a step is `Field { name, optional, boxed, array }`
  or `Downcast { to_type, is_trait_cast }`.
- Polymorphism: when a struct has children, it emits `Downcast` steps to **all**
  descendants (not just direct children), except a blocklist (`SKIP_DESCENDANTS_TYPES`:
  `Any`, `DataObject`, `MethodFault`, `Issue`, …) to avoid combinatorial blowup.
- Limits: `max_depth = 10`; during collection `max_paths_per_type * 10` (=100) paths are
  kept per type; cycles cut by a `visited` set.
- Paths are stored **on the destination type** ("here are ways to reach me"). Then at
  render time `select_paths_for_display` sorts (properties before methods, inventory
  priority, then depth), caps at ~3–10, and at most 5 per managed object.

Key structural facts that matter for redesign:
- Paths are **materialized and truncated**, computed once at generation time. There is
  no "path from A to B" query, no "who references type T", no "shortest access route
  under constraint (e.g. PropertyCollector-only, no method calls)".
- The **selection heuristic is baked in** (inventory priority list, properties-first).
  Good default, but the agent can't ask for other orderings or the full set.
- The BFS is **origin-rooted at managed objects**, so a type only gets paths if it is
  reachable from an MO within depth 10 and before the per-type cap fills. Types that are
  only reachable deep, or only via skipped polymorphic bases, may have poor or no paths.
- `property_collector_path()` already knows how to project a path down to a
  PropertyCollector-legal dotted string (drops downcasts, snake_cases) — so the mapping
  from "access path" to "PropertyCollector spec" partially exists.

### 2.4 The property-navigation graph (`mcp/server/src/field_data.rs` + `resolver.rs`)

- `field_data.rs` is a **generated PHF map**: `type_name → { field_name → NodeData }`,
  13.6k lines. `NodeData` carries `path_segment` (camelCase VIM name), `type_decl`,
  `type_name` (for recursion), `is_optional`, `processing_type` (Struct/Trait/Enum), `doc`.
- This is effectively a **real, traversable adjacency list** of the type→field→type
  graph, and it's what powers `get_property_tree` / `get_property_path`.
- **How the ~234 entries are chosen** (corrected — the generator is
  `vim_build/src/rs_emitter/field_data.rs`): seed with the **16 inventory managed
  objects**' GET property accessors, then `add_referenced_structs` walks the **transitive
  closure** of all referenced struct types. For each struct it flattens the **entire
  inheritance chain** (own fields + all ancestor fields) into one flat field map — so
  inheritance is already resolved away here. The 234 is therefore *not* an arbitrary
  cap; it is the natural size of the **property-reachable, non-array** data-type closure.
- **The real cutoff is arrays, not count.** `extract_struct_references` deliberately does
  **not** recurse into array types (`// Not recursively resolving array types`). Arrays
  surface as leaf `ArrayOf*` nodes with `processing_type = Enum("ArrayOf…")`, navigable
  only via `.length`. Consequence: you **cannot navigate into the fields of array
  elements** through this tool — e.g. `config.hardware.device` resolves to
  `ArrayOfVirtualDevice` and stops; you can't walk `…device[*].backing`. A large fraction
  of interesting data (devices, disks, NICs, VMs on a host, datastore hosts) lives behind
  arrays and is invisible to `get_property_tree`.
- **This is why `paths.rs` and `field_data.rs` are complementary, not redundant** (I
  earlier framed them only as "two representations that can drift" — that's incomplete):
  - `field_data` = interactive adjacency for **scalar/struct property reads that stop at
    array and polymorphic-leaf boundaries**; non-truncated within that scope.
  - `paths` = precomputed reachability that **does** cross arrays and emits **downcast**
    steps into descendants; truncated by a display heuristic.
  They cover different edges of the same graph. A unified edge store (see §6) should
  subsume both, but we must preserve *both behaviors*: array-crossing + downcasts (paths)
  and clean scalar tree-walking (field_data).

### 2.5 Scale (verified)

- ~5330 generated structs; plus enums, ~185 managed objects, their methods, traits.
- Exploded into roughly ~5k `ApiItemEntry` items (managed objects + methods + structures
  + fields + enums + traits + examples). Each has one 384-dim embedding.
- `field_data` PHF: 234 types = the property-reachable, non-array data-type closure of
  the 16 inventory managed objects (inheritance pre-flattened). Not an arbitrary cap.
- `api_database.bin`: ~76 MB.

**On synthetic `ArrayOf*` types (per Kiril — why they exist):** these are a
**serialization artifact**, not a data model concept. PropertyCollector (and a few similar
APIs) return values addressed by a property path, and a path can resolve to an array
(e.g. `config.hardware.device`). Because the wire formats — XML, and the JSON later
derived from it — cannot dynamically tag an element as "array of X," they need a
**named** array type in the schema. So the API defines synthetic `ArrayOfX` types, and
the generator emits one for **every** data type defensively (it can't know which types
will be fetched as an array through an open slot). Key points:
- They surface **only in open/`Any`-typed slots** (`VimAny`), i.e. PropertyCollector-style
  responses. In a statically-typed struct field, an array is just `Vec<T>` — the field is
  flagged as an array, no wrapper.
- So the ~5330 struct count is inflated by wrappers that are **not real, independently
  reachable API nodes**. `ArrayOf*` should **not** be search-indexed and should **not** be
  graph nodes.
- But the name *is* useful runtime knowledge for the "how do I query this generically"
  answer: "fetching path P returns `ArrayOfX`, which vim_rs deserializes to `Vec<…X…>`."
  Model it as an annotation on the *access answer*, not as an entity. See §7.5.

---

## 3. The graph topology, made explicit

If we were to draw the API as a property/knowledge graph, the node and edge inventory is
already fully determined by the vim_build model — it's just not *stored* as a graph. This
is the raw material.

**Node types:** `ManagedObject`, `Method`, `Field`, `Struct`, `Trait`, `Enum`, `Example`
(and arguably `Package`/subsystem groupings, which don't exist yet).

**Edge types (all derivable from existing data):**

| Edge | Source in code | Currently stored as |
|------|----------------|---------------------|
| MO → Method (has method) | `ManagedObjectEntry.methods` | list on MO |
| Method → Type (returns) | `method.output` | only inside materialized `paths` |
| Method → Type (param takes) | request-type fields | only inside materialized `paths` |
| MO → Type (property accessor GET) | GET methods | `paths` + `field_data` (inventory only) |
| Struct → Field → Type (has field, typed as) | `struct.fields` | `field_data` PHF (234 types) + `related_types` |
| Struct → Struct (extends / parent) | `parent` | list on node |
| Struct → Struct (child / descendant) | `children`, `all_descendants` | list on node |
| Struct → Trait (implements) | `implements_traits` | list on node |
| Trait → Struct (deref target / implementing types) | `deref_target`, `implementing_types` | list on node |
| Field → Enum (typed as enum) | field type | implicit |
| Example → Type/Method (mentions/uses) | example source code | **not extracted at all** |

Two things jump out:
1. The edges exist but live in **three disconnected forms**: denormalized lists on
   nodes, the materialized `paths`, and the `field_data` PHF. Nothing reconciles them.
2. **Examples are not linked into the graph** — we have 18 real runnable snippets
   (`examples/snippets/src/*.rs`) that demonstrate access patterns, but no edges from an
   example to the types/methods it exercises. For "how do I actually do this," that's
   arguably the highest-value edge set and it's missing.

---

## 4. Gap analysis vs. the goals

1. **No lexical / full-text search.** Concepts that are named exactly ("EVC", "vMotion",
   "NUMA", a specific enum variant) can be *worse* under pure vector search than under
   BM25. We have zero exact-token recall. This is the single most obvious, lowest-risk
   gap.
2. **Paths are frozen and lossy.** Truncated to a heuristic top-N, computed only from
   MO roots, not queryable ("A→B?", "reach T without calling a method"), and the
   selection policy can't be steered by the agent.
3. **Property navigation stops at array boundaries.** Not a coverage-count problem — the
   234-type closure is the *right* set for non-array reads — but you cannot walk into
   array-element fields (`…device[*].backing`) or across downcasts with the interactive
   tools. Much high-value data (devices/disks/NICs, child inventory objects) sits behind
   arrays and is only reachable via the frozen `paths`, not the live tree tools.
4. **No neighborhood / expansion retrieval.** Search returns isolated items. There is no
   "give me this method plus its parameter types plus their key fields plus one example"
   in a single hop — which is exactly what an agent needs to write correct code.
5. **No subsystem / community structure.** No notion of "networking", "storage",
   "guest customization" clusters to support coarse-to-fine navigation or global
   ("what are the major areas of this API") questions.
6. **Two complementary views, no single edge store.** `paths` (array-crossing +
   downcasts, truncated) and `field_data` (non-array scalar tree-walk, complete) cover
   *different edges* of the same graph and are both legitimate. The gap is that there is
   no unified edge store they are both views of — so their overlap can drift and neither
   alone answers arbitrary graph queries.

---

## 5. How GraphRAG maps onto this problem (and where it doesn't)

Worth being sober here: classic GraphRAG's headline move is *LLM-extracted* entities and
relationships from unstructured text, plus community detection and LLM-generated
community summaries. **Our graph is not extracted — it's already exact and typed**
(it's a machine-generated API model). So the parts of GraphRAG that transfer well vs.
poorly:

**Transfers well:**
- **Hybrid retrieval** (vector + lexical + graph) as the query pipeline.
- **Local search**: seed with vector/lexical hits, then expand N hops along typed edges
  to assemble a context subgraph.
- **Path / reachability queries** over an explicit edge index.
- **Community summaries** *if* we can define good communities (subsystems). These could
  even be LLM-authored once, offline, and cached — a legitimately GraphRAG-flavored idea.

**Transfers poorly / unnecessary:**
- LLM entity extraction — we don't need it; the schema gives us entities for free. Don't
  spend effort re-deriving what vim_build already knows.
- Fuzzy "relationship strength" scoring — edges here are exact and typed; invented
  weights would be noise unless grounded in something real (e.g. usage frequency in
  examples/docs).

The honest framing: **this is less "RAG over a knowledge graph we must build" and more
"expose the API graph we already have as a first-class, queryable index, and add the
retrieval modes (lexical, neighborhood, path) that a graph enables."** Keeping that framing
prevents us from over-engineering an LLM-extraction pipeline we don't need.

---

## 6. Design speculations (NOT decided — options to react to)

These are deliberately presented as alternatives, not a chosen path.

### 6.1 Core tech: storage & indexing

- **Option A — Explicit edge store.** Build a real graph at generation time: a node
  table (id, kind, name) + an edge table (src, dst, edge_kind, attrs like optional/array/
  boxed/downcast). Ship it alongside the item DB. Traversal at runtime replaces the
  frozen `paths`. Paths become a *query* (BFS/Dijkstra with edge-kind constraints and a
  cost model), not a stored artifact. This directly answers "A→B", "who references T",
  "PropertyCollector-only route". Reconciles §2.3 and §2.4 into one structure — the
  `field_data` PHF becomes just one view over the edge store.
- **Option B — Keep materialized paths but make them complete + queryable.** Store all
  paths (not truncated), indexed both by destination and by origin, with a small query
  layer for filtering/sorting on the fly. Cheaper to build than a general graph engine,
  but doesn't answer arbitrary A→B and keeps the two-representation problem.
- **Option C — Embed an actual graph/kv engine** (e.g. an in-process indexed structure,
  or SQLite with recursive CTEs). Pro: mature query semantics, FTS5 gives us BM25 for
  free in the same store. Con: dependency + build weight; need to check binary-embedding
  story since today everything is compiled in.
- **Lexical index (orthogonal, wanted under all options):** add BM25/FTS over
  `embedding_text` (or a richer lexical doc). Either SQLite FTS5 (if we go Option C) or a
  small pure-Rust inverted index (tantivy is heavy; a hand-rolled BM25 over ~5k docs is
  trivial and fast). Then `search` becomes **hybrid**: reciprocal-rank-fusion of vector
  + BM25.

### 6.2 Core tech: retrieval modes

- **Hybrid ranking**: RRF or weighted fusion of {vector, BM25, exact-name boost}.
- **Neighborhood assembly**: after ranking, expand each top hit 1–2 hops (method→param
  types→their required fields; struct→reachability path→example) and return a compact
  *context pack* rather than isolated cards.
- **Path query with a cost model**: prefer property accessors over methods, shorter over
  longer, non-optional over optional, avoid downcasts through skipped bases; let the
  caller override weights or constrain edge kinds (e.g. "read-only / PropertyCollector").

### 6.3 Tools bestowed to the agent (speculative)

- `search(query, mode = hybrid|semantic|lexical, filter = {kind, managed_object, subsystem, reachable_via})` — extend the existing tool rather than add many.
- `find_paths(from, to, constraints)` — reachability with edge-kind/optionality/downcast
  constraints and a `style` (method-call vs PropertyCollector vs `vim_retrievable!`).
- `neighbors(id, edge_kinds, hops)` — raw graph expansion for an agent that wants to
  "look around" a node.
- `explain_access(target)` — the assembled context pack: best route + required
  construction fields + a matching example, in one call. This is the "make the agent
  correct on the first try" tool.
- Extend property navigation to **cross array boundaries and downcasts** (the thing
  `field_data` deliberately stops at), so the interactive tree tools reach the data that
  today only the frozen `paths` expose.

---

## 7. OPEN QUESTIONS (spend the most time here)

These are the questions I think we must answer *before* modelling. Grouped by theme.
None of these are rhetorical — several genuinely change the data model.

### 7.1 What questions is the agent actually asking? (retrieval intent)
- What is the real distribution of agent queries — concept lookup, "how to reach X",
  "how to construct X", "how to query X generically", or "explain this type I already
  found"? We are guessing. **No telemetry exists today (confirmed).** The plan (§7.9) is
  to *generate* this data: log all tool calls during eval runs, then classify/cluster to
  learn the distribution empirically rather than guessing.
- For "how do I do X", is the winning artifact usually a **method**, a **path**, or an
  **example**? If it's overwhelmingly examples, the highest-ROI work is
  extracting example→API edges, not building a graph engine.
- When the agent gets a search result, what does it do next — `get` the item, or ask for
  a path? The current two-step (`search` then `get`) — is the friction there, or in the
  quality of the first hit?

### 7.2 What does "relevant bit of the API" mean at retrieval time?
- Is the unit of retrieval the **item** (as today) or the **subgraph** (a type plus its
  access route plus construction requirements)? If the latter, the whole "return isolated
  cards" model is wrong and we should design around context packs from day one.
- Should reachability/paths be part of *ranking* (a type reachable from `VirtualMachine`
  in 2 hops beats an identical-looking one buried 8 hops deep) or only part of the
  *answer*? I.e., is graph distance a retrieval signal or just presentation?
  **→ Decided (Kiril): shorter paths score higher; include path length as a ranking
  signal.** Remaining question is only the weight relative to vector/lexical score and
  whether other path costs (optionality, downcasts, method-vs-property) fold into the same
  cost — settle empirically via §7.9.

### 7.3 Where should the graph live and be computed?
- **vim_build (generation time) vs. the MCP data_transformer vs. the server (runtime)?**
  The edges are all knowable at generation time and vim_build already computes most of
  them. Argument for vim_build: single source of truth, the `paths` logic already lives
  there. Argument for data_transformer: keeps vim_build focused on emitting Rust and lets
  the MCP evolve its index independently. **Which project owns the edge store?**
- Do we keep computing `paths` in vim_build at all, or does vim_build export a raw edge
  list and the MCP side derives paths/queries? (I lean toward: vim_build exports edges,
  MCP owns query — but that's a real architectural call.)

### 7.4 Materialize vs. traverse-at-query-time?
- ~5330 nodes and maybe tens of thousands of edges is *tiny*. Runtime BFS is
  microseconds. So the "truncate to 10" limitation is **self-imposed by the materialized
  design, not by scale.** Given that — do we even want stored paths anymore, or is the
  answer simply "store the edge list, traverse on demand"?
- If we traverse at query time, what's the **cost model / stopping rule** so we don't
  return 400 near-identical downcast paths? (This is where the current
  `select_paths_for_display` heuristic knowledge should migrate to, but as a
  *ranking function over live results*, not a truncation.)

### 7.5 How do we handle polymorphism and arrays without exploding?
- **Synthetic `ArrayOf*` types (per Kiril):** serialization artifacts for open/`Any`
  (PropertyCollector) slots — see §2.5. Not real nodes. Decision seems clear: **exclude
  from the node/search set**; surface the `ArrayOfX` name only as an annotation on a
  generic-query access answer ("this path returns `ArrayOfX` → `Vec<…>`"). Remaining
  question is purely plumbing: where in the pipeline do we drop them, and do we keep a
  path→`ArrayOfX` lookup for the generic-query answer style?
- **Crossing arrays in navigation:** if we let the interactive tools walk into
  `field[*].subfield`, what's the syntax and the depth/cost budget so we don't
  re-introduce the blowup `field_data` avoided by stopping at arrays?
- The descendant-downcast expansion is the main source of path blowup, hence
  `SKIP_DESCENDANTS_TYPES`. In a live-query world, do we (a) never auto-expand
  descendants and instead offer a `downcast_options(type)` tool, (b) expand lazily only
  when the target is a specific subtype, or (c) keep an expansion budget? The blocklist
  approach (`Any`/`DataObject`/`MethodFault`/`Issue`) is a heuristic — is it still the
  right cut once paths are queries?
- `MethodFault` and `Event` subtypes aren't generated as Rust types (they use
  `type_` + `extra_fields_`). How should those appear in the graph — as real nodes with
  a "not a Rust struct, access via extra_fields_" annotation? The item model already has
  the `Skip`/`Prune` machinery; the graph needs to mirror it or agents will be told to
  use types that don't exist in Rust.

### 7.6 Lexical search — how much, and fused how?
- Is a hand-rolled BM25 over ~5k docs enough, or do we want a real FTS engine (pulls in
  SQLite/tantivy and complicates the "everything embedded in one binary" story)? What's
  our tolerance for binary size / build-time growth here?
- What's the **lexical document** per item? Just `embedding_text`, or a richer field set
  (type name tokens split on camelCase, enum variant names, VIM camelCase names, doc
  text)? Tokenization of `VirtualMachineConfigInfo` → `virtual machine config info`
  matters a lot for recall.
- Fusion: simple RRF, or learned/weighted? Do we have any way to **evaluate** ranking
  quality (see 7.9) before we tune weights?

### 7.7 Subsystems / communities — do we need them, and where from?
- Is there value in coarse "subsystem" grouping (networking, storage, HA/DRS, guest
  customization) for navigation and for "global" questions? If yes, where do the
  groupings come from — the OpenAPI spec's tags/namespaces, module structure, name
  prefixes, or clustering on embeddings? Each gives a different (and differently wrong)
  partition.
- Are LLM-authored, cached subsystem summaries (the genuinely GraphRAG bit) worth the
  offline cost, or is that gold-plating for an API where names are already descriptive?

### 7.8 Examples as first-class graph citizens
- **Corpus confirmed:** examples are harvested by `build_examples` (WalkDir) from
  `examples/snippets/src/*.rs` — **18 snippets** today (e.g. `vm_ip`, `mac_monitor`,
  `property_collector`, `perf_metrics`, `vm_events`, `retrieve_host_info`). Small but
  real and curated; the `vtui` app and the `lib.rs` connection helper are excluded.
- We have these runnable examples but **zero edges** from them to the API. How do we
  extract example→{method, type, field} edges — parse the Rust (`syn`), or grep for known
  identifiers from the item index? How reliable does it need to be for "here's a snippet
  that uses this method"?
- If examples are among the best "how to" signals, should an example be *rankable in the
  same search* as methods/types (it is today) **and** attached as the "proof" in
  path/access answers? (Currently examples live in the index but aren't linked to what
  they use.)
- With only 18 snippets, coverage is thin. Do we (a) grow the snippet corpus deliberately
  to cover the subsystems evals reveal as weak, (b) mine the VMware SDK samples (see 7.9)
  as *additional* example material, or (c) both? Note (b) is Java, so it's illustrative
  intent, not directly indexable Rust.

### 7.9 How do we know any of this is better? (evaluation) — **primary near-term focus**

We have **no query telemetry** (confirmed — none exists) and **no evaluation harness**.
Kiril's proposed approach, which I think is the right spine for this whole effort:

- **⚠ Baseline is a full coding harness *with source access*, not a bare model (per
  Kiril).** The realistic deployment is an agent (Pi / OpenCode+OpenRouter / Cursor
  Composer) that already has the vim_rs source (~731k lines) and native grep/file-read.
  So the A/B is **harness+source (no MCP)** vs. **harness+source+MCP** — the MCP must show
  lift *on top of ripgrep over the source*. This reframes the whole project's value:
  - **Generic search (Phase 2) is largely redundant** with the harness's native code
    search. Exact/lexical recall over API items is what grep already does. Residual value
    is only concept→name when the agent lacks the vocabulary. **Deprioritize.**
  - **What the harness *cannot* cheaply derive from 731k lines of generated code is the
    value**: reachability (A→B, PropertyCollector / `vim_retrievable!` paths, downcast
    routes), "how to construct X," and curated access patterns/examples. That's Phases
    3–4 and the starter guide. **This is where the MCP earns its keep.**
  - **Metric likely shifts from pass/fail to efficiency.** With a capable harness both
    arms may eventually compile *something*; the honest signal is **turns / tool-calls /
    tokens / wall-clock to a correct result**, plus quality (right type, no hallucinated
    method). Capture these, not just a boolean.
  - Consequence for harness choice: we need one that is **scriptable + telemetry-
    extractable** for automated sweeps (OpenCode / Pi lean open; Cursor Composer is closed
    and better for manual qualitative sessions than batch measurement).


- **Task-level eval, not retrieval-level.** Measure *did the MCP help the agent write
  correct vim_rs code*, A/B: same model, same coding problems, MCP on vs. off (and later,
  variant-of-MCP vs. variant). This sidesteps the fact that we have no gold retrieval
  labels, and it measures the thing we actually care about.
- **Problem source — two tiers, don't conflate them:**
  - *First slice (Phase 1, Event/MethodFault):* **~4 small hand-authored, fixture-backed
    tasks**, graded offline by compile **+ a runtime fixture assertion** (compile alone
    can't check `extra_fields_` string keys). This is the authoritative Phase-1 set — see
    **`EvalHarness.md`**. It is *not* the Java-SDK set below.
  - *Later phases (PropertyCollector, Metrics):* derive a broader ~20-problem set from
    VMware's SDK samples
    (<https://github.com/vmware/vcf-sdk-java/tree/main/vsphere-samples/src/main/java/com/vmware/sdk/samples>),
    **web-service (SOAP/VMODL) subset only** (maps to what vim_rs exposes; REST/vAPI out of
    scope). The Java sample states intent; the derived problem asks for the same in Rust and
    the Java source doubles as a rubric.
- **Explicitly out of scope for iteration 1: REST / vAPI samples.** vim_rs doesn't expose
  the REST/vAPI surface yet, and linking the index to those operations/objects is
  non-trivial. Planned, but later. Filter those samples out of the eval set.
- **Grading / success metric.** The metric that matters is *compiling, correct* vim_rs
  code — retrieval recall@k can diverge from it. Options to grade, cheapest first:
  (a) does it compile (`cargo build` the generated snippet against vim_rs)?
  (b) LLM-as-judge against the Java reference behavior; (c) if we can stand up a test
  vCenter, actually run it. Compilation is the cheap gate, but **for the Event/Fault slice
  compile-only under-measures** (wrong `extra_fields_` keys still compile), so Phase 1 adds
  a **runtime fixture assertion** tier — see `EvalHarness.md` §3. For later phases,
  compile + LLM-judge is the likely bar.
- **Eval model choice.** Kiril suggests a fixed, cheap model (e.g. DeepSeek v4 Flash) for
  the measurement sweeps. Requirements I'd put on whatever we pick: reliable **tool /
  function calling** (MCP is useless to a model that can't call tools well), low/zero
  temperature for reproducibility, and cheap enough to run the full matrix repeatedly.
  Caveat worth flagging: if the eval model is *too weak*, MCP-on and MCP-off both fail and
  we measure nothing; if *too strong*, it may already know vim_rs-shaped patterns and MCP
  shows no lift. Suggest a quick calibration pass — spot-check one stronger and one weaker
  model on a handful of tasks — to confirm the chosen model sits in the sensitive band
  before spending on the full sweep.
- **This is how we bootstrap the telemetry we don't have.** Every eval run *is* a source
  of real agent queries against the MCP. Log every tool call (query, filter, chosen
  result, follow-up `get`s) during eval runs, then classify/cluster them (§7.1) to learn
  the actual query distribution empirically. We are at the foot of the mountain — the eval
  harness is simultaneously our first measurement instrument and our first telemetry pipe.

Open sub-questions:
- How many derived problems is enough to see signal without huge cost — 20, 50, 100?
- How do we keep the problems from leaking the answer (don't hand the model the vim_rs
  method name in the prompt; describe the *goal*, like the Java sample's docstring)?
- Do we grade per-task pass/fail only, or also capture partial signals (compiles but
  wrong, wrong type chosen, hallucinated method) — the latter is far more diagnostic for
  improving retrieval.

### 7.10 Output shape & token budget
- Graph/neighborhood answers can be large. What's the token budget per tool response,
  and how do we compress a context pack (summaries + IDs to drill into) vs. dumping full
  docs? The current `detailed_document()` is already big per item.
- Should tools return **structured** results (JSON the agent can navigate) in addition to
  the markdown they emit today? A graph is easier to consume structured; markdown is
  easier to read. Which does the agent actually do better with?

### 7.11 Build / packaging constraints
- Everything is compiled into the binary today (76 MB DB already). What's the ceiling?
  An edge store + lexical index adds more. Does that change the "single static binary"
  distribution model, and is that acceptable?
- Regeneration cost: the embeddings step is the slow one. Adding graph/lexical indexes —
  do they fit in the same `data-transformer` run, and what's the incremental build time?

### 7.12 Value thesis — where is the MCP *genuinely irreplaceable*? (per Kiril)

This is the most important framing question and it should drive scope. The realistic
consumer is **not** working in the vim-rs monorepo; they consume `vim_rs` as a **Cargo
dependency** in a vtui-like app (confirmed: `vtui/Cargo.toml` →
`vim_rs = { version = "0.5", … }`, and vtui uses `vim_retrievable!`/PropertyCollector
across ~10 files plus a `perf_worker.rs`). So the harness greps the **crate** source
(generated `types/` + `mo/`) — not `examples/`, `mcp/`, or `vim_build/`.

Given a harness with that source + rg + possibly rust-analyzer, honestly rank what the MCP
adds:

- **Exact-symbol location of *generated* types (e.g. `VirtualVmxnet3`) — commodity, do
  NOT invest.** `rg` (and certainly an LSP) beats any BM25 we would ship over types that
  exist in the crate source. **But note the scoping word "generated"** — see the pruned
  Event/MethodFault pillar below, where the symbols are *not* in the source and search
  becomes MCP-unique. So Phase 2 is dead *for generated types*, not universally.
- **Type-based "which functions accept / return struct T" — uncertain, don't bet.**
  Plausibly useful, but this is exactly the kind of thing the harness race + rust-analyzer
  (find-references, type hierarchy) is likely to commoditize. Building it ourselves may be
  overtaken. Treat as speculative.
- **PropertyCollector property paths & retrieval specs — GENUINELY MCP-unique.** The path
  string (`config.hardware.device`), its camelCase VIM spelling, the array/downcast
  crossings, which MO roots reach it, and the projection to a `vim_retrievable!` /
  PropertyCollector spec are **not trivially derivable from the Rust source**. This is
  where our graph work (`paths.rs`, `property_collector_path()`) pays off — but scoped **in
  service of producing property paths/specs**, not general reachability for its own sake.
- **Performance / metrics counters — GENUINELY grep-proof, and a bigger workstream than the
  MCP (per Kiril).** Counters (`cpu.usage.average` …), their group/name/rollup/unit metadata,
  and the `PerfQuerySpec` / `PerfMetricId` query flow are a server-side taxonomy absent from
  the Rust types entirely. Grep gives the agent *nothing* — this is the only truly grep-proof
  pillar. **But metrics are also under-supported in `vim_rs` itself**: only the raw generated
  `mo/performance_manager.rs` exists, no ergonomic helpers (verified). So the real work is
  cross-cutting — **library utilities + reference docs + MCP + evals** (see Phase 3′) — and it
  improves vim_rs for all users, not just agents. Deferred behind PropertyCollector because
  of that upfront library cost.
- **Pruned `Event` / `MethodFault` hierarchies — a PRESENTATION/DISCOVERY advantage, not
  an information monopoly (corrected after verification).** These subtypes are **not
  generated as Rust structs**: `vim_rs` keeps base `Event` / `MethodFault` with a
  `type_: Option<StructType>` discriminator + `extra_fields_: HashMap<String, Value>`.
  Grounding: **445 `*Event` + 117 `*Fault` discriminators** in `StructType` with no struct.
  ⚠ **Correction:** an earlier version of this doc claimed "grep finds nothing" here. **That
  is false.** Verified in the crate source:
  - Discriminator names are in `types/struct_enum.rs` — **greppable**.
  - Per-subtype **field→type maps** are in `types/api_field_registry.rs` (schema-guided XML
    deser), e.g. `UserLogoutSessionEvent → ("callCount", AFT_I_64), ("loginTime", AFT_STR)`
    — **greppable**; plus (de)ser shims in `structs.rs`.
  So of the three pillars, **only Metrics is truly grep-proof.** What remains genuinely thin
  for an agent here is: (a) **concept-level discovery** across ~560 subtypes ("events meaning
  a datastore filled up") — registry/name grep needs the right keyword and gives no
  semantics; and (b) **usable presentation** — the registry is a 40k-line machine-generated
  file of cryptic `AFT_*` constants with no docs, whereas the MCP hands over per-subtype
  keys+types+docs cleanly. So the honest value is **presentation + discovery**, and the
  hypothesis must be tested, not asserted (see `EvalHarness.md` §1.1).
  - **Good news: the MCP already carries the data.** `api_database`'s `StructureEntry`
    Skip/Prune fields + `format_structure_doc`'s "Not Generated in Rust" branch already emit
    it → mostly **discovery + presentation**, not new authoring. (Scoped search over pruned
    types is still worthwhile — it's the discovery half.)

**Consequence for the whole project:** the differentiated value is **three** confusing
subsystems, ranked by how source-opaque they actually are — **Performance metrics**
(genuinely grep-proof; absent from source), **PropertyCollector** (pieces are in source but
the assembled path is a synthesis the harness does poorly), and **pruned Event/MethodFault**
(field data *is* in source via `api_field_registry.rs`, so the edge is presentation +
concept-discovery, not information). General "search the API" / "find a method" for
*generated* types is the harness's job. Hard implications:
1. **Eval problems must over-weight these three.** A "power on a VM" task is grep-solved
   (`power_on_vm_task`); MCP shows no lift and it pollutes the measurement. Bias problems
   toward property-path construction, perf-counter queries, and event/fault-subtype
   handling (vtui — which has `event.rs`, `perf_worker.rs`, and PropertyCollector usage —
   is a good source of realistic riddles).
2. **The mechanisms differ per pillar:** PropertyCollector = graph→path projection (the
   original GraphSearch idea, re-scoped); Metrics = a curated external knowledge base (not
   a graph); Event/MethodFault = scoped search + presentation over data the MCP already
   has. The doc's "GraphSearch" title only covers the first — the effort is really "expose
   the source-opaque parts of the API."

---

## 8. Suggested first probes (small, reversible, information-gathering)

Ordered by information-per-effort, not by build order. Each is a spike to *answer open
questions above*, not a commitment. The eval harness now leads — it is both our
measurement instrument and the source of the telemetry we lack.

1. **Stand up the task-level eval harness** (7.9; full spec in `EvalHarness.md`): ~4
   fixture-backed **Event/Fault** tasks, a fixed cheap tool-calling model, grade on
   **compile + fixture assertion**, run MCP-on vs. MCP-off. (The ~20 Java-SDK problems are
   for later phases, not this slice.) **Log every tool call** during these runs — this is how
   we bootstrap query telemetry (7.1).
2. **Calibrate the eval model** (7.9): spot-check one weaker + one stronger model on a few
   tasks to confirm the chosen model sits in the band where MCP actually moves the needle.
3. **Prototype BM25 over the existing `embedding_text`** and A/B hybrid vs. pure vector on
   the eval set (7.6). Lowest-risk change, likely-highest-value, now measurable via (1).
4. **Dump the full edge list** from the vim_build model into a scratch file and *look at
   it* — node/edge counts, degree distribution, polymorphic fan-out, how many nodes are
   synthetic `ArrayOf*` (7.4, 7.5). Tells us whether "traverse at query time" is obviously
   fine and how to treat arrays.
5. **Extract example→API edges for a few of the 18 snippets** (7.8) to see if the
   "examples as proof" idea is viable, then feed those into an eval A/B.

---

## 9. Parking lot / notes

- `paths.rs` `property_collector_path()` already projects an access path to a
  PropertyCollector dotted string — reuse this when we build the "generic query" answer
  style rather than reinventing it.
- `select_paths_for_display` encodes real domain priors (inventory priority, properties
  before methods, cap per MO). Whatever replaces materialized paths must *keep this
  knowledge* — as a ranking function, not as a truncation.
- The `field_data` PHF (234 types) and the item `paths` are the two existing graph views;
  any redesign should explicitly decide their fate (unify vs. keep one as a cache).
- `MethodFault`/`Event`/`Skip`/`Prune` types: the item model already documents the
  `type_` + `extra_fields_` access pattern richly — the graph must not contradict it.

---

## 10. Proposed phased plan

Design principle: **the eval harness comes first and everything after it is independently
shippable and independently measurable through it.** Phases 2–4 do not depend on each
other; they depend only on Phase 1 existing. This lets us reorder 2–4 by whatever the
Phase 1 telemetry says matters, and abandon any of them cheaply.

Each phase below lists: goal, deliverables, and the decisions that must be made *inside*
that phase (so a planning session has a concrete agenda).

### Phase 1 — Evaluation harness + tool-call logging  *(the spine; do first)*

> **Detailed, implementation-ready design: [`EvalHarness.md`](./EvalHarness.md)** (Goose +
> OpenRouter + podman, Event/MethodFault proving slice, two-tier grade). The summary below
> is kept for context; EvalHarness.md is the build spec.

**Goal:** measure "does the MCP help a coding agent write correct vim_rs code," A/B, and
capture real tool-call traces as our first telemetry.

> Note: the bullets below are the original broad sketch; the **authoritative, corrected
> Phase-1 deliverables are in `EvalHarness.md`** (Event/Fault slice, ~4 fixture tasks,
> two-tier grade, sealed scaffold, identical arm instructions). Where they differ, follow
> EvalHarness.md.

**Deliverables:**
- `mcp/evals/problems/` — the **first slice is ~4 Event/Fault fixture tasks** (not the ~20
  Java-SDK problems, which are for later phases). Each: `{ id, goal (NL, no vim_rs names),
  solve_signature, fixtures[] + expected[], forbidden[], notes }`.
- A **runner** that, per problem, drives a **full coding harness with vim_rs source
  access** (per §7.9): arm A = harness+source, **no MCP**; arm B = harness+source+MCP.
  Captures the produced Rust snippet **and the run trace** (turns, tool calls, tokens).
- A **grader**: drop the snippet into a scratch crate template and `cargo build` against
  vim_rs; pass = compiles (iteration-1 bar). Capture failure class (didn't compile / wrong
  type / hallucinated method) for diagnostics.
- **Efficiency metrics, not just pass/fail:** turns, tool-call count, tokens, wall-clock
  to a compiling result — because pass/fail may saturate with a capable harness (§7.9).
- **Telemetry:** log every MCP tool call per run (tool, args, result IDs, follow-ups,
  latency) — our first query dataset.
- **Scorecard:** A vs B per problem on {pass, efficiency}, plus tool-usage counts.

**Decisions to make in this phase:**
- **Harness** *(researched — see below; recommend Goose)*: needs scriptable headless runs,
  structured output, MCP support, OpenRouter, and extractable per-call telemetry.
  - **Goose (Block / Linux Foundation) — recommended.** `goose run` headless (`-t` text /
    `-i` file / stdin; output `text|json|stream-json`), and **Recipes**: YAML bundling
    system prompt + MCP extensions + provider/model + params + a **structured output
    schema**, version-controlled and composable. That maps 1:1 to our eval matrix (a recipe
    per arm × params per problem). Rust codebase → fits our stack and containerizes cleanly.
    Docs: block.github.io/goose (headless tutorial + recipe reference).
  - **OpenCode (SST, opencode.ai) — strong alternative.** `opencode run --format json`
    (nd-JSON event stream = tool-call telemetry), `-m provider/model`, **`--auto`**
    (auto-approve — needed for unattended), config-as-code `opencode.json` (providers +
    MCP), client/server + ACP mode. Equally viable; tie-breaker is Goose recipes being
    purpose-built for repeatable structured runs. ⚠ use **opencode.ai**, not the `open-code.ai`
    copycat domain.
  - **Pi (pi.dev, Mario Zechner) — most hackable; the upgrade path, not the starter.**
    MIT, TypeScript/Bun, ships an **SDK** (`@earendil-works/pi-agent-core` + `pi-ai`) plus
    headless print/JSON, RPC (JSON over stdio), and TUI modes. 30+ providers incl OpenRouter
    + DeepSeek. Philosophy "primitives, not features": **MCP is not built-in** (add via a TS
    extension), and **no permission system — you sandbox it yourself** (fits the podman plan).
    Prior art `spences10/my-pi` already does MCP + **local eval telemetry**. Best choice if
    we later want a bespoke SDK-driven eval rig with fine-grained telemetry; for the *first
    slice* its MCP-via-extension + more assembly make Goose faster to signal.
  - Cursor Composer stays manual-only (closed → poor batch/telemetry).
- **Containerization (podman, per Kiril):** each eval run in a podman container bundling:
  the chosen agent, the `vim_rs` MCP server binary, a Rust toolchain + `vim_rs` crate dep
  (for `cargo build` grading), the problem, `OPENROUTER_API_KEY` via env. Ephemeral,
  reproducible, parallelizable.
- **Model via OpenRouter**: the fixed cheap tool-calling model (DeepSeek v4 Flash
  candidate) + calibration spot-check; must do reliable tool calling (§7.9).
- **Problem count & selection**: **first slice = PropertyCollector** (per Kiril), ~4
  fixture/mock-backed property-path tasks — see `EvalHarness.md`. "power on a VM" is
  grep-solved and won't show MCP lift; property-path synthesis is the strongest genuinely-hard
  signal. vtui's PropertyCollector usage is good raw material. (Snippet-leak worry is small:
  a consumer workspace doesn't ship `examples/`.) Event/Fault demoted (weak signal); metrics
  deferred behind a library workstream (Phase 3′).
- **Snippet scaffold**: how much boilerplate (connect/auth) we provide vs. expect the model
  to produce, so grading isolates the API-usage skill.
- **Telemetry extraction**: how we pull tool-call traces out of the chosen harness
  (built-in logs / MCP-server-side logging / proxy).

### Phase 2 — Generic lexical/hybrid search over *generated* types  *(DROPPED)*

**Dropped (§7.12):** for types that exist in the crate source, ripgrep / LSP beat anything
we'd ship. Do **not** build generic search over generated types. Note this is *not* the
same as search over *pruned* Event/MethodFault types — that survives and moves to Phase 3″
below, because those symbols are absent from the source.

### Phase 3 — PropertyCollector path & retrieval assistant  *(real bet #1 — FIRST FOCUS, per Kiril)*

**Re-scoped (§7.12):** not "general graph search" — the graph exists to **produce valid
property paths and retrieval specs**. Why this is the strongest first bet: even though the
raw field data is in `structs.rs`, assembling a *correct multi-hop property path* — camelCase
spelling, crossing arrays, downcasts, PropertyCollector-legal, reachable from the right MO
root — is a synthesis over 473k lines that grep does **poorly** (a much harder task than the
flat `api_field_registry` lookup that made Event/Fault a weak signal). This is also the
**first eval slice** (see `EvalHarness.md`).

**Goal:** given a target (type/field/concept) reachable in the inventory, produce the
property path string (VIM camelCase), which MO roots reach it, array/downcast handling,
and the ready-to-paste `vim_retrievable!` / PropertyCollector spec.

**Deliverables:** unified node + typed-edge store (exclude `ArrayOf*`, honor
`Skip`/`Prune`); query-time traversal with the `select_paths_for_display` priors as a cost
function (shorter paths score higher, §7.2); reuse `property_collector_path()` for the
projection; array-crossing + downcast navigation so the tree tools reach array-element
fields; a `build_retrieval(target, root?)` tool that emits the spec + `vim_retrievable!`
block. Eval A/B specifically on property-path tasks (grade: compile + offline mock-retrieval
via `tests/support/mock_vim_client.rs`).

**Decisions:** edge-store ownership — vim_build vs. data_transformer (§7.3); materialize
vs. traverse (§7.4, likely traverse given tiny scale); polymorphism expansion (§7.5); fate
of frozen `paths` (delete vs. cache).

*Prereq spike:* dump the full edge list and eyeball it (§8 probe 4) before committing.

### Phase 3′ — Performance metrics: cross-cutting workstream  *(real bet #2; bigger than the MCP)*

**Reframed (per Kiril):** metrics are **not well-supported in vim_rs itself** — there is only
the raw generated `mo/performance_manager.rs`, no ergonomic helpers (verified). So this isn't
just "add a counter KB to the MCP"; it's a **workstream spanning the library, docs, MCP, and
eval**, and it improves vim_rs for everyone, not only agents:
1. **vim_rs library utilities** — ergonomic helpers over `PerformanceManager`: counter
   lookup/caching, `PerfQuerySpec`/`PerfMetricId` builders, interval/level handling. (New
   `core/` helpers, mirroring what `TaskTracker`/`ObjectRetriever` do for other subsystems.)
2. **Reference docs** — the counter taxonomy (`cpu.usage.average`, `mem.*`, `disk.*`,
   `net.*`), rollup/stats/unit semantics, the query flow. Source: VMware perf docs +
   `perf_metrics` snippet + vtui `perf_worker.rs`.
3. **MCP** — surface the counter KB + query-pattern tool (this is the genuinely grep-proof
   pillar; the taxonomy is absent from source entirely).
4. **Metrics evals** — A/B on "pick counters + build a valid query for metric X on entity Y."

**Decisions:** scope/ordering of the library work vs. MCP work; ship counter data statically
vs. document the runtime `QueryPerfCounter` discovery flow (or both); how much of (1) lands
before the eval can run.

### Phase 3″ — Event / MethodFault subtype assistant  *(DEMOTED — weak signal)*

**Demoted (§7.12 correction):** the per-subtype field→type data *is* in the crate
(`api_field_registry.rs`) as a flat lookup, so the MCP edge is only presentation + concept
discovery — the weakest of the three. Keep as a **later, cheap** add (the data is already in
`api_database`; deliverable is a scoped search over Skip/Prune types + the documented
`extra_fields_`/`type_` detail view), but it is **no longer the first slice**.

### Phase 4 — Context packs + examples linking  *("correct on the first try")*

**Goal:** return assembled subgraphs, not isolated cards; make the 18 snippets first-class
(they're MCP-only — a consumer workspace does *not* have `examples/`, so indexing them is
legitimate, non-redundant value).

**Deliverables:** example→{method,type,field} edge extraction (`syn` or identifier match);
`explain_access(target)` = best route + construction fields + a matching example in one
call; neighborhood assembly in search results. A/B on Phase 1.

**Decisions:** unit of retrieval — item vs. subgraph (§7.2); structured vs. markdown tool
output and token budget (§7.10); grow snippet corpus vs. mine Java samples (§7.8).

### Sequencing note

Revised again (Kiril, focus on PropertyCollector): **1 (eval rig, PropertyCollector slice)
→ 3 (+edge-spike) → 3′ (metrics workstream) → {3″, 4}**, with **2 dropped**. Reasoning:
- **3 PropertyCollector = first focus** — the strongest genuinely-hard signal (multi-hop
  path synthesis), and it's where the existing `paths.rs` machinery pays off.
- **3′ Metrics = the big parallel bet** — a cross-cutting library+docs+MCP+eval workstream,
  the only truly grep-proof pillar; also fixes a real vim_rs ergonomics gap.
- **3″ Event/Fault demoted** — weak (presentation-only) signal; cheap to add later.
- **4** layers curated examples on top.
Phase 1's slice is now PropertyCollector, not Event/Fault (`EvalHarness.md`).
