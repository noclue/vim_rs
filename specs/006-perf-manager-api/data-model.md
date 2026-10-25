# Data Model: Ergonomic Performance Manager API

**Feature**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md) | **Date**: 2026-10-24

Field lists match Part II. Wire rules (identity split, 16 units, interval rows, fault classes, fixture names) stay in spec.md and are referenced, not copied.

## Catalog document (`vim_rs.perf-catalog/1`)

Committed twice, byte-identical for the vCenter dump (research R11). `CounterRegistry::from_catalog_json` and `vim_build` both accept this shape. Unknown extra fields may be ignored. `schema` must equal `vim_rs.perf-catalog/1`.

| Field | Role |
|-------|------|
| `source` | Provenance. Sanitized before commit. Not used at query time. |
| `intervals[]` | `{ key, sampling_period, name, length, level, enabled }`. `sampling_period` is the wire interval id. `length` is retention in seconds. `key` is not sent as `interval_id`. |
| `counters[]` | One row per counter. Identity is `(group, name, rollup, stats)`. `key` is the server `i32` id. `level` and `per_device_level` are JSON numbers or `null`. `unit` is the wire token (`percent`, `kiloBytes`, `gigaBytes`, …). `other.unit` is set when the token is not a first-class `PerformanceManagerUnitEnum` variant (`gigaBytes`). |
| `providers[]` | Probe evidence. Not required to build a registry. Useful for fixture tests of `ProviderCaps`. |
| `availability.<MoType>.counters.<key>` | `{ intervals: [i32], per_instance: bool }`. Tier-1 scope and the generated `per_instance` flag are computed from these rows. |
| `availability.<MoType>.samples` | Lab dump only. Not required for codegen or `from_catalog_json` resolution. |
| `missing_types` | Kinds with no probe (`StoragePod`, `VirtualApp` on the vCenter dump). They still map through `mo_type_bit` at runtime. |
| `derived` | Probe summary (unit set, other-enum tokens, null-level counts). Ignored by the registry. |

ESXi fixture: 353 counters, `level` and `per_device_level` null, `intervals` empty.

Validation:

- Duplicate `(group, name, rollup, stats)` is a catalog error.
- Duplicate `group.name.rollup` with different `stats` is allowed (the SCSI reservation pair).
- `rollup` and `stats` strings are the lowercase wire forms (`average`, `rate`, `absolute`, `delta`, …).

## Counter registry

| Type | Fields | Rules |
|------|--------|-------|
| `CounterId` | `i32` newtype | Server-local. Never hardcoded in callers. |
| `CounterName` | `group`, `name`, `rollup`, `full` (`group.name.rollup`), each `Arc<str>` | `full` is the lookup string. Split rules are in the spec. |
| `CounterDef` | `id`, `name`, `label`, `unit: UnitKey`, `rollup`, `stats`, `level: Option<i32>`, `per_device_level: Option<i32>` | `level` stays `None` when the server left it unset. |
| `UnitKey` | `PerformanceManagerUnitEnum` | `gigaBytes` is stored as `Other_("gigaBytes")`. |
| `CounterRegistry` | `generation: u64`, `by_full_str` (rate preferred), `by_qualified` (`full`, NUL, lowercase stats), `by_id`, `intervals: Vec<PerfInterval>` | Generation from a process-wide counter (research R8). `by_name` and a bare `resolve` read `by_full_str`. An explicit stats kind reads `by_qualified`, and the stats token is compared case-insensitively. A hit builds that key in a stack buffer. A name that does not fit allocates the same key and reads the same map. |

`from_wire` builds the same maps from `PerfCounterInfo` + `PerfInterval`. Name parsing uses `group_info.key`, `name_info.key`, `rollup_type`, and `stats_type` from the VIM objects (not the display labels).

Relationships: one registry per successful `perf_counter` + `historical_interval` load. `PerfClient` holds `RwLock<Option<Arc<CounterRegistry>>>`. Many `PreparedQuery` values may share that `Arc`.

