# Feature Specification: MCP Performance Counters

**Feature Branch**: `007-mcp-perf-counters`

**Created**: 2026-10-25

**Status**: Counter vectors removed. Catalog descriptions are rustdoc on `vim_rs/src/perf/metrics.rs`. The MCP server does not embed counters. Evaluation is two query problems, `pf01_vm_balloon_costop_query` and `pf02_vm_electricity_turn_query`.

**Input**: User description: `docs/vim_rs-perf-mcp.md` (canonical specification v4.2, Part I). This file is the tracked copy of that Part I. `docs/**` is gitignored, so planning and implementation use this spec. Part II of the source (type layouts, pipeline code, example bodies, evaluation fixture schemas, and the phase plan) is input to `/speckit-plan` and is not repeated here. Normative tables and acceptance checks in this spec must not be softened.

## Context & Problem

The assistant-facing documentation server embeds a semantic index of the vSphere API and of the runnable examples. That index is built from the OpenAPI specification and from the example sources. The performance module and its 732 generated counter constants are not in the OpenAPI specification, so the server knows nothing about them.

An assistant asked to monitor virtual machines therefore fails in four systematic ways:

1. **It reinvents raw performance-manager plumbing.** A search for “virtual machine cpu usage” returns the raw query methods, the query specification, and the polymorphic metric base. The assistant never discovers the performance client, and writes a long program of counter-id maps, per-object provider loops, and broken downcasts. That program already exists as the legacy raw example.
2. **It forgets the feature flag.** The performance module exists only when the `perf` feature is enabled. An assistant that finds the API and writes a plain `vim_rs` 0.7 dependency gets an unresolved import.
3. **Operator language does not match wire names.** Intent uses words the catalog does not, and the catalog itself has empty labels and case-only near-duplicates:
   - “VM memory ballooning” is `mem.vmmemctl.average`, not `balloon`.
   - “vCPU scheduling contention” is `cpu.ready.summation` (read as a ready percentage) or `cpu.costop.summation`.
   - “virtual disk IOPS” is `virtualDisk.numberReadAveraged.average` and `numberWriteAveraged.average`.
   - “memory swap-in rate” is `mem.swapinRate.average` (kilobytes per second). “memory swapped in” is an amount. `mem.swapIn.average` (empty catalog label, kilobytes) and `mem.swapin.average` (label “Swap in”, kilobytes) differ only by case. The same split exists for `swapOut` / `swapout`.
4. **Time-series idioms are missing.** Correct use over time needs three distinct entry points: a live poll with a prepared query, refresh rate, missed-tick skipping, and sparklines; a historical extract with a past-day interval, an explicit window, time chunks, and tidy rows; and a lossless incremental export with a per-entity watermark. The starter guide and the indexed examples must teach those entry points.

The companion library feature is [`specs/006-perf-manager-api/`](../006-perf-manager-api/spec.md). This feature does not change that runtime API. It makes the shipped counters and workflows discoverable to an assistant.

### Goals

1. Index all 732 vCenter 8.0.3 performance counters as first-class searchable documents. Identifiers, dimensions, scopes, and tiers come from the same planner that emits the generated constants. No second derivation of those facts.
2. Record ESXi 8.0.3 availability on each counter document, and append the same availability marker to each generated constant’s documentation. The ESXi catalog is a strict subset (353 of 353 match vCenter on group, name, rollup, and stats kind) and adds no constants or scopes.
3. Extend the existing search and lookup tools. Add a counters filter. Resolve counter aliases in three steps: exact primary key, exact case-sensitive alias, then an unambiguous lowercase alias. An ambiguous lowercase query returns no document and lists every candidate.
4. Add a compact Step 5 (at most 40 lines) to the starter guide: the feature flag, how to discover a counter, a five-row workflow table, value and gap and scope rules, and the resilient poll loop.
5. Ship five self-contained performance examples, all indexed under the performance category: one-shot snapshot, live monitor, historical extract, incremental export, and a labelled legacy raw contrast.
6. Measure assistant lift on five performance problems in the evaluation harness at `mcp/evals/`. The target is a 0.20 retrieval pass-rate lift. That measurement is a scored evaluation, not a release gate.

### Non-Goals

- Adding assistant tools. The tool count stays at 6.
- Connecting the documentation server to a live vCenter or ESXi. The server stays an offline static index.
- Changing the performance-module runtime API, or changing generated constant names, dimensions, scopes, or per-entity re-exports. The only generated-output change is documentation text.
- Committing a counter index under the generator’s data directory. The counter index is a documentation-server artifact, written only when debug JSON is requested.
- Indexing performance-module types or methods through the OpenAPI extractor. Usage is taught through Step 5, the counter documents, and the performance examples.
- Separate poller or typed-scope examples. Typed-scope teaching lives in the live monitor and in the counter documents. Incremental-export teaching lives in the export example.

### Settled Product Decisions

These decisions are closed. Planning must not reopen them.

