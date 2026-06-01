# Quickstart: Implementing metadata-guided XML deserialization

**Feature**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

For maintainers landing this work in `vim_rs` + `vim_build`.

## Prerequisites

- Read **`vim_rs/docs/METADATA_DRIVEN_XML_DE.md`** (detailed decision table and edge cases).
- Branch **`001-metadata-xml-de`** (or your working branch).

## Implementation order (recommended)

1. **`vim_build` — `DataTypeAware`**  
   Emit trait + impls for the same type surface as method return types need. Verify **`cargo check -p vim_build`**.

2. **Regenerate `vim_rs`**  
   Run **`vim_build`** generator per **`CLAUDE.md`** so **`types/`** and **`mo/`** pick up new items.

3. **`vim_rs::core::client`**  
   Add conditional **`DataTypeAware`** bounds on **`unmarshal`**, **`unmarshal_array`**, SOAP helpers.

4. **`vim_rs/src/xml/de.rs`**  
   Rewrite descent to use **`ApiFieldType`**; delete probe-first leaf handling; add **`tracing`** on error paths (scoped to **`wire::soap`**).

5. **Tests**  
   **`cd vim_rs && cargo test --features xml`**; add/adjust fixtures for negative cases (unknown field, bad **`xsi:type`**).

6. **Docs**  
   Rustdoc on **`DataTypeAware`** and entry points; **`CHANGELOG.md`** entry for generic bound changes under **`xml`**.

7. **Mandatory probe removal (FR-014 / SC-003)** — **completion gate**  
   Confirm **`vim_rs/src/xml/de.rs`** contains **no** `deliver_text`, **`try_seq_or_single`**, **`stream_drive_legacy_probe`**, or **`stream_children`** (removed); boxed **`Any`** arrays use **`stream_drive_any_array_typed`**. Run **`cargo test -p vim_rs --features xml`** and **`cargo check -p vim_rs --all-features`**. **`METADATA_DRIVEN_XML_DE.md`** documents zero probing.

## Commands (reference)

```bash
cd vim_build
cargo run --bin generate --release

cd ../vim_rs
cargo test --features xml
cargo check --all-features
```

## Definition of done (preview)

- **Zero** probe-first patterns in **`vim_rs::xml::de`** (**SC-003**, **FR-014**): no legacy probe drivers or seq-vs-single guessing.
- Constitution release gates acknowledged (build time / vtui size / transport parity where applicable).

Full task breakdown: run **`/speckit-tasks`** → **`tasks.md`**.
