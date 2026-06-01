# Tasks: Selectable TLS backend / opt-out of default client configuration

**Input**: Design documents from `/specs/002-selectable-tls-backend/`  
**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/README.md](./contracts/README.md), [quickstart.md](./quickstart.md)

**Tests**: Not requested in spec — verification via `cargo test`, `cargo tree`, and optional `examples/tls_rustls_only` crate (SC-001/SC-006).

**Organization**: Tasks grouped by user story (P1 → P2) with shared foundation first.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no blocking deps on incomplete tasks)
- **[Story]**: US1, US2, US3 per [spec.md](./spec.md)

## Path Conventions

- Library crate: `vim_rs/`
- Examples workspace: `examples/`
- Root docs: `CHANGELOG.md`, `README.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Align manifest and design contracts before code changes.

- [x] T001 Review API contracts in `specs/002-selectable-tls-backend/contracts/README.md` against current `vim_rs/src/core/client.rs`
- [x] T002 Update `vim_rs/Cargo.toml`: `reqwest = "0.13"`, `default-features = false`, base features `charset`/`http2`/`system-proxy`; add `default = ["default-client"]`, `default-client = ["reqwest/default"]`; set `xml = ["dep:quick-xml", "reqwest/cookies"]` (remove duplicate cookies from base)
- [x] T003 [P] Document target feature matrix in `specs/002-selectable-tls-backend/quickstart.md` if manifest naming differs from plan (keep quickstart in sync with final `Cargo.toml`)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: reqwest 0.13 upgrade, shared HTTP client builder, and `default-client` cfg infrastructure. **Blocks all user stories.**

**⚠️ CRITICAL**: No user story validation until this phase completes.

- [x] T004 Add `#[cfg(feature = "default-client")]` helper `build_default_http_client(insecure: Option<bool>, cookie_store: bool) -> Result<reqwest::Client>` in `vim_rs/src/core/client.rs` (use `tls_danger_accept_invalid_*` or existing aliases; apply `.cookie_store(true)` only when `cookie_store` is true)
- [x] T005 Replace duplicated inline `reqwest::ClientBuilder` blocks in `build_json`, `build_soap_facade`, and `build_auto_facade` in `vim_rs/src/core/client.rs` with calls to `build_default_http_client` (`cookie_store: false` for JSON and Hello probe; `cookie_store: true` for SOAP path only)
- [x] T006 Gate auto-created client behind `#[cfg(feature = "default-client")]` in `build_json`, `build_soap_facade`, and `build_auto_facade` in `vim_rs/src/core/client.rs` (when cfg off, use `self.http_client` from `new(server, client)` only)
- [x] T007 Implement feature-gated `ClientBuilder::new` in `vim_rs/src/core/client.rs`: `new(server)` with `default-client`; `new(server, http_client: reqwest::Client)` without `default-client`
- [x] T008 Add `#[cfg(feature = "default-client")]` to `ClientBuilder::insecure` and `ClientBuilder::http_client` in `vim_rs/src/core/client.rs`
- [x] T009 Run `RUSTUP_HOME=~/.rustup CARGO_HOME=~/.cargo cargo check -p vim_rs` and `cargo check -p vim_rs --all-features`; fix reqwest 0.13 API breakages in `vim_rs/src/core/client.rs` and `vim_rs/src/xml/client.rs` if any
- [x] T010 Update `vim_rs/Cargo.lock` after manifest and dependency resolution

**Checkpoint**: `vim_rs` compiles with new features; single `build_default_http_client`; SOAP-only `cookie_store`.

---

## Phase 3: User Story 1 — OpenSSL-free / opt-out build (Priority: P1) 🎯 MVP

**Goal**: Consumers can disable `default-client` and supply their own `reqwest::Client` at `new`, with no forced TLS/OpenSSL from `vim_rs`.

**Independent Test**: `examples/tls_rustls_only` (or equivalent) builds with `vim_rs` `default-features = false`; `cargo tree -i openssl-sys` is empty; VI/JSON connects with header session auth without `xml` feature.

### Implementation for User Story 1

- [x] T011 [US1] Ensure `build()` paths in `vim_rs/src/core/client.rs` never call `build_default_http_client` when `default-client` is disabled (client always taken from `new(server, http_client)`)
- [x] T012 [P] [US1] Add `examples/tls_rustls_only/` workspace member: `Cargo.toml` with `vim_rs = { default-features = false }`, `reqwest` with `rustls` + base features, minimal `main.rs` using `ClientBuilder::new(host, http)` per `specs/002-selectable-tls-backend/quickstart.md`
- [x] T013 [P] [US1] Register `tls_rustls_only` in `examples/Cargo.toml` workspace `members`
- [x] T014 [US1] Run `cargo tree -i openssl-sys` in `examples/tls_rustls_only/` and confirm empty output (SC-001)
- [x] T015 [US1] Add opt-out consumer `Cargo.toml` snippet and feature table to `README.md` (FR-010)

**Checkpoint**: Opt-out consumer builds without `openssl-sys`; compile-time `new(server, client)` enforced.

---

## Phase 4: User Story 2 — Turnkey consumers unchanged (Priority: P1)

**Goal**: Default features preserve auto-build, `insecure(true)`, and unchanged examples/tests; document TLS backend change (native-tls → rustls).

**Independent Test**: `cargo test -p vim_rs`, `cargo test -p vim_rs --all-features`, `examples/snippets` and `examples/vtui` build without source edits.

### Implementation for User Story 2

