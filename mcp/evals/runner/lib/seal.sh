#!/usr/bin/env bash
# Seal/reset work tree from canonical scaffold; detect tamper on non-solve.rs files.
set -euo pipefail

_SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")" && pwd)"
EVAL_ROOT="$(cd "${_SCRIPT_DIR}/../.." && pwd)"
CANONICAL="${EVAL_ROOT}/scaffold"
SOLVE_REL="src/solve.rs"

hash_file() {
  python3 - "$1" <<'PY'
import hashlib, sys
path = sys.argv[1]
h = hashlib.sha256()
with open(path, "rb") as f:
    h.update(f.read())
print(h.hexdigest())
PY
}

hash_tree_excluding_solve() {
  local dir="$1"
  python3 - "$dir" <<'PY'
import hashlib, os, sys
root = sys.argv[1]
skip = {"src/solve.rs", "src/problem_emit.rs", "Cargo.lock"}
h = hashlib.sha256()
for dirpath, _, files in sorted(os.walk(root)):
    for name in sorted(files):
        rel = os.path.relpath(os.path.join(dirpath, name), root)
        if rel in skip or rel.startswith("target" + os.sep):
            continue
        h.update(rel.encode())
        with open(os.path.join(root, rel), "rb") as f:
            h.update(f.read())
print(h.hexdigest())
PY
}

materialize_work() {
  local work_dir="$1"
  rm -rf "$work_dir"
  mkdir -p "$work_dir"
  rsync -a --exclude target "$CANONICAL/" "$work_dir/"
}

install_challenge_start() {
  local work_dir="$1"
  local problem="$2"
  local start="${EVAL_ROOT}/runner/starts/${problem}.rs"
  local agent_emit="${EVAL_ROOT}/runner/agent_emit/${problem}.rs"
  local grade_emit="${EVAL_ROOT}/runner/problem_emit/${problem}.rs"
  if [[ -f "$start" ]]; then
    cp "$start" "$work_dir/src/solve.rs"
  fi
  if [[ -f "$agent_emit" ]]; then
    cp "$agent_emit" "$work_dir/src/problem_emit.rs"
  elif [[ -f "$grade_emit" ]]; then
    cp "$grade_emit" "$work_dir/src/problem_emit.rs"
  fi
}

restore_seal() {
  local work_dir="$1"
  if [[ ! -d "$work_dir" ]]; then
    materialize_work "$work_dir"
    return 0
  fi
  local solve_backup=""
  if [[ -f "$work_dir/$SOLVE_REL" ]]; then
    solve_backup="$(mktemp)"
    cp "$work_dir/$SOLVE_REL" "$solve_backup"
  fi
  rsync -a --delete --exclude target "$CANONICAL/" "$work_dir/"
  if [[ -n "$solve_backup" ]]; then
    cp "$solve_backup" "$work_dir/$SOLVE_REL"
    rm -f "$solve_backup"
  fi
}

check_tamper() {
  local work_dir="$1"
  local canon_hash work_hash
  canon_hash="$(hash_tree_excluding_solve "$CANONICAL")"
  work_hash="$(hash_tree_excluding_solve "$work_dir")"
  if [[ "$canon_hash" != "$work_hash" ]]; then
    return 0
  fi
  return 1
}

install_solve() {
  local work_dir="$1"
  local solve_src="$2"
  cp "$solve_src" "$work_dir/$SOLVE_REL"
}

install_problem_emit() {
  local work_dir="$1"
  local emit_src="$2"
  cp "$emit_src" "$work_dir/src/problem_emit.rs"
}
