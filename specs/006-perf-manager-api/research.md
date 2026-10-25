# Research: Ergonomic Performance Manager API

**Feature**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

**Date**: 2026-10-24. Decisions below close Part II of `docs/vim_rs-perf-spec.md` against the `vim_rs` 0.6.1 tree (`rust-version = "1.86"`, `vim_rs` version `0.6.1`). No item is left as NEEDS CLARIFICATION.

## R1 — Where the module lives, and which managed object it wraps

**Decision**: Public path is `vim_rs::perf` only, declared with `pub mod perf;` in `vim_rs/src/lib.rs`. `PerfClient::new` takes `VimClientHandle`, reads `client.service_content().perf_manager` (`Option<ManagedObjectReference>`), and constructs `vim_rs::mo::PerformanceManager`. If that field is `None`, return `PerfError::NoPerformanceManager`. Do not call `core/root_objects.rs` `performance_manager()`; that helper is the vSAN object.

**Rationale**: The vSphere performance manager and the vSAN performance manager are different managed objects. A second re-export under `core` would split the public path the spec forbids.

**Alternatives considered**:

- Put the module at `vim_rs::core::perf` and re-export it. Rejected. Two paths, and `core` is the session/client layer.
- Hand-edit `vim_rs/src/mo/performance_manager.rs` to grow ergonomic methods. Rejected. That file is generated (Principle I).

## R2 — Catalog JSON inside `vim_rs` uses miniserde, not serde

**Decision**: `CounterRegistry::from_catalog_json` parses with `miniserde::json::from_str::<miniserde::json::Value>` and walks the `vim_rs.perf-catalog/1` document. `vim_build` keeps using `serde_json`, which it already depends on, to read the same file while emitting `metrics.rs`.

**Rationale**: Constitution marshalling rule forbids adding `serde` / `serde_json` to the `vim_rs` crate. miniserde is already the crate’s JSON stack and can parse arbitrary JSON as `Value`. The generator is a separate crate whose serde dependency is pre-existing.

**Alternatives considered**:

- `serde` derive on a catalog struct inside `vim_rs`. Rejected. New default dependency and a second marshalling stack.
- Share a serde struct from `examples/perf-catalog`. Rejected. The library must not depend on the examples workspace, and that crate is a probe tool.

## R3 — Timestamps, `perf-chrono`, and `perf-polars`

**Decision**: `Timestamp` is a newtype over `std::time::SystemTime`. RFC 3339 parse and UTC format are hand-written for the VIM subset (offset or `Z`, optional fractional seconds). `perf-chrono` adds `Timestamp` conversions to `chrono::DateTime<Utc>` only. `perf-polars` adds `PerfResult::to_dataframe`. Both features are optional, `default-features = false`, and off by default.

```toml
perf-chrono = ["dep:chrono"]
perf-polars = ["dep:polars"]

chrono = { version = "0.4", optional = true, default-features = false, features = ["std"] }
polars = { version = "0.x", optional = true, default-features = false }
```

Pin `polars` to the stable 0.x minor current at implementation time (the design note used 0.46 as a placeholder, not a frozen pin). Document in README and CHANGELOG that `perf-polars` is outside the `vim_rs` semver guarantee: a later patch may bump the polars minor. Callers who need a stable frame schema use `PerfResult::rows()` instead.

**Rationale**: FR-25 forbids default dependencies. `chrono` on the default path would tax every consumer for a conversion most of them do not need. Polars breaks its Rust API on most minors, so pretending it follows `vim_rs` semver would be false.

**Alternatives considered**:

- `SystemTime` only, no RFC 3339 helper. Rejected. CSV sample info and `since` strings are RFC 3339 on the wire.
- Always depend on `chrono`. Rejected. Default-build cost.
- Enable `chrono`’s `clock` / `serde` features. Rejected. Unused, and `serde` must not leak in.

## R4 — Concurrency without enabling `tokio/time`