| ID | Decision | Why |
|---|---|---|
| **MCP-ADR-01** | Extend search (`filter = "counters"`) and lookup. Add no new tools. | Assistants already follow starter guide, then search, then lookup. A counters filter makes counters discoverable both in the default “all” filter and on their own. |
| **MCP-ADR-02** | Canonical id is `counter::<group>.<name>.<rollup>`. The `.absolute` suffix appears only on `counter::disk.scsiReservationCnflctsPct.average.absolute` (key 148). Lookup resolves aliases in three steps. | vCenter 8.0.3 has eight case-only collision pairs. A lowercased alias is accepted only when it maps to exactly one counter. Module-qualified Rust aliases stay distinct even when lowercased. Bare identifiers such as `USAGE_AVERAGE` are not aliases, because 35 identifiers repeat across modules. |
| **MCP-ADR-03** | Counter documents are produced by the documentation pipeline, which calls the generator’s read-only planner. | The library never needed a sidecar. The pipeline owns presentation: id, label fallback, scale description, valid entities, and the Rust constant path. Rejected: the generator writing an MCP sidecar; re-deriving identifiers in the documentation crates; parsing the generated constants file. |
| **MCP-ADR-04** | Performance-manager pages distinguish read from administration. | The object page, the six query methods, and the two catalog accessors redirect to the performance client. The five interval and level mutation methods get a neutral note that they stay on the raw manager. The description accessor gets no banner. |
| **MCP-ADR-05** | Two-tier testing, plus a scored behavioral evaluation. | Hermetic tests cover alias lookup, banners, embedding text, and the 732/353 generation checks, without loading the embedding model. Model-backed ranking tests are opt-in. End-to-end lift is measured in the evaluation harness. |
| **MCP-ADR-06** | ESXi availability is documentation-only in the generator. | The ESXi catalog adds no constants. The generator reads it only to append `· ESXi 8.0.3` or `· vCenter only`, and exposes the same flag through the planner. |
| **MCP-ADR-07** | Version and feature are always explicit. | Every snippet, template, and document that uses the performance module says `vim_rs` 0.7 with `features = ["perf"]`. Examples that do not use it, including the legacy raw example, say plain `vim_rs` 0.7. The example-index template drops the separate macros crate, because the library re-exports those macros and the two versions need not move together. |
| **MCP-ADR-08** | Five performance examples, one workflow each. The legacy raw example stays and is labelled. | One-shot, live monitor, historical, incremental export, and raw contrast. A sixth near-duplicate would compete in retrieval. |

### Release Coupling Gate

The performance module ships in `vim_rs` 0.7.0. The in-tree crate is still 0.6.1. The module is behind the `perf` feature, which is off by default. The updated documentation server and its embedded index must not be released before `vim_rs` 0.7.0 is published. Every rendered counter document, redirect banner, starter-guide section, and dependency template of every example that uses the performance module must state both requirements: `vim_rs` at least 0.7, and `features = ["perf"]`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Natural-language counter discovery (Priority: P1)

An assistant is asked to monitor VM memory ballooning, swap-in rate, and CPU ready time. It searches with the counters filter and receives the balloon counter, with its label, summary, Rust constant, unit and scaled type, valid entities, ESXi availability, and the `perf` feature requirement. It then looks the counter up by canonical id or by a Rust alias and gets the same document.

**Why this priority**: This is the failure mode that produces the wrong counter, the wrong unit, or a missing feature flag. Search and lookup of the 732 counters deliver that correction even before any new example exists.

**Independent Test**: Build the counter index from the committed catalogs and confirm 732 unique documents, the locked humanized labels, the ESXi split of 353 versus 379, and the alias cases in MCP-FR-04. With the embedding model, confirm the five meaning queries in MCP-FR-06 rank the expected counter in the top five when the filter is counters.

**Acceptance Scenarios**:

1. **Given** the committed vCenter and ESXi catalogs, **When** counters are collected, **Then** there are 732 documents with 732 unique ids, every label and summary is non-empty, 353 are marked available on ESXi 8.0.3, and key 148’s id ends in `.absolute`.
2. **Given** a search for “VM memory ballooning” limited to counters, **When** the top five results return, **Then** every result is a counter and the balloon counter (`counter::mem.vmmemctl.average`) is among them.
3. **Given** the case-distinct swap-in amounts, **When** the assistant looks up `mem.swapIn.average` and `mem.swapin.average`, **Then** each lookup returns a different counter, and a case-insensitive `MEM.SWAPOUT.NONE` returns no document and names both candidates.
4. **Given** a repeated bare identifier such as `USAGE_AVERAGE`, **When** the assistant looks it up, **Then** no document is returned.

---

### User Story 2 - Scopes and ESXi availability in the counter document (Priority: P1)

An assistant writing typed inventory code against vCenter or a standalone ESXi host looks up a counter and sees the compile-time scope, the derivation tier, the per-entity constant paths, what to do for an inferred (tier 3b) scope, and whether the counter exists on ESXi 8.0.3.

**Why this priority**: A discovered constant that is used on the wrong entity kind, or queried on a host that does not have it, fails at compile time or returns an empty series. The document has to prevent both. It is the same index as Story 1, tested by rendering documents rather than by ranking.

**Independent Test**: Render the summary and the full document for CPU usage, CPU ready, per-device disk latency, one tier-3b counter, and a synthetic counter with no collection level. Confirm the fields in MCP-FR-07, including the feature requirement and the ESXi line.

**Acceptance Scenarios**:

1. **Given** `cpu.usage.average`, **When** the summary and the full document are rendered, **Then** they show the constant, the scope, aggregate and per-device support, ESXi availability, and the full scale description.
2. **Given** `cpu.ready.summation`, **When** the usage snippet is rendered, **Then** the readout is the raw-series ready-percentage call, not a scaled-series call.
3. **Given** a tier-3b counter such as `vcResources.buffersz.average`, **When** the document is rendered, **Then** it tells the reader to widen the scope or to use the unchecked metric, it lists no per-entity path, and it says the counter is vCenter-only.
4. **Given** a counter whose collection level is unset, **When** the summary is rendered, **Then** the level prints as `n/a`.

---

### User Story 3 - Live monitoring dashboard (Priority: P2)

An assistant is asked to show live CPU and memory for every virtual machine, refreshing on the realtime interval, with a small trend sparkline. It finds a complete example that polls a prepared query at the realtime refresh rate, skips missed ticks, draws a six-slot sparkline, reads the current slot, shows CPU ready as a percentage of the VM’s vCPU count, and survives a failed tick and a powered-off VM.

**Why this priority**: Live monitoring is the second most common request after “which counter is this”, and it is the workflow that goes wrong when the assistant copies the one-shot snapshot. It depends on the counter documents only as retrieval context; the example itself is independently runnable.

**Independent Test**: The live-monitor example compiles with the other snippet binaries, uses per-entity virtual-machine constants, stops after a bounded number of ticks, and closes the client. Its module title matches MCP-FR-11.

**Acceptance Scenarios**:

