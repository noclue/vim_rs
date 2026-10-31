# Quickstart: MCP Performance Counters

**Feature**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md) | **Date**: 2026-10-25

Commands use the repo Cargo when the agent sandbox overrides `CARGO_HOME`. From a normal shell, `cargo` is enough. Run MCP commands from `mcp/`. Run snippet commands from `examples/`. Run generator and library commands from the repo root.

```bash
export RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}"
export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
CARGO="${CARGO_HOME}/bin/cargo"
```

## 1. Record the search baseline first

Before editing catalogs, examples, or the starter guide, capture the current top 5 ids for these queries with `filter = "all"`:

- power on a virtual machine
- retrieve virtual machine mac address
- vSphere task progress tracker
- create snapshot of virtual machine

Write them to `mcp/server/tests/fixtures/baseline_top5.json`. Later ranking tests fail if a query’s previous first id moves, or if fewer than four of the five previous ids remain.

## 2. Planner and ESXi rustdoc

The ESXi catalog file is a byte copy of `vim_rs/tests/fixtures/perf/esxi_catalog.json` at `vim_build/data/esxi-perf-catalog.json`.

```bash
$CARGO test -p vim_build
$CARGO run -p vim_build
git diff -- vim_rs/src/perf/metrics.rs
```

The diff is documentation comments only. 353 lines end with `· ESXi 8.0.3`, 379 with `· vCenter only`. Then:

```bash
$CARGO test -p vim_rs --features perf
```

## 3. Counter documents and aliases

```bash
cd mcp
$CARGO test -p api_database
$CARGO test -p build-counters
```

These stay offline. They lock 732 ids, the key-148 `.absolute` suffix, the four humanized labels, 353 ESXi flags, the swap-in / swap-out alias cases, embedding phrases, summary and document renders, and the PerformanceManager banners.

## 4. Examples and the guide

```bash
cd examples
$CARGO check -p snippets --bins
cd ../mcp
$CARGO test -p build-examples
```

Five `perf_*` binaries, no warnings. Four of them declare `features = ["perf"]`. `perf_metrics_manual` does not. Step 5 of `mcp/server/guides/VIM_RS_STARTER_GUIDE.md` is at most 40 lines, and its ticker, missed-tick, and `match query.fetch_incremental` statements match `examples/snippets/src/perf_export.rs`.

Live runs of the new snippets against vCenter use the same environment variables as the other snippets. They are the release-time sample run, not the merge gate for this feature.

## 5. Rebuild the embedded index

From `mcp/`, after the pipeline and the examples exist:

```bash
$CARGO run -p data-transformer --release
$CARGO build -p vim_mcp_server
```

A clean rebuild prints `✓ api_database.bin is up to date` on the server build. Touching `vim_build/data/perf-catalog.json`, `vim_build/data/esxi-perf-catalog.json`, or `vim_rs/src/perf/metrics.rs` without rerunning the transformer makes the server build panic with `ERROR: api_database.bin is out of date!`. Touching only `examples/vtui` or `examples/perf-catalog` does not.

Startup logs a counter count of 732.

## 6. Model-backed search

Needs the BGE model in `mcp/data/model_cache` or a network fetch on first use. Not part of default `cargo test`.

```bash
cd mcp
$CARGO test -p vim_mcp_server -- --ignored
```

Counters-only queries in [contracts/documents.md](./contracts/documents.md) rank the expected counter in the top 5. The four baseline queries still satisfy the rule in step 1.

## 7. Evaluation

The harness is in `mcp/evals/`. Grade and sweep with the PowerShell runners. Problem ids are YAML stems. Details and the image-copy rules are in [contracts/evals.md](./contracts/evals.md).

From `mcp/evals`, after the scaffold path-depends on in-tree `vim_rs` with `features = ["perf"]` and `vim-rs-eval` has been rebuilt:

```powershell
.\runner\grade.ps1 --problem pf01_vm_balloon_costop_query --solve solutions\pf01_correct.rs
.\runner\grade.ps1 --problem pf02_vm_electricity_turn_query --solve solutions\pf02_correct.rs
# pc01_vm_power_and_host .. pc05_vm_nic_labeled_macs: solutions\pc0N_correct.rs

.\runner\run_matrix.ps1 `
  --problems pf01_vm_balloon_costop_query,pf02_vm_electricity_turn_query `
  --arms A,B --repeats 1 --model $env:MODEL

python runner\scorecard.py --min-lift 0.20 <matrixDir> <matrixDir>\scorecard.json $env:MODEL 3
```

Write `mcp/evals/reports/pf-slice-<date>.md` whether the thresholds pass or not. A miss does not block publishing the index once `vim_rs` 0.7.0 is on crates.io.

## 8. What not to do

- Do not hand-edit `vim_rs/src/perf/metrics.rs`. Regenerate it.
- Do not add an MCP tool.
- Do not point the documentation server at a live vCenter.
- Do not publish the new `api_database.bin` before `vim_rs` 0.7.0 is on crates.io.
- Do not treat a missed 0.20 lift as a reason to hold that publish.
