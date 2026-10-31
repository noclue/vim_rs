# PropertyCollector Eval Harness

Offline-gradable A/B harness measuring whether vim-rs MCP property tools help coding agents fill correct VIM PropertyCollector path strings in pinned `vim_retrievable!` contracts.

**Design**: [mcp/docs/EvalHarness.md](../docs/EvalHarness.md)  
**Spec / contracts**: [specs/003-eval-harness/](../../../specs/003-eval-harness/)  
**Operator quickstart**: [specs/003-eval-harness/quickstart.md](../../specs/003-eval-harness/quickstart.md)

JSON schemas: `specs/003-eval-harness/contracts/` (`problem.schema.json`, `result.schema.json`, `scorecard.schema.json`).

## Layout

| Path | Role |
|------|------|
| `scaffold/` | Sealed Cargo crate; agent edits only `src/solve.rs` |
| `fixtures/` | Oracle `RetrieveResult` payloads (not agent-visible in `/work`) |
| `problems/` | Goals + contracts + expected extracts |
| `recipes/` | Goose Arm A (`mcp_off.yaml`) / Arm B (`mcp_on.yaml`) |
| `solutions/` | Golden correct / wrong-but-compiling references |
| `runner/` | `grade.ps1`, `run_one.ps1`, `run_matrix.ps1` (this machine). `grade.sh` / `run_one.sh` / `run_matrix.sh` are the Unix twins and are not the provider path |
| `results/` | Gitignored traces + scorecards |

## Offline grade (Slice 1 — no Goose)

```bash
cd mcp/evals

# Rust 1.95+ (see scaffold/rust-toolchain.toml). Once per machine (network required):
(cd scaffold && cargo fetch)

./runner/grade.sh \
  --problem pc01_vm_power_and_host \
  --solve solutions/pc01_correct.rs

./runner/grade.sh \
  --problem pc01_vm_power_and_host \
  --solve solutions/pc01_wrong_paths.rs   # expect retrieval_fail
```

**Tamper check** (modify `Cargo.toml` in a work copy, then):

```bash
WORK=$(mktemp -d)
source runner/lib/seal.sh
materialize_work "$WORK"
echo >> "$WORK/Cargo.toml"
source runner/lib/result.sh
if source runner/lib/seal.sh && check_tamper "$WORK"; then echo tamper=true; fi
```

## Seal rules

- Canonical tree: `scaffold/` (container: `/eval/scaffold-canonical` → `/work`).
- Only `src/solve.rs` is solver-owned in agent runs.
- `grade.ps1` (and `grade.sh`) installs per-problem `runner/problem_emit/*.rs` as sealed `src/problem_emit.rs`.
- Hash mismatch on other files → `tamper=true` (use `--check-tamper` after agent runs).

## Windows (Podman)

Agent runs stay in the `vim-rs-eval` container. The container receives the sealed work tree at `/work` and only that attempt's `agent/` directory at `/out` (the recipe in, the trace and `solve.rs` out). Problem files, fixtures, grade output, and `solutions/` stay on the host and are graded after the container exits. Host grading compiles into a Cargo target outside the repo (`%LOCALAPPDATA%\vim-rs-eval\host-target`, or `$XDG_CACHE_HOME/vim-rs-eval/host-target`) and does not pass `CARGO_TARGET_DIR` to the graded binary. The image keeps its own warmed Cargo target at `/opt/eval-target`, so the `/work` mount does not throw it away. Rebuild the image after changing that image target.

```powershell
cd mcp\evals
podman build -t vim-rs-eval -f Containerfile ..\..
.\runner\run_one.ps1 --problem pc01_vm_power_and_host --arm A --repeat 1 --model deepseek/deepseek-v4-flash --out results\smoke_a
```

`run_one.ps1` reads `OPEN_ROUTER_API_KEY` and `LLM_MODEL` from the repo `.env` when those variables are not already set. Offline grading is `.\runner\grade.ps1`.

## Agent arms (Slices 2–3)

Build image from **repo root** (requires `mcp/data/api_database.bin` on the host — run `cargo run -p data-transformer --release` from `mcp/` if missing):

```bash
podman build -t vim-rs-eval -f mcp/evals/Containerfile .
```

The image uses a **minimal MCP workspace** (`mcp/evals/docker/Cargo.toml`) so the build does not pull in `data_processing/*` crates that depend on repo-root `vim_build`.

MCP is built **without** `embed-model` (runtime cache via `VIM_MCP_MODEL_CACHE=/opt/vim_mcp_model_cache`) so Podman does not OOM linking the build script. If `ld` is still killed (signal 9), raise machine memory:

```bash
podman machine stop
podman machine set --memory 8192
podman machine start
```

**Goose pin**: `GOOSE_VERSION` build-arg in `Containerfile` (default `v1.45.0` from [aaif-goose/goose](https://github.com/aaif-goose/goose/releases)). Verify after build: `goose run --help` lists `--output-format stream-json` and `--no-session`.

```bash
export OPENROUTER_API_KEY=…   # never commit
./runner/run_one.sh --problem pc01_vm_power_and_host --arm A --repeat 1 --model "$MODEL" --out results/smoke_a
./runner/run_one.sh --problem pc01_vm_power_and_host --arm B --repeat 1 --model "$MODEL" --out results/smoke_b
```

**Network policy**: OpenRouter egress only inside container; `CARGO_NET_OFFLINE=true` after image warm.

**Env hooks**: `EVAL_COST_CEILING`, `GOOSE_MAX_TURNS` (default 40), `EVAL_IMAGE` (default `vim-rs-eval`).

## Matrix + scorecard (Slice 4)

```bash
./runner/run_matrix.sh --problems pc01_vm_power_and_host,pc02_vm_name_num_cpu,pc03_vm_nic_mac_gap,pc04_host_status_product --arms A,B --repeats 3 --model "$MODEL"
```

Clear-lift rule (simplified): Arm B pass rate ≥ Arm A on every problem and wins on ≥2 problems → `clear_lift`; else `inconclusive`. See `specs/003-eval-harness/research.md` R7.

**Arm C** (`recipes/mcp_off_hinted.yaml`) is deferred until clear A/B lift (FR-015).

## Calibration

Before a paid sweep: run one weaker + one stronger tool-calling model on 2–3 problems; record choice in scorecard `calibration` (or skip reason).

## Example `result.json`

```json
{
  "problem": "pc01_vm_power_and_host",
  "arm": "A",
  "repeat": 1,
  "compiled": true,
  "retrieval_pass": true,
  "failure_class": null,
  "tamper": false,
  "turns": null,
  "tool_calls": null,
  "vim_rs_tool_calls": null,
  "tokens_in": null,
  "tokens_out": null,
  "wall_s": null
}
```

## Problems (Phase 1 pack)

| ID | Notes |
|----|-------|
| `pc01_vm_power_and_host` | VM power + guest hostname |
| `pc02_vm_name_num_cpu` | name + `config.hardware.numCPU` trap |
| `pc03_vm_nic_mac_gap` | device array / MAC gap probe |
| `pc04_host_status_product` | Host overall status + product fullName |

Grading uses mock `RetrievePropertiesEx` (not `WaitForUpdatesEx`). Regenerate fixtures from the host-only package: `cargo run --manifest-path mcp/evals/fixture_gen/Cargo.toml`. That generator is not in the agent scaffold.
