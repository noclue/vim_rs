# Tasks: MCP Performance Counters

**Input**: Design documents from `/specs/007-mcp-perf-counters/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/README.md](./contracts/README.md), [contracts/planner.md](./contracts/planner.md), [contracts/documents.md](./contracts/documents.md), [contracts/examples.md](./contracts/examples.md), [contracts/evals.md](./contracts/evals.md), [quickstart.md](./quickstart.md)

**Tests**: Included. Hermetic `cargo test` is the merge gate for the planner and the generated docs. Model-backed counter ranking is withdrawn. The evaluation is `pf01_vm_balloon_costop_query` and `pf02_vm_electricity_turn_query`, a scored report, not a release gate.

**Organization**: Tasks grouped by user story (P1 → P3) with shared foundation first.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no blocking deps on incomplete tasks)
- **[Story]**: US1–US7 per [spec.md](./spec.md)

## Path Conventions

- Generator: `vim_build/src/perf_metrics.rs`, `vim_build/src/generator.rs`, `vim_build/data/`
- Generated docs: `vim_rs/src/perf/metrics.rs` (regenerate only; do not hand-edit)
- Index model: `mcp/api_database/src/lib.rs`
- Pipeline: `mcp/data_processing/build_counters/`, `mcp/data_processing/data_transformer/`, `mcp/data_processing/build_examples/`, `mcp/data_processing/build_embeddings/`
- Server: `mcp/server/src/main.rs`, `mcp/server/build.rs`, `mcp/server/guides/VIM_RS_STARTER_GUIDE.md`
- Snippets: `examples/snippets/src/perf_*.rs`, `examples/snippets/Cargo.toml`
- Evaluation (only after `003-eval-harness` is merged): `mcp/evals/`
- Run MCP package commands from `mcp/`. Run snippet commands from `examples/`. Run generator commands from the repo root. When the sandbox overrides Cargo home, prefix with `RUSTUP_HOME=$HOME/.rustup CARGO_HOME=$HOME/.cargo $HOME/.cargo/bin/`.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Freeze the non-performance search baseline, and add the empty pipeline crate. No catalog or example edits in this phase.

- [X] T001 Record the current top-5 ids for filter `all` and limit 5, before any other task changes an index input, into `mcp/server/tests/fixtures/baseline_top5.json`. Queries: “power on a virtual machine”, “retrieve virtual machine mac address”, “vSphere task progress tracker”, “create snapshot of virtual machine”. Store query, rank, and id. This file is the MCP-FR-06 regression oracle.
- [X] T002 [P] Add the `build-counters` crate at `mcp/data_processing/build_counters/Cargo.toml` and `mcp/data_processing/build_counters/src/lib.rs` (package name `build-counters`, deps `api_database`, `vim_build`, `serde_json`, `anyhow`, `chrono`, `tracing`). Add the member in `mcp/Cargo.toml`. Add `build-counters = { path = "../build_counters" }` in `mcp/data_processing/data_transformer/Cargo.toml`. Leave `collect_counters` unimplemented until T011.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Public planner, ESXi rustdoc, and the `CounterEntry` model. **Blocks US1 and US2.** US3, US4, and US5 use the already shipped `vim_rs::perf` and may start after Phase 1. US6’s legacy doc edit (T029) may start after Phase 1. US6’s indexer and starter guide (T030, T031) wait for T022, T023, T025, T027, and T029.

**⚠️ CRITICAL**: No US1 or US2 work until this phase completes. Do not hand-edit `vim_rs/src/perf/metrics.rs`.

- [X] T003 Copy `vim_rs/tests/fixtures/perf/esxi_catalog.json` byte-for-byte to `vim_build/data/esxi-perf-catalog.json`. In `vim_build/src/perf_metrics.rs`, add a unit test that the two files are byte-equal, beside the existing vCenter catalog test.
- [X] T004 In `vim_build/src/perf_metrics.rs`, replace the private plan result with public `PlanTier`, `PlannedCounter`, and `plan_counters(vcenter_catalog: &str, esxi_catalog: Option<&str>) -> Result<Vec<PlannedCounter>>` per [contracts/planner.md](./contracts/planner.md). `Some(esxi)` sets `esxi_supported` and errors if ESXi lists a `(group, name, rollup, stats)` absent from vCenter. Identifier rules (flat camelCase, SCSI `_ABSOLUTE`) stay inside this function. Point the existing 732-constant test at `plan_counters`. Add rustdoc on `PlanTier`, `PlannedCounter`, `plan_counters`, and `emit_perf_metrics` as required by contracts/planner.md.
- [X] T005 In `vim_build/src/perf_metrics.rs` and `vim_build/src/generator.rs`, change `emit_perf_metrics` to `(catalog, esxi_catalog, output)` and call `plan_counters`. Pass `vi_json_spec_path.with_file_name("esxi-perf-catalog.json")`. Module header names both catalogs. Each constant line ends with `· ESXi 8.0.3` or `· vCenter only`. Tier 3b text is `Tier 3b (inferred): {kinds}`. Do not change constant names, types, or `by_entity` re-exports.
- [X] T006 Run `cargo run -p vim_build` and commit the regenerated `vim_rs/src/perf/metrics.rs` only if `git diff` is limited to `//!` and `///` lines (353 `· ESXi 8.0.3`, 379 `· vCenter only`, tiers 265/96/197/174). Run `cargo test -p vim_build` and `cargo test -p vim_rs --features perf`.
- [X] T007 [P] Add `ScopeTier`, `CounterEntry`, and `CountersOutput` in `mcp/api_database/src/lib.rs` per [data-model.md](./data-model.md). Add `ApiItemEntry::Counter` (`id()` is `&c.id`, `item_type()` is `"counter"`). Add a trailing `counters: Vec<CounterEntry>` argument to `ApiData::from_parts` and insert each entry by `c.id`. Load `api_definitions/counters.json` from `load_from_dir` when present. Pass `Vec::new()` at the current call in `mcp/data_processing/data_transformer/src/main.rs` so the tree still compiles. Do not implement alias lookup or document text in this task.