1. **Given** the live-monitor example, **When** an assistant reads it, **Then** it polls a prepared realtime query, renders six-slot CPU and memory sparklines, and reads current megahertz, bytes, and CPU-ready from the newest slot.
2. **Given** a tick that fails or a VM that is powered off, **When** the example handles that tick, **Then** the loop continues, a failed tick is logged, and a powered-off VM draws a blank row rather than a frozen previous value.
3. **Given** a sparkline slot with no sample, **When** it is drawn, **Then** the glyph is a space, so “no data” is distinct from zero percent.

---

### User Story 4 - Historical extraction (Priority: P2)

An assistant is asked to export 5-minute CPU and memory metrics for the past day, or for several days, as tidy rows. It finds an example that uses the past-day interval, an explicit start and end, a six-hour time chunk, and row export that skips gap samples.

**Why this priority**: Historical queries are a different contract from realtime (the server ignores a maximum-sample cap, and an unbounded multi-day query is one large call). The example is independently runnable once the library feature exists.

**Independent Test**: The historical example compiles, its title matches MCP-FR-11, and its comments state that historical intervals ignore the maximum-sample cap and that time chunks bound each call.

**Acceptance Scenarios**:

1. **Given** the historical example, **When** an assistant reads the query, **Then** it requests past-day CPU usage and consumed memory between now and 24 hours ago, split into six-hour chunks, and prints tidy rows with gaps omitted.
2. **Given** a reader who might add a maximum-sample cap, **When** they read the comments, **Then** the example says the server ignores that cap on historical intervals and that a multi-day extract uses the past-week interval with a “since ago” window.

---

### User Story 5 - Lossless incremental export (Priority: P2)

An assistant is asked to stream new VM samples to a time-series database on every refresh, without duplicates or gaps. It finds an example that pulls only new samples against a watermark, drops gap slots, forgets entities that left the inventory, and logs a failed tick without stopping.

**Why this priority**: Export is the workflow where a shared watermark and a fatal error produce duplicates, gaps, or a dead process. It is independently runnable and is also the source of the three poll statements the starter guide must quote.

**Independent Test**: The export example compiles, runs a bounded number of ticks, closes the client, and contains the three starter-guide statements character for character (MCP-FR-09).

**Acceptance Scenarios**:

1. **Given** the export example, **When** an assistant reads the loop, **Then** each tick fetches incrementally, keeps only samples newer than the watermark, and continues after a logged error.
2. **Given** inventory that changes, **When** the refresh interval elapses, **Then** the watermark forgets entities that are no longer present, and a powered-off VM’s mark does not hold the others back.
3. **Given** the starter guide’s poll loop, **When** its ticker, missed-tick, and match statements are compared with this example, **Then** those three statements are character-for-character the same. The guide’s `loop` header and the example’s bounded tick header may differ.

---

### User Story 6 - Raw API contrast (Priority: P3)

An assistant asked to work without the `perf` feature, or reading code that predates it, finds the raw performance-manager example clearly labelled as a legacy contrast. It reaches for that example only when the feature is truly unavailable, and it can see what the performance client replaces.

**Why this priority**: The raw example must remain for callers who cannot enable the feature, but it must stop winning “how do I get CPU usage” over the performance client. Relabelling is a small, independently shippable change.

**Independent Test**: The raw example’s code is unchanged. Its opening documentation line begins with `Legacy:`, and the example index does not add the `perf` feature to its dependency template.

**Acceptance Scenarios**:

1. **Given** the raw example, **When** an assistant reads the module documentation, **Then** the first line marks it as legacy, and the following text says it contrasts with the one-shot performance example and shows direct performance-manager use for callers who cannot or do not want the `perf` feature.
2. **Given** the example index, **When** dependencies are rendered, **Then** this example and the non-performance examples depend on plain `vim_rs` 0.7, while the four performance-module examples require the `perf` feature.

---

### User Story 7 - Measured assistant lift (Priority: P3)

A maintainer runs five performance problems on the harness in `mcp/evals/`, with the documentation server off and on. The on arm should retrieve a correct solution more often, compile at least as often, and not use more tool calls. A miss is written down and starts a tuning pass. It does not block release of the index.

**Why this priority**: The lift number is how the project knows the documents and examples changed assistant behavior. Stories 1–6 are useful without it, and the harness is not merged yet.

**Independent Test**: Reference solutions for the five problems pass the grader. The scorecard records the model id and applies the three thresholds in MCP-FR-14. Existing non-performance problems still pass.

**Acceptance Scenarios**:

1. **Given** the five problems and the same model on both arms, **When** the matrix finishes, **Then** the scorecard is committed whether the thresholds are met or not.
2. **Given** a miss, **When** the outcome is recorded, **Then** it names the failures and opens a tuning pass over synonyms, Step 5 wording, or counter-document layout, and the index may still ship.

---

### Edge Cases

