# Contracts: MCP Performance Counters

**Feature**: [spec.md](../spec.md) | **Plan**: [plan.md](../plan.md)

Assistant-facing and pipeline contracts. Field shapes are in [data-model.md](../data-model.md). Generator surface is [planner.md](./planner.md). Document text is [documents.md](./documents.md). Examples and the starter guide are [examples.md](./examples.md). The evaluation slice is [evals.md](./evals.md).

## Tools

The server still exposes six tools. This feature changes `search` and `get` only.

### `search`

`SearchFilter` gains `Counters`.

| Variant | JSON (`snake_case`) | `as_str()` / `item_type()` |
|---|---|---|
| `Counters` | `counters` | `counter` |

Existing variants are unchanged. Default remains `All`.

Tool description includes performance counters, in the same sentence as managed objects, methods, structures, enums, and examples. Suggested wording: the search returns Rust managed objects, methods, structures, enums, examples, and `vim_rs::perf` performance counters.

When `filter` is `Counters`, every returned item has `item_type() == "counter"`. The schema-driven `--web` UI picks up the variant from `JsonSchema` without a UI edit.

### `get`

`GetInput::id` description adds `'counter::cpu.usage.average' or 'metrics::cpu::USAGE_AVERAGE' (performance counter)` and replaces `example::connection_basic` with `example::perf_metrics`.

Resolution is `ApiDatabase::get` ([data-model.md](../data-model.md)). On `None`:

- If `ambiguous_candidates(id)` is non-empty, the reply is `Multiple counters match '<id>' (case-sensitive):` and one line per candidate with `id`, `rust_constant`, `label`, and `unit`. No other suggestions.
- Otherwise the existing not-found scan walks every `items` key (remove `.take(1000)`), still returns at most five suggestions, lists `- Counter: \`counter::cpu.usage.average\` (or \`metrics::cpu::USAGE_AVERAGE\`)`, and uses `example::perf_metrics`.

### Server instructions and logs

`get_info()` instructions add: performance metrics use `vim_rs::perf` (`vim_rs >= 0.7`, `features = ["perf"]`); find counters with `search(filter="counters")`.

Startup logging in `mcp/server/src/main.rs` and the count log in `build_embeddings/src/lib.rs` include `count_by_type("counter")`. After a rebuild that count is 732.

## Pipeline

`data-transformer` steps, in order:

1. Build API definitions.
2. Collect examples.
3. Collect counters (`build_counters::collect_counters`). Log the total and the ESXi count (732 and 353).
4. `ApiData::from_parts(..., examples, counters)`.
5. Generate embeddings.

`ApiDatabase { items, embeddings: Some(...), counter_aliases: Default::default() }`.

`--emit-json` also calls `build_counters::build_counters`, writing `counters.json` beside the other debug files.

`build-counters` `Cargo.toml` depends on `api_database` (path `../../api_database`), `vim_build` (path `../../../vim_build`), `serde_json`, `anyhow`, `chrono`, and `tracing`. `mcp/Cargo.toml` members include `data_processing/build_counters`. `data-transformer` depends on `build-counters`.

`collect_counters` signature: `fn collect_counters(vim_build_data_dir: &Path) -> Result<Vec<CounterEntry>>`.

`build_counters` signature: `fn build_counters(vim_build_data_dir: &Path, output_dir: &Path) -> Result<()>`.

## Staleness

`mcp/server/build.rs` treats these as rebuild inputs, in addition to the OpenAPI spec and example sources:

- `vim_build/data/perf-catalog.json`
- `vim_build/data/esxi-perf-catalog.json`
- `vim_rs/src/perf/metrics.rs`

Each existing path gets `cargo:rerun-if-changed` and is pushed to `stale_sources` when `is_newer_than` the bin file. A clean tree prints `✓ api_database.bin is up to date`. Any stale path panics with `ERROR: api_database.bin is out of date!`.

`find_rust_sources_recursive` skips directory names `perf-catalog` and `vtui` alongside `target`, `.git`, and dot-directories. Changing only `examples/perf-catalog/src/*.rs` or `examples/vtui/src/*.rs` does not panic.

## Index rebuild acceptance

`cargo run -p data-transformer --release` from `mcp/` produces a bin with exactly 732 items of type `counter` and the five performance examples. Default `cargo test` for `api_database`, `build-counters`, `build-examples`, and `vim_mcp_server` passes offline. `cargo test -p vim_mcp_server -- --ignored` passes when the BGE model is cached under `mcp/data/model_cache` or can be downloaded. Ignored tests are not part of the default offline run.

## Release

Do not publish the server or the bin until `vim_rs` 0.7.0 is on crates.io. Publishing steps stay in `docs/mcp-publishing.md`.
