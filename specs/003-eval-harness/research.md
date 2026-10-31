# Research: PropertyCollector Eval Harness

**Feature**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)  
**Date**: 2026-08-09

Resolves Technical Context unknowns and EvalHarness.md `[VERIFY]` / `[DECIDE]` items needed before implementation.

---

## R1 — Mock client: vendor + extend for `RetrievePropertiesEx`

**Decision**: Vendor a copy of `vim_rs/tests/support/mock_vim_client.rs` into `mcp/evals/scaffold/src/mock/`, then **extend** it so Tier-2 grading scripts **`RetrievePropertiesEx`** (and related view lifecycle) rather than relying on `WaitForUpdatesEx`.

**Rationale**:
- `ObjectRetriever` (used by snippets / `vim_retrievable!` workflows) calls `PropertyCollector::retrieve_properties_ex` → `invoke_optional(..., "RetrievePropertiesEx", ...)`.
- Upstream `MockVimClient` today scripts `WaitForUpdatesEx` (TaskTracker tests), `CreateListView`, `CreateFilter`, `DestroyView`, etc. Unhandled `RetrievePropertiesEx` falls through `handle_option_bytes` to `Ok(None)` → empty retrieval, useless as an oracle.
- EvalHarness.md §3 wording (“canned `WaitForUpdatesEx`”) is **incorrect for ObjectRetriever**; correct oracle is a canned **`RetrieveResult`** whose `ObjectContent.propSet[].name` keys match the **correct VIM paths**. Wrong agent paths → fields `None` / mismatch → Tier-2 fail.

**Alternatives considered**:
| Option | Rejected because |
|--------|------------------|
| Depend on `vim_rs` test support via path | Not shipped on crates.io; agent/container must use published pin |
| Grade via `WaitForUpdatesEx` + filters | Diverges from real `ObjectRetriever` consumer path |
| Patch upstream mock in `vim_rs` first | Nice follow-up; not required to unblock eval (Principle III — keep eval out of core compile) |

**Implementation note**: Queue fixture bytes (or decode fixture → `RetrieveResult` → miniserde JSON) for `RetrievePropertiesEx`. Prefer **`retrieve_objects_from_list`** with fixed MoRefs so `CreateContainerView` need not be mocked initially (`CreateListView` already exists).

---

## R2 — Sealed `main` / ObjectRetriever entry point

**Decision**: Sealed `main.rs` builds `MockVimClient` + dummy `ServiceContent` (with `property_collector` + `view_manager` MoRefs), seeds one fixture per grade invocation via `FIXTURE` env, calls `ObjectRetriever::retrieve_objects_from_list::<solve::Target>(&fixed_morefs)`, prints JSON extract for comparison.

**Rationale**: Matches public API in `vim_rs/src/core/pc_retrieve.rs` and snippet patterns (`retrieve_host_info`, `vm_ip`). List path minimizes mock surface vs container views.

**Alternatives considered**: `retrieve_objects_from_container` (needs `CreateContainerView` mock); `retrieve_object` single MoRef (also fine for single-VM problems — acceptable simplification for `pc01`).

---

## R3 — Scaffold `vim_rs` features

**Decision**: Depend on `vim_rs` with **default features** (`default-client`). Do **not** enable `xml` unless a locked problem proves SOAP-only types are required (unlikely for PropertyCollector path strings + JSON mock).

**Rationale**: Mock implements `Transport::Json`. `vim_retrievable!` / `ObjectRetriever` work on JSON. Dropping `xml` shrinks container warm/fetch and matches Principle III for the scaffold image.

**Verify at implement time**: Exact crates.io version pin (`0.5.x`); confirm `MockVimClient`-shaped types compile against that pin (vendored mock may need small API sync).

---

## R4 — Runner language

**Decision**: **Bash** orchestration (`grade.sh`, `run_matrix.sh`, `scorecard.sh`) plus a small **Python or `jq`** helper for Goose `stream-json` → metrics if bash alone is too brittle.

**Rationale**: Spec assumes bash is enough; matrix is `(problem × arm × repeat)` with podman + file I/O. Python only for JSONL event aggregation if needed.

**Alternatives considered**: Full Python runner (more moving parts); Rust runner crate (overkill for Phase 1).

---

## R5 — Goose install, flags, tokens

