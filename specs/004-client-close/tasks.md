# Tasks: Explicit Client Session Close (0.6.1)

**Input**: Design documents from `/specs/004-client-close/`  
**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/README.md](./contracts/README.md), [quickstart.md](./quickstart.md)

**Tests**: Included. Spec Independent Tests / SC-001–SC-004 and plan R6 require wire-stub and `current_thread` Drop tests (not a generated-API feature).

**Organization**: Tasks grouped by user story (P1 → P3) with shared foundation first.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no blocking deps on incomplete tasks)
- **[Story]**: US1, US2, US3, US4 per [spec.md](./spec.md)

## Path Conventions

- Library: `vim_rs/src/core/client.rs`, `vim_rs/src/xml/client.rs`
- Helpers (docs only): `vim_rs/src/core/tasks.rs`, `vim_rs/src/core/pc_cache.rs`
- Examples: `examples/snippets/src/`, `examples/vtui/src/main.rs`
- Docs: `CHANGELOG.md`, `README.md`, `vim_rs/docs/CLIENT.md`, `vim_rs/src/lib.rs`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Confirm 0.6.1 compatible landing; no new Cargo feature.

- [ ] T001 Review [contracts/README.md](./contracts/README.md) against current `Drop` in `vim_rs/src/core/client.rs` and `vim_rs/src/xml/client.rs`
- [ ] T002 Confirm `vim_rs/Cargo.toml` stays without a logout-on-drop feature and keeps `tokio` `rt-multi-thread` (needed for 0.6.x Drop fallback)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared session-ended flag and extracted async logout HTTP. **Blocks all user stories.** Existing Drop behavior must remain `block_in_place` until US3.

**⚠️ CRITICAL**: No user story work until this phase completes.

- [ ] T003 Add `session_ended: AtomicBool` to `JsonClient` and every `JsonClient { ... }` construction site in `vim_rs/src/core/client.rs` (`build_json` bootstrap/json, `test_json_client_wire_transport`, `test_json_client_http_origin`)
- [ ] T004 [P] Add `session_ended: AtomicBool` to `SoapClient`, `SoapClient::new`, and `soap_test_client_for_logout_drop` in `vim_rs/src/xml/client.rs`
- [ ] T005 Extract `async fn logout_session` from JSON `Drop` HTTP in `vim_rs/src/core/client.rs`; `Drop` still uses `block_in_place` + `block_on(logout_session)` (0.6.0 behavior)
- [ ] T006 [P] Extract `async fn logout_session` from SOAP `Drop` HTTP in `vim_rs/src/xml/client.rs`; `Drop` still uses `block_in_place` + `block_on(logout_session)`
- [ ] T007 Run `cargo check -p vim_rs --all-features` after T003–T006 in `vim_rs/src/core/client.rs` and `vim_rs/src/xml/client.rs`

**Checkpoint**: Crate compiles; logout HTTP is one async function per transport; Drop still blocks on multi-thread.

---

## Phase 3: User Story 1 — Explicit close while handles are shared (Priority: P1) 🎯 MVP

**Goal**: Non-consuming `close(&self)` ends the session with clones still live; second close / later Drop send no second Logout. `VimClient` default no-op keeps mocks compiling.

**Independent Test**: Wire stub: clone handle, `close()`, one Logout; `close()` again and drop remaining clones → no second Logout. `MockVimClient` still compiles without a `close` method.

### Tests for User Story 1

> Write these first; they fail until T011–T014 land.

- [ ] T008 [US1] Add JSON `close` wire tests (HTTP success, non-success, transport error) in `wire_logging_transport_tests` in `vim_rs/src/core/client.rs`
- [ ] T009 [US1] Add SOAP `close` wire tests (`#[cfg(feature = "xml")]`) in `wire_logging_transport_tests` in `vim_rs/src/core/client.rs`
- [ ] T010 [US1] Add tests for close-twice, close with an extra live `Arc` clone, and close-then-drop (exactly one Logout) in `vim_rs/src/core/client.rs`

### Implementation for User Story 1

- [ ] T011 [US1] Add `VimClient::close` default no-op (`Box::pin(async { Ok(()) })`) plus rustdoc in `vim_rs/src/core/client.rs`
- [ ] T012 [US1] Add inherent `Client::close(&self)` and override `impl VimClient for Client` to forward to `inner` (must not use the default) in `vim_rs/src/core/client.rs`
- [ ] T013 [US1] Implement `JsonClient` `close`: CAS `session_ended`, `logout_session` while the session key is still set, then `take()` the key; `Drop` returns immediately if `session_ended` in `vim_rs/src/core/client.rs`
- [ ] T014 [P] [US1] Implement `SoapClient` `close`: CAS `session_ended`, `logout_session`; `Drop` returns immediately if `session_ended` in `vim_rs/src/xml/client.rs`
- [ ] T015 [US1] Run `cargo test -p vim_rs` (and `--features xml` / `--all-features` for SOAP close tests); confirm `vim_rs/tests/support/mock_vim_client.rs` needs no `close` method (SC-001, SC-004)

