# Tasks: Ergonomic Performance Manager API

**Input**: Design documents from `/specs/006-perf-manager-api/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/README.md](./contracts/README.md), [contracts/generator.md](./contracts/generator.md), [quickstart.md](./quickstart.md)

**Tests**: Included. Spec independent tests, SC-001–SC-011, and [quickstart.md](./quickstart.md) make offline fixture, mock, and compile-fail tests the merge gate.

**Organization**: Tasks grouped by user story (P1 → P3) with shared foundation first.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no blocking deps on incomplete tasks)
- **[Story]**: US1–US5 per [spec.md](./spec.md)

## Path Conventions

- Library: `vim_rs/src/perf/`, `vim_rs/src/lib.rs`, `vim_rs/Cargo.toml`
- Generator: `vim_build/src/perf_metrics.rs`, `vim_build/src/generator.rs`, `vim_build/data/perf-catalog.json`
- Tests: `vim_rs/tests/perf_*.rs`, `vim_rs/tests/support/perf_mock.rs`, `vim_rs/tests/fixtures/perf/`, `vim_rs/tests/perf_scope_fail/`
- Snippet: `examples/snippets/src/perf_metrics.rs`
- Docs: `CHANGELOG.md`, `README.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Public module shell and feature flags. No query behavior yet.

- [X] T001 [P] Add `pub mod perf;` in `vim_rs/src/lib.rs` and create `vim_rs/src/perf/mod.rs` declaring submodules `units`, `entity`, `metric`, `registry`, `query`, `decode`, `result`, `watermark`, `error`, `typed`, plus `#[cfg(feature = "perf-polars")] pub mod polars`. Stub files may be empty until later tasks. Do not add `vim_rs::core::perf`.
- [X] T002 [P] In `vim_rs/Cargo.toml`, add optional `chrono` 0.4 (`default-features = false`, features `["std"]`) and optional `polars` (current stable 0.x, `default-features = false`) with features `perf-chrono` and `perf-polars`. Add dev-dependency `trybuild`. Do not add `serde`, `serde_json`, or the `tokio` `time` feature.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Sanitized catalogs, error and unit types, scopes, `Metric`, timestamps, and `CounterRegistry`. **Blocks all user stories.**

**⚠️ CRITICAL**: No user story work until this phase completes.

- [X] T003 Commit sanitized fixtures from `examples/perf-catalog/out/` (gitignored) using the rules in [quickstart.md](./quickstart.md): `vim_build/data/perf-catalog.json`, `vim_rs/tests/fixtures/perf/vcenter_catalog.json` (byte-identical), `esxi_catalog.json`, `query_perf_csv.json`, `query_perf_normal.json`, `query_perf_gaps.json`, `query_perf_soap.xml`, `query_composite.json`. Server host `vcenter.example.internal`, zeroed `instance_uuid`, synthetic MoRefs. Schema `vim_rs.perf-catalog/1`.
- [X] T004 [P] Implement `PerfError`, `PerfWarning`, `EntityStatus`, and `UnsupportedReason` in `vim_rs/src/perf/error.rs` per [data-model.md](./data-model.md). `PerfError::Client` wraps `vim_rs::core::client::Error` with `#[from]`. Enums are `#[non_exhaustive]`.
- [X] T005 [P] Implement `Dimension`, `ScaledValue`, `UnitKey`, and the scaled newtypes in `vim_rs/src/perf/units.rs` for all 16 wire units in [spec.md](./spec.md), including `Other_("gigaBytes")` as exact `i128` bytes. `Display` follows the spec column. `Bytes::as_u64` saturates.
- [X] T006 [P] Implement `Scope<const MASK>`, mask constants, `mo_type_bit` (`VirtualApp` → pool bit, `StoragePod` → datastore bit), and the scope aliases in `vim_rs/src/perf/entity.rs` per [data-model.md](./data-model.md).
- [X] T007 [P] Implement `Timestamp` (`SystemTime` newtype) with hand-written RFC 3339 parse/format in `vim_rs/src/perf/result.rs`. No `chrono` on the default path (research R3).
- [X] T008 Implement `Rollup`, `StatsKind`, and `Metric<D, S>` (`new`, `with_scope`, `custom`) in `vim_rs/src/perf/metric.rs`. `Metric::new` is a public `const fn`. Generated constants are not created in this task.
- [X] T009 Implement `CounterId`, `CounterName`, `CounterKey`, `CounterDef`, `ProviderCaps`, and `CounterRegistry::{from_wire, from_catalog_json, generation, resolve, by_name, by_name_with_stats, by_id, intervals}` in `vim_rs/src/perf/registry.rs`. Parse catalog JSON with `miniserde::json::Value` only. Name split and rate-preferred `by_name` follow spec FR-01. Generation uses a process-wide `AtomicU64` (research R8). `from_wire` reads `group_info.key` and `name_info.key` via `ElementDescription` deref, and keeps wire `PerfSummaryTypeEnum` / `PerfStatsTypeEnum` including `Other_`.
- [X] T010 Add registry tests in `vim_rs/tests/perf_registry.rs`: 732 vCenter and 353 ESXi counters resolve; `cpu.capacity.contention.average`; both `mem.zip.saved.latest` and `mem.zipSaved.latest`; `disk.scsiReservationCnflctsPct.average` bare name is rate and stats-kind lookup selects absolute; that SCSI pair is the only `group.name.rollup` with two stats kinds; `cpu.ready.summation` is delta and is the only variant of its name, so bare lookup returns that delta; ESXi levels stay `None`; the two vCenter catalog paths are byte-identical. Run `cargo test -p vim_rs --test perf_registry`.

