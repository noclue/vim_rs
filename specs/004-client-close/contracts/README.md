# Contracts: Session close (vim_rs 0.6.1)

**Feature**: [spec.md](../spec.md) | **Plan**: [plan.md](../plan.md)

Public consumer contract. Frozen for 0.7.0 except destructor behavior noted below.

## Cargo features

No new features. `default-client`, `xml`, `defaults`, `vcsim_compat` unchanged.

## `Client`

| Method | Signature (conceptual) | Contract |
|--------|------------------------|----------|
| **`close`** | `async fn close(&self) -> Result<()>` | Non-consuming. Ends the vSphere session if one exists. Safe with live clones. Idempotent: second call `Ok` and **no** second Logout. Logout I/O errors are returned **after** the session is marked ended (Drop will not retry). |

`Client` still has **no** `Drop`.

## `VimClient`

| Method | Default | Contract |
|--------|---------|----------|
| **`close`** | `Ok(())` (no-op) | Object-safe (`BoxFuture`). Built-in JSON/SOAP **and** the `Client` facade **override** with real logout. Mocks need not implement it. |

Callers holding `Arc<dyn VimClient>` / `VimClientHandle` use the trait method (async).

## Destructor (0.6.x only — deprecated)

| Runtime | Live session, `close` never called | After successful/failed `close` |
|---------|------------------------------------|----------------------------------|
| Tokio **multi-thread** | Logout once (0.6.0 parity), via `block_in_place` | No logout |
| Tokio **current_thread** | **No panic**. `warn!` to call `close`. Session may linger until idle timeout | No logout, no warn |
| No Tokio handle | `warn!`; no logout; no panic | No logout |

**0.7.0 (this contract’s allowed break):** delete the multi-thread Logout-from-Drop row. `close` signatures stay. Drop only warns if still live.

## Transports

| Transport | Logout mechanism | Wire logging |
|-----------|------------------|--------------|
| JSON | SessionManager Logout + `vmware-api-session-id` | `kind=logout`, SessionManager **denylisted** bodies |
| SOAP (`xml`) | SOAP Logout (cookies) | same denylist |

Parity: both honor idempotent `close` and the Drop table above.

## Errors

| Condition | `close` result |
|-----------|----------------|
| Never logged in / no session manager | `Ok(())` |
| Already closed | `Ok(())` |
| HTTP/transport failure during Logout | `Err(...)` (existing `Error` variants); session marked ended |
| `NotAuthenticated` on later API calls after close | existing error mapping; expected |

## Non-goals (stable in 0.6.1)

- Consuming `close(self)`.
- Feature-gated logout-on-drop.
- `TaskTracker` / `CacheManager` new methods.
- Auto-cancel of in-flight `WaitForUpdatesEx` inside `close`.

## Recommended shutdown (docs)

1. Stop PropertyCollector waits (`Monitor::cancel_wait`, `TaskTracker::shutdown`).
2. `CacheManager::destroy().await`.
3. `client.close().await`.
4. Drop handles.
