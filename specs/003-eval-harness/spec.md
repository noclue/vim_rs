# Feature Specification: PropertyCollector Eval Harness

**Feature Branch**: `003-eval-harness`

**Created**: 2026-08-09

**Status**: Draft

**Input**: User description: "Detail a specification from mcp/docs/EvalHarness.md for a PropertyCollector proving-slice eval harness, delivered in four increments: (1) scaffold + vendored mock + pc01 end-to-end offline without Goose; (2) Containerfile + Goose smoke on Arm A; (3) Arm B + trace parsing; (4) expand to pc02–pc04 and calibration. Broader context from mcp/docs/GraphSearch.md."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Offline single-problem proving loop (Priority: P1)

A harness author can define one PropertyCollector problem (`pc01`), give an agent (or a human) a sealed project where only the path strings in a fixed retrieval contract may be filled in, and grade the result offline with no live vCenter and no coding-agent harness. Grading first checks that the solution builds, then checks that mock retrieval returns the expected field values for every fixture attached to the problem.

**Why this priority**: Without a trustworthy offline grader and one locked problem, later A/B runs cannot distinguish wrong-but-compiling paths from correct paths, and container/Goose work has nothing durable to measure.

**Independent Test**: With a known-correct `pc01` solution and a known-incorrect-but-compiling solution, the offline grader passes the first on all fixtures and fails the second; a tampered non-contract file is flagged.

**Acceptance Scenarios**:

1. **Given** the sealed scaffold and `pc01` with at least two fixtures that use different novel values, **When** a correct path solution is graded offline, **Then** the build gate passes and every fixture retrieval matches expected values.
2. **Given** a solution that builds but uses incorrect path strings whose leaf types still type-check, **When** graded offline, **Then** retrieval fails (fields unpopulated or mismatched) and the run is recorded as a retrieval failure, not a pass.
3. **Given** an attempted edit outside the single agent-writable contract file, **When** the grader resets and seals the workspace, **Then** the run is marked tampered and does not count as a clean pass.
4. **Given** only the problem goal and pinned contract (no correct path spellings exposed to the solver), **When** grading runs, **Then** the oracle (correct-keyed fixtures) remains unavailable to the solver workspace.

---

### User Story 2 - Baseline agent smoke in a sealed environment (Priority: P2)

A harness operator can run Arm A (baseline coding agent with filesystem/search tools only — no vim-rs MCP) against `pc01` inside a fresh sealed environment, capture a machine-readable run trace, and produce the same graded result record as the offline loop.

**Why this priority**: Proves the containerized agent path works before attributing any lift to MCP tools; establishes the baseline arm the scorecard compares against.

**Independent Test**: One Arm A smoke run on `pc01` completes (or fails with a classified outcome), writes a trace and a result record, and grades via the same two-tier offline path as Story 1.

**Acceptance Scenarios**:

1. **Given** a configured model and credentials for the coding harness, **When** Arm A is run once on `pc01` in a fresh sealed environment, **Then** the agent may edit only the contract file, a trace is captured, and Tier-1/Tier-2 grading runs after the agent finishes.
2. **Given** a completed Arm A run, **When** the operator inspects outputs, **Then** they see compile/retrieval/tamper/timeout (or equivalent) classification plus efficiency fields available from the trace (turns, tool calls, wall time; tokens when present).
3. **Given** network policy for the sealed environment, **When** the agent attempts to pull new dependencies or reach arbitrary hosts, **Then** dependency resolution stays offline for the library under test and only the intended model-provider egress is allowed.

---

### User Story 3 - Treatment arm and comparable telemetry (Priority: P2)

A harness operator can run Arm B (same instructions and baseline tools as Arm A, plus the existing vim-rs MCP property/search tools), parse traces so MCP tool use is visible, and compare Arm A vs Arm B on the same problem under identical prompts.

**Why this priority**: The product question is whether the *existing* MCP property surface helps agents synthesize correct VIM property paths; without Arm B and parsed traces, there is no attributable A/B signal.

**Independent Test**: One Arm A and one Arm B run on `pc01` with identical instructions (extensions differ only by MCP presence) each produce result records; Arm B traces show whether vim-rs tools were invoked.

**Acceptance Scenarios**:

1. **Given** identical task instructions for both arms except extension availability, **When** Arm A and Arm B each run on `pc01`, **Then** any outcome delta is attributable to tool surface, not prompt wording.
2. **Given** an Arm B run that called vim-rs property tools, **When** the trace is parsed, **Then** the result record reports total tool calls and vim-rs-specific tool-call counts (and tool names/args when present in the trace).
3. **Given** Arm B is configured with the MCP server available in the environment, **When** the run starts, **Then** the agent can use the existing property and search tools without changing the problem contract or fixtures.

