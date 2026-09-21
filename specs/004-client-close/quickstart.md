# Quickstart: Client `close` (0.6.1)

**Feature**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

For implementers on branch **`004-client-close`**.

## Prerequisites

- Read [research.md](./research.md) (R1–R8) and [contracts/README.md](./contracts/README.md)
- Rust toolchain as in `.cursor/rules/rust-env.mdc` when running cargo from the agent

## Implementation order

1. **`VimClient::close`** default no-op + rustdoc (recommend `close`; Drop fallback → 0.7.0).
2. **`Client::close`** inherent + **`impl VimClient for Client`** forward to `inner`.
3. **`JsonClient`**: `session_ended`, extract `logout_session`, `close`, Drop policy (R4).
4. **`SoapClient`** (`xml`): same pattern (`xml/client.rs`).
5. **Tests** in existing wire-log module: `close` paths; close-then-drop; `current_thread` Drop no panic.
6. **Docs**: `CLIENT.md` §5, README sample, CHANGELOG Added/Deprecated, TaskTracker + CacheManager rustdoc.
7. **Examples**: snippets `close().await`; delete yield sleeps; `examples/vtui` destroy then close.

## Consumer snippet (0.6.1)

```rust
let client = ClientBuilder::new("vcenter.example.com")
    .basic_authn(user, pass)
    .build()
    .await?;

// … work, including clones …

client.close().await?;
```

`VimClientHandle`:

```rust
client.close().await?; // trait method; Client facade must delegate
```

Shutdown with caches:

```rust
cache_manager.destroy().await?;
client.close().await?;
```

## Commands (reference)

```bash
RUSTUP_HOME=/Users/kiril/.rustup CARGO_HOME=/Users/kiril/.cargo \
  /Users/kiril/.cargo/bin/cargo test -p vim_rs

RUSTUP_HOME=/Users/kiril/.rustup CARGO_HOME=/Users/kiril/.cargo \
  /Users/kiril/.cargo/bin/cargo test -p vim_rs --all-features

# After examples: grep that yield-for-drop sleeps are gone
rg "Yield to run async drop" examples/snippets
```

## Definition of done (preview)

- `MockVimClient` compiles without a `close` method.
- Multi-thread Drop logout tests still pass; `close` tests show one logout; `current_thread` Drop does not panic.
- CHANGELOG has no Breaking section for this work.
- Snippets call `close`; in-tree vtui calls `close` after `destroy`.

Full tasks: **`/speckit-tasks`** → `tasks.md`.
