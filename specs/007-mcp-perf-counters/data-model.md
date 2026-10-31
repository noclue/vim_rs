# Data Model: MCP Performance Counters

**Feature**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md) | **Date**: 2026-10-25

Wire-unit, scope-mask, and synonym tables are normative in [spec.md](./spec.md). This file is the stored and in-memory shape those tables attach to. `api_database` does not depend on `vim_build`. `build_counters` maps planner types onto these types.

## `ScopeTier`

How the compile-time scope was derived. Serializes as `Tier1Direct`, `Tier2SiblingRollup`, `Tier3aGroupUnion`, or `Tier3bInferred`.

| Variant | Meaning | Count |
|---|---|---|
| `Tier1Direct` | Observed on sample entities | 265 |
| `Tier2SiblingRollup` | Inherited from sibling rollups of the same `(group, name)` | 96 |
| `Tier3aGroupUnion` | Group table or union of tier-1 masks in the group | 197 |
| `Tier3bInferred` | Inferred for a group with no lab samples. Not in `by_entity` | 174 |

`build_counters` maps `vim_build::perf_metrics::PlanTier` onto this enum one-to-one. The names match.

## `CounterEntry`

One vCenter 8.0.3 counter. Stored in the index and in `counters.json`. Serde field names are the Rust field names (snake_case). No field may be omitted except the two level options.

| Field | Type | Rule |
|---|---|---|
| `id` | `String` | `counter::{group}.{name}.{rollup}`, plus `.absolute` only when another planned counter has the same wire name and this one has `stats == "absolute"` (key 148). Unique. Primary key. |
| `key` | `i32` | Catalog key. Reference only. Keys differ per server. |
| `full_name` | `String` | `{group}.{name}.{rollup}`. No stats suffix. |
| `group` | `String` | Catalog group key (`cpu`, `virtualDisk`, `clusterServices`). |
| `group_label`, `group_summary` | `String` | From catalog labels. Never empty on this catalog. |
| `name` | `String` | May contain dots (`capacity.contention`). |
| `rollup` | `String` | `average`, `maximum`, `minimum`, `latest`, `summation`, or `none`. |
| `stats` | `String` | `rate`, `absolute`, or `delta`. |
| `unit` | `String` | One of the 16 wire tokens in the spec. |
| `unit_label`, `unit_summary` | `String` | From catalog labels. Never empty on this catalog. |
| `label` | `String` | Catalog name label, or the humanized name when that label and the name summary are both empty. Never empty. |
| `summary` | `String` | Catalog name summary, or the synthesized sentence in [research R7](./research.md). Never empty. |
| `level`, `per_device_level` | `Option<u8>` | vCenter 8.0.3 reference levels, 1..=4, configurable per server. `None` prints `n/a`. |
| `per_instance` | `bool` | Per-instance series exist. |
| `esxi_supported` | `bool` | True for 353 counters. |
| `rust_module` | `String` | Snake-case group module (`virtual_disk`). |
| `rust_ident` | `String` | Constant identifier after flat / `_ABSOLUTE` disambiguation (`USAGE_AVERAGE`). |
| `rust_constant` | `String` | `vim_rs::perf::metrics::{rust_module}::{rust_ident}`. |
| `dimension` | `String` | Planner dimension (`Ratio`, `Memory`, `Time`, …). |
| `scaled_type` | `String` | From the spec unit table (`Percent`, `Bytes`, …). |
| `scale_description` | `String` | From the spec unit table. Full text. No shortened form. |
| `scope_mask` | `u8` | Bits 0..=5: VM, Host, Cluster, Pool, Datastore, Datacenter. |
| `scope_alias` | `String` | Planner `scope_type`. Named alias or `Scope<true, false, …>`. |
| `scope_tier` | `ScopeTier` | |
| `valid_entities` | `Vec<String>` | Ordered entity names from the spec scope table. |

### Identity rules

- 732 entries, 732 unique `id`s.
- `full_name` is not unique: key 358 and key 148 share `disk.scsiReservationCnflctsPct.average`. Only key 148’s `id` ends in `.absolute`.
- Case-distinct names stay distinct: `mem.swapIn.average` and `mem.swapin.average` are different entries.
- `rust_ident` may repeat across `rust_module` (35 identifiers). It is unique inside a module.

### Label fallback

Apply only when both catalog name label and name summary are empty (34 counters, groups `datastore`, `mem`, `vmotion`, `vmx`):

1. Split `name` on `.`.
2. Split each piece on a boundary between a lowercase letter or digit and an uppercase letter.
3. Lowercase the pieces, join with spaces, capitalize the first character.

`summary = format!("{group_summary} {label_lower} ({unit_summary}, {rollup})")`.

Locked labels: `Swap in`, `Poll iters per s`, `Swapunreserved`, `Capacity usage vm ovrhd`.

## `CountersOutput`

Debug file `mcp/data/api_definitions/counters.json`, written by `build_counters::build_counters` when `--emit-json` runs. Not committed as a generator input.

