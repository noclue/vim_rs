# Data Model: PropertyCollector Eval Harness

**Feature**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

Logical entities persisted as YAML/JSON files under `mcp/evals/`. No database.

---

## Problem

A single PropertyCollector evaluation task.

| Field | Type | Rules |
|-------|------|-------|
| `id` | string | Unique; pattern `pcNN_…` (e.g. `pc01_vm_power_and_host`) |
| `pillar` | string | Phase-1: `propcollector` |
| `goal` | string | Natural language only — **must not** reveal correct VIM path spellings or camelCase answers |
| `contract` | string | Pinned `vim_retrievable!` stub text (struct name, MO type, field names/types fixed; path placeholders only) |
| `fixtures` | list&lt;FixtureRef&gt; | Length ≥ 2 |
| `notes` | string (optional) | Grader-only; may list verified correct paths; **not** copied into agent workspace |

**Validation**: Target paths in `notes` verified against field graph before problem is “locked”. Unlocked problems must not enter the sweep matrix.

**Relationships**: Problem 1—* FixtureRef; Problem 1—* Run.

---

## FixtureRef

Pointer from a problem to a canned response + expected extract.

| Field | Type | Rules |
|-------|------|-------|
| `file` | path | Relative to `mcp/evals/fixtures/` |
| `expected` | list&lt;object&gt; | Expected retrieved field values after Tier-2 (same shape as grader stdout JSON array elements) |

---

## Fixture (file)

Canned PropertyCollector **`RetrieveResult`** (VI-JSON) used to seed the mock.

| Field | Meaning |
|-------|---------|
| Wire payload | Bytes/JSON unmarshalled as `RetrieveResult` with `objects[].propSet[].name` = **correct** VIM paths |
| Novelty | Values must not copy published `vim_rs` test literals the agent can `rg` from the warmed crate source |

**Invariant**: Grading keys off path names in `propSet`. Agent-supplied wrong paths → unpopulated fields → mismatch vs `expected`.

---

## Arm

Experimental condition.

| Value | Meaning |
|-------|---------|
| `A` | Baseline tools only (`developer` / filesystem); no vim-rs MCP |
| `B` | Arm A + vim-rs MCP stdio extension |
| `C` | Optional later: Arm A + hint line; **out of Phase-1 delivery** |

---

## Recipe

Goose recipe file selecting extensions + shared instructions.

| Field | Rules |
|-------|-------|
| Instructions / prompt | Identical across A/B except extension list |
| Parameters | `goal`, `contract`, `model` (and arm-invariant settings: temperature 0, max turns) |

---

## Run

One execution of `(problem_id × arm × repeat)`.

| Field | Type | Rules |
|-------|------|-------|
| `problem` | string | Problem `id` |
| `arm` | enum | `A` \| `B` ( \| `C` later) |
| `repeat` | int ≥ 1 | |
| `compiled` | bool | Tier-1 |
| `retrieval_pass` | bool | Tier-2 all fixtures |
| `failure_class` | enum \| null | `compile_error` \| `retrieval_fail` \| `timeout` \| `no_output` \| `tamper` \| null if clean pass |
| `tamper` | bool | Non-`solve.rs` divergence after reset |
| `turns` | int \| null | From trace |
| `tool_calls` | int \| null | From trace |
| `vim_rs_tool_calls` | int \| null | Arm B; 0 or null for A |
| `tokens_in` / `tokens_out` | int \| null | Null if unavailable |
| `wall_s` | number \| null | |

**Outputs**: `/out/trace.jsonl`, `/out/build.log`, `/out/result.json`, copy of `/out/solve.rs`.

**State transitions**:

```text
pending → agent_running → seal_reset → tier1_build → tier2_fixtures → recorded
                ↘ timeout / no_output ↗
```

Tamper may short-circuit to `recorded` with `tamper=true` without counting as clean pass.

---

## Result record

Canonical serialization of **Run** fields → `result.json` (see [contracts/result.schema.json](./contracts/result.schema.json)).

---

## Scorecard

Aggregation over a matrix of Runs.

| Field | Meaning |
|-------|---------|
| `model` | Fixed sweep model id |
| `n_repeats` | |
| `problems` | Per-problem pass rates A vs B + efficiency means |
| `verdict` | `clear_lift` \| `inconclusive` per documented rule |
| `calibration` | Optional note: models tried / skip reason |

---

## Sealed scaffold

Canonical tree under `/eval/scaffold-canonical` → copied to `/work`.

| Path | Writability |
|------|-------------|
| `src/solve.rs` | Solver-writable |
| All other files | Restored from canonical before grade; hash mismatch → tamper |

---

## Entity relationship (summary)

```text
Problem ──┬── FixtureRef ──► Fixture (RetrieveResult file)
          └── notes (grader-only)

Recipe (per Arm) ──► Goose run ──► Run ──► Result record
                                      └── Trace events

Runs ──► Scorecard (verdict)
```
