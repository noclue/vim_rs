# Contract: `vim_build` counter planner

**Feature**: [spec.md](../spec.md) | **Plan**: [plan.md](../plan.md)

Public, read-only planning API. `emit_perf_metrics` calls it. `build_counters` calls it. There is no third planner.

## Data

`vim_build/data/esxi-perf-catalog.json` is a byte copy of `vim_rs/tests/fixtures/perf/esxi_catalog.json`. A unit test reads both files and asserts equality, matching the existing vCenter pair.

## Types

```rust
pub enum PlanTier {
    Tier1Direct,
    Tier2SiblingRollup,
    Tier3aGroupUnion,
    Tier3bInferred,
}

pub struct PlannedCounter {
    pub group: String,
    pub name: String,
    pub rollup: String,
    pub stats: String,
    pub unit: String,
    pub level: Option<u8>,
    pub per_device_level: Option<u8>,
    pub per_instance: bool,
    pub rust_module: String,
    pub rust_ident: String,
    pub dimension: &'static str,
    pub scope_mask: u8,
    /// Named alias ("VmHostCluster") or "Scope<true, false, …>".
    /// The emitter adds `crate::perf::entity::` only when writing the generic form.
    pub scope_type: String,
    pub tier: PlanTier,
    pub esxi_supported: bool,
}
```

`PlanTier` derives `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`. `PlannedCounter` derives `Debug`, `Clone`.

`PlanTier`, `PlannedCounter`, `plan_counters`, and `emit_perf_metrics` each carry rustdoc. The docs state that planning is read-only, that `esxi_supported` comes from the ESXi catalog, and that a counter present only in the ESXi catalog is an error. `PlannedCounter::scope_type` documents the named-alias form and the generic `Scope<...>` form.

## Functions

```rust
pub fn plan_counters(
    vcenter_catalog: &str,
    esxi_catalog: Option<&str>,
) -> Result<Vec<PlannedCounter>>;

pub fn emit_perf_metrics(
    catalog: &Path,
    esxi_catalog: &Path,
    output: &Path,
) -> Result<()>;
```

`plan_counters` plans every vCenter counter. With `Some(esxi)`, it sets `esxi_supported` and fails if ESXi contains a `(group, name, rollup, stats)` absent from vCenter. `None` is for callers that have no ESXi file. The generator always passes `Some`.

`emit_perf_metrics` reads both files, calls `plan_counters`, and writes `metrics.rs`. The previous two-argument function is removed. `generator.rs` calls:

```rust
emit_perf_metrics(
    &catalog,
    &vi_json_spec_path.with_file_name("esxi-perf-catalog.json"),
    &metrics,
)
```

The private counter / emitted pair becomes `PlannedCounter`. Identifier disambiguation (flat camelCase versus dotted names, SCSI `_ABSOLUTE`) stays inside this function. MCP must not reimplement it.

## Rustdoc

Module header:

```text
//! Generated from vim_build/data/perf-catalog.json (vCenter 8.0.3) and vim_build/data/esxi-perf-catalog.json (ESXi 8.0.3 availability). Do not edit.
```

Each constant’s documentation line ends with `· ESXi 8.0.3` or `· vCenter only`. Examples:

```text
/// `cpu.usage.average` (key 2) · percent (hundredths) · level 1, per_device_level 3 · Tier 1: Cluster, Host, VM · ESXi 8.0.3
/// `vcResources.buffersz.average` (key 293) · kiloBytes · level 4, per_device_level 4 · Tier 3b (inferred): Host · vCenter only
```

Tier 3b uses `Tier 3b (inferred): {kinds}`. Other tier wording stays. Constant items, types, and `by_entity` re-exports are unchanged.

## Acceptance

- `git diff vim_rs/src/perf/metrics.rs` after regeneration touches only `//!` and `///` lines.
- 353 lines end with `· ESXi 8.0.3`. 379 end with `· vCenter only`.
- Tier counts stay 265, 96, 197, 174.
- `cargo test -p vim_build` and `cargo test -p vim_rs --features perf` pass.
- `plan_counters` on the committed pair returns 732 counters, 353 with `esxi_supported`.