## Metric request (compile-time)

| Type | Fields | Rules |
|------|--------|-------|
| `Rollup` | `Average`, `Maximum`, `Minimum`, `Latest`, `Summation`, `None` | Non-exhaustive. Generated constants use only these. `CounterDef.rollup` keeps the wire `PerfSummaryTypeEnum`, including `Other_`. |
| `StatsKind` | `Absolute`, `Delta`, `Rate` | Non-exhaustive. Same split: constants use these three; `CounterDef.stats` keeps the wire enum. |
| `Metric<D, S>` | `group`, `name`, `rollup`, `stats: Option<StatsKind>`, `level: Option<u8>`, `per_device_level: Option<u8>`, `per_instance: bool` | `stats: Some` on all 732 generated constants. `stats: None` only on `Metric::custom` (match any stats, prefer rate). `with_scope::<S2>()` keeps `D`. |
| `Dimension` | `Ratio`, `Memory`, `Frequency`, `Time`, `Throughput`, `Power`, `Energy`, `Temperature`, `Count`, `AnyDimension` | `accepts(unit)` iff `scale(0, unit)` is `Some`, except `AnyDimension`, which accepts every unit and uses factor 1 for unknown `Other_` tokens while warning `UnscalableUnit`. |

`D` and `S` are phantom. The value is `Copy`.

## Entity scope

| Mask | Bit | Kinds |
|------|-----|-------|
| `MASK_VM` | `0x01` | `VirtualMachine` |
| `MASK_HOST` | `0x02` | `HostSystem` |
| `MASK_CLUSTER` | `0x04` | `ClusterComputeResource` |
| `MASK_RESOURCE_POOL` | `0x08` | `ResourcePool`, `VirtualApp` |
| `MASK_DATASTORE` | `0x10` | `Datastore`, `StoragePod` |
| `MASK_DATACENTER` | `0x20` | `Datacenter` |
| `MASK_ALL` | `0x3F` | Custom metrics only |

`Entity<K>` wraps a `ManagedObjectReference` plus `PhantomData<fn() -> K>`. `TryFrom<ManagedObjectReference>` fails with `PerfError::EntityKindMismatch` when `moref.r#type` is not `K::MO_TYPE`. `VirtualApp` does not have an `EntityKind`; callers use a dynamic query or a `ResourcePool` entity only when the reference’s type is `ResourcePool`.

Aliases (`VmOnly`, `HostOnly`, `VmOrHost`, `HostCluster`, `VmHostCluster`, `VmOrPool`, `ClusterPool`, `ComputeScope`, `DatastoreOnly`, `DatastoreVm`, `DatastoreVmHost`, `ClusterDc`, `AnyEntity`) are `Scope<MASK>` type aliases. Tier assignment per group is the spec’s tier 3a / 3b tables. Tier 1 and tier 2 are computed from `availability`.

## Provider capabilities

```text
ProviderCaps { current_supported: bool, summary_supported: bool, refresh_rate: Option<i32> }
```

Cache key: `MoTypesEnum`. Transitions:

| Probe result | Next state |
|--------------|------------|
| `Ok(summary)` | Cache `current_supported`, `summary_supported`, `refresh_rate` for that kind |
| `InvalidArgument` | Cache all-false, `refresh_rate: None` |
| Not-authenticated, invalid-login, cancellation, transport | `Err(PerfError::Client)`. Do not cache |
| Any other fault | Do not cache. Try the next entity of this kind. If every entity of this kind fails, mark those entities `Failed` and continue other kinds |

After caps are known, each entity runs the spec interval table (first match). Outcomes are `Queryable(PerfQuerySpec fields)`, `Skip(UnsupportedReason)`, or fetch-level `InvalidInterval`. If every entity is `Skip` from rows 1, 2, or 5, the fetch returns `NoSupportedEntities` and does not call `QueryPerf`.

## Query description

Owned by `PerfQuery` / `PreparedQuery` before resolution.

