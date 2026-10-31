# Contract: performance evaluation slice

**Feature**: [spec.md](../spec.md) | **Plan**: [plan.md](../plan.md)

Tasks Phase 10 only (plan Phase 5). `003-eval-harness` is already in this tree at `mcp/evals/`. Extend that tree. Do not vendor a second harness.

This phase is not a release gate. The release gate is `vim_rs` 0.7.0 on crates.io plus the hermetic tests in tasks Phases 1–9.

## Harness as landed

Operator entry points on this machine are the PowerShell runners. `mcp/evals/README.md` (Windows section) and `mcp/evals/Containerfile` describe the image. `grade.sh` and `run_matrix.sh` are not the path to extend: `run_one.sh` still assumes OpenRouter and does not select `EVAL_PROVIDER`.

| Step | Command |
|---|---|
| Offline grade | `mcp/evals/runner/grade.ps1 --problem <yaml-stem> --solve <path>` |
| One agent attempt | `mcp/evals/runner/run_one.ps1 --problem <yaml-stem> --arm A\|B --repeat N --model <id> --out <dir>` |
| Sweep | `mcp/evals/runner/run_matrix.ps1 --problems <stems> --arms A,B --repeats N --model <id>` |

Problem ids are YAML stems (`pf01_vm_balloon_costop_query`), matching `problems/<stem>.yaml`. A short id such as `pf01` does not resolve.

`run_one.ps1` builds the Goose recipe with `runner/lib/render_recipe.py` (goal, contract, model, provider, and `--mcp` on arm B). `recipes/mcp_off.yaml` and `recipes/mcp_on.yaml` are not the live prompt. Editing them does not change what the model sees.

The container receives the sealed work tree at `/work` and that attempt's `agent/` directory at `/out`. Problem files, fixtures, solutions, and grade output stay on the host. `.dockerignore` excludes `mcp/evals/problems`, `mcp/evals/fixtures`, `mcp/evals/solutions`, and `specs`. Do not remove those lines, and do not `COPY` or bind-mount them into the image.

Host `grade.ps1` installs `runner/problem_emit/<stem>.rs` as `src/problem_emit.rs`, then compiles outside the repo (`%LOCALAPPDATA%\vim-rs-eval\host-target`). It does not use `/opt/eval-target`.

The image warms the scaffold at `CARGO_TARGET_DIR=/opt/eval-target`, outside `/work`, so the work mount does not hide it. `CARGO_BUILD_JOBS=1` applies only to that image `RUN`, not to later agent compiles. Rebuild `vim-rs-eval` from the repo root after a scaffold dependency change:

```text
podman build -t vim-rs-eval -f mcp/evals/Containerfile .
```

A cache hit on the `cargo build --bin eval_scaffold` layer leaves the previous target in place.

Default provider is OpenRouter (`OPENROUTER_API_KEY`, `LLM_MODEL` from the repo `.env`). A local OpenAI-compatible endpoint is `EVAL_PROVIDER=openai`, `OPENAI_HOST` with no `/v1` suffix, `OPENAI_BASE_PATH=v1/chat/completions`, and `OPENAI_API_KEY=none`. Set `GOOSE_MAX_TOKENS` high enough that a reasoning model still returns `content`. `run_matrix.ps1` inherits the process environment.

Still true, and still required:

- `tier2.py` exports `VM_IDS` and `MO_TYPE`, runs `eval_scaffold` once per fixture, and compares the last stdout line to `expected`.
- `scaffold/Cargo.toml` pins `vim_rs = "0.5.0"` until T037.
- `parse_trace.py` sets `tool_calls` to `None` when the count is zero.
- `scorecard.py` `mean_metric` skips `None`, and `arm_stats` already reports `mean_tool_calls`.
- `run_matrix.ps1` writes `scorecard.json` with positional args `RESULTS_DIR OUT.json model n_repeats` and exits 0 even when a child attempt fails. The lift gate is a separate `scorecard.py` invocation.

## Harness extensions

Required extensions:

1. `tier2.py` also exports `ENTITY_REFS=<type>:<value>,…` from `objects[].obj`. `VM_IDS` stays.
2. `scaffold/src/main.rs` parses `ENTITY_REFS` into typed morefs when `MO_TYPE == "Mixed"`. The current performance problems use `VirtualMachine`.
3. The mock captures `mock_now = SystemTime::now()` at construction. Fixture samples are `offset_secs` before `mock_now`. Drivers derive windows from `mock_now`. Drivers do not call `PerfClient::set_clock`. The solution builds its own client.
4. Scaffold `Cargo.toml` path-depends on the in-tree crate: `vim_rs = { path = "../../../vim_rs", features = ["perf"] }`. That crate is `0.6.1` today. Do not write `version = "0.7"` on the path dependency, and do not bump `vim_rs` to publish it from this phase. After `0.7.0` is on crates.io, replace the path with `vim_rs = { version = "0.7", features = ["perf"] }`. The mock is updated to the `VimClient` trait the in-tree crate exports. Arm A then has local `vim_rs::perf` and `metrics.rs`.

   The image copies the crates at build time, at the paths the scaffold and `vim_rs` already use: `COPY vim_rs /vim_rs` and `COPY vim_macros /vim_macros` (`vim_rs` path-depends on `../vim_macros`). `.dockerignore` lists `vim_rs` and would hide that `COPY`. Remove that one line. `**/target` still drops build output. Do not bind-mount the repo, and do not copy `vim_build`, `examples`, `specs`, problems, fixtures, or solutions. `mcp/evals/docker/Cargo.toml` (the MCP server stage) does not change its `vim_rs` dependency. Rebuild the image after this change so `/opt/eval-target` is warmed against the new crate.
