# Quickstart: Third-party dependency refresh

**Feature**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

For implementers on branch **`005-bump-third-party-deps`**.

## Prerequisites

- Read [research.md](./research.md) (R1–R8) and [contracts/README.md](./contracts/README.md).
- When cargo runs under the Cursor agent sandbox, prefix it as in `.cursor/rules/rust-env.mdc`:

```bash
RUSTUP_HOME=/Users/kiril/.rustup CARGO_HOME=/Users/kiril/.cargo /Users/kiril/.cargo/bin/cargo <args>
```

Commands below use plain `cargo` for a normal shell.

There is no root workspace. Run each step in the package root it names.

## 1. Compatible refresh

In each root, update the lockfile only. Do not edit manifests in this step.

```bash
for d in vim_rs vim_macros vim_build openapi30 tls_rustls_only examples mcp; do
  (cd "$d" && cargo update)
done
```

Confirm the named compatible moves:

```bash
grep -n -A1 -E 'name = "(rmcp|rmcp-macros|anyhow)"' mcp/Cargo.lock examples/Cargo.lock
```

Expect **rmcp 3.5.1**, **rmcp-macros 3.5.1**, **anyhow 1.0.104** (or a newer 3.5.x / 1.0.x if one was published after 2026-10-15). quick-xml stays **0.40.1** until step 3.

## 2. Prove the compatible refresh builds

```bash
(cd vim_rs && cargo test)
(cd vim_macros && cargo test)
(cd vim_build && cargo test)
(cd openapi30 && cargo test)
(cd tls_rustls_only && cargo build)
(cd examples && cargo build)
(cd mcp && cargo build -p vim_mcp_server)
```

`vim_mcp_server` needs the local API database the MCP build already requires. Do not regenerate bindings and do not commit `mcp/data/`.

If a package fails to compile, or a test shows different parsed VIM/JSON values:

```bash
cargo update -p <crate> --precise <previous-version>
```

Record the crate, the rejected version, and the reason for the changelog. Do not widen that crate's declaration.

## 3. quick-xml 0.42

In `vim_rs/Cargo.toml`, set:

```toml
quick-xml = { version = "0.42", optional = true }
```

Then:

```bash
(cd vim_rs && cargo update -p quick-xml)
(cd examples && cargo update -p quick-xml)
```

Migrate call sites in `vim_rs/src/xml/de.rs` and `vim_rs/src/xml/client.rs`:

- Name and local-name `as_ref()` is `&str`. Delete `std::str::from_utf8(...)`.
- Compare local names to `"type"` and `"nil"`, not `b"type"` and `b"nil"`.
- Replace `BytesText::decode()` and `from_utf8(&event)` with `as_ref()` / deref to `str`.
- Keep `normalized_value(XmlVersion::Implicit1_0)`.
- Keep `quick_xml::escape::escape` in `ser.rs`.

Do not edit files under `vim_rs/src/types/` or `vim_rs/src/mo/`.

## 4. XML and full tests

```bash
(cd vim_rs && cargo test --features xml)
(cd vim_rs && cargo test --features xml vcsim_xml)
(cd vim_rs && cargo test --all-features)
```

`vcsim_xml` must keep the same VIM values for `_XMLSchema-instance:type` and `xsi:type` fixtures.

`soap_integration` stays ignored:

```bash
# only if step 4 shows parsed XML values changed and the bump is still in play
source .govc_env && cargo test --features xml --test soap_integration -- --ignored
```

Run this from `vim_rs/`.

Re-run the step 2 builds for `examples` and `mcp` after the quick-xml lockfile update. If rmcp 3.5 fails in `mcp/server/src/main.rs`, fix that file. Change `rmcp` to `"3.5"` only when the fix needs an API that 3.4 does not have.

## 5. Changelog

Under `## [Unreleased]` in `CHANGELOG.md`:

- **Dependencies**: quick-xml 0.40.1 → 0.42.0, rmcp 3.4.0 → 3.5.1, anyhow 1.0.103 → 1.0.104, plus one sentence for the other lockfile updates.
- **Security**: quick-xml 0.42 nesting / namespace-depth fix.
- **Exceptions**: only the precise-pins from step 2.

Do not bump `version` in `vim_rs/Cargo.toml` or `vim_macros/Cargo.toml`.

## Done when

- All seven roots build, and the tests in steps 2 and 4 pass.
- Lockfiles resolve quick-xml 0.42.0, rmcp 3.5.1, anyhow 1.0.104 (or newer adoptable patches).
- Every `anyhow = "1.0"` package shares that anyhow release.
- `vim_rs` version is still 0.6.0.
- Changelog matches [contracts/README.md](./contracts/README.md).
