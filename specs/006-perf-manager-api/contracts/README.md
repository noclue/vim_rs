# Contracts: `vim_rs::perf` (0.6.2)

**Feature**: [spec.md](../spec.md) | **Plan**: [plan.md](../plan.md) | **Generator**: [generator.md](./generator.md)

Public consumer contract for the additive 0.6.2 release. Wire tables (units, interval rows, faults, fixtures) are normative in spec.md. This file is the Rust surface those tables attach to.

## Cargo

Default `vim_rs` features are unchanged. New optional features, both off by default:

| Feature | Enables | Semver |
|---------|---------|--------|
| `perf-chrono` | `Timestamp` ↔ `chrono::DateTime<Utc>` | Follows `vim_rs` |
| `perf-polars` | `PerfResult::to_dataframe` | **Not** part of the `vim_rs` semver guarantee. A patch may bump the `polars` 0.x minor. Use `rows()` for a stable schema. |

No new default dependencies. `trybuild` is a dev-dependency only.

`vim_rs::perf` is the only public path. `cargo build -p vim_rs` does not regenerate `metrics.rs`.

## Prelude

`vim_rs::perf` re-exports what generated constants and the snippet need: `PerfClient`, `PerfQuery`, `PreparedQuery`, `PreparedQueryBuilder`, `CompositeQuery`, `Instance`, `Interval`, `WireFormat`, `Metric`, `Rollup`, `StatsKind`, dimension markers (`Ratio`, `Memory`, `Frequency`, `Time`, `Throughput`, `Power`, `Energy`, `Temperature`, `Count`, `AnyDimension`), scaled newtypes, scope aliases, `PerfResult`, `CompositePerfResult`, `EntitySeries`, `EntityStatus`, `UnsupportedReason`, `Sample`, `Row`, `Timestamp`, `SampleWatermark`, `PerfError`, `PerfWarning`, `CounterId`, `CounterRegistry`.

Submodules stay public: `metrics`, `typed`, `units`, `entity`, `query`, `registry`, `result`, `error`.

## `PerfClient`

| Method | Contract |
|--------|----------|
| `new(client) -> Result<Self, PerfError>` | Fails with `NoPerformanceManager` when `service_content().perf_manager` is `None`. Cheap; does not fetch the catalog. |
| `query() -> PerfQuery` | One-shot builder. |
| `prepare() -> PreparedQueryBuilder` | `build()` is sync and infallible. |
| `composite(host) -> CompositeQuery` | `fetch` checks `host` type is `HostSystem` before any RPC. |
| `registry() -> Result<Arc<CounterRegistry>>` | Loads once (`perf_counter` + `historical_interval`) and caches. |
| `refresh_registry()` | Replaces the cache. New generation. In-flight fetches keep the `Arc` they already loaded. |
| `realtime_refresh_rate(entity) -> Result<Duration>` | Probe that entity’s summary. Error if realtime is unsupported or `refresh_rate` is missing or not positive. |
| `raw() -> &PerformanceManager` | Escape hatch to the generated stub. |
| `decode(raw) -> Result<PerfResult>` | Same decoder `fetch` uses. Test and advanced entry. Does not invent entities that were not in `raw` (no requested-entity list). |

`PerfClient` is `Clone` (`Arc` inner). Provider-cap cache and availability cache are shared across clones.

## Builders

`PerfQuery` accepts `metric`, `metric_with_instance`, `metric_by_name`, `metrics_by_name`, `all_available`, `groups`, `instances`, `interval`, `since`, `since_ago`, `between`, `latest`, `format`, `chunk_size` (≥ 1, default 256), `time_chunk`, `concurrency` (≥ 1, default 4). `PreparedQueryBuilder` accepts those same methods except absolute `since` and `between`. A prepared query stores `since_ago` only; absolute bounds are `PreparedQuery::fetch_since`.

`typed_entity` / `typed_entities` consume the untyped builder, replace any entities already added, and return `TypedPerfQuery<K>`. Further entities must be the same `K`. `metric` requires `S: Admits<K>`. `metric_unchecked` does not. Interval, time, format, and chunk methods are available on the typed builder and return `Self`.

`Instance`: `Aggregate` (`""`), `All` (`"*"`), `Named(String)`. `is_aggregate` is true only for the empty wire string.

`Interval`: `Realtime`, `Historical(i32)`, `Auto` (default). Constants `PAST_DAY` (300), `PAST_WEEK` (1800), `PAST_MONTH` (7200), `PAST_YEAR` (86400) are sampling periods.

`WireFormat`: `Auto` (default), `Normal`, `Csv`. Selection is [research R10](../research.md).

### `PreparedQuery`

| Method | Contract |
|--------|----------|
| `fetch(entities)` | Applies the built interval and window. `since_ago` is subtracted from `SystemTime::now()` on this call, not at `build`. |
| `fetch_since(entities, since)` | One start time for every entity. Omits `max_sample`. Overrides `since_ago`. |
| `fetch_incremental(entities, watermark)` | Per-entity start from `watermark.entity_high_water`. Omits `max_sample` for entities that have a mark. Entities with no mark use the builder window / `latest`. Does not mutate the watermark; the caller runs `filter_new`. |

