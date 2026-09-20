# Research: Explicit Client Session Close (0.6.1)

**Feature**: [spec.md](./spec.md)  
**Date**: 2026-09-21  
**Branch**: `004-client-close`

Each item: **Decision**, **Rationale**, **Alternatives considered**.

---

## R1 — Where `close` lives (`Client` vs `VimClient`)

**Decision**: Add both:

1. Inherent `Client::close(&self) -> impl Future<Output = Result<()>>` for `Arc<Client>` holders (snippets, 0.4-style code).
2. `VimClient::close(&self) -> BoxFuture<'_, Result<()>>` with a **default** `Ok(())` body.
3. **`impl VimClient for Client` MUST override `close` and forward to `inner`.** JSON and SOAP override with real logout.

**Rationale**: Spec FR-001/FR-002. vtui and MO stubs hold `VimClientHandle` (`Arc<dyn VimClient>`), typically unsized from `Arc<Client>`. If the facade used the default no-op, `handle.close().await` would never log out. A default body keeps `MockVimClient` compiling (SC-004).

**Alternatives considered**:

- **Inherent `Client::close` only** — **rejected** (`Arc<dyn VimClient>` cannot call it without downcast).
- **Required trait method, no default** — **rejected** (compile break for mocks / third-party impls; FR-008).
- **Consuming `close(self)` / `Arc::try_unwrap`** — **rejected** (NFR-003; clones always exist).

---

## R2 — Session-ended flag vs taking the JSON key first

**Decision**: Add `session_ended: AtomicBool` (false until close or Drop logout starts). `close`:

1. `compare_exchange(false, true)` — if already true, return `Ok(())`.
2. Perform logout **while credentials are still present** (JSON header still in `session_key`; SOAP cookies still on `reqwest::Client`).
3. JSON: `session_key.write().await.take()`.
4. Return the logout `Result` (network/HTTP errors propagate; session stays marked ended).

`Drop` returns immediately if `session_ended` is true (or there is no session to end).

**Rationale**: FR-003/FR-007 and the “failed logout still ends locally” edge case. Taking the JSON key *before* logout would strip the auth header from `invoke`/`Logout`. SOAP has no optional key — cookies remain — so a flag is required anyway. One flag on both transports keeps Drop/`close` races uniform.

**Alternatives considered**:

- **JSON: `take()` key then raw HTTP with the taken value** — works for JSON only; SOAP still needs a flag; two protocols.
- **`tokio::sync::Mutex<()>` around logout** so the second `close` awaits the first — nicer “join” but more lock surface; spec allows “already-ended”; CAS + `Ok(())` is enough.

---

## R3 — Concurrent `close`

**Decision**: At most one logout HTTP request. The CAS loser returns `Ok(())` without waiting.

**Rationale**: Spec edge case: no panic, no double logout. `Ok` on the loser matches “already ended.” Waiters do not need the first call’s error.

**Alternatives considered**:

- **Broadcast the first error** — extra channels; not required.
- **Second call returns a dedicated `AlreadyClosed` error** — noisier for `close(); close();` shutdown.

---

## R4 — Drop runtime policy (0.6.1 compatibility)

**Decision**: Shared helper (JSON + SOAP `Drop`):

```text
if session_ended || !has_session → return
match Handle::try_current() {
  Ok(h) if MultiThread → session_ended=true; block_in_place(|| h.block_on(logout_session))
  Ok(_) CurrentThread  → warn!("… call Client::close().await …"); return
  Err(_)               → warn!(…); return   // no runtime (e.g. process teardown)
}
```

Do **not** call `Handle::current()` (panics if none). Do **not** `block_in_place` on `current_thread`.

**Rationale**: FR-005 vs FR-006. Today’s Drop always `block_in_place` + `Handle::current()`, which panics on single-thread runtimes and with no runtime. Existing wire tests use `#[tokio::test]` (multi-thread) and keep passing. 0.7.0 deletes the `MultiThread` branch (R8).

