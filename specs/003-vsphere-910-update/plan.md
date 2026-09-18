# Implementation Plan: vSphere 9.1.0.0 API Binding Update

**Branch**: `003-vsphere-910-update` | **Date**: 2026-06-26 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/003-vsphere-910-update/spec.md`

**Note**: Filled by `/speckit-plan`. Workflow: `.specify/templates/plan-template.md`.

## Summary

Mechanically regenerate all `vim_rs` bindings, `vim_macros`/`mcp` field-path metadata, and (locally) the MCP API database from **vSphere 9.1.0.0** OpenAPI input; perform a **full monorepo third-party dependency refresh** (FR-016–FR-020) with **`phf` / `phf_codegen` 0.14** alignment; bump **`vim_rs`** and **`vim_macros`** to **0.6.0**; purge **0.5.1** release-target references; validate compilation and tests. On failure, triage **OpenAPI input correctness first** (FR-013). `mcp/data/api_database.bin` is **gitignored** — regenerate locally per [quickstart.md](./quickstart.md) Step 4 for manual MCP testing (FR-021).

## Technical Context

**Language/Version**: Rust 2021, stable (MSRV effectively **1.66+** after `phf` 0.14 — see [research R10](./research.md#r10--phf-014-alignment))  
**Primary Dependencies**: In-tree `vim_build`, `openapi30`, `vim_macros`; runtime `miniserde`, `tokio`, `reqwest`; **aligned `phf` 0.14** across binding stack  
**Storage**: Generated Rust under `vim_rs/src/`; `vim_macros`/`mcp/server` `field_data.rs`; **local** `mcp/data/api_database.bin` (gitignored)  
**Testing**: `cargo test` in core crates; `cargo build --bins` in `examples/`; MCP via quickstart Step 4 after local DB regen  
**Target Platform**: Cross-platform library (Linux/macOS/Windows)  
**Project Type**: Multi-crate monorepo (generated bindings + tooling)  
**Performance Goals**: No unjustified regression vs 0.5.0 release gates (Principle III)  
**Constraints**: No hand-edits to generated files (Principle I); `PRUNED_TYPES` unchanged unless blocked; full dependency refresh per clarification 2026-06-26  
**Scale/Scope**: ~322K-line OpenAPI JSON; ~200K+ LOC `structs.rs`; PHF maps in `struct_enum.rs`, `deser.rs`, `field_data.rs`; 8+ `Cargo.lock` files; major migrations: `phf` 0.14, `quick-xml` 0.40, `criterion` 0.8, `convert_case` 0.11, `check_keyword` 0.4, `bincode` 3, `tera` 2

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle / standard | Status | Evidence |
|---------------------|--------|----------|
| **I. Generated from authoritative specs** | Pass | Single generator run from 9.1.0.0 JSON; PHF maps regenerated after `phf_codegen` 0.14 — not hand-edited |
| **II. Complete type-safe surface** | Pass | Full regeneration; `PRUNED_TYPES` preserved |
| **III. Build-time & binary budgets** | Pass* | Capture metrics vs 0.5.0; dependency bumps may affect compile time — document in CHANGELOG (*measure in implement) |
| **IV. Multi-transport parity** | Pass* | Compile/test gate; live vCenter deferred per spec assumptions |
| **V. Documented public surfaces** | Pass | CHANGELOG `[0.6.0]` with API breaks + **enumerated major dependency bumps** (FR-011) |
| **VI. Readable source** | Pass | Generator output only |
| **VII. Ecosystem tooling** | Pass | `vim_macros`, MCP pipeline, dependency alignment including `bincode`/`tera` migrations |
| **OpenAPI ingestion** | Pass | Pre-flight `DatastoreAccessible_enum`; FR-013 triage |
| **CHANGELOG discipline** | Pass | Major deps old→new lines; minor/patch summary line; 0.5.1 MUST NOT ship |
| **Marshalling: miniserde** | Pass | No `serde` added to `vim_rs`; `serde` in `vim_build`/`openapi30` unchanged pattern |

**Post-design re-check**: [research.md](./research.md) R10–R16 resolve dependency decisions. **Documented deferral**: live vCenter integration (Principle IV) recommended post-release.

## Project Structure

### Documentation (this feature)

```text
specs/003-vsphere-910-update/
├── plan.md              # This file
├── research.md          # Phase 0 (R1–R16)
├── data-model.md        # Phase 1
├── quickstart.md        # Phase 1 validation runbook
├── contracts/           # Phase 1 release contract
├── spec.md
└── tasks.md             # /speckit-tasks
```

### Source Code (repository root)

```text
vim_build/                    # phf_codegen 0.14; convert_case 0.11; check_keyword 0.4
vim_rs/                       # phf 0.14; quick-xml 0.40; criterion 0.8 (dev)
vim_macros/                   # phf 0.14
mcp/server/                   # phf 0.14; bincode 3; tera 2
mcp/data_processing/data_transformer/  # bincode 3
openapi30/, examples/*, tls_rustls_only/  # minor/patch refresh
mcp/data/api_database.bin     # LOCAL ONLY (.gitignore)
```

**Structure Decision**: **Input-path wiring + dependency refresh + PHF-aligned regeneration + version bump + validation**. Hand-written edits: manifests, lockfiles, migration fixes (`bincode`, `tera`, `quick-xml`, `criterion`, `convert_case`), docs, sample fixes for spec breaks — never generated binding bodies.

## Complexity Tracking

> No constitution violations requiring justification.

| Violation | Why needed | Simpler alternative rejected because |
|-----------|------------|-------------------------------------|
| — | — | — |

---

## Phase 0: Research

**Status**: Complete → [research.md](./research.md)

Resolved decisions R1–R9 (spec bump, paths, MCP) plus **R10–R16** (dependency refresh). No open **NEEDS CLARIFICATION** items.

---

## Phase 1: Design

**Status**: Complete

### Implementation phases (ordered)

#### Phase A — Pre-flight spec validation

1. Confirm `vim_build/data/vi_json_openapi_specification_v9_1_0_0.json` exists and parses.
2. Spot-check `DatastoreAccessible_enum` string values (FR-013).
3. `cd openapi30 && cargo test`.

#### Phase B — Wire generator to 9.1.0.0

Update hard-coded paths `vi_json_openapi_specification_v9_0_0_0_24798170.json` → `vi_json_openapi_specification_v9_1_0_0.json` in five locations ([research R4](./research.md#r4--hard-coded-spec-path-updates)).

#### Phase C — Dependency refresh: PHF alignment first (FR-017)

**Order matters**: bump `phf_codegen` before regenerating PHF-backed output.

| Crate | Manifest change |
|-------|-----------------|
| `vim_build/Cargo.toml` | `phf_codegen = "0.14"` |
| `vim_macros/Cargo.toml` | `phf = "0.14"` |
| `vim_rs/Cargo.toml` | `phf = "0.14"` |
| `mcp/server/Cargo.toml` | `phf = "0.14"` |

Run `cargo update -p phf -p phf_codegen` in affected crates; refresh locks.

#### Phase D — Regenerate bindings

```bash
cd vim_build
cargo run --bin generate --release
```

Outputs: `vim_rs/src/types/*`, `vim_rs/src/mo/*`, `vim_macros/src/field_data.rs`, `mcp/server/src/field_data.rs`, PHF maps in `struct_enum.rs` / `deser.rs` / enum impls.

Verify idempotency (SC-005, SC-014).

#### Phase E — Dependency refresh: remaining majors + migrations (FR-018)

| Crate | Bump | Migration touchpoints |
|-------|------|----------------------|
| `vim_build` | `convert_case` 0.11, `check_keyword` 0.4 | `vim_model/types.rs`, `names.rs`, emitters — verify `Case`/`Casing`/`CheckKeyword` APIs |
| `vim_rs` | `quick-xml` 0.40 | `vim_rs/src/xml/*` if compile errors |
| `vim_rs` (dev) | `criterion` 0.8 | `benches/miniserde_bench.rs`, `Cargo.toml` bench harness |
| `mcp/server`, `data-transformer` | `bincode` 3 | `mcp/server/src/model.rs`, `data_transformer/src/main.rs` — use `bincode::serde::encode_to_vec` / `decode_from_slice` or equivalent v3 API |
| `mcp/server` (optional `web-ui`) | `tera` 2 | `mcp/server/src/web_ui/handlers.rs` |

#### Phase F — Dependency refresh: minor/patch + lockfiles (FR-019, FR-020)

Bump stale compat/patch deps per [research R16](./research.md#r16--dependency-refresh-matrix); run `cargo update` per crate; commit all in-scope `Cargo.lock` files.

Verify SC-012 (`cargo outdated --root-deps-only` clean) and SC-013 (single `phf` 0.14.x in `vim_rs/Cargo.lock`).

#### Phase G — Version bump to 0.6.0 (FR-006, FR-007, FR-015)

`vim_rs`, `vim_macros`, example path deps → **0.6.0**; CHANGELOG `[0.6.0]`; purge 0.5.1 (SC-011).

#### Phase H — MCP local rebuild (FR-005, FR-021)

```bash
cd mcp
cargo run -p data-transformer --release --features cuda
cargo build --release -p vim_mcp_server
```

Output: gitignored `mcp/data/api_database.bin`. Required for SC-007 manual validation, not committed.

#### Phase I — Compile & test validation

Full matrix in [quickstart.md](./quickstart.md). Minimum: `vim_rs`, `vim_macros`, `vim_build`, `openapi30` tests; examples `--bins`; `tls_rustls_only` build; MCP after Phase H.

#### Phase J — Release notes & metrics (FR-011, SC-006, SC-008)

`CHANGELOG.md` `[0.6.0]` must include:

- vSphere API 9.1.0.0 target + breaking minor rationale
- Spec-driven breaks (e.g. `EventFilterSpec`)
- **Each major dependency bump** as old→new (minimum set in FR-011)
- **One line** summarizing minor/patch dependency refresh
- Build/size metrics vs 0.5.0

### Tests

| Gate | Purpose | Spec ref |
|------|---------|----------|
| Regen idempotency | Second generator run → no diff | SC-005, SC-014 |
| `cargo test -p vim_rs` (+ `--all-features`) | Core regression | SC-004, SC-002 |
| `cargo test -p vim_macros` | PHF field_data + macros | SC-004 |
| `cargo outdated --root-deps-only` | Zero outdated direct deps | SC-012 |
| `vim_rs/Cargo.lock` phf count | Single 0.14.x major | SC-013 |
| Examples + tls_rustls_only | Monorepo compile | SC-001 |
| MCP after local DB regen | Manual validation | SC-007 |
| CHANGELOG audit | Majors enumerated + minor summary | SC-006 |

### Design artifacts

- [research.md](./research.md) — R1–R16
- [data-model.md](./data-model.md) — entities + dependency matrix
- [quickstart.md](./quickstart.md) — validation runbook
- [contracts/README.md](./contracts/README.md) — release + dependency contract

---

## Phase 2

**Not in scope for `/speckit-plan`**. Run **`/speckit-tasks`** to generate actionable `tasks.md` from phases A–J.
