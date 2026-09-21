# Feature Specification: Explicit Client Session Close (0.6.1)

**Feature Branch**: `004-client-close`

**Created**: 2026-09-21

**Status**: Draft

**Input**: User description: "the `fn close(&self)` as discussed for version 0.6.1 with back compat that allows removal of the Drop block_in_place behavior in a future 0.7.0."

## Context & Problem

`vim_rs` today ends a vSphere session only when the last strong handle to the inner transport client is released. That implicit end performs blocking work on the async runtime. That works for the default multi-worker runtime used by existing samples, but it:

- Crashes on a single-threaded async runtime (the blocking helper is not allowed there).
- Fights shared ownership: applications clone the session handle into helpers and background waiters, so unique ownership at shutdown is not realistic.
- Cannot be marked deprecated as an operation — it is destructor behavior, not a callable API.

Applications such as `vtui` and the snippet binaries need a first-class, non-consuming way to end the remote session while other handles still exist. Existing 0.6.0 consumers must keep compiling and must still get implicit logout on the multi-worker runtime they already use. A future 0.7.0 can then drop the blocking destructor path without another surprise for callers who already switched to explicit close.

This feature is the 0.6.1 compatible landing: add explicit close, keep implicit logout as a documented fallback, and deprecate that fallback in writing so 0.7.0 can remove it.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Explicitly end a session while handles are still shared (Priority: P1)

An application holds several clones of the session handle (UI, cache manager, managed-object stubs, background workers). When the user quits, the application asks the session to end. Logout runs even though clones still exist. Later, releasing those clones does not attempt a second logout.

**Why this priority**: This is the API that makes shared handles and graceful shutdown work. Without it, 0.7.0 cannot remove destructor logout without leaking sessions for every typical consumer.

**Independent Test**: Connect, clone the handle into a helper that stays alive, call close on the original handle, confirm the server session is gone and a second close is a no-op (or a non-fatal already-ended result). Dropping remaining clones must not issue another logout.

**Acceptance Scenarios**:

1. **Given** a logged-in session and at least one extra live clone of the handle, **When** the application calls close on any handle, **Then** the vSphere session is terminated (VI/JSON and SOAP/XML).
2. **Given** a session that has already been closed, **When** close is called again or the last handle is released, **Then** no second logout is sent to the server.
3. **Given** a custom/mock implementation of the client trait that does not override close, **When** that project is compiled against 0.6.1, **Then** it compiles without source changes (close has a default no-op).

---

### User Story 2 - Existing 0.6.0 apps keep implicit logout on the multi-worker runtime (Priority: P1)

A consumer who does not call close, running on the default multi-worker async runtime (today’s snippets and most services), still gets a server logout when the last handle is released. No source change is required to keep that behavior in 0.6.1.

**Why this priority**: 0.5.0 and 0.6.0 were already breaking minors. 0.6.1 must not force another compile or behavior break on the path the majority already uses.

**Independent Test**: Run an existing snippet that never calls close on the default runtime; when the process drops the last handle, a logout request is still observed (same as 0.6.0). The crate version bump is 0.6.1 (compatible), not 0.7.0.

**Acceptance Scenarios**:

1. **Given** an unchanged 0.6.0-style program on the default multi-worker runtime that never calls close, **When** the last session handle is released, **Then** a logout is still sent, matching 0.6.0.
2. **Given** a program that called close successfully, **When** the last handle is later released on that same runtime, **Then** destructor logout is skipped (session already ended).
3. **Given** the 0.6.1 changelog and rustdoc, **When** a consumer reads them, **Then** destructor logout is documented as a compatibility fallback that will be removed in 0.7.0, and explicit close is the recommended API.

---

### User Story 3 - Single-threaded async runtimes no longer panic at session end (Priority: P2)

An application uses a single-threaded async runtime (one worker; blocking the event loop is forbidden). In 0.6.0, releasing the last handle panics because destructor logout blocks the runtime. In 0.6.1, that path does not panic: if close was not called, the destructor logs a warning that the session was not ended and leaves idle timeout to the server. If close was called, the destructor is silent.

**Why this priority**: Unblocking this runtime is the reason to add close now; it must work in 0.6.1 without waiting for 0.7.0. It is P2 because the default runtime path (Story 2) must stay intact first.

**Independent Test**: Build and drop a logged-in client on a single-threaded runtime without calling close; the process must not panic. After calling close first, drop is silent.

**Acceptance Scenarios**:

1. **Given** a logged-in session on a single-threaded async runtime, **When** the last handle is released without a prior close, **Then** the process does not panic, and a warning is emitted that close was not called.
2. **Given** the same runtime after a successful close, **When** remaining handles are released, **Then** no warning is emitted and no logout is attempted.
3. **Given** that runtime, **When** the application calls close before releasing handles, **Then** the server session ends and the process exits cleanly.

---