**Checkpoint**: Planner and `CounterEntry` exist. `metrics.rs` documentation mentions ESXi. Search still has no counters filter.

---

## Phase 3: User Story 1 — Natural-language counter discovery (Priority: P1) 🎯 MVP

**Goal**: An assistant searches with `filter = "counters"` and looks up a counter by canonical id or by an unambiguous alias, including case-distinct swap-in and swap-out counters.

**Independent Test**: From `mcp/`, `cargo test -p api_database` and `cargo test -p build-counters`. 732 unique ids, key 148 ends in `.absolute`, 353 `esxi_supported`, locked humanized labels, and the alias cases in MCP-FR-04. Model ranking is T033–T036, after the examples exist.

### Tests for User Story 1

> Write T008 and the tests inside T010 and T011 so they fail until the matching implementation lands.

- [X] T008 [US1] Add failing tests in the `#[cfg(test)]` module of `mcp/api_database/src/lib.rs` that build an `ApiDatabase` containing the swap-in, swap-out, CPU usage, and `management_agent` `SWAP_IN_AVERAGE` counters. Assert `get("mem.swapIn.average")` and `get("mem.swapin.average")` differ; `get("metrics::mem::swap_in_average")`, `get("metrics::mem::swapin_average")`, and `get("metrics::management_agent::swap_in_average")` each resolve; `get("COUNTER::CPU.USAGE.AVERAGE")` resolves; `get("MEM.SWAPOUT.NONE")` is `None` and `ambiguous_candidates` is exactly `counter::mem.swapOut.none` and `counter::mem.swapout.none`; `get("USAGE_AVERAGE")` is `None`.

### Implementation for User Story 1

