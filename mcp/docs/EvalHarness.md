# EvalHarness — Phase 1 design (PropertyCollector proving slice)

> **Status:** Implementation-ready design for Phase 1 of the effort in `GraphSearch.md`
> (strategy there; this is the *how*). **This doc is authoritative for Phase 1.** The
> first slice is **PropertyCollector** (per Kiril); the Event/MethodFault variant that an
> earlier draft used is demoted (weak signal — its field data is a flat `api_field_registry`
> lookup) and the ~20 Java-SDK problems belong to later phases. **[VERIFY]** = external fact
> to confirm; **[DECIDE]** = open choice.

---

## 1. What Phase 1 proves

**Method:** A/B inside a real coding harness (**Goose**), same model, same task:
- **Arm A (baseline):** Goose + `developer` builtin (shell/rg/file over the crate source),
  **no MCP**.
- **Arm B (treatment):** Arm A **+ the vim-rs MCP** extension (its *existing* property tools:
  `list_property_collector_root_types`, `get_property_path`, `get_property_tree`, plus
  `search`/`get` with the frozen access-paths).
- **Arm C (optional confound control):** Arm A + a one-line "property paths are awkward to
  derive from source; look carefully" hint, **no MCP**. Separates the hint from the tool.
  Run only if A/B shows lift.

Instructions are **identical across arms** except the extension list (and, for C, one hint
line). Any delta is then attributable to tool surface, not prompt wording.

### 1.1 Hypothesis (honest, testable)

The field graph *is* in the crate source (`structs.rs`) — so this is **not** an information
monopoly. What is genuinely hard for grep/LSP is **synthesizing a correct PropertyCollector
property path**:
- the **camelCase VIM spelling** (`config.hardware.numCPU`) vs. the snake_case Rust fields
  the agent sees in source (`config.hardware.num_cpu`);
- **which managed-object root** reaches the target;
- **crossing arrays** and applying **downcasts** (`config.hardware.device[*]→VirtualEthernetCard.macAddress`);
- **PropertyCollector-legality** of the path.

Assembling that over 473k lines of generated code is a multi-hop synthesis, not a lookup —
a genuinely harder task than the flat Event/Fault registry lookup. The MCP hands the path
over directly.

> **What this slice actually measures:** the **existing** MCP property tools vs. baseline.
> That's deliberate — it establishes the **bar Phase 3 must beat**. The current tools cover
> only the ~234-type inventory closure and **stop at array boundaries** (GraphSearch §2.4),
> so a polymorphism/downcast task (`pc03`, redesigned per §5.2) is the discriminating probe:
> the agent must write the array-iterate + trait-downcast navigation, which the MCP's `→`
> shorthand encodes and grep struggles with. pc01/02/04 are grep-solvable negative controls.

**Corrected expectation:** lift in **correctness** (right paths → right values) and/or
**efficiency** (fewer exploration turns/tokens). A weak result is informative, not fatal.

**Primary metric:** two-tier grade (§3) + efficiency (turns, tool calls, tokens, wall-clock).
Not pass/fail alone — a capable harness may saturate pass/fail (GraphSearch §7.9).

---

## 2. The PropertyCollector shape (grounded in the code)

From `examples/snippets/src/env_browser.rs` and `vim_rs/core/pc_retrieve`:
- `vim_retrievable!(struct T: <ManagedObject> { field = "vim.path", … })` declares a struct
  whose fields map to **VIM property-path strings**; `ObjectRetriever` fetches instances via
  PropertyCollector and populates them (`obj.id`, `obj.<field>`).
- The **path strings are the answer** and the MCP-unique skill. The Rust field types are
  chosen by the author, but the strings must be valid VIM paths to the right leaves.
- `vim_rs/tests/support/mock_vim_client.rs` (`MockVimClient`) scripts PropertyCollector:
  it dispatches on VI-JSON URL layout and returns canned `WaitForUpdatesEx` bytes
  (`PcEvent::Bytes`). We seed it with a canned `ObjectContent` whose `propSet` is keyed by
  the **correct** paths → this is our offline grading oracle.

**Grading insight:** compilation checks that each field's Rust type is coherent, but a wrong
path whose leaf type *happens* to match the declared field type **still compiles**. So
compile-only under-measures → we add a runtime mock-retrieval tier (§3).

