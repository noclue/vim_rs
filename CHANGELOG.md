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

## [Unreleased]

### Added

- **Serde benchmarks**: Added `vim_rs/benches/serde_bench.rs` to measure serialization/deserialization performance.
- **Serde tests**: Added `vim_rs/tests/serde_test.rs` to verify enum serialization round-trips.

### Changed

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


