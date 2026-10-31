#!/usr/bin/env python3
"""Aggregate result.json files into scorecard.json (clear-lift rule from research R7)."""

from __future__ import annotations

import json
import sys
from collections import defaultdict
from pathlib import Path
from typing import Any


def load_results(root: Path) -> list[dict[str, Any]]:
    """Load attempt records only.

    Each attempt also has grade/result.json. That row has null efficiency
    fields, and averaging it with the merged record skews the scorecard.
    """
    paths = []
    direct = root / "result.json"
    if direct.is_file():
        paths.append(direct)
    paths.extend(sorted(root.glob("*/result.json")))
    results: list[dict[str, Any]] = []
    for path in paths:
        if path.parent.name == "grade":
            continue
        try:
            results.append(json.loads(path.read_text(encoding="utf-8")))
        except (json.JSONDecodeError, OSError):
            continue
    return results


def pass_rate(rows: list[dict[str, Any]]) -> float:
    if not rows:
        return 0.0
    return sum(1 for r in rows if r.get("retrieval_pass")) / len(rows)


def mean_metric(rows: list[dict[str, Any]], key: str) -> float | None:
    vals = [r.get(key) for r in rows if r.get(key) is not None]
    if not vals:
        return None
    return sum(vals) / len(vals)


def compile_rate(rows: list[dict[str, Any]]) -> float:
    if not rows:
        return 0.0
    return sum(1 for r in rows if r.get("compiled")) / len(rows)


def mean_filled(rows: list[dict[str, Any]], key: str) -> float:
    """Mean of `key`. A missing value counts as 0 (do not skip it)."""
    if not rows:
        return 0.0
    total = 0.0
    for row in rows:
        value = row.get(key)
        if value is not None:
            total += float(value)
    return total / len(rows)


def arm_stats(rows: list[dict[str, Any]]) -> dict[str, Any]:
    return {
        "retrieval_pass_rate": pass_rate(rows),
        "compile_rate": sum(1 for r in rows if r.get("compiled")) / len(rows) if rows else 0.0,
        "mean_turns": mean_metric(rows, "turns"),
        "mean_tool_calls": mean_metric(rows, "tool_calls"),
        "mean_tokens_in": mean_metric(rows, "tokens_in"),
        "mean_tokens_out": mean_metric(rows, "tokens_out"),
        "mean_wall_s": mean_metric(rows, "wall_s"),
    }


def clear_lift(problems: list[dict[str, Any]]) -> tuple[str, str]:
    """Simplified clear-lift: B wins every problem on pass rate and delta >= 2 problems."""
    wins = 0
    notes: list[str] = []
    for p in problems:
        a = p["arm_a"]["retrieval_pass_rate"]
        b = p["arm_b"]["retrieval_pass_rate"]
        if b >= a and b > 0:
            wins += 1
        notes.append(f"{p['id']}: A={a:.2f} B={b:.2f}")
    if wins >= 2 and wins == len(problems):
        return "clear_lift", "; ".join(notes)
    return "inconclusive", "; ".join(notes)


def parse_args(argv: list[str]) -> tuple[float | None, list[str]]:
    """Keep positional RESULTS_DIR OUT.json [model] [n_repeats]. `--min-lift` is optional."""
    min_lift: float | None = None
    positional: list[str] = []
    i = 0
    while i < len(argv):
        arg = argv[i]
        if arg == "--min-lift":
            if i + 1 >= len(argv):
                print("usage: scorecard.py [--min-lift X] RESULTS_DIR OUT.json [model] [n_repeats]", file=sys.stderr)
                raise SystemExit(2)
            min_lift = float(argv[i + 1])
            i += 2
            continue
        if arg.startswith("--min-lift="):
            min_lift = float(arg.split("=", 1)[1])
            i += 1
            continue
        positional.append(arg)
        i += 1
    return min_lift, positional


def main() -> None:
    min_lift, args = parse_args(sys.argv[1:])
    if len(args) < 2:
        print("usage: scorecard.py [--min-lift X] RESULTS_DIR OUT.json [model] [n_repeats]", file=sys.stderr)
        raise SystemExit(2)
    root = Path(args[0])
    out = Path(args[1])
    model = args[2] if len(args) > 2 else "unknown"
    n_repeats = int(args[3]) if len(args) > 3 else 1

    results = load_results(root)
    by_problem_arm: dict[str, dict[str, list]] = defaultdict(lambda: defaultdict(list))
    for r in results:
        by_problem_arm[r["problem"]][r["arm"]].append(r)

    problems_out: list[dict[str, Any]] = []
    for pid in sorted(by_problem_arm.keys()):
        problems_out.append(
            {
                "id": pid,
                "arm_a": arm_stats(by_problem_arm[pid].get("A", [])),
                "arm_b": arm_stats(by_problem_arm[pid].get("B", [])),
            }
        )

    verdict, verdict_notes = clear_lift(problems_out)
    arm_a = [row for arms in by_problem_arm.values() for row in arms.get("A", [])]
    arm_b = [row for arms in by_problem_arm.values() for row in arms.get("B", [])]
    retrieval_a = pass_rate(arm_a)
    retrieval_b = pass_rate(arm_b)
    compile_a = compile_rate(arm_a)
    compile_b = compile_rate(arm_b)
    tools_a = mean_filled(arm_a, "tool_calls")
    tools_b = mean_filled(arm_b, "tool_calls")
    vim_a = mean_filled(arm_a, "vim_rs_tool_calls")
    vim_b = mean_filled(arm_b, "vim_rs_tool_calls")
    retrieval_delta = retrieval_b - retrieval_a
    lift_pass = retrieval_delta >= (min_lift if min_lift is not None else 0.0) and compile_b >= compile_a and tools_b <= tools_a
    card = {
        "model": model,
        "n_repeats": n_repeats,
        "problems": problems_out,
        "verdict": verdict,
        "verdict_notes": verdict_notes,
        "calibration": {"note": "Run weaker + stronger model on 2-3 tasks before sweep; record here."},
        "lift": {
            "min_lift": min_lift,
            "retrieval_pass_rate_a": retrieval_a,
            "retrieval_pass_rate_b": retrieval_b,
            "retrieval_delta": retrieval_delta,
            "compile_rate_a": compile_a,
            "compile_rate_b": compile_b,
            "mean_tool_calls_a": tools_a,
            "mean_tool_calls_b": tools_b,
            "mean_vim_rs_tool_calls_a": vim_a,
            "mean_vim_rs_tool_calls_b": vim_b,
            "pass": lift_pass if min_lift is not None else None,
        },
    }
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(card, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(
        f"scorecard: {out} verdict={verdict} model={model} "
        f"retrieval_delta={retrieval_delta:.3f} compile={compile_a:.3f}->{compile_b:.3f} "
        f"tool_calls={tools_a:.3f}->{tools_b:.3f} vim_rs_tool_calls={vim_a:.3f}->{vim_b:.3f}"
    )
    if min_lift is not None and not lift_pass:
        print(
            f"min-lift {min_lift:.2f} missed "
            f"(need retrieval +{min_lift:.2f}, compile not down, tool calls not up)",
            file=sys.stderr,
        )
        raise SystemExit(1)


if __name__ == "__main__":
    main()
