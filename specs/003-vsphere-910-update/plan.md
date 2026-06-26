# Implementation Plan: vSphere 9.1.0.0 API Binding Update

**Branch**: `003-vsphere-910-update` | **Date**: 2026-06-26 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/003-vsphere-910-update/spec.md`

**Note**: Filled by `/speckit-plan`. Workflow: `.specify/templates/plan-template.md`.

## Summary

Mechanically regenerate all `vim_rs` bindings, `vim_macros`/`mcp` field-path metadata, and the MCP API database from the new **vSphere 9.1.0.0** VI/JSON OpenAPI input (`vim_build/data/vi_json_openapi_specification_v9_1_0_0.json`), bump **`vim_rs`** and **`vim_macros`** to **0.6.0** (breaking minor — not 0.5.1 patch), purge all **0.5.1** release-target references (FR-015), and validate full monorepo compilation. On failure, triage **OpenAPI input correctness first** (YAML→JSON coercion, string enum values) before changing the generator or downstream crates (FR-013, spec clarification 2026-06-26).

## Technical Context

**Language/Version**: Rust 2021, stable  
**Primary Dependencies**: In-tree `vim_build`, `openapi30`, `vim_macros`; `miniserde`, `tokio` (unchanged)  
**Storage**: Generated Rust sources under `vim_rs/src/`; `vim_macros/src/field_data.rs`; `mcp/server/src/field_data.rs`; `mcp/data/api_database.bin`  
**Testing**: `cargo test` in `vim_rs`, `vim_build`, `vim_macros`, `openapi30`; `cargo build --bins` in `examples/`; `cargo build -p vim_mcp_server` in `mcp/` (after data-transformer)  
**Target Platform**: Cross-platform library (Linux/macOS/Windows)  
**Project Type**: Multi-crate monorepo (generated bindings + tooling)  
**Performance Goals**: No unjustified regression vs 0.5.0 release gates (Principle III): `vim_rs` debug/release build time, `examples/vtui` binary size  
**Constraints**: No hand-edits to generated files (Principle I); `PRUNED_TYPES` unchanged unless generator blocked; `DatastoreAccessible_enum` must use `"True"`/`"False"` strings in JSON  
**Scale/Scope**: ~322K-line OpenAPI JSON; ~200K+ LOC generated `structs.rs`; all `vim_rs/src/mo/*.rs`; two ~13K-line `field_data.rs` files; MCP pipeline rebuild

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle / standard | Status | Evidence |
|---------------------|--------|----------|
| **I. Generated from authoritative specs** | Pass | Single generator run from 9.1.0.0 JSON; zero hand-edits to generated output |
| **II. Complete type-safe surface** | Pass | Full regeneration; `PRUNED_TYPES` preserved for `MethodFault`/`Event` |
| **III. Build-time & binary budgets** | Pass* | Capture metrics vs 0.5.0; document if spec growth causes regression (*measure in implement) |
| **IV. Multi-transport parity** | Pass* | Compile/test gate in scope; live vCenter run deferred per spec assumptions |
| **V. Documented public surfaces** | Pass | CHANGELOG `[0.6.0]` entry; classify as breaking minor; document API diffs (e.g. `EventFilterSpec`) |
| **VI. Readable source** | Pass | Generator output only; no readability regressions in hand-written code |
| **VII. Ecosystem tooling** | Pass | `vim_macros` field_data + MCP database regenerated and version-aligned |
| **OpenAPI ingestion** | Pass | Pre-flight validate `DatastoreAccessible_enum`; triage spec on failure (FR-013) |
| **CHANGELOG discipline** | Pass | `[Unreleased]` → `[0.6.0]` with spec version, semver rationale, and breaking changes; **0.5.1 MUST NOT** be published |

**Post-design re-check**: [research.md](./research.md), [data-model.md](./data-model.md), [contracts/](./contracts/) align with gates. **Documented deferral**: live vCenter integration (Principle IV) is recommended post-release, not blocking this mechanical update.

## Project Structure

### Documentation (this feature)

```text
specs/003-vsphere-910-update/
├── plan.md              # This file
├── research.md          # Phase 0
├── data-model.md        # Phase 1
├── quickstart.md        # Phase 1
├── contracts/           # Phase 1
├── spec.md
└── tasks.md             # /speckit-tasks (not created by /speckit-plan)
```

### Source Code (repository root)

```text
vim_build/
├── data/
│   ├── vi-json.yaml                              # source (reference)
│   ├── vi_json_openapi_specification_v9_1_0_0.json  # NEW canonical input
│   └── vi_json_openapi_specification_v9_0_0_0_24798170.json  # prior (reference)
└── src/main.rs                                   # point at 9.1.0.0 JSON

vim_rs/src/                                       # GENERATED (full regen)
├── types/  (structs.rs, enums.rs, traits.rs, …)
└── mo/     (per managed-object stubs)

vim_macros/src/field_data.rs                      # GENERATED
mcp/server/src/field_data.rs                      # GENERATED
mcp/data/api_database.bin                         # REBUILT via data-transformer

openapi30/src/openapi.rs                          # test fixture path (update)
mcp/data_processing/data_transformer/src/main.rs  # spec path (update)
mcp/server/build.rs                               # staleness check path (update)

vim_rs/Cargo.toml                                 # 0.6.0
vim_macros/Cargo.toml                             # 0.6.0
examples/snippets/Cargo.toml                      # path dep 0.6.0
examples/vtui/Cargo.toml                          # path dep 0.6.0
CHANGELOG.md                                      # [0.6.0] (retitle any [0.5.1] draft)
mcp/README.md                                     # aligned version mention
```

**Structure Decision**: Changes are **input-path wiring + regeneration + version bump + validation + semver artifact purge**. No new modules; hand-written edits limited to path constants, manifest versions, sample fixes for spec-driven breaks, release notes, and FR-015 scope (eliminate 0.5.1 as release target).

## Complexity Tracking

> No constitution violations requiring justification.

| Violation | Why needed | Simpler alternative rejected because |
|-----------|------------|-------------------------------------|
| — | — | — |

---

## Phase 0: Research

**Status**: Complete → [research.md](./research.md)

Resolved decisions:

| ID | Topic | Outcome |
|----|-------|---------|
| R1 | Canonical spec file | `vi_json_openapi_specification_v9_1_0_0.json` |
| R2 | Generation command | `cd vim_build && cargo run --bin generate --release` |
| R3 | Spec validation | Pre-flight `DatastoreAccessible_enum` + FR-013 triage on failure |
| R4 | Path constant updates | 5 locations reference old 9.0.x filename |
| R5 | MCP rebuild | `cargo run -p data-transformer --release --features cuda --features cuda` then `cargo build -p vim_mcp_server` |
| R6 | Version bump scope | `vim_rs` + `vim_macros` to **0.6.0**; path deps in examples; purge 0.5.1 (FR-015) |
| R7 | Build metrics | Record debug/release `vim_rs` + `examples/vtui` size vs 0.5.0 |
| R9 | Semver classification | **0.6.0 minor breaking** — struct field additions (e.g. `EventFilterSpec`) break literals without `defaults` feature |

No open **NEEDS CLARIFICATION** items.

---

## Phase 1: Design

**Status**: Complete

### Implementation phases (ordered)

#### Phase A — Pre-flight spec validation

1. Confirm `vim_build/data/vi_json_openapi_specification_v9_1_0_0.json` exists and parses.
2. Spot-check `DatastoreAccessible_enum`: values MUST be `"True"` / `"False"` strings (verified in plan research — already correct in supplied JSON).
3. If generator or tests fail, inspect YAML/JSON for boolean coercion **before** editing `vim_build` (FR-013).

#### Phase B — Wire generator to 9.1.0.0

Update hard-coded spec paths from `vi_json_openapi_specification_v9_0_0_0_24798170.json` → `vi_json_openapi_specification_v9_1_0_0.json`:

| File | Purpose |
|------|---------|
| `vim_build/src/main.rs` | Generator entry point |
| `mcp/data_processing/data_transformer/src/main.rs` | MCP API database input |
| `mcp/server/build.rs` | Staleness detection |
| `openapi30/src/openapi.rs` | Embedded test fixture |
| `vim_build/src/vim_model/loader.rs` | Unit test fixture path |

#### Phase C — Regenerate bindings

```bash
cd vim_build
cargo run --bin generate --release
```

**Outputs** (single run, no hand-edits):

- `vim_rs/src/types/*` (structs, enums, traits, ser/de, defaults, …)
- `vim_rs/src/mo/*.rs`
- `vim_macros/src/field_data.rs`
- `mcp/server/src/field_data.rs`

Verify idempotency: second run produces no diff (SC-005).

#### Phase D — Version bump to 0.6.0 (FR-006, FR-007, FR-015)

| File | Field |
|------|-------|
| `vim_rs/Cargo.toml` | `version`, `vim_macros` path dep version → **0.6.0** |
| `vim_macros/Cargo.toml` | `version` → **0.6.0** |
| `examples/snippets/Cargo.toml` | `vim_rs` path dep version → **0.6.0** |
| `examples/vtui/Cargo.toml` | `vim_rs` path dep version → **0.6.0** |
| `CHANGELOG.md` | Retitle `[0.5.1]` → `[0.6.0]`; add breaking-minor classification |
| `mcp/README.md` | Version alignment mention → **0.6.0** |
| `specs/003-vsphere-910-update/*` | Plan/contracts/data-model/quickstart/research/tasks — no 0.5.1 release targets |

**Semver audit** (SC-011):

```bash
rg '0\.5\.1' --glob '!**/Cargo.lock' --glob '!specs/**/spec.md'
# Expected after implement: zero hits in manifests and release docs
# (spec.md Input line may retain original user quote for history)
```

#### Phase E — MCP pipeline rebuild

```bash
cd mcp
cargo run -p data-transformer --release --features cuda
cargo build --release -p vim_mcp_server
```

#### Phase F — Compile & test validation

See [quickstart.md](./quickstart.md) for full command matrix. Minimum gates:

- `cargo build --release` / `cargo test` in `vim_rs`, `vim_build`, `vim_macros`, `openapi30`
- `cargo build --bins` in `examples/` (default features)
- `cargo build --bins --all-features` where applicable
- Fix sample/test code only for spec-driven API renames/removals (FR-010)

#### Phase G — Release notes & metrics

- `CHANGELOG.md`: `[0.6.0]` entry — target API 9.1.0.0, **breaking minor release** rationale, breaking changes from spec diff (include `EventFilterSpec` field additions), build/size metrics (SC-008)
- Document failure root-cause if any triage occurred (SC-009)
- Confirm no manifest or release doc still targets 0.5.1 (SC-011)

### Tests

| Test / gate | Purpose |
|-------------|---------|
| Regeneration idempotency | SC-005 — re-run generator, expect clean git diff on generated files |
| `cargo test -p vim_rs` | Core library regression |
| `cargo test -p vim_rs --all-features` | Optional feature surface |
| `cargo test -p openapi30` | OpenAPI loader against embedded 9.1.0.0 fixture |
| `cargo test -p vim_macros` | Macro + field_data resolution |
| `cargo build --bins` (examples) | SC-001 sample projects |
| `cargo build -p vim_mcp_server` | SC-007 MCP rebuild |
| Build timing + vtui size | SC-008 release gate |
| Semver artifact audit | SC-011 — no 0.5.1 in manifests/release docs |

### Documentation deliverables

- **CHANGELOG [0.6.0]**: API 9.1.0.0 target, breaking minor classification, breaking changes, build metrics
- **contracts/**: release artifact and validation contract ([contracts/README.md](./contracts/README.md))
- **quickstart.md**: ordered validation commands for implementers

### Design artifacts

- [research.md](./research.md) — decisions R1–R9
- [data-model.md](./data-model.md) — entities and regeneration flow
- [quickstart.md](./quickstart.md) — validation runbook
- [contracts/README.md](./contracts/README.md) — release contract

---

## Phase 2

**Not in scope for `/speckit-plan`**. Run **`/speckit-tasks`** to refresh `tasks.md` for the **0.6.0** semver correction (Phase D/G updates, SC-011 audit task) if not already present.
