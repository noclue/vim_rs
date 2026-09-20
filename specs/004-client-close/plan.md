# Implementation Plan: Explicit Client Session Close (0.6.1)

**Branch**: `004-client-close` | **Date**: 2026-09-21 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/004-client-close/spec.md`

**Note**: Filled by `/speckit-plan`. Workflow: `.specify/templates/plan-template.md`.

## Summary

Add a non-consuming async **`close(&self)`** on the public **`Client`** facade and on **`VimClient`** (default no-op for mocks; JSON/SOAP override with SessionManager Logout). Keep 0.6.0 destructor logout on the **multi-worker** Tokio runtime when `close` was never called. On **`current_thread`**, skip `block_in_place` and `warn!` instead of panicking. Document destructor logout as a 0.6.x fallback to be removed in **0.7.0** without changing the `close` signature. No Cargo feature. Update snippets, in-tree vtui, README, and helper rustdoc.

## Technical Context

**Language/Version**: Rust 2021, stable (existing `vim_rs` MSRV / CI)  
**Primary Dependencies**: `tokio` 1.x (`rt-multi-thread` already on), `reqwest`, `log`; optional `quick-xml` (`xml`)  
**Storage**: N/A (session id in memory: JSON header / SOAP cookie jar)  
**Testing**: `cargo test -p vim_rs`, `cargo test -p vim_rs --all-features`; new wire-stub tests for `close` + `current_thread` Drop; existing Drop-logout tests remain on multi-thread  
**Target Platform**: Cross-platform library (Linux/macOS/Windows)  
**Project Type**: Rust library (`vim_rs`) + examples workspace  
**Performance Goals**: No material Principle III regression (small hand-written client change)  
**Constraints**: No generated `types/` / `mo/` edits; no consuming `close(self)`; no `logout-on-drop` feature; blocking Drop only on multi-thread as documented 0.6.x exception  
**Scale/Scope**: `JsonClient` / `SoapClient` / `Client` / `VimClient`; docs; ~16 snippet bins + `examples/vtui`; existing wire-log logout tests

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle / standard | Status | Evidence |
|---------------------|--------|----------|
| **I. Generated from specs** | Pass | Hand-written `core/client.rs`, `xml/client.rs`, docs, examples only |
| **II. Complete type-safe surface** | Pass | Additive `close`; trait method has default body (FR-002) |
| **III. Build-time & binary budgets** | Pass | No generated expansion; measure at release if shipping 0.6.1 |
| **IV. Multi-transport parity** | Pass | JSON + SOAP both implement logout via `close` and Drop fallback; `--all-features` tests |
| **V. Documented public surfaces** | Pass | rustdoc, CHANGELOG Added/Deprecated, README, snippets, [contracts/](./contracts/) |
| **VI. Readable source** | Pass | Shared async logout helper per transport; Drop only chooses runtime policy |
| **VII. Ecosystem tooling** | Pass | Macros/MCP unchanged; examples/docs coordinated |
| **Async-first I/O** | **Justified exception** | `close` is async. Destructor logout keeps `block_in_place` **only** on multi-thread (FR-005 / NFR-002). 0.7.0 removes it. See Complexity Tracking. |
| **Marshalling (miniserde)** | Pass | Unchanged |
| **Security & secrets** | Pass | SessionManager wire denylist applies to `close` (FR-012) |
| **Feature flags** | Pass | No new feature (NFR-001) |
| **Errors** | Pass | Logout failures remain `Error` (`ReqwestError` / `MethodFault`); close still marks session ended locally |

**Post-design re-check**: [research.md](./research.md), [data-model.md](./data-model.md), [contracts/](./contracts/) match the gates. Exception remains only the 0.6.x multi-thread Drop path.

## Project Structure

### Documentation (this feature)

```text
specs/004-client-close/
├── plan.md              # This file
├── research.md          # Phase 0
├── data-model.md        # Phase 1
├── quickstart.md        # Phase 1
├── contracts/           # Phase 1
├── spec.md
├── checklists/requirements.md
└── tasks.md             # /speckit-tasks (not created by /speckit-plan)
```

### Source Code (repository root)

```text
vim_rs/src/core/client.rs          # VimClient::close default; Client::close; JsonClient close + Drop policy
vim_rs/src/xml/client.rs           # SoapClient close + Drop policy + session_ended flag
vim_rs/src/core/tasks.rs           # rustdoc shutdown-then-close (no API change)
vim_rs/src/core/pc_cache.rs        # rustdoc destroy-then-close (no API change)
vim_rs/docs/CLIENT.md              # §5 Drop vs close
vim_rs/src/lib.rs                  # crate-level connect sample
vim_rs/tests/...                   # wire-log close + current_thread Drop
examples/snippets/src/*.rs         # close() at shutdown; remove yield-for-drop sleeps
examples/vtui/src/main.rs          # destroy() then close()
CHANGELOG.md
README.md
```

**Structure Decision**: Lifecycle lives on the crate-private transports and the public `Client` / `VimClient` surface. Helpers (`TaskTracker`, `CacheManager`) get documentation only.

## Complexity Tracking

| Violation | Why needed | Simpler alternative rejected because |
|-----------|------------|-------------------------------------|
| Blocking I/O in `Drop` on the multi-worker runtime (constitution async-first) | FR-005: 0.6.0 consumers who never call `close` must still log out; 0.5/0.6 already broke compile | Remove Drop logout in 0.6.1 — behavioral break the spec forbids. Cargo feature for Drop logout — dual semantics, rejected (NFR-001). |

---

## Phase 0: Research

**Status**: Complete → [research.md](./research.md)

| ID | Topic | Outcome |
|----|-------|---------|
| R1 | Trait vs inherent `close` | Both: default no-op on `VimClient`; inherent `Client::close`; `Client` trait impl **must** delegate (vtui holds `Arc<dyn VimClient>`) |
| R2 | Claiming logout | `AtomicBool session_ended`; CAS then logout while credentials still present; then clear JSON key |
| R3 | Concurrent close | Loser of CAS returns `Ok(())` immediately (at most one logout) |
| R4 | Drop runtime policy | `Handle::try_current` + `RuntimeFlavor`; multi-thread keeps `block_in_place`; current_thread / no runtime → warn |
| R5 | Shared logout HTTP | Extract today’s Drop HTTP into async `logout_session` used by `close` and Drop |
| R6 | Tests | Keep Drop wire tests; add close wire tests; add `flavor = "current_thread"` no-panic Drop test |
| R7 | Examples | All snippet connect paths call `close`; remove sleeps in `vm_rename` / `vm_toggle_wol`; in-tree vtui after `destroy` |
| R8 | 0.7.0 | Same `close` signature; Drop becomes warn-only; optional tokio `rt-multi-thread` trim |

No open **NEEDS CLARIFICATION** items.

---

## Phase 1: Design

**Status**: Complete → [data-model.md](./data-model.md), [contracts/README.md](./contracts/README.md), [quickstart.md](./quickstart.md)

### `VimClient` (additive, object-safe)

```rust
fn close(&self) -> BoxFuture<'_, Result<()>> {
    Box::pin(async { Ok(()) })
}
```

### `Client`

- Inherent `pub async fn close(&self) -> Result<()>` → `self.inner.close().await`
- `impl VimClient for Client`: override `close` to forward to `inner` (**not** the default)

### Transports

- `JsonClient`: `session_ended: AtomicBool`; `close` CAS + existing JSON Logout HTTP + `session_key` take
- `SoapClient`: `session_ended: AtomicBool`; `close` CAS + existing SOAP Logout HTTP
- `Drop`: if `session_ended` or no session → return; else flavor policy (R4)

### Tests / docs

See [quickstart.md](./quickstart.md). Changelog: **Added** `close`, **Deprecated** destructor logout (remove in 0.7.0) — no Breaking.

**Post-design constitution re-check**: Pass, with the same Drop exception in Complexity Tracking.

---

## Phase 2: Implementation tasks (preview for `/speckit-tasks`)

Not created by this command. Suggested groups:

1. **Core API** — trait default, `Client` inherent + delegate, JSON/SOAP `close` + `session_ended`, Drop policy, extract logout HTTP
2. **Tests** — close wire logs; close-then-drop no second logout; current_thread Drop no panic; existing Drop tests still pass
3. **Docs** — rustdoc, `CLIENT.md`, README, CHANGELOG, TaskTracker / CacheManager shutdown order
4. **Examples** — snippets + in-tree vtui
5. **Release** — `vim_rs` 0.6.1 version bump when shipping (macros unchanged unless needed)

---

## Risk & mitigation

| Risk | Mitigation |
|------|------------|
| `Client` forgets to override trait `close` | vtui/`VimClientHandle` would no-op; contract + unit test on facade |
| Double logout (close + Drop) | CAS `session_ended` before I/O; Drop checks flag first |
| `close` during `WaitForUpdatesEx` | Documented; wait fails; no auto-shutdown of TaskTracker in 0.6.1 |
| Logout fails then Drop retries | Mark ended before/at start of close I/O; return the error |
| current_thread tests need Tokio `rt` | Already pulled in via `rt-multi-thread` |

## Acceptance mapping (spec → plan)

| Criterion | Plan element |
|-----------|----------------|
| SC-001 | `close` while clones live; wire stub shows one logout; second close no HTTP |
| SC-002 | Drop wire tests unchanged on `#[tokio::test]` (multi-thread) |
| SC-003 | `#[tokio::test(flavor = "current_thread")]` Drop without panic + warn |
| SC-004 | `MockVimClient` unchanged; default trait method |
| SC-005 | Snippet grep: `close(` present; yield-for-drop sleeps gone |
| SC-006 | CHANGELOG Added/Deprecated only |
| SC-007 | [contracts/](./contracts/) freeze `close(&self)` for 0.7.0 Drop-only change |