### User Story 4 - Samples and docs teach close before 0.7.0 (Priority: P3)

Snippet programs, crate README, and client rustdoc show explicit close at shutdown. The old “sleep so destructor cleanup can run” pattern is removed. Property-collector and task-tracker docs state the recommended order: stop waiters / destroy caches, then close the session. Destructor logout is described as deprecated fallback, not the primary contract.

**Why this priority**: Documentation is how 0.6.1 consumers migrate before 0.7.0 removes the fallback. The library API in Stories 1–3 is usable without samples, but Principle V requires runnable examples for a new public operation.

**Independent Test**: Read README, client rustdoc, and snippet mains: close is called; no yield-for-drop sleep remains; changelog lists Added close and Deprecated destructor logout, not a 0.6.1 Breaking section.

**Acceptance Scenarios**:

1. **Given** the snippet binaries that previously slept after work to “let async drop run”, **When** they are updated, **Then** they call close instead of sleeping.
2. **Given** README / crate rustdoc connect examples, **When** a new user copies them, **Then** they see close before process exit.
3. **Given** TaskTracker / CacheManager documentation, **When** a user looks up shutdown, **Then** they are told to stop waits and destroy filters/views before close, without new required methods on those helpers in 0.6.1.

---

### Edge Cases

- **Close with no session**: Handle was built but login never happened, or session key/cookie was never established. Close is a successful no-op (nothing to log out).
- **Close after failed logout**: Network or server error on logout. Close still marks the session ended locally so destructor logout is not retried; the error is returned to the caller so they can log it.
- **Concurrent close**: Two tasks call close on clones at the same time. At most one logout is sent; the other sees already-ended / joins the in-flight end. No panic, no double logout.
- **Close during a long poll**: A PropertyCollector wait or TaskTracker loop is in flight. Close may cause that wait to fail with an authentication/transport error; that is expected. 0.6.1 does not auto-shut down TaskTracker or CacheManager.
- **Mock / test clients**: Implementations that only satisfy the existing trait methods keep compiling; default close does nothing. Tests that asserted destructor logout still pass on the multi-worker runtime until 0.7.0.
- **SOAP vs JSON**: Both transports end the session (SOAP Logout vs JSON SessionManager Logout). Wire-capture denylist for SessionManager still applies on the close path.
- **No cargo feature**: There is no `logout-on-drop` (or similar) feature flag. Compatibility is the 0.6.1 fallback destructor on the multi-worker runtime, not a feature matrix.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The public session facade (`Client`) MUST expose a non-consuming asynchronous `close` operation that ends the current vSphere session. Callers MUST be able to invoke it while other clones of the handle still exist.
- **FR-002**: The object-safe client trait (`VimClient`) MUST expose the same `close` operation with a **default no-op implementation**, so existing third-party or test implementations keep compiling in 0.6.1. JSON and SOAP transport implementations MUST override it with real logout.
- **FR-003**: `close` MUST be idempotent: after a successful or locally completed close, further `close` calls MUST NOT send another logout. Releasing the last handle MUST NOT send another logout either.
- **FR-004**: `close` MUST work for both VI/JSON and SOAP/XML transports (when the SOAP feature is enabled), using each transport’s existing SessionManager logout mechanism.
- **FR-005**: On the **default multi-worker async runtime**, if `close` was never called and a session is still live, releasing the last inner transport handle MUST still send logout, preserving 0.6.0 behavior.
- **FR-006**: On a **single-threaded async runtime**, releasing the last handle MUST NOT use the blocking-on-runtime helper that panics today. If a session is still live, the destructor MUST emit a warning urging `close` and MUST NOT panic.
- **FR-007**: After `close` has marked the session ended, destructors on both runtimes MUST be silent no-ops with respect to logout.
- **FR-008**: 0.6.1 MUST be a compatible release (patch/minor on the 0.6 line): no required trait methods without defaults, no removal of destructor logout on the multi-worker path, no new Cargo feature for this behavior, and changelog categorization MUST be Added / Deprecated — not Breaking.
- **FR-009**: Rustdoc on `Client` and `VimClient` MUST recommend `close` as the supported way to end a session and MUST state that destructor logout is a 0.6.x compatibility fallback scheduled for removal in 0.7.0.
- **FR-010**: In-tree snippet examples and crate README connect samples MUST call `close` at shutdown. Comments/sleeps whose only purpose was to yield for destructor logout MUST be removed.
- **FR-011**: Documentation for TaskTracker and CacheManager/Monitor MUST describe the recommended shutdown order (stop waiters / destroy filters and views, then `close`) without requiring new APIs on those types in 0.6.1.
- **FR-012**: SessionManager traffic triggered by `close` MUST obey the existing wire-logging denylist (no logout bodies even in detailed mode).
- **FR-013**: A future **0.7.0** (out of implementation scope for this feature, but a required design constraint) MUST be able to remove destructor logout entirely — leaving only a warning if a session is still live — without changing the `close` signature introduced in 0.6.1. 0.6.1 MUST NOT introduce a Cargo feature or second close API that 0.7.0 would have to carry.

