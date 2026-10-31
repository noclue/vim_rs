# Contract: counter documents and callouts

**Feature**: [spec.md](../spec.md) | **Plan**: [plan.md](../plan.md) | **Fields**: [data-model.md](../data-model.md)

Synonym rows, units, and scope masks are the tables in [spec.md](../spec.md). This file is how those tables are rendered. Hermetic tests compare rendered strings. They do not load the embedding model.

## `embedding_text`

For a counter, the text is one paragraph:

```text
{label}: {summary}{synonyms}. Counter: {full_name} ({group_summary}, {unit_summary}, {rollup} {stats}). Entities: {entities}. Systems: {vCenter, ESX|vCenter}. Rust: metrics::{module}::{ident} ({dimension} -> {scaled_type}).
```

- `summary` passes through the existing `prepare_text`.
- `synonyms` is `counter_synonyms(&group, &name)` from the spec table. First match wins. The empty phrase appends nothing, so the summary’s period still closes the first sentence.
- `entities` is `valid_entities` joined with `", "`.
- ESXi is `yes` or `no`.

Hermetic checks: the text starts with a non-empty `{label}: {summary}`, and these counters contain the exact synonym phrase (or, for aborted commands, do not contain the IOPS phrase):

| Counter | Phrase requirement |
|---|---|
| `mem.vmmemctl.average` | memory balloon, ballooning, vmmemctl, balloon driver |
| `cpu.ready.summation` | CPU ready time, vCPU ready percentage, scheduling wait, CPU contention |
| `cpu.latency.average` | CPU/memory contention latency (not storage) |
| `datastore.totalWriteLatency.average` | I/O latency, response time, storage wait |
| `virtualDisk.numberReadAveraged.average` | IOPS, I/O operations per second, read/write rate |
| `disk.commandsAborted.summation` | no IOPS phrase |
| `mem.swapIn.average` | label is `Swap in` |

### Model-backed ranking (`#[ignore]`)

Filter `counters`, limit 5. The named counter is in the top 5, and every hit is a counter.

| Query | Counter |
|---|---|
| VM memory ballooning | `counter::mem.vmmemctl.average` |
| vCPU ready time percentage | `counter::cpu.ready.summation` |
| datastore write latency | `counter::datastore.totalWriteLatency.average` |
| ESXi host power consumption in watts | `counter::power.power.average` |
| virtual disk IOPS | `counter::virtualDisk.numberReadAveraged.average` |

Filter `all`, limit 5, against [research R13](../research.md): each baseline query keeps its previous rank-1 id, and at least 4 of its 5 baseline ids remain.

## `search_summary`

```markdown
## cpu.usage.average — Usage (Counter)

**ID:** `counter::cpu.usage.average`
**Rust Constant:** `vim_rs::perf::metrics::cpu::USAGE_AVERAGE` (`Metric<Ratio, VmHostCluster>`) · requires feature `perf`
**Unit:** `percent` → `Percent` (`raw (hundredths of a percent) * 0.01 -> Percent(f64) in 0.0..=100.0`) · **Ref Levels (vCenter 8.0.3):** aggregate `1`, per-device `3` · **Instances:** aggregate & per-device
**Entities:** `VirtualMachine`, `HostSystem`, `ClusterComputeResource` · **Systems:** `vCenter`, `ESX`

CPU usage as a percentage during the interval

---
```

Rules:

- The unit cell is `{unit}` → `{scaled_type}` and then the full `scale_description` in backticks. No shortened scale text.
- An unset level prints `n/a`.
- Instances are `aggregate & per-device` when `per_instance`, otherwise `aggregate only`.
- The trailing paragraph is `summary`.

## `detailed_document`

`format_counter_doc`. Sections, in order:

1. Header `# Counter: {full_name} ({label})`, the canonical id, the line `*(Requires vim_rs >= 0.7 with features = ["perf"])*`, and `summary`.
2. Dependency and constant:

```toml
vim_rs = { version = "0.7", features = ["perf"] }
```

```rust
use vim_rs::perf::metrics::{rust_module};
// {rust_module}::{rust_ident}: Metric<{dimension}, {scope_alias}>
```

