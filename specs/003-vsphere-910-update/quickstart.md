# Quickstart: vSphere 9.1.0.0 binding update

**Feature**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

Ordered validation runbook for implementers on branch **`003-vsphere-910-update`**.

## Prerequisites

- Branch **`003-vsphere-910-update`**
- `vim_build/data/vi_json_openapi_specification_v9_1_0_0.json` present (user-supplied)
- Read [research.md](./research.md) for path updates and triage policy

## Implementation order

1. **Pre-flight** — validate OpenAPI JSON (see Step 1 below).
2. **Path wiring** — update 5 hard-coded spec paths to 9.1.0.0 filename ([research R4](./research.md#r4--hard-coded-spec-path-updates)).
3. **Regenerate** — `vim_build` generator run (Step 2).
4. **Version bump** — `vim_rs`, `vim_macros`, example path deps → **`0.6.0`**; retitle CHANGELOG; purge **0.5.1** (FR-015).
5. **MCP rebuild** — data-transformer + server build (Step 4).
6. **Validate** — compile/test matrix (Step 3).
7. **Release notes** — `CHANGELOG.md` `[0.6.0]` + breaking-minor rationale + build metrics.
8. **Idempotency check** — re-run generator, expect no diff on generated files.
9. **Semver audit** — confirm no `0.5.1` release-target references remain (Step 7).

## Step 1 — Pre-flight spec validation

```bash
# Confirm file exists
ls vim_build/data/vi_json_openapi_specification_v9_1_0_0.json

# Spot-check DatastoreAccessible_enum (must be strings, not booleans)
grep -A5 '"DatastoreAccessible_enum"' vim_build/data/vi_json_openapi_specification_v9_1_0_0.json | head -10
# Expected: "enum": [ "True", "False" ]

# Quick parse test via openapi30
cd openapi30 && cargo test
```

**On failure**: inspect `vi-json.yaml` and re-run conversion; fix coercion before touching `vim_build` (FR-013).

## Step 2 — Regenerate bindings

```bash
cd vim_build
cargo run --bin generate --release
```

**Expected outputs** (large diffs):

- `vim_rs/src/types/*`, `vim_rs/src/mo/*`
- `vim_macros/src/field_data.rs`
- `mcp/server/src/field_data.rs`

**Idempotency** (SC-005):

```bash
cd vim_build
cargo run --bin generate --release
cd ..
git diff --stat vim_rs/src vim_macros/src/field_data.rs mcp/server/src/field_data.rs
# Expected: no changes
```

## Step 3 — Compile & test matrix

```bash
# Core crates
cd vim_rs && cargo build && cargo test
cd ../vim_macros && cargo test
cd ../vim_build && cargo test
cd ../openapi30 && cargo test

# All features
cd ../vim_rs && cargo build --all-features && cargo test --all-features

# Examples (all binaries)
cd ../examples && cargo build --bins
cd ../examples && cargo build --bins --all-features

# Release build spot-check
cd ../vim_rs && cargo build --release
cd ../examples/vtui && cargo build --release
```

Fix sample/test source **only** for spec-driven API renames or removals (FR-010). Do not hand-edit generated files.

## Step 6 — Stale 9.0.x reference audit (FR-014)

Search hand-written sources for stale binding-target references:

```bash
# From repo root — review each hit; ignore generated vim_rs/src/types|mo and archived 9.0 spec file
rg '9_0_0|9\.0\.0\.0|vi_json_openapi_specification_v9_0' \
  --glob '!vim_rs/src/types/**' --glob '!vim_rs/src/mo/**' \
  --glob '!vim_build/data/vi_json_openapi_specification_v9_0_0_0_24798170.json'
```

**Must update** (known examples):

- `vim_rs/src/core/client.rs` — `API_RELEASE`, `COMPATIBLE_API_RELEASES`
- `README.md`, `CLAUDE.md`, `mcp/README.md` — binding version and spec filename mentions

**Do not update**: VMware `***Since:*** vSphere API Release 9.0.0.0` in generated rustdoc.

## Step 4 — MCP rebuild

```bash
cd mcp
cargo run -p data-transformer --release --features cuda
cargo build --release -p vim_mcp_server
```

If `build.rs` reports stale database, re-run data-transformer.

## Step 5 — Build metrics (SC-008)

Capture for CHANGELOG comparison vs 0.5.0:

```bash
# Example: release binary size (Windows)
dir examples\vtui\target\release\vtui.exe

# Use cargo build --timings for compile time comparison if needed
cd vim_rs && cargo build --timings
```

## Step 7 — Semver artifact audit (FR-015, SC-011)

Confirm **0.5.1** is not a release target anywhere in manifests or release docs:

```bash
# From repo root — review each hit
rg '0\.5\.1' --glob '!**/Cargo.lock'
```

**Must show 0.6.0** in:

- `vim_rs/Cargo.toml`, `vim_macros/Cargo.toml`
- `examples/snippets/Cargo.toml`, `examples/vtui/Cargo.toml`
- `CHANGELOG.md` — section header `[0.6.0]` (not `[0.5.1]`)
- `mcp/README.md`

**Acceptable hits**: historical `spec.md` Input quote; git history; third-party crate versions in lockfiles (e.g. `socket2 0.5.10`).

## Failure triage checklist (FR-013)

When generator, compile, or tests fail:

1. **Spec** — JSON valid? Known enum string values correct? YAML→JSON coercion issues?
2. **Generator** — only after spec confirmed; check `vim_build` / `openapi30` errors.
3. **Downstream** — sample fixes for renamed/removed API symbols; lockfile sync.
4. **Document** — record root-cause class in CHANGELOG or PR notes (SC-009).

## Definition of done (preview)

- [ ] Generator points at 9.1.0.0 JSON only
- [ ] All generated files from clean run; re-run produces zero diff
- [x] `vim_rs` + `vim_macros` at **0.6.0** (not 0.5.1)
- [ ] Full compile/test matrix passes
- [ ] MCP database rebuilt; `vim_mcp_server` builds
- [x] CHANGELOG `[0.6.0]` with breaking-minor classification and breaking changes
- [ ] Build metrics recorded
- [x] No `0.5.1` release-target references in manifests/docs (SC-011)

Full task breakdown: **`/speckit-tasks`** → refresh `tasks.md` for 0.6.0 semver correction if needed.