**Checkpoint**: `close` works on `Arc<Client>` and `Arc<dyn VimClient>`; idempotent; mocks compile.

---

## Phase 4: User Story 2 — Multi-thread Drop logout unchanged (Priority: P1)

**Goal**: Programs that never call `close` still Logout when the last inner handle is dropped on the default multi-worker runtime (0.6.0 parity). After `close`, Drop is silent.

**Independent Test**: Existing `json_drop_logout_*` / `soap_drop_logout_*` tests still pass on `#[tokio::test]` (multi-thread). Close-then-drop from US1 still shows one Logout.

### Implementation for User Story 2

- [ ] T016 [US2] Confirm `JsonClient` `Drop` still `block_in_place` + `logout_session` when `!session_ended` and a session exists in `vim_rs/src/core/client.rs`
- [ ] T017 [P] [US2] Confirm `SoapClient` `Drop` still `block_in_place` + `logout_session` when `!session_ended` and a session exists in `vim_rs/src/xml/client.rs`
- [ ] T018 [US2] Run `json_drop_logout_emits_wire_lines_on_http_success`, non-success, transport, and SOAP equivalents in `vim_rs/src/core/client.rs` (SC-002)
- [ ] T019 [US2] Document destructor logout as a 0.6.x compatibility fallback to be removed in 0.7.0 on `Client` and `VimClient` rustdoc in `vim_rs/src/core/client.rs`

**Checkpoint**: Unchanged multi-thread Drop logout; rustdoc points at `close`.

---

## Phase 5: User Story 3 — Single-thread runtime Drop does not panic (Priority: P2)

**Goal**: On `current_thread` (and when no Tokio handle exists), `Drop` must not call `block_in_place`. Warn if the session is still live; silent if `close` already ran.

**Independent Test**: `#[tokio::test(flavor = "current_thread")]` drops a logged-in client without panic; after `close`, Drop is silent.

### Tests for User Story 3

- [ ] T020 [US3] Add `#[tokio::test(flavor = "current_thread")]` Drop-without-close does not panic (and emits warn if practical) in `vim_rs/src/core/client.rs`
- [ ] T021 [US3] Add `current_thread` close-then-drop silent test (JSON; SOAP if `xml`) in `vim_rs/src/core/client.rs`

### Implementation for User Story 3

- [ ] T022 [US3] Add `pub(crate)` Drop runtime helper (`Handle::try_current`, `RuntimeFlavor::MultiThread` vs `CurrentThread` vs none) in `vim_rs/src/core/client.rs`
- [ ] T023 [US3] Switch `JsonClient` `Drop` to the helper: multi-thread claims `session_ended` and `block_on(logout_session)`; otherwise `warn!` and return in `vim_rs/src/core/client.rs`
- [ ] T024 [P] [US3] Switch `SoapClient` `Drop` to the same helper in `vim_rs/src/xml/client.rs`
- [ ] T025 [US3] Run the `current_thread` tests in `vim_rs/src/core/client.rs` (SC-003); re-run US2 Drop tests to confirm multi-thread still logs out

**Checkpoint**: `current_thread` Drop does not panic; multi-thread Drop logout still works.

---

## Phase 6: User Story 4 — Samples and docs teach close (Priority: P3)

**Goal**: Snippets, README, crate rustdoc, and helper docs show `close` at shutdown. Yield-for-drop sleeps gone. CHANGELOG is Added/Deprecated, not Breaking.

**Independent Test**: Snippet mains call `close`; `rg "Yield to run async drop" examples/snippets` is empty; CHANGELOG has no Breaking section for this work.

### Implementation for User Story 4

- [ ] T026 [P] [US4] Rewrite Drop/logout section to `close` + 0.6.x fallback in `vim_rs/docs/CLIENT.md`
- [ ] T027 [P] [US4] Add `client.close().await?` to the connect sample in `README.md`
- [ ] T028 [P] [US4] Add `close` to the crate-level sample in `vim_rs/src/lib.rs`
- [ ] T029 [P] [US4] Add `[Unreleased]` **Added** `close` and **Deprecated** destructor logout (remove in 0.7.0) in `CHANGELOG.md` — no Breaking (SC-006)
- [ ] T030 [P] [US4] Document shutdown-then-close (no new API) in `vim_rs/src/core/tasks.rs`
- [ ] T031 [P] [US4] Document destroy/cancel-wait-then-close (no new API) in `vim_rs/src/core/pc_cache.rs`
- [ ] T032 [US4] Call `client.close().await?` and remove yield-for-drop `sleep` in `examples/snippets/src/vm_rename.rs`
- [ ] T033 [P] [US4] Call `client.close().await?` and remove yield-for-drop `sleep` in `examples/snippets/src/vm_toggle_wol.rs`
- [ ] T034 [US4] Call `client.close().await?` on shutdown in remaining snippet binaries under `examples/snippets/src/`: `dynamic_property_fetch.rs`, `env_browser.rs`, `eventster.rs`, `inventory_path.rs`, `mac_monitor.rs`, `perf_metrics.rs`, `print_vm_addresses.rs`, `property_collector.rs`, `retrieve_ds_hosts.rs`, `retrieve_host_info.rs`, `retrieve_recent_task.rs`, `root_objects.rs`, `vm_disabled_method_len.rs`, `vm_events.rs`, `vm_ip.rs`
- [ ] T035 [P] [US4] After `cache_manager.destroy()`, call `client.close().await?` in `examples/vtui/src/main.rs`
- [ ] T036 [US4] Confirm `rg "Yield to run async drop" examples/snippets` is empty and snippet mains call `close` (SC-005)