**Decision**: Chunk fan-out uses `tokio::sync::Semaphore` and `tokio::task::JoinSet`, both available from the existing `tokio` features (`rt-multi-thread` implies `rt`; `sync` is already enabled). Do not add the `time` feature. Dropping the `JoinSet` (including when the caller drops the `fetch` future) aborts the tasks, which drops the in-flight request futures. Do not spawn work onto a detached task that outlives `fetch`.

**Rationale**: The spec gives cancellation and a default concurrency of 4 to the library, and gives the poll timer to the caller. `examples` and external `vtui` already enable `tokio/time` themselves. The §10.4 snippet’s `tokio::time::interval` stays in the caller.

**Alternatives considered**:

- Sequential chunks. Rejected. The spec requires bounded concurrency.
- `futures::stream::buffer_unordered` and a new `futures` dependency. Rejected. Tokio is already present.
- Enable `tokio/time` “just in case”. Rejected. Unused feature weight on every build.

## R5 — Trait-object downcast

**Decision**: Downcast `Box<dyn PerfEntityMetricBaseTrait>` and `Box<dyn PerfMetricSeriesTrait>` only after deref: `box_val.as_ref().as_any_ref().downcast_ref::<...>()` (equivalently `(*box_val).as_any_ref()`). Never call `as_any_ref()` on the `Box` itself.

**Rationale**: `AsAny` is implemented for `Box<T>`, so `box.as_any_ref()` downcasts to `Box<dyn _>` and every concrete downcast fails. Inherited fields are reached through `Deref` (`entity` on `PerfEntityMetricBase`, `id` on `PerfMetricSeries`). This was verified against `vim_rs` 0.6.1 structs.

**Alternatives considered**:

- Match on `PerfFormatEnum` and also sniff the CSV string as a fallback. Keep the format flag as the selector; still accept whichever concrete type the box holds so a server that ignores the requested format does not fail the decode. If neither concrete type downcasts, that chunk is `ParseError` (chunk-local).

## R6 — Error wrap target and fault class

**Decision**: `PerfError::Client` wraps `vim_rs::core::client::Error` (the transport enum in `core/client.rs`), not `vim_rs::core::Error`. Fatal chunk failures are:

- `Error::MethodFault` whose `type_` is `StructType::NotAuthenticated` or `StructType::InvalidLogin`
- `vim_rs::core::client::is_request_canceled_error` (`StructType::RequestCanceled`)
- every other `client::Error` variant except `MethodFault` and `ParseError` (`ReqwestError`, `MissingOrInvalidSessionKey`, `InvalidObjectType`, `CannotNegotiateAPIRelease`)

`ParseError` and every other `MethodFault` (including object-scoped `NoPermission`) are chunk-local. Provider-summary `InvalidArgument` is not this table: it is cached as “not a provider” for that `MoTypesEnum` (spec FR-10).

**Rationale**: `is_request_canceled_error` and `MethodFault(structs::MethodFault)` live on `client::Error`. `core::Error` is a different type. Object-scoped `NoPermission` shares the fault family with authentication faults and must be split on `StructType`, not on the Rust variant.

**Alternatives considered**:

- Map everything into `core::Error`. Rejected. ADR-08 keeps performance failures in `PerfError`.
- Treat all `MethodFault`s as fatal. Rejected. One object’s `InvalidArgument` or `NoPermission` must not drop sibling chunks (ADR-06).

## R7 — `NoMetricsAvailable` versus `NoSamples`

**Decision**:

- `EntityStatus::NoMetricsAvailable` — the object is a supported provider, but the request’s availability filter (`.all_available()` or `.groups()`) produced zero metric IDs, so it was not sent in `QueryPerf`.
- `EntityStatus::NoSamples` — the object was included in a successful chunk and the server omitted it or returned no timestamps or values (powered off, deleted, empty series). Spec FR-19.
- `Unsupported` — skipped by the interval table before `QueryPerf` (not a provider, realtime unsupported, historical unsupported).
- `Failed` — that object’s chunk failed locally (`Arc<PerfError>`).

