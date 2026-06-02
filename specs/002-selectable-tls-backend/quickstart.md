# Quickstart: Selectable TLS / opt-out default client

**Feature**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

For implementers and consumers validating issue #37.

## Prerequisites

- Branch **`002-selectable-tls-backend`**
- Read [research.md](./research.md) for manifest layout

## Implementation order

1. **`vim_rs/Cargo.toml`** — reqwest `0.13`, `default-features = false`, base features; add `default-client`; decouple `cookies` from `xml`.
2. **`vim_rs/src/core/client.rs`** — `build_default_http_client`; cfg-gate auto-build + `insecure`; `MissingHttpClient`; dedupe three call sites.
3. **`vim_rs/src/lib.rs`** — re-export / docs if needed.
4. **Tests** — `cargo test -p vim_rs`, `cargo test -p vim_rs --all-features`; optional unit test for `MissingHttpClient` with `required-features` inverse (see below).
5. **Docs** — `CHANGELOG.md`, `README.md`, `ClientBuilder` rustdoc.
6. **Lockfiles** — `vim_rs/Cargo.lock`, `examples/Cargo.lock`, `mcp/Cargo.lock`.
7. **Verification crate** (recommended) — `examples/tls_rustls_only` for SC-001.

## Commands (reference)

```bash
# Default path (unchanged examples)
RUSTUP_HOME=/Users/kiril/.rustup CARGO_HOME=/Users/kiril/.cargo \
  /Users/kiril/.cargo/bin/cargo test -p vim_rs

RUSTUP_HOME=/Users/kiril/.rustup CARGO_HOME=/Users/kiril/.cargo \
  /Users/kiril/.cargo/bin/cargo test -p vim_rs --all-features

# Confirm turnkey uses rustls, not openssl-sys
RUSTUP_HOME=/Users/kiril/.rustup CARGO_HOME=/Users/kiril/.cargo \
  /Users/kiril/.cargo/bin/cargo tree -p vim_rs -i openssl-sys

# Opt-out consumer (from examples/tls_rustls_only or local crate)
RUSTUP_HOME=/Users/kiril/.rustup CARGO_HOME=/Users/kiril/.cargo \
  /Users/kiril/.cargo/bin/cargo tree -i openssl-sys
# expect: no packages
```

## Consumer snippet (opt-out)

```rust
use std::sync::Arc;
use vim_rs::core::client::{ClientBuilder, Error};

let http = reqwest::Client::builder()
    .build()
    .map_err(Error::ReqwestError)?;

let client = ClientBuilder::new("vcenter.example.com", http)
    .basic_authn("user", "pass")
    .build()
    .await?;
```

With `xml` + SOAP, enable cookies on **your** builder:

```rust
let http = reqwest::Client::builder()
    .cookie_store(true)
    .build()?;
```

## Definition of done (preview)

- `cargo tree -i openssl-sys` empty for opt-out verification crate.
- Default `cargo test -p vim_rs` and `--all-features` pass.
- `core/client.rs` has one `build_default_http_client` (SC-005).
- CHANGELOG notes TLS default change (FR-012).

Full tasks: **`/speckit-tasks`** → `tasks.md`.