**Checkpoint**: Catalogs load offline. No `QueryPerf` client yet.

---

## Phase 3: User Story 1 — One-shot diagnostic query (Priority: P1) 🎯 MVP

**Goal**: A caller names counters, passes managed objects, and gets human-scaled values, gap-aware last/current slots, and a row for every requested object, including powered-off VMs as no-samples.

**Independent Test**: `cargo test -p vim_rs --test perf_decode --test perf_interval` plus `cargo test -p vim_rs --features xml --test perf_decode`. Fixtures decode to the same series; `[420, 410, -1, -1]` yields last 410 and a blank current slot; kiloBytes and gigaBytes scale to the exact byte counts in spec SC-007.

### Tests for User Story 1

> Write these first; they fail until T013–T016 land.

- [X] T011 [P] [US1] Add failing tests in `vim_rs/tests/perf_decode.rs` for normal/CSV parity (ignore `requested_at`), SOAP parity under `#[cfg(feature = "xml")]` using `query_perf_soap.xml`, gap fixture `[38, 41, -1, -1]` and `[420, 410, -1, -1]`, `window::<5>()` of `[100, -1, 300]`, exact byte scaling, `Display` for each of the 16 spec wire units (percent raw `4523` displays `45.23 %`; kiloBytes and other-unit gigaBytes match SC-007), `ready_percent` 2.5% for an aggregate sample of 1000 ms / 20 s / 2 vCPUs, blank `ready_percent` for that same sample on a non-aggregate instance with 2 vCPUs and for a vCPU count of 0 or a non-positive interval, and a union grid (`[t1, t2]` plus `[t2, t3]` → holes filled with `-1`, plus `IntervalSubstituted` when the observed interval differs from the requested period).
- [X] T012 [P] [US1] Add failing pure tests in `vim_rs/tests/perf_interval.rs` for the spec interval table: realtime refresh rate; realtime with neither latest-N nor a window leaves start, end, and max samples unset; a realtime start older than 3600 seconds still queries and records `RealtimeWindowTooOld`; past-day with neither latest-N nor a window sets start to now minus the server interval length (86400 seconds on the vCenter fixture), end to now, and omits `max_sample` (SC-011); historical `latest(n)` start `now - (n + 2) * period` with `max_sample` omitted; invalid period 999 → `InvalidInterval`; empty interval list → historical unsupported; all-skipped → `NoSupportedEntities`; automatic VM 20s vs cluster 300s with `AutoIntervalFallback`.

### Implementation for User Story 1

