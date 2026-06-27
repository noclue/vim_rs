# Feature Specification: vSphere 9.1.0.0 API Binding Update

**Feature Branch**: `003-vsphere-910-update`

**Created**: 2026-06-26

**Status**: Draft

**Input**: User description: "vSphere 9.1.0.0 is out. We should update the vim_rs crate accordingly. I have already downloaded the yaml API definition and converted it to json. We need to rebuild the vim_rs bindings cleanly with vim_build. We should rebuild the mcp as well. The vim_macros field_data should too be rebuilt. We should bump the version of vim_rs and vim_macros to 0.5.1. As a test we need to make sure everything including the sample projects compiles without a glitch."

## Context & Problem

VMware has released vSphere API version 9.1.0.0. The `vim_rs` project provides type-safe bindings to the VMware VIM API and must track the authoritative upstream API definition to remain accurate and useful. The project currently targets an older 9.0.x API specification. Without this update, consumers cannot access new or changed VIM types, properties, and managed-object methods introduced in 9.1.0.0, and existing bindings may drift from what a current vCenter exposes.

The authoritative OpenAPI definition for 9.1.0.0 has already been obtained and converted to the canonical JSON form used by the in-tree code generator. This feature delivers a clean, mechanical regeneration of all generated surfaces and a coordinated **minor** release (**0.6.0**) so downstream users can adopt 9.1.0.0 coverage with semver-appropriate expectations for breaking VIM API surface changes.

## Clarifications

### Session 2026-06-26

- Q: When generation or build fails during a vSphere spec update, where should investigation begin? → A: Verify OpenAPI input correctness first — including YAML-to-JSON conversion artifacts (e.g., `true`/`false` literals coerced to booleans where the VIM API expects string enum values such as `"True"`/`"False"`) — before concluding the fault lies in the generator or downstream crates alone.
- Q: How should the MCP API database embedding step be run during this update? → A: Enable the data-transformer `cuda` feature (`--features cuda`) when rebuilding the MCP database on a machine with NVIDIA CUDA, to reduce rebuild time and energy versus CPU-only embedding generation.
- Q: Must hand-written project references to the binding target API version be audited for stale 9.0.x mentions? → A: Yes — scan and update hand-written docs, constants, and LLM/MCP guidance (e.g. `README.md`, `CLAUDE.md`, `vim_rs/src/core/client.rs` `API_RELEASE` / `COMPATIBLE_API_RELEASES`, MCP READMEs) so they state 9.1.0.0 where they describe the **current binding target**; exclude generated VMware API metadata (e.g. `***Since:*** vSphere API Release 9.0.0.0` in generated rustdoc) and intentional historical references (prior spec file under `vim_build/data/`).
- Q: Should release semver be 0.5.1 (patch) or 0.6.0 given breaking API changes from the 9.1.0.0 spec diff (e.g. new required `EventFilterSpec` fields)? → A: **0.6.0 minor** — breaking VIM API surface changes warrant a minor semver bump, not a patch. Align `vim_rs`, `vim_macros`, all example path dependencies, `CHANGELOG.md`, MCP/LLM docs, and in-repo spec/plan/contracts artifacts to **0.6.0**. Version **0.5.1 MUST NOT** be published or remain as the target release version anywhere in the repository.
- Q: What scope should the third-party dependency refresh take for the 0.6.0 release? → A: **Full monorepo refresh** — bump all direct dependencies across every in-repository crate to their latest compatible versions, including major-version upgrades (not only `phf` 0.14 alignment). Minimum crate surface: `vim_rs`, `vim_macros`, `vim_build`, `openapi30`, MCP workspace (`mcp/server`, `mcp/api_database`, `mcp/data_processing/*`), `examples/*`, and `tls_rustls_only`. Regenerate PHF-backed artifacts after aligning `phf` / `phf_codegen` to **0.14** everywhere they are consumed or emitted.
- Q: How should dependency version changes be documented in release notes? → A: **Enumerate each major bump** (old→new version) in the `[0.6.0]` `CHANGELOG.md` entry; summarize minor and patch updates in a single summary line (not per-crate enumeration).
- Q: Should `mcp/data/api_database.bin` be committed after `bincode` migration? → A: **No** — `mcp/data/` is `.gitignore`d. Regenerate `api_database.bin` locally via the data-transformer pipeline for **manual MCP validation** documented in `quickstart.md`; the binary is a local build artifact, not a release commit.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Access vSphere 9.1.0.0 API surface (Priority: P1)