- **Case-only wire collisions.** Eight pairs (16 counters): `mem.swapIn` / `mem.swapin` and `mem.swapOut` / `mem.swapout`, each with average, maximum, minimum, and none. Exact lookup keeps them apart. A lowercased alias that hits more than one counter returns no document and lists every canonical id.
- **Same wire name, different stats kind.** `disk.scsiReservationCnflctsPct.average` is key 358 (rate, unsuffixed id) and key 148 (absolute, id ends in `.absolute`). No other counter gains that suffix.
- **Empty catalog labels.** 34 counters have both the name label and the name summary empty (groups `datastore`, `mem`, `vmotion`, `vmx`). No counter has an empty group or unit label. The label is humanized from the name by the literal rule in MCP-FR-01, with no dictionary. Locked results: `swapIn` → “Swap in”, `vmx.poll.itersPerS` → “Poll iters per s”, `swapunreserved` → “Swapunreserved”, `capacity.usage.vmOvrhd` → “Capacity usage vm ovrhd”.
- **Repeated bare identifiers.** 35 constant identifiers repeat across modules (for example `USAGE_AVERAGE`). They are not aliases. Module-qualified forms resolve.
- **Ambiguous lookup presented to the assistant.** The lookup tool says multiple counters match, and lists each candidate’s id, Rust constant, label, and unit. It does not guess.
- **Lookup miss that is not ambiguous.** The suggestion scan covers every indexed id, still returns at most five suggestions, and mentions a counter id and the one-shot performance example. It does not stop after the first thousand keys.
- **ESXi catalog disagrees with vCenter.** If the ESXi catalog contains a counter absent from vCenter, planning fails. A missing catalog file is an error, because both files are committed.
- **Stale embedded index.** A newer vCenter catalog, ESXi catalog, or generated constants file makes the server build fail until the index is rebuilt. Changes under the unindexed performance-catalog example or the terminal UI do not.
- **Feature flag mentioned only in comments.** A documentation comment that names the performance module does not turn on the `perf` feature in the dependency template. Only a source line that starts, after trimming, with `use vim_rs::perf` does. The legacy raw example stays on the plain dependency.
- **Category order.** The reworked live monitor retrieves properties. Performance categorization is decided before the property-collector rule, so the five performance examples stay in the performance category.
- **Embedding regression.** Four non-performance queries are recorded before any index change. Afterwards each query’s previous first result stays first, and at least four of its five previous top ids stay in the top five.
- **Synonym collisions.** The synonym table is first match wins. CPU latency is contention, not storage latency. Aborted disk commands do not receive the IOPS phrase.
- **Ready percentage.** The counter document calls ready-percentage on the raw series. It is blank for a gap slot and for a non-aggregate instance when the vCPU count is greater than one.
- **Sparkline.** The live monitor reimplements the seven-glyph sparkline for scaled percents. A missing sample is a space. Zero percent stays the lowest glyph. The implementation is not a copy of the terminal UI’s raw-hundredths function.
- **Historical sample cap.** Examples and the starter guide say the server ignores a maximum-sample cap on historical intervals. Time chunks bound each call.
- **Evaluation clock and phases.** Performance fixtures store sample times as offsets from the mock’s construction time, not as absolute timestamps. The incremental problem is phased so the third sample is invisible on the first tick. A missing tool-call count is treated as zero. The operator chooses the model; the scorecard records it; both arms use that model.
- **Evaluation harness.** The five-problem slice extends the harness in `mcp/evals/`. Stories 1–6 do not depend on it, and a missed lift does not hold their release.

## Requirements *(mandatory)*

### Functional Requirements

- **MCP-FR-01**: A documentation-pipeline step must build one counter document per planned vCenter counter.
  - It reads the committed vCenter catalog and the committed ESXi catalog. Either file missing is an error.
  - It calls the generator’s public planner with both catalogs, joins catalog labels by counter key, and does not re-derive identifiers, dimensions, scopes, or tiers.
  - For the 34 counters with an empty name label and an empty name summary, the label is the humanized name: split on `.` and on lower-or-digit to upper camelCase boundaries, lowercase the parts, join with spaces, and capitalize the first letter. The summary is the group summary, the lowercased label, the unit summary, and the rollup.
  - When debug JSON is requested, the step writes a counter file beside the other debug artifacts. The main pipeline passes the documents into the index in memory.
  - **Acceptance**: **Given** the committed catalogs, **When** the counter step’s tests run, **Then** it returns 732 entries with 732 unique ids; every label and summary is non-empty; 353 have ESXi support; key 148’s id is `counter::disk.scsiReservationCnflctsPct.average.absolute`; the four locked humanized labels above match exactly; and every entry’s module, identifier, dimension, and scope appear as a generated constant. **When** the index is rebuilt, **Then** it contains exactly 732 items of type counter.

- **MCP-FR-02**: The generator must annotate ESXi availability in documentation only, and must expose the planner it already uses.
  - The ESXi 8.0.3 catalog is committed next to the vCenter catalog and is byte-identical to the library’s ESXi fixture. A generator test asserts that equality.
  - Every constant’s documentation line ends with `· ESXi 8.0.3` when that `(group, name, rollup, stats)` is in the ESXi catalog, and with `· vCenter only` otherwise.
  - Tier 3b text is normalized to `Tier 3b (inferred): {kinds}`.
  - The generated file’s header names both catalogs. Planning fails if the ESXi catalog contains a counter absent from vCenter.
  - The planner is public and is the only planning path the emitter uses. Generated constant names, dimensions, scopes, and per-entity re-exports do not change.
  - **Acceptance**: **Given** regeneration, **When** the constants file is diffed, **Then** only documentation comments change; 353 lines end with `· ESXi 8.0.3` and 379 end with `· vCenter only`; tier counts stay 265, 96, 197, and 174; generator tests and library tests with the performance feature pass.

- **MCP-FR-03**: The documentation server’s build must notice a stale embedded index, and must ignore unindexed trees.
  - It rebuild-triggers on the vCenter catalog, the ESXi catalog, and the generated constants file, and treats each as stale when newer than the embedded index.
  - It skips directories named `perf-catalog` and `vtui` when scanning example sources, matching the example indexer.
  - **Acceptance**: **Given** a fresh index, **When** the server is built with no input changes, **Then** it reports the index up to date. **When** any of the three files is newer, **Then** the build stops and says the index is out of date. **When** only those skipped example trees change, **Then** the build does not stop.

- **MCP-FR-04**: Lookup must resolve a counter by primary key or by an unambiguous alias, in order, after trimming.
  1. Exact primary key.
  2. Exact case-sensitive alias. Aliases are the canonical id, the bare wire name (including the `.absolute` suffix for key 148), `{module}::{IDENT}`, `metrics::{module}::{IDENT}`, `perf::metrics::{module}::{IDENT}`, and the full `vim_rs::perf::metrics::{module}::{IDENT}` path.
  3. The same alias lowercased, only when exactly one canonical key matches. Otherwise no document.
  - A separate query returns every canonical key when the lowercased alias is ambiguous, and an empty list otherwise.
  - **Acceptance**: **Given** all 732 counters, **When** the lookups in Story 1 run, **Then** `mem.swapIn.average` and `mem.swapin.average` resolve to different counters; `metrics::mem::swap_in_average`, `metrics::mem::swapin_average`, and `metrics::management_agent::swap_in_average` each resolve to the right counter; `COUNTER::CPU.USAGE.AVERAGE` resolves; `MEM.SWAPOUT.NONE` returns nothing and its candidate list is exactly the two `swapOut` / `swapout` none ids; `USAGE_AVERAGE` returns nothing.