- [X] T013 [US1] Implement normal and CSV decoding in `vim_rs/src/perf/decode.rs`. Downcast only via `as_ref()` / deref (research R5). Map empty CSV tokens and `-1` to `-1`. Build the union timestamp grid and record `IntervalSubstituted`. Shared `Arc<[Timestamp]>` per entity.
- [X] T014 [US1] Implement `RawSeries`, `EntitySeries` (O(1) index), `Series`, `ScaledSeries`, `PerfResult`, `last`, `last_slot`, `window::<N>`, `ready_percent`, and `display_value` in `vim_rs/src/perf/result.rs`. Negative raw values become `None` in windows and are not scaled.
- [X] T015 [US1] Implement `PerfClient::new`, lazy `registry` / `refresh_registry`, `realtime_refresh_rate`, and the per-`MoTypesEnum` provider-cap cache in `vim_rs/src/perf/mod.rs`. `None` perf manager → `NoPerformanceManager`. `InvalidArgument` on summary caches not-a-provider. Not-found is not cached and retries the next entity of that kind (spec FR-10). Do not call vSAN `performance_manager()`.
- [X] T016 [US1] Implement `PerfQuery` in `vim_rs/src/perf/query.rs`: metric and name builders, `Interval` presets (300 / 1800 / 7200 / 86400), `since` / `since_ago` / `between` / `latest`, `WireFormat::Auto` (research R10: CSV on SOAP, CSV on JSON, normal JSON for `latest(1)`), the full interval decision table, greedy packer (research R9, default chunk 256, including `ceil(M / chunk size)` for one object), `JoinSet` + `Semaphore` (default concurrency 4, no `tokio/time`, dropping the fetch cancels in-flight calls), spec fault classes (research R6 and R16), and `fetch` that marks omitted or empty entities `NoSamples`. Warnings, which do not drop a fetch that still has a resolvable counter: `CounterNotOnServer`, `DimensionMismatch`, `LevelTooLow`, `PerDeviceLevelTooLow` (only when both levels are present and the counter level is greater; the metric is still sent; unset ESXi levels do not warn), `ScopeMismatch`, `UnscalableUnit`, `AutoIntervalFallback`, `RealtimeWindowTooOld`, `IntervalSubstituted`. `UnknownCounter` or `DimensionMismatch` fails the fetch only when zero requested counters remain. Re-export the public types from `vim_rs/src/perf/mod.rs` per [contracts/README.md](./contracts/README.md).
- [X] T017 [US1] Port `examples/snippets/src/perf_metrics.rs` to `PerfClient::query().entities().all_available().instances(Instance::All).interval(Interval::Realtime).latest(1).fetch()` and `display_value()`. Remove the hand-built counter map, per-VM provider loop, and `as_any_ref` downcasts. `all_available` may return `NoMetricsAvailable` until T022; keep the call so the snippet matches the contract.
- [X] T018 [US1] Run `cargo test -p vim_rs --test perf_decode --test perf_interval` and `cargo test -p vim_rs --features xml --test perf_decode` for `vim_rs/tests/perf_decode.rs` and `vim_rs/tests/perf_interval.rs`. `cargo check -p snippets --bin perf_metrics` for `examples/snippets/src/perf_metrics.rs`.

**Checkpoint**: One-shot fetch and the diagnostic snippet compile. Formatted samples and gap accessors pass offline.

---

## Phase 4: User Story 2 — Live inventory poller (Priority: P1)

**Goal**: A reusable prepared query polls a changing object list, packs 100×4 counters into two calls, blanks current slots and sparklines, and records no-samples for omitted VMs so the caller can overwrite stale rows.

**Independent Test**: `cargo test -p vim_rs --test perf_query`. Mock shows exactly 2 `QueryPerf` calls (64 then 36). ESXi `[vm, datastore]` is `Ok` on `Auto` and `NoSupportedEntities` on `PAST_DAY`. Omitted entities are `NoSamples`.

### Tests for User Story 2

> Write these first. Packing, format, warning, and fault tests may already pass after T016. Prepared-query, availability, generation, and `time_chunk` assertions fail until T021–T023.

