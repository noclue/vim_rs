# Data model: Client configuration & feature wiring

**Feature**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

This feature is dependency- and builder-state wiring, not persistent storage. “Entities” below are **configuration concepts** and **builder state**.

---

## Entities

### `default-client` (Cargo feature)

| Attribute | Description |
|-----------|-------------|
| **Default** | Enabled via `vim_rs` `default = ["default-client"]` |
| **Enables** | `reqwest/default` on the dependency graph → TLS + charset + http2 + system-proxy (reqwest 0.13: TLS = rustls) |
| **Code effect** | `#[cfg(feature = "default-client")]` blocks: auto `reqwest::Client` creation, `ClientBuilder::insecure` |
| **When disabled** | Consumer must inject `reqwest::Client`; no `openssl-sys` from `vim_rs` unless consumer adds `native-tls` |

### `ClientBuilder` (runtime)

| Field / concern | Role |
|-----------------|------|
| `http_client: Option<reqwest::Client>` | Turnkey: optional until `build()` (auto-create if `None`). Opt-out: **always `Some`** — set in `new(server, client)` |
| `insecure: Option<bool>` | Only meaningful with `default-client`; passed to shared `build_default_http_client` |
| `transport_mode` | Json / Soap / Auto — unchanged |
| Other fields | `server_address`, auth, API release, wire logging — unchanged |

**State transitions (build)**:

```text
[default-client ON, http_client None]
  → build_default_http_client(insecure, cookie_store per path)
  → connected Client

[default-client ON, http_client Some]
  → use injected client (insecure on builder ignored for TLS — existing behavior)

[default-client OFF]
  → new(server, http_client) only (FR-006); http_client always present

[default-client OFF, invalid API use]
  → compile error (e.g. calling turnkey-only `new(server)`)
```

### `build_default_http_client` (internal helper)

| Parameter | JSON path | SOAP path | Auto probe |
|-----------|-----------|-----------|------------|
| `insecure` | from builder | from builder | from builder |
| `cookie_store` | `false` (VI/JSON uses header session) | `true` (SOAP only) | `false` (Hello probe) |

**Validation**: `insecure == true` logs existing warning; applies `tls_danger_accept_invalid_*` on builder.

### `reqwest` dependency (manifest)

| Always-on features (base dep) | Via `default-client` | Via `xml` |
|------------------------------|----------------------|-----------|
| `charset`, `http2`, `system-proxy` | `default` → `default-tls` → `rustls` | `cookies` (for `.cookie_store`) |

### `xml` feature

| Attribute | Value |
|-----------|-------|
| Enables | `quick-xml`, `reqwest/cookies` |
| Auto-build | `build_default_http_client(..., cookie_store: true)` on SOAP path only |

### `Error` (extension)

No new error variant for missing HTTP client in opt-out mode — prevented at compile time (FR-006).

Existing `ReqwestError`, `MethodFault`, etc. unchanged.

---

## Relationships

```text
Consumer Cargo.toml
  ├─ default-features = true  → vim_rs/default-client → reqwest/default → rustls
  └─ default-features = false → no reqwest TLS from vim_rs
        └─ consumer reqwest + ClientBuilder::http_client(...)

ClientBuilder
  └─ build() → JsonClient | SoapClient (via Client facade)
        └─ holds reqwest::Client (owned or injected)
```

---

## Validation rules (from spec)

| Rule | Enforcement |
|------|-------------|
| FR-003 | No auto-build without `default-client` |
| FR-006 | `MissingHttpClient` at `build()` |
| FR-007 | `cookies` on base `reqwest`, not only `xml` |
| FR-008 | Single `build_default_http_client` |
| FR-009 | `http_client()` works in both modes |
| SOAP opt-out | Document: injected client needs `cookie_store(true)` |