- **MCP-FR-05**: Search must gain a counters filter without a new tool.
  - The filter value in tool input is `counters`. The item type it matches is `counter`.
  - The search tool’s description mentions performance counters.
  - The schema-driven web UI shows the new value without a separate UI change.
  - **Acceptance**: A hermetic test asserts the filter serializes as `counters` and matches item type `counter`. The model-backed check in MCP-FR-06 confirms a counters-only search returns only counters.

- **MCP-FR-06**: Counter embedding text must lead with meaning, then identity, and must not displace existing searches.
  - The text starts with `{label}: {summary}` plus the synonym phrase, then the wire name, group, unit, rollup, stats kind, entities, ESXi yes or no, and the Rust constant with dimension and scaled type.
  - Synonyms come from the normative table. First match wins.
  - **Acceptance**:
    1. Hermetic: embedding text for `mem.vmmemctl.average`, `cpu.ready.summation`, `cpu.latency.average`, `datastore.totalWriteLatency.average`, `virtualDisk.numberReadAveraged.average`, `disk.commandsAborted.summation`, and `mem.swapIn.average` starts with a non-empty label and summary and contains that row’s exact synonym phrase (or no IOPS phrase, for aborted commands). `mem.swapIn.average`’s label is `Swap in`.
    2. Model-backed, opt-in, against the built index: with the counters filter and limit 5, each of these queries has its counter in the top 5: “VM memory ballooning” → `counter::mem.vmmemctl.average`; “vCPU ready time percentage” → `counter::cpu.ready.summation`; “datastore write latency” → `counter::datastore.totalWriteLatency.average`; “ESXi host power consumption in watts” → `counter::power.power.average`; “virtual disk IOPS” → `counter::virtualDisk.numberReadAveraged.average`.
    3. With the default filter and limit 5, the four baseline queries recorded before any input change (“power on a virtual machine”, “retrieve virtual machine mac address”, “vSphere task progress tracker”, “create snapshot of virtual machine”) each keep their previous first result, and at least four of their five previous ids stay in the top five.

- **MCP-FR-07**: A counter’s search summary and full document must be enough to write a correct call.
  - The summary shows the heading and canonical id, the Rust constant and its metric type, the wire unit and scaled type with the full scale description, reference levels, instance support, valid entities, ESXi availability, and the summary. An unset level prints `n/a`. Instances print `aggregate & per-device` or `aggregate only`.
  - The full document adds the requirement line `*(Requires vim_rs >= 0.7 with features = ["perf"])*`, the dependency line, the import, the constant signature, the scale rule, gap handling (newest valid sample versus current slot versus a fixed window), scope and per-entity paths (tiers 1, 2, and 3a only), tier-3b widening guidance, instance support, reference levels, ESXi availability, and a filled usage snippet.
  - The snippet omits an explicit interval so automatic selection applies. Per-instance counters request every instance and read devices. CPU ready uses the raw-series ready-percentage call. A counter absent from ESXi 8.0.3 says so in a comment.
  - **Acceptance**: Hermetic renders of `cpu.usage.average`, `cpu.ready.summation`, `disk.deviceReadLatency.average`, `vcResources.buffersz.average`, and a synthetic entry with no level match Story 2. Every snippet contains `features = ["perf"]` and `use vim_rs::perf::`.

- **MCP-FR-08**: Performance-manager pages must redirect readers without hiding administrative methods.
  - The object page, the six query methods (`query_perf`, `query_perf_composite`, `query_available_perf_metric`, `query_perf_provider_summary`, `query_perf_counter`, `query_perf_counter_by_level`), and the two catalog accessors (`perf_counter`, `historical_interval`) open with an important-callout pointing at the performance client, Step 5, counter search, and the four performance examples plus the legacy contrast.
  - The five admin methods (`create_perf_interval`, `update_perf_interval`, `remove_perf_interval`, `update_counter_level_mapping`, `reset_counter_level_mapping`) open with a note that interval and level configuration stays on the raw manager.
  - The description accessor has no banner.
  - **Acceptance**: Rendering the object page, `query_perf`, `perf_counter`, `update_perf_interval`, and `description` yields important, important, important, note, and no banner.

- **MCP-FR-09**: The starter guide must teach performance as a lookup section, and its poll loop must be code that compiles in the export example.
  - The opening usage note keeps Steps 1–4 as the skim set and names Step 5 as a lookup section that requires `vim_rs` at least 0.7 and the `perf` feature.
  - Step 5 follows Step 4 and is at most 40 lines. Its ticker declaration, its missed-tick setting, and its whole incremental-fetch match expression appear character for character in the export example. The guide may show `loop` where the example shows a bounded tick loop. The tick bound and the inventory refresh exist only in the example.
  - The anti-pattern table gains the five performance rows from the source (raw query maps, a missing feature flag, a historical maximum-sample cap, using the newest valid sample as “current”, and failing the loop on one error). The search row of the tools table includes the counters filter.
  - The four existing dependency snippets move from 0.5 to 0.7 and keep the features they already had.
  - **Acceptance**: Step 5 is at most 40 lines, and the three named statements are present in the export example. The guide does not claim those statements are a verbatim copy of the example’s loop header.

- **MCP-FR-10**: Example indexing must categorize and templatize the performance examples without sweeping in unindexed trees or the legacy example’s feature flag.
  - Sources under the performance-catalog example are skipped, as the terminal UI already is.
  - The performance category is chosen when the example name contains `perf` or `metric`, and that test happens before the property-collector test. Other categories stay as they are.
  - The dependency template is `vim_rs` 0.7 with the `perf` feature only when a trimmed source line starts with `use vim_rs::perf`. Otherwise it is plain `vim_rs` 0.7. The macros crate is not a dependency. The async runtime features include macros, the multi-thread runtime, and time.
  - **Acceptance**: Indexing the examples tree yields no files from the performance-catalog example or the terminal UI; the 16 non-performance snippets keep their current categories; all five `perf_*` examples are performance; the four performance-module examples declare the `perf` feature; the legacy raw example and the 16 non-performance snippets do not.