Explicit named metrics that are missing on the server follow FR-02 (warning, or a fetch-level error if none remain). They do not use `NoMetricsAvailable`.

**Rationale**: Part II’s enum has both variants. Part I defines omitted and empty responses as no-samples. The only remaining meaning that does not collapse the two is “nothing to ask for” versus “asked and got nothing”.

**Alternatives considered**:

- Drop `NoMetricsAvailable` and use `NoSamples` for both. Rejected. A poller cannot tell “this datastore has no counters in the requested groups” from “this VM was powered off”.
- Use `NoMetricsAvailable` for an entity that returns a series of all `-1`. Rejected. Those slots are real timestamps; `last_slot` already blanks them.

## R8 — Registry generation

**Decision**: A process-wide `AtomicU64` starts at 1 and increments on every `CounterRegistry` construction (`from_wire` and `from_catalog_json`). `PreparedQuery` caches `(generation, resolved metrics)` and resolves again when `registry.generation()` differs. `refresh_registry` replaces the `Arc<CounterRegistry>` and therefore bumps the generation even if the counter list is unchanged.

**Rationale**: FR-21 requires lazy resolution with no ABA if a registry is rebuilt at the same address. An `Arc` pointer compare would be wrong if the allocator reused the allocation. A monotonic counter cannot.

**Alternatives considered**:

- Compare `Arc::ptr_eq` only. Rejected. ABA.
- Store the generation only on `PerfClient` and not on the registry. Rejected. Offline registries used in tests must also invalidate a prepared query if the caller swaps them; putting the counter on the registry value matches `CounterRegistry::generation(&self)`.

## R9 — Greedy packing and historical time slices

**Decision**: Walk entities in request order. Maintain one open `QueryPerf` batch whose summed `|metric_id|` is at most `chunk_size` (default 256, clamped to at least 1). An entity that fits in the remainder is appended. An entity with `M > chunk_size` IDs is split into contiguous specs of length `chunk_size` (last spec shorter). A full split closes the batch. A short tail may share a batch with later entities that still fit. Do not reorder entities to pack tighter. Specs in one call keep their own `intervalId`, `startTime`, `endTime`, and `maxSample`. Those fields do not split the batch.

`.time_chunk(window)` on a bounded historical query emits slices `(t0, t1], (t1, t2], …` with `t_i = min(t_{i-1} + window, end)`, slice start exclusive and slice end inclusive. It is ignored for realtime. Slices are packed with everything else by metric-id count, so two slices of one entity can share a call. The merger then builds one union timestamp grid per entity across chunks and slices, filling holes with `-1`.

**Rationale**: FR-15’s acceptance test is 100 VMs × 4 metrics → batches of 64 and 36, which is first-fit in request order, not best-fit. Exclusive start plus inclusive end is why the shared boundary timestamp is neither dropped nor duplicated (FR-16). A read-only probe of vCenter 8 (`vc8.home`, JSON, 2026-10-25) accepted one `QueryPerf` that mixed realtime `20` with historical `300` and `1800`, different starts, different `maxSample`, CSV with normal, a VM with a host and a cluster, and two windows for the same VM. Each spec came back with its own interval and window. Historical `maxSample` was ignored by that server even for a single spec, which is why the client still omits it (FR-13). Realtime `maxSample` was honored per spec.

**Alternatives considered**:

- One RPC per entity. Rejected. That is the friction the feature removes.
- Best-fit reordering. Rejected. Harder to test, and the spec says request order.
- Group calls by identical interval and time window. Rejected after the vCenter probe. Incremental watermarks and `Interval::Auto` mixes were issuing one call per distinct window.

## R10 — Wire format