- [X] T019 [US2] Add `PerfMockClient` in `vim_rs/tests/support/perf_mock.rs` implementing `VimClient`: records `PerfQuerySpec` batches (including format and whether `max_sample` was sent), reports `Transport::Json` or `Transport::Soap` (`xml` feature), and injects per-call `MethodFault` / transport / `ParseError` results. Accept an injected `SystemTime` clock so `since_ago` can move between fetches. Do not enable `tokio/time`. Reuse patterns from `vim_rs/tests/support/mock_vim_client.rs`.
- [X] T020 [US2] Add failing tests in `vim_rs/tests/perf_query.rs`: 100 VMs × 4 metrics → 2 calls; one object with more metric IDs than the chunk size splits into `ceil(M / chunk size)` specs; `WireFormat::Auto` records CSV for SOAP, CSV for JSON, and normal for JSON `latest(1)`; a present counter mixed with a missing one returns `Ok` plus `CounterNotOnServer`; zero resolvable counters return `UnknownCounter`; a unit that rejects the requested dimension warns `DimensionMismatch` and, when it is the only counter, returns `DimensionMismatch`; an ESXi counter with no level emits no `LevelTooLow`, while both levels present and a higher counter level records `LevelTooLow` and still sends the metric; a dynamic query that gets no series for an out-of-scope counter records `ScopeMismatch` and stays `Ok`; an unknown other-unit on the unconstrained dimension records `UnscalableUnit`; `NotAuthenticated` aborts and object-scoped `NoPermission` or `InvalidArgument` fails one chunk; dropping the fetch cancels in-flight calls; every chunk failing locally returns the first of those errors; an entity split across chunks stays `Ok` with `-1` holes when one of its chunks succeeds, and `Failed` only when every chunk that contained it failed (research R16); provider-summary `ManagedObjectNotFound` on the first VM retries the second and caches caps; ESXi `[vm, datastore]` cases from spec FR-10; omitted entity is `NoSamples`; two `since_ago` fetches see two clocks and `build` does not freeze the start; `.time_chunk` boundary timestamp is neither dropped nor duplicated; `.all_available()` empty result is `NoMetricsAvailable` and is not cached; the 1024-entry cache evicts the oldest key; `refresh_registry` bumps the generation and the next prepared fetch resolves again.

### Implementation for User Story 2

- [X] T021 [US2] Implement `PreparedQueryBuilder::build` (sync, infallible) and `PreparedQuery::{fetch, client}` in `vim_rs/src/perf/query.rs`. `PreparedQueryBuilder` accepts `since_ago` and does not accept absolute `since` or `between` (those stay on `PerfQuery`; absolute bounds on a prepared query are `fetch_since` in T036). Cache resolved metrics under `CounterRegistry::generation`. Evaluate `since_ago` at each `fetch`, not at `build`. Refreshing the registry bumps that generation so the next fetch resolves again.
- [X] T022 [US2] Implement the 1024-entry FIFO availability cache (`IndexMap`, `shift_remove_index(0)`, ignore poison, never store empty vecs) and `.all_available()` / `.groups()` in `vim_rs/src/perf/query.rs`. Key is `(ManagedObjectReference, interval id)`. Zero ids → `EntityStatus::NoMetricsAvailable` and the entity is not sent.
- [X] T023 [US2] Implement `.time_chunk` historical slices in `vim_rs/src/perf/query.rs` (research R9). Ignore `time_chunk` on realtime. Merge slices with the union grid from T013.
- [X] T024 [US2] Run `cargo test -p vim_rs --test perf_query` for `vim_rs/tests/perf_query.rs` and `vim_rs/tests/support/perf_mock.rs`.

**Checkpoint**: Poller behaviors pass on the mock. External `vtui` is not modified.

---

## Phase 5: User Story 3 — Statically checked counters (Priority: P2)

**Goal**: `cargo run -p vim_build` emits 732 scoped constants. CPU or memory on a datastore, and host-only disk latency on a VM, fail to compile. Datastore listings have no CPU or memory group.

**Independent Test**: `cargo test -p vim_rs --test perf_catalog` and `cargo test -p vim_rs --test perf_scope_fail`. Every generated mask excludes at least one kind. The four illegal snippets fail and the legal VM demand snippet passes.