**Alternatives considered**:

- **`tokio::spawn` from Drop** — **rejected** (often never runs at runtime shutdown; origin of snippet sleeps).
- **Always warn, never Drop-logout in 0.6.1** — **rejected** (FR-005 / 0.6.0 parity).
- **Cargo feature `logout-on-drop`** — **rejected** (NFR-001).

---

## R5 — Reuse logout HTTP (wire denylist)

**Decision**: Move the existing Drop HTTP (JSON path + SOAP envelope Logout, including `kind=logout` wire lines and SessionManager denylist) into `async fn logout_session(&self)` (or free fn taking cloned handles) called from `close` and from Drop’s `block_on`. Do not route 0.6.1 Drop through `SessionManager::logout` on `&self` after clearing state; keep explicit header/cookie behavior identical to 0.6.0.

**Rationale**: FR-012; existing `json_drop_logout_*` / `soap_drop_logout_*` tests stay valid. `close` must emit the same denylisted wire lines.

**Alternatives considered**:

- **`SessionManager::new(arc).logout().await` from `close` only** — elegant but Drop cannot hold `Arc<Self>` easily and would duplicate wire behavior unless invoke paths log identically; extract one HTTP helper instead.

---

## R6 — Tests

**Decision**:

| Test | Runtime | Assert |
|------|---------|--------|
| Existing Drop logout wire tests | default `#[tokio::test]` (multi-thread) | unchanged `kind=logout` |
| New: `close` success / HTTP error / transport error | multi-thread | same wire lines as Drop |
| New: `close` then drop | multi-thread | **one** logout, not two |
| New: `close` twice | multi-thread | **one** logout |
| New: Drop with live session | `flavor = "current_thread"` | **no panic**; no `block_in_place`; optional warn capture |
| New: `close` then Drop | `current_thread` | silent Drop |
| Existing `MockVimClient` tests | any | still compile without `close` override |

Reuse the localhost HTTP stub pattern already used for Drop wire tests (background OS thread; comment today warns about Drop + `block_on` deadlocks).

**Rationale**: SC-001–SC-004. `rt-multi-thread` already enables `rt`, so `current_thread` tests work without a Cargo.toml tokio feature change.

**Alternatives considered**:

- **Only document current_thread** — weaker than an automated no-panic test.

---

## R7 — Examples and docs

**Decision**:

- Every snippet that `connect()`s calls `client.close().await?` on the success (and practical error) exit path.
- Remove yield-for-drop `sleep` in `vm_rename` and `vm_toggle_wol`.
- In-tree `examples/vtui`: `cache_manager.destroy().await?` then `client.close().await?` (FR-011 order).
- `TaskTracker` / `CacheManager` / `Monitor` rustdoc: stop waiters / `destroy` / `shutdown` **then** `close`; no new APIs.
- `README.md`, crate rustdoc, `vim_rs/docs/CLIENT.md` §5: recommend `close`; destructor logout is 0.6.x fallback, gone in 0.7.0.
- CHANGELOG `[Unreleased]` → 0.6.1: **Added** / **Deprecated**, not **Breaking**.

**Rationale**: FR-009–FR-011, SC-005/SC-006. Standalone vtui repo is out of scope (spec).

**Alternatives considered**:

- **Close inside `snippets::connect` via a guard type** — extra API; binaries should show the call.
- **Join `TaskTracker::shutdown` in 0.6.1** — spec out of scope.

---

## R8 — 0.7.0 (design constraint, not this PR)

**Decision**: Keep `fn close(&self)` unchanged. 0.7.0 `Drop` only warns if `!session_ended && has_session`. Then `vim_rs` may drop the `tokio` **`rt-multi-thread`** feature if nothing else needs it (consumers supply the runtime). No feature flag to carry forward.

**Rationale**: FR-013 / SC-007.

**Alternatives considered**: — (specified).