**Decision**: `WireFormat::Auto` (the default) selects `PerfFormatEnum::Csv` when `client.transport()` is `Transport::Soap` (`#[cfg(feature = "xml")]`), and `PerfFormatEnum::Csv` on `Transport::Json` unless `.latest(1)` was set on the builder, in which case `PerfFormatEnum::Normal`. `CompositeQuery` always sends `Normal` and never sends `max_sample`, regardless of `WireFormat`. Empty CSV tokens and the token `-1` both decode as `-1`.

**Rationale**: SOAP normal format wraps every sample in its own element (FR-08). JSON `latest(1)` is small enough that normal format avoids a CSV split. Composite responses were probed as normal entity metrics (spec fixture `query_composite`).

**Alternatives considered**:

- Always CSV, including composite and `latest(1)`. Rejected. Composite fixture and FR-23 require normal; FR-08 requires normal for JSON latest-1.

## R11 — Two committed copies of one catalog

**Decision**: Byte-identical sanitized vCenter document at `vim_build/data/perf-catalog.json` and `vim_rs/tests/fixtures/perf/vcenter_catalog.json`. A unit test asserts the bytes match. ESXi catalog and wire fixtures live only under `vim_rs/tests/fixtures/perf/`. Sanitization: `source.server = "vcenter.example.internal"`, `instance_uuid` zeroed, sample MoRefs replaced with synthetic ids. `cargo build -p vim_rs` does not read or write these files. Only `cargo run -p vim_build` writes `metrics.rs`.

**Rationale**: ADR-07. `docs/**` and `examples/perf-catalog/out/` are gitignored. Library tests must not read `vim_build/`. The probe crate `examples/perf-catalog` is the producer of the dump; it is not linked into `vim_rs` and is not required to compile the library.

**Alternatives considered**:

- `include_str!` the generator data file from `vim_rs` tests via a relative path out of the crate. Rejected. Cross-crate path, breaks packaged test layouts.
- Generate `metrics.rs` in `build.rs`. Rejected. Principle III (every consumer rebuild would parse the catalog) and the spec line that ordinary `cargo build -p vim_rs` must not overwrite the file.

## R12 — Compile-fail tests

**Decision**: Dev-dependency `trybuild` on `vim_rs`, with `tests/perf_scope_fail/` snippets that must fail for `cpu::USAGE_AVERAGE`, `cpu::USAGE_MAXIMUM`, and `mem::USAGE_AVERAGE` on `Entity<Datastore>`, and for `disk::DEVICE_READ_LATENCY_AVERAGE` on `Entity<VirtualMachine>`. A passing snippet queries `cpu::DEMAND_AVERAGE` on `Entity<VirtualMachine>`.

**Rationale**: FR-26.4 is a compile-time outcome. A runtime assertion cannot see it. `trybuild` is not compiled into the published library and does not change the default dependency set of dependents.

**Alternatives considered**:

- A `cargo check` child process inside an integration test. Rejected. Slower and depends on the cargo binary layout.
- Doctest compile-fail. `vim_rs` sets `doctest = false`.

## R13 — Scope bits and when to warn

**Decision**: Bit assignments are exactly the spec’s masks (`VM=0x01` through `DATACENTER=0x20`, `MASK_ALL=0x3F`). `mo_type_bit` maps `VirtualApp` to the resource-pool bit and `StoragePod` to the datastore bit. `EntityKind` is implemented only for `VirtualMachine`, `HostSystem`, `ClusterComputeResource`, `ResourcePool`, `Datastore`, and `Datacenter`. `Admits<K>` is implemented for every mask `1..=0x3F` that contains `K::BIT` (192 impls) via one macro. `AnyEntity` (`MASK_ALL`) is used only by `Metric::custom`.

Dynamic `.entities(&[ManagedObjectReference])` does not reject out-of-scope metrics at runtime. If the server returns no series for that counter on that entity, record `PerfWarning::ScopeMismatch` and continue. Typed `.metric()` does not need the warning for combinations that fail to compile; `.metric_unchecked` still sends the query and may warn.

Tier 3a and tier 3b group masks are the tables in spec.md. The generator must not re-infer them.