### Implementation for User Story 3

Generator output must exist before compile-fail snippets can name `cpu::USAGE_AVERAGE`. Implement the generator, then add tests that lock it.

- [X] T025 [P] [US3] Implement `emit_perf_metrics` in `vim_build/src/perf_metrics.rs` per [contracts/generator.md](./contracts/generator.md): snake_case group modules, identifier rules (`SWAP_IN` / `SWAPIN`, `ZIP_SAVED_LATEST` / `ZIPSAVED_LATEST`, rate unsuffixed and absolute `_ABSOLUTE`), three-tier masks from the spec tables, `per_instance`, rustdoc including tier 3b inferred text, and `by_entity` re-exports that omit tier 3b and empty groups. Fail on identifier collisions or unknown rollup/stats. Emit with `vim_build::printer::Printer`.
- [X] T026 [US3] Declare `mod perf_metrics;` from `vim_build/src/lib.rs` and call `emit_perf_metrics` at the end of `emit_vim_bindings` in `vim_build/src/generator.rs`, reading `vim_build/data/perf-catalog.json` and writing `vim_rs/src/perf/metrics.rs`. Missing catalog is a hard error.
- [X] T027 [US3] Run `cargo run -p vim_build` and commit the generated `vim_rs/src/perf/metrics.rs`. Do not hand-edit constants.
- [X] T028 [P] [US3] Implement `EntityKind`, `Admits<K>` (192 impls via macro), `Entity<K>`, and `TryFrom<ManagedObjectReference>` in `vim_rs/src/perf/typed.rs`. Kinds: `VirtualMachine`, `HostSystem`, `ClusterComputeResource`, `ResourcePool`, `Datastore`, `Datacenter`. `#[diagnostic::on_unimplemented]` names the scope and the kind. `EntityKindMismatch` on a wrong MoRef type.
- [X] T029 [US3] Implement `TypedPerfQuery<K>` and `PerfQuery::typed_entity` / `typed_entities` in `vim_rs/src/perf/query.rs` and `vim_rs/src/perf/typed.rs`. `metric` requires `S: Admits<K>`. `metric_unchecked` does not. Fetch reuses T016.
- [X] T030 [US3] Add `vim_rs/tests/perf_catalog.rs`: all 732 constants resolve to the catalog key, `MASK != MASK_ALL`, the three naming pairs above, and `metrics::by_entity::datastore` has no `cpu` or `mem` module.
- [X] T031 [US3] Add `trybuild` tests under `vim_rs/tests/perf_scope_fail/` (research R12): `cpu::USAGE_AVERAGE`, `cpu::USAGE_MAXIMUM`, and `mem::USAGE_AVERAGE` on `Entity<Datastore>` fail; `disk::DEVICE_READ_LATENCY_AVERAGE` on `Entity<VirtualMachine>` fails; `cpu::DEMAND_AVERAGE` on `Entity<VirtualMachine>` passes. Harness in `vim_rs/tests/perf_scope_fail.rs`.
- [X] T032 [US3] Run `cargo test -p vim_rs --test perf_catalog --test perf_scope_fail` for `vim_rs/tests/perf_catalog.rs` and `vim_rs/tests/perf_scope_fail.rs`.

**Checkpoint**: Illegal counter/kind pairs fail before the program runs. Dynamic `.entities()` queries still send and may record `ScopeMismatch`.

---

## Phase 6: User Story 4 — Incremental export (Priority: P2)

**Goal**: A watermark emits each valid sample once per entity, counter, and instance, skips `-1` gaps, and does not pin live VMs to a powered-off VM’s window.

**Independent Test**: `cargo test -p vim_rs --test perf_watermark`. A gap does not move the mark. Two entities in one batch get independent `start_time`. Incremental specs omit `max_sample` when start is set. Dropping the row iterator still advances marks.

### Tests for User Story 4

- [X] T033 [US4] Add failing tests in `vim_rs/tests/perf_watermark.rs` for `rows()` and `filter_new` skipping `raw < 0`, eager mark updates, `entity_high_water` as the minimum mark, `retain_entities` dropping removed devices, and `fetch_incremental` per-entity start with `max_sample` omitted only for entities that already have a mark.

