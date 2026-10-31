# Research: MCP Performance Counters

**Feature**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

**Date**: 2026-10-25. Decisions below close Part II of `docs/vim_rs-perf-mcp.md` (v4.2) against this tree (`vim_rs` 0.6.1, `rust-version = "1.86"`, branch `007-mcp-perf-counters`). No item is left as NEEDS CLARIFICATION.

## R1 — Counter documents are an MCP pipeline product

**Decision**: New crate `mcp/data_processing/build_counters`, package name `build-counters`, member of the `mcp/` workspace. `collect_counters` reads `vim_build/data/perf-catalog.json` and `vim_build/data/esxi-perf-catalog.json`, calls `vim_build::perf_metrics::plan_counters`, and maps each `PlannedCounter` into a `CounterEntry`. `data-transformer` calls `collect_counters` in memory and writes `counters.json` only on `--emit-json`.

**Rationale**: The 732 constants are not in the OpenAPI spec, so `build_api_definitions` cannot see them. The planner already disambiguates flat identifiers (`zip.saved` / `zipSaved`) and the SCSI `_ABSOLUTE` suffix. Calling it keeps one code path. Confirmed on this tree: `emit_perf_metrics(catalog, output)` takes no ESXi path, and there is no `plan_counters` yet. The MCP workspace members are `server`, `build_api_definitions`, `build_embeddings`, `build_examples`, and `data_transformer`.

**Alternatives considered**:

- `vim_build` writes `perf-metrics-index.json`. Rejected. Couples the core generator to MCP, and the library never needed a sidecar (MCP-ADR-03).
- Re-derive identifiers and scopes inside `mcp/`. Rejected. The two derivations will drift.
- Parse `metrics.rs`. Rejected. rustfmt wrapping changes the parse.

## R2 — `api_database` does not depend on `vim_build`

**Decision**: `ScopeTier` and `CounterEntry` are owned by `api_database`. `build_counters` maps `PlanTier` onto `ScopeTier` one-to-one. `api_database/Cargo.toml` gains no `vim_build` dependency.

**Rationale**: The server deserializes `api_database` types out of the embedded bin. Linking `vim_build` (OpenAPI parser and generator) into that crate would pull the generator into the server build. Pipeline compile cost of `vim_build` stays on `data-transformer` only, which already sits outside the `vim_rs` compile budget (Principle III).

**Alternatives considered**:

- Share `PlannedCounter` as the stored document. Rejected. That type is a generator result (`&'static str` dimension, no labels, no scale description) and would make `api_database` depend on `vim_build`.

## R3 — ESXi availability is rustdoc plus a boolean

**Decision**: Commit `vim_build/data/esxi-perf-catalog.json` as a byte copy of `vim_rs/tests/fixtures/perf/esxi_catalog.json`. A `vim_build` unit test asserts the bytes match, the same way the vCenter pair is checked. `plan_counters` sets `esxi_supported` when `(group, name, rollup, stats)` is in the ESXi catalog and returns an error if ESXi lists a counter vCenter does not. The emitter appends `· ESXi 8.0.3` or `· vCenter only`. Expected split: 353 and 379. Tier counts stay 265 / 96 / 197 / 174. Tier 3b wording becomes `Tier 3b (inferred): {kinds}`.

**Rationale**: The ESXi catalog is a strict subset, so it adds no constants or scopes (MCP-ADR-06). Documentation is the only `vim_rs` diff, which keeps Principle I and the binary-size budget.

**Alternatives considered**:

- Generate a second ESXi constant set. Rejected. Every ESXi counter already has a vCenter constant.
- Leave ESXi only in the MCP documents. Rejected. Readers of `metrics.rs` rustdoc would not see host availability.

## R4 — No new MCP tool; counters are a search filter

**Decision**: Add `SearchFilter::Counters`, serialized `counters`, `as_str()` → `counter`. Search already drops items whose `item_type()` is not `filter.as_str()` when the filter is not `All` (`mcp/server/src/main.rs`). The web UI is schema-driven, so the new enum variant appears without a UI change. Tool count stays 6.