- [X] T009 [US1] Implement `CounterAliasIndex`, `ApiDatabase::alias_index`, `get`, and `ambiguous_candidates` in `mcp/api_database/src/lib.rs` per [data-model.md](./data-model.md). Field is `#[serde(skip)] pub counter_aliases: std::sync::OnceLock<CounterAliasIndex>`. Set `counter_aliases: Default::default()` on the `ApiDatabase` literal in `mcp/data_processing/data_transformer/src/main.rs`. Exact alias, then unambiguous lowercase alias. Bare `USAGE_AVERAGE` is not an alias. Make T008 pass.
- [X] T010 [US1] Implement `counter_synonyms` (first match wins, table in [spec.md](./spec.md)) and `ApiItemEntry::Counter` `embedding_text` in `mcp/api_database/src/lib.rs` per [contracts/documents.md](./contracts/documents.md). Hermetic tests: the seven listed counters start with `{label}: {summary}` and contain the required phrase; `mem.swapIn.average` label handling is not this task’s fallback (that is T011) but a synthetic entry with label `Swap in` must embed that label; `disk.commandsAborted.summation` must not contain the IOPS phrase.
- [X] T011 [P] [US1] Implement `collect_counters` and `build_counters` in `mcp/data_processing/build_counters/src/lib.rs`. Join vCenter labels by key. Map `PlanTier` to `ScopeTier`. `id` is `counter::{group}.{name}.{rollup}` plus `.absolute` only for the absolute stats twin (key 148). Humanize empty name label and name summary per [data-model.md](./data-model.md) (locked: `Swap in`, `Poll iters per s`, `Swapunreserved`, `Capacity usage vm ovrhd`). `scaled_type` and `scale_description` from the spec unit table; unknown unit is an error. `valid_entities` from the spec scope table. Tests: 732 unique ids, 353 `esxi_supported`, key 148 id, four locked labels, and every entry’s `pub const {rust_ident}: Metric<{dimension}, {scope}>` appears inside `pub mod {rust_module}` in `vim_rs/src/perf/metrics.rs`.
- [X] T012 [US1] In `mcp/data_processing/data_transformer/src/main.rs`, insert “Collect Counters” so the pipeline is five steps: API definitions, examples, `collect_counters`, `ApiData::from_parts` with the real vector, embeddings. Log total and ESXi count. On `--emit-json`, call `build_counters` and write `counters.json`. Depends on T011.
- [X] T013 [P] [US1] Add `SearchFilter::Counters` in `mcp/server/src/main.rs` (`serde` rename already `snake_case`, so JSON is `counters`; `as_str()` returns `"counter"`). Mention performance counters in the `search` tool description. Unit test the rename and `as_str`. Do not add a seventh tool.
- [X] T014 [US1] In `mcp/server/src/main.rs`, extend `GetInput::id` description with `counter::cpu.usage.average` and `metrics::cpu::USAGE_AVERAGE`, and replace `example::connection_basic` with `example::perf_metrics`. When `get` misses and `ambiguous_candidates` is non-empty, reply `Multiple counters match '<id>' (case-sensitive):` plus id, `rust_constant`, label, and unit. Otherwise scan every `items` key (delete `.take(1000)`), keep the five-suggestion cap, and mention the counter id and `example::perf_metrics`. Add the `filter="counters"` and `features = ["perf"]` sentence to `get_info()`. Log `count_by_type("counter")` in `mcp/server/src/main.rs` and in `mcp/data_processing/build_embeddings/src/lib.rs`.
- [X] T015 [P] [US1] In `mcp/server/build.rs`, `cargo:rerun-if-changed` and stale-check `vim_build/data/perf-catalog.json`, `vim_build/data/esxi-perf-catalog.json`, and `vim_rs/src/perf/metrics.rs`. In `find_rust_sources_recursive`, skip directory names `perf-catalog` and `vtui` next to `target` and `.git`.
- [X] T016 [US1] From `mcp/`, run `cargo test -p api_database` and `cargo test -p build-counters`. Both must pass offline.

**Checkpoint**: Hermetic discovery works. The embedded bin is not rebuilt yet. Stop here to demo aliases and the 732-entry collector without examples or the model.

---

## Phase 4: User Story 2 — Scopes and ESXi availability in the counter document (Priority: P1)

**Goal**: Lookup renders scope, tier, per-entity paths, tier-3b guidance, unit scaling, and ESXi 8.0.3 availability, and raw performance-manager pages point at `PerfClient`.

**Independent Test**: `cargo test -p api_database` renders the five fixtures in [contracts/documents.md](./contracts/documents.md). Rustdoc markers already landed in T006.

### Tests for User Story 2

- [X] T017 [US2] Add failing render tests in `mcp/api_database/src/lib.rs` for `cpu.usage.average` (aggregate and per-device, ESXi yes, full scale description), `cpu.ready.summation` (snippet contains `.ready_percent(`), `disk.deviceReadLatency.average` (`Instance::All` and `devices(`), `vcResources.buffersz.average` (`.with_scope::<AnyEntity>()` or `.metric_unchecked()`, no `by_entity` path, vCenter only), and a synthetic `level: None` (`n/a`). Every snippet must contain `features = ["perf"]` and `use vim_rs::perf::`.

