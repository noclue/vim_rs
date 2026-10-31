#!/usr/bin/env bash
set -euo pipefail

EVAL_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUNNER_LIB="${EVAL_ROOT}/runner/lib"

source "${RUNNER_LIB}/result.sh"
source "${RUNNER_LIB}/seal.sh"

PROBLEM=""
ARM="A"
REPEAT=1
MODEL=""
OUT=""
IMAGE="${EVAL_IMAGE:-vim-rs-eval}"
COST_CEILING="${EVAL_COST_CEILING:-}"

usage() {
  echo "Usage: $0 --problem ID --arm A|B --repeat N --model MODEL --out DIR"
  exit 2
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --problem) PROBLEM="$2"; shift 2 ;;
    --arm) ARM="$2"; shift 2 ;;
    --repeat) REPEAT="$2"; shift 2 ;;
    --model) MODEL="$2"; shift 2 ;;
    --out) OUT="$2"; shift 2 ;;
    --image) IMAGE="$2"; shift 2 ;;
    -h|--help) usage ;;
    *) echo "unknown: $1" >&2; usage ;;
  esac
done

[[ -n "$PROBLEM" && -n "$MODEL" && -n "$OUT" ]] || usage
[[ "$ARM" == "A" || "$ARM" == "B" ]] || usage

mkdir -p "$OUT"
OUT="$(cd "$OUT" && pwd)"

PROBLEM_FILE="${EVAL_ROOT}/problems/${PROBLEM}.yaml"
PROBLEM_JSON="$(python3 "${RUNNER_LIB}/load_problem.py" "$PROBLEM_FILE")"
python3 -c "import json,sys; d=json.load(sys.stdin); open(sys.argv[1],'w').write(d['goal']); open(sys.argv[2],'w').write(d['contract'])" \
  "${OUT}/goal.txt" "${OUT}/contract.txt" <<<"$PROBLEM_JSON"

# Container sees only the recipe plus the trace and solve.rs it writes.
AGENT_DIR="${OUT}/agent"
rm -rf "$AGENT_DIR"
mkdir -p "$AGENT_DIR"

RENDER_ARGS=(
  --out "${AGENT_DIR}/recipe.yaml"
  --goal-file "${OUT}/goal.txt"
  --contract-file "${OUT}/contract.txt"
  --model "$MODEL"
)
if [[ "$ARM" == "B" ]]; then
  RENDER_ARGS+=(--mcp)
fi
python3 "${RUNNER_LIB}/render_recipe.py" "${RENDER_ARGS[@]}"

# Persistent work tree so agent edits survive the container
WORK="${OUT}/work"
materialize_work "$WORK"
install_challenge_start "$WORK" "$PROBLEM"

TRACE="${AGENT_DIR}/trace.jsonl"
WALL_START=$SECONDS

if [[ -z "${OPENROUTER_API_KEY:-}" ]]; then
  echo "OPENROUTER_API_KEY required for agent arms" >&2
  emit_result_json "${OUT}/result.json" "$PROBLEM" "$ARM" "$REPEAT" \
    false false no_output false null null null null null null
  exit 6
fi

set +e
podman run --rm \
  --network=pasta \
  -e OPENROUTER_API_KEY \
  -e GOOSE_PROVIDER=openrouter \
  -e GOOSE_MODEL="$MODEL" \
  -e GOOSE_MODE=auto \
  -e GOOSE_DISABLE_SESSION_NAMING=true \
  -e GOOSE_MAX_TURNS="${GOOSE_MAX_TURNS:-40}" \
  -e CARGO_NET_OFFLINE=true \
  -e VIM_MCP_MODEL_CACHE=/opt/vim_mcp_model_cache \
  ${COST_CEILING:+-e EVAL_COST_CEILING="$COST_CEILING"} \
  -v "${WORK}:/work" \
  -v "${AGENT_DIR}:/out" \
  -w /work \
  "$IMAGE" \
  sh -c 'goose run --recipe /out/recipe.yaml --no-session --output-format stream-json > /out/trace.jsonl; ec=$?; cp -f /work/src/solve.rs /out/solve.rs 2>/dev/null || true; exit $ec'