A developer building automation against a vCenter running vSphere 9.1.0.0 depends on `vim_rs` and expects the library's types, enums, traits, and managed-object stubs to reflect the current VMware VIM API — including any additions or changes present in the 9.1.0.0 definition.

**Why this priority**: Matching the authoritative API spec is the primary reason for this release; without regenerated bindings the library is stale relative to the platform it targets.

**Independent Test**: After release, a developer can reference types and methods that exist only in the 9.1.0.0 API definition and compile a minimal program that uses them without manual workarounds or hand-maintained shims.

**Acceptance Scenarios**:

1. **Given** the 9.1.0.0 OpenAPI definition is the generator input, **When** bindings are regenerated, **Then** the published library surface corresponds to that definition (no hand-edits to generated output).
2. **Given** a developer on `vim_rs` 0.6.0, **When** they inspect available VIM types and managed-object methods, **Then** they reflect additions and changes from 9.1.0.0 relative to the prior 9.0.x baseline.
3. **Given** the project's documented pruned-type policy (`MethodFault`, `Event`), **When** bindings are regenerated, **Then** pruned hierarchies continue to use discriminated `type_` and `extra_fields_` rather than silently dropping API information.

---

### User Story 2 - Entire monorepo compiles cleanly (Priority: P1)

A maintainer or contributor checks out the updated branch and expects every crate and sample project in the repository — core library, procedural macros, generator tooling, MCP server, and all example binaries — to build without errors after the spec upgrade and version bump.

**Why this priority**: A partial update that breaks compilation blocks all consumers and invalidates the release; compile success is the minimum bar for shipping.

**Independent Test**: Run a full build across the core library, macros, generator, MCP workspace, and all sample projects with default features; repeat with all optional features enabled where applicable; all complete with zero errors.

**Acceptance Scenarios**:

1. **Given** regenerated bindings and bumped library versions, **When** the core library and macro crates are built, **Then** compilation succeeds with no errors.
2. **Given** the same update, **When** all in-repository sample projects are built, **Then** each compiles without source changes required solely because of the API update (aside from any intentional breaking API renames documented in the changelog).
3. **Given** optional feature flags documented for the library, **When** builds are run with all features enabled, **Then** compilation still succeeds.

---

### User Story 3 - Macro and MCP tooling stay in sync (Priority: P2)

A developer using property-path macros or the MCP exploration server expects field-path reference data and semantic API databases to match the same 9.1.0.0 definition as the core bindings, so property paths resolve correctly and AI-assisted exploration reflects current types.

**Why this priority**: Property-collector macros and the MCP server depend on generated field metadata; stale metadata causes compile-time or runtime path resolution failures even when core types are updated.

**Independent Test**: Regenerate field-path metadata from the same generator run; rebuild the MCP API database; confirm macro expansion succeeds for representative property paths and the MCP server builds.

**Acceptance Scenarios**:

1. **Given** a fresh generator run against 9.1.0.0, **When** field-path reference data for procedural macros is emitted, **Then** it is regenerated from the same model as the core bindings (not hand-edited).
2. **Given** the same generator run, **When** field-path reference data for the MCP server is emitted, **Then** it matches the macro crate's data source and reflects 9.1.0.0 paths.
3. **Given** updated bindings, **When** the MCP data pipeline is executed with GPU acceleration enabled (`cuda` feature), **Then** the MCP server and its API database rebuild successfully in less time than a CPU-only embedding run.
4. **Given** a `bincode` major migration, **When** an implementer follows `quickstart.md` Step 4 (local data-transformer run into gitignored `mcp/data/`), **Then** `vim_mcp_server` builds and manual MCP testing can proceed without committing `api_database.bin`.