**Checkpoint**: Docs and in-tree examples teach `close`; sleeps gone.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Full test matrix and quickstart DoD. Version bump is a release step, not required in the implementation PR.

- [ ] T037 Run `cargo test -p vim_rs` from repo root
- [ ] T038 Run `cargo test -p vim_rs --all-features` from repo root
- [ ] T039 Walk definition of done in `specs/004-client-close/quickstart.md`
- [ ] T040 Leave `vim_rs` version at `0.6.0` in `vim_rs/Cargo.toml` unless this PR is the 0.6.1 release; keep the 0.6.1 note in `CHANGELOG.md` only

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies
- **Foundational (Phase 2)**: Depends on Setup — **BLOCKS** all user stories
- **US1 (Phase 3)**: Depends on Foundational — MVP
- **US2 (Phase 4)**: Depends on US1 (`session_ended` skip path); verifies Drop fallback
- **US3 (Phase 5)**: Depends on US2 (must not remove multi-thread Drop while adding flavor checks)
- **US4 (Phase 6)**: Depends on US1 API existing; can overlap late US2/US3 docs except `client.rs` rustdoc (T019)
- **Polish (Phase 7)**: After desired stories

### User Story Dependencies

- **US1 (P1)**: After Phase 2 — no other stories
- **US2 (P1)**: After US1 — Drop skip-if-ended comes from US1
- **US3 (P2)**: After US2 — flavor helper must preserve multi-thread logout
- **US4 (P3)**: After US1 (`close` must exist); changelog/rustdoc should wait until Drop policy (US2/US3) is accurate

### Within Each User Story

- Tests (US1, US3) before the implementation that makes them pass
- JSON `client.rs` work is sequential (same file)
- SOAP `xml/client.rs` can run in parallel with JSON when marked [P]

### Parallel Opportunities

- T003 ∥ T004 (JSON vs SOAP struct field)
- T005 ∥ T006 (extract logout_session)
- T013 ∥ T014 (close impl)
- T016 ∥ T017 (Drop confirm)
- T023 ∥ T024 after T022 (Drop helper consumers)
- T026–T031, T033, T035 docs/examples on different files

---

## Parallel Example: User Story 1

```bash
# After Phase 2, SOAP close can proceed beside JSON close:
Task: "T013 [US1] JsonClient close + Drop skip-if-ended in vim_rs/src/core/client.rs"
Task: "T014 [US1] SoapClient close + Drop skip-if-ended in vim_rs/src/xml/client.rs"
```

## Parallel Example: User Story 4

```bash
Task: "T026 [US4] vim_rs/docs/CLIENT.md"
Task: "T027 [US4] README.md"
Task: "T028 [US4] vim_rs/src/lib.rs"
Task: "T029 [US4] CHANGELOG.md"
Task: "T030 [US4] vim_rs/src/core/tasks.rs"
Task: "T031 [US4] vim_rs/src/core/pc_cache.rs"
Task: "T035 [US4] examples/vtui/src/main.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1 Setup
2. Phase 2 Foundational
3. Phase 3 US1 (`close` + idempotency tests)
4. **STOP and VALIDATE** US1 Independent Test
5. Then US2 (compat Drop) before calling 0.6.1 done for default-runtime users

### Incremental Delivery

1. Setup + Foundational → shared logout HTTP
2. US1 → demo `close` with clones (MVP)
3. US2 → 0.6.0 Drop parity on multi-thread
4. US3 → `current_thread` no panic
5. US4 → samples/docs; then Polish tests

### Parallel Team Strategy

1. Together: Setup + Foundational
2. After foundation: one person JSON `client.rs` (US1–US3 sequential on that file), another SOAP `xml/client.rs` [P] tasks, a third US4 docs/examples (after T011–T012 exist)

---

## Notes

- [P] = different files, no incomplete deps
- Do not edit generated `vim_rs/src/types/` or `vim_rs/src/mo/`
- Standalone `vtui` repo is out of scope (in-tree `examples/vtui` only)
- 0.7.0 Drop removal is **not** a task here (FR-013 design constraint only)
- Commit after each task or logical group
- Format: checkbox, Task ID, optional `[P]`, story label on US phases, file path in every description
