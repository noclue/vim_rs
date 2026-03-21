# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## How we maintain this

- **One entry per release**: `## x.y.z - YYYY-MM-DD`
- **Unreleased goes first**: changes that will ship next live under `## [Unreleased]`
- **Categorize changes**:
  - **Added**: new features
  - **Changed**: behavior changes (non-breaking)
  - **Deprecated**: features slated for removal
  - **Removed**: features removed
  - **Fixed**: bug fixes
  - **Security**: security fixes
- **Call out breaking changes explicitly** (especially while pre-1.0).

## [UNRELEASED]

## [0.4.1] - 2026-03-21

### Added

- Experimental opt-in `xml` cargo feature for SOAP/XML transport against the VIM APIs.
- `ClientBuilder::transport(TransportMode)` to select `Json` (default), `Soap`, or `Auto`.

### Changed

- Public API remains aligned with `0.4.0`; the only intended user-facing API addition is the
  builder `transport()` option.
- `TransportMode::Auto` first probes the vCenter Hello System JSON API and falls back to SOAP/XML
  when that API is unavailable, which allows direct ESXi connections.
- When XML transport is used, Hello System negotiation is skipped. `client.api_release()` therefore
  reflects the build-time/library release behavior rather than a negotiated remote capability level.
  Use `client.service_content().about.api_version` when making decisions about server capability.
- Enabling `xml` increases release binary size by about 500 KB and increases debug build times by
  roughly 30-40%. Disabling the feature returns functionality, build times, and executable sizes to
  `0.4.0` levels.
- The XML changes also affect polymorphic JSON deserialization: if you manually deserialize such
  JSON while the `xml` feature is enabled, `_typeName` must appear before subtype fields.
- **Examples workspace** (`examples/`): consolidated runnable samples into the `snippets` crate.
  The former `macro_examples` binaries (`retrieve_host_info`, `retrieve_ds_hosts`, `vm_events`,
  `print_vm_addresses`, `retrieve_recent_task`) are now `snippets` binaries; the shared
  `connect()` helper lives in `snippets` as a small library (`snippets::connect`). Removed the
  separate `utils` and `macro_examples` workspace members. Run with
  `cargo run -p snippets --bin <name>` from `examples/`; see `examples/README.md` for the full list.

### Known limitations

- XML transport currently supports only the VIM APIs. Other APIs such as VSAN, SPBM/PBM, SMS, VSLM,
  and EAM will return errors when used over XML transport.
- XML support is currently an experimental hack. If it fails, enable `trace` logging for `vim_rs`
  and capture the failing request/response packets for debugging.

## [0.4.0] - 3.3.2026: The efficincy overhaul

### Added

- **`defaults` cargo feature** (opt-in): `Default` trait implementations for all generated structs, enums, and `Box<dyn Trait>` types.
  - Enables struct update syntax (`..Default::default()`) for concise construction, replacing verbose `None` fields.
  - Enum defaults use the first variant.
  - Trait object defaults (e.g., `Box::<dyn VirtualDeviceTrait>::default()`) create the base struct.
  - Enable with `vim_rs = { features = ["defaults"] }` in Cargo.toml.

- **MCP tools**: Updated API documentation and starter guide to align with vim_rs compositional inheritance, trait getter removal, and enum changes.
  - Traits now show "Fields (accessible via Deref)" instead of removed getter methods; usage examples use direct field access (`device.key`, `eth.mac_address`).
  - Structs document parent fields (e.g., `selection_spec_: SelectionSpec`) required for construction when the parent has meaningful fields.
  - Enums document `Other_` variant and `as_str()` / `from_str()` string conversion.
  - Managed objects note the distinction between property accessors and `*_task` methods.
  - Starter guide: new Step 3.6 "Compositional Inheritance — How Structs and Traits Use Deref" with parent field convention, construction examples, and multi-level Deref chain; Common Mistakes entry for parent field construction.
  - Added `mcp/MCP_GAPS.md` with gap analysis for implementer reference.

