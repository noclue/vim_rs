# Quickstart: Ergonomic Performance Manager API

**Feature**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

Offline checks are the merge gate. A live vCenter is optional and not required to land the feature.

## 1. Fixtures

Sanitize the probe dump (`examples/perf-catalog`, output under `examples/perf-catalog/out/`, which is gitignored) into:

| Path | Contents |
|------|----------|
| `vim_build/data/perf-catalog.json` | vCenter 8.0.3, schema `vim_rs.perf-catalog/1`, 732 counters |
| `vim_rs/tests/fixtures/perf/vcenter_catalog.json` | Same bytes as the generator file |
| `vim_rs/tests/fixtures/perf/esxi_catalog.json` | 353 counters, null levels, empty `intervals` |
| `vim_rs/tests/fixtures/perf/query_perf_csv.json` | CSV entity metrics, four diagnostic counters |
| `vim_rs/tests/fixtures/perf/query_perf_normal.json` | Normal-format twin of the CSV fixture |
| `vim_rs/tests/fixtures/perf/query_perf_gaps.json` | Mid-series and trailing `-1`, including `[38, 41, -1, -1]` |
| `vim_rs/tests/fixtures/perf/query_perf_soap.xml` | SOAP envelope matching the CSV/normal samples |
| `vim_rs/tests/fixtures/perf/query_composite.json` | One host plus two child VMs |

Sanitization: `source.server` is `vcenter.example.internal`, `instance_uuid` is `00000000-0000-0000-0000-000000000000`, MoRefs are synthetic (`vm-1001` and similar). No live hostnames.

## 2. Generate constants (phase 3)

From the repository root, after the catalog file exists:

```bash
cargo run -p vim_build
```

That rewrites `vim_rs/src/perf/metrics.rs` and the existing OpenAPI outputs. Review the perf file; do not hand-edit it. A later `cargo build -p vim_rs` must leave it untouched.

## 3. Test

```bash
cargo test -p vim_rs
cargo test -p vim_rs --features perf
cargo test -p vim_rs --features perf,xml
cargo test -p vim_rs --features perf,perf-chrono,perf-polars
cargo test -p vim_build
```

Expected offline coverage:

- 732 vCenter counters and 353 ESXi counters resolve, including dotted names and both SCSI reservation variants.
- Normal JSON, CSV JSON, and (with `xml`) SOAP XML decode to the same entities, timestamps, values, and statuses.
- `[420, 410, -1, -1]`: `last` is 410, `last_slot` is blank. `window::<5>()` of `[100, -1, 300]` is blank, blank, 100, blank, 300.
- `1048576` kiloBytes → `1_073_741_824` bytes. `16` gigaBytes → `17_179_869_184` bytes.
- Mock: 100 VMs × 4 metrics, chunk 256 → 2 `QueryPerf` calls (64, then 36).
- Mock: `NotAuthenticated` aborts the fetch; object-scoped `NoPermission` or `InvalidArgument` fails only that chunk.
- Mock: provider summary `ManagedObjectNotFound` on the first VM retries the second and caches caps.
- Mock: ESXi `[vm, datastore]` succeeds on `Auto` (`datastore` = not a provider) and returns `NoSupportedEntities` on `PAST_DAY`.
- `trybuild`: CPU/memory usage on `Entity<Datastore>` and host-only disk latency on `Entity<VirtualMachine>` fail; `cpu::DEMAND_AVERAGE` on a VM compiles.
- Watermark: a powered-off VM does not move its mark and does not set `start_time` for a live VM; incremental specs omit `max_sample` when start is set.
- Composite fixture: one host series and two children.
- Default `vim_rs` `Cargo.toml` dependency list is unchanged aside from optional `chrono` / `polars` and dev-only `trybuild`.

Agent shells that override Rust paths should prefix cargo with `RUSTUP_HOME=$HOME/.rustup CARGO_HOME=$HOME/.cargo`.

## 4. Snippet

`examples/snippets/src/perf_metrics.rs` should shrink to the one-shot shape: `PerfClient::new`, `.query().entities(&vms).all_available().instances(Instance::All).interval(Interval::Realtime).latest(1).fetch()`, then print `display_value()` for `EntityStatus::Ok`. Delete the hand-built counter map, per-VM provider loop, and trait-object downcasts.

```bash
cargo run -p snippets --bin perf_metrics
```

This binary still needs a live vCenter to print real samples. Compile-check it in CI; do not fail the merge on a missing lab.

## 5. Caller sketches

Prepared poll (external vtui, not edited here):

```rust
let q = PerfClient::new(client)?
    .prepare()
    .metric(cpu::USAGE_AVERAGE)
    .metric(mem::USAGE_AVERAGE)
    .metric(cpu::USAGEMHZ_AVERAGE)
    .metric(mem::CONSUMED_AVERAGE)
    .interval(Interval::Auto)
    .latest(6)
    .build();
let res = q.fetch(&morefs).await?;
```

Sparkline: `series.window::<6>()`. Current slot: `last_slot()` (blank when the newest raw value is negative). Memory bytes: `scaled().last_slot().map(|s| s.value.0)`. A missing entity in `res` is `NoSamples`; overwrite the previous row.

Incremental export: caller-owned `tokio::time::interval`, `fetch_incremental`, then `watermark.filter_new`. Do not enable `tokio/time` on `vim_rs` for this.

Typed query: `perf.query().typed_entity(&Entity::<VirtualMachine>::try_from(moref)?)`. Widening an inferred counter is `metric.with_scope::<AnyEntity>()` or `metric_unchecked`.

## 6. Docs to touch when implementing

- Crate rustdoc on `vim_rs::perf` (module page: identity, `-1`, presets, chunk default, no internal timer).
- `CHANGELOG.md` Added for 0.6.2, including the `perf-polars` semver carve-out and the build-time note (catalog is const data; optional features off by default).
- README feature table: `perf-chrono`, `perf-polars`.

Do not hand-edit `vim_rs/src/types/**` or `vim_rs/src/mo/**`.