### Implementation for User Story 2

- [X] T018 [US2] Implement `search_summary` and `format_counter_doc` for `ApiItemEntry::Counter` in `mcp/api_database/src/lib.rs` per [contracts/documents.md](./contracts/documents.md). Full `scale_description` in backticks. Unset level prints `n/a`. Instances print `aggregate & per-device` or `aggregate only`. Per-entity paths only for tiers 1, 2, and 3a. Ready snippet calls `ready_percent` on the raw series. `!esxi_supported` adds the standalone-ESXi comment. Requirement line is `*(Requires vim_rs >= 0.7 with features = ["perf"])*`.
- [X] T019 [US2] In `format_managed_object_doc` and `format_method_doc` in `mcp/api_database/src/lib.rs`, prepend the IMPORTANT block for `PerformanceManager`, the six query methods, and `perf_counter` / `historical_interval`. Prepend the NOTE line for the five admin methods (`create_perf_interval`, `update_perf_interval`, `remove_perf_interval`, `update_counter_level_mapping`, `reset_counter_level_mapping`). Leave `description` with no banner. Assert those five renders in the test module.
- [X] T020 [US2] Run `cargo test -p api_database` from `mcp/` and confirm T017 and T019 pass.

**Checkpoint**: P1 discovery is complete offline. Counter pages and performance-manager callouts are testable without a model.

---

## Phase 5: User Story 3 — Live monitoring dashboard (Priority: P2)

**Goal**: `perf_vm_usage` polls a prepared realtime query, draws six-slot sparklines, and keeps going when a tick fails or a VM has no samples.

**Independent Test**: From `examples/`, `cargo check -p snippets --bin perf_vm_usage --bin perf_metrics` with no warnings from those bins.

### Implementation for User Story 3

- [X] T021 [US3] In `examples/snippets/Cargo.toml`, set `tokio = { version = "1.44", features = ["macros", "rt-multi-thread", "time"] }`. Keep the existing `perf_metrics` and `perf_metrics_manual` bins.
- [X] T022 [US3] Rewrite `examples/snippets/src/perf_vm_usage.rs` per [contracts/examples.md](./contracts/examples.md). Title line: `Live realtime CPU and memory monitor for every virtual machine, with sparklines.` Counters come from `vim_rs::perf::metrics::by_entity::virtual_machine`. Sparkline glyphs `▁▂▃▄▅▆▇`; `None` is a space; index `(p.0.clamp(0.0, 100.0) * 6.0 / 100.0).round() as usize`. Do not copy vtui. `PERF_TICKS` defaults to 3. Call `client.close().await`.
- [X] T023 [P] [US3] In `examples/snippets/src/perf_metrics.rs`, change only the module-doc sentence that describes `perf_vm_usage` as a static CPU-and-memory table so it describes the live monitor. Leave the rest of the file unchanged.
- [X] T024 [US3] From `examples/`, run `cargo check -p snippets --bin perf_vm_usage --bin perf_metrics`. Both bins must compile with no warnings.

**Checkpoint**: The live monitor compiles. It does not need the MCP index.

---

## Phase 6: User Story 4 — Historical extraction (Priority: P2)

**Goal**: `perf_historical` exports past-day CPU and memory as tidy rows, chunked every six hours, without a historical sample cap.

**Independent Test**: From `examples/`, `cargo check -p snippets --bin perf_historical` with no warnings. Comments state that historical intervals ignore `max_sample` and that a multi-day run uses `Interval::PAST_WEEK` with `.since_ago`.

### Implementation for User Story 4

- [X] T025 [US4] Register `perf_historical` in `examples/snippets/Cargo.toml` and add `examples/snippets/src/perf_historical.rs` per [contracts/examples.md](./contracts/examples.md). Title: `Past-day 5-minute CPU and memory rollups for every virtual machine, exported as tidy rows.` Query `metrics::cpu::USAGE_AVERAGE` and `metrics::mem::CONSUMED_AVERAGE` with `Interval::PAST_DAY`, `.between(now - 24h, now)`, and `.time_chunk(Duration::from_secs(6 * 3600))`. Print `res.rows()` as `entity,counter,instance,rfc3339_ts,raw,scaled` and print `res.warnings`.
- [X] T026 [US4] From `examples/`, run `cargo check -p snippets --bin perf_historical` with no warnings.