---

### User Story 4 - Problem pack expansion, calibration, and decision scorecard (Priority: P3)

A harness operator can run the full PropertyCollector slice (`pc01`–`pc04`), including the array/downcast gap probe (`pc03`), repeat runs for variance, optionally calibrate model choice with a small stronger/weaker pair, and produce an A-vs-B scorecard that supports a clear-lift / inconclusive decision for whether to invest in richer retrieval tooling.

**Why this priority**: One problem cannot support the Phase-3 investment gate; the slice must cover camelCase traps, nested paths, a known MCP gap, and a second managed-object root, with repeats and a documented lift rule.

**Independent Test**: With problems `pc01`–`pc04` locked (paths validated against the field graph), an operator can execute a small matrix (problems × arms × repeats), get an aggregated scorecard, and apply the clear-lift rule without manual spreadsheet work.

**Acceptance Scenarios**:

1. **Given** validated problems `pc01`–`pc04` with ≥2 novel fixtures each, **When** the matrix is run for Arms A and B, **Then** per-problem and aggregate retrieval-pass, compile rate, and efficiency summaries appear with repeat variance.
2. **Given** `pc03` (array + downcast MAC path) as a gap probe, **When** results are reviewed, **Then** the scorecard still records outcomes honestly even if both arms struggle — failure of current tools on `pc03` is informative, not a harness defect.
3. **Given** the documented clear-lift rule and chosen repeat count, **When** the scorecard is produced, **Then** the operator gets an explicit verdict of clear lift or inconclusive (not a silent pass/fail of the harness itself).
4. **Given** a short calibration pass (one weaker and one stronger model on a few tasks), **When** the operator selects a model for the sweep, **Then** the choice is recorded and the sweep uses that fixed model and temperature-0 / bounded-turn settings for comparability.

---

### Edge Cases

- Agent produces no `solve` output or empties the contract → classified failure (no silent pass).
- Build succeeds but retrieval fails on only one of several fixtures → overall retrieval failure (all fixtures must pass).
- Agent hard-codes expected values instead of paths → multi-fixture grading with different values defeats the cheat.
- Trace missing token fields → efficiency still recorded from available turn/tool/wall metrics; token fields marked unavailable rather than invented.
- Rate limits or provider 5xx during agent runs → bounded retries with backoff; cost ceiling aborts further paid runs.
- Arm C (hint-only confound control) is out of scope until A/B shows lift; operators must not treat a missing Arm C as incomplete Phase-1 delivery.
- Target paths that do not resolve in the field graph must not be locked into problems; authoring fails closed until paths are verified.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The harness MUST provide a sealed evaluation project where the solver may change only the property-path strings in a pinned retrieval contract (struct name, managed-object target, and field names/types fixed).
- **FR-002**: Grading MUST be two-tier and offline: (1) the solution must build against the real library; (2) mock PropertyCollector retrieval seeded with correct-keyed fixtures MUST return expected values for every fixture on the problem.
- **FR-003**: Each problem MUST ship at least two fixtures with distinct novel values so hardcoded returns cannot pass all fixtures.
- **FR-004**: The grading oracle (correct path keys and expected values) MUST remain outside the solver-writable workspace; graders restore non-contract files from a canonical copy before grading and flag tamper when those files diverge.
- **FR-005**: Problem definitions MUST express a natural-language goal and the pinned contract without revealing correct VIM path spellings to the solver; grader-only notes may hold verified correct paths.
- **FR-006**: The harness MUST support offline end-to-end grading for `pc01` without any coding-agent runtime (human or scripted fill of the contract is sufficient to prove the loop).
- **FR-007**: The harness MUST support running Arm A (baseline tools only) and Arm B (baseline tools plus existing vim-rs MCP property/search tools) with instructions identical across arms except extension availability.
- **FR-008**: Each agent run MUST execute in a fresh sealed environment with offline library dependency resolution and restricted egress limited to the configured model provider.
- **FR-009**: After each agent run the harness MUST capture a machine-readable trace, grade via FR-002, and emit a per-run result record including problem id, arm, repeat, compile outcome, retrieval outcome, failure class, tamper flag, and available efficiency metrics.
- **FR-010**: Trace parsing MUST expose turn counts, tool-call counts, wall time, and — when the trace or provider usage data includes them — token counts; for Arm B it MUST count vim-rs MCP tool invocations separately when identifiable.
- **FR-011**: The initial problem pack MUST include four PropertyCollector tasks once expanded: `pc01` (VM power state + guest hostname), `pc02` (VM name + nested hardware CPU count spelling trap), `pc03` (array + downcast NIC MAC gap probe), `pc04` (HostSystem overall status + product full name); each target path MUST be verified against the field graph before the problem is locked.
- **FR-012**: Failure classification for the first iteration MUST include at least: compile error, retrieval failure, timeout, no output, and tamper.
- **FR-013**: The harness MUST aggregate repeats into an A-vs-B scorecard (retrieval-pass rate, compile rate, mean efficiency metrics, per-problem breakdown with variance) and apply a documented clear-lift vs inconclusive rule suitable for small-N runs.
- **FR-014**: Delivery MUST be incremental and demonstrable in this order: (1) offline scaffold + mock + `pc01`; (2) sealed environment + Arm A smoke; (3) Arm B + trace parsing; (4) `pc02`–`pc04` + repeats/calibration/scorecard.
- **FR-015**: Arm C (baseline plus a single hint line, no MCP) MUST remain optional and deferred until A/B shows lift; it is not required to exit the Phase-1 proving slice.
- **FR-016**: The harness MUST allow a cost ceiling and concurrency cap so operators can bound paid model usage during sweeps.

