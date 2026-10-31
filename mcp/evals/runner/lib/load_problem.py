#!/usr/bin/env python3
"""Load eval problem YAML (stdlib only — no PyYAML required)."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from typing import Any


def load_problem_text(text: str) -> dict[str, Any]:
    lines = text.splitlines()
    data: dict[str, Any] = {}
    i = 0
    while i < len(lines):
        line = lines[i]
        if not line.strip() or line.strip().startswith("#"):
            i += 1
            continue
        m = re.match(r"^([a-z_]+):\s*(.*)$", line)
        if not m:
            i += 1
            continue
        key, rest = m.group(1), m.group(2).strip()
        if rest == "|":
            block: list[str] = []
            i += 1
            while i < len(lines) and (lines[i].startswith(" ") or lines[i].startswith("\t") or not lines[i].strip()):
                if lines[i].strip():
                    block.append(lines[i].rstrip())
                i += 1
            data[key] = "\n".join(block)
            continue
        if key == "fixtures" and not rest:
            fixtures: list[dict[str, Any]] = []
            i += 1
            while i < len(lines) and lines[i].startswith("  -"):
                entry: dict[str, Any] = {}
                head = lines[i].strip()
                if head.startswith("- file:"):
                    entry["file"] = head.split(":", 1)[1].strip()
                i += 1
                while i < len(lines) and lines[i].startswith("    "):
                    sub = lines[i].strip()
                    if sub.startswith("file:"):
                        entry["file"] = sub.split(":", 1)[1].strip()
                    elif sub.startswith("expected:"):
                        exp_rest = sub.split(":", 1)[1].strip()
                        if exp_rest.startswith("'") and exp_rest.endswith("'"):
                            exp_rest = exp_rest[1:-1]
                        if exp_rest.startswith('"') and exp_rest.endswith('"'):
                            exp_rest = exp_rest[1:-1]
                        entry["expected"] = json.loads(exp_rest)
                    i += 1
                if entry:
                    fixtures.append(entry)
            data["fixtures"] = fixtures
            continue
        if rest:
            data[key] = rest.strip('"')
        i += 1
    return data


def _parse_expected_block(items: list[str]) -> list[dict[str, Any]]:
    """Parse flattened fixture expected entries from indented yaml fragments."""
    # items like "- power_state: poweredOn" grouped per object
    result: list[dict[str, Any]] = []
    current: dict[str, Any] = {}
    for item in items:
        if item.startswith("- "):
            if current:
                result.append(current)
            current = {}
            item = item[2:].strip()
        if ":" in item:
            k, v = item.split(":", 1)
            k, v = k.strip(), v.strip()
            if v == "null":
                current[k] = None
            elif v.startswith('"') and v.endswith('"'):
                current[k] = v[1:-1]
            elif v.isdigit():
                current[k] = int(v)
            else:
                current[k] = v
    if current:
        result.append(current)
    return result


def load_problem(path: Path) -> dict[str, Any]:
    try:
        import yaml  # type: ignore

        return yaml.safe_load(path.read_text(encoding="utf-8"))
    except ImportError:
        return load_problem_text(path.read_text(encoding="utf-8"))


def fixture_vm_ids(fixtures_dir: Path, file_name: str) -> str:
    payload = json.loads((fixtures_dir / file_name).read_text(encoding="utf-8"))
    return ",".join(o["obj"]["value"] for o in payload.get("objects", []))


def main() -> None:
    if len(sys.argv) < 2:
        print("usage: load_problem.py PROBLEM_YAML [fixtures_dir]", file=sys.stderr)
        raise SystemExit(2)
    problem_path = Path(sys.argv[1])
    problem = load_problem(problem_path)
    print(json.dumps(problem, indent=2))


if __name__ == "__main__":
    main()