---

## 3. Grading: two tiers + anti-cheat (offline, no vCenter)

- **Tier 1 — Compiles (gate).** `cargo build` the agent's `solve.rs` against real `vim_rs`.
- **Tier 2 — Mock retrieval (correctness).** A **sealed** `main` builds a `MockVimClient`
  seeded with a canned `WaitForUpdatesEx`/`ObjectContent` whose `propSet` is keyed by the
  **correct** paths, retrieves `Vec<Target>` via `ObjectRetriever`, and asserts the field
  values equal the expected. **Wrong paths → field unpopulated (`None`) → fail** — this is
  what catches an incorrect-but-compiling path.

**Anti-cheat:**
1. **Fixed struct contract.** The task pins the `vim_retrievable!` struct **name, target MO,
   and field names/types**; the agent supplies **only the RHS path strings**. So the answer
   space is exactly "the correct paths," and the sealed `main` can name `Target` to retrieve
   and assert it.
2. **Multi-fixture grading.** Each problem carries **2–3 canned responses with different
   values**; `solve` is graded on all → defeats hardcoding a return literal.
3. **Oracle is hidden.** The correct-keyed `propSet` lives inside the **sealed harness**, not
   in the agent's workspace — the agent cannot read the answer, only produce paths that must
   match it at retrieval time.
4. **Seal the scaffold.** Agent edits **only `src/solve.rs`**; before grading the runner
   **restores every other file** from a read-only canonical copy and hash-flags any
   non-`solve.rs` change as `tamper`.
5. **Fresh values.** Don't reuse values from shipped tests — `vim_rs`'s `Cargo.toml`
   `exclude` does **not** drop `tests/` (verified), so the published crate's test data is
   `rg`-able by Arm A. Author novel `propSet` values.

**Failure class (honest scope for iter-1):** `{ compile_error, retrieval_fail, timeout,
no_output, tamper }`. Finer classes (wrong-camelCase vs wrong-parent vs missing-downcast)
would need per-field assertions — **[DECIDE]** whether to add.

> Offline works: `MockVimClient` needs no endpoint; it replays canned PropertyCollector
> bytes. No vCenter for Phase 1.

---

## 4. Architecture (per-run flow) and fixed container layout

Canonical paths (diagram, runner, Containerfile agree):
- `/work` — the scaffold cargo crate; only `/work/src/solve.rs` is writable in spirit
  (enforced by post-run reset).
- `/eval` — read-only inputs: `recipes/`, `problems/`, `fixtures/` (canned responses), and
  the canonical scaffold copy for reset.
- `/out` — per-run outputs: `trace.jsonl`, `build.log`, `result.json`, `solve.rs`.
- `~/.cargo/registry` — warmed with the `vim_rs` **source** so Arm A's `rg` is honest.

```
runner (host)                          podman container (ephemeral, per problem × arm × repeat)
for problem, arm, repeat:               ┌──────────────────────────────────────────────┐
  podman run --network … \              │ /work  scaffold (agent edits src/solve.rs)     │
     -e OPENROUTER_API_KEY \            │ /eval  recipes+problems+fixtures+canonical     │
     -e GOOSE_MODEL=$MODEL …            │ /out   trace/build/result                      │
   (1) goose run --recipe /eval/... ───▶│ ~/.cargo/registry = vim_rs SOURCE (rg-able)    │
       --output-format stream-json      │ /opt/vim_mcp_server (arm B/C image; unused A)  │
   (2) reset non-solve files            │ CARGO_NET_OFFLINE=true (build offline)         │
   (3) cargo build            (Tier 1)  │ env: GOOSE_MODE=auto, MAX_TURNS, temp=0        │
   (4) run sealed harness (Tier 2,      └──────────────────────────────────────────────┘
       all canned responses)
   (5) collect metrics + trace
aggregate → scorecard
```

Rootless podman; fresh container per run; runs independent → parallelizable with a
concurrency cap (§5.5, OpenRouter rate limits).

---

## 5. Components

### 5.1 Scaffold crate (`mcp/evals/scaffold/`)

- `Cargo.toml` → `vim_rs = { version = "0.5", features = ["xml"] }` (pin exact version;
  **[VERIFY]** whether a task needs another feature). The mock lives in
  `vim_rs/tests/support/` — **[DECIDE]** vendor a copy into the scaffold or depend on it.