- **MCP-FR-11**: The snippets package must contain exactly five performance programs, each opening with the title below.
  1. One-shot snapshot (`perf_metrics`): code unchanged. The one sentence that describes the live monitor as a static CPU-and-memory table is updated to describe the live monitor.
  2. Legacy raw walk (`perf_metrics_manual`): code unchanged. The first documentation line begins with `Legacy:`. The text states the contrast and the no-feature use. The existing description stays below that.
  3. Live monitor (`perf_vm_usage`): reworked. Title: “Live realtime CPU and memory monitor for every virtual machine, with sparklines.” Inventory becomes typed virtual-machine entities. Counters come from the virtual-machine per-entity listing: CPU and memory usage as six-slot sparklines, current CPU megahertz, current consumed memory, and CPU ready as a percentage of that VM’s vCPU count. A missing sparkline sample is a space. Value strings use each scaled type’s display form; megahertz is a whole number of MHz. The query is prepared once at realtime with six samples. Timing uses the realtime refresh rate, or 20 seconds if that call fails, and skips missed ticks. A failed tick is logged and the loop continues. A VM with no samples draws a blank row. Unsupported and failed statuses are shown. The run stops after `PERF_TICKS` ticks (default 3) and closes the client.
  4. Historical extract (`perf_historical`): new. Title: “Past-day 5-minute CPU and memory rollups for every virtual machine, exported as tidy rows.” Past-day interval, a 24-hour window, six-hour chunks, tidy rows that skip gaps, and warnings printed. Comments state the historical sample-cap rule and the multi-day variant.
  5. Incremental export (`perf_export`): new. Title: “Incremental realtime export of VM metrics with per-entity watermarks (no duplicates, no gaps).” Prepared realtime query of CPU usage and consumed memory, a bootstrap window of four samples, and a watermark. The loop contains the three Step 5 statements. Inventory is refreshed every two ticks in the example, with a comment that a real exporter refreshes on a longer period. The run stops after `PERF_TICKS` ticks (default 3) and closes the client.
  - The snippets package enables the async runtime’s macros, multi-thread runtime, and time features directly.
  - **Acceptance**: All snippet binaries type-check, and the five performance binaries produce no warnings. The live monitor and the export example stop after the bounded tick count and close the client.

- **MCP-FR-12**: Server help, lookup errors, and startup counts must mention counters.
  - The lookup id’s description includes a canonical counter id and a Rust alias, and uses the one-shot performance example in place of the nonexistent basic-connection example.
  - When lookup misses and the alias is ambiguous, the reply lists every candidate’s id, Rust constant, label, and unit under a “multiple counters match” heading.
  - Otherwise the not-found scan covers every key, still caps suggestions at five, and mentions both a counter id and the one-shot example.
  - The server’s tool instructions mention the counters filter and the `perf` feature.
  - Startup and embedding-build logs include the counter count.
  - **Acceptance**: The help text and the two not-found shapes match the bullets above, and a built server log line includes a counter count of 732 after the index is rebuilt.

- **MCP-FR-13**: The rebuilt embedded index and the test suites must agree.
  - Rebuilding the index produces 732 counters and the five performance examples.
  - Tests for the index model, the counter step, the example indexer, and the server pass.
  - The opt-in model-backed tests pass when the embedding model is available. They are not part of the default offline test run.
  - **Acceptance**: The default offline tests pass without network. The opt-in ranking tests pass in a run that can load the model.

- **MCP-FR-14**: Five performance problems must be scored against the documentation server on the harness in `mcp/evals/`.
  - The harness scaffold moves from `vim_rs` 0.5.0 to the in-tree crate with the `perf` feature, and its mock client matches that crate’s client trait. The in-tree crate stays `0.6.1` until the 0.7.0 publish. The crates.io pin `vim_rs` 0.7 replaces the path dependency only after that publish. The documentation server’s own image does not need a library change. Grading and the sweep use the harness’s PowerShell runners. The agent container does not receive problem files, fixtures, or reference solutions.
  - The harness learns typed entity references for mixed inventories, a mock clock based on construction time, and a performance-manager mock that serves counters, intervals, provider summaries, availability, and queries, and that records every query it receives. Historical fixtures include an enabled 300-second interval. Samples are offsets from the mock clock. Drivers must not set the client clock, because the solution builds its own client.
  - The five problems are: balloon and swap-in rate as current-slot display strings (`none` on a gap); past-day tidy rows inside an exclusive start and inclusive end, with four six-hour windows and no sample cap; a two-tick incremental watermark on a phased fixture; per-device host disk latency as a typed host query; and mixed entities under automatic interval, including historical-unsupported and not-a-provider.
  - The operator chooses the model by the harness calibration procedure. The scorecard records that model id. Both arms use it. A missing tool-call count counts as zero.
  - The scorecard’s minimum-lift check fails unless all three hold: retrieval pass rate improves by at least 0.20, compile rate does not fall, and mean tool calls do not rise. Retrieval pass means both fixtures matched. Tool calls count every tool request.
  - **Outcome on a miss**: this phase is not a release gate for the index, the examples, or the guide. The release gate is only the Release Coupling Gate. A miss is committed as a dated report with the scorecard and per-problem notes, and it opens a tuning iteration before a re-run.
  - **Acceptance**: Reference solutions pass the grader, and the existing non-performance problems still pass. The scorecard is committed. Thresholds are met, or the miss report exists and does not block release.

### Normative Mapping Tables

These tables are the behavior under test. Planning must preserve them.

#### Wire unit to document metadata (all 16 units)

Dimension comes from the planner. Scaled type and scale description are documentation-only.