**Checkpoint**: Historical extract compiles on its own.

---

## Phase 7: User Story 5 — Lossless incremental export (Priority: P2)

**Goal**: `perf_export` pulls only new samples, drops gaps, forgets departed entities, and logs a failed tick without stopping. Its three poll statements are the ones Step 5 will quote.

**Independent Test**: From `examples/`, `cargo check -p snippets --bin perf_export` with no warnings. The file contains the ticker declaration, `set_missed_tick_behavior`, and `match query.fetch_incremental(&vms, &watermark).await` exactly as [contracts/examples.md](./contracts/examples.md) specifies.

### Implementation for User Story 5

- [X] T027 [US5] Register `perf_export` in `examples/snippets/Cargo.toml` and add `examples/snippets/src/perf_export.rs` per [contracts/examples.md](./contracts/examples.md). Title: `Incremental realtime export of VM metrics with per-entity watermarks (no duplicates, no gaps).` `Interval::Realtime`, `.latest(4)`, `SampleWatermark::new()`, `for tick in 0..ticks`, `INVENTORY_EVERY = 2`, `retain_entities`, `PERF_TICKS` default 3, `client.close().await`. If `?` does not convert `PerfError` into `anyhow::Error`, use `.map_err(|e| anyhow::anyhow!(e.to_string()))` on the refresh-rate call and use that same expression later in T031.
- [X] T028 [US5] From `examples/`, run `cargo check -p snippets --bin perf_export` with no warnings.

**Checkpoint**: Export compiles. Step 5 is not written yet.

---

## Phase 8: User Story 6 — Raw API contrast (Priority: P3)

**Goal**: The raw example is labelled legacy and does not gain the `perf` feature. The indexer classifies all five `perf_*` programs as performance, and the starter guide teaches Step 5 in at most 40 lines.

**Independent Test**: From `mcp/`, `cargo test -p build-examples`. From `examples/`, `cargo check -p snippets --bins` with no warnings on the five `perf_*` bins. Step 5’s ticker, missed-tick, and `match query.fetch_incremental` text appear character for character in `examples/snippets/src/perf_export.rs`.

### Implementation for User Story 6

- [X] T029 [P] [US6] Rewrite only the module docs of `examples/snippets/src/perf_metrics_manual.rs`. First line: `//! Legacy: hand-rolled `PerformanceManager` sample (contrast with `perf_metrics`).` State that it contrasts with `perf_metrics` / `vim_rs::perf` and that it is the direct `PerformanceManager` path when `perf` is off. Keep the existing description below. Do not change code, and do not add `use vim_rs::perf`.
- [X] T030 [P] [US6] In `mcp/data_processing/build_examples/src/lib.rs`, skip paths containing `/perf-catalog/`. Move the `name.contains("perf") || name.contains("metric")` category ahead of the `PropertyCollector` test and immediately after `macro_usage`. Choose `vim_rs = { version = "0.7", features = ["perf"] }` only when a trimmed line starts with `use vim_rs::perf`; otherwise `vim_rs = "0.7"`. Drop `vim_macros`. Tokio features `["macros", "rt-multi-thread", "time"]`. Add tests that `collect_examples(Path::new("examples"))` indexes nothing under `perf-catalog/` or `vtui/`, keeps the 16 non-perf categories, marks all five `perf_*` examples `performance`, and puts `features = ["perf"]` on every perf example except `perf_metrics_manual`.
- [X] T031 [US6] Update `mcp/server/guides/VIM_RS_STARTER_GUIDE.md` per [contracts/examples.md](./contracts/examples.md). Replace the line-3 usage note so Step 5 is a lookup section (`vim_rs >= 0.7`, feature `perf`). Insert Step 5 after Step 4 at ≤ 40 lines, with `loop {` and the three statements copied from `examples/snippets/src/perf_export.rs`. Add the five anti-pattern rows, add `counters` to the search-tool row, and move the four dependency snippets from `0.5` to `version = "0.7"` without adding `perf` to them.
- [X] T032 [US6] From `mcp/`, run `cargo test -p build-examples`. From `examples/`, run `cargo check -p snippets --bins` and confirm the five `perf_*` bins warn nothing.

