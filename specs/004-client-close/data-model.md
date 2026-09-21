# Data Model: Client Session Close (0.6.1)

**Feature**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

In-memory session lifecycle for `vim_rs` transports. No persistence.

## Entities

### Session handle (`Client` / `VimClientHandle`)

Shared, cloneable owner of a transport. Public `close` does not consume the handle.

| Field / role | Notes |
|--------------|--------|
| `Client.inner` | `Arc<dyn VimClient>` → `JsonClient` or `SoapClient` |
| Clones | MO stubs, `CacheManager`, `TaskTracker`, app tasks |

**Validation**: `close` is valid with any clone count ≥ 1.

### Remote session

Server-side login established by `ClientBuilder` + `SessionManager.Login`.

| Transport | Credential | Logout |
|-----------|------------|--------|
| JSON | `vmware-api-session-id` in `JsonClient.session_key: Arc<RwLock<Option<String>>>` | POST `…/SessionManager/{id}/Logout` with header |
| SOAP | Cookie jar on `reqwest::Client` | SOAP `Logout` on `SessionManager` |

**Validation**: No credential / no `session_manager` moref → `close` is `Ok(())` (no HTTP).

### Session-ended flag

`session_ended: AtomicBool` on `JsonClient` and `SoapClient`.

| Value | Meaning |
|-------|---------|
| `false` | Logout not claimed; Drop may still log out (multi-thread) or warn (`current_thread`) |
| `true` | Logout claimed by `close` or Drop; no further logout HTTP |

Shared across all `Arc` clones of that inner transport (the flag lives on the inner object).

## State transitions

```text
                    login success
  [NoSession] ─────────────────────► [Live]
       │                                │
       │ close (no-op Ok)               │ close CAS false→true
       ▼                                ▼
  [NoSession]                      [Ending] ──logout HTTP──► [Ended]
                                        │                      │
                                        │ logout error         │ close again → Ok, no HTTP
                                        ▼                      │
                                   [Ended] (error returned)    │ last Arc Drop → silent
                                                               ▼
                                                            [Dropped]
```

| From | Event | To | HTTP | Drop later |
|------|-------|-----|------|------------|
| NoSession | `close` | NoSession | none | none |
| Live | `close` (wins CAS) | Ended | one Logout | silent |
| Live | `close` (loses CAS) | Ended | none | silent |
| Live | last Arc Drop, multi-thread | Ended | one Logout | n/a |
| Live | last Arc Drop, current_thread / no runtime | Live (leaked until idle timeout) | none | warn once |
| Ended | Drop | Dropped | none | silent |

`[Ending]` is not observable: CAS and HTTP run under `close`/`Drop` with `session_ended` already true so a racing Drop cannot start a second logout.

## Invariants

1. At most one Logout is sent per inner transport instance.
2. After `session_ended == true`, Drop never calls `block_in_place`.
3. JSON logout HTTP uses the session header that was valid at claim time (clear key after send).
4. Facade `Client` has no `Drop` (unchanged); only inner transports log out.

## Relationships

- `TaskTracker` / `CacheManager` / `ObjectRetriever` **hold** session handles; they do not own the ended flag.
- Recommended order (docs only): cancel waits → `CacheManager::destroy` / `TaskTracker::shutdown` → `close`.