| Wire unit | Dimension | Scaled type | Scale description |
|---|---|---|---|
| `percent` | Ratio | Percent | `raw (hundredths of a percent) * 0.01 -> Percent(f64) in 0.0..=100.0` |
| `kiloBytes` | Memory | Bytes | `i128::from(raw) * 1024 -> Bytes(i128) (exact integer bytes)` |
| `megaBytes` | Memory | Bytes | `i128::from(raw) * 1_048_576 -> Bytes(i128) (exact integer bytes)` |
| `gigaBytes` | Memory | Bytes | `i128::from(raw) * 1_073_741_824 -> Bytes(i128) (exact integer bytes)` |
| `teraBytes` | Memory | Bytes | `i128::from(raw) * 1_099_511_627_776 -> Bytes(i128) (exact integer bytes)` |
| `kiloBytesPerSecond` | Throughput | BytesPerSecond | `raw as f64 * 1024.0 -> BytesPerSecond(f64)` |
| `megaBytesPerSecond` | Throughput | BytesPerSecond | `raw as f64 * 1_048_576.0 -> BytesPerSecond(f64)` |
| `megaHertz` | Frequency | Hertz | `raw as f64 * 1_000_000.0 -> Hertz(f64) (use .as_mhz() for MHz)` |
| `nanosecond` | Time | Seconds | `raw as f64 * 1e-9 -> Seconds(f64) (use .as_millis() for ms)` |
| `microsecond` | Time | Seconds | `raw as f64 * 1e-6 -> Seconds(f64) (use .as_millis() for ms)` |
| `millisecond` | Time | Seconds | `raw as f64 * 1e-3 -> Seconds(f64) (use .as_millis() for ms)` |
| `second` | Time | Seconds | `raw as f64 -> Seconds(f64)` |
| `watt` | Power | Watts | `raw as f64 -> Watts(f64)` |
| `joule` | Energy | Joules | `raw as f64 -> Joules(f64)` |
| `celsius` | Temperature | Celsius | `raw as f64 -> Celsius(f64)` |
| `number` | Count | Number | `raw as f64 -> Number(f64)` |

#### Scope mask to scope name and valid entities

The scope name equals the planner’s scope type. For the 12 named aliases, that is the type text in the generated constants. For any other mask, the scope type is the generic six-flag form and the emitter, not the planner, adds the entity-module path when writing the constants file. No current counter needs that fallback.

| Scope mask | Scope name | Valid entities |
|---:|---|---|
| `0x01` | VmOnly | VirtualMachine |
| `0x02` | HostOnly | HostSystem |
| `0x03` | VmOrHost | VirtualMachine, HostSystem |
| `0x06` | HostCluster | HostSystem, ClusterComputeResource |
| `0x07` | VmHostCluster | VirtualMachine, HostSystem, ClusterComputeResource |
| `0x09` | VmOrPool | VirtualMachine, ResourcePool |
| `0x0C` | ClusterPool | ClusterComputeResource, ResourcePool |
| `0x0F` | ComputeScope | VirtualMachine, HostSystem, ClusterComputeResource, ResourcePool |
| `0x10` | DatastoreOnly | Datastore |
| `0x11` | DatastoreVm | VirtualMachine, Datastore |
| `0x13` | DatastoreVmHost | VirtualMachine, HostSystem, Datastore |
| `0x24` | ClusterDc | ClusterComputeResource, Datacenter |
| any other M | `Scope<{vm}, {host}, {cluster}, {pool}, {datastore}, {datacenter}>` with true or false per bit | ordered subset of VirtualMachine, HostSystem, ClusterComputeResource, ResourcePool, Datastore, Datacenter for bits 0 through 5 |

#### Domain synonyms for embedding text

First match on `(group, name)` wins. The phrase is appended inside the leading label sentence. An empty phrase appends nothing.

| Match | Appended phrase |
|---|---|
| group `mem`, name starts with `vmmemctl` | ` (memory balloon, ballooning, vmmemctl, balloon driver)` |
| group `cpu`, name `ready` | ` (CPU ready time, vCPU ready percentage, scheduling wait, CPU contention)` |
| group `cpu`, name `costop` | ` (CPU co-stop, SMP scheduling skew, multi-vCPU contention)` |
| group `cpu`, name `demand` or `entitlement` | ` (CPU demand, entitlement, active CPU load)` |
| group `cpu` or `mem`, name `latency` or starting with `latency.` | ` (CPU/memory contention latency, resource wait time, scheduling or memory access latency)` |
| group `mem`, name `swapinRate` | ` (memory swap-in rate, host swap in KB/s, active swap pressure)` |
| group `mem`, name `swapin` or `swapIn` | ` (memory swapped in, cumulative swap-in amount, host swap)` |
| group `mem`, name starts with `swapout` or `swapOut`, or name `swapped` | ` (memory swap-out, swapped memory, host swap)` |
| group `mem`, name `consumed`, `active`, or `granted` | ` (memory usage, host memory consumed, active guest memory)` |
| group `mem`, name `overhead` | ` (hypervisor memory overhead, VMX overhead)` |
| group `mem`, name starts with `llSwap` | ` (host cache swap, low-latency SSD swap)` |
| group is a storage group (`datastore`, `disk`, `virtualDisk`, `storageAdapter`, `storagePath`, `nfs`, `vvol`, `vsanDomObj`, `hbr`) and name contains `Latency` or `latency` | ` (I/O latency, response time, storage wait)` |
| group `virtualDisk`, name contains `ReadAveraged` or `WriteAveraged` | ` (virtual disk IOPS, IOPS, I/O operations per second, read/write rate)` |
| group is `datastore`, `disk`, `virtualDisk`, `storageAdapter`, or `storagePath`, and name contains `ReadAveraged` or `WriteAveraged`, or name starts with `commands` and does not end with `Aborted` or `Resets` | ` (IOPS, I/O operations per second, read/write rate)` |
| group `net`, name `usage`, `received`, or `transmitted` | ` (network throughput, bandwidth, NIC traffic rate)` |
| group `disk`, `datastore`, or `virtualDisk`, and name `usage`, `read`, or `write` | ` (storage throughput, disk bandwidth, read/write KB/s)` |
| group `power`, name `power` or `energy` | ` (host power consumption, watts, energy draw)` |
| otherwise | empty |