---

### User Story 4 - Coordinated 0.6.0 minor release (Priority: P2)

A downstream consumer upgrading from 0.5.0 sees aligned version numbers on the core library and macro crate, a changelog entry describing the 9.1.0.0 update as a **breaking minor release**, and explicit notes on API diffs that require source changes (e.g. new fields on `EventFilterSpec`).

**Why this priority**: Version alignment and release notes are how consumers discover and safely adopt the update; a minor bump correctly signals breaking VIM surface changes.

**Independent Test**: Verify published crate versions are 0.6.0 for both the core library and macros; verify changelog documents the spec version change, semver rationale, and breaking API diffs; confirm no artifact still targets 0.5.1.

**Acceptance Scenarios**:

1. **Given** the completed update, **When** crate versions are inspected, **Then** both the core library and macro crate report version 0.6.0.
2. **Given** the release, **When** a consumer reads the changelog, **Then** they find a `[0.6.0]` entry stating the binding now targets vSphere API 9.1.0.0, classifying it as a breaking minor release, listing breaking changes from the spec diff (including struct field additions such as `EventFilterSpec`), enumerating each major third-party dependency bump as old→new (e.g. `phf` 0.11/0.13→0.14, `bincode` 1→3), and summarizing minor/patch dependency updates in one line.
3. **Given** the release, **When** a maintainer audits hand-written project docs and constants, **Then** no stale references remain that imply 9.0.0.0 is the current binding target or that 0.5.1 is the release version (excluding generated API metadata and the archived 9.0.x spec file).
4. **Given** the completed update, **When** a repository search is run for `0.5.1` in version-bearing manifests and release docs, **Then** zero hits remain outside historical git history or intentional changelog comparison text referencing the superseded plan.

---

### User Story 5 - Monorepo dependencies current and aligned (Priority: P2)

A maintainer shipping 0.6.0 expects third-party crates across the repository to be on recent, aligned versions — especially `phf` 0.14 shared by generated lookup tables in `vim_rs`, `vim_macros`, `vim_build` codegen, and the MCP server — so lockfiles do not resolve duplicate major versions of the same runtime crate.

**Why this priority**: Stale or split dependency versions (e.g. `phf` 0.11 via `vim_macros` alongside 0.13 in `vim_rs`) increase compile time, risk subtle runtime mismatches in generated PHF maps, and leave security/maintenance debt on a release consumers will treat as current.

**Independent Test**: Run `cargo outdated --root-deps-only` in each in-scope crate; verify zero outdated direct dependencies remain (or any exception is documented). Confirm `vim_rs/Cargo.lock` resolves a single `phf` major version (0.14). Rebuild and test after lockfile refresh.

**Acceptance Scenarios**:

1. **Given** the dependency refresh, **When** `vim_macros`, `vim_rs`, `vim_build`, and `mcp/server` manifests are inspected, **Then** `phf` (runtime) and `phf_codegen` (build-time) both target **0.14** and generated PHF maps are regenerated from `vim_build`.
2. **Given** the full monorepo scope, **When** each in-scope crate's direct dependencies are compared to crates.io latest, **Then** all major, minor, and patch updates identified at clarification time are applied or explicitly documented as blocked with remediation notes.
3. **Given** updated lockfiles, **When** the full build and test matrix runs, **Then** no new failures are attributable to dependency version skew (including MCP `bincode` / `tera` major migrations and `vim_rs` `quick-xml` / `criterion` major bumps).

---

### Edge Cases

