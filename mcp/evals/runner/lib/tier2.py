#!/usr/bin/env python3
"""Tier-2 fixture loop for grade.sh"""

from __future__ import annotations

import json
import os
import subprocess
import sys
from pathlib import Path

# A solve.rs that never returns must not hang the grader.
SOLVE_TIMEOUT_SECS = float(os.environ.get("EVAL_SOLVE_TIMEOUT", "120"))


def main() -> int:
    work_dir = Path(sys.argv[1])
    fixtures_dir = Path(sys.argv[2])
    raw_problem = sys.argv[3]
    problem_path = Path(raw_problem)
    if problem_path.is_file():
        problem = json.loads(problem_path.read_text(encoding="utf-8-sig"))
    else:
        problem = json.loads(raw_problem)
    mo_type = sys.argv[4]
    target_dir = sys.argv[5] if len(sys.argv) > 5 and sys.argv[5] else None

    for fx in problem.get("fixtures", []):
        file_name = fx["file"]
        expected = fx["expected"]
        payload = json.loads((fixtures_dir / file_name).read_text(encoding="utf-8"))
        objects = payload.get("objects", [])
        vm_ids = ",".join(o["obj"]["value"] for o in objects)
        entity_refs = ",".join(
            f"{o['obj'].get('type', 'VirtualMachine')}:{o['obj']['value']}" for o in objects
        )
        fixture_path = str(fixtures_dir / file_name)
        env = os.environ.copy()
        env["FIXTURE"] = fixture_path
        env["VM_IDS"] = vm_ids
        env["ENTITY_REFS"] = entity_refs
        env["MO_TYPE"] = mo_type
        # Run the binary built above. `cargo run` would keep CARGO_TARGET_DIR
        # in the child, and that path can walk back to fixtures/ and fixture_gen/.
        env.pop("CARGO_TARGET_DIR", None)
        if target_dir:
            exe = "eval_scaffold.exe" if os.name == "nt" else "eval_scaffold"
            cmd = [str(Path(target_dir) / "debug" / exe)]
        else:
            cmd = ["cargo", "run", "--quiet", "--bin", "eval_scaffold"]
        try:
            proc = subprocess.run(
                cmd,
                cwd=work_dir,
                env=env,
                capture_output=True,
                text=True,
                encoding="utf-8",
                errors="replace",
                timeout=SOLVE_TIMEOUT_SECS,
            )
        except subprocess.TimeoutExpired:
            print(json.dumps({"ok": False, "error": f"timed out after {SOLVE_TIMEOUT_SECS:g}s", "file": file_name}))
            return 1
        if proc.returncode != 0:
            print(json.dumps({"ok": False, "error": proc.stderr, "file": file_name}))
            return 1
        out = proc.stdout.strip().splitlines()[-1] if proc.stdout else ""
        got = json.loads(out) if out else []
        if got != expected:
            print(json.dumps({"ok": False, "file": file_name, "got": got, "expected": expected}))
            return 1
    print(json.dumps({"ok": True}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
