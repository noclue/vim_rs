# Follow-on: series lookup and single-payload decode

**Status**: Implemented. Normative for the read path and the decoder. It does not amend FR-01 through FR-26.

**Parent**: [spec.md](./spec.md) FR-09, FR-17, FR-21. [research.md](./research.md) R8, R9, R16. [data-model.md](./data-model.md) result table.

A monitoring poll calls `series()` once per entity per metric, then decodes one `QueryPerf` payload per entity on the common path (a handful of counters, under the chunk size, no time slice). Both steps do more allocation than that shape needs. This note fixes that cost. Public types, method signatures, and decoded values stay as in the spec, the data model, and the contract.

## Series lookup

`EntitySeries::series`, `series_instance`, and `devices` call `CounterRegistry::resolve` before the private `(CounterId, Instance)` index. `resolve` formats `group.name.rollup` into a `String`. `by_name` formats it again. `by_name_with_stats` builds a `CounterKey` (a `CounterName` of four `Arc<str>` values, plus another `Arc` for the stats kind). `series_instance` then clones `Instance` to probe the index. `PerfResult::series` repeats that once per entity. A poll of N entities and M metrics pays N×M resolves.

`series()` always looks up the aggregate instance. That variant is a unit, so cloning it does not heap-allocate. The heap cost on that path is the resolve. `series_instance` of a named device clones a `String` as well.

The index built in `EntitySeries::new` is already an O(1) slot map. This follow-on leaves the map in place and removes the allocation in front of it.

### Which registry

`series()` resolves against the `Arc<CounterRegistry>` stored on that `EntitySeries` at decode time. `refresh_registry` installs a new registry for the next fetch. A `PerfResult` already returned keeps the registry it decoded with, and therefore the same counter ids.

The lookup structure lives on that registry value. It is not a process-wide map from metric name to counter id: two servers assign different ids, and two registries can be alive together (a prepared query holds one generation while the client refreshes another; see R8 and FR-21).

FR-21 caches resolved wire metrics on the prepared query, keyed by registry generation. That cache stays as specified. This note is the read path after decode.

### Requirements

- **FP-1**: A successful `series()`, `series_instance()`, or `devices()` lookup does not allocate on the hit path. That covers the formatted counter name, a `CounterKey`, and any `Arc` built only to query the map. `devices()` resolves once per call, then walks series; that resolve follows this rule. The unknown-counter path may allocate the name it reports.
- **FP-2**: Lookup uses the metric’s existing `group`, `name`, `rollup`, and `stats` (`&'static` on generated and custom metrics). Rollup comparison stays case-insensitive (FR-01). A hit does not allocate a lowered rollup string.
- **FP-3**: Bare lookup (`stats` is `None`) and stats-kind lookup keep FR-01. Rate wins when both a rate and another kind exist. An explicit stats kind selects that kind. A bare name that matches more than one kind and has no rate still fails.
- **FP-4**: `series()` does not heap-allocate an instance key. `series_instance` takes the caller’s instance by reference and compares that wire string with the `Instance` already stored on each `RawSeries`. It does not clone a `Named` string and does not build a second copy of the instance at decode time. `EntitySeries` stays a normal struct.
- **FP-5**: A metric missing from the registry still yields `None` from `series()` and `series_instance()`, and an empty iterator from `devices()`. A metric the registry has, and the entity did not return, does the same. Dimension and entity-scope type parameters stay compile-time only and are not part of the lookup key.

### Acceptance

- A decoded entity with an aggregate `cpu.usage.average` series and a named device series returns the same slots and values on a second `series()` and `series_instance()` call.
- A name absent from that entity’s registry makes `series()` return `None`.
- Two registries with different ids for the same name resolve independently. Each entity uses the registry it was decoded with.

Hit-path allocation is a review check against FP-1 and FP-4. These tests lock the observable result. Do not add a global allocator harness for this follow-on.

## Single-payload decode

`execute` gathers every payload for one entity across chunks, then calls `decode_metrics` once with that list. `parse_normal` and `parse_csv` build a `Point { ts, interval, value }` per sample per series. `build_entity` inserts every point into a `BTreeMap<Timestamp, i32>`, builds a `HashMap` from timestamp to column, and copies values into a new vector. `decode_composite` uses the same build for the host and each child.

