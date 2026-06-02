# Feature Specification: Selectable TLS Backend / Opt-Out of Default Client Configuration

**Feature Branch**: `002-selectable-tls-backend`

**Created**: 2026-06-01

**Status**: Draft

**Input**: User description: "Address GitHub issue #37. Plan: (1) bump reqwest to 0.13; (2) let a library user opt out of vim_rs's default configuration — which also removes the default reqwest configuration and the ability for ClientBuilder to auto-create a reqwest::Client; (3) deduplicate the reqwest::Client creation code in core/client.rs, currently duplicated across the JSON and XML paths."

## Context & Problem

`vim_rs` today depends on the HTTP client crate with its full default configuration. That default configuration forces a specific TLS backend into every downstream binary, and downstream crates cannot opt out (build configuration is additive — a consumer cannot turn off another crate's defaults from their own manifest). The practical consequences reported in issue #37 are:

- **Supply-chain / image hardening**: the forced backend links a native crypto library that is a recurring source of security advisories; consumers targeting minimal/distroless images want it gone entirely (smaller image, smaller SBOM, fewer advisories to triage).
- **Cross-compilation friction**: the forced native backend makes cross-compiling (e.g. to `linux/arm64`) painful because it needs a target build of the native crypto library; a pure-Rust backend cross-compiles cleanly.
- **Duplication**: consumers already standardized on a pure-Rust backend end up linking two TLS stacks.

This feature lets a consumer fully control the HTTP/TLS stack `vim_rs` links, while preserving today's turnkey experience for everyone who does not opt out.

## Clarifications

### Session 2026-06-01

- Q: When users opt out of the default feature set, how must they supply an HTTP client? → A: **`ClientBuilder::new` requires a non-optional `reqwest::Client` at construction time** (feature-gated API: `new(server, http_client)` when `default-client` is off). Enforcement is **compile-time**, not a runtime error on `build()`.
- Q: Should default auto-created `reqwest::Client` enable `cookie_store`, and should `reqwest/cookies` stay on the base dependency? → A: **No cookie store for VI/JSON.** Auto-build MUST call `.cookie_store(true)` only on the SOAP/XML transport path. The `reqwest` **`cookies` crate feature** MUST be gated on **`xml`** (not unconditional). VI/JSON session auth uses the **`vmware-api-session-id` header**, not HTTP cookies.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Build an OpenSSL-free / pure-Rust-TLS binary (Priority: P1)

A consumer who hardens their image (or cross-compiles) turns off `vim_rs`'s default client configuration in their dependency declaration. With it off, `vim_rs` no longer pulls in any TLS/transport defaults from the HTTP client crate and no longer creates an HTTP client on the consumer's behalf. The consumer constructs their own HTTP client (with whatever TLS backend, roots, proxy, and timeouts they choose) and passes it as a **required** argument to **`ClientBuilder::new`**. The resulting binary links only the TLS stack the consumer selected.

**Why this priority**: This is the core ask of issue #37 and the only path that actually removes the unwanted native crypto library from the dependency tree (runtime injection alone does not, because the default backend is still compiled and linked).

**Independent Test**: In a throwaway consumer crate, depend on `vim_rs` with default features disabled, supply a pure-Rust-TLS HTTP client, build for a Linux target, and confirm the native crypto library is absent from the dependency tree and the build succeeds (including VI/JSON session **header** auth) without the optional SOAP/XML feature enabled.

**Acceptance Scenarios**:

1. **Given** a consumer crate that disables `vim_rs`'s default client configuration and provides its own HTTP client, **When** it builds, **Then** the native crypto library (`openssl-sys`) and the native-TLS bridge do not appear anywhere in the dependency tree.
2. **Given** the same opted-out configuration, **When** the consumer connects and authenticates over VI/JSON, **Then** session authentication works via the **`vmware-api-session-id` header** without enabling the SOAP/XML feature (no HTTP cookie jar required).
3. **Given** the opted-out configuration, **When** the consumer cross-compiles to another Linux architecture, **Then** the build does not require a target build of the native crypto library.
4. **Given** the opted-out configuration, **When** the consumer attempts to call `ClientBuilder::new` without supplying an HTTP client, **Then** the project **does not compile** (the opt-out `new` signature requires `reqwest::Client`).

---

### User Story 2 - Existing consumers keep working unchanged (Priority: P1)

A consumer who does nothing (keeps default features) sees no behavioral or source-level change: the client builder still creates an HTTP client automatically, the `insecure(true)` convenience still works, and connecting/authenticating over VI/JSON and SOAP/XML behaves exactly as before.

**Why this priority**: The change must not break the large majority of consumers who rely on the turnkey path; backward compatibility for the default build is non-negotiable.

**Independent Test**: Build and run the existing in-repo examples (`examples/snippets`, `examples/vtui`) and the test suite with default features and confirm no source edits are required and behavior is unchanged.

**Acceptance Scenarios**:

1. **Given** an unchanged consumer using default features, **When** they call the builder without supplying an HTTP client, **Then** a working client is created automatically as today.
2. **Given** an unchanged consumer, **When** they call the `insecure(true)` convenience, **Then** certificate/hostname verification is disabled exactly as before (with the same warning).
3. **Given** an unchanged consumer, **When** TLS verification, custom CA bundles, and connections to a vCenter fronted by a private/corporate CA are exercised, **Then** they continue to work.

---

### User Story 3 - Single, deduplicated client-creation path (Priority: P2)

A maintainer reading `core/client.rs` finds one place that builds the default HTTP client, shared by the VI/JSON, SOAP/XML, and auto-detect transport paths, instead of three near-identical copies. The shared path correctly accounts for transport-specific needs (e.g. the session-cookie store required by SOAP/XML).

**Why this priority**: Reduces maintenance risk and keeps the source readable at scale; it is a refactor that supports — but is not strictly required for — the consumer-facing goals.

**Independent Test**: Inspect `core/client.rs`: the default HTTP-client construction (including the insecure flag handling and cookie-store handling) exists in exactly one helper; the JSON, SOAP, and auto-detect build paths call it. The full test suite (default and all-features) still passes.

**Acceptance Scenarios**:

1. **Given** the JSON, SOAP, and auto-detect build paths, **When** they need a default HTTP client, **Then** they obtain it from a single shared helper rather than constructing it inline.
2. **Given** the SOAP/XML path (`xml` feature), **When** the shared helper auto-builds a client, **Then** `.cookie_store(true)` is applied **only** for that path so SOAP session auth keeps working; the VI/JSON path does not enable a cookie store.

---

### Edge Cases

- **Opt-out with no HTTP client at construction**: A consumer disables the default configuration but calls a `new` entry point that omits `reqwest::Client`. The project **must not compile** (see FR-006); there is no runtime fallback on `build()`.
- **Opt-out + `insecure(true)`**: With the default configuration off, `vim_rs` does not create or configure the HTTP client, so the `insecure` convenience is not applicable. The behavior when an opted-out consumer references the insecure convenience must be unambiguous (compile-time absence or a clearly documented no-effect), not a silent partial behavior.
- **SOAP/XML without the default config**: The session-cookie store is part of `vim_rs`'s default client creation. When a consumer opts out and uses SOAP/XML, they are responsible for enabling a cookie store on their injected client; this requirement must be documented.
- **JSON-only builds without `xml`**: Default auto-built clients for VI/JSON MUST NOT enable `cookie_store`; the `reqwest/cookies` dependency feature MUST NOT be required for JSON-only builds. VI/JSON session state uses response/request headers, not the cookie jar.
- **TLS default change on upgrade**: Upgrading the HTTP client crate to 0.13 changes the out-of-the-box TLS backend for the turnkey path from native-TLS (OpenSSL) to the pure-Rust stack (rustls), because `vim_rs` simply enables that crate's default feature set (FR-011). This is a deliberate, documented behavior change for existing turnkey consumers (FR-012), not an accident.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: `vim_rs` MUST allow a consumer to disable, from their own dependency declaration, the bundled default client configuration — including any TLS/transport defaults `vim_rs` would otherwise force into the dependency tree via the HTTP client crate.
- **FR-002**: When the default client configuration is disabled, `vim_rs` MUST NOT pull in the HTTP client crate's default feature set (so the consumer's selected TLS backend is the only one compiled/linked), while still depending on the HTTP client crate for the injected-client API surface.
- **FR-003**: When the default client configuration is disabled, `vim_rs` MUST NOT auto-create an HTTP client; the consumer MUST supply one as a **required** parameter to **`ClientBuilder::new`** (feature-gated signature: `new(server_address, http_client)`). The optional `http_client()` override on the builder MUST NOT be the only injection path in opt-out mode (that method is for turnkey overrides only; see FR-009).
- **FR-004**: When the default client configuration is enabled (the default), `vim_rs` MUST preserve today's turnkey behavior: the builder auto-creates a working HTTP client when none is injected, and the `insecure(true)` convenience disables certificate/hostname verification with the existing warning.
- **FR-005**: The HTTP client crate dependency MUST be upgraded to its 0.13 series, and the default (turnkey) path MUST continue to support TLS verification, custom CA bundles, connections to vCenter fronted by a private/corporate CA, and the `insecure(true)` escape hatch.
- **FR-006**: When the default client configuration is disabled, **`ClientBuilder::new` MUST require a `reqwest::Client` argument** (non-optional). Omitting it MUST be a **compile-time error** via feature-gated API surface, not a runtime error on `build()`. The system MUST NOT silently fall back to auto-creating a client at `build()`.
- **FR-007**: VI/JSON session authentication MUST continue to work on the default (turnkey) build **without** the `xml` feature enabled, using the existing **`vmware-api-session-id` header** mechanism (not HTTP cookies). The `reqwest` **`cookies` crate feature** MUST be enabled only when the **`xml`** feature is enabled; JSON-only builds MUST NOT depend on `reqwest/cookies`.
- **FR-008**: The default HTTP-client construction logic (including insecure-flag handling) MUST exist in a single shared helper in `core/client.rs`, used by the VI/JSON, SOAP/XML, and auto-detect build paths. The helper MUST accept a **`cookie_store: bool`** (or equivalent); **only** the SOAP/XML build path (behind `feature = "xml"`) passes `true` so `.cookie_store(true)` is applied; VI/JSON and Hello probe paths pass `false`.
- **FR-009**: With **default client configuration enabled**, consumers MAY use `ClientBuilder::new(server_address)` and optionally override the HTTP client via `http_client()` before `build()`. With **default client configuration disabled**, consumers MUST pass `reqwest::Client` to `new(server_address, http_client)`; `http_client()` MUST NOT be available (compile-time) so injection is unambiguous at construction. Transport behavior after a client is supplied MUST be identical in both modes.
- **FR-010**: User-visible changes — the new opt-out capability, the TLS-backend implications, the cookie capability move, and any behavior change to the default TLS backend — MUST be documented in `CHANGELOG.md`, `README.md`, and crate-level/builder rustdoc, including the requirement to bring a cookie-enabled client when using SOAP/XML in opt-out mode.
- **FR-011**: The turnkey (default-config-enabled) path MUST enable the HTTP client crate's own default feature set rather than pinning a specific TLS backend; whatever TLS backend that crate ships as its default at the pinned version is the turnkey default. With the 0.13 upgrade this means the turnkey default TLS backend becomes the pure-Rust stack (rustls), and `vim_rs` MUST NOT force the previous native-TLS backend back on. `vim_rs` MUST NOT expose its own per-backend selection toggles; consumers who need a different backend opt out of the default configuration (FR-001–FR-003) and inject their own client.
- **FR-012**: Because FR-011 changes the out-of-the-box TLS backend for existing turnkey consumers (native-TLS → rustls), this behavior change MUST be called out explicitly as a notable (potentially breaking) change in `CHANGELOG.md` and `README.md`, including that the turnkey path no longer links the native crypto library by default.

