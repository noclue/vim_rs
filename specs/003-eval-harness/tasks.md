# Tasks: PropertyCollector Eval Harness

**Input**: Design documents from `/specs/003-eval-harness/`  
**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/](./contracts/), [quickstart.md](./quickstart.md)

**Tests**: Spec does not request a separate TDD suite. Validation is via golden `solve.rs` solutions (correct / wrong-but-compiling / tamper), offline `grade.sh`, and Arm A/B smokes per [quickstart.md](./quickstart.md).

**Organization**: Tasks grouped by user story (P1 → P2 → P2 → P3) with shared setup + foundation first. Delivery maps to FR-014 slices.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no blocking deps on incomplete tasks)
- **[Story]**: US1–US4 per [spec.md](./spec.md)

## Path Conventions

- Harness root: `mcp/evals/`
- Scaffold: `mcp/evals/scaffold/`
- Runner: `mcp/evals/runner/`
- Specs: `specs/003-eval-harness/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create `mcp/evals/` tree, ignore rules, and operator-facing stubs so later phases have a home.

- [X] T001 Create directory layout `mcp/evals/{scaffold/src,fixtures,problems,recipes,runner,results,solutions}` per [plan.md](./plan.md)
- [X] T002 [P] Add `mcp/evals/results/` to the appropriate `.gitignore` (repo root and/or `mcp/evals/.gitignore`)
- [X] T003 [P] Write skeleton `mcp/evals/README.md` linking design notes (`mcp/docs/EvalHarness.md`) and [quickstart.md](./quickstart.md)
- [X] T004 [P] Copy contract schemas reference note into `mcp/evals/README.md` (point at `specs/003-eval-harness/contracts/`)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Sealed scaffold crate, vendored/extended mock with `RetrievePropertiesEx`, shared result emission, and offline grade skeleton. **Blocks all user stories.**

**⚠️ CRITICAL**: No user-story grading or Goose work until this phase completes.

- [X] T005 Initialize standalone Cargo package in `mcp/evals/scaffold/Cargo.toml` pinning `vim_rs` from crates.io (default features; no `xml` unless proven needed) plus `tokio`, JSON helpers for grader stdout
- [X] T006 Vendor `vim_rs/tests/support/mock_vim_client.rs` into `mcp/evals/scaffold/src/mock/mod.rs` (and support files as needed) and wire `mod mock` in the scaffold crate
- [X] T007 Extend `mcp/evals/scaffold/src/mock/mod.rs` to script `RetrievePropertiesEx` returning canned `RetrieveResult` bytes (fixture-driven); keep existing `CreateListView` / `DestroyView` behavior for list retrieval
- [X] T008 Add dummy `ServiceContent` helper (property_collector + view_manager MoRefs) in `mcp/evals/scaffold/src/mock/service_content.rs` (or adjacent module)
- [X] T009 Implement sealed grader entry in `mcp/evals/scaffold/src/main.rs`: load `FIXTURE` env, seed mock, `ObjectRetriever::retrieve_objects_from_list` (or `retrieve_object`) on `solve::Target`, print JSON extract line
- [X] T010 Add agent-writable stub `mcp/evals/scaffold/src/solve.rs` with blank/`TODO` path strings for a placeholder contract (replaced per-problem in later tasks)
- [X] T011 [P] Implement result-record writer helper in `mcp/evals/runner/lib/result.sh` (or `result_emit.sh`) matching `specs/003-eval-harness/contracts/result.schema.json`
- [X] T012 Implement seal/reset + tamper hash check in `mcp/evals/runner/lib/seal.sh` (canonical copy vs work tree; only `src/solve.rs` may differ)
- [X] T013 Implement Tier-1 + Tier-2 loop skeleton in `mcp/evals/runner/grade.sh` (restore seal → `cargo build` → per-fixture run → emit `result.json`) per `specs/003-eval-harness/contracts/runner-cli.md`
- [X] T014 Verify scaffold builds offline-capable after `cargo fetch` once on a developer machine; document fetch step in `mcp/evals/README.md`

**Checkpoint**: Scaffold compiles; mock can return a hand-made `RetrieveResult`; `grade.sh` exists and can emit a `result.json` shape (even if `pc01` not locked yet).

---

## Phase 3: User Story 1 — Offline single-problem proving loop (Priority: P1) 🎯 MVP

**Goal**: Lock `pc01`, fixtures, golden solutions; offline grade correct / wrong / tamper without Goose.

**Independent Test**: Correct `pc01` solution passes all fixtures in &lt; 5 minutes; wrong-but-compiling paths → `retrieval_fail`; non-`solve.rs` edit → `tamper=true` ([quickstart.md](./quickstart.md) Slice 1; SC-001–SC-003).

### Implementation for User Story 1

- [X] T015 [US1] Verify candidate paths `runtime.powerState` and `guest.hostName` against the field graph / MCP property tools; record verified paths in grader-only `notes` of `mcp/evals/problems/pc01_vm_power_and_host.yaml`
- [X] T016 [P] [US1] Author `mcp/evals/problems/pc01_vm_power_and_host.yaml` per `specs/003-eval-harness/contracts/problem.schema.json` (goal + pinned contract; no path spoilers in `goal`)
- [X] T017 [P] [US1] Create ≥2 novel fixtures `mcp/evals/fixtures/pc01_a.json` and `mcp/evals/fixtures/pc01_b.json` as `RetrieveResult` payloads keyed by **correct** paths with distinct values
- [X] T018 [US1] Align sealed `mcp/evals/scaffold/src/main.rs` and default `solve.rs` contract with `pc01` field names/types (`VirtualMachinePowerState`, `Option<String>`, etc.)
- [X] T019 [P] [US1] Add golden correct solution `mcp/evals/solutions/pc01_correct.rs`
- [X] T020 [P] [US1] Add wrong-but-compiling solution `mcp/evals/solutions/pc01_wrong_paths.rs` (valid types, incorrect path strings)
- [X] T021 [US1] Wire `mcp/evals/runner/grade.sh` to load problem YAML, map fixtures/`expected`, install `--solve` into work `src/solve.rs`, and compare stdout JSON to `expected` for all fixtures
- [X] T022 [US1] Run `mcp/evals/runner/grade.sh --problem pc01_vm_power_and_host --solve mcp/evals/solutions/pc01_correct.rs` and confirm `compiled=true`, `retrieval_pass=true`, `failure_class=null` in emitted `result.json`
- [X] T023 [US1] Run `mcp/evals/runner/grade.sh --problem pc01_vm_power_and_host --solve mcp/evals/solutions/pc01_wrong_paths.rs` and confirm `retrieval_fail` in `result.json` (SC-002)
- [X] T024 [US1] Document/perform tamper check (modify non-`solve.rs` under work tree) and confirm `tamper=true` via `mcp/evals/runner/lib/seal.sh` + `grade.sh` (SC-003)
- [X] T025 [US1] Copy canonical scaffold snapshot instructions into `mcp/evals/README.md` (how `/eval/scaffold-canonical` will be populated in-container; host offline uses `scaffold/` + seal copy)

**Checkpoint**: Offline `pc01` MVP complete — no Goose required. Stop here to demo SC-001–SC-003.

---

## Phase 4: User Story 2 — Baseline agent smoke in sealed environment (Priority: P2)

**Goal**: Containerized Goose Arm A (`mcp_off`) on `pc01` produces the same graded `result.json` schema as offline grading.

**Independent Test**: One Arm A smoke run writes `trace.jsonl` + `result.json` after seal + Tier-1/Tier-2 ([quickstart.md](./quickstart.md) Slice 2).

### Implementation for User Story 2

- [X] T026 [US2] Author Arm A recipe `mcp/evals/recipes/mcp_off.yaml` (developer builtin only; instructions identical to future Arm B minus MCP extension; params `goal`, `contract`, `model`; temperature 0; max_turns 40)
- [X] T027 [US2] Write multi-stage `mcp/evals/Containerfile`: rust build base, warm `vim_rs` into registry/cache, copy canonical scaffold + recipes/problems/fixtures to `/eval`, set `CARGO_NET_OFFLINE=true` / Goose env defaults
- [X] T028 [US2] Pin Goose CLI install in `mcp/evals/Containerfile` (record version/tag in `mcp/evals/README.md`); verify `goose run --help` lists `--output-format stream-json` and `--no-session`
- [X] T029 [US2] Implement `mcp/evals/runner/run_one.sh` for Arm A: `podman run` with OpenRouter env, recipe `mcp_off.yaml`, params from problem YAML, redirect stream-json to `$OUT/trace.jsonl`, then invoke seal + `grade.sh` logic on container `/work` or copied `solve.rs`
- [X] T030 [US2] Enforce network policy notes in `mcp/evals/README.md` / `run_one.sh` (OpenRouter egress; cargo offline) matching FR-008
- [X] T031 [US2] Add cost-ceiling / timeout env hooks in `mcp/evals/runner/run_one.sh` (`EVAL_COST_CEILING`, turn/wall bounds → `failure_class=timeout` when exceeded)
- [X] T032 [US2] Run Arm A smoke on `pc01` and confirm `result.json` schema validity vs `specs/003-eval-harness/contracts/result.schema.json` (efficiency fields may be null until US3 parser)

**Checkpoint**: Arm A path works end-to-end for `pc01`.

---

## Phase 5: User Story 3 — Treatment arm and comparable telemetry (Priority: P2)

**Goal**: Arm B (+ vim-rs MCP) with identical instructions; parse Goose traces for turns/tool calls/`vim_rs_tool_calls`.

**Independent Test**: Arm A and Arm B smokes on `pc01` share result schema; Arm B with MCP tool use shows non-zero `vim_rs_tool_calls` when tools appear in the trace (SC-004–SC-005).

### Implementation for User Story 3

- [X] T033 [P] [US3] Author Arm B recipe `mcp/evals/recipes/mcp_on.yaml` (copy Arm A instructions; add stdio extension `vim-rs` → `/opt/vim_mcp_server`)
- [X] T034 [US3] Extend `mcp/evals/Containerfile` multi-stage build to compile release `vim_mcp_server` from `mcp/` and install at `/opt/vim_mcp_server`; include or mount embedding model assets so `search` starts (research R6)
- [X] T035 [US3] Extend `mcp/evals/runner/run_one.sh` to select `mcp_on.yaml` when `--arm B`
- [X] T036 [US3] Implement `mcp/evals/runner/parse_trace.py` (or `parse_trace.sh` + `jq`) reading Goose `stream-json` JSONL → `turns`, `tool_calls`, `vim_rs_tool_calls`, `tokens_*` (null if absent), `wall_s`
- [X] T037 [US3] Integrate trace parser into `mcp/evals/runner/run_one.sh` so `result.json` fills efficiency fields per `specs/003-eval-harness/contracts/result.schema.json`
- [X] T038 [US3] Run Arm B smoke on `pc01` and confirm comparable `result.json` vs Arm A; document observed `vim_rs_tool_calls` behavior in `mcp/evals/README.md` (SC-004–SC-005)

**Checkpoint**: A/B attributable on `pc01` with parsed telemetry.

---

## Phase 6: User Story 4 — Problem pack, calibration, and scorecard (Priority: P3)

**Goal**: Lock `pc02`–`pc04` (including gap probe `pc03`), matrix runner, calibration note, clear-lift scorecard.

**Independent Test**: Matrix over `pc01`–`pc04` × A/B × N repeats emits `scorecard.json` with `verdict` `clear_lift` or `inconclusive` (SC-006–SC-008).

### Implementation for User Story 4

- [X] T039 [P] [US4] Verify and lock paths for `pc02` (name + `config.hardware.numCPU`); author `mcp/evals/problems/pc02_*.yaml` + ≥2 fixtures under `mcp/evals/fixtures/`
- [X] T040 [P] [US4] Verify and lock paths for `pc03` array/downcast MAC gap probe; author `mcp/evals/problems/pc03_*.yaml` + ≥2 fixtures under `mcp/evals/fixtures/` (honest expectation: both arms may struggle)
- [X] T041 [P] [US4] Verify and lock paths for `pc04` HostSystem status + product fullName; author `mcp/evals/problems/pc04_*.yaml` + ≥2 fixtures under `mcp/evals/fixtures/`
- [X] T042 [US4] Extend scaffold/grader to support per-problem contracts (parameterized `solve` stub install from problem `contract`, or per-problem sealed main variants) in `mcp/evals/scaffold/` + `mcp/evals/runner/grade.sh`
- [X] T043 [US4] Implement `mcp/evals/runner/run_matrix.sh` (`--problems`, `--arms`, `--repeats`, `--model`, concurrency cap, cost ceiling) calling `run_one.sh`
- [X] T044 [US4] Implement `mcp/evals/runner/scorecard.sh` aggregating `result.json` files into `specs/003-eval-harness/contracts/scorecard.schema.json` with clear-lift rule from [research.md](./research.md) R7
- [X] T045 [US4] Add calibration procedure section to `mcp/evals/README.md` (weaker + stronger model, 2–3 tasks; record chosen model / skip reason in scorecard `calibration`)
- [X] T046 [US4] Run a dry matrix (at least 1 repeat × all problems × A/B if budget allows, or offline-only grade for new problems + 1 paid smoke) and produce a sample `mcp/evals/results/` scorecard artifact locally (gitignored)
- [X] T047 [US4] Confirm Arm C remains undocumented as required delivery (optional only); note deferral in `mcp/evals/README.md`

**Checkpoint**: Full Phase-1 proving slice exit criteria available (clear lift or inconclusive + reusable rig).

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Docs sync, quickstart validation, operator ergonomics.

- [X] T048 [P] Expand `mcp/evals/README.md` with layout, seal rules, anti-cheat, env vars, and clear-lift definition
- [X] T049 [P] Align `specs/003-eval-harness/quickstart.md` commands with final `runner/*.sh` flag names
- [X] T050 Run through `specs/003-eval-harness/quickstart.md` Slice 1 fully; fix gaps in `mcp/evals/runner/*.sh` and `mcp/evals/README.md`
- [X] T051 [P] Add example `result.json` / `scorecard.json` snippets under `specs/003-eval-harness/contracts/examples/` or `mcp/evals/README.md` (no secrets)
- [X] T052 Confirm no secrets (`OPENROUTER_API_KEY`, traces with keys) are staged; verify ignore rules in `mcp/evals/.gitignore` and/or root `.gitignore` cover `mcp/evals/results/`
- [X] T053 [P] Optional: add `mcp/evals/recipes/mcp_off_hinted.yaml` stub commented as Arm C deferred (do not wire into `mcp/evals/runner/run_matrix.sh`)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Start immediately
- **Foundational (Phase 2)**: Depends on Setup — **BLOCKS** US1–US4
- **US1 (Phase 3)**: Depends on Foundational — **MVP**
- **US2 (Phase 4)**: Depends on US1 (`pc01` + `grade.sh` must work offline first)
- **US3 (Phase 5)**: Depends on US2 (Containerfile + `run_one.sh` base); recipe file can be drafted in parallel earlier
- **US4 (Phase 6)**: Depends on US3 for full A/B matrix; new problems can be authored offline in parallel after US1
- **Polish (Phase 7)**: After desired stories complete

### User Story Dependencies

```text
Setup → Foundation → US1 (offline pc01) → US2 (Arm A) → US3 (Arm B + traces) → US4 (pack + scorecard)
                         ↘ (parallel after US1) author pc02–pc04 fixtures for US4
```

- **US1**: No dependency on Goose/Podman
- **US2**: Needs US1 problem + grader
- **US3**: Needs US2 container/run path
- **US4**: Needs US3 for paid matrix; offline locking of pc02–pc04 can proceed once foundation + US1 patterns exist

### Parallel Opportunities

- T002–T004 (setup docs/gitignore)
- T011 parallel with T006–T010 once grade skeleton planned
- T016–T017, T019–T020 within US1
- T033 parallel with T034–T036 within US3
- T039–T041 problem authoring in parallel within US4 (after path verify)

---

## Parallel Example: User Story 1

```bash
# After foundation (T005–T014), in parallel:
Task: "Author pc01_vm_power_and_host.yaml in mcp/evals/problems/"
Task: "Create fixtures pc01_a.json and pc01_b.json in mcp/evals/fixtures/"
Task: "Add solutions/pc01_correct.rs and solutions/pc01_wrong_paths.rs"

# Then sequential:
Task: "Wire grade.sh expected comparison"
Task: "Run correct + wrong + tamper validation"
```

## Parallel Example: User Story 4

```bash
# Path-verify then author in parallel:
Task: "pc02 problem + fixtures"
Task: "pc03 problem + fixtures"
Task: "pc04 problem + fixtures"
# Then:
Task: "run_matrix.sh + scorecard.sh"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1 Setup  
2. Phase 2 Foundational (mock + sealed main + `grade.sh`)  
3. Phase 3 US1 (`pc01` offline)  
4. **STOP and VALIDATE** SC-001–SC-003  
5. Demo offline proving loop before any Goose spend  

### Incremental Delivery

1. US1 → offline bar exists  
2. US2 → Arm A smoke (baseline)  
3. US3 → Arm B + telemetry (attributable A/B)  
4. US4 → investment gate scorecard for GraphSearch Phase 3  
5. Polish → docs/quickstart sync  

### Suggested MVP Scope

**T001–T025** (Setup + Foundation + US1). Do not start Containerfile/Goose until offline `pc01` is green.

---

## Notes

- Primary telemetry is Goose `stream-json`; MCP-side JSONL logging is out of scope (research R7)
- Arm C deferred until clear A/B lift (FR-015)
- Correct EvalHarness doc error: grade via **`RetrievePropertiesEx`**, not `WaitForUpdatesEx` (research R1)
- Prefer `retrieve_objects_from_list` to avoid mocking `CreateContainerView` initially
- Commit after each phase checkpoint; keep `results/` untracked
