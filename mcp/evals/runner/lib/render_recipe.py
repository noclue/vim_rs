#!/usr/bin/env python3
"""Render a Goose recipe with multiline goal/contract safely inlined (no --params)."""

from __future__ import annotations

import argparse
import json
import textwrap
from pathlib import Path


def literal_block(text: str, indent: int = 2) -> str:
    """Emit a YAML `|` block with uniform content indentation."""
    pad = " " * indent
    raw = textwrap.dedent(text.replace("\r\n", "\n")).strip("\n")
    lines = raw.split("\n") if raw else [""]
    return "|\n" + "".join(f"{pad}{line}\n" for line in lines)


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", required=True, help="Output recipe YAML path")
    ap.add_argument("--goal-file", required=True)
    ap.add_argument("--contract-file", required=True)
    ap.add_argument("--model", required=True)
    ap.add_argument("--provider", default="openrouter", help="Goose provider id (openrouter or openai)")
    ap.add_argument("--mcp", action="store_true", help="Include vim-rs stdio extension (Arm B)")
    ap.add_argument(
        "--mcp-cmd",
        default="/opt/vim_mcp_server",
        help="Arm B MCP server executable (container path or host .exe)",
    )
    args = ap.parse_args()

    goal = textwrap.dedent(Path(args.goal_file).read_text(encoding="utf-8")).strip()
    contract = textwrap.dedent(Path(args.contract_file).read_text(encoding="utf-8")).strip()
    property_contract = "vim_retrievable" in contract
    if property_contract:
        how = (
            "Complete exactly this contract in src/solve.rs (fill in the property-path "
            "strings, and implement any function stubs shown):\n"
        )
    else:
        how = (
            "Implement the function already declared in src/solve.rs. "
            "Its signature is this contract:\n"
        )
    prompt = f"{goal}\n\n{how}{contract}\n"

    extensions = ["  - type: builtin\n    name: developer\n"]
    if args.mcp:
        extensions.append(
            "  - type: stdio\n"
            "    name: vim-rs\n"
            f"    cmd: {json.dumps(args.mcp_cmd)}\n"
            "    args: []\n"
            "    timeout: 300\n"
            "    description: vim_rs vSphere API assistant\n"
        )

    if property_contract:
        description = "Complete the vim_retrievable! contract in src/solve.rs."
        task = (
            "Complete `src/solve.rs` only: fill in the property-path strings in the\n"
            "  `vim_retrievable!` contract and implement any function stubs it contains."
        )
        find = "Use whatever tools are available to find the correct VIM property paths and API usage."
    else:
        description = "Implement the function declared in src/solve.rs."
        task = (
            "Complete `src/solve.rs` only: implement the function whose signature is\n"
            "  already in that file. Do not add a `vim_retrievable!` macro."
        )
        find = "Use whatever tools are available to find the correct vim_rs API usage."
    body = f"""version: "1.0.0"
title: vim_rs eval
description: {description}
instructions: |
  You write Rust using the `vim_rs` crate (a vSphere API client). The project is in the
  working directory. {task}
  Do not modify any file other than `src/solve.rs`. Do not mock the vSphere API, write
  fixtures, or run the program. That is outside the test scope.
  The only verification you should perform is `cargo build`.
  {find}
prompt: {literal_block(prompt, 2)}extensions:
{''.join(extensions)}settings:
  goose_provider: {json.dumps(args.provider)}
  goose_model: {json.dumps(args.model)}
  temperature: 0
  max_turns: 40
"""
    Path(args.out).write_text(body, encoding="utf-8", newline="\n")
    print(args.out)


if __name__ == "__main__":
    main()
