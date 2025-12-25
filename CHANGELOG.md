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

### Changed

- **`ObjectCacheListener` now returns `CacheAction` from `on_new/on_update`** so listeners can request immediate eviction of objects (triggering `on_remove(T)` with ownership).

### Removed

- **`SharedRefCacheProxy`**: removed because `CacheManager` caches must be `Send + Sync` (used from async tasks and may move between threads). Use `ReadWriteCacheProxy` (`Arc<RwLock<_>>`) or pass `ObjectCache<T>` directly.

### Fixed

- Examples updated to use thread-safe cache patterns required by `CacheManager` (`Send + Sync`).


