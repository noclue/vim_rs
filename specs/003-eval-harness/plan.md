# Implementation Plan: PropertyCollector Eval Harness

**Branch**: `003-eval-harness` | **Date**: 2026-08-09 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/003-eval-harness/spec.md` (design notes: `mcp/docs/EvalHarness.md`, context: `mcp/docs/GraphSearch.md`)

**Note**: Filled by `/speckit-plan`. Workflow: `.specify/templates/plan-template.md`.

## Summary

Build a sealed, offline-gradable **A/B eval harness** under `mcp/evals/` that measures whether the **existing** vim-rs MCP property/search tools help coding agents fill correct VIM PropertyCollector path strings in a pinned `vim_retrievable!` contract. Delivery is incremental: (1) scaffold + vendored/extended mock + `pc01` offline grading; (2) Containerfile + Goose Arm A smoke; (3) Arm B + stream-json trace parsing; (4) `pc02`–`pc04`, repeats, calibration, scorecard. No live vCenter; Tier-2 grades via mock `RetrievePropertiesEx` fixtures keyed by correct paths.

## Technical Context

**Language/Version**: Rust 2021 (scaffold + sealed harness; pin toolchain to `rust:1.95` eval image); Bash runner; Goose CLI (Linux, pinned release)  
**Primary Dependencies**: `vim_rs` (crates.io pin, default features — no `xml` required for JSON mock path), `vim_macros` via `vim_rs`, `tokio`, `miniserde`/`serde_json` for result lines; Goose + OpenRouter for agent arms; rootless Podman for sealed runs  
**Storage**: Filesystem only — `problems/*.yaml`, `fixtures/*.json`, `/out/{trace,result,build.log,solve.rs}`; `results/` gitignored  
**Testing**: Offline golden solutions (correct + wrong-but-compiling + tamper); container smoke (Arm A then Arm B); matrix scorecard vs clear-lift rule  
**Target Platform**: Developer macOS/Linux for offline loop; Linux containers (amd64/arm64) for Goose runs  
**Project Type**: Eval tooling under `mcp/evals/` (scaffold Cargo crate + bash runner + Containerfile); not a change to published `vim_rs` API  
**Performance Goals**: Offline `pc01` correct-path grade &lt; 5 minutes (SC-001); agent runs bounded by `max_turns` (40) and cost ceiling  
**Constraints**: Solver edits only `src/solve.rs`; cargo offline inside agent env; OpenRouter egress only; no MCP API changes required for Phase 1 (optional JSONL logging deferred); measure existing tools only  
**Scale/Scope**: 4 problems × 2 arms × 3–5 repeats; ~one Containerfile; bash orchestration; scorecard aggregation

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle / standard | Status | Evidence |
|---------------------|--------|----------|
| **I. Generated from specs** | Pass | No edits to generated `vim_rs/src/types/` or `mo/` |
| **II. Complete type-safe surface** | Pass | Harness *consumes* `vim_retrievable!` / `ObjectRetriever`; does not alter VIM surface |
| **III. Build-time & binary budgets** | Pass | New code lives under `mcp/evals/`; does not expand default `vim_rs` compile surface. MCP binary already large; container builds release `vim_mcp_server` only for Arm B image path |
| **IV. Multi-transport parity** | Pass (scoped) | Phase 1 is **intentionally offline** via mock JSON client — not a substitute for release vCenter gates. Documented in Assumptions / research |
| **V. Documented public surfaces** | Pass | `mcp/evals/README.md` + [quickstart.md](./quickstart.md) + contracts for problem/result schemas |
| **VI. Readable source** | Pass | Small scaffold, vendored mock with clear RetrievePropertiesEx scripting, bash runner in focused scripts |
| **VII. Ecosystem tooling** | Pass | Eval harness is first-party MCP product tooling (measures property tools / GraphSearch Phase-3 bar) |
| **Marshalling (miniserde)** | Pass | Mock fixtures use VI-JSON shapes compatible with `vim_rs` unmarshal; scaffold may use `serde_json` only for grader stdout lines (eval tooling, not `vim_rs` crate) |
| **Security & secrets** | Pass | `OPENROUTER_API_KEY` via env at `podman run`; never committed; traces may be gitignored under `results/` |
| **Feature flags** | Pass | Scaffold pins `vim_rs` without forcing `xml`/`defaults` unless a locked problem proves need |

**Post-design re-check**: [research.md](./research.md), [data-model.md](./data-model.md), [contracts/](./contracts/), [quickstart.md](./quickstart.md) align with gates. Complexity Tracking empty — no justified violations.

## Project Structure

### Documentation (this feature)

```text
specs/003-eval-harness/
├── plan.md              # This file
├── research.md          # Phase 0
├── data-model.md        # Phase 1
├── quickstart.md        # Phase 1
├── contracts/           # Phase 1
├── spec.md
├── checklists/
└── tasks.md             # /speckit-tasks (not created by /speckit-plan)
```

### Source Code (repository root)

```text
mcp/evals/
├── README.md
├── Containerfile
├── scaffold/                 # Sealed Cargo project
│   ├── Cargo.toml            # vim_rs pin; vendored mock module
│   ├── src/
│   │   ├── main.rs           # Sealed grader entry (per-problem or parameterized)
│   │   ├── solve.rs          # Agent-writable contract stub
│   │   └── mock/             # Vendored + extended MockVimClient
│   └── solutions/            # Optional: known-correct / wrong-but-compiling (not in /work)
├── fixtures/                 # Canned RetrieveResult / ObjectContent JSON (≥2 per problem)
├── problems/*.yaml           # goal + contract + fixtures[] + grader notes
├── recipes/
│   ├── mcp_off.yaml          # Arm A
│   └── mcp_on.yaml           # Arm B
├── runner/                   # bash: grade-offline, run-one, aggregate
│   ├── grade.sh
│   ├── run_matrix.sh
│   ├── parse_trace.py        # or jq/awk — parse Goose stream-json → metrics
│   └── scorecard.sh
└── results/                  # gitignored traces + scorecards

mcp/docs/EvalHarness.md       # Design notes (already present; remains authoritative for intent)
```

**Structure Decision**: All harness code under **`mcp/evals/`** per EvalHarness layout. Scaffold is a standalone Cargo package (not added as an `mcp/` workspace member unless packaging requires it). Runner is bash-first. Container multi-stage builds Linux `vim_mcp_server` for Arm B.

## Complexity Tracking

> No constitution violations requiring justification.

| Violation | Why needed | Simpler alternative rejected because |
|-----------|------------|-------------------------------------|
| — | — | — |

---

## Phase 0: Research

**Status**: Complete → [research.md](./research.md)

Resolved decisions:

| ID | Topic | Outcome |
|----|-------|---------|
| R1 | Mock / ObjectRetriever | Vendor mock; **extend for `RetrievePropertiesEx`** (not WaitForUpdatesEx) |
| R2 | Retrieval entry point | Prefer `retrieve_objects_from_list` + fixed MoRefs |
| R3 | `xml` feature | Default features only unless a problem forces otherwise |
| R4 | Runner | Bash + small JSON helpers |
| R5 | Goose telemetry | `--output-format stream-json --no-session`; pin Goose release |
| R6 | Embeddings in image | Bundle/mount model so Arm B `search` works |
| R7 | N / Arm C / MCP JSONL | N=3–5; Arm C deferred; MCP JSONL deferred |
| R8 | Delivery slices | Match FR-014 / user stories P1→P3 |

---

## Phase 1: Design & Contracts

**Status**: Complete

| Artifact | Path |
|----------|------|
| Data model | [data-model.md](./data-model.md) |
| Contracts | [contracts/](./contracts/) |
| Quickstart | [quickstart.md](./quickstart.md) |

---

## Phase 2: Tasks

**Status**: Not started — run `/speckit-tasks` to generate `tasks.md`.

Suggested task grouping (non-binding until tasks command):

1. Offline scaffold + mock extension + `pc01` fixtures + `grade.sh`
2. Containerfile + Goose pin + Arm A recipe smoke
3. Arm B recipe + MCP binary in image + trace parser
4. `pc02`–`pc04`, matrix, calibration notes, scorecard / clear-lift