- `src/main.rs` — **sealed**, per-problem, restored before grading. Builds the mock, seeds
  it with the run's canned response, retrieves the agent's `solve::Target`, asserts, prints
  a JSON result line for the runner. Sketch:
  ```rust
  mod solve;                                  // agent-owned; declares vim_retrievable! Target
  use vim_rs::core::pc_retrieve::ObjectRetriever;
  #[tokio::main] async fn main() {
      let client = mock_seeded_from(std::env::var("FIXTURE").unwrap());  // canned propSet
      let objs: Vec<solve::Target> =
          ObjectRetriever::new(client).unwrap()
              .retrieve_objects_from_container(&root).await.unwrap();
      println!("{}", serde_json::to_string(&extract(&objs)).unwrap());   // Tier-2 compares
  }
  ```
  **[VERIFY]** exact `ObjectRetriever` entry point + how to seed `MockVimClient` with a
  container/root + one canned `WaitForUpdatesEx` payload.
- `src/solve.rs` — the **only** agent-writable file. Contains the pinned contract with blank
  paths, e.g.:
  ```rust
  vim_rs::vim_retrievable!(struct Target: VirtualMachine {
      power_state = "TODO",     // agent fills the VIM path
      guest_host  = "TODO",
  });
  ```

### 5.2 Problem schema (`mcp/evals/problems/*.yaml`)

```yaml
id: pc01_vm_power_and_host
pillar: propcollector
goal: |                          # NL only — never give the path or camelCase spelling
  For each virtual machine, retrieve its current power state and the guest OS hostname.
contract: |                      # pinned struct the agent must complete (fields fixed)
  vim_retrievable!(struct Target: VirtualMachine {
      power_state: VirtualMachinePowerState = "<path>",
      guest_host:  Option<String>           = "<path>",
  });
fixtures:                        # ≥2 canned PropertyCollector responses, novel values
  - { file: fixtures/pc01_a.json, expected: [{power_state: "poweredOn",  guest_host: "web01"}] }
  - { file: fixtures/pc01_b.json, expected: [{power_state: "poweredOff", guest_host: null   },
                                             {power_state: "poweredOn",  guest_host: "db02"}] }
notes: |                         # graders only
  Correct paths: "runtime.powerState", "guest.hostName" (camelCase). [VERIFY against field graph]
```

**Slice set (~4):**
1. `pc01` VM `runtime.powerState` + `guest.hostName` — nested/optional. ⚠ **weak:** see the
   §1.1 correction — `vim_retrievable!` takes snake_case and converts to camelCase, so there
   is no spelling gap; the agent reads the field name straight off the struct. Both arms pass.
2. `pc02` VM name + `config.hardware.numCPU` — same weakness; snake_case `num_cpu` in source.
3. `pc03` **full-navigation probe (redesigned).** Not a single path — the agent supplies the
   path **and implements `ethernet_macs(t)`**: iterate `Vec<Box<dyn VirtualDeviceTrait>>`,
   `CastInto::into_ref::<dyn VirtualEthernetCardTrait>()` per device, filter non-ethernet,
   read `mac_address`. Fixtures **mix `VirtualCdrom` + `VirtualE1000` + `VirtualVmxnet3`** so
   that: reading a base field won't compile, matching one concrete type misses the other NIC
   subtype, and no filtering yields wrong output. This is the polymorphism/downcast skill
   grep struggles with and that the MCP shorthand encodes as
   `config.hardware.device[*]→VirtualEthernetCard.macAddress`. **Offline-validated to
   discriminate:** reference solution passes; concrete-`E1000`-only → `retrieval_fail` (misses
   Vmxnet3); wrong path → `compile_error`. The sealed emit only calls the agent's function.
4. `pc04` HostSystem `summary.overallStatus` + `summary.config.product.fullName` — weak, as 1/2.

> pc01/02/04 remain as **negative controls** (grep-solvable → expect A≈B). pc03 is the
> discriminating probe. The `render_recipe` instructions were generalized to "fill path
> strings **and implement any function stubs**" so pc03's function is legitimate and the arms
> stay identical. Fixtures are built by the host-only `mcp/evals/fixture_gen` package from real vim_rs
> types (guaranteed round-trip); regenerate with `cargo run --manifest-path mcp/evals/fixture_gen/Cargo.toml`.
> That generator is not copied into the agent scaffold or the eval image.