**Rationale**: ADR-02 and FR-24. Runtime rejection would false-fail tier-3b counters the caller deliberately widened, and would hide server-side data when the lab dump was incomplete.

**Alternatives considered**:

- Fail the fetch on scope mismatch. Rejected. ADR-02 says the dynamic path always queries and warns.
- Give every unknown group `AnyEntity`. Rejected. FR-24: zero generated constants use that mask.

## R14 — External vtui is not a merge gate

**Decision**: Port `examples/snippets/src/perf_metrics.rs` in this repo. Do not require a commit in `/Users/kiril/projects/rust/vtui`. The prepared-query shape in the design’s §10.2 is the acceptance behavior (windows, `last_slot`, scaled bytes, omitted entities become `NoSamples` so the caller overwrites the previous row). Document a `[patch.crates-io]` path patch as the downstream integration step in the changelog, not as a CI job.

**Rationale**: Spec assumption. `vtui`’s caret dependency on `0.6.1` accepts `0.6.2` once published; until then a path patch is how that repo tries the API.

**Alternatives considered**:

- Vendor a vtui poller test into `vim_rs`. Rejected. The mock tests already lock the behaviors that poller needs.

## R15 — Historical preset constants

**Decision**: `Interval::PAST_DAY = Historical(300)`, `PAST_WEEK = Historical(1800)`, `PAST_MONTH = Historical(7200)`, `PAST_YEAR = Historical(86400)`. These integers are sampling periods in seconds, not `PerfInterval.key`. Retention (`PerfInterval.length`) still comes from the server interval with that sampling period. On the measured vCenter, past day length is 86400 seconds and past week length is the matching interval’s `length`.

**Rationale**: Spec FR-13 / ADR-09, and vcsim reuses `key = 1` for several periods. Hardcoding the key would query the wrong interval.

**Alternatives considered**:

- Look up presets by the interval’s `name` string (`"Past day"`). Rejected. Names are display text and not stable across locales or vcsim. The sampling period is the wire id.

## R16 — An entity split across chunks

**Decision**: If every chunk that contained an entity failed locally, that entity is `Failed`. If at least one of its chunks succeeded, the entity is `Ok` or `NoSamples` from the successful payloads, the union grid fills metrics that never arrived with `-1`, and sibling entities in the failed chunk that had no successful chunk are `Failed`. The fetch returns `Err` only when every chunk in the call failed, or when a fatal fault aborted the rest.

**Rationale**: ADR-06 isolates a bad chunk so sibling work is kept. A single entity that was split because it exceeded `chunk_size` is in more than one chunk. Marking it `Failed` after a successful sibling chunk would drop samples the merger is required to align (ADR-05). The spec’s “entities in that chunk” rule applies unchanged to entities that appear in only the failed chunk.

**Alternatives considered**:

- Last write wins, so a later failed chunk overwrites a successful one with `Failed`. Rejected. Loses data the caller already paid for.
- Fail the whole fetch on any chunk-local error. Rejected. ADR-06.

## R17 — Series lookup and single-payload decode

**Decision**: Implemented as specified in [fastpath.md](./fastpath.md). A successful `series()` / `series_instance()` / `devices()` hit allocates nothing, and looks up the registry stored on that `EntitySeries` (counter ids differ per server; FR-21’s prepared-query cache stays the fetch-time cache). Decode keeps parallel columns when an entity has one strictly increasing payload, and keeps the union timestamp grid when it has two or more payloads or the single payload is not strictly increasing (FR-17, R16).

**Rationale**: A monitoring poll resolves the same metrics on every entity, every cycle, and most of those fetches are one payload per entity. The grid stays for chunks and time slices.

**Alternatives considered**:

- A process-wide map from metric name to counter id. Rejected in the follow-on. Two live registries can assign different ids to one name (R8).
- Always skip the grid. Rejected. FR-17’s union fill is the merge of several payloads.