- **OpenAPI ingestion quirks**: The canonical JSON input must satisfy project ingestion rules (e.g., `DatastoreAccessible` enum values as strings `"True"` / `"False"`, not booleans). Invalid input must be caught and corrected before generation, not patched into generated output. Prior vSphere spec updates have failed because YAML `true`/`false` literals were interpreted as booleans per YAML semantics when the VIM API contract expects quoted string values; the same class of error can survive into the JSON file if conversion is not validated.
- **Failure triage order**: On any generator, compile, or test failure during this update, investigation MUST start with OpenAPI input validation (YAML source, JSON conversion output, and known coercion pitfalls) in parallel with — not after — generator and crate debugging. A spec defect must be corrected at the input layer; generator or crate changes are appropriate only after input correctness is confirmed.
- **Spec-only removals or renames**: If 9.1.0.0 removes or renames VIM types or properties present in 9.0.x, downstream sample code or tests that referenced them may fail to compile; such breaks must be documented in the changelog and sample code updated as part of this feature.
- **Struct field additions without `defaults` feature**: New required fields on generated structs (e.g. `EventFilterSpec.auditable`, `EventFilterSpec.audit_id`) break existing struct literals at compile time unless the `defaults` feature is enabled; these count as breaking API changes and justify the **0.6.0** minor release (not 0.5.1 patch).
- **Build-time and binary-size regression**: Regenerating a larger API surface may increase compile time or binary size; if the increase exceeds project release-gate tolerances, it must be measured, documented, and justified per project constitution rather than ignored.
- **Generator-only changes**: If the 9.1.0.0 spec exposes constructs the generator cannot yet emit, fixes belong in the generator or model layer — not in hand-edits to generated files.
- **MCP database rebuild without CUDA**: If NVIDIA CUDA is unavailable, the data-transformer MAY fall back to CPU embedding generation; the resulting `api_database.bin` MUST be equivalent. CUDA is the preferred path when hardware is present (FR-005).
- **Stale 9.0.x references in hand-written sources**: Generated rustdoc lines such as `***Since:*** vSphere API Release 9.0.0.0` reflect upstream VMware API history and MUST NOT be hand-edited. Stale **project** references — binding-target constants (`API_RELEASE`), README/CLAUDE.md/MCP guidance, old OpenAPI filenames in docs — MUST be updated to 9.1.0.0 (FR-014).
- **PHF version skew**: `vim_macros` on `phf` 0.11 while `vim_rs` uses 0.13 causes duplicate `phf` copies in the dependency graph; alignment to **0.14** MUST include `vim_build` `phf_codegen` 0.14 and a clean generator rerun so emitted maps match the runtime crate.
- **High-risk major migrations**: `bincode` 1→3 (MCP API database serialization) and `tera` 1→2 (MCP web UI) may require source changes beyond manifest bumps; migration fixes are in scope for 0.6.0, not deferred, per full-monorepo refresh decision.
- **Gitignored MCP database**: `mcp/data/` (including `api_database.bin`) is listed in `.gitignore` and MUST NOT be committed. After dependency or spec changes affecting serialization or embeddings, implementers regenerate the database locally via data-transformer; `quickstart.md` Step 4 is the authoritative manual-validation runbook.
- **Optional-feature dependency paths**: `quick-xml` 0.40 (`vim_rs` `xml` / `vcsim_compat`) and `criterion` 0.8 (`vim_rs` benches) MUST compile under their respective feature/dev configurations after bump.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The binding generator MUST use the vSphere 9.1.0.0 VI/JSON OpenAPI definition (`vi_json_openapi_specification_v9_1_0_0.json`) as its sole authoritative input for this release.
- **FR-002**: All generated binding output for the core library MUST be produced by a clean generator run with no manual edits to generated source files.
- **FR-003**: Field-path reference data used by procedural macros MUST be regenerated from the same generator run and model as the core bindings.
- **FR-004**: Field-path reference data used by the MCP server MUST be regenerated from the same generator run and model as the core bindings.
- **FR-005**: The MCP API database and server artifacts MUST be rebuilt so they reflect the 9.1.0.0 API surface and updated dependency versions (including `bincode` 3 serialization). The embedding generation step MUST use the data-transformer `cuda` feature (`--features cuda`) on machines with NVIDIA CUDA available, to minimize rebuild time and energy consumption. Output is written to gitignored `mcp/data/api_database.bin` via local data-transformer execution — not committed to the repository.
- **FR-006**: The core library version MUST be bumped to **0.6.0** (not 0.5.1).
- **FR-007**: The procedural macro crate version MUST be bumped to **0.6.0** (not 0.5.1).
- **FR-008**: The core library, macro crate, generator, OpenAPI parser, MCP workspace, and all in-repository sample projects MUST compile successfully after the update.
- **FR-009**: Existing automated tests for the core library, macros, generator, and OpenAPI parser MUST pass after the update (integration tests requiring live vCenter may remain `#[ignore]` but must not be broken structurally).
- **FR-010**: Any compile failures in sample projects caused by API renames or removals in 9.1.0.0 MUST be resolved as part of this feature, not deferred.
- **FR-011**: User-visible release notes MUST document the move to vSphere API 9.1.0.0, version **0.6.0** as a **breaking minor release**, all breaking changes attributable to the spec diff (including struct field additions such as `EventFilterSpec`), and all major third-party dependency bumps as explicit old→new version lines in `CHANGELOG.md` `[0.6.0]` (minimum: `phf`, `phf_codegen`, `quick-xml`, `criterion`, `convert_case`, `check_keyword`, `bincode`, `tera`). Minor and patch dependency updates MUST be summarized in a single CHANGELOG line, not enumerated per crate.
- **FR-012**: The prior 9.0.x OpenAPI input file MAY remain in the repository for reference, but the generator MUST point exclusively at the 9.1.0.0 JSON file for this release.
- **FR-013**: When generation, compilation, or tests fail during this update, the team MUST validate OpenAPI input correctness (YAML source, JSON output, and known type-coercion pitfalls such as boolean literals where string enum values are required) before attributing the failure solely to the generator, OpenAPI parser, or downstream crates.
- **FR-014**: Before release, hand-written project sources that describe the **current binding target** MUST be audited and updated from 9.0.x to 9.1.0.0 where applicable. Scope includes at minimum: `README.md`, `CLAUDE.md`, `vim_rs/src/core/client.rs` (`API_RELEASE`, `COMPATIBLE_API_RELEASES`), and MCP/LLM guidance docs (`mcp/README.md`, related MCP docs). Excludes generated binding rustdoc (VMware `Since` metadata) and the archived 9.0.x OpenAPI file retained under `vim_build/data/`.
- **FR-015**: All version-bearing artifacts MUST be updated from **0.5.1** to **0.6.0** and MUST NOT retain 0.5.1 as a release target. Minimum scope: `vim_rs/Cargo.toml`, `vim_macros/Cargo.toml`, `vim_rs` path dependency version in `examples/snippets/Cargo.toml` and `examples/vtui/Cargo.toml`, `CHANGELOG.md` (`[0.6.0]` entry replacing any `[0.5.1]` draft), `mcp/README.md`, and in-repository spec kit artifacts under `specs/003-vsphere-910-update/` (`plan.md`, `tasks.md`, `contracts/README.md`, `data-model.md`, `quickstart.md`, `research.md`).
- **FR-016**: All in-scope crates (`vim_rs`, `vim_macros`, `vim_build`, `openapi30`, MCP workspace crates, `examples/*`, `tls_rustls_only`) MUST have direct dependencies bumped to the latest published versions on crates.io as of this release, including semver-major upgrades where applicable.
- **FR-017**: `phf` MUST be aligned to **0.14** in `vim_rs`, `vim_macros`, and `mcp/server`; `phf_codegen` MUST be aligned to **0.14** in `vim_build`. After bumping, PHF-backed generated files (`struct_enum.rs`, `field_data.rs`, `deser.rs` type registry, enum PHF maps, macro/MCP `field_data.rs`) MUST be regenerated via `vim_build` — not hand-edited.
- **FR-018**: The following major direct-dependency upgrades identified at clarification time MUST be included in this release (manifest bump plus any required source migration): `quick-xml` 0.39→0.40 (`vim_rs`), `criterion` 0.5→0.8 (`vim_rs` dev), `convert_case` 0.8→0.11 and `check_keyword` 0.3→0.4 (`vim_build`), `bincode` 1→3 and `tera` 1→2 (`mcp/server`).
- **FR-019**: Semver-compatible stale direct dependencies MUST receive minor and patch bumps in the same pass. Baseline from clarification analysis: `bytes`, `env_logger`, `log` (`vim_rs`); `chrono`, `indexmap`, `log`, `serde`, `serde_json`, `thiserror` (`vim_build`); `actix-web`, `anyhow`, `fastembed`, `regex`, `rmcp` (`mcp/server`); plus equivalent refresh for `examples/*`, `openapi30`, `tls_rustls_only`, and remaining MCP pipeline crates.
- **FR-020**: All in-scope `Cargo.lock` files MUST be updated and committed so a clean checkout resolves the bumped dependency graph without duplicate major versions of `phf` in `vim_rs`.
- **FR-021**: `quickstart.md` MUST document the local MCP database regeneration workflow (Step 4): run data-transformer into gitignored `mcp/data/`, then build `vim_mcp_server` for manual MCP testing — explicitly noting that `api_database.bin` is not tracked in git and must be regenerated after spec or `bincode`/embedding pipeline changes.