3. Value and unit: wire unit, dimension, scaled type, `scale_description`, and gap handling (`last()` skips trailing gaps, `last_slot()` is only the newest slot, `window::<N>()` maps gaps to `None`).
4. Scope: `valid_entities`, then `**Systems:**`. Every counter lists `vCenter`. A counter in the ESXi catalog also lists `ESX` (`vCenter, ESX`). For tiers 1, 2, and 3a, the per-entity path `vim_rs::perf::metrics::by_entity::{entity_snake}::{rust_module}::{rust_ident}` for each admitted kind. Tier 3b lists no per-entity path and says: if a server reports it on another kind, widen with `.with_scope::<AnyEntity>()` or use `.metric_unchecked()`. Also reference levels (configurable per server) and `per_instance`.
5. Usage snippet, filled from this template. The interval is omitted so `Interval::Auto` applies. `per_instance` adds `.instances(Instance::All)` and reads `e.devices(..)` instead of `e.series(..)`. `cpu.ready.summation` replaces the readout with `s.ready_percent(vcpus).last().copied().flatten()` on the raw series. `!esxi_supported` adds `// Not available on standalone ESXi 8.0.3.`

```rust
// Cargo.toml: vim_rs = { version = "0.7", features = ["perf"] }
use vim_rs::perf::{metrics::{rust_module}, PerfClient};
// `morefs`: {valid_entities joined with " / "} references.
let perf = PerfClient::new(client.clone())?;
let res = perf.query().entities(&morefs).metric({rust_module}::{rust_ident}).latest(1).fetch().await?;
for e in res.entities() {
    if let Some(s) = e.series({rust_module}::{rust_ident}) {
        let current = s.scaled().last_slot().map(|x| x.value.to_string());
        println!("{} {}", e.entity.value, current.as_deref().unwrap_or("-"));
    }
}
```

Every snippet contains `features = ["perf"]` and `use vim_rs::perf::`.

Hermetic renders:

| Counter | Assert |
|---|---|
| `cpu.usage.average` | aggregate and per-device, ESXi yes, full scale description |
| `cpu.ready.summation` | snippet contains `.ready_percent(` |
| `disk.deviceReadLatency.average` | `per_instance`, snippet uses `Instance::All` and `devices(` |
| `vcResources.buffersz.average` | tier 3b guidance, no `by_entity` path, `vCenter only` |
| synthetic `level: None` | summary prints `n/a` |

## PerformanceManager callouts

Prepend this block when formatting the `PerformanceManager` object page, or a method whose name is one of `query_perf`, `query_perf_composite`, `query_available_perf_metric`, `query_perf_provider_summary`, `query_perf_counter`, `query_perf_counter_by_level`, `perf_counter`, `historical_interval`:

```markdown
> [!IMPORTANT]
> **Prefer `vim_rs::perf::PerfClient` (`vim_rs >= 0.7`, `features = ["perf"]`) over raw `PerformanceManager` queries.**
> `vim_rs::perf` handles counter-ID resolution per server, interval selection, 256-id chunking, CSV/normal decoding, unit scaling (`Percent`, `Bytes(i128)`, `Seconds`, …) and `-1` gap sentinels.
> - Read **Step 5** of `get_starter_guide()`.
> - Find typed counters with `search(query="...", filter="counters")` and inspect them with `get(id="counter::cpu.usage.average")`.
> - Examples: `get(id="example::perf_metrics")`, `get(id="example::perf_vm_usage")`, `get(id="example::perf_historical")`, `get(id="example::perf_export")`. Raw-API contrast for builds without `perf`: `get(id="example::perf_metrics_manual")`.
```

Prepend this line for `create_perf_interval`, `update_perf_interval`, `remove_perf_interval`, `update_counter_level_mapping`, and `reset_counter_level_mapping`:

```markdown
> [!NOTE]
> Administrative interval/level configuration is intentionally outside `vim_rs::perf::PerfClient`; call this `PerformanceManager` method directly (or via `perf.raw()`).
```

`description` gets neither block.

Hermetic renders of the object page, `query_perf`, `perf_counter`, `update_perf_interval`, and `description` yield important, important, important, note, and no banner.
