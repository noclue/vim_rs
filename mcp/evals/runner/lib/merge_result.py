"""Merge a host grade result.json with trace metrics."""

from __future__ import annotations

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from parse_trace import parse_trace


def main() -> None:
    out_path, grade_path, wall_s, trace_path = sys.argv[1:5]
    grade = json.loads(Path(grade_path).read_text(encoding="utf-8-sig"))
    grade.update(parse_trace(Path(trace_path)))
    grade["wall_s"] = float(wall_s)
    Path(out_path).write_text(json.dumps(grade, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    main()
