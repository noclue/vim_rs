# Client architecture (vim_rs 0.4.x line)

This document matches the **current** implementation: a small public `Client` facade (same surface as **0.4.0**), crate-private transport implementations (`JsonClient` and `SoapClient`), optional SOAP when the `xml` feature is enabled, and generated code against `VimClient`. Neither transport type is public API.

---

## 1. Public `Client` (facade) — **only** what 0.4.0 exposed

From release **0.4.0** ([`3e1a171`](https://github.com/noclue/vim_rs/commit/3e1a17121ef6d4b0c2e4c651a314c82ff97b9130)), the **user-visible** methods on `Client` were:

| Method | Role |
|--------|------|
| `service_content()` | Root `ServiceContent` (managers, root folder, etc.). |
| `api_release()` | Negotiated / effective API release string. |
| `fetch_property<T>(obj, property)` | Typed property read (JSON in 0.4.0; now uses `unmarshal` so it also works over SOAP). |

Everything else on the old type that was **only** `pub(crate)` in 0.4.0 (`get_request`, `post_bare`, `execute`, …) stays **implementation detail** of `JsonClient`.

**Restored / fixed vs intermediate XML work:**

- The name **`fetch_property`** (not `fetch_property_typed`) matches 0.4.0.
- **`ClientBuilder::build()`** is the single entry point; it returns **`Arc<Client>`** for **Json**, **Soap**, and **Auto** (with `xml`). There is **no** `build_dyn`.
- **`get_request`** is not part of the public API; it lives on `JsonClient` only.

**Note:** `transport()` exists on **`VimClient`**, not on the public three-method `Client`. If you hold `Arc<Client>` and need the transport enum, use **`VimClient::transport(&*client)`** (after `use … VimClient`).

---

## 2. Crate-private transport types

### `JsonClient`

The former concrete `Client` struct: VI JSON base URL, Hello negotiation, `vmware-api-session-id`, JSON faults, JSON logout in `Drop`.

Built only from `ClientBuilder::build` (or the JSON branch of **Auto**). Wrapped as `Arc::new(Client { inner: arc_json })` for the handle users store.

### `SoapClient`

Defined under the crate-private `xml::client` module. Cookie-based SOAP to `/sdk`, built only from `ClientBuilder::build` with `TransportMode::Soap` or **Auto** (when the `xml` feature is enabled).

---

## 3. `VimClient` and `fetch_property_raw`

Generated `mo::*` types hold `Arc<dyn VimClient>`. They call:

- `invoke` / `invoke_optional` / `invoke_void`
- **`fetch_property_raw(svc, mo_type, mo_id, property)`** → `Option<Bytes>` (wire payload)
- then `unmarshal(self.transport(), &bytes)`.

The name **`fetch_property_raw`** avoids clashing with the public facade method **`fetch_property`**, which is **typed** and uses `fetch_property_raw("", type, id, prop)` + `unmarshal` internally.

The public `Client` **also** implements `VimClient` by delegating to its `inner: Arc<dyn VimClient>`, so `Arc<Client>` coerces cleanly where `Arc<dyn VimClient>` is required.

---

## 4. `ClientBuilder`

- Configures address, TLS, user-agent, credentials, locale, API releases, and **transport** (`TransportMode`: Json / Soap / Auto with `xml`).
- **`build().await` → `Result<Arc<Client>>`** for all modes (Soap/Auto require feature `xml`). There is no separate `build_dyn`.

---

## 5. Drop and logout

The **facade** `Client` has **no** `Drop` implementation.

Logout runs when the **last** `Arc` to the **inner** transport client is dropped:

- **JSON client (`JsonClient`, crate-private)**: `SessionManager.Logout` over the VI JSON API.
- **SOAP client (`SoapClient`, crate-private)**: SOAP `Logout`.

Dropping `Arc<Client>` only drops the facade shell; the inner `Arc<dyn VimClient>` is decremented. When that count hits zero, the concrete type’s `Drop` runs.

---

## 6. Helpers (`ObjectRetriever`, `RootObjects`, …)

They take **`Arc<Client>`** (the facade). That matches examples and 0.4-style code paths; SOAP/JSON both work wherever the underlying `VimClient` and `unmarshal` paths support the operation.

---

## 7. SOAP limitations (unchanged)

The crate-private SOAP client still rejects non-empty **`svc`** on invoke / raw property fetch for paths that assume auxiliary JSON services (PBM, VSAN, …). Those remain **vCenter JSON**-oriented until mapped explicitly for SOAP.