### Non-Functional / Constraints

- **NFR-001**: No new optional Cargo feature for logout-on-drop or close. Optional features remain for compile-time/binary cost (`defaults`, `xml`, `vcsim_compat`, `default-client`).
- **NFR-002**: Blocking I/O on the runtime remains a documented 0.6.x exception **only** for the multi-worker destructor fallback (FR-005). `close` itself MUST be async. 0.7.0 is the release that aligns destructor behavior with the constitution’s async-first rule.
- **NFR-003**: `close` MUST NOT require unique ownership of the handle (no consuming `close(self)` / `try_unwrap` as the public contract).
- **NFR-004**: Build-time and binary-size budgets (constitution III) MUST not regress materially; this is a small hand-written client-lifecycle addition, not a generated-surface change.
- **NFR-005**: Public behavior MUST NOT diverge between JSON and SOAP except where logout mechanics already differ (header session vs cookie session); both MUST honor FR-001–FR-007.

### Key Entities

- **Session handle**: Shared, reference-counted client a consumer clones into stubs and helpers. Close is invoked on this handle without taking ownership.
- **Remote session**: Server-side vSphere login (JSON session id or SOAP cookie). Ended by `close` or, in 0.6.1 only, by last-handle release on the multi-worker runtime.
- **Session-ended flag**: Local state shared across clones so logout runs at most once.
- **Compatibility fallback**: 0.6.x destructor logout on the multi-worker runtime; deprecated in docs; removed in 0.7.0.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A program with extra live clones can end the remote session via close; a subsequent server session listing (or equivalent test double) shows that session gone, and a second close does not send another logout.
- **SC-002**: Unchanged 0.6.0-style snippet code, run on the default multi-worker runtime without calling close, still produces a logout when the last handle is released (parity with 0.6.0).
- **SC-003**: The same logged-in client, dropped on a single-threaded runtime without close, does **not** panic; a warning is observable. After close, drop is silent.
- **SC-004**: A crate that implements the client trait with only the pre-0.6.1 methods **compiles against 0.6.1** without adding a close method.
- **SC-005**: 100% of in-tree snippet binaries that connect to vSphere call close (or document why they cannot); zero binaries retain a sleep whose only purpose is destructor logout.
- **SC-006**: The 0.6.1 changelog has no Breaking section for this work; close is under Added; destructor logout is under Deprecated with an explicit 0.7.0 removal note.
- **SC-007**: A maintainer can remove destructor logout in 0.7.0 without changing the close signature or introducing a feature flag — verifiable by this spec’s FR-013 and the rustdoc deprecation text shipped in 0.6.1.

## Assumptions

- Target release for this work is **vim_rs 0.6.1** (compatible). **0.7.0** is a later breaking minor that only removes the destructor fallback and may drop the multi-worker-only runtime dependency that fallback needs.
- Consumers who ignore close in 0.6.1 on the default runtime keep today’s logout-on-last-drop; consumers on a single-threaded runtime must call close or accept a leaked session until server idle timeout.
- A leftover session after a missed close is an operational inconvenience (session list, session limits), not a process-safety failure. Warning plus idle timeout is the accepted safety net on the single-threaded path.
- `TaskTracker::shutdown` remaining fire-and-forget (not a joined wait) is acceptable for 0.6.1; races between close and in-flight wait teardown may log destroy/filter errors. Joining shutdown is a possible 0.7.0 or follow-up improvement, not required here.
- Standalone `vtui` (separate repository) is a downstream consumer: 0.6.1 enables it to call close; updating that repo is not a merge gate for this spec. In-tree `examples/snippets` and `examples/vtui` are in scope for FR-010.
- Custom `VimClient` implementors are rare (tests/mocks); a default no-op is sufficient compatibility. JSON/SOAP built-in clients always override.
- Wire-logging tests that cover destructor logout remain valid on the multi-worker runtime; they should also cover the close path.
- No OpenAPI / generator changes: SessionManager Logout already exists.

## Out of Scope

- Removing destructor logout or the blocking-on-runtime helper from the multi-worker path (that is **0.7.0**).
- A consuming `close(self)` or `Arc::try_unwrap`-based close as the public API.
- A Cargo feature to toggle logout-on-drop.
- Making the client trait non-thread-safe (`!Send` / `Rc`) or switching the crate to local-only tasks. Single-threaded runtime support here means “do not panic on last drop / allow explicit close,” not a `!Send` rewrite.
- New TaskTracker, CacheManager, ObjectRetriever, or Monitor APIs (including making `shutdown` wait for the background loop).
- Changing session idle timeout policy on the server.
- Updating the standalone `vtui` repository (documented as a downstream follow-up).
