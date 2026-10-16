# Tasks: Third-Party Dependency Refresh

**Input**: Design documents from `/specs/005-bump-third-party-deps/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/README.md](./contracts/README.md), [quickstart.md](./quickstart.md)

**Tests**: No new tests. The spec accepts the existing suites. Verification tasks run those commands; do not add test files and do not edit fixture expectations to hide a parse change.

**Organization**: Tasks grouped by user story (P1 → P2). Compatible lockfile refresh (US1) lands before the quick-xml 0.42 migration (US2). Changelog (US3) uses the versions actually resolved.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependency on an incomplete sibling task)
- **[Story]**: US1, US2, US3 per [spec.md](./spec.md)

## Path Conventions

- Lockfiles: `vim_rs/Cargo.lock`, `vim_macros/Cargo.lock`, `vim_build/Cargo.lock`, `openapi30/Cargo.lock`, `tls_rustls_only/Cargo.lock`, `examples/Cargo.lock`, `mcp/Cargo.lock`
- Named manifests: `vim_rs/Cargo.toml`, `mcp/server/Cargo.toml`, `examples/snippets/Cargo.toml`, `examples/vtui/Cargo.toml`
- XML call sites: `vim_rs/src/xml/de.rs`, `vim_rs/src/xml/client.rs`, `vim_rs/src/xml/soap.rs`, `vim_rs/src/xml/ser.rs`
- Notes: `CHANGELOG.md`, `specs/005-bump-third-party-deps/research.md`
- Do not edit `vim_rs/src/types/` or `vim_rs/src/mo/`

Agent cargo prefix, when the sandbox overrides Rust paths: `RUSTUP_HOME=/Users/kiril/.rustup CARGO_HOME=/Users/kiril/.cargo /Users/kiril/.cargo/bin/cargo`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Confirm the tree still matches the 2026-10-15 plan before any update.

- [X] T001 Confirm `quick-xml` is `{ version = "0.40", optional = true }` in `vim_rs/Cargo.toml`, `rmcp` is `{ version = "3.4", features = ["server", "macros"] }` and `anyhow = "1.0"` in `mcp/server/Cargo.toml`, and `anyhow = "1.0"` in `examples/snippets/Cargo.toml` and `examples/vtui/Cargo.toml`
- [X] T002 [P] Confirm lockfiles exist at `vim_rs/Cargo.lock`, `vim_macros/Cargo.lock`, `vim_build/Cargo.lock`, `openapi30/Cargo.lock`, `tls_rustls_only/Cargo.lock`, `examples/Cargo.lock`, and `mcp/Cargo.lock`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Freeze the before-state and the declarations that must not widen. **Blocks all user stories.**

**⚠️ CRITICAL**: No user story work until this phase completes.

- [X] T003 Read resolved versions for `quick-xml` in `vim_rs/Cargo.lock` and `examples/Cargo.lock`, and for `rmcp`, `rmcp-macros`, and `anyhow` in `mcp/Cargo.lock` and `examples/Cargo.lock`. If any differ from quick-xml 0.40.1, rmcp 3.4.0, rmcp-macros 3.4.0, or anyhow 1.0.103, update the baseline sentences in `specs/005-bump-third-party-deps/research.md` and the "Resolved before" column in `specs/005-bump-third-party-deps/data-model.md` before running `cargo update`
- [X] T004 Confirm do-not-widen declarations still match `specs/005-bump-third-party-deps/research.md` R5: `syn` `"2.0"` in `vim_macros/Cargo.toml`, `convert_case` `"0.11"` in `vim_build/Cargo.toml`, `convert_case` `"0.10"` in `mcp/data_processing/build_api_definitions/Cargo.toml`, `bincode` `"2.0"` in `mcp/server/Cargo.toml` and `mcp/data_processing/data_transformer/Cargo.toml`, `fastembed` `"5.8"` in `mcp/server/Cargo.toml` and `"5.8.1"` in `mcp/data_processing/build_embeddings/Cargo.toml`, `actix-files` `"0.6"` in `mcp/server/Cargo.toml`, `base64` `"0.22"` in `vim_rs/Cargo.toml`, and `version = "0.6.0"` in `vim_rs/Cargo.toml` and `vim_macros/Cargo.toml`

**Checkpoint**: Baselines are accurate. Manifests that must stay put are confirmed. User story work can start.

---

## Phase 3: User Story 1 — Current compatible libraries (Priority: P1) 🎯 MVP

**Goal**: Every lockfile resolves the newest release still allowed by its current declarations. Builds and existing tests stay green. quick-xml stays 0.40.1 until US2.

**Independent Test**: After T005–T019, each package root builds or tests clean. `vim_rs/Cargo.lock` still has quick-xml 0.40.1. `mcp/Cargo.lock` has rmcp 3.5.1 (or a newer 3.5.x) and anyhow 1.0.104 (or a newer 1.0.x). `examples/Cargo.lock` has the same anyhow release. No manifest declaration changed. Any precise-pin is written in `specs/005-bump-third-party-deps/research.md`.

### Implementation for User Story 1

- [X] T005 [P] [US1] Run `cargo update` in `vim_rs/` so only `vim_rs/Cargo.lock` changes. Do not edit `vim_rs/Cargo.toml`
- [X] T006 [P] [US1] Run `cargo update` in `vim_macros/` so only `vim_macros/Cargo.lock` changes. Leave `syn` at `"2.0"` in `vim_macros/Cargo.toml`
- [X] T007 [P] [US1] Run `cargo update` in `vim_build/` so only `vim_build/Cargo.lock` changes. Leave `convert_case` at `"0.11"` in `vim_build/Cargo.toml`
- [X] T008 [P] [US1] Run `cargo update` in `openapi30/` so only `openapi30/Cargo.lock` changes. Do not edit `openapi30/Cargo.toml`
- [X] T009 [P] [US1] Run `cargo update` in `tls_rustls_only/` so only `tls_rustls_only/Cargo.lock` changes. Do not edit `tls_rustls_only/Cargo.toml`
- [X] T010 [P] [US1] Run `cargo update` in `examples/` so only `examples/Cargo.lock` changes. Leave `anyhow = "1.0"` in `examples/snippets/Cargo.toml` and `examples/vtui/Cargo.toml`
- [X] T011 [P] [US1] Run `cargo update` in `mcp/` so only `mcp/Cargo.lock` changes. Leave `rmcp`, `anyhow`, `bincode`, `fastembed`, `actix-files`, and `convert_case` declarations unchanged under `mcp/server/Cargo.toml`, `mcp/api_database/Cargo.toml`, and `mcp/data_processing/*/Cargo.toml`
- [X] T012 [P] [US1] Run `cargo test` in `vim_rs/` (default features, no manifest edits). On failure, stop and leave the precise-pin to T019
- [X] T013 [P] [US1] Run `cargo test` in `vim_macros/`. On failure, stop and leave the precise-pin to T019
- [X] T014 [P] [US1] Run `cargo test` in `vim_build/`. On failure, stop and leave the precise-pin to T019
- [X] T015 [P] [US1] Run `cargo test` in `openapi30/`. On failure, stop and leave the precise-pin to T019
- [X] T016 [P] [US1] Run `cargo build` in `tls_rustls_only/`. On failure, stop and leave the precise-pin to T019
- [X] T017 [P] [US1] Run `cargo build` in `examples/`. On failure, stop and leave the precise-pin to T019
- [X] T018 [P] [US1] Run `cargo build -p vim_mcp_server` in `mcp/`. Do not commit `mcp/data/` and do not regenerate bindings. On failure, stop and leave the precise-pin to T019
- [X] T019 [US1] For each failure from T012–T018, in that package root run `cargo update -p <crate> --precise <previous>` against the matching `Cargo.lock`, re-run that root's command, and append package, rejected version, kept version, and reason under an "Implementation exceptions" heading in `specs/005-bump-third-party-deps/research.md`. Then confirm `vim_rs/Cargo.lock` still resolves quick-xml 0.40.1, `mcp/Cargo.lock` resolves rmcp 3.5.1 (or newer 3.5.x) plus rmcp-macros at the same 3.5 line and anyhow 1.0.104 (or newer 1.0.x), and `examples/Cargo.lock` resolves that same anyhow release. If rmcp or anyhow did not move, re-run `cargo update -p rmcp -p anyhow` in `mcp/` and `cargo update -p anyhow` in `examples/`

**Checkpoint**: Compatible refresh is green. Named declarations are unchanged. quick-xml is still 0.40.1. rmcp and anyhow have moved.

---

## Phase 4: User Story 2 — quick-xml, rmcp, and anyhow on the newest adoptable release (Priority: P1)

**Goal**: quick-xml resolves 0.42.0 (or a newer 0.42.x) and the XML transport still parses the same VIM values. rmcp 3.5 and anyhow 1.0 from US1 stay in place and still build.

**Independent Test**: `cargo test --features xml` and `cargo test --features xml vcsim_xml` pass in `vim_rs/` with unchanged fixture values. `vim_rs/Cargo.lock` and `examples/Cargo.lock` resolve the same quick-xml 0.42.x. `mcp/Cargo.lock` resolves one rmcp 3.5.x and one anyhow 1.0.x, matching `examples/Cargo.lock` for anyhow. `vim_rs` version is still 0.6.0.

### Implementation for User Story 2

- [X] T020 [US2] Set `quick-xml = { version = "0.42", optional = true }` in `vim_rs/Cargo.toml`. Do not change `version = "0.6.0"` or the `xml` / `vcsim_compat` features
- [X] T021 [P] [US2] Run `cargo update -p quick-xml` in `vim_rs/` so `vim_rs/Cargo.lock` resolves quick-xml 0.42.0 or a newer 0.42.x published since the plan
- [X] T022 [P] [US2] Run `cargo update -p quick-xml` in `examples/` so `examples/Cargo.lock` resolves the same quick-xml release as `vim_rs/Cargo.lock`
- [X] T023 [P] [US2] Migrate quick-xml 0.40 byte APIs to 0.42 string APIs in `vim_rs/src/xml/de.rs`: `as_ref()` on names is `&str` (drop `std::str::from_utf8`), compare local names to `"type"` and `"nil"` instead of `b"type"` and `b"nil"`, replace `BytesText::decode()` and `from_utf8(&event)` with `as_ref()` or deref to `str`, and keep `normalized_value(XmlVersion::Implicit1_0)`
- [X] T024 [P] [US2] Apply the same string-name migration in `vim_rs/src/xml/client.rs` (`extract_soap_fault` and `extract_soap_method_fault`). Keep `Reader::from_str`
- [X] T025 [US2] Compile `vim_rs` with `--features xml`. If `vim_rs/src/xml/soap.rs` or `vim_rs/src/xml/ser.rs` fails, fix only those call sites. Keep `quick_xml::escape::escape` in `vim_rs/src/xml/ser.rs`. Do not enable quick-xml's `serde` feature in `vim_rs/Cargo.toml`
- [X] T026 [US2] Run `cargo test --features xml` and `cargo test --features xml vcsim_xml` in `vim_rs/`. vcsim `_XMLSchema-instance:type` and vCenter `xsi:type` fixtures in `vim_rs/tests/vcsim_xml_regression.rs` must keep the same VIM values. If a fixture fails with too many namespace bindings, raise the limit via `NamespaceResolver::set_max_namespace_bindings` in `vim_rs/src/xml/de.rs`. If parsed values differ for any other reason, `cargo update -p quick-xml --precise 0.40.1` in `vim_rs/` and `examples/`, revert `vim_rs/Cargo.toml` and the `vim_rs/src/xml/` edits, and record the rejection in `specs/005-bump-third-party-deps/research.md`. Do not edit `vim_rs/tests/fixtures/vcsim.xml` to match a new parse
- [X] T027 [P] [US2] If `cargo build -p vim_mcp_server` in `mcp/` fails on rmcp 3.5, fix call sites in `mcp/server/src/main.rs` (`#[tool]`, `#[tool_router]`, `#[tool_handler]`, `ServerHandler`, `ServiceExt`). Change `rmcp` to `"3.5"` in `mcp/server/Cargo.toml` only when the fix needs an API that 3.4 does not have. Confirm a single anyhow 1.0.x in both `mcp/Cargo.lock` and `examples/Cargo.lock`
- [X] T028 [US2] Run `cargo test --all-features` in `vim_rs/`, `cargo build` in `examples/`, and `cargo build -p vim_mcp_server` in `mcp/`. Leave `soap_integration` ignored unless T026 changed parsed XML values; only then run `cargo test --features xml --test soap_integration -- --ignored` from `vim_rs/` with `.govc_env` sourced

**Checkpoint**: quick-xml 0.42.x is in both lockfiles that carry it. XML fixtures still parse. MCP server and examples build. Core library version is unchanged.

---

## Phase 5: User Story 3 — Reviewers can see what changed (Priority: P2)

**Goal**: The changelog lists the three named old→new versions, summarizes every other lockfile move, notes the quick-xml security fix, and lists any precise-pin exceptions.

**Independent Test**: `CHANGELOG.md` `## [Unreleased]` matches `specs/005-bump-third-party-deps/contracts/README.md`: three named rows, one summary sentence, a Security note, and exceptions only when `specs/005-bump-third-party-deps/research.md` has them. No consumer source change is required. `vim_rs` version is still 0.6.0.

### Implementation for User Story 3

- [X] T029 [US3] Under `## [Unreleased]` in `CHANGELOG.md`, add **Dependencies** (quick-xml, rmcp, and anyhow old→new from the lockfiles, plus one sentence for the other updates across the seven `Cargo.lock` files), **Security** (quick-xml 0.42 nesting / namespace-depth fix for the optional XML transport), and **Exceptions** only when T019 or T026 recorded any in `specs/005-bump-third-party-deps/research.md`. Do not list bincode 3, fastembed 7, base64 0.23, syn 3, or convert_case 0.12 as upgrades. State that `\r` may serialize as `&#13;` via `vim_rs/src/xml/ser.rs`

**Checkpoint**: A reviewer can see what moved without reading the lockfiles.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Prove the contract holds across stories.

- [X] T030 [P] Confirm `version = "0.6.0"` is unchanged in `vim_rs/Cargo.toml` and `vim_macros/Cargo.toml`
- [X] T031 [P] Confirm the diff does not modify `vim_rs/src/types/` or `vim_rs/src/mo/`, and does not add or remove third-party libraries in any `Cargo.toml`
- [X] T032 Run the build and test commands in `specs/005-bump-third-party-deps/quickstart.md` steps 2 and 4. Confirm `vim_macros/Cargo.lock` has no syn 3.x, `mcp/Cargo.lock` has no bincode 3.x and no fastembed 7.x, and `vim_rs/Cargo.lock` has no base64 0.23.x

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies
- **Foundational (Phase 2)**: Depends on Setup. Blocks all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational. No dependency on US2 or US3
- **User Story 2 (Phase 4)**: Depends on US1. quick-xml must stay 0.40.1 until the compatible refresh is green, so a 0.42 compile failure is not mixed with transitive bumps
- **User Story 3 (Phase 5)**: Depends on US1 and US2 so the changelog quotes resolved versions
- **Polish (Phase 6)**: Depends on US1–US3

### User Story Dependencies

- **User Story 1 (P1)**: After Foundational. Delivers a green compatible refresh by itself
- **User Story 2 (P1)**: After US1. rmcp and anyhow already moved; this story widens quick-xml and repairs call sites
- **User Story 3 (P2)**: After US1 and US2. Documentation only

### Within Each User Story

- US1: T005–T011 together, then T012–T018 together, then T019
- US2: T020, then T021 and T022 together, then T023 and T024 together, then T025, then T026. T027 can run beside T023–T026. T028 last
- US3: T029 after the lockfiles and exceptions are final

### Parallel Opportunities

- T005–T011 touch seven different lockfiles
- T012–T018 test seven different roots
- T021 and T022 update two different lockfiles after T020
- T023 and T024 edit two different XML files
- T027 (MCP) does not touch `vim_rs/src/xml/`
- T030 and T031 check different paths

---

## Parallel Example: User Story 1

```bash
# After T003–T004, update every lockfile together:
(cd vim_rs && cargo update)
(cd vim_macros && cargo update)
(cd vim_build && cargo update)
(cd openapi30 && cargo update)
(cd tls_rustls_only && cargo update)
(cd examples && cargo update)
(cd mcp && cargo update)

# Then verify together:
(cd vim_rs && cargo test)
(cd vim_macros && cargo test)
(cd vim_build && cargo test)
(cd openapi30 && cargo test)
(cd tls_rustls_only && cargo build)
(cd examples && cargo build)
(cd mcp && cargo build -p vim_mcp_server)
```

## Parallel Example: User Story 2

```bash
# After T020 (vim_rs/Cargo.toml quick-xml = "0.42"):
(cd vim_rs && cargo update -p quick-xml)
(cd examples && cargo update -p quick-xml)

# Then migrate call sites together:
# vim_rs/src/xml/de.rs
# vim_rs/src/xml/client.rs
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Finish Phase 1 and Phase 2
2. Finish Phase 3 (compatible `cargo update` and green builds)
3. **STOP and VALIDATE**: quick-xml still 0.40.1, rmcp and anyhow moved, no manifest edits, exceptions written if any

US1 does not bump quick-xml. Continue into US2 before treating the requested named bumps as done.

### Incremental Delivery

1. Setup + Foundational → baselines checked
2. User Story 1 → compatible refresh green
3. User Story 2 → quick-xml 0.42.x, XML fixtures unchanged, MCP and examples build
4. User Story 3 → changelog matches the contract
5. Polish → version, generated tree, and quickstart matrix

### Parallel Team Strategy

1. One person runs Setup and Foundational
2. After that, seven lockfile updates (T005–T011) and then seven verifications (T012–T018) run side by side
3. After US1, one person migrates `vim_rs/src/xml/` while another watches `mcp/server/src/main.rs` (T027)
4. Changelog waits until both are green

---

## Notes

- [P] tasks use different files and do not need a sibling task's output
- US2 is still P1. It is sequenced after US1 so the quick-xml migration is not debugged on a stale lockfile
- Precise-pin with `cargo update -p <crate> --precise <previous>`. Do not widen a non-named declaration to chase a failure
- Do not adopt bincode 3 (unmaintained placeholder) or fastembed 7
- Commit after US1, after US2, and after the changelog if you want reviewable steps