### Implementation for User Story 4

- [X] T034 [US4] Implement `SampleWatermark` in `vim_rs/src/perf/watermark.rs` (`new`, `filter_new`, `entity_high_water`, `min_high_water`, `retain_entities`) per [data-model.md](./data-model.md).
- [X] T035 [US4] Implement `PerfResult::rows` and `Row::scaled` in `vim_rs/src/perf/result.rs`. Rows never carry `raw < 0`.
- [X] T036 [US4] Implement `PreparedQuery::fetch_since` and `fetch_incremental` in `vim_rs/src/perf/query.rs`. `fetch_incremental` does not mutate the watermark. Entities with no mark use the builder window / `latest`.
- [X] T037 [US4] Run `cargo test -p vim_rs --test perf_watermark` for `vim_rs/tests/perf_watermark.rs`.

**Checkpoint**: Exporter loop can poll with a caller-owned timer. The library still has no `tokio::time` dependency.

---

## Phase 7: User Story 5 — Host and guest composite query (Priority: P3)

**Goal**: One host reference returns that host’s series and its child VM series. A non-host fails before any RPC. Resource pools, folders, and clusters are not composite roots.

**Independent Test**: `cargo test -p vim_rs --test perf_composite`. `query_composite.json` yields one host and two children. A VM reference returns `EntityKindMismatch` without a recorded composite call.

### Tests for User Story 5

- [X] T038 [US5] Add failing tests in `vim_rs/tests/perf_composite.rs`: decode `vim_rs/tests/fixtures/perf/query_composite.json` into `CompositePerfResult` (host plus two children); `composite` on a non-`HostSystem` returns `EntityKindMismatch` and the mock records no `query_perf_composite` call; the sent spec uses normal format and omits `max_sample`.

### Implementation for User Story 5

- [X] T039 [US5] Implement `CompositeQuery` and composite decoding in `vim_rs/src/perf/query.rs` and `vim_rs/src/perf/decode.rs`. `PerfClient::composite` lives in `vim_rs/src/perf/mod.rs`. Force `PerfFormatEnum::Normal`. One RPC, no child chunking. Missing host payload is `None`, not an error.
- [X] T040 [US5] Run `cargo test -p vim_rs --test perf_composite` for `vim_rs/tests/perf_composite.rs`.

**Checkpoint**: Composite is limited to a single host plus children.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Docs, optional features, and the quickstart command list. Not in this phase, and not a merge gate: when 0.6.2 ships, publish the Principle III debug and release build times and the optimized `examples/vtui` binary size against the previous release, and run `examples/snippets` plus `vtui` on a real vCenter over VI/JSON and SOAP (Principle IV). The offline commands in T045 are the merge gate.

- [X] T041 [P] Write crate-level rustdoc on `vim_rs/src/perf/mod.rs`: counter identity, `-1` gaps, preset periods vs `PerfInterval.key`, default chunk 256, caller-owned timer, single public path. Generated constant docs already come from T025.
- [X] T042 [P] Document Added `vim_rs::perf` for 0.6.2 in `CHANGELOG.md` and the `perf-chrono` / `perf-polars` rows in `README.md`, including the polars semver carve-out (research R3) and that default builds gain no required dependencies.
- [X] T043 [P] Behind `feature = "perf-chrono"`, add `Timestamp` ↔ `chrono::DateTime<Utc>` conversions in `vim_rs/src/perf/result.rs`.
- [X] T044 [P] Behind `feature = "perf-polars"`, implement `PerfResult::to_dataframe` in `vim_rs/src/perf/polars.rs` with columns matching `Row` (entity, counter, instance, timestamp, interval, raw, scaled).
- [X] T045 Run the offline commands in `specs/006-perf-manager-api/quickstart.md`: `cargo test -p vim_rs`, `cargo test -p vim_rs --features xml`, `cargo test -p vim_rs --features perf-chrono,perf-polars`, and `cargo test -p vim_build`.
- [X] T046 Confirm `cargo build -p vim_rs` does not rewrite `vim_rs/src/perf/metrics.rs`, and that `vim_rs/Cargo.toml` default dependencies are unchanged aside from optional `chrono` / `polars` and dev-only `trybuild`.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies.
- **Foundational (Phase 2)**: Depends on Setup. Blocks every user story.
- **User Story 1 (Phase 3)**: Depends on Foundational. MVP. No dependency on other stories.
- **User Story 2 (Phase 4)**: Depends on User Story 1 (`PerfQuery::fetch` and the decoder).
- **User Story 3 (Phase 5)**: Generator and `Entity<K>` depend only on Foundational (`Metric`, scopes, catalog fixture). `TypedPerfQuery` (T029) also depends on User Story 1’s `query.rs`.
- **User Story 4 (Phase 6)**: Depends on User Story 2 (`PreparedQuery`).
- **User Story 5 (Phase 7)**: Depends on User Story 1 (`PerfClient`, decoder). Does not need User Story 2, 3, or 4.
- **Polish (Phase 8)**: Depends on the stories you intend to ship. Optional features can land as soon as `Timestamp` and `PerfResult` exist (after User Story 1).