### Key Entities

- **Counter document**: One vCenter 8.0.3 counter as an assistant-facing record. Canonical id, numeric key (reference only; keys differ per server), wire name, group, name, rollup, stats kind, unit, human label and summary, optional reference levels, per-instance flag, ESXi availability, Rust module and constant, dimension, scaled type, scale description, scope mask, scope name, derivation tier, and the entity kinds that scope admits.
- **Counter alias index**: Exact and lowercased aliases for those documents. Built when first needed. Not stored in the embedded index. Ambiguous lowercase aliases yield a candidate list instead of a document.
- **Scope tier**: How the compile-time scope was derived. Direct observation (265), sibling rollup (96), group union (197), or inferred (174). Inferred counters are absent from per-entity listings.
- **Search filter**: The existing type filter, plus counters. Default search still includes counters.
- **Performance example**: One of the five indexed programs, category performance, with a dependency template that advertises the feature flag only when the program imports the performance module.
- **Starter-guide step**: The lookup section that points at the four workflows and the legacy contrast, and whose poll statements are compiled inside the export example.
- **Read versus admin callout**: The redirect on performance-manager query pages, and the neutral note on interval and level mutations.
- **Evaluation problem**: One of the five scored tasks. Two fixtures, a reference solution, and a last-line result the grader compares. Not a release gate.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: An assistant searching “VM memory ballooning”, “vCPU ready time percentage”, “datastore write latency”, “ESXi host power consumption in watts”, or “virtual disk IOPS”, with the counters filter and five results, finds the specified counter every time, and every result is a counter.
- **SC-002**: The index contains exactly 732 counter documents, 353 marked available on ESXi 8.0.3 and 379 marked vCenter-only, with 732 unique ids and no empty label or summary.
- **SC-003**: Exact lookup separates all eight case-only collision pairs. A case-insensitive query that matches both members of a pair returns no document and lists both. A bare repeated identifier returns no document. A module-qualified alias returns the one counter it names.
- **SC-004**: Regenerating constants changes only documentation comments. Tier counts remain 265, 96, 197, and 174. ESXi markers are 353 and 379.
- **SC-005**: For the four recorded non-performance queries, the previous top result stays first, and at least four of the five previous top results stay in the top five.
- **SC-006**: The starter guide’s performance section is at most 40 lines, and an assistant can copy its poll statements into the export example without editing those statements.
- **SC-007**: Five performance examples compile with no warnings. Four that import the performance module declare `vim_rs` 0.7 and the `perf` feature. The legacy raw example and the 16 non-performance examples do not. The live monitor and the exporter stop after a bounded number of ticks and close the client.
- **SC-008**: The assistant tool count stays at 6. A server built from an unchanged index reports the index up to date. A newer catalog or constants file stops the build until the index is rebuilt.
- **SC-009**: On the five-problem evaluation, with three repeats and one model recorded in the scorecard, retrieval pass rate with the server on is at least 0.20 higher than with it off, compile rate does not fall, and mean tool calls do not rise. If that misses, a dated report exists and release of the index is still allowed once `vim_rs` 0.7.0 is published.
- **SC-010**: Every counter document, redirect banner, starter-guide performance section, and performance-module example tells the reader that `vim_rs` 0.7 and the `perf` feature are both required.

## Assumptions

- The companion performance module from `specs/006-perf-manager-api` is the runtime this feature documents. Its public query, scope, and scaling behavior is not reopened here. In-tree `vim_rs` is 0.6.1; the release that contains the module is 0.7.0. `Error` becoming `Clone` is the breaking change that forces 0.7.0. The `perf` feature stays off by default, and `perf-chrono` and `perf-polars` imply it.
- Catalogs are the sanitized vCenter 8.0.3 catalog already used by the generator (732 counters) and the ESXi 8.0.3 fixture (353 counters). All 353 ESXi counters match a vCenter counter on group, name, rollup, and stats kind. The ESXi file moves into the generator data set as a byte copy of that fixture.
- Scope tiers stay 265 / 96 / 197 / 174. All 12 named scope aliases are in use. No counter needs the generic scope fallback today.
- The documentation server has six tools today. Lookup is exact-key only. Search has eight filters. The not-found scan stops after 1,000 keys. Help text cites an example id that does not exist. Those are the defects this feature closes; the tool list itself does not grow.
- Sixteen non-performance snippets keep their categories. Three performance snippets already exist and are edited only as this spec says. Two new snippets are added. No sixth performance example is added.
- The embedding model is the one the server already ships. Ranking tests that load it are opt-in and may download or reuse a local cache. Default tests stay offline.
- Baseline top-five ids for the four non-performance queries are recorded before any input to the index changes, and that record is kept with the server tests.
- The evaluation harness is in `mcp/evals/`. Its scaffold still pins `vim_rs` 0.5.0 until the five-problem slice retargets it. That slice follows the harness’s layout, PowerShell grader, and arm definitions in [contracts/evals.md](./contracts/evals.md).
- Display strings in the evaluation are the scaled types’ ordinary display: percents as two decimals, bytes and byte rates in binary IEC units, seconds by magnitude.
- Session credentials are never logged. The documentation server does not connect to a live vCenter.

## Out of Scope

- New assistant tools, or a live connection from the documentation server to vCenter or ESXi.
- Changes to generated constant names, dimensions, scopes, per-entity re-exports, or to the performance module’s runtime behavior.
- A counter JSON artifact committed under the generator data directory.
- Indexing performance-module structs and methods through the OpenAPI extractor.
- Additional examples beyond the five named here, including a separate poller example or a separate typed-scope example.
- Releasing the documentation server before `vim_rs` 0.7.0 is published.
- Treating a missed evaluation threshold as a reason to hold that release.