### 5.3 Goose recipes (`mcp/evals/recipes/`)

Grounded field names (`settings.goose_provider/goose_model/temperature/max_turns`; stdio
extensions `type/name/cmd/args/timeout`; Jinja `{{ }}`; `type: builtin` for `developer`).

**`mcp_on.yaml` (Arm B):**
```yaml
version: "1.0.0"
title: vim_rs eval
description: Complete the vim_retrievable! contract in src/solve.rs.
instructions: |                  # IDENTICAL across arms (no MCP coaching) — anti-confound
  You write Rust using the `vim_rs` crate (a vSphere API client). The project is in the
  working directory. Fill in the property-path strings in the `vim_retrievable!` contract in
  src/solve.rs so `cargo build` succeeds and the program prints the correct values. Use
  whatever tools are available to find the correct VIM property paths.
prompt: |
  {{ goal }}

  Complete exactly this contract (fill only the path strings):
  {{ contract }}
parameters:
  - { key: goal,     input_type: string, requirement: required, description: task }
  - { key: contract, input_type: string, requirement: required, description: struct to complete }
  - { key: model,    input_type: string, requirement: required, description: model id }
extensions:
  - { type: builtin, name: developer }
  - { type: stdio, name: vim-rs, cmd: /opt/vim_mcp_server, args: [], timeout: 300,
      description: vim_rs vSphere API assistant (property-collector + search tools) }
settings:
  goose_provider: openrouter
  goose_model: "{{ model }}"       # [VERIFY] id + tool-calling reliability
  temperature: 0
  max_turns: 40
```

**`mcp_off.yaml` (Arm A):** identical **minus the `vim-rs` extension**. **`mcp_off_hinted.yaml`
(Arm C, optional):** Arm A + one hint line.

### 5.4 Container (`mcp/evals/Containerfile`)

Multi-stage — the MCP binary **must be Linux** (host build is darwin, ~238 MB, won't run in
the container):
```
FROM docker.io/library/rust:1.95 AS build
COPY mcp/ /src/mcp ; WORKDIR /src/mcp
RUN cargo build --release -p vim_mcp_server        # → target/release/vim_mcp_server

FROM docker.io/library/rust:1.95
COPY --from=build /src/mcp/target/release/vim_mcp_server /opt/vim_mcp_server
COPY mcp/evals/scaffold /eval/scaffold-canonical
COPY mcp/evals/{recipes,problems,fixtures} /eval/
RUN cp -r /eval/scaffold-canonical /work && cd /work && cargo fetch   # warm vim_rs source
# install + PIN goose  [VERIFY install cmd + pin version]
ENV GOOSE_MODE=auto GOOSE_DISABLE_SESSION_NAMING=true GOOSE_MAX_TURNS=40 CARGO_NET_OFFLINE=true
WORKDIR /work
```
`OPENROUTER_API_KEY`, `GOOSE_PROVIDER=openrouter`, `GOOSE_MODEL` passed at `podman run`.
**[DECIDE]** embed-model (self-contained `search`) vs mounted model-cache volume — `pc*` tasks
lean on `get_property_path`/`get_property_tree` (no embeddings), but `search` needs the model;
keep it available so Arm B can also discover by concept.

### 5.5 Runner (`mcp/evals/runner/`) — **[DECIDE]** bash vs Python (bash suffices)

Per `(problem, arm, repeat)`:
1. `podman run --rm --network=… -e OPENROUTER_API_KEY -e GOOSE_MODEL=$MODEL -v $OUT:/out …
   goose run --recipe /eval/recipes/<arm>.yaml --params goal="…" --params contract="…"
   --params model="$MODEL" --no-session --output-format stream-json` **[VERIFY flag]** →
   `/out/trace.jsonl`.
2. **Reset** `/work` to canonical **except `src/solve.rs`**; hash-diff → `tamper`.
3. `cargo build` → Tier 1.
4. For **each** canned response: set `FIXTURE`, run the binary, compare stdout JSON to
   `expected` → Tier 2 (pass only if all pass).
5. Emit `result.json` (§5.6).