### Non-Functional / Constraints

- **NFR-001**: The change to default-feature wiring and client construction MUST NOT regress `vim_rs` debug/release build time or `examples/vtui` binary size beyond the project's release-gate tolerances; the SOAP/XML and `defaults` features remain opt-in.
- **NFR-002**: Marshalling, error handling, async-I/O, and wire-diagnostics conventions remain unchanged; this feature only affects dependency wiring and HTTP-client construction, not transport semantics.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A consumer crate that disables `vim_rs`'s default client configuration and injects a pure-Rust-TLS HTTP client produces a dependency tree with **zero** occurrences of the native crypto library (`openssl-sys`) and the native-TLS bridge, verifiable with a dependency-tree inspection.
- **SC-002**: The default (turnkey) build requires **no** source changes from existing consumers: in-repo examples and tests build and behave identically to the prior release on both VI/JSON and SOAP/XML.
- **SC-003**: VI/JSON session authentication (via **`vmware-api-session-id` header**) succeeds on a default build **without** the `xml` feature enabled, with **no** `reqwest/cookies` in the dependency graph for that build configuration.
- **SC-004**: The opted-out configuration cross-compiles to at least one additional Linux architecture **without** requiring a target build of the native crypto library.
- **SC-005**: `core/client.rs` contains exactly **one** default-HTTP-client construction helper; the JSON, SOAP, and auto-detect paths all call it, and `cargo test` (default and all-features) passes.
- **SC-006**: A consumer who opts out and omits `reqwest::Client` from `ClientBuilder::new` **cannot compile** — confirmable by a compile-fail test, API documentation, or review of the feature-gated `new` signature (not a runtime `build()` failure).
- **SC-007**: On a default (turnkey) build with the 0.13 upgrade, the linked TLS backend is the HTTP client crate's default (rustls) — verifiable by dependency-tree inspection — and this change is recorded as a notable change in `CHANGELOG.md`/`README.md`.