### Key Entities

- **VI/JSON OpenAPI Definition (9.1.0.0)**: The authoritative VMware VIM API contract in JSON form; source for all generated artifacts.
- **Generated Bindings**: Type definitions, enums, traits, managed-object stubs, and serialization helpers emitted into the core library.
- **Field-Path Reference Data**: Generated lookup tables mapping VIM property paths to Rust types; consumed by procedural macros and the MCP server.
- **MCP API Database**: Processed, searchable representation of the VIM API used by the MCP exploration server.
- **Release Artifacts (0.6.0)**: Versioned core library and macro crate packages published together with changelog documentation; semver signals breaking VIM API changes from 9.1.0.0.
- **Dependency Refresh Matrix**: Per-crate manifest of direct third-party dependencies and their target versions; major bumps (`phf` 0.14, `quick-xml` 0.40, `criterion` 0.8, `convert_case` 0.11, `check_keyword` 0.4, `bincode` 3, `tera` 2) tracked for migration and changelog disclosure.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of in-repository crates and sample projects compile with zero errors on a clean checkout after the update (default features), **except** `vim_mcp_server` which requires a local data-transformer run per `quickstart.md` Step 4 to produce gitignored `mcp/data/api_database.bin`.
- **SC-002**: Builds with all documented optional features enabled complete with zero errors.
- **SC-003**: The core library and macro crate both report version **0.6.0** in their package metadata; no manifest or release doc targets 0.5.1.
- **SC-004**: Automated test suites for the core library, macros, generator, and OpenAPI parser pass (`cargo test` equivalent) with no new failures attributable to the spec update.
- **SC-005**: Zero hand-edited lines exist in generated binding files or generated field-path reference files (verifiable by regeneration producing no diff).
- **SC-006**: Changelog contains a `[0.6.0]` entry identifying vSphere API 9.1.0.0 as the new binding target, classifying the release as a breaking minor bump, enumerating breaking API changes from the spec diff (including `EventFilterSpec` field additions), listing each major dependency bump as old→new, and including one summary line for minor/patch dependency updates.
- **SC-007**: Following `quickstart.md` Step 4 (local data-transformer into `mcp/data/`, then `vim_mcp_server` build), the MCP data pipeline and server complete without errors; `api_database.bin` remains gitignored.
- **SC-008**: Build-time and binary-size metrics for the core library and the primary consumer benchmark are captured for comparison against the 0.5.0 baseline (per project release gates); any regression beyond tolerance is documented with justification.
- **SC-009**: Any failure encountered during the update has a documented root-cause classification distinguishing OpenAPI input defects (including YAML-to-JSON coercion errors) from generator or downstream crate defects; input defects are resolved by correcting the spec, not by patching generated output.
- **SC-010**: A repository-wide audit of hand-written sources (FR-014 scope) finds zero stale references that present 9.0.0.0 as the current binding target; verifiable via search for `9_0_0`, `9.0.0.0`, and `vi_json_openapi_specification_v9_0` outside generated code and intentional archives.
- **SC-011**: A repository-wide search for `0.5.1` in version manifests and release documentation (FR-015 scope) finds zero remaining release-target references; any `[0.5.1]` CHANGELOG draft is retitled to `[0.6.0]`.
- **SC-012**: `cargo outdated --root-deps-only` reports zero outdated direct dependencies for each in-scope crate (`vim_rs`, `vim_macros`, `vim_build`, `mcp/server`, `examples/snippets`, `examples/vtui`, `tls_rustls_only`, `openapi30`), or every remaining outdated line has a documented exception in `research.md`.
- **SC-013**: `vim_rs/Cargo.lock` contains exactly one resolved major version of the `phf` crate (0.14.x) after `vim_macros` alignment; no `phf` 0.11.x or 0.13.x entries remain.
- **SC-014**: PHF-backed generated sources rebuild identically from `vim_build` after `phf_codegen` 0.14 upgrade (regeneration produces no unexpected diff beyond the version bump itself).

