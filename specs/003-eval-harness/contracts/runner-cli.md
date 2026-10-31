# Runner CLI contract

Operator-facing commands for `mcp/evals/runner/`. Exact flag names may use long options; behavior is normative.

## Offline grade (Slice 1 — no Goose)

```bash
./runner/grade.sh --problem pc01_vm_power_and_host --solve path/to/solve.rs [--work-dir DIR]
```

**Behavior**:
1. Materialize sealed work tree from canonical scaffold; install `--solve` as `src/solve.rs`.
2. Reset/hash non-`solve.rs` files → set `tamper` if diverged.
3. `cargo build` (Tier 1) with offline deps as configured for host offline loop.
4. For each fixture: set `FIXTURE`, run binary, compare stdout JSON to `expected`.
5. Write `result.json` (arm may be omitted or set to `"offline"` only if schema extended; default for golden tests: treat as non-agent run with `arm: "A"`, `repeat: 1`, efficiency nulls — or a dedicated `grade-only` mode that still emits the required result fields with null metrics).

**Exit codes**: `0` clean retrieval pass; non-zero on compile/retrieval/tamper (still emit `result.json`).

## Single agent run (Slices 2–3)

```bash
./runner/run_one.sh \
  --problem pc01_vm_power_and_host \
  --arm A|B \
  --repeat N \
  --model MODEL \
  --out DIR
```

**Behavior**:
1. `podman run` ephemeral container with recipe for arm, env secrets, mount `--out`.
2. Capture Goose `stream-json` → `trace.jsonl`.
3. Seal reset + Tier-1 + Tier-2 as in offline grade.
4. Parse trace → efficiency fields; emit `result.json`.

**Env**: `OPENROUTER_API_KEY` required for agent arms; `EVAL_COST_CEILING` / concurrency caps honored by matrix wrapper.

## Matrix + scorecard (Slice 4)

```bash
./runner/run_matrix.sh --problems pc01,pc02,pc03,pc04 --arms A,B --repeats 3 --model MODEL
./runner/scorecard.sh --results results/… --out results/scorecard.json
```

**Verdict**: Apply clear-lift rule from [research.md](../research.md) R7; write `verdict` into scorecard schema.