| Field | Default | Notes |
|-------|---------|-------|
| Entities | empty | Empty fetch → `PerfError::NoEntities` |
| Metric specs | empty | Empty after resolution → `PerfError::NoMetrics`. Each spec is a `Metric` or a name, plus `Instance` |
| `Instance` | `Aggregate` (`""`) unless `instances()` sets a default | `All` is `"*"`. `Named` is the device key. `from_wire("")` is `Aggregate`, `from_wire("*")` is `All` |
| `Interval` | `Auto` | Presets: research R15 |
| Time window | none | `since(SystemTime)`, `since_ago(Duration)` evaluated at fetch, or `between` |
| `latest` | none | `u32`. Historical: not sent as `max_sample`; truncates after decode. Realtime non-incremental: sent as `max_sample` |
| `WireFormat` | `Auto` | Research R10 |
| `chunk_size` | 256 | Clamped to ≥ 1 |
| `time_chunk` | none | Ignored for realtime |
| `concurrency` | 4 | Clamped to ≥ 1 |

`PreparedQueryBuilder::build` stores this description and does no I/O. First `fetch*` resolves names against the current registry and caches them under the registry generation.

Resolved metric: `Arc<CounterDef>` + `Instance` + requested `Dimension` tag (for later `Series<D>`). A name that misses, or a unit the dimension rejects, becomes a warning and is dropped unless zero metrics remain (FR-02).

## Availability cache

`IndexMap<(ManagedObjectReference, i32), Vec<PerfMetricId>>`, capacity 1024, FIFO via `shift_remove_index(0)`. Key interval id is the resolved `interval_id` (refresh rate or historical period). Empty `QueryAvailablePerfMetric` results are not stored. Used by `.all_available()` and `.groups()`.

## Result

| Type | Fields | Rules |
|------|--------|-------|
| `PerfResult` | `entities: IndexMap<ManagedObjectReference, EntitySeries>` in request order, `registry`, `requested_at`, `warnings` | Every requested entity appears exactly once. `rows()` yields only `raw >= 0` samples. |
| `EntitySeries` | `entity`, `status`, `timestamps: Arc<[Timestamp]>`, `interval_secs`, `series: Vec<RawSeries>` | All `RawSeries` in one entity share the same `Arc` timestamps. `series()` resolves the metric against the registry stored on the entity and walks `series`, comparing the counter id and the borrowed instance wire string. A hit does not allocate. One strictly increasing payload stays in columns; several payloads, or a payload that is not strictly increasing, use the union grid. A payload with no value series does not contribute timestamps. See [fastpath.md](./fastpath.md). |
| `RawSeries` | `counter`, `instance`, `timestamps`, `interval_secs`, `values: Vec<i64>` | `values.len() == timestamps.len()` after merge. Missing slots are `-1`. |
| `Series<'a, D>` | borrows one `RawSeries` | `last` skips trailing `< 0`. `last_slot` reads only the final index. `window::<N>` is the newest N slots, oldest-first, left-padded, negatives as `None`. |
| `ScaledSeries<'a, D>` | same grid, values scaled | Memory uses `Bytes(i128)` exact multiply. `window` returns `[Option<D::Scaled>; N]` and skips scaling negative slots (`None`). |
| `Sample` | `ts`, `value: i64`, `interval_secs` | Raw layer. |
| `Row<'a>` | entity, counter, instance, ts, interval, raw, unit | Never `raw < 0`. `scaled()` is `Option<f64>` via `AnyDimension` numeric value. |
| `CompositePerfResult` | `host: Option<EntitySeries>`, `children: IndexMap<…>`, `warnings` | Host missing from the payload is `None`, not an error. Child omitted by the server is absent from the map (composite does not have a caller-supplied child list). `latest(n)` still omits `max_sample` and keeps the newest `n` slots of the host and each child after decode. |

`EntityStatus` transitions for one entity inside a single fetch:

