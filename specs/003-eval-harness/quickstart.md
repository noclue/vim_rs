# Quickstart: PropertyCollector Eval Harness

**Feature**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

How an operator proves each delivery slice. Paths relative to repo root.

## Prerequisites

- Rust toolchain 1.95+ (matches `rust:1.95` eval image)
- For agent arms: Podman (rootless), OpenRouter API key, network to OpenRouter only inside the container policy you apply
- Optional: Goose locally only if debugging recipes outside the container

## Slice 1 — Offline `pc01` (no Goose)

```bash
cd mcp/evals

# After scaffold + fixtures exist:
./runner/grade.sh \
  --problem pc01_vm_power_and_host \
  --solve solutions/pc01_correct.rs

./runner/grade.sh \
  --problem pc01_vm_power_and_host \
  --solve solutions/pc01_wrong_paths.rs   # expect retrieval_fail

# Tamper check: modify Cargo.toml in a work copy and confirm tamper=true
```

**Expect**: Correct solution → `compiled=true`, `retrieval_pass=true` in &lt; 5 minutes; wrong paths → `retrieval_fail`; tamper flagged.

## Slice 2 — Arm A container smoke

```bash
# Build eval image (pins Goose + warms vim_rs source)
podman build -t vim-rs-eval -f mcp/evals/Containerfile .

export OPENROUTER_API_KEY=…   # never commit
./runner/run_one.sh --problem pc01_vm_power_and_host --arm A --repeat 1 --model "$MODEL" --out results/smoke_a
```

**Expect**: `results/smoke_a/trace.jsonl` + `result.json` with Tier-1/2 outcomes.

## Slice 3 — Arm B + trace parsing

```bash
./runner/run_one.sh --problem pc01_vm_power_and_host --arm B --repeat 1 --model "$MODEL" --out results/smoke_b
# Inspect vim_rs_tool_calls in result.json when tools were used
```

**Expect**: Same result schema as Arm A; Arm B may show non-zero `vim_rs_tool_calls` when MCP tools appear in the trace.

## Slice 4 — Pack + scorecard

1. Lock `pc02`–`pc04` only after field-graph path verification (see [research.md](./research.md) R8).
2. Optional calibration: one weaker + one stronger model on 2–3 tasks; record choice.
3. Run matrix and aggregate:

```bash
./runner/run_matrix.sh --problems pc01,pc02,pc03,pc04 --arms A,B --repeats 3 --model "$MODEL"
./runner/scorecard.sh --results results/matrix_… --out results/scorecard.json
```

**Expect**: `verdict` is `clear_lift` or `inconclusive`. Either way the rig is reusable for later GraphSearch Phase-3 tools.

## Layout reminder

| Path | Role |
|------|------|
| `mcp/evals/scaffold/` | Sealed crate; only `src/solve.rs` is solver-owned |
| `mcp/evals/fixtures/` | Oracle `RetrieveResult` payloads (not agent-visible in `/work`) |
| `mcp/evals/problems/` | Goals + contracts |
| `mcp/evals/recipes/` | Arm A / Arm B Goose recipes |
| `mcp/evals/results/` | Gitignored outputs |
