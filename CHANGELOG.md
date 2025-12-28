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

### Changed

### Deprecated

### Removed

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