5. Mock `PerformanceManager`: `service_content.perf_manager` is `Some(PerformanceManager, "PerfMgr")`. The fixture `perf` section serves `perfCounter`, `historicalInterval`, `QueryPerfProviderSummary` (or `InvalidArgument` when configured), `QueryAvailablePerfMetric`, and `QueryPerf` for `csv` and `normal`. Samples visible in the current phase (`perf.phases`, or a flat `perf.samples` when there is one phase). Return samples with `start_time < ts <= end_time`. Honor `max_sample` on realtime. Record every `PerfQuerySpec`. Historical problems list an enabled 300-second interval, or `Interval::PAST_DAY` returns `InvalidInterval`. Provider capabilities are cached per managed-object type, so each kind appears once per fixture. `mock.advance_phase()` moves phases.
6. Each problem has `problems/<stem>.yaml`, `fixtures/<id>_{a,b}.json`, `runner/problem_emit/<stem>.rs` (prints one JSON line last), and `solutions/<id>_correct.rs`.

## Scorecard

Add `scorecard.py --min-lift <x>`. It exits non-zero unless all three hold, with `x = 0.20`:

- `retrieval_pass_rate(B) - retrieval_pass_rate(A) >= x`
- `compile_rate(B) >= compile_rate(A)`
- `mean_tool_calls(B) <= mean_tool_calls(A)`

`retrieval_pass` means both fixtures matched. `tool_calls` counts every tool request, MCP and shell/file. A missing value is `0` before the mean (do not use `mean_metric`’s skip-None behavior for this gate). `vim_rs_tool_calls` is printed and not gated.

`--min-lift` is an optional flag on `scorecard.py`. Keep the positional `RESULTS_DIR OUT.json [model] [n_repeats]` that `run_matrix.ps1` already passes. The matrix script itself does not enforce the gate.

The run, from `mcp/evals`, after the image rebuild:

```powershell
.\runner\grade.ps1 --problem pf01_vm_balloon_costop_query --solve solutions\pf01_correct.rs
.\runner\grade.ps1 --problem pf02_vm_electricity_turn_query --solve solutions\pf02_correct.rs
# pc01_vm_power_and_host .. pc05_vm_nic_labeled_macs use solutions\pc0N_correct.rs

.\runner\run_matrix.ps1 `
  --problems pf01_vm_balloon_costop_query,pf02_vm_electricity_turn_query `
  --arms A,B --repeats 1 --model $env:MODEL

python runner\scorecard.py --min-lift 0.20 <matrixDir> <matrixDir>\scorecard.json $env:MODEL 3
```

`$MODEL` is the operator-chosen id recorded in the scorecard. Both arms use it. Arm A is `render_recipe.py` without `--mcp`. Arm B passes `--mcp` (stdio server `/opt/vim_mcp_server`). Set `EVAL_PROVIDER` in the environment before `run_matrix.ps1` when the endpoint is not OpenRouter.

Pass or miss, write `mcp/evals/reports/pf-slice-<date>.md` with the scorecard. On a miss, add per-problem notes and open a tuning pass (synonym table, Step 5 wording, or counter-document layout) before a re-run. Do not block tasks Phases 1–9.

Reference solutions pass `grade.ps1`. Existing `pc01`..`pc05` still pass.

## Problems

Both problems take `PerfClient` and VM ids and return one `PerfQuery`. The grader fetches the query and compares recorded counter ids, entities, the 300-second interval, and an 86400-second window. Display formatting is not graded.

### pf01 — balloon and co-stop

`async fn solve(client: PerfClient, vm_ids: &[&str]) -> PerfQuery`. `Interval::PAST_DAY`. Counters `mem.vmmemctl.average` (90) and `cpu.costop.summation` (448). The prompt says "balloon driver" and "not scheduled as a group" and does not name the counters.

### pf02 — electricity and waiting for a processor turn

Same signature. Counters `power.power.average` (164) and `cpu.ready.summation` (12). The prompt says "electricity" and "waiting for a turn on the processor" and does not name the counters. `cpu.usage.average` and `cpu.readiness.average` are decoys.