Concurrency cap (OpenRouter rate/cost); retry on 429/5xx with backoff; **cost ceiling** env
that aborts past a threshold. Network: allow OpenRouter egress only; cargo offline so the
agent can't add phone-home deps and must use `vim_rs`.

### 5.6 Telemetry & metrics — **corrected claim**

The MCP does **not** currently log per-call query/args/result IDs (only startup to stderr).
So:
- **Primary Phase-1 telemetry = Goose `stream-json` trace**: turns, tool calls (name + args
  — including *which property tool* Arm B used), timing, token usage **[VERIFY tokens present;
  else read OpenRouter usage]**.
- **Optional small Phase-1 build task:** add per-call JSONL logging in the MCP tool handlers
  → clean, harness-independent query telemetry and our first dataset (GraphSearch §7.1). Do
  this **or** rely on the trace; don't claim it exists.

Per-run record:
```json
{ "problem":"pc01","arm":"B","repeat":1,"compiled":true,"retrieval_pass":true,
  "failure_class":null,"tamper":false,"turns":5,"tool_calls":7,"vim_rs_tool_calls":3,
  "tokens_in":7400,"tokens_out":520,"wall_s":33.1 }
```

### 5.7 Scorecard (`mcp/evals/results/`, gitignored)

A vs B (and C if run): retrieval-pass rate, compile rate, mean turns/tokens/tool-calls, per
problem, with variance across repeats.

**"Clear lift" rule (pragmatic, small-N):** with `N` repeats × `P` problems, treat lift as
real only if **B ≥ A on retrieval-pass in every repeat** *and* median `P_pass(B) − P_pass(A)
≥ 2` problems *and* a bootstrap CI on the per-run delta excludes 0. Otherwise **inconclusive**
→ investigate (model band? tools not surfacing paths? array gap dominating?). **[DECIDE N]**
(suggest 3–5).

---

## 6. Directory layout

```
mcp/evals/
├── README.md
├── Containerfile
├── scaffold/               # Cargo.toml, src/main.rs (sealed, per-problem), src/solve.rs (contract stub)
├── fixtures/               # fresh canned PropertyCollector responses (≥2 per problem)
├── problems/*.yaml         # goal + contract + fixtures[]/expected[] + notes
├── recipes/{mcp_on,mcp_off,mcp_off_hinted}.yaml
├── runner/                 # orchestration + reset/seal + grader + aggregation
└── results/                # traces, scorecards (gitignored)
```

---

## 7. Verify-before-build checklist

- **[VERIFY]** Goose machine-trace flag (`--output-format stream-json`?) and whether token
  usage is in it.
- **[VERIFY]** OpenRouter model id (DeepSeek candidate) **and reliable tool-calling** — run
  the §7.9 calibration (one weaker + one stronger model, 2–3 tasks) before the sweep.
- **[VERIFY]** Goose install/pin command; pin a version.
- **[VERIFY]** `ObjectRetriever` API + how to seed `MockVimClient` with a root/container and a
  single canned `WaitForUpdatesEx` payload keyed by property path; whether the mock needs a
  `ServiceContent`/root fixture. Vendor vs. depend-on `tests/support/mock_vim_client.rs`.
- **[VERIFY]** each task's target paths resolve in the field graph before locking it.
- **[VERIFY]** scaffold builds with `features=["xml"]`; whether property retrieval works
  under the JSON client mock without `xml` (drop the feature if so).
- **[DECIDE]** runner language; N repeats; finer `failure_class`; add MCP-side JSONL logging
  in Phase 1 or not; embed-model vs mounted cache.

---

## 8. Exit criteria (decision gate)

- **Clear lift** (§5.7) → build Phase 3 proper: the `build_retrieval` tool + array-crossing /
  downcast navigation (which the gap probe `pc03` shows current tools miss); reuse this rig
  for Phase 3′ (metrics) and later slices.
- **Inconclusive/no lift** → diagnose before building: is the model too weak/strong (§7.9),
  do the current property tools actually surface camelCase paths, is the array gap dominating
  (in which case *that's* the Phase 3 target)? This slice measures the **current** tools, so a
  null result points at what Phase 3 must add, not that the thesis is wrong.
- Either way we exit with: a reusable, sealed, containerized A/B rig; first real MCP query
  telemetry; a calibrated model choice; and a baseline number Phase 3 must beat.
