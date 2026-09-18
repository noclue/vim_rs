# Quickstart: vSphere 9.1.0.0 binding update

**Feature**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

Ordered validation runbook for implementers on branch **`003-vsphere-910-update`**.

## Prerequisites

- Branch **`003-vsphere-910-update`**
- `vim_build/data/vi_json_openapi_specification_v9_1_0_0.json` present (user-supplied)
- Rust **1.66+** (required by `phf` 0.14)
- Read [research.md](./research.md) for path updates, dependency matrix (R10–R16), and triage policy
- Optional: NVIDIA CUDA for faster MCP embedding generation (`--features cuda`)

## Implementation order

1. **Pre-flight** — validate OpenAPI JSON (Step 1).
2. **Path wiring** — update 5 hard-coded spec paths ([research R4](./research.md#r4--hard-coded-spec-path-updates)).
3. **PHF alignment** — bump `phf_codegen` 0.14 (`vim_build`) + `phf` 0.14 (`vim_macros`, `vim_rs`, `mcp/server`) before regen (Step 2a).
4. **Regenerate** — `vim_build` generator run (Step 2b).
5. **Remaining dependency majors** — `bincode` 3, `tera` 2, `quick-xml` 0.40, `criterion` 0.8, `convert_case` 0.11, `check_keyword` 0.4 + source migrations (Step 2c).
6. **Minor/patch refresh** — `cargo update` per crate; commit lockfiles (Step 2d).
7. **Version bump** — `vim_rs`, `vim_macros`, example path deps → **0.6.0**; CHANGELOG; purge **0.5.1** (Step 6).
8. **MCP local rebuild** — data-transformer + server build (Step 4) for manual testing.
9. **Validate** — compile/test matrix (Step 3).
10. **Release notes** — CHANGELOG majors enumerated + minor summary + build metrics (Step 5).
11. **Idempotency** — re-run generator, expect no diff (Step 2b).
12. **Audits** — `cargo outdated`, semver, stale 9.0.x refs (Steps 6–8).

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

## Step 2a — PHF 0.14 alignment (before regeneration)

Update manifests:

| File | Change |
|------|--------|
| `vim_build/Cargo.toml` | `phf_codegen = "0.14"` |
| `vim_macros/Cargo.toml` | `phf = "0.14"` |
| `vim_rs/Cargo.toml` | `phf = "0.14"` |
| `mcp/server/Cargo.toml` | `phf = "0.14"` |

```bash
cd vim_build && cargo update -p phf_codegen && cd ..
cd vim_macros && cargo update -p phf && cd ..
cd vim_rs && cargo update -p phf && cd ..
cd mcp && cargo update -p phf && cd ..
```

## Step 2b — Regenerate bindings

```bash
cd vim_build
cargo run --bin generate --release
```

**Expected outputs** (large diffs):

- `vim_rs/src/types/*`, `vim_rs/src/mo/*`
- `vim_macros/src/field_data.rs`
- `mcp/server/src/field_data.rs`

**Idempotency** (SC-005, SC-014):

```bash
cd vim_build
cargo run --bin generate --release
cd ..
git diff --stat vim_rs/src vim_macros/src/field_data.rs mcp/server/src/field_data.rs
# Expected: no changes
```

## Step 2c — Remaining major dependency migrations

See [research R11–R15](./research.md). After manifest bumps, fix compile errors in:

- `mcp/server/src/model.rs`, `mcp/data_processing/data_transformer/src/main.rs` (`bincode` 3)
- `mcp/server/src/web_ui/handlers.rs` (`tera` 2, `--features web-ui`)
- `vim_build/src/vim_model/*`, emitters (`convert_case`, `check_keyword`)
- `vim_rs` XML/bench code (`quick-xml`, `criterion`)

```bash
cd vim_build && cargo test
cd ../vim_rs && cargo build --all-features
cd ../mcp && cargo build -p vim_mcp_server --features web-ui
```

## Step 2d — Minor/patch refresh + lockfiles

```bash
# Per in-scope crate (examples from research R16)
cd vim_rs && cargo update && cargo outdated --root-deps-only
cd ../vim_macros && cargo update && cargo outdated --root-deps-only
cd ../vim_build && cargo update && cargo outdated --root-deps-only
cd ../openapi30 && cargo update && cargo outdated --root-deps-only
cd ../examples && cargo update && cargo outdated --root-deps-only
cd ../tls_rustls_only && cargo update && cargo outdated --root-deps-only
cd ../mcp && cargo update && cargo outdated --root-deps-only
```

**SC-013 check** — single `phf` major in `vim_rs` lock:

```bash
rg '^name = "phf"' -A1 vim_rs/Cargo.lock
# Expected: only 0.14.x entries
```

Commit all updated `Cargo.lock` files (FR-020).

## Step 3 — Compile & test matrix

```bash
# Core crates
cd vim_rs && cargo build && cargo test
cd ../vim_macros && cargo test
cd ../vim_build && cargo test
cd ../openapi30 && cargo test
cd ../tls_rustls_only && cargo build

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

> **Note**: `vim_mcp_server` requires Step 4 first on a fresh clone (no `mcp/data/api_database.bin`).

## Step 4 — MCP rebuild (local artifact)

`mcp/data/` is **gitignored** (see repo `.gitignore`). `api_database.bin` is **not committed** — regenerate locally for manual MCP testing after spec or dependency changes (including `bincode` 3 migration).

```bash
cd mcp
cargo run -p data-transformer --release --features cuda
cargo build --release -p vim_mcp_server
```

If `build.rs` reports a missing or stale database, re-run data-transformer. A fresh clone will not contain `mcp/data/api_database.bin` until this step is run.

## Step 5 — Build metrics (SC-008)

Capture for CHANGELOG comparison vs 0.5.0:

```bash
# Example: release binary size (Windows)
dir examples\vtui\target\release\vtui.exe

# Use cargo build --timings for compile time comparison if needed
cd vim_rs && cargo build --timings
```

## Step 6 — Stale 9.0.x reference audit (FR-014)

```bash
rg '9_0_0|9\.0\.0\.0|vi_json_openapi_specification_v9_0' \
  --glob '!vim_rs/src/types/**' --glob '!vim_rs/src/mo/**' \
  --glob '!vim_build/data/vi_json_openapi_specification_v9_0_0_0_24798170.json'
```

**Must update**: `vim_rs/src/core/client.rs` (`API_RELEASE`), `README.md`, `CLAUDE.md`, `mcp/README.md`.

## Step 7 — Semver artifact audit (FR-015, SC-011)

```bash
rg '0\.5\.1' --glob '!**/Cargo.lock'
```

**Must show 0.6.0** in `vim_rs/Cargo.toml`, `vim_macros/Cargo.toml`, example path deps, `CHANGELOG.md` `[0.6.0]`, `mcp/README.md`.

## Step 8 — Dependency freshness audit (SC-012)

```bash
cargo outdated --root-deps-only   # run in each in-scope crate directory
```

Zero outdated direct dependencies, or document exceptions in `research.md`.

## Failure triage checklist (FR-013)

When generator, compile, or tests fail:

1. **Spec** — JSON valid? Enum string values correct? YAML→JSON coercion?
2. **Dependencies** — PHF aligned before regen? `bincode`/`tera` migrations complete?
3. **Generator** — only after spec confirmed.
4. **Downstream** — sample fixes for API symbols; lockfile sync.
5. **Document** — root-cause class in CHANGELOG or PR notes (SC-009).

## Definition of done (preview)

- [ ] Generator points at 9.1.0.0 JSON only
- [ ] `phf` 0.14 aligned; single major in `vim_rs/Cargo.lock`
- [ ] All generated files from clean run; re-run produces zero diff
- [ ] Full dependency refresh; `cargo outdated` clean (SC-012)
- [x] `vim_rs` + `vim_macros` at **0.6.0** (not 0.5.1)
- [ ] Full compile/test matrix passes (core + examples + tls_rustls_only)
- [ ] MCP database rebuilt locally; `vim_mcp_server` builds (Step 4)
- [ ] CHANGELOG `[0.6.0]` with API breaks, **major dep old→new lines**, minor summary
- [ ] Build metrics recorded
- [x] No `0.5.1` release-target references in manifests/docs (SC-011)

Full task breakdown: **`/speckit-tasks`**.