### User Story Dependencies

- **US1 (P1)**: After Foundational. Independent.
- **US2 (P1)**: After US1. Independently testable on `PerfMockClient`.
- **US3 (P2)**: After Foundational for codegen and `Entity`. After US1 for typed fetch.
- **US4 (P2)**: After US2.
- **US5 (P3)**: After US1. Independent of US2–US4.

### Parallel Opportunities

- T001 and T002 together.
- T003, T004, T005, T006, and T007 together after T001.
- T011 and T012 together.
- After Foundational, one person can start US1 while another starts T025 and T028 (generator and `Entity`, different files from `query.rs`).
- T041, T042, T043, and T044 together after US1 (T044 needs `PerfResult`).

---

## Parallel Example: User Story 1

```bash
# Tests first, different files:
Task: "T011 failing decode/gap/scale tests in vim_rs/tests/perf_decode.rs"
Task: "T012 failing interval-table tests in vim_rs/tests/perf_interval.rs"

# Then implementation in order:
# T013 decode.rs → T014 result.rs → T015 mod.rs → T016 query.rs → T017 snippet → T018 cargo test
```

## Parallel Example: User Story 3

```bash
# After Foundational, different crates/files:
Task: "T025 emit_perf_metrics in vim_build/src/perf_metrics.rs"
Task: "T028 Entity and Admits in vim_rs/src/perf/typed.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1 and Phase 2.
2. Complete Phase 3 (User Story 1).
3. Stop and run T018. Diagnostic snippet and fixture decode are enough to demo formatted queries.

### Incremental Delivery

1. Setup + Foundational → catalogs resolve.
2. User Story 1 → one-shot query (MVP).
3. User Story 2 → prepared poller and packing proof.
4. User Story 3 → generated constants and compile-time rejection.
5. User Story 4 → incremental export.
6. User Story 5 → host composite.
7. Polish → docs and optional features.

### Parallel Team Strategy

1. Together: Setup and Foundational.
2. Then: Developer A on US1, Developer B on T025/T028 (US3 generator and `Entity`).
3. After US1: Developer A on US2, Developer B finishes T029–T032, Developer C on US5.
4. US4 starts when US2’s `PreparedQuery` lands.

---

## Notes

- Do not hand-edit `vim_rs/src/types/**`, `vim_rs/src/mo/**`, or generated `vim_rs/src/perf/metrics.rs`.
- Do not enable `tokio/time` or add `serde` to `vim_rs`.
- External `/Users/kiril/projects/rust/vtui` is not a task. T021–T022 are what that port needs.
- Commit after each phase checkpoint. The working tree may already contain unrelated edits (`.gitignore`, `examples/Cargo.toml`, `examples/perf-catalog/`, `skills/`); do not mix those into feature commits unless they are the fixture source for T003.
- `checklists/requirements.md` leaves two “no implementation details” boxes open on purpose. That is an accepted exception for this wire contract. Do not strip the normative tables, and do not check those boxes.