## Assumptions

- The user-supplied YAML has already been converted to `vi_json_openapi_specification_v9_1_0_0.json` and is valid for the in-tree OpenAPI loader.
- The JSON file satisfies project ingestion rules (string enum values where required, canonical JSON as generator input); if validation fails, the YAML source or conversion step — not the generator — is the presumed first suspect based on prior vSphere update experience.
- No changes to the generator's pruned-type policy (`PRUNED_TYPES`) are required unless 9.1.0.0 introduces constructs that cannot be emitted; any such generator fixes are in scope only if blocking compilation.
- Live vCenter integration testing against 9.1_Test vCenter is desirable per project constitution but is not a gate for completing this spec-driven update; compile and unit-test success is the primary acceptance gate stated by the user.
- Version **0.6.0** is a **minor breaking release** (not 0.5.1 patch): the 9.1.0.0 spec diff introduces compile-breaking struct changes (e.g. new fields on `EventFilterSpec`) that require consumer source updates or the `defaults` feature; all such breaks are documented in CHANGELOG under `[0.6.0]`. Version 0.5.1 MUST NOT be published.
- Sample project source updates are limited to fixes required by spec-driven API changes, not opportunistic refactors.
- The developer machine performing the MCP database rebuild has NVIDIA CUDA available; embedding generation uses the data-transformer's `cuda` feature for this release.
- `phf` 0.14 MSRV (Rust 1.66+) is acceptable for this project; no separate MSRV bump is required unless toolchain gate fails.
- `bincode` 3 and `tera` 2 API migrations are feasible within this release cycle; `api_database.bin` regeneration after `bincode` migration is a **local** data-transformer step (gitignored `mcp/data/`), documented in `quickstart.md` for manual MCP validation — not a committed artifact.

## Dependencies

- Availability of the 9.1.0.0 OpenAPI JSON at `vim_build/data/vi_json_openapi_specification_v9_1_0_0.json`.
- NVIDIA CUDA (optional but preferred) for MCP embedding generation via data-transformer `cuda` feature.
- Existing in-tree generator (`vim_build`), OpenAPI parser (`openapi30`), MCP data pipeline, and procedural macro infrastructure — all regenerated, not replaced.
- Project constitution principles I (generated from authoritative specs), III (build-time budgets), VII (ecosystem tooling is part of the product).
- crates.io availability of target versions: `phf` / `phf_codegen` **0.14.0**, `quick-xml` **0.40.x**, `criterion` **0.8.x**, `convert_case` **0.11.x**, `bincode` **3.x**, `tera` **2.x**, and current minor/patch releases for remaining direct dependencies.
