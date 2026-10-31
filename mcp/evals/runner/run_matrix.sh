#!/usr/bin/env bash
set -euo pipefail

EVAL_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

PROBLEMS=""
ARMS="A,B"
REPEATS=3
MODEL=""
CONCURRENCY=2

usage() {
  echo "Usage: $0 --problems pc01,pc02 --arms A,B --repeats N --model MODEL"
  exit 2
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --problems) PROBLEMS="$2"; shift 2 ;;
    --arms) ARMS="$2"; shift 2 ;;
    --repeats) REPEATS="$2"; shift 2 ;;
    --model) MODEL="$2"; shift 2 ;;
    --concurrency) CONCURRENCY="$2"; shift 2 ;;
    -h|--help) usage ;;
    *) usage ;;
  esac
done

[[ -n "$PROBLEMS" && -n "$MODEL" ]] || usage

STAMP="$(date +%Y%m%d_%H%M%S)"
MATRIX_DIR="${EVAL_ROOT}/results/matrix_${STAMP}"
mkdir -p "$MATRIX_DIR"

IFS=',' read -ra PROB_ARR <<<"$PROBLEMS"
IFS=',' read -ra ARM_ARR <<<"$ARMS"

for problem in "${PROB_ARR[@]}"; do
  for arm in "${ARM_ARR[@]}"; do
    for ((r=1; r<=REPEATS; r++)); do
      out="${MATRIX_DIR}/${problem}_${arm}_r${r}"
      echo "matrix: $problem arm=$arm repeat=$r"
      "${EVAL_ROOT}/runner/run_one.sh" \
        --problem "$problem" \
        --arm "$arm" \
        --repeat "$r" \
        --model "$MODEL" \
        --out "$out" || true
    done
  done
done

python3 "${EVAL_ROOT}/runner/scorecard.py" "$MATRIX_DIR" "${MATRIX_DIR}/scorecard.json" "$MODEL" "$REPEATS"
echo "matrix complete: $MATRIX_DIR"
