# Implementation Plan: Selectable TLS backend / opt-out of default client configuration

**Branch**: `002-selectable-tls-backend` | **Date**: 2026-06-01 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/002-selectable-tls-backend/spec.md` (GitHub [#37](https://github.com/noclue/vim_rs/issues/37))

**Note**: Filled by `/speckit-plan`. Workflow: `.specify/templates/plan-template.md`.

## Summary

Upgrade **`reqwest` to 0.13** and rewire **`vim_rs` Cargo features** so consumers can disable **`default-client`** (`default-features = false`) and supply their own **`reqwest::Client`**—removing forced TLS/OpenSSL from the dependency tree. Turnkey users keep **`ClientBuilder`** auto-build and **`insecure(true)`** via **`default-client` → `reqwest/default`** (0.13 default TLS = **rustls**, documented breaking change). Consolidate duplicated **`reqwest::ClientBuilder`** logic in **`core/client.rs`** into one helper; **`reqwest/cookies`** remains on **`xml`** only; **`.cookie_store(true)`** only on SOAP auto-build path.

## Technical Context

**Language/Version**: Rust 2021, stable (reqwest 0.13 MSRV ≥ 1.85 per upstream; verify against project CI)  
**Primary Dependencies**: `reqwest` 0.13, `tokio`, `miniserde`; optional `quick-xml` (`xml`)  
**Storage**: N/A  
**Testing**: `cargo test -p vim_rs`, `cargo test -p vim_rs --all-features`; optional `examples/tls_rustls_only` + `cargo tree` for SC-001; existing integration tests unchanged  
**Target Platform**: Cross-platform library (Linux/macOS/Windows consumers)  
**Project Type**: Rust library (`vim_rs`) + examples workspace  
**Performance Goals**: No regression vs Principle III release gates (`vim_rs` build time, `examples/vtui` binary size)  
**Constraints**: Hand-written changes only in `vim_rs/Cargo.toml`, `vim_rs/src/core/client.rs`, docs/locks; no generated file edits  
**Scale/Scope**: ~100–200 LOC touched in `client.rs`; manifest + lockfiles + docs

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle / standard | Status | Evidence |
|---------------------|--------|----------|
| **I. Generated from specs** | Pass | No generated `types/` or `mo/` edits |
| **II. Complete type-safe surface** | Pass | Public API additive except cfg-gated `insecure` when opted out |
| **III. Build-time & binary budgets** | Pass | Feature-gated TLS; measure vtui size in release notes per gate |
| **IV. Multi-transport parity** | Pass | JSON + SOAP paths share helper; run `--all-features` + snippets/vtui on release |
| **V. Documented public surfaces** | Pass | CHANGELOG, README, `ClientBuilder` rustdoc, [contracts/](./contracts/) |
| **VI. Readable source** | Pass | Single `build_default_http_client` (FR-008) |
| **VII. Ecosystem tooling** | Pass | Lockfile sync for `mcp/` if needed |
| **Marshalling (miniserde)** | Pass | Unchanged |
| **Security & secrets** | Pass | `insecure` opt-in unchanged; SessionManager wire denylist unchanged |
| **Feature flags** | Pass | Opt-in `default-client`; `xml`/`defaults` unchanged semantics |

**Post-design re-check**: [research.md](./research.md), [data-model.md](./data-model.md), [contracts/](./contracts/) align with gates. **Documented exception**: turnkey TLS backend changes (native-tls → rustls) per FR-012—not a constitution violation when called out in CHANGELOG.

## Project Structure

### Documentation (this feature)

```text
specs/002-selectable-tls-backend/
├── plan.md              # This file
├── research.md          # Phase 0
├── data-model.md        # Phase 1
├── quickstart.md        # Phase 1
├── contracts/           # Phase 1
├── spec.md
└── tasks.md             # /speckit-tasks (not created by /speckit-plan)
```

### Source Code (repository root)

```text
vim_rs/
├── Cargo.toml            # reqwest 0.13, features: default-client, xml tweak
├── Cargo.lock
└── src/core/client.rs    # build_default_http_client, cfg gates, MissingHttpClient

examples/                 # optional: tls_rustls_only member for SC-001
├── Cargo.toml
└── snippets/ vtui/       # unchanged (default features)

