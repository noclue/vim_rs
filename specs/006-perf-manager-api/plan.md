# Implementation Plan: Ergonomic Performance Manager API

**Branch**: `006-perf-manager-api` | **Date**: 2026-10-24 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/006-perf-manager-api/spec.md`, plus Part II of `docs/vim_rs-perf-spec.md` (v5). `docs/**` is gitignored. This plan and its sibling artifacts are the tracked design. Normative wire tables stay in [spec.md](./spec.md) §“Normative Wire & Probe Rules” and are not restated here.

**Spec-quality checklist**: `checklists/requirements.md` leaves two “no implementation details” boxes open on purpose (the wire contract is the subject matter). That is not a constitution-gate failure.

## Summary

Add a hand-written `vim_rs::perf` module that turns `PerformanceManager` into named, unit-scaled, chunked queries, plus a generated counter catalog (`vim_rs/src/perf/metrics.rs`) emitted by `vim_build` from a sanitized vCenter 8.0.3 catalog. Callers get one-shot queries, a reusable prepared query for live polls, per-entity incremental watermarks, compile-time entity scopes, and a host-plus-children composite query. The default build adds no dependencies. `chrono` and `polars` are opt-in features. Release target is `vim_rs` 0.7.0 because `vim_rs::core::client::Error` is now `Clone`, with `MethodFault` and `ReqwestError` each holding an `Arc`. There is no `vim_rs::core::perf` re-export and no library-owned poll timer.

## Technical Context

**Language/Version**: Rust 2021, `rust-version = "1.86"` (`vim_rs` today). Const generics and `#[diagnostic::on_unimplemented]` are in range.

**Primary Dependencies**: Existing `vim_rs` stack only on the default path: `tokio` 1.44 (`rt-multi-thread`, `macros`, `sync` — not `time`), `indexmap` 2.9, `thiserror` 2, `miniserde` 0.1, `log`. Optional: `chrono` 0.4 (`perf-chrono`), `polars` current stable 0.x (`perf-polars`, semver carve-out). Dev-only: `trybuild` for compile-fail scope tests. Generator side already has `serde` / `serde_json` in `vim_build`.

**Storage**: In-memory only. Sanitized catalog JSON committed at `vim_build/data/perf-catalog.json` and copied to `vim_rs/tests/fixtures/perf/vcenter_catalog.json`. No cross-process cache.

**Testing**: `cargo test -p vim_rs` offline against fixtures; `cargo test -p vim_rs --features xml` for the SOAP fixture; `cargo test -p vim_rs --features perf-chrono,perf-polars` for the optional converters. Mock client records `PerfQuerySpec` batches. No live vCenter required for the merge gate.

**Target Platform**: Library, same targets as `vim_rs` (Linux, macOS, Windows).

**Project Type**: Rust library crate plus `vim_build` generator and one in-tree snippet.

**Performance Goals**: 100 entities × 4 counters at the default chunk size of 256 metric IDs issue exactly 2 `QueryPerf` calls. Series lookup by `(counter id, instance)` is O(1) after name resolution. Repeat `series()` resolution and the single-payload decoder are specified in [fastpath.md](./fastpath.md) and implemented there. Memory scaling stays integer (`i128`). Dropping a `fetch` future cancels in-flight calls.

**Constraints**: No edits under `vim_rs/src/types/` or `vim_rs/src/mo/` except consuming them. No `serde` / `serde_json` in `vim_rs`. No `tokio/time` in `vim_rs`. No second public module path. Generated `metrics.rs` is rewritten only by `cargo run -p vim_build`, never by `cargo build -p vim_rs`.

**Scale/Scope**: 732 generated counters, 26 group modules, 6 typed entity kinds, 16 wire units, 4 historical presets. New hand-written modules under `vim_rs/src/perf/`. One snippet port (`examples/snippets/src/perf_metrics.rs`). External `vtui` is a consumer example, not a merge gate.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle / standard | Status | Evidence |
|---------------------|--------|----------|
| **I. Generated from authoritative specs** | Pass | `metrics.rs` is emitted by `vim_build` from committed `perf-catalog.json`. Hand edits to that file are forbidden. `types/` and `mo/` stay untouched. The perf runtime is hand-written, like `core/`. |
| **II. Complete, type-safe VIM surface** | Pass | Existing `PerformanceManager` stub remains. New typed layer rejects illegal counter/kind pairs before run. Custom metrics are the documented escape hatch (`Metric::custom`, `metric_unchecked`), not a silent type erasure. |
| **III. Build-time & binary-size budgets** | Pass | Zero default dependencies. 732 `const` items are negligible next to `structs.rs`. `chrono` and `polars` are off by default. `trybuild` is dev-only. Release notes still compare debug/release time and `examples/vtui` size when 0.6.2 ships. |
| **IV. Multi-transport parity** | Pass | One decoder for normal JSON, CSV JSON, and SOAP XML (`xml` feature). Automatic format is CSV on SOAP and CSV on JSON except `latest(1)`, which uses normal JSON. Fixtures must decode to the same series. |
| **V. Documented public surfaces** | Pass | Rustdoc on the public module, generated constants carry counter name, unit, level, and tier. Snippet `perf_metrics.rs` is the runnable sample. CHANGELOG Added entry for 0.6.2. |
| **VI. Readable source at scale** | Pass | Split listed under Project Structure. Generated constants are one line each with a rustdoc header. Decoder downcast rule is commented at the call site (the `Box<dyn>` trap). |
| **VII. Ecosystem tooling** | Pass | `vim_build` grows `emit_perf_metrics` beside OpenAPI emission. `vim_macros` and MCP are unchanged. |
| **Async-first I/O** | Pass | `fetch` is async. Chunks use `tokio::sync::Semaphore` and `tokio::task::JoinSet` on the caller’s runtime. No `block_in_place`. No `tokio::time` inside the library; callers own the ticker. |
| **Marshalling (miniserde)** | Pass | Wire decode reuses generated VIM types. Catalog JSON inside `vim_rs` is parsed with `miniserde::json::Value`. `serde_json` stays in `vim_build` only. |
| **Errors** | Pass | `PerfError` wraps `vim_rs::core::client::Error` (`#[from]`). It does not replace `core::Error`. Server faults stay `MethodFault` values, classified by `StructType`. |
| **Security & secrets** | Pass | No credentials in result types or logs. Catalog fixtures are sanitized (documentation host, zeroed instance id, synthetic MoRefs). |
| **Feature flags** | Pass | `perf-chrono` and `perf-polars` are opt-in, default-features off, and documented with their cost. `polars` is outside the crate semver guarantee (see research R3). |

**Post-design re-check**: [research.md](./research.md), [data-model.md](./data-model.md), [contracts/](./contracts/), and [quickstart.md](./quickstart.md) match this table. No Complexity Tracking exception.

## Project Structure

### Documentation (this feature)

```text
specs/006-perf-manager-api/
├── plan.md              # This file
├── research.md          # Phase 0
├── data-model.md        # Phase 1
├── quickstart.md        # Phase 1
├── contracts/           # Phase 1 (public API + generator contract)
├── spec.md
├── fastpath.md          # series lookup and single-payload decode
├── checklists/requirements.md
└── tasks.md             # /speckit-tasks (not created here)
```

### Source Code (repository root)

```text
vim_rs/src/lib.rs                         # pub mod perf;
vim_rs/src/perf/
├── mod.rs                                # PerfClient, prelude re-exports
├── units.rs                              # Dimension, scaled values, Display
├── entity.rs                             # Scope<MASK>, kind bits, aliases
├── metric.rs                             # Metric<D, S>, Rollup, StatsKind
├── metrics.rs                            # GENERATED (phase 3); not written by cargo build
├── registry.rs                           # CounterRegistry, ProviderCaps
├── query.rs                              # PerfQuery, TypedPerfQuery, PreparedQuery, CompositeQuery
├── decode.rs                             # normal + CSV + composite; union grid
├── result.rs                             # PerfResult, Series, Row, Timestamp
├── watermark.rs                          # SampleWatermark
├── error.rs                              # PerfError, PerfWarning, EntityStatus
├── typed.rs                              # Entity<K>, Admits<K>
└── polars.rs                             # cfg(feature = "perf-polars")
vim_rs/tests/fixtures/perf/               # sanitized catalogs + wire fixtures
vim_rs/tests/support/perf_mock.rs         # records PerfQuerySpec, injects faults
vim_rs/tests/perf_*.rs                    # offline tests + trybuild compile-fail
vim_build/data/perf-catalog.json          # same bytes as vcenter_catalog.json
vim_build/src/perf_metrics.rs             # emit_perf_metrics (new)
vim_build/src/generator.rs                # call emit after OpenAPI emission
examples/snippets/src/perf_metrics.rs     # port off raw PerformanceManager
```

**Structure Decision**: Runtime code is a new sibling of `core` / `mo` / `types`, not a submodule of `core` (`core/root_objects.rs` already uses `performance_manager()` for vSAN). The managed-object stub stays generated and is called from `PerfClient`. Catalog constants are generated; everything else is hand-written.

## Complexity Tracking

None. Optional heavy crates are feature-gated. The generated catalog is a const table, not a new serialization stack.

---

## Phase 0: Research

**Status**: Complete → [research.md](./research.md)

Part II already chose the API. Research locks the choices that would otherwise drift during implementation: miniserde for catalog JSON, `SystemTime` timestamps, existing Tokio features, the `Box<dyn>` downcast, `core::client::Error` as the wrap target, `NoMetricsAvailable` versus `NoSamples`, process-wide registry generations, greedy packing, and `trybuild` as a dev-dependency only.

No open **NEEDS CLARIFICATION** items.

---

## Phase 1: Design

**Status**: Complete → [data-model.md](./data-model.md), [contracts/README.md](./contracts/README.md), [contracts/generator.md](./contracts/generator.md), [quickstart.md](./quickstart.md)

### Runtime shape

`PerfClient::new` reads `service_content().perf_manager` and builds `mo::PerformanceManager`. `query()` / `prepare()` / `composite()` are the three entry points. Provider capabilities are cached per `MoTypesEnum`. The counter registry is lazy, behind a generation counter, and refreshed explicitly. Fetch resolves the [spec interval table](./spec.md) per entity, packs metric IDs, optionally slices historical time, runs chunks with bounded concurrency, and merges each entity onto one timestamp grid.

### Generated catalog

`cargo run -p vim_build` reads `vim_build/data/perf-catalog.json` and writes `vim_rs/src/perf/metrics.rs` through the existing printer. Naming, collision suffixes, and the three scope tiers are normative in [contracts/generator.md](./contracts/generator.md) and the spec’s scope tables. No generated constant uses the any-entity mask.

### What stays out

No poll task, no `QueryOptions` RPC, no missing-object bisection, no composite root except `HostSystem`, no vSAN performance manager, no hand-maintained counter list. The series-lookup and single-payload decoder follow-on is specified in [fastpath.md](./fastpath.md).

**Post-design constitution re-check**: Pass, with no exception added.

---

## Phase 2: Implementation tasks (preview for `/speckit-tasks`)

Not created by this command. Phases compile and test using only earlier deliverables. Order is fixed because later phases import earlier types.

| Phase | Scope | Done when |
|-------|--------|-----------|
| **0. Fixtures** | Sanitize and commit the vCenter catalog (both paths), ESXi catalog, and the five wire fixtures named in the spec | Files parse; vCenter fixture and generator catalog are identical |
| **1. Units, registry, decoder** | `units`, `entity`, `metric`, `registry`, `decode`, `result`, `error`. Five hand-written test metrics, not `metrics.rs` yet | 732 and 353 counters resolve; normal, CSV, and SOAP fixtures match; gap and union-grid tests pass |
| **2. Query execution** | `PerfClient`, builders, mock, chunker, provider cache, fault policy. Port `perf_metrics.rs` | 100×4 packs into 2 calls; ESXi mixed batch and fault-isolation tests pass |
| **3. Codegen and typed queries** | `emit_perf_metrics`, `metrics.rs`, `typed.rs`, `trybuild` | Every generated mask excludes some kind; datastore CPU and VM host-only disk fail to compile |
| **4. Watermark, composite, optional features** | `SampleWatermark`, `fetch_incremental`, `CompositeQuery`, `perf-chrono`, `perf-polars` | Powered-off VM does not pin live VMs; composite fixture decodes; optional features off by default |

Suggested task groups inside those phases: fixtures, pure library modules, mock and chunker tests, generator, snippet, changelog and rustdoc.