### Key Entities

- **Problem**: Identified PropertyCollector task (`pc01`–`pc04`) with natural-language goal, pinned contract, fixture list, and grader-only notes.
- **Fixture**: Canned PropertyCollector response plus expected retrieved field values; multiple per problem; values must be novel relative to published library test data.
- **Arm**: Experimental condition — A (baseline), B (baseline + MCP), optional later C (hint confound).
- **Run**: One execution of (problem × arm × repeat) producing trace, graded outcomes, and efficiency metrics.
- **Result record**: Structured per-run summary used for aggregation.
- **Scorecard**: Aggregated A/B comparison with clear-lift / inconclusive verdict.
- **Sealed scaffold**: Canonical evaluation project; only the contract file is solver-writable in spirit; other files restored before grading.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A known-correct `pc01` solution is graded offline end-to-end (build + all fixtures) in under 5 minutes on a developer machine without live vCenter or a coding agent.
- **SC-002**: A known incorrect-but-compiling `pc01` solution fails Tier-2 retrieval on at least one fixture 100% of the time in offline grading.
- **SC-003**: Tampering with any non-contract project file causes the run to be flagged tampered in 100% of checked cases.
- **SC-004**: An operator can complete one Arm A and one Arm B smoke run on `pc01` and obtain comparable result records (same schema) without changing problem text between arms.
- **SC-005**: For Arm B runs that invoke vim-rs tools, the parsed result identifies that those tools were used (non-zero vim-rs tool-call count when the trace lists them).
- **SC-006**: After expanding to `pc01`–`pc04` with the chosen repeat count (default 3–5), the scorecard reports per-problem retrieval-pass rates for A and B and an explicit clear-lift or inconclusive verdict under the documented rule.
- **SC-007**: Calibration (or a recorded decision to skip after a failed provider/tool-calling check) completes before a full sweep; the sweep model and turn bound are fixed and documented in the scorecard outputs.
- **SC-008**: Either clear lift or inconclusive exit still leaves a reusable sealed A/B rig and a baseline number that later retrieval-tool work must beat — operators do not need to rebuild the harness to measure a successor tool surface.

## Assumptions

- Phase 1 measures the **existing** vim-rs MCP property/search tools against baseline; it does not require new MCP retrieval tools (those are a later GraphSearch phase motivated by this slice, especially `pc03`).
- Offline mock retrieval is sufficient; no live vCenter is required for this feature.
- Coding-agent integration targets Goose with OpenRouter (or equivalent provider) as used in the design notes; exact install pin and machine-trace flags are verified during implementation, not left open in product scope.
- The mock client used today for library tests is vendored (or otherwise made available) into the sealed scaffold so grading does not depend on reaching into published crate test trees at runtime.
- Runner implementation language is an implementation detail; bash is an acceptable default if it meets FR-009–FR-013.
- Default repeats N = 3–5; clear-lift means B ≥ A on retrieval-pass in every repeat, median problem-pass delta ≥ 2 problems, and a bootstrap CI on per-run delta excluding 0 — otherwise inconclusive.
- Finer per-field failure taxonomy (wrong camelCase vs wrong parent vs missing downcast) is deferred beyond coarse FR-012 classes.
- Optional MCP-side per-call JSONL logging is a stretch; Phase 1 primary telemetry is the agent harness trace.
- Embeddings/search remain available to Arm B so discovery-by-concept is possible; property-path tasks primarily stress property tools.
- Event/MethodFault and Java-SDK problem packs are out of scope for this feature.
- Broader GraphSearch data-model work is out of scope; this feature only establishes the evaluation bar those efforts must clear.