**Checkpoint**: Five examples, the indexer, and Step 5 agree. The embedded bin is still stale.

---

## Phase 9: User Story 1 — Model-backed ranking (Priority: P1)

**Goal**: Close MCP-FR-06.2 and MCP-FR-13. The rebuilt index has 732 counters and the five examples. Default-filter search does not lose the T001 baseline.

**Independent Test**: From `mcp/`, `cargo test -p vim_mcp_server` passes offline. `cargo test -p vim_mcp_server -- --ignored` passes when the BGE model is in `mcp/data/model_cache` or can be downloaded. Ignored tests are not part of the default run.

### Tests for User Story 1

- [X] T033 [US1] From `mcp/`, run `cargo run -p data-transformer --release` and replace `mcp/data/api_database.bin`. The bin must contain 732 `counter` items and the five performance examples.
- [X] T034 [US1] From `mcp/`, run `cargo build -p vim_mcp_server`. A second build with no input changes prints `✓ api_database.bin is up to date`. Startup logging includes a counter count of 732.
- [X] T035 [US1] Add `#[ignore]` tests in `mcp/server/tests/counter_search.rs` that load the built bin and the model. With `filter = "counters"` and limit 5, the five queries in [contracts/documents.md](./contracts/documents.md) each contain the named counter and only counters. With `filter = "all"` and limit 5, each T001 query keeps its previous first id, and at least 4 of its 5 baseline ids remain.
- [X] T036 [US1] From `mcp/`, run `cargo test -p vim_mcp_server` (offline, ignored tests skipped) and `cargo test -p vim_mcp_server -- --ignored` (needs the model). Both pass.

**Checkpoint**: P1 is done, including ranking. This is the release candidate for the index once `vim_rs` 0.7.0 is on crates.io. US7 is not required.

---

## Phase 10: User Story 7 — Measured assistant lift (Priority: P3)

**Goal**: Five performance problems are scored with the server off and on, on the harness already in `mcp/evals/`. A miss is written down and does not block release.

**Independent Test**: Reference solutions pass `mcp/evals/runner/grade.ps1`. Existing `pc01`..`pc05` still pass. `mcp/evals/reports/pf-slice-<date>.md` exists. `scorecard.py --min-lift 0.20` enforces the three thresholds in [contracts/evals.md](./contracts/evals.md), treating a missing `tool_calls` as 0.

`003-eval-harness` is in this tree. Extend it. Do not vendor a second copy. Follow [contracts/evals.md](./contracts/evals.md) “Harness as landed” and the Windows section of `mcp/evals/README.md`. Grade and sweep with `grade.ps1` and `run_matrix.ps1`. Problem ids are YAML stems. The container sees only the sealed work tree and that attempt’s `agent/` directory. Rebuild `vim-rs-eval` once after T037 so `/opt/eval-target` matches the new scaffold. Leave `grade.sh` and `run_matrix.sh` unchanged.

### Implementation for User Story 7