| Field | Type | Rule |
|---|---|---|
| `generated_at` | `DateTime<Utc>` | Write time. |
| `source` | `String` | `vim_build/data/perf-catalog.json + vim_build/data/esxi-perf-catalog.json` |
| `counters` | `Vec<CounterEntry>` | All 732, planner order. |

`ApiData::load_from_dir` loads this file when present, the same way it loads `examples.json`. A missing file warns and continues, so older debug trees still load. The production path is `from_parts`, which requires the vector.

## `ApiItemEntry::Counter`

New variant. `id()` returns `&counter.id`. `item_type()` returns `"counter"`.

`ApiData::from_parts` gains a trailing `counters: Vec<CounterEntry>` and inserts each by `c.id`. In-tree callers today are `data_transformer` and `api_database` tests. Both must pass the new argument.

`ApiDatabase` items stay an `IndexMap<String, ApiItemEntry>` keyed by canonical id. Embeddings stay aligned with map order. Adding counters changes every subsequent embedding index, which is why the baseline top-five is recorded by id, not by position in an old bin.

## `CounterAliasIndex`

Not serialized. Built once from `items`.

| Map | Key | Value |
|---|---|---|
| `exact` | case-sensitive alias | one canonical id |
| `lower` | `alias.to_ascii_lowercase()` | every canonical id that alias lowercases to |

Aliases inserted for each counter, in this order:

1. `id` (`counter::mem.swapIn.average`)
2. bare wire name, `id` with the `counter::` prefix removed (`mem.swapIn.average`, and `disk.scsiReservationCnflctsPct.average.absolute` for key 148)
3. `{rust_module}::{rust_ident}`
4. `metrics::{rust_module}::{rust_ident}`
5. `perf::metrics::{rust_module}::{rust_ident}`
6. `rust_constant`

`exact` keeps the first writer (`or_insert`). Canonical ids are unique, and the alias set of one counter does not collide with another counter’s exact aliases except through case. `lower` appends a canonical id only if that id is not already in the list.

### Lookup

`get(id)` after `trim()`:

1. `items.get(trimmed)`
2. `exact.get(trimmed)`, then `items.get`
3. `lower.get(trimmed.to_ascii_lowercase())` only when the list length is 1

Otherwise `None`. Non-counter ids are unchanged: step 1 still returns them, and steps 2–3 do not apply.

`ambiguous_candidates(id)` returns the lowercase list when its length is greater than 1, otherwise an empty slice. Order is insertion order and is not significant. For `MEM.SWAPOUT.NONE` the set is exactly `counter::mem.swapOut.none` and `counter::mem.swapout.none`.

`get("USAGE_AVERAGE")` is `None`. `get("metrics::mem::swap_in_average")`, `get("metrics::mem::swapin_average")`, and `get("metrics::management_agent::swap_in_average")` each hit one counter. `get("COUNTER::CPU.USAGE.AVERAGE")` hits `counter::cpu.usage.average` through the unambiguous lowercase alias.

## `PlannedCounter` (generator, not stored in the index)

Public result of `plan_counters`. Defined in [contracts/planner.md](./contracts/planner.md). `build_counters` is the only translator:

| `CounterEntry` | Source |
|---|---|
| `key`, `group`, `name`, `rollup`, `stats`, `unit`, `level`, `per_device_level`, `per_instance`, `rust_module`, `rust_ident`, `dimension`, `scope_mask`, `esxi_supported` | same field on `PlannedCounter` |
| `scope_alias` | `scope_type` |
| `scope_tier` | `tier` via the one-to-one map |
| `full_name` | `{group}.{name}.{rollup}` |
| `id` | `counter::{full_name}`, plus `.absolute` under the rule above |
| `rust_constant` | `vim_rs::perf::metrics::{rust_module}::{rust_ident}` |
| `scaled_type`, `scale_description` | spec unit table, keyed by wire `unit`. An unknown unit is an error, not a fallback. |
| `valid_entities` | spec scope table, keyed by `scope_mask` |
| `label`, `summary`, `group_label`, `group_summary`, `unit_label`, `unit_summary` | vCenter catalog labels joined by `key` |

A planned counter with no label row is an error. Either catalog file missing is an error.

## Relationships

```text
perf-catalog.json ──┐
                     ├── plan_counters ── PlannedCounter ── to_entry ── CounterEntry
esxi-perf-catalog.json ┘                                              │
                                                                      ├─ items["counter::…"]
                                                                      ├─ CounterAliasIndex (lazy)
                                                                      └─ embedding_text() → vectors in api_database.bin
```

`metrics.rs` is emitted from the same `PlannedCounter` list. MCP does not read `metrics.rs` to build documents. The hermetic `build-counters` test reads `metrics.rs` only to assert that `pub const {rust_ident}: Metric<{dimension}, {scope}>` occurs inside `pub mod {rust_module}`.

## State

Counter documents are immutable after the index is built. There is no per-session state. The alias `OnceLock` moves from empty to filled on first lookup and stays filled for the process. Rebuilding the bin file is a new process.

Search filters do not change the documents. `filter = "counters"` keeps items with `item_type() == "counter"`. `filter = "all"` ranks counters together with every other item.