Series inside one payload already share that payload’s `sampleInfo`. The grid exists to merge several payloads onto one sorted timeline (ADR-05, FR-17, R9, R16): metric chunks and historical time slices.

### Requirements

- **FP-6**: When an entity has exactly one payload, decode keeps one timestamp column and one value column per series. It does not allocate a `Point` per sample, a `BTreeMap` of timestamps, or a `HashMap` from timestamp to index.
- **FP-7**: The single-payload path covers normal and CSV `QueryPerf` entities, and each host or child of `QueryPerfComposite`. Normal and CSV still decode to the same result (FR-09).
- **FP-8**: The single-payload path requires strictly increasing timestamps. Column order is then the sorted unique grid. The entity interval is the first sample’s interval, which is the earliest timestamp’s interval. Every sample interval is still compared with the requested interval. A difference still records `IntervalSubstituted`. Duplicate warning pairs are still removed once per decode.
- **FP-9**: A single payload whose timestamps are not strictly increasing, including duplicates, uses the grid. The grid sorts timestamps, keeps the first interval recorded for a duplicate timestamp, and merges values with FP-11. Preserving wire order on that payload would disagree with FR-17.
- **FP-10**: Two or more payloads for one entity use the grid. Timestamps are the sorted unique union. Missing slots are `-1`. The entity interval is the earliest timestamp’s interval, or the requested interval when the grid is empty. This is the metric-chunk and time-slice case, including two payloads whose timestamps happen to be identical.
- **FP-11**: A value column shorter than the timestamp column is padded with `-1`. Values past the timestamp column are ignored. An empty CSV token and the token `-1` are both `-1` (FR-09). Two series in one payload with the same counter id and instance merge into one column: a new value is written when it is `>= 0` or the slot is still `< 0`.
- **FP-12**: An unparseable timestamp, an odd `sampleInfoCSV` field count, or a series that is not `PerfMetricIntSeries` still returns `Err` from the decoder. The fast path does not turn that into an empty series. `execute` still fails only that entity, unless the error is fatal (authentication, cancellation, or transport).
- **FP-13**: An empty sample list is still `NoSamples`. A payload that has timestamps and no value series is still `NoSamples`: those timestamps do not join another payload’s grid and do not record interval warnings. An omitted entity is still `NoSamples`. Every `RawSeries` on an entity still shares that entity’s `Arc<[Timestamp]>`.

### Acceptance

- Existing `perf_decode` tests (normal, CSV, gaps, SOAP, composite, union grid) pass with the same entities, statuses, timestamps, values, and warnings.
- One strictly increasing payload of two counters decodes to those timestamps and values. A short value column is padded with `-1`. One sample interval that differs from the request records a single interval-substituted warning.
- Two payloads for one entity, timestamps `[t1, t2]` and `[t2, t3]`, merge to `[t1, t2, t3]` with `-1` in the holes (the FR-17 acceptance case).
- Inside a fetch that also contains a valid entity, a payload with an unparseable timestamp fails only the bad entity.

## Where to change it

| Concern | Code |
|---|---|
| Hit path | `vim_rs/src/perf/result.rs` — `EntitySeries::series`, `series_instance`, `devices` |
| Name resolution | `vim_rs/src/perf/registry.rs` — `resolve`, `by_name`, `by_name_with_stats` |
| Gather, then one decode per entity | `vim_rs/src/perf/engine.rs` — `execute`, `run_chunks` |
| Points and grid | `vim_rs/src/perf/decode.rs` — `parse_normal`, `parse_csv`, `align_points`, `build_entity`, `decode_metrics`, `decode_composite` |

## Out of scope

- A caller-side cache from a metric constant to a `CounterId` across polls. FP-1 is the requirement. A generation-keyed cache on the caller is a later option only if a profile shows the allocation-free lookup is still too expensive.
- Skipping the grid because two payloads happen to share one timestamp column. Two payloads always merge (FP-10).
- Changing CSV versus normal selection (FR-08).
- A new benchmark crate. `cargo bench -p vim_rs` already exists; a bench may follow after the paths land.
