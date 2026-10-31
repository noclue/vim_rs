"""Write one eval result.json record. All values arrive as strings."""

from __future__ import annotations

import json
import sys


def jnull(value: str):
    if value == "null" or value == "":
        return None
    if value[:1] in "{[":
        return json.loads(value)
    if value.isdigit() or (value[:1] == "-" and value[1:].isdigit()):
        return int(value)
    try:
        if "." in value:
            return float(value)
    except ValueError:
        pass
    return value


def main() -> None:
    (
        out,
        problem,
        arm,
        repeat,
        compiled,
        retrieval_pass,
        failure_class,
        tamper,
        turns,
        tool_calls,
        vim_rs_tool_calls,
        tokens_in,
        tokens_out,
        wall_s,
    ) = sys.argv[1:15]
    record = {
        "problem": problem,
        "arm": arm,
        "repeat": int(repeat),
        "compiled": compiled == "true",
        "retrieval_pass": retrieval_pass == "true",
        "failure_class": None if failure_class == "null" else failure_class,
        "tamper": tamper == "true",
        "turns": jnull(turns),
        "tool_calls": jnull(tool_calls),
        "vim_rs_tool_calls": jnull(vim_rs_tool_calls),
        "tokens_in": jnull(tokens_in),
        "tokens_out": jnull(tokens_out),
        "wall_s": jnull(wall_s),
    }
    with open(out, "w", encoding="utf-8", newline="\n") as handle:
        json.dump(record, handle, indent=2)
        handle.write("\n")


if __name__ == "__main__":
    main()