- **Serialization benchmarks**: Added `vim_rs/benches/miniserde_bench.rs` to measure miniserde serialization/deserialization performance.
- **Serialization tests**: Added `vim_rs/tests/serde_test.rs` to verify enum serialization round-trips with miniserde.

- **`vim_rs::vim_retrievable!` and `vim_rs::vim_updatable!` re-exported from `vim_rs`**: The
  `vim_macros` macros are now available directly via `use vim_rs::vim_retrievable;` and
  `use vim_rs::vim_updatable;`. Users no longer need a separate `vim_macros` dependency in
  `Cargo.toml` — `vim_rs = "0.4"` is sufficient.
  - `vim_macros` can be removed from `[dependencies]` in user crates.
  - The `vim_macros` crate remains published separately for users who prefer explicit imports.

- **Helper utilities for miniserde** (`vim_rs::types::mini_helpers`):
  - `from_value(v: miniserde::json::Value) -> Result<T>` — deserializes a `miniserde::json::Value` into any `T: miniserde::Deserialize`; equivalent to `serde_json::from_value`.
  - `Base64` — a newtype wrapper around `Vec<u8>` that serializes/deserializes as a Base64-encoded JSON string, required because miniserde has no built-in bytes type.
  - `replay_value_to_visitor()` — low-level helper that feeds a parsed `miniserde::json::Value` to an arbitrary miniserde `Visitor`; useful when you need to deserialize a value whose concrete type is not known at parse time.
- **`ValueElements::as_str()`**: Returns the VIM API type name as it appears in the OpenAPI specification.
  - Useful for type discrimination, logging, and debugging when working with `VimAny` and dynamic property values.
  - Example: `ValueElements::PrimitiveString(_)` returns `"string"`, `ValueElements::ArrayOfManagedObjectReference(_)` returns `"ArrayOfManagedObjectReference"`.

### Changed

- **BREAKING: Replaced `serde`/`serde_json` with `miniserde` for all JSON serialization**.
  - All generated types now derive `miniserde::Serialize` and `miniserde::Deserialize` instead of the `serde` equivalents.
  - **Dependencies removed**: `serde`, `serde_json`, `erased-serde`.
  - **Dependencies added**: `miniserde`.
  - Affects all types in `vim_rs::types` (structs, enums, trait objects).
  - **Performance benefits**: faster compilation, smaller binary sizes, and reduced LLVM IR complexity.
  - **Migration guide** for users upgrading from 0.3.0:
    - Replace `use serde::{Serialize, Deserialize}` with `use miniserde::{Serialize, Deserialize}`.
    - Replace `serde_json::from_str(&s)` with `miniserde::json::from_str(&s)`.
    - Replace `serde_json::to_string(&v)` with `miniserde::json::to_string(&v)`.
    - Replace `serde_json::from_value(v)` with `vim_rs::types::mini_helpers::from_value(v)`.
    - `miniserde` supports only a subset of the serde data model (no `#[serde(rename)]`, no flattening, no untagged enums). vim_rs generated types are already adapted to miniserde's constraints.

- **BREAKING: Compositional inheritance using Deref/DerefMut**.
  - Child structs no longer have parent fields expanded inline. Instead, they contain a single parent field (e.g., `virtual_ethernet_card_: VirtualEthernetCard`).
  - Child structs implement `Deref` and `DerefMut` to their parent, providing seamless field access.
  - Access parent fields directly: `device.key` instead of the old expanded field pattern.
  - This change significantly reduces generated code size while maintaining ergonomic field access.
  - **Migration**: Field access patterns remain the same due to Deref coercion. No code changes needed for read access.

- **BREAKING: Removed trait getter methods (`get_*`)**.
  - Trait getter methods like `get_key()`, `get_mac_address()`, `get_backing()` are removed.
  - Use direct field access via Deref instead: `device.key`, `eth.mac_address`, `device.backing`.
  - **Migration**: Replace `obj.get_field()` with `obj.field` or `&obj.field` for references.
  - Example: `eth.get_mac_address()` → `&eth.mac_address`

- **BREAKING: Removed DataObject trait methods**.
  - The `DataObjectTrait` no longer has getter methods.
  - Empty descendant types (those with no additional fields beyond their parent) are pruned from the hierarchy.
  - This simplifies the type hierarchy and reduces code size.