**Decision**:
- Install CLI via official script in Containerfile; **pin a release tag/version** (record digest or version env) — do not float `stable` unmarked.
- Run: `goose run --recipe … --params … --no-session --output-format stream-json` (flags confirmed in Goose docs / CLI).
- Redirect stream to `/out/trace.jsonl`.
- Token fields: parse from stream events if present; else mark `tokens_*` null and optionally consult OpenRouter usage later — do not invent.

**Rationale**: Docs confirm `--output-format stream-json|json|text` and `--no-session`. Stream-json is JSONL events suited to turn/tool accounting.

**Still verify at first container smoke** (implementation checklist, not open product scope): exact event schema for tool name/args; whether usage tokens appear in-stream.

---

## R6 — Embeddings / model cache in Arm B image

**Decision**: Keep semantic `search` available for Arm B. Prefer **embedding model assets baked or volume-mounted** so the MCP server starts without network model download during a sealed run. Property tools (`get_property_path` / `get_property_tree`) remain the primary stress; `search` is secondary discovery.

**Rationale**: Spec Assumptions; EvalHarness §5.4. Property tasks do not require embeddings, but identical MCP binary behavior vs production avoids a confound (“MCP without search”).

**Alternatives considered**: Strip embeddings from eval image (smaller, but changes Arm B tool surface vs production).

---

## R7 — Repeats, Arm C, MCP JSONL, failure classes

**Decision** (locked from spec Assumptions):
- **N = 3–5** repeats (default **3** for first sweep; raise to 5 if variance high).
- **Arm C** deferred until clear A/B lift.
- **MCP-side JSONL logging** deferred; telemetry = Goose trace.
- **Failure classes**: `{ compile_error, retrieval_fail, timeout, no_output, tamper }` only.

**Clear-lift rule** (scorecard): B ≥ A on retrieval-pass in every repeat **and** median `P_pass(B) − P_pass(A) ≥ 2` problems **and** bootstrap CI on per-run delta excludes 0; else **inconclusive**.

---

## R8 — Problem path verification

**Decision**: Before locking each problem, resolve grader-only correct paths with MCP `get_property_path` / field graph (`resolver::resolve_path` or equivalent). Provisional candidates from EvalHarness:

| ID | Intent | Candidate paths (verify before lock) |
|----|--------|--------------------------------------|
| pc01 | VM power + guest host | `runtime.powerState`, `guest.hostName` |
| pc02 | name + numCPU trap | `name`, `config.hardware.numCPU` |
| pc03 | array + downcast gap | `config.hardware.device[*]→VirtualEthernetCard.macAddress` |
| pc04 | Host status + product | `summary.overallStatus`, `summary.config.product.fullName` |

**Rationale**: FR-011 fail-closed; wrong locked paths invalidate the whole slice.

---

## R9 — Delivery increments (maps to FR-014)

| Slice | Delivers | Exit check |
|-------|----------|------------|
| 1 | Scaffold, vendored+extended mock, `pc01` fixtures, offline `grade.sh` | SC-001–SC-003 |
| 2 | Containerfile, Goose pin, `mcp_off.yaml`, Arm A smoke | Arm A result.json + trace |
| 3 | `mcp_on.yaml`, MCP in image, trace parser, Arm B smoke | SC-004–SC-005 |
| 4 | pc02–pc04, matrix, calibration note, scorecard | SC-006–SC-008 |

---

## R10 — Network / seal / anti-cheat

**Decision**: Fresh container per run; `CARGO_NET_OFFLINE=true` after warm fetch at image build; restore `/work` from `/eval/scaffold-canonical` except `src/solve.rs`; hash-diff → `tamper`; ≥2 novel fixtures per problem; oracle fixtures live under `/eval/fixtures` (not agent-writable).

**Rationale**: Spec FR-001–FR-004; EvalHarness §3–§4.

---

## Open items deferred to implementation smoke (not NEEDS CLARIFICATION)

These are verify-at-build checks, not unresolved product decisions:

1. Exact Goose release pin string and stream-json event fields for tool calls / tokens.
2. OpenRouter model id with reliable tool calling (calibration slice).
3. Final path strings after field-graph verification.
4. Whether per-problem sealed `main` variants are needed vs one parameterized grader.