- [x] T016 [US2] Verify turnkey `build_json` auto-creates client when `http_client` unset and `default-client` enabled in `vim_rs/src/core/client.rs`
- [x] T017 [US2] Verify `insecure(true)` still applies via `build_default_http_client` on turnkey path in `vim_rs/src/core/client.rs`
- [x] T018 [US2] Run `cargo test -p vim_rs` from repo root (default features)
- [x] T019 [US2] Run `cargo test -p vim_rs --all-features` from repo root
- [x] T020 [P] [US2] Run `cargo build` in `examples/` for `snippets` and `vtui` without modifying their sources (SC-002)
- [x] T021 [US2] Run `cargo tree -p vim_rs -i openssl-sys` on default features; confirm empty (SC-007 rustls default) and note in `CHANGELOG.md`

**Checkpoint**: All default-feature tests and examples pass; TLS change documented.

---

## Phase 5: User Story 3 — Deduplicated client creation (Priority: P2)

**Goal**: Exactly one default HTTP client construction helper; cookie store only on SOAP/XML auto-build.

**Independent Test**: Code review of `vim_rs/src/core/client.rs` — one helper, three call sites; JSON/auto probe use `cookie_store: false`; SOAP uses `true`.

### Implementation for User Story 3

- [x] T022 [US3] Audit `vim_rs/src/core/client.rs`: confirm no remaining inline `reqwest::ClientBuilder::new()` duplication outside `build_default_http_client` (SC-005)
- [x] T023 [US3] Audit test helpers in `vim_rs/src/core/client.rs` (`test_dead_port_http_client`, etc.): align `cookie_store` with `xml` cfg where `.cookie_store(true)` is used
- [x] T024 [US3] Confirm `cargo tree -p vim_rs --no-default-features` shows no `cookie_store`/`cookies` dep unless `xml` enabled (SC-003)

**Checkpoint**: FR-008 satisfied; JSON-only builds omit `reqwest/cookies`.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, lockfiles, release notes.

- [x] T025 [P] Add `[Unreleased]` entry to `CHANGELOG.md`: `default-client` feature, opt-out instructions, TLS default rustls (FR-012), `reqwest/cookies` on `xml` only
- [x] T026 [P] Expand `ClientBuilder` and feature-flag rustdoc in `vim_rs/src/core/client.rs` (opt-out `new`, SOAP cookie requirement, turnkey vs injected client)
- [x] T027 [P] Update `vim_rs/src/lib.rs` crate-level docs if `ClientBuilder` usage is referenced at crate root
- [x] T028 Update `examples/Cargo.lock` and `mcp/Cargo.lock` after reqwest 0.13 resolution
- [x] T029 Run full validation from `specs/002-selectable-tls-backend/quickstart.md` (test commands + tree checks)
- [x] T030 [P] Record `vim_rs` / `examples/vtui` build-time and binary-size notes in `CHANGELOG.md` if Principle III release gate is measured for this release (NFR-001)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)** → **Phase 2 (Foundational)** → **Phases 3–5 (User Stories)** → **Phase 6 (Polish)**
- **US1** and **US2** both depend on Phase 2; can proceed in parallel after T010
- **US3** is mostly satisfied by T004–T005; T022–T024 validate after US1/US2

### User Story Dependencies

| Story | Depends on | Can parallel with |
|-------|------------|-------------------|
| **US1** (P1) | Phase 2 complete | US2 after T010 |
| **US2** (P1) | Phase 2 complete | US1 after T010 |
| **US3** (P2) | T004–T005 (Phase 2) | US1/US2 validation |

### Within Each User Story

- Manifest (T002) before compile fixes (T009)
- Helper + dedupe (T004–T005) before cfg-gated `new` (T007)
- US1 verification crate (T012–T014) after opt-out `build()` wiring (T011)
- Docs (T025–T026) after behavior stable (T018–T021)

### Parallel Opportunities

- **Phase 1**: T003 ∥ T002 (after T001)
- **Phase 3**: T012 ∥ T013; T014 after both
- **Phase 4**: T020 ∥ T021 (after tests T018–T019)
- **Phase 6**: T025 ∥ T026 ∥ T027; T028 after T010

---

## Parallel Example: User Story 1

```bash
# After Phase 2 checkpoint:
# Agent A: T011 (build() wiring) then T014 (tree verify)
# Agent B: T012 + T013 (tls_rustls_only crate) in parallel
```

---

## Parallel Example: User Story 2

```bash
# After Phase 2 checkpoint:
# Agent A: T016–T017 (code verify) → T018–T019 (tests)
# Agent B: T020 (examples build) → T021 (openssl tree + CHANGELOG note)
```

---

## Implementation Strategy

### MVP First (User Story 1)

1. Complete Phase 1 + Phase 2 (T001–T010)
2. Complete Phase 3 US1 (T011–T015)
3. **STOP and VALIDATE**: `tls_rustls_only` + empty `openssl-sys` tree
4. Add US2 + polish before release

### Incremental Delivery

1. Foundation (reqwest 0.13 + helper + cfg) → compiles
2. US1 opt-out → SC-001/SC-006
3. US2 turnkey regression → SC-002/SC-007
4. US3 audit → SC-005/SC-003
5. Polish docs + lockfiles

### Suggested MVP Scope

**Phases 1–3 (T001–T015)** deliver issue #37 core value: OpenSSL-free opt-out builds with compile-time client injection.

---

## Notes

- Do not hand-edit generated files under `vim_rs/src/types/` or `vim_rs/src/mo/`
- `vim_rs` root is not a workspace; run `cargo` with `-p vim_rs` from repo root
- Use explicit `RUSTUP_HOME` / `CARGO_HOME` in CI/agent per `.cursor/rules/rust-env.mdc`
- Turnkey TLS changes from OpenSSL to rustls (FR-012) — document, do not pin native-tls back on