```text
start
  → Unsupported(reason)     interval table skip
  → Failed(error)           probe failed for every object of this kind, or the chunk failed locally
  → NoMetricsAvailable      availability filter produced no ids
  → (sent in QueryPerf)
        → Ok                at least one value series
        → NoSamples         omitted, empty sample info, or sample info with no value series
```

`Ok` with a series of only `-1` values is still `Ok`. Callers blank the current slot via `last_slot`.

`interval_secs` on the entity is the first observed `sampleInfo.interval`, else the requested interval id. A later sample or slice with a different interval records `PerfWarning::IntervalSubstituted { requested_secs, actual_secs }` once per distinct pair.

## Watermark

`SampleWatermark.high_water: HashMap<(ManagedObjectReference, CounterId, Instance), Timestamp>`.

| Operation | Effect |
|-----------|--------|
| `filter_new` | For every `raw >= 0` sample strictly newer than the stored mark (or unseen), set the mark to that timestamp, then yield the row. Updates happen before the iterator is consumed. |
| `entity_high_water` | Minimum timestamp among that entity’s keys, or `None` if it has never reported. A removed per-device instance keeps its mark until `retain_entities`. |
| `min_high_water` | Minimum among the given entities that have reported. Not used to set a shared `start_time`. |
| `retain_entities` | Drop keys whose entity is not in the set. |
| `fetch_incremental` | Each entity’s `start_time` is `entity_high_water` when `Some`. Those entities omit `max_sample`. Entities with no mark use the prepared query’s own window / `latest`. |

Negative samples do not move the mark and are not rows.

## Warnings and errors

`PerfWarning` (non-exhaustive, does not fail the fetch): `CounterNotOnServer`, `DimensionMismatch`, `ScopeMismatch`, `LevelTooLow`, `PerDeviceLevelTooLow`, `AutoIntervalFallback`, `RealtimeWindowTooOld`, `IntervalSubstituted`, `UnscalableUnit`, `UnsupportedSeries` (a decoded series whose counter id is not in the registry), `PartialChunk` (one chunk for an entity failed after another chunk for that entity succeeded; status stays `Ok`).

`PerfError` (fails the fetch, or is stored in `EntityStatus::Failed` when chunk-local): `NoPerformanceManager`, `NoEntities`, `NoMetrics`, `NoSupportedEntities`, `InvalidTimeRange`, `InvalidTimestamp`, `UnknownCounter`, `DimensionMismatch`, `EntityKindMismatch`, `InvalidInterval(i32)`, `Client(client::Error)`.

Fetch-level versus per-entity:

| Condition | Result |
|-----------|--------|
| No perf manager at construction | `Err(NoPerformanceManager)` from `new` |
| Session / transport / cancellation during probe or chunk | `Err(Client)` immediately; remaining chunks aborted |
| All entities skipped by interval rows 1, 2, or 5 | `Err(NoSupportedEntities)` before `QueryPerf` |
| Historical period not enabled, and some entity has summary support, and the interval list is non-empty | `Err(InvalidInterval)` before `QueryPerf` |
| Zero requested counters resolve | `Err(UnknownCounter)` or `Err(DimensionMismatch)` |
| One chunk’s local fault, another chunk ok | `Ok` with `Failed` on that chunk’s entities |
| Every chunk failed locally | `Err` of the first chunk error |

## Chunk and slice identity

A work item is one `PerfQuerySpec` batch: one or more specs whose metric-id sum is at most the chunk size. Each spec keeps its own interval, start, end, and max-sample, including different time slices of one entity. Items share a semaphore of `concurrency` permits. Fatal errors abort the join set. Local errors stamp `Failed` on every entity that was only in failed items and did not already succeed in another item. An entity split across items is `Ok` if any item succeeded, with the union grid; series from a failed item stay absent and are filled with `-1` where the successful items have timestamps. If all of that entity’s items failed, status is `Failed`. A parse error while decoding one entity marks that entity `Failed` and leaves the other entities in the fetch.
