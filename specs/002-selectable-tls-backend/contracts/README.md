# Contracts: HTTP client & TLS configuration

**Feature**: [spec.md](../spec.md) | **Plan**: [plan.md](../plan.md)

Public behavior for **`vim_rs`** consumers around **`ClientBuilder`**, Cargo features, and **`reqwest`**.

## Cargo features

| Feature | Default | Contract |
|---------|---------|----------|
| **`default-client`** | yes (via `default`) | Enables `reqwest/default` (TLS + reqwest default non-TLS pieces). Auto-creates HTTP client when none injected. Exposes `ClientBuilder::insecure`. |
| **`defaults`** | no | Unrelated: generates `Default` impls for types (existing). |
| **`xml`** | no | SOAP/XML transport; does **not** gate `reqwest/cookies`. |
| **`vcsim_compat`** | no | Implies `xml` (existing). |

**Opt-out manifest**:

```toml
vim_rs = { version = "0.5", default-features = false }
reqwest = { version = "0.13", default-features = false, features = ["rustls", "charset", "http2", "system-proxy", "cookies"] }
```

Consumer **must** pass `reqwest::Client` to **`ClientBuilder::new(server, client)`** (feature-gated signature).

## `ClientBuilder` API

| Method | `default-client` on | `default-client` off |
|--------|----------------------|----------------------|
| `new(server)` | Available | **Not available** |
| `new(server, http_client)` | **Not available** | **Required** |
| `transport`, `basic_authn`, `app_details`, `locale`, `wire_logging`, `api_release`, … | Available | Available |
| `http_client(client)` | Optional override before `build()` | **Not compiled** (`cfg`) |
| `insecure(bool)` | Available; affects auto-built client | **Not compiled** (`cfg`) |
| `build().await` | Auto-builds client if none injected | Uses client from `new`; no auto-build |

**Injected client**: Same behavior whether or not `default-client` is enabled (FR-009). TLS, proxies, timeouts, and cookie store are the consumer’s responsibility when injecting.

## TLS behavior

| Build | Linked TLS (typical Linux) | Trust roots |
|-------|---------------------------|-------------|
| Default (`default-client`) | **rustls** (reqwest 0.13 default) | OS / platform verifier |
| Opt-out + consumer `rustls` | rustls only | Consumer’s reqwest config |
| Opt-out + consumer `native-tls` | native-tls + OpenSSL | Consumer’s choice |

**Breaking change (0.5+)**: Turnkey builds that previously linked **OpenSSL** via reqwest 0.12 now link **rustls** by default. Documented in **CHANGELOG** / **README**.

## SOAP/XML (`xml` feature)

| Mode | Cookie store |
|------|----------------|
| Turnkey + `default-client` + SOAP | Auto-built client uses `cookie_store(true)` |
| Opt-out + SOAP | Consumer’s injected client **must** use `.cookie_store(true)` |

## Error contract

| Condition | Result |
|-----------|--------|
| Opt-out, `new` without `http_client` | **Compile error** (wrong `new` signature / missing argument) |
| Network / HTTP failures | Unchanged (`ReqwestError`, `MethodFault`, …) |

## Non-goals (stable)

- No `vim_rs` feature to pick `native-tls` vs `rustls` without opting out.
- No change to wire logging targets, session header auth, or transport method semantics.
