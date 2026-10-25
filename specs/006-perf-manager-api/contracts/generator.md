# Contract: Performance catalog generator

**Feature**: [spec.md](../spec.md) | **Plan**: [plan.md](../plan.md) | **Runtime API**: [README.md](./README.md)

`cargo run -p vim_build` (binary `generate`, working directory `vim_build/`) reads the OpenAPI spec it already reads and `data/perf-catalog.json`. It writes `vim_rs/src/perf/metrics.rs`. Missing or schema-mismatched catalog is a hard error. `cargo build -p vim_rs` does not run this path.

Emission uses `vim_build::printer::Printer`. The committed file must be rustfmt-clean.

## Output shape

```rust
pub mod metrics {
    pub mod cpu {
        use super::super::*;
        /// `cpu.usage.average` (key 2) · percent (hundredths) · level 1, per_device_level 3 · Tier 1: Cluster, Host, VM
        pub const USAGE_AVERAGE: Metric<Ratio, VmHostCluster> =
            Metric::new("cpu", "usage", Rollup::Average, Some(StatsKind::Rate), Some(1), Some(3), true);
    }
    pub mod by_entity {
        pub mod virtual_machine {
            pub mod cpu {
                pub use crate::perf::metrics::cpu::USAGE_AVERAGE;
            }
        }
    }
}
```

`use super::super::*` requires the prelude in [README.md](./README.md). Group modules are `snake_case` of the catalog `group` (`clusterServices` → `cluster_services`, `virtualDisk` → `virtual_disk`, `vcDebugInfo` → `vc_debug_info`, `vsanDomObj` → `vsan_dom_obj`). The vCenter dump yields 26 groups; the generator does not hardcode that count.

`Metric::new` is `#[doc(hidden)] pub const fn`.

## Constant identifiers

1. Replace `.` in `name` with `_`.
2. Insert `_` at lowercase-or-digit to uppercase boundaries, then ASCII-uppercase, then append `_{ROLLUP}`. `swapIn` → `SWAP_IN`, `swapin` → `SWAPIN`, `usagemhz` → `USAGEMHZ`, `maxTotalLatency` → `MAX_TOTAL_LATENCY`.
3. If a dotted name and a non-dotted camelCase name in the same `(group, rollup)` collapse to one identifier, the non-dotted name drops internal camelCase underscores. `mem.zip.saved.latest` → `ZIP_SAVED_LATEST`. `mem.zipSaved.latest` → `ZIPSAVED_LATEST`.
4. If two counters share `(group, name, rollup)` and differ in `stats`, the `rate` counter keeps the base name and the `absolute` counter appends `_ABSOLUTE`.

The generator fails if any identifier still collides, or if a counter’s rollup or stats string is not one of the `Rollup` / `StatsKind` variants (those cannot be emitted as constants). Live `CounterRegistry::from_wire` still keeps `Other_` on `CounterDef`. Expected result on the vCenter catalog: 732 constants, zero collisions.

## Scope

Apply the first matching tier. Masks are the spec bit table.

| Tier | When | Mask |
|------|------|------|
| 1 | `availability.*.counters` contains this key | OR of `mo_type_bit` for those types. 265 counters on the vCenter dump, including all `rescpu` and `vmop`. |
| 2 | Tier 1 is 0 | OR of tier-1 masks of other counters with the same `(group, name)`. 96 counters. `cpu.usage.maximum` inherits `VmHostCluster` from `cpu.usage.average`. |
| 3a | Tier 2 is 0 and the group has a non-zero tier-1 union | That union. The 13 group results are fixed by the spec table (`cpu`/`mem`/`power` → `ComputeScope`, and so on). 197 counters. |
| 3b | Group has no tier-1 row | Spec inferred mask. Rustdoc includes `tier 3b: inferred scope (no active samples in lab dump)`. 174 counters. Overridable with `Metric::with_scope`. |

No generated constant has `MASK_ALL`.

`per_instance` is true iff any availability row for that key sets `per_instance: true`.

`level` / `per_device_level` are `Some(n as u8)` or `None` when JSON null. ESXi is not the codegen input; the committed vCenter catalog has levels.

## `by_entity`

Submodules: `virtual_machine`, `host_system`, `cluster_compute_resource`, `resource_pool`, `datastore`, `datacenter`.

Each re-exports tier 1, tier 2, and tier 3a constants whose mask contains that kind’s bit. Tier 3b groups are omitted until a later catalog has real sample rows. Omit empty group modules. `by_entity::datastore` has no `cpu` and no `mem`.

## Rustdoc

Each constant documents the wire name, server key, unit, levels, tier number, and the kind names in the mask. Tier 3b adds the inferred-scope sentence.

## Tests the generator output must satisfy

- Every constant resolves through `CounterRegistry::from_catalog_json` of `vcenter_catalog.json` to the documented key.
- `MASK != MASK_ALL` for all 732.
- Identifier rules for `SWAP_IN` / `SWAPIN`, `ZIP_SAVED_LATEST` / `ZIPSAVED_LATEST`, and `SCSI_RESERVATION_CNFLCTS_PCT_AVERAGE` / `_ABSOLUTE`.
- `trybuild` failures listed in [research R12](../research.md).
