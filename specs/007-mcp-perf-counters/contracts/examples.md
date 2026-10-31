# Contract: performance examples and Step 5

**Feature**: [spec.md](../spec.md) | **Plan**: [plan.md](../plan.md)

Five binaries under `examples/snippets/src/`, registered in `examples/snippets/Cargo.toml`. Each file’s first `//!` line is the title below. `tokio` in that package is `{ version = "1.44", features = ["macros", "rt-multi-thread", "time"] }`.

`cargo check -p snippets --bins` from `examples/` succeeds. The five `perf_*` binaries warn nothing. `perf_vm_usage` and `perf_export` stop after `PERF_TICKS` ticks (default 3) and call `client.close().await`.

## Programs

| Id | File | Code | Title |
|---|---|---|---|
| `example::perf_metrics` | `perf_metrics.rs` | unchanged except one doc sentence | existing one-shot title |
| `example::perf_metrics_manual` | `perf_metrics_manual.rs` | unchanged | Legacy: hand-rolled PerformanceManager sample (contrast with perf_metrics). |
| `example::perf_vm_usage` | `perf_vm_usage.rs` | reworked | `Live realtime CPU and memory monitor for every virtual machine, with sparklines.` |
| `example::perf_historical` | `perf_historical.rs` | new | `Past-day 5-minute CPU and memory rollups for every virtual machine, exported as tidy rows.` |
| `example::perf_export` | `perf_export.rs` | new | `Incremental realtime export of VM metrics with per-entity watermarks (no duplicates, no gaps).` |

The legacy file’s first documentation line is:

```rust
//! Legacy: hand-rolled `PerformanceManager` sample (contrast with `perf_metrics`).
```

`perf_metrics`: the sentence that says `perf_vm_usage` asks only for CPU percent and consumed memory and prints a table is rewritten to describe the live monitor.

`perf_metrics_manual` documentation, after the title line, states that the program is what `perf_metrics` replaced, that it shows the work `vim_rs::perf` does (counter-id maps, interval choice, decoding, unit scaling), and that it is the direct `PerformanceManager` path for callers who cannot or prefer not to enable `perf`. New code uses `PerfClient`. The existing PyVmomi-style description stays below that.

### Live monitor (`perf_vm_usage`)

- Inventory: container view of virtual machines, each converted with `Entity::<VirtualMachine>::try_from`. vCPU count once per VM from `config.hardware.num_cpu` via `vim_retrievable!` and `ObjectRetriever`.
- Counters, imported through `vim_rs::perf::metrics::by_entity::virtual_machine`:

| Constant | Readout |
|---|---|
| `cpu::USAGE_AVERAGE` | `scaled().window::<6>()` sparkline, oldest to newest |
| `mem::USAGE_AVERAGE` | same |
| `cpu::USAGEMHZ_AVERAGE` | `scaled().last_slot()`, shown as `{:.0} MHz` via `.as_mhz()` |
| `mem::CONSUMED_AVERAGE` | `scaled().last_slot()`, `Display` of `Bytes` |
| `cpu::READY_SUMMATION` | raw `series.ready_percent(vcpus)`, newest element; blank on a gap |

- Sparkline glyphs are `▁▂▃▄▅▆▇`. Input is `[Option<Percent>; 6]`. Index is `(p.0.clamp(0.0, 100.0) * 6.0 / 100.0).round() as usize`. `None` renders as `' '`. Do not copy vtui’s raw-hundredths helper.
- Other values use `Display` (`42.50 %`, `1.50 GiB`).
- Query: `perf.prepare()…interval(Interval::Realtime).latest(6).build()`, once.
- Interval: `perf.realtime_refresh_rate` on the first VM, or 20 seconds on error. `MissedTickBehavior::Skip`.
- Each tick: `match query.fetch(&morefs).await`. `Err` logs and continues. `EntityStatus::Ok` prints values. `NoSamples` prints a blank row. `Unsupported` and `Failed` print the status.

### Historical (`perf_historical`)

- `perf.query().entities(&vms)` with `metrics::cpu::USAGE_AVERAGE` and `metrics::mem::CONSUMED_AVERAGE`.
- `.interval(Interval::PAST_DAY).between(now - 24h, now).time_chunk(Duration::from_secs(6 * 3600))`.
- Print `res.rows()` as CSV `entity,counter,instance,rfc3339_ts,raw,scaled` using `row.ts.to_rfc3339_utc()` and `row.scaled()`. Print `res.warnings`.
- Comments: historical intervals ignore `max_sample`; `time_chunk` bounds each RPC; a multi-day run uses `Interval::PAST_WEEK` and `.since_ago`.

### Export (`perf_export`)

- `perf.prepare()` with `cpu::USAGE_AVERAGE` and `mem::CONSUMED_AVERAGE` from `by_entity::virtual_machine`, `.interval(Interval::Realtime).latest(4).build()`. Start from `SampleWatermark::new()`.
- `for tick in 0..ticks`. The body contains these three statements character for character, because Step 5 quotes them:

```rust
let mut ticker = tokio::time::interval(perf.realtime_refresh_rate(&vms[0]).await?);
ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
match query.fetch_incremental(&vms, &watermark).await {
```

  The match applies `watermark.filter_new(&res)` and prints `entity,counter,instance,ts,raw`. The `Err` arm logs and continues. If `?` on `PerfError` does not convert to `anyhow::Error`, both this file and Step 5 use `.map_err(|e| anyhow::anyhow!(e.to_string()))` on the refresh-rate call.

- Every `INVENTORY_EVERY` ticks (`2`), re-list VMs, build `active: HashSet<ManagedObjectReference>`, call `watermark.retain_entities(&active)`. Comments: marks are per entity, so a powered-off VM does not hold the others; `fetch_incremental` restarts an entity from the earliest mark across that entity’s series (`entity_high_water`), and `filter_new` drops the duplicates that causes. A real exporter refreshes inventory every few minutes. The constant `2` is so a short demo run shows the call.

## Example indexer

`collect_examples` in `mcp/data_processing/build_examples/src/lib.rs`:

- Skip when the normalized path contains `/perf-catalog/`, beside the existing `/snippets/src/lib.rs`, `/vtui/`, and `/target/` skips.
- Category order: `macro_usage`, then `perf`/`metric` → `performance`, then `property_collector`, then `events`, then `connection`, then `general`. The performance test is a name test (`name.contains("perf") || name.contains("metric")`) and must run before the source test for `PropertyCollector`.
- Dependencies: `vim_rs = { version = "0.7", features = ["perf"] }` when any trimmed line starts with `use vim_rs::perf`, otherwise `vim_rs = "0.7"`. No `vim_macros`. Tokio features as above. Keep `anyhow`, `log`, and `env_logger`. Keep the comment that `vim_retrievable!` / `vim_updatable!` are re-exported.

Acceptance for `collect_examples(Path::new("examples"))`:

- Zero indexed files under `examples/perf-catalog/` or `examples/vtui/`.
- The 16 non-perf snippets keep their current categories.
- All five `perf_*` examples have category `performance`.
- `perf_metrics`, `perf_vm_usage`, `perf_historical`, and `perf_export` dependencies contain `features = ["perf"]`.
- `perf_metrics_manual` and the 16 non-perf snippets do not.

## Starter guide

File: `mcp/server/guides/VIM_RS_STARTER_GUIDE.md`.

1. Replace the line-3 usage note with:

> **How to use this file:** Skim **Steps 1–4** first (connection → calls → retrieval → tasks). Treat **Step 3.5+** (polymorphism, Deref), **Step 5** (performance metrics; `vim_rs >= 0.7`, feature `perf`), **Navigation paths**, **MCP tools**, and **Anti-patterns** as lookup sections—do not load them all unless the task needs them.

2. Insert Step 5 immediately after Step 4. Budget: at most 40 lines (the source draft is 30). It states the `perf` feature, forbids hand-rolled `query_perf` / counter maps / historical `max_sample`, points at `search(filter="counters")` and `get`, and has this workflow table:

| Workflow | Example id |
|---|---|
| One-shot snapshot | `example::perf_metrics` |
| Live monitor / sparklines | `example::perf_vm_usage` |
| Historical rollups | `example::perf_historical` |
| Incremental export | `example::perf_export` |
| Raw API, no `perf` feature | `example::perf_metrics_manual` |

Value rules in the step: raw versus scaled, gaps (`last` / `last_slot` / `window` / `rows` / `filter_new`), scopes (`Metric<D, S>`, `by_entity`, `typed_entities`, tier 3b widen). The poll-loop code block uses `loop {` and contains the three `perf_export.rs` statements above character for character. The tick bound and the inventory refresh are not in the guide.

3. Anti-pattern rows:

| Wrong | Right |
|---|---|
| `PerformanceManager::query_perf` plus a hand-built counter-id map | `vim_rs::perf::PerfClient` (Step 5) |
| `vim_rs = "0.7"` and `use vim_rs::perf` | `vim_rs = { version = "0.7", features = ["perf"] }` |
| `max_sample` / `latest(n)` to bound a historical query | `.between` / `.since_ago` plus `.time_chunk` |
| `last()` as a dashboard’s current value | `last_slot()` |
| `?` on the fetch inside a poll loop | `match`, log, and `MissedTickBehavior::Skip` |

4. MCP tools table: the `search` row’s filter list includes `counters`.

5. The four dependency snippets (Step 1 plain, Step 1.1 `xml`, Step 1.2 `xml` + `vcsim_compat`, Step 3.6 `defaults`) move from `0.5` to `version = "0.7"` and keep those features. They do not gain `perf`.