**Rationale**: Assistants already follow starter guide → search → get (MCP-ADR-01). A seventh tool competes with the existing ones. Confirmed: `SearchFilter` today is `All`, `ManagedObjects`, `Methods`, `Structures`, `Enums`, `Fields`, `Examples`, `Traits`.

**Alternatives considered**:

- A dedicated `find_counter` tool. Rejected. Extra tool, same retrieval.

## R5 — Alias index is lazy and not stored

**Decision**: `ApiDatabase` gains `#[serde(skip)] pub counter_aliases: OnceLock<CounterAliasIndex>`. `data_transformer`’s struct literal sets `Default::default()`. bincode’s serde path (`bincode::serde::encode_to_vec` / `decode_from_slice`, already used in `data_transformer` and `mcp/server/src/model.rs`) omits the field and restores `Default` on load. The index is built on first `get` or `ambiguous_candidates`.

**Rationale**: Aliases are a pure function of the 732 counter items. Storing them would bloat the bin file and risk disagreeing with the items. `OnceLock` is `Clone` on 1.86, and `ApiDatabase` is already `Clone`.

**Alternatives considered**:

- Build the index in `data-transformer` and serialize it. Rejected. Duplicate data, and every future loader must know the format.
- Resolve aliases by scanning items on each `get`. Rejected. The spec requires three O(1) steps.

## R6 — Three-step lookup, and ambiguous lowercase is a miss

**Decision**: After `trim()`: exact primary key in `items`, then exact alias, then lowercase alias only when that lowercase key maps to exactly one canonical id. `ambiguous_candidates` returns the full list when the lowercase key has more than one id, otherwise an empty slice. Bare identifiers such as `USAGE_AVERAGE` are not aliases. Module-qualified forms are.

**Rationale**: Eight case-only pairs (`swapIn`/`swapin`, `swapOut`/`swapout`, each with four rollups) must not collapse. Thirty-five identifiers repeat across modules, so a bare constant is ambiguous by construction (MCP-ADR-02). Confirmed today: `ApiDatabase::get` is `self.items.get(id)` and `ApiData::get` is the same. The not-found path uses `items.keys().take(1000)` and the help text cites `example::connection_basic`, which is not a real example.

**Alternatives considered**:

- Case-insensitive lookup that returns the first match. Rejected. It would silently pick `swapIn` or `swapin`.
- Register bare `USAGE_AVERAGE` when the module is unique. Rejected. The spec says bare identifiers are not aliases, including the unique ones, so the rule stays uniform.

## R7 — Empty labels are humanized by a literal rule

**Decision**: For the 34 counters whose catalog `labels.name` and `labels.name_summary` are both empty, `label` is the counter name split on `.` and on lower-or-digit → upper camelCase boundaries, lowercased, joined with spaces, first letter capitalized. `summary` is `"{group_summary} {label lowercase} ({unit_summary}, {rollup})"`. No dictionary. Locked strings: `swapIn` → `Swap in`, `itersPerS` → `Poll iters per s`, `swapunreserved` → `Swapunreserved`, `capacity.usage.vmOvrhd` → `Capacity usage vm ovrhd`.

**Rationale**: Embeddings and summaries cannot lead with an empty label. A dictionary would drift from the catalog and hide the rule the tests lock.

**Alternatives considered**:

- Fall back to the wire name as the label. Rejected. `mem.swapIn.average` does not read as a label, and the spec locks the humanized forms.
- Hand-write 34 labels. Rejected. The literal rule already produces the locked strings.

## R8 — Synonyms are a first-match table in `api_database`

**Decision**: `counter_synonyms(group, name) -> &'static str` implements the spec’s domain-synonym table. First match wins. The phrase is appended in `embedding_text` after the summary. CPU `latency` is contention, not storage latency. `disk.commandsAborted` does not receive the IOPS phrase.

**Rationale**: Operator language (“ballooning”, “IOPS”, “ready time”) is not in the wire names. Keeping the table next to `embedding_text` makes the hermetic tests a string compare, with no model loaded (MCP-ADR-05).