CHANGELOG.md
README.md
```

**Structure Decision**: All implementation in **`vim_rs`** core client module; verification example optional under **`examples/`** workspace.

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
| R1 | Feature layout | `default-client` → `reqwest/default`; base reqwest features without TLS |
| R2 | Missing client | `Error::MissingHttpClient` at `build()` |
| R3 | `insecure` | `#[cfg(feature = "default-client")]` |
| R4 | Dedup | `build_default_http_client(insecure, cookie_store)` |
| R5 | reqwest 0.13 | Pin 0.13; turnkey rustls via reqwest defaults |
| R6 | SC-001 proof | Optional `examples/tls_rustls_only` |
| R7 | Docs | CHANGELOG breaking TLS note |
| R8 | Lockfiles | Update vim_rs + examples + mcp locks |

No open **NEEDS CLARIFICATION** items.

---

## Phase 1: Design

**Status**: Complete

### Manifest (target)

```toml
[features]
default = ["default-client"]
defaults = []
default-client = ["reqwest/default"]
xml = ["dep:quick-xml", "reqwest/cookies"]
vcsim_compat = ["xml"]

[dependencies.reqwest]
version = "0.13"
default-features = false
features = ["charset", "http2", "system-proxy"]
```

### `core/client.rs` changes

1. Add **`build_default_http_client(insecure, cookie_store) -> Result<reqwest::Client>`** (`#[cfg(feature = "default-client")]`).
2. Replace inline builders in **`build_json`**, **`build_soap_facade`**, **`build_auto_facade`** with helper calls.
3. Feature-gated **`ClientBuilder::new`**:
   - `default-client` on: `new(server: &str)`
   - `default-client` off: `new(server: &str, http_client: reqwest::Client)` (required)
4. Resolve client in **`build()`**: use `self.http_client` (always `Some` when opted out); when `default-client` on and `None`, call `build_default_http_client(...)`.
5. **`#[cfg(feature = "default-client")]`** on **`insecure()`** and **`http_client()`** (override only on turnkey path).

### Tests

| Test | Purpose |
|------|---------|
| Existing wire/client tests | Regression default + `xml` |
| New: `build_without_client_errors_when_default_client_disabled` | Use `[[test]]` + `required-features = []` cannot express “feature off”. Prefer **integration test in `examples/tls_rustls_only`** or `#[cfg(not(feature = "default-client"))]` unit test in separate **dev-only** pattern — simplest: **document + manual**; optional compile-only example that calls `build()` without client under `default-features = false` |
| `cargo tree -i openssl-sys` | SC-001 / SC-007 verification |

### Documentation deliverables

- **CHANGELOG [Unreleased]**: `default-client`, opt-out instructions, **TLS default rustls**, `xml` no longer pulls `cookies`
- **README**: feature table + consumer `Cargo.toml` snippet
- **`ClientBuilder` rustdoc**: feature + injection + SOAP cookie note

Artifacts: [data-model.md](./data-model.md), [contracts/README.md](./contracts/README.md), [quickstart.md](./quickstart.md)

**Post-design constitution re-check**: Pass (see table above).

---

## Phase 2: Implementation tasks (preview for `/speckit-tasks`)

Not created by this command. Suggested task groups:

1. **Manifest & lockfiles** (Cargo.toml, locks)
2. **Client refactor** (helper, cfg, error, dedupe)
3. **Docs** (CHANGELOG, README, rustdoc)
4. **Verification** (tests, optional `tls_rustls_only` example, tree checks)
5. **Release notes** (TLS breaking change, Principle III metrics if releasing)

---

## Risk & mitigation

| Risk | Mitigation |
|------|------------|
| Corporate CA trust differs rustls vs native-tls | reqwest 0.13 uses platform verifier; document in CHANGELOG; opt-out users choose backend |
| Examples call `insecure()` | Default features unchanged; method remains |
| MSRV bump from reqwest 0.13 | Check CI/toolchain; note in CHANGELOG if raised |
| MCP/examples lockfile conflicts | `cargo update -p reqwest` in each workspace |

## Acceptance mapping (spec → plan)

| Criterion | Plan element |
|-----------|----------------|
| SC-001 | Opt-out + `cargo tree`; `examples/tls_rustls_only` |
| SC-002 | Default features; snippets/vtui unchanged |
| SC-003 | `cookies` on base reqwest |
| SC-004 | Document cross-compile in README/quickstart |
| SC-005 | `build_default_http_client` |
| SC-006 | Compile-fail / cfg-gated `new(server, client)` |
| SC-007 | `default-client` → `reqwest/default`; CHANGELOG |