GOOSE_RC=$?
set -e

WALL_S=$((SECONDS - WALL_START))

if [[ $GOOSE_RC -ne 0 ]]; then
  TRACE_METRICS="{}"
  FAILURE="no_output"
  if [[ -f "$TRACE" ]]; then
    cp -f "$TRACE" "${OUT}/trace.jsonl"
    TRACE_METRICS="$(python3 "${EVAL_ROOT}/runner/parse_trace.py" "$TRACE")"
    TURNS="$(python3 -c "import json,sys; m=json.load(sys.stdin); print(m.get('turns') or 'null')" <<<"$TRACE_METRICS")"
    if [[ "$TURNS" != "null" ]]; then
      FAILURE="timeout"
    fi
  fi
  emit_result_json "${OUT}/result.json" "$PROBLEM" "$ARM" "$REPEAT" \
    false false "$FAILURE" false \
    "$(python3 -c "import json,sys; m=json.load(sys.stdin); print(m.get('turns') or 'null')" <<<"$TRACE_METRICS")" \
    "$(python3 -c "import json,sys; m=json.load(sys.stdin); print(m.get('tool_calls') or 'null')" <<<"$TRACE_METRICS")" \
    "$(python3 -c "import json,sys; m=json.load(sys.stdin); print(m.get('vim_rs_tool_calls') or 'null')" <<<"$TRACE_METRICS")" \
    "$(python3 -c "import json,sys; m=json.load(sys.stdin); print(m.get('tokens_in') or 'null')" <<<"$TRACE_METRICS")" \
    "$(python3 -c "import json,sys; m=json.load(sys.stdin); print(m.get('tokens_out') or 'null')" <<<"$TRACE_METRICS")" \
    "$WALL_S"
  exit 7
fi

cp -f "$TRACE" "${OUT}/trace.jsonl" 2>/dev/null || true
SOLVE_SRC="${AGENT_DIR}/solve.rs"
if [[ ! -f "$SOLVE_SRC" && -f "${WORK}/src/solve.rs" ]]; then
  cp "${WORK}/src/solve.rs" "$SOLVE_SRC"
fi
if [[ -f "$SOLVE_SRC" ]]; then
  cp -f "$SOLVE_SRC" "${OUT}/solve.rs"
fi

if [[ ! -f "$SOLVE_SRC" ]]; then
  emit_result_json "${OUT}/result.json" "$PROBLEM" "$ARM" "$REPEAT" \
    false false no_output false null null null null null null
  # still attach wall_s below via merge if needed
  python3 -c "import json,sys; p=sys.argv[1]; w=float(sys.argv[2]); d=json.load(open(p)); d['wall_s']=w; open(p,'w').write(json.dumps(d,indent=2)+'\n')" \
    "${OUT}/result.json" "$WALL_S"
  exit 8
fi

GRADE_OUT="${OUT}/grade"
"${EVAL_ROOT}/runner/grade.sh" \
  --problem "$PROBLEM" \
  --solve "$SOLVE_SRC" \
  --out "$GRADE_OUT" \
  --arm "$ARM" \
  --repeat "$REPEAT" \
  --check-tamper \
  --agent-work "$WORK" || true

TRACE_METRICS="$(python3 "${EVAL_ROOT}/runner/parse_trace.py" "$TRACE")"
python3 - "$OUT/result.json" "$GRADE_OUT/result.json" "$TRACE_METRICS" "$WALL_S" <<'PY'
import json, sys
out_path, grade_path, metrics_json, wall_s = sys.argv[1:5]
metrics = json.loads(metrics_json)
grade = json.loads(open(grade_path).read())
grade.update(metrics)
grade["wall_s"] = float(wall_s)
open(out_path, "w").write(json.dumps(grade, indent=2) + "\n")
PY

echo "run_one OK: ${OUT}/result.json"
