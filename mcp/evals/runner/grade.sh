#!/usr/bin/env bash
set -euo pipefail

EVAL_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUNNER_LIB="${EVAL_ROOT}/runner/lib"

source "${RUNNER_LIB}/result.sh"
source "${RUNNER_LIB}/seal.sh"

PROBLEM=""
SOLVE=""
WORK_DIR=""
OUT_DIR=""
ARM="A"
REPEAT=1
CHECK_TAMPER=false
AGENT_WORK=""
CLEAN_WORK=false

usage() {
  echo "Usage: $0 --problem ID --solve PATH [--work-dir DIR] [--out DIR] [--check-tamper] [--agent-work DIR]"
  exit 2
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --problem) PROBLEM="$2"; shift 2 ;;
    --solve) SOLVE="$2"; shift 2 ;;
    --work-dir) WORK_DIR="$2"; shift 2 ;;
    --out) OUT_DIR="$2"; shift 2 ;;
    --arm) ARM="$2"; shift 2 ;;
    --repeat) REPEAT="$2"; shift 2 ;;
    --check-tamper) CHECK_TAMPER=true; shift ;;
    --agent-work) AGENT_WORK="$2"; shift 2 ;;
    -h|--help) usage ;;
    *) echo "unknown arg: $1" >&2; usage ;;
  esac
done

[[ -n "$PROBLEM" && -n "$SOLVE" ]] || usage

PROBLEM_FILE="${EVAL_ROOT}/problems/${PROBLEM}.yaml"
[[ -f "$PROBLEM_FILE" ]] || { echo "missing problem: $PROBLEM_FILE" >&2; exit 2; }

# scaffold/Cargo.toml names vim_rs relative to mcp/evals/scaffold. A work copy
# under $TMPDIR cannot resolve that, so point it at this checkout.
repair_host_vim_rs_path() {
  python3 - "$1/Cargo.toml" "$(cd "${EVAL_ROOT}/../../vim_rs" && pwd)" <<'PY'
import sys
path, repo = sys.argv[1], sys.argv[2]
needle = 'path = "../../../vim_rs"'
with open(path, encoding="utf-8") as f:
    text = f.read()
if needle in text:
    with open(path, "w", encoding="utf-8") as f:
        f.write(text.replace(needle, f'path = "{repo}"'))
PY
}

if [[ -z "$WORK_DIR" ]]; then
  WORK_DIR="$(mktemp -d "${TMPDIR:-/tmp}/eval-work.XXXXXX")"
  CLEAN_WORK=true
fi
cleanup_work() {
  if [[ "$CLEAN_WORK" == true ]]; then
    rm -rf "$WORK_DIR"
  fi
}
trap cleanup_work EXIT
if [[ -z "$OUT_DIR" ]]; then
  OUT_DIR="${EVAL_ROOT}/results/grade_${PROBLEM}_$$"
fi
mkdir -p "$OUT_DIR"
OUT_DIR="$(cd "$OUT_DIR" && pwd)"

EMIT_SRC="${EVAL_ROOT}/runner/problem_emit/${PROBLEM}.rs"
[[ -f "$EMIT_SRC" ]] || EMIT_SRC="${EVAL_ROOT}/scaffold/src/problem_emit.rs"

# Hash the tree the agent edited. The compile tree is a fresh scaffold plus
# solve.rs, so hashing it cannot see other agent edits.
TAMPER=false
if [[ "$CHECK_TAMPER" == true && -n "$AGENT_WORK" ]] && check_tamper "$AGENT_WORK"; then
  TAMPER=true
  emit_result_json "${OUT_DIR}/result.json" "$PROBLEM" "$ARM" "$REPEAT" \
    false false tamper true null null null null null null
  exit 3
fi

materialize_work "$WORK_DIR"
install_solve "$WORK_DIR" "$SOLVE"
install_problem_emit "$WORK_DIR" "$EMIT_SRC"
repair_host_vim_rs_path "$WORK_DIR"

BUILD_LOG="${OUT_DIR}/build.log"
# Outside the repo so the graded binary cannot walk from its own path into
# fixtures/ or fixture_gen/. Unset before tier-2; the binary path is passed
# separately and is not left in the child environment.
if [[ -n "${EVAL_HOST_TARGET:-}" ]]; then
  HOST_TARGET="$EVAL_HOST_TARGET"
else
  HOST_TARGET="${XDG_CACHE_HOME:-$HOME/.cache}/vim-rs-eval/host-target"
fi
mkdir -p "$HOST_TARGET"
export CARGO_TARGET_DIR="$HOST_TARGET"
# Copies keep the source mtimes. A shared host target then treats the previous
# grade's binary as fresh even though solve.rs and problem_emit.rs changed.
(cd "$WORK_DIR" && cargo clean -p eval_scaffold >/dev/null 2>&1) || true
if ! (cd "$WORK_DIR" && cargo build --bin eval_scaffold 2>"$BUILD_LOG"); then
  emit_result_json "${OUT_DIR}/result.json" "$PROBLEM" "$ARM" "$REPEAT" \
    false false compile_error false null null null null null null
  exit 4
fi
unset CARGO_TARGET_DIR

PROBLEM_JSON="$(python3 "${RUNNER_LIB}/load_problem.py" "$PROBLEM_FILE" "${EVAL_ROOT}/fixtures")"
MO_TYPE="$(python3 -c "import json,sys; print(json.load(sys.stdin).get('mo_type','VirtualMachine'))" <<<"$PROBLEM_JSON")"

if ! python3 "${RUNNER_LIB}/tier2.py" "$WORK_DIR" "${EVAL_ROOT}/fixtures" "$PROBLEM_JSON" "$MO_TYPE" "$HOST_TARGET"; then
  emit_result_json "${OUT_DIR}/result.json" "$PROBLEM" "$ARM" "$REPEAT" \
    true false retrieval_fail false null null null null null null
  exit 5
fi

emit_result_json "${OUT_DIR}/result.json" "$PROBLEM" "$ARM" "$REPEAT" \
  true true null false null null null null null null

echo "grade OK: ${OUT_DIR}/result.json"
exit 0