**Alternatives considered**:

- A synonym file generated from the catalog. Rejected. The catalog has no synonym field. The table is editorial.
- Append every matching row. Rejected. The spec says first match wins, and stacked phrases would dilute the embedding.

## R9 — Five examples, performance category before property-collector

**Decision**: Keep `perf_metrics` (one sentence updated), relabel `perf_metrics_manual` as legacy without changing its code, rework `perf_vm_usage` into the live monitor, add `perf_historical` and `perf_export`. Do not add `perf_poller` or `perf_typed`. In `build_examples`, the `perf` / `metric` category test moves to immediately after `macro_usage` and before the `PropertyCollector` test. Skip paths containing `/perf-catalog/` as well as `/vtui/`.

**Rationale**: Confirmed on this tree: the `performance` branch currently sits after `property_collector`, and the reworked monitor retrieves properties, so it would be indexed as `property_collector`. `build.rs` `find_rust_sources_recursive` skips `target`, `.git`, dot directories, and `Cargo.lock`, and does not skip `vtui` or `perf-catalog`. Both skips must be added or unindexed trees mark the bin stale. MCP-ADR-08: a sixth example competes in retrieval.

**Alternatives considered**:

- Delete the raw example. Rejected. Callers who cannot enable `perf` still need a labelled contrast.
- Leave categorization as it is and avoid `PropertyCollector` in the monitor. Rejected. The monitor’s inventory uses the retriever, and the category rule is the bug.

## R10 — The feature flag is detected from a real import

**Decision**: The example dependency template uses `vim_rs = { version = "0.7", features = ["perf"] }` only when a trimmed source line starts with `use vim_rs::perf`. Otherwise `vim_rs = "0.7"`. Doc comments do not count. `vim_macros` is dropped from the template because `vim_rs` re-exports `vim_retrievable!` and `vim_updatable!`. Tokio features are `macros`, `rt-multi-thread`, and `time`. `examples/snippets/Cargo.toml` today has `tokio` features `["macros"]` only. The snippets package must enable `rt-multi-thread` and `time` itself. The perf examples that poll cannot rely on another crate in the examples workspace turning those features on.

**Rationale**: `perf_metrics_manual` mentions `vim_rs::perf` in its documentation and must not gain the feature (MCP-ADR-07, MCP-FR-10). The current template still says `vim_rs = "0.3"` and `vim_macros = "0.3"`.

**Alternatives considered**:

- Turn the feature on for any file whose name contains `perf`. Rejected. The legacy example’s name contains `perf`.
- Match the substring `vim_rs::perf` anywhere. Rejected. The v4.2 review fixed this: doc mentions must not count.

## R11 — Release coupling and the version string

**Decision**: Documents, banners, Step 5, and perf-using templates say `vim_rs >= 0.7` and `features = ["perf"]`. The updated `api_database.bin` is not published until `vim_rs` 0.7.0 is on crates.io. In-tree version today is `0.6.1`. The `perf` feature exists and is empty (`perf = []`), off by default. `perf-chrono` and `perf-polars` are out of scope here.

**Rationale**: An assistant that copies a 0.6 dependency, or a 0.7 dependency without the feature, fails to compile. The server must not advertise an API crates.io does not have yet.

**Alternatives considered**:

- Ship the MCP index against the path dependency before 0.7.0. Rejected. Downstream users resolve `vim_rs` from crates.io.
- Keep telling assistants `0.5` in the starter guide. Rejected. The guide’s four dependency snippets move to 0.7 and keep their existing features (`xml`, `vcsim_compat`, `defaults`).

## R12 — Evaluation is a scored phase on the merged harness