### `CompositeQuery`

`metric` / `metric_by_name` / `interval` / `since` / `since_ago` / `between`, then `fetch() -> Result<CompositePerfResult, PerfError>`. Forces normal format. Does not send `max_sample`. When `latest(n)` is set, each decoded series is truncated to the newest `n` slots. Does not chunk child VMs (one composite RPC). Non-host → `EntityKindMismatch` before I/O.

## Resolution and fetch

1. Load registry (unless `decode` only).
2. Resolve metrics. Warnings for missing counters and dimension mismatches. Error if none remain.
3. Probe provider caps per distinct managed-object type (spec FR-10).
4. Decide each entity with the spec interval table.
5. Expand `.all_available()` / `.groups()` through the availability cache. Entities with zero ids become `NoMetricsAvailable` and are not sent.
6. Pack by metric-id count in request order ([research R9](../research.md)). Interval, start, end, and max-sample stay on each spec and do not split the call.
7. Run chunks. Classify faults with the spec fault table ([research R6](../research.md)).
8. Decode, union-align, attach status for every requested entity.

Level warnings (`LevelTooLow`, `PerDeviceLevelTooLow`) when both sides are `Some` and the counter’s level is greater. The metric is still sent. Unset levels never warn.

Dynamic queries that receive no series for a counter whose scope excludes that entity’s type record `ScopeMismatch` and still return `Ok`.

`start_time` on the wire is exclusive; `end_time` is inclusive. Callers pass the same instants they mean; the client does not shift them.

Historical `latest(n)` without a start: start = now − `(n + 2) * period`, end = now, `max_sample` omitted, decoded series truncated to the newest `n` slots.

Historical preset with neither `latest` nor a window: start = now − `PerfInterval.length`, end = now, `max_sample` omitted.

Realtime with neither `latest` nor a window: start, end, and `max_sample` all omitted.

Realtime start older than 3600 seconds: still query, add `RealtimeWindowTooOld`.

## Results

`PerfResult::entity`, `entities` (request order), `series(metric)` across entities, `rows()` (skips `raw < 0`), `requested_at`, public `warnings`.

`EntitySeries::series`, `series_instance`, `devices` (non-aggregate instances of that metric).

`Series<D>`: `samples`, `last`, `last_slot`, `window::<N>`, `raw`, `unit`, `scaled`. `Series<Time>::ready_percent(vcpus)` follows spec FR-07.

`ScaledSeries<D>`: `samples`, `last`, `last_slot`, `window::<N>`, `to_vec_f64`. Negative raw slots are `None` in scaled windows and are not passed to `Dimension::scale`.

`RawSeries::display_value` uses the spec display column for that unit.

Scaled `Display` formats are the spec unit table (two-decimal percent, binary IEC bytes, and so on). `Bytes::as_u64` saturates (negative → 0, above `u64::MAX` → `u64::MAX`).

## Watermark

`SampleWatermark`: `new`, `filter_new(&mut self, &PerfResult) -> vec::IntoIter<Row>`, `entity_high_water`, `min_high_water`, `retain_entities`. See [data-model.md](../data-model.md). `filter_new` updates marks for every new valid sample before returning, so dropping the iterator early does not skip marks.

## Errors

`PerfError` is `#[non_exhaustive]` and `thiserror`. `Client(client::Error)` is `#[from]`, wrapping `vim_rs::core::client::Error`. Variants and the fetch-level versus per-entity split are in [data-model.md](../data-model.md).

`PerfWarning` is `#[non_exhaustive]`. Callers match with a wildcard.

`EntityStatus` is `#[non_exhaustive]`: `Ok`, `NoMetricsAvailable`, `NoSamples`, `Unsupported(UnsupportedReason)`, `Failed(Arc<PerfError>)`.

`UnsupportedReason`: `NotAProvider`, `RealtimeUnsupported`, `HistoricalUnsupported`.

## Decoder obligations

- Accept `PerfEntityMetric` and `PerfEntityMetricCsv`. Fixtures must match on entities, timestamps, values, and statuses, ignoring `requested_at`.
- CSV: split `sample_info_csv` and series value strings on commas. Empty token and `-1` both become `-1`.
- Downcast only through `as_ref()` / deref. See [research R5](../research.md).
- `query_perf` returning `Ok(None)` or `Ok(Some(vec![]))` still lists every requested entity as `NoSamples`.
- Union grid: sorted unique timestamps per entity; holes filled with `-1`; one shared `Arc<[Timestamp]>` per entity.
- SOAP fixture decoded only under `#[cfg(feature = "xml")]`.

## Non-goals (stable for 0.6.2)

- Background interval or `Stream`.
- `OptionManager.QueryOptions` for max query metrics.
- Bisection on `ManagedObjectNotFound`.
- Composite roots other than `HostSystem`.
- vSAN performance manager.
- Persisting the registry across processes.
- A `core::perf` re-export.
- Hand-edited `metrics.rs`.
