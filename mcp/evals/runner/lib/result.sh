#!/usr/bin/env bash
# Emit result.json matching specs/003-eval-harness/contracts/result.schema.json
set -euo pipefail

emit_result_json() {
  local out_file="$1"
  local problem="$2"
  local arm="$3"
  local repeat="$4"
  local compiled="$5"
  local retrieval_pass="$6"
  local failure_class="$7"
  local tamper="$8"
  local turns="${9:-null}"
  local tool_calls="${10:-null}"
  local vim_rs_tool_calls="${11:-null}"
  local tokens_in="${12:-null}"
  local tokens_out="${13:-null}"
  local wall_s="${14:-null}"

  python3 - "$out_file" "$problem" "$arm" "$repeat" "$compiled" "$retrieval_pass" \
    "$failure_class" "$tamper" "$turns" "$tool_calls" "$vim_rs_tool_calls" \
    "$tokens_in" "$tokens_out" "$wall_s" <<'PY'
import json, sys
out, problem, arm, repeat, compiled, retrieval_pass, failure_class, tamper = sys.argv[1:9]
rest = sys.argv[9:]

def jnull(v):
    return None if v == "null" else json.loads(v) if v.startswith("{") or v.startswith("[") else (
        int(v) if v.isdigit() else float(v) if "." in v else v
    )

record = {
    "problem": problem,
    "arm": arm,
    "repeat": int(repeat),
    "compiled": compiled == "true",
    "retrieval_pass": retrieval_pass == "true",
    "failure_class": None if failure_class == "null" else failure_class,
    "tamper": tamper == "true",
    "turns": jnull(rest[0]),
    "tool_calls": jnull(rest[1]),
    "vim_rs_tool_calls": jnull(rest[2]),
    "tokens_in": jnull(rest[3]),
    "tokens_out": jnull(rest[4]),
    "wall_s": jnull(rest[5]),
}
with open(out, "w") as f:
    json.dump(record, f, indent=2)
    f.write("\n")
PY
}
