#!/usr/bin/env python3
"""Parse Goose stream-json JSONL into efficiency fields for result.json."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any


VIM_RS_TOOL_PREFIXES = ("vim-rs", "vim_rs", "get_property", "list_property", "search")


def parse_trace(path: Path) -> dict[str, Any]:
    """Parse Goose `--output-format stream-json` traces.

    Goose streams many tiny `message` events (token chunks). Count:
    - turns ≈ distinct assistant message ids (or assistant messages with toolRequest)
    - tool_calls from content[].type == toolRequest
    - tokens from the final `complete` event when present
    """
    assistant_ids: set[str] = set()
    tool_calls = 0
    vim_rs_tool_calls = 0
    tokens_in: int | None = None
    tokens_out: int | None = None

    if not path.exists():
        return {
            "turns": None,
            "tool_calls": None,
            "vim_rs_tool_calls": None,
            "tokens_in": None,
            "tokens_out": None,
        }

    for line in path.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line or not line.startswith("{"):
            continue
        try:
            evt = json.loads(line)
        except json.JSONDecodeError:
            continue

        etype = evt.get("type") or evt.get("event")

        if etype == "complete":
            if evt.get("input_tokens") is not None:
                tokens_in = int(evt["input_tokens"])
            if evt.get("output_tokens") is not None:
                tokens_out = int(evt["output_tokens"])
            continue

        if etype != "message":
            continue

        msg = evt.get("message") or {}
        role = msg.get("role")
        mid = msg.get("id")
        content = msg.get("content") or []

        if role == "assistant" and mid:
            assistant_ids.add(mid)

        if not isinstance(content, list):
            continue
        for part in content:
            if not isinstance(part, dict):
                continue
            if part.get("type") != "toolRequest":
                continue
            tool_calls += 1
            tool_call = part.get("toolCall") or {}
            value = tool_call.get("value") or {}
            name = str(value.get("name") or "").lower()
            ext = ""
            meta = part.get("_meta") or {}
            if isinstance(meta, dict):
                ext = str(meta.get("goose_extension") or "").lower()
            hay = f"{name} {ext}"
            if any(p in hay for p in VIM_RS_TOOL_PREFIXES):
                vim_rs_tool_calls += 1

    turns = len(assistant_ids) if assistant_ids else None
    return {
        "turns": turns,
        "tool_calls": tool_calls if tool_calls else None,
        "vim_rs_tool_calls": vim_rs_tool_calls if vim_rs_tool_calls else None,
        "tokens_in": tokens_in,
        "tokens_out": tokens_out,
    }


def main() -> None:
    trace = Path(sys.argv[1])
    print(json.dumps(parse_trace(trace)))


if __name__ == "__main__":
    main()