**Decision**: Phase 5 extends the harness that `003-eval-harness` added. The scaffold pins `vim_rs = "0.5.0"` until T037. `parse_trace.py` stores `tool_calls` as `None` when the count is zero (`tool_calls if tool_calls else None`). `scorecard.py` `mean_metric` skips `None`. `--min-lift` must treat a missing `tool_calls` as `0` before averaging, or an arm that made no tool calls disappears from the mean. `vim_rs_tool_calls` stays a diagnostic. The operator chooses the model. The scorecard records it. Both arms use it. A miss is `mcp/evals/reports/pf-slice-<date>.md` and does not block Phases 0–4.

The harness is now in `mcp/evals/`. Operator commands are `grade.ps1` and `run_matrix.ps1`, as written in [contracts/evals.md](./contracts/evals.md). The shell names below are the layout this ADR was written against. They are not the commands to run.

**Rationale**: MCP-FR-14 and the v4.2 review. The harness layout (`problems/`, `fixtures/`, `runner/problem_emit/`, `solutions/`, `scaffold/`, `recipes/`, `run_matrix.ps1`, `grade.ps1`, `tier2.py`) is the one to extend, not a new runner.

**Alternatives considered**:

- Vendor a private copy of the harness on this branch. Rejected. Two harnesses will diverge before the real one merges.
- Make the 0.20 lift a release gate. Rejected. The only release gate is crates.io 0.7.0 plus the hermetic tests.

## R13 — Baseline queries are recorded first

**Decision**: Before changing any index input, run the current server (or a small ignored test) for these four queries at limit 5 and commit the top-five ids as `mcp/server/tests/fixtures/baseline_top5.json`: “power on a virtual machine”, “retrieve virtual machine mac address”, “vSphere task progress tracker”, “create snapshot of virtual machine”. After the rebuild, each query’s previous first id stays first, and at least four of the five previous ids stay in the top five.

**Rationale**: Adding 732 counter embeddings can reorder default-filter search. The regression check is meaningless if the baseline is captured after the change (MCP-FR-06).

**Alternatives considered**:

- Freeze the entire top-five list. Rejected. The spec allows one id to leave the top five.
- Skip non-perf regression. Rejected. Counter text that steals “power on a virtual machine” is a failed index.

## R14 — Sparkline and ready-percentage stay on the specified series

**Decision**: The live monitor reimplements the seven glyphs `▁▂▃▄▅▆▇` for `[Option<Percent>; 6]`. Index is `(p.0.clamp(0.0, 100.0) * 6.0 / 100.0).round() as usize`. `None` is a space, so 0% (`▁`) and no-data differ. Do not copy the terminal UI’s raw-hundredths function. CPU ready is `series.ready_percent(vcpus)` on the raw series, and the counter document’s snippet uses the same call. `ready_percent` returns `Vec<Option<Percent>>` and is `None` for gap slots and for non-aggregate instances when `vcpus > 1`.

**Rationale**: v4.2 pinned the glyph rule and the raw-series call. Copying vtui would couple the snippet to a function that takes raw hundredths and draws gaps as `▁`.

**Alternatives considered**:

- Draw gaps as `▁`. Rejected. Indistinguishable from 0%.
- Call `ready_percent` on `ScaledSeries`. Rejected. The method is on `Series<Time>`.

## R15 — Staleness inputs are the catalogs and the generated constants

**Decision**: `mcp/server/build.rs` adds `cargo:rerun-if-changed` and `is_newer_than` checks for `vim_build/data/perf-catalog.json`, `vim_build/data/esxi-perf-catalog.json`, and `vim_rs/src/perf/metrics.rs`. `metrics.rs` is the proxy for planner changes. A fresh bin prints `✓ api_database.bin is up to date`. A newer input panics with `ERROR: api_database.bin is out of date!`. Directory names `perf-catalog` and `vtui` are skipped in `find_rust_sources_recursive`, matching `build_examples`.

**Rationale**: Confirmed: `build.rs` tracks the OpenAPI file and example `.rs` files only. A catalog edit would otherwise ship a server whose documents disagree with `metrics.rs`.

**Alternatives considered**:

- Hash the planner source file instead of `metrics.rs`. Rejected. The spec names `metrics.rs` as the proxy, and a planner change that does not change emitted docs is still visible once regeneration runs.