- [X] T037 [US7] Change `mcp/evals/scaffold/Cargo.toml` from `vim_rs = "0.5.0"` to `vim_rs = { path = "../../../vim_rs", features = ["perf"] }`. The in-tree crate is `0.6.1`; do not pin `version = "0.7"` on that path and do not bump `vim_rs`. Adapt `mcp/evals/scaffold/src/mock/` to the in-tree `VimClient` trait. In `mcp/evals/Containerfile`, `COPY vim_rs /vim_rs` and `COPY vim_macros /vim_macros`. Remove the `vim_rs` line from `.dockerignore` so the build context includes the crate (`**/target` stays). Do not bind-mount the repo. Do not change `mcp/evals/docker/Cargo.toml`. Rebuild the image from the repo root. `CARGO_BUILD_JOBS=1` stays on the image `RUN` only.
- [X] T038 [P] [US7] Export `ENTITY_REFS=<type>:<value>,…` from `mcp/evals/runner/lib/tier2.py` (keep `VM_IDS`). Parse it in `mcp/evals/scaffold/src/main.rs` when `MO_TYPE == "Mixed"`.
- [X] T039 [P] [US7] In `mcp/evals/scaffold/src/mock/mod.rs` and `mcp/evals/scaffold/src/mock/service_content.rs`, set `perf_manager` to `PerformanceManager` / `PerfMgr`, capture `mock_now` at construction, serve counters, intervals, provider summary, availability, and `QueryPerf` from fixture offsets, honor realtime `max_sample`, record every `PerfQuerySpec`, and add `advance_phase()`. Historical fixtures need an enabled 300-second interval. Do not expose `PerfClient::set_clock` to drivers.
- [X] T040 [P] [US7] Add optional `--min-lift <x>` to `mcp/evals/runner/scorecard.py`. Keep positional `RESULTS_DIR OUT.json [model] [n_repeats]`, which `run_matrix.ps1` already passes. Exit non-zero unless retrieval pass rate rises by at least `x`, compile rate does not fall, and mean tool calls do not rise. Read missing `tool_calls` as `0` before averaging (`parse_trace.py` stores zero as `None`). Record the operator-chosen model id in the scorecard. `vim_rs_tool_calls` is diagnostic only. Do not make `run_matrix.ps1` fail the sweep on a miss.
- [X] T041 [US7] `mcp/evals/problems/pf01_vm_balloon_costop_query.yaml`. `PerfClient` and VM ids in, one `PerfQuery` out, `Interval::PAST_DAY`. Graded counters are `mem.vmmemctl.average` (90) and `cpu.costop.summation` (448).
- [X] T042 [US7] `mcp/evals/problems/pf02_vm_electricity_turn_query.yaml`. Same contract. Graded counters are `power.power.average` (164) and `cpu.ready.summation` (12). The prompt does not use those names.
- [X] T043 [US7] Removed. The incremental-watermark challenge is not part of the perf slice.
- [X] T044 [US7] Removed. The typed host-disk challenge is not part of the perf slice.
- [X] T045 [US7] Removed. The mixed-entity automatic-interval challenge is not part of the perf slice.
- [ ] T046 [US7] From `mcp/evals`, grade `pf01_vm_balloon_costop_query` and `pf02_vm_electricity_turn_query`, then the existing `pc01`..`pc05` references. A matrix is optional and is not a release gate.

**Checkpoint**: Scorecard is committed. Release of the index does not wait on a pass.

---

## Phase 11: Polish & Cross-Cutting Concerns

**Purpose**: Release notes and a full quickstart pass.

- [X] T047 [P] Under `## [Unreleased]` in `CHANGELOG.md`, note the public `plan_counters` API, the ESXi rustdoc markers, MCP counter search and lookup, the `perf` feature requirement on `vim_rs` 0.7, and that the updated server must not be published before `vim_rs` 0.7.0 is on crates.io. State that `vim_rs` compile cost is rustdoc-only and that `vim_build` is not linked into `vim_mcp_server`.
- [X] T048 Walk [quickstart.md](./quickstart.md) from baseline through the ignored tests. Confirm a touch of `examples/vtui` or `examples/perf-catalog` does not stale `mcp/server/build.rs`, and a touch of `vim_rs/src/perf/metrics.rs` does.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: T001 before any task that changes catalogs, examples, the guide, or `api_database.bin`. T002 may run beside T001.
- **Foundational (Phase 2)**: Depends on Setup. **Blocks US1 and US2.** T003 → T004 → T005 → T006. T007 may run beside T003–T006.
- **US1 hermetic (Phase 3)**: Depends on Phase 2. T008 → T009 → T010 in `mcp/api_database/src/lib.rs`. T011 may run beside T008–T010. T012 after T011 and T009. T014 after T009 and T013. T015 beside T013.
- **US2 (Phase 4)**: Depends on Phase 2 and on T010, because both edit `mcp/api_database/src/lib.rs`. Does not depend on the server filter.
- **US3, US4, US5 (Phases 5–7)**: Depend on Phase 1 only for ordering hygiene. They need shipped `vim_rs::perf`, not the MCP index. T025 and T027 also edit `examples/snippets/Cargo.toml`, so they follow T021.
- **US6 (Phase 8)**: T030 and T031 need the five example files (T022, T023, T025, T027, T029). T031 copies three statements from T027.
- **US1 model gate (Phase 9)**: After Phase 8, so the bin includes counters and the five examples.
- **US7 (Phase 10)**: After Phase 9. The harness is already in `mcp/evals/`. T041–T045 after T039. Rebuild `vim-rs-eval` after T037 and before T046. T046 last.
- **Polish (Phase 11)**: After Phase 9. T047 may run in parallel with T048. US7 does not block T047.

