#!/usr/bin/env bash
set -euo pipefail
EVAL_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
python3 "${EVAL_ROOT}/runner/scorecard.py" "$@"