- **BREAKING: Enum `as_str()` now returns `&str` with instance lifetime** instead of `&'static str`.
  - This enables `as_str()` to return the actual string value held in `Other_` variants.
  - Previously, `Other_` variants would return the placeholder string `__OTHER__`.
  - Now, `as_str()` returns the actual runtime value, providing much better debugging and error messages.

- **BREAKING: Removed `From<EnumType> for &'static str` implementations**.
  - Use `.as_str()` method instead of `.into()` or `From::from()` for string conversion.
  - Example: `my_enum.as_str()` instead of `my_enum.into()`.
  - this applies to the `ValueElements` enum too.

- **Performance: Replaced macro-based enum derives with PHF (Perfect Hash Function) maps**.
  - Removed `serde::Deserialize`, `serde::Serialize`, and `strum_macros::IntoStaticStr` derives from all ~300+ enums.
  - Manual implementations use PHF maps for O(1) string-to-enum lookups.
  - PHF maps are generated at code-generation time using `phf_codegen`, eliminating compile-time macro expansion overhead.
  - Expected compile time improvements: 5-10% for incremental builds, 3-5% for clean builds.
  - Expected LLVM lines reduction: 20,000-40,000 lines from enum deserialize implementations.

- **Code generation: New `enum_impls.rs` module**.
  - Generates individual PHF maps for each enum (e.g., `MO_TYPES_ENUM_MAP`, `ENTITY_REFERENCE_ENTITY_TYPE_ENUM_MAP`).
  - Generates manual `Serialize`, `Deserialize`, `Display`, and `Debug` implementations.
  - Each enum's PHF map provides constant-time lookups for string-to-variant conversion.

- **Examples updated** to use direct field access instead of trait getter methods and `.as_str()` API.

### Deprecated

### Removed

- **BREAKING: `serde`, `serde_json`, and `erased-serde` dependencies removed**.
  - Replace with `miniserde` and `vim_rs::types::mini_helpers` as described in the Changed section above.

- **BREAKING: `strum_macros` derive removed from all enums**.
  - `#[strum(serialize = "...")]` attributes no longer used.
  - String conversions now handled by generated PHF maps and manual implementations.

- **BREAKING: Trait getter methods removed** (see Changed section for migration).

### Fixed

### Security

## [0.3.0] - 2025-12-28

### Added

- **`TaskTracker`**: High-level API for awaiting vSphere `Task` completion using PropertyCollector.
  - Efficient background monitoring with shared `ListView` and incremental updates.
  - Two APIs: `wait::<T>()` for convenient deserialization and `wait_any()` for zero-allocation path.
  - Automatic resource management: background loop starts lazily and stops when all tasks complete.
  - Memory efficient: tasks are evicted from cache immediately upon reaching terminal state.
  - See `examples/snippets/src/vm_rename.rs` and `vim_rs/tests/task_tracker_integration.rs`.

### Changed

- **BREAKING: Managed object stubs now use `VimClient` dynamic trait** instead of concrete `Client` type.
  - Stubs store `Arc<dyn VimClient>` internally, allowing for easier testing and mocking.
  - The concrete `Client` implements `VimClient`, so existing code using `client.clone()` continues to work.
  
- **BREAKING: `ObjectCacheListener` now returns `CacheAction` from `on_new/on_update`**.
  - Listeners can request immediate eviction (`CacheAction::Evict`) to take ownership of cached objects via `on_remove(T)`.
  - Useful for objects that won't change after reaching a terminal state (e.g., completed Tasks).
  - ⚠️ **Warning**: Evicting an object that later receives updates from the server will cause an error.

### Removed

- **BREAKING: `SharedRefCacheProxy`** removed.
  - `CacheManager` requires caches to be `Send + Sync` (used from async tasks, may move between threads).
  - Use `ReadWriteCacheProxy` (`Arc<RwLock<_>>`) or pass `ObjectCache<T>` directly instead.

### Fixed

- Examples updated to use thread-safe cache patterns required by `CacheManager` (`Send + Sync`).
