# Research: Selectable TLS backend / opt-out of default client configuration

**Feature**: [spec.md](./spec.md)  
**Date**: 2026-06-01  
**Tracks**: GitHub issue [#37](https://github.com/noclue/vim_rs/issues/37)

Each item: **Decision**, **Rationale**, **Alternatives considered**.

---

## R1 — Feature flag layout (`vim_rs` + `reqwest`)

**Decision**: Introduce a **`default-client`** feature on `vim_rs`, enabled by the crate **`default`** feature array. Wire:

```toml
[features]
default = ["default-client"]
default-client = ["reqwest/default"]

[dependencies.reqwest]
version = "0.13"
default-features = false
features = ["charset", "http2", "system-proxy", "cookies"]

xml = ["dep:quick-xml", "reqwest/cookies"]   # cookies only for SOAP cookie_store
```

Consumers opt out with `vim_rs = { version = "…", default-features = false }`. They then enable TLS on **their** `reqwest` dependency (e.g. `features = ["rustls"]`) and call `ClientBuilder::http_client(injected)`.

**Rationale**: Matches spec FR-001/002/011: turnkey path enables reqwest’s **default** feature set (0.13 → `default-tls` → `rustls`); opt-out removes `reqwest/default` from the unified feature graph so `openssl-sys` is not pulled unless the consumer adds `native-tls`. **`reqwest/cookies` stays on `xml` only** (FR-007): VI/JSON uses header session auth, not cookie jar; only SOAP auto-build calls `.cookie_store(true)`.

**Alternatives considered**:

- **`native-tls` / `rustls-tls` toggles on `vim_rs`** (issue #37 draft) — **rejected** per spec FR-011 (user: use reqwest defaults on turnkey; no per-backend vim_rs toggles).
- **`reqwest` with `default-features = true` always** — **rejected** (cannot satisfy FR-001; Cargo features are additive from dependents).
- **Optional `reqwest` dependency** — **rejected** (`reqwest::Client` is public API on `ClientBuilder::http_client`).

---

## R2 — Failure mode when opted out without injected client (FR-006)

**Decision**: **Compile-time enforcement** via feature-gated `ClientBuilder::new` signatures:

- **`default-client` on**: `new(server_address: &str) -> Self` (unchanged); optional `http_client()` override before `build()`.
- **`default-client` off**: `new(server_address: &str, http_client: reqwest::Client) -> Self` — **required** client at construction; `http_client()` not in API surface.

No runtime `MissingHttpClient` on `build()` for opt-out mode.

**Rationale**: Spec clarification (2026-06-01): fail as early as possible; impossible to construct a builder without a client when opted out. `#[cfg(feature = "default-client")]` on `impl` methods is idiomatic Rust and avoids a second builder type.

**Alternatives considered**:

- **Runtime error at `build().await`** — **rejected** (user clarification; weaker than compile-time).
- **Separate `ClientBuilderMinimal` type** — **rejected** (API duplication; cfg-gated `new` is sufficient).

---

## R3 — `insecure(true)` when `default-client` is off (FR-004 edge case)

**Decision**: **`#[cfg(feature = "default-client")]`** on `ClientBuilder::insecure` (and keep `http_client` always available). Opt-out consumers configure TLS on their injected `reqwest::Client` (e.g. `tls_danger_accept_invalid_certs`).

**Rationale**: Spec allows compile-time absence; avoids silent no-op that looks like it worked. Examples/snippets use default features and keep `insecure()`.

**Alternatives considered**:

- **Runtime warning + ignore** — **rejected** (ambiguous).
- **Keep method, error at `build`** — **acceptable** but worse than cfg for discoverability.

---

## R4 — Shared default HTTP client helper (FR-008)

**Decision**: Single private function in `core/client.rs`, e.g. `build_default_http_client(insecure: Option<bool>, cookie_store: bool) -> Result<reqwest::Client>`, called from `build_json` (`false`), `build_soap_facade` (`true`, `#[cfg(feature = "xml")]`), and `build_auto_facade` probe (`false`). `.cookie_store(true)` is compiled only when `cookie_store` is true **and** `feature = "xml"` (or the call site is already xml-gated).

**Rationale**: Three copies today; SOAP needs cookies; JSON/hello probe do not. Aligns with actual transport semantics (header auth vs cookie jar).

**Alternatives considered**:

- **Unconditional `cookies` on base reqwest dep** — **rejected** (spec clarification 2026-06-01; bloats JSON-only builds).
- **Always enable cookie store on all transports** — **rejected** (unnecessary for JSON).

---

## R5 — reqwest 0.13 upgrade details

**Decision**: Pin **`reqwest = "0.13"`** (latest patch in series). Turnkey TLS = reqwest **`default`** → **`default-tls`** → **`rustls`** with **`rustls-platform-verifier`** (OS trust store). Keep using **`danger_accept_invalid_*`** on `ClientBuilder` (soft-deprecated aliases still present in 0.13.2); optional follow-up to rename to `tls_danger_accept_invalid_*`.

**Rationale**: Spec FR-011/012; platform verifier preserves corporate-CA vCenter behavior vs webpki-only roots. Aliases avoid churn in this PR.

**Alternatives considered**:

- **Stay on 0.12 + only feature flags** — **rejected** (user plan item 1).
- **Pin native-tls on turnkey via `reqwest/native-tls`** — **rejected** (contradicts FR-011 / user clarification).

---

## R6 — Verification of OpenSSL-free opt-out (SC-001)

**Decision**: Add a minimal **`examples/tls_rustls_only`** workspace member (or documented one-liner consumer in `quickstart.md`) that depends on `vim_rs` with `default-features = false`, `reqwest` with `features = ["rustls", "cookies", …]`, and runs `cargo tree -i openssl-sys` expecting empty output. Wire into manual/CI checklist in `quickstart.md`; not required for default `cargo test -p vim_rs`.

**Rationale**: Root repo is not a Cargo workspace with `vim_rs`; feature-matrix tests inside `vim_rs` cannot easily assert absence of transitive deps without a separate manifest.

**Alternatives considered**:

- **Only document manual steps** — **acceptable** but weaker SC-001 evidence.
- **`trybuild` compile-fail** — **insufficient** for dependency-tree claims.

---

## R7 — Documentation / breaking-change messaging (FR-010, FR-012)

**Decision**: **`CHANGELOG.md` [Unreleased]**: breaking/Changed entry — default TLS for turnkey users moves **native-tls/OpenSSL → rustls**; new `default-client` feature; how to opt out; SOAP cookie-store note when opting out. **`README.md`**: feature table + consumer manifest snippet. **`ClientBuilder` rustdoc**: feature requirements and injection examples.

**Rationale**: Constitution V + spec FR-010/012; US2 “unchanged source” still holds for examples, but TLS linkage changes for default builds (documented, not hidden).

**Alternatives considered**: —

---

## R8 — Downstream lockfiles

**Decision**: Update **`vim_rs/Cargo.lock`**, **`examples/Cargo.lock`**, and **`mcp/Cargo.lock`** after `reqwest` bump (MCP server already uses reqwest 0.13 in dev-deps; unified resolution may change).

**Rationale**: Reproducible CI; avoid drift.

**Alternatives considered**: —