## Assumptions

- The opt-out is expressed through `vim_rs`'s feature wiring: a default-on feature carries the bundled client configuration (HTTP-client-crate defaults needed by `vim_rs` plus the auto-create/`insecure` convenience), and disabling default features turns it off. Exact feature naming is an implementation/plan decision.
- The HTTP client crate remains a non-optional dependency of `vim_rs` because its client type appears in the public injection API; opting out changes which of *its* features `vim_rs` enables, not whether it is present.
- Opt-out enforcement uses **compile-time** API shape: `ClientBuilder::new(server, http_client)` when `default-client` is disabled; turnkey keeps `new(server)` only (FR-006).
- The pure-Rust default of the upgraded HTTP client crate uses the platform/OS trust store, which preserves compatibility with vCenter fronted by private/corporate CAs (matching the prior native backend's behavior); webpki-only roots are not assumed.
- `vim_rs` uses none of the HTTP client crate's `json` / `stream` / `multipart` / compression features and does not need them re-added when defaults are disabled. The **`cookies`** reqwest feature is **SOAP/XML-only** (`xml` feature); VI/JSON does not use HTTP cookie storage for sessions.
- This repository's generated-binding policy is unaffected: changes are confined to `vim_rs/Cargo.toml` and hand-written `core/client.rs` (and docs), not generated files.

## Out of Scope

- Replacing the HTTP client crate with a different library, or changing transport semantics (request/response handling, wire logging, session management).
- Adding runtime TLS configuration knobs to `vim_rs` beyond the existing injected-client entry point and `insecure` convenience.
- Per-consumer migration tooling or forks; the change is a manifest/feature-wiring change plus a documented client-injection requirement in opt-out mode.
- Broadening certificate/roots handling beyond what the upgraded HTTP client crate provides by default and via the consumer's injected client.