### User Story Dependencies

- **US1 (P1)**: Hermetic slice after Phase 2. Model slice after US6.
- **US2 (P1)**: After Phase 2. Shares `mcp/api_database/src/lib.rs` with US1, so it follows T010. Independently testable with synthetic `CounterEntry` values.
- **US3 (P2)**: No MCP dependency. Start after T021’s tokio features.
- **US4 (P2)**: No dependency on US3’s monitor. Shares `Cargo.toml` with US3, so register the bin after T021.
- **US5 (P2)**: No dependency on US3 or US4 behavior. Shares `Cargo.toml`. Step 5 (T031) depends on this file’s three statements.
- **US6 (P3)**: Doc edit is independent. Indexer tests and the guide need US3–US5 files.
- **US7 (P3)**: Needs the rebuilt bin (Phase 9) and the merged harness. Not a release gate.

### Parallel Opportunities

- T002 beside T001.
- T007 beside T003–T006.
- T011 beside T008–T010. T013 and T015 beside T008–T010. T014 starts only after T009 and T013 are both done, because it calls `ambiguous_candidates` and edits `mcp/server/src/main.rs`.
- T022 beside T023.
- T029 beside T030.
- T038, T039, and T040 beside each other after T037.
- T041, T042, T043, T044, and T045 beside each other after T039.
- Do not edit `mcp/api_database/src/lib.rs` or `examples/snippets/Cargo.toml` on two tasks at once.

### Parallel Example: User Story 1

```bash
# After T007, different files:
Task: "T008 alias tests in mcp/api_database/src/lib.rs"
Task: "T011 collect_counters in mcp/data_processing/build_counters/src/lib.rs"
Task: "T013 SearchFilter::Counters in mcp/server/src/main.rs"
Task: "T015 staleness rules in mcp/server/build.rs"
# After T009 and T013, then T014 in mcp/server/src/main.rs
```

### Parallel Example: User Story 7

```bash
# After T039, one problem per task:
Task: "T041 pf01 balloon and swap-in rate"
Task: "T042 pf02 past-day rows"
Task: "T043 pf03 incremental watermark"
Task: "T044 pf04 host disk latency"
Task: "T045 pf05 mixed automatic interval"
```

---

## Implementation Strategy

### MVP First (User Stories 1 and 2)

1. Phase 1, with T001 done before anything else.
2. Phase 2.
3. Phase 3 through T016.
4. Phase 4 through T020.
5. **STOP and VALIDATE**: hermetic alias, embedding, document, and banner tests. This is the discovery MVP. Examples and the model are not required to prove the collector and the documents.

### Incremental Delivery

1. Setup + foundation → planner and `CounterEntry`.
2. US1 hermetic + US2 → searchable counter contract offline.
3. US3 → live monitor compiles.
4. US4 → historical extract compiles.
5. US5 → export compiles, including the three quoted statements.
6. US6 → indexer and Step 5.
7. Rebuild the bin and run ignored ranking tests.
8. US7 only after the harness merges. Publish the server only after `vim_rs` 0.7.0 is on crates.io, whether or not the 0.20 lift is met.

### Parallel Team Strategy

1. One person completes T001 and Phase 2.
2. Then:
   - Developer A: US1 and US2 in `mcp/api_database/src/lib.rs` (sequential).
   - Developer B: US3, US4, and US5 (sequential on `examples/snippets/Cargo.toml`, example bodies otherwise separate).
3. US6 joins both streams. One person rebuilds the bin (Phase 9). US7 waits for the harness merge.

---

## Notes

- [P] means a different file and no unfinished dependency.
- Do not hand-edit `vim_rs/src/perf/metrics.rs`. T006 regenerates it.
- Do not add an MCP tool. Tool count stays 6.
- Do not connect `vim_mcp_server` to a live vCenter.
- `api_database` must not depend on `vim_build`. Only `build-counters` does.
- A missed `--min-lift 0.20` is a report, not a reason to hold the index.
- Commit after each phase checkpoint.
