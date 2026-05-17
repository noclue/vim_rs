# Implementation Plan: Metadata-guided XML deserialization

**Branch**: `001-metadata-xml-de` | **Date**: 2026-05-09 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/001-metadata-xml-de/spec.md`

**Note**: Filled by `/speckit-plan`. Execution workflow: `.specify/templates/plan-template.md`.

## Summary

Complete the metadata-driven XML tree driver in **`vim_rs::xml::de`**: every descent step chooses visitor traffic from **`ApiFieldType`** (**`lookup_api_field`**, root **`DataTypeAware`**) and, where **`xsi:type`** applies, from **`lookup_xml_type`** with order **struct (`StructType`) → boxed wrapper (`lookup_any_value_wrapper` / PHF) → XSD primitives** (**FR-010–FR-013**). **Shipped code must contain zero datatype / visitor probing** (**spec Goal**, **FR-014**, **SC-003**): remove legacy helpers such as ordered leaf trials, **`try_seq_or_single`** shape probing, and **`stream_drive_legacy_probe`** once **`Any`** and arrays are fully driven by metadata. **`vim_build`** owns **`DataTypeAware`**, **`lookup_any_value_wrapper`** (PHF), and registry layout; **`xml/soap.rs`** keeps SOAP boundaries (**FR-008**).

## Technical Context

**Language/Version**: Rust 2021 edition, stable toolchain as used in CI  
**Primary Dependencies**: `miniserde`, `quick-xml` (feature `xml`), `log` / `tracing` for wire-scoped diagnostics  
**Storage**: N/A (streaming XML; no persistent store)  
**Testing**: `cargo test -p vim_rs --features xml`, `cargo check -p vim_rs --all-features`; curated XML fixtures in `vim_rs/tests/`  
**Target Platform**: Cross-platform **`vim_rs`** library  
**Project Type**: Rust library + in-tree generator (**`vim_build`**)  
**Performance Goals**: No full-document DOM on the primary path; **O(1)** wrapper name resolution via PHF; avoid regressing **`vim_rs`** / **`vtui`** build times per Principle III  
**Constraints**: **`miniserde` only** (no `serde` in `vim_rs`); public **`Result`** errors stay unit-style; **`vcsim_compat`** may only apply documented finish/build swallowing—**not** new probing (**spec Goal**)  
**Scale/Scope**: Generated **`api_field_registry`** (~tens of kLOC); hand-written **`de.rs`** driver (~1.5k LOC target after probe deletion)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle / standard | Status | Evidence |
|---------------------|--------|----------|
| **I. Generated from specs** | Pass | Registries, **`DataTypeAware`**, PHF emitted from **`vim_build`**; no hand-edits to generated `vim_rs/src/types/*` |
| **II. Complete type-safe surface** | Pass | Driver follows **`ApiFieldType`**; no silent string buckets for unknown children |
| **III. Build-time & binary budgets** | Pass | **`xml`** / **`vcsim_compat`** opt-in; PHF vs giant **`match`** for hot lookups |
| **IV. Multi-transport parity** | Pass | Fixtures + **`--all-features`**; real-vCenter examples remain release gate outside this doc |
| **V. Documented public surfaces** | Pass | **`METADATA_DRIVEN_XML_DE.md`**, rustdoc for **`DataTypeAware`** / unmarshalling, **CHANGELOG** for generic bound changes |
| **VI. Readable source** | Pass | Probe removal simplifies **`de.rs`**; dispatch stays structured by type metadata |
| **VII. Ecosystem tooling** | Pass | **`vim_build`** / **`openapi30`** changes reviewed like product code |
| **Marshalling (miniserde)** | Pass | No serde migration |
| **Wire diagnostics (`wire::soap`)** | Pass | Logging scoped; no credential leakage |

**Post-design re-check**: **research.md** / **data-model.md** / **contracts/** encode registry-only typing and **zero residual probing**; no new violations.

## Project Structure

### Documentation (this feature)

```text
specs/001-metadata-xml-de/
├── plan.md              # This file
├── research.md          # Phase 0
├── data-model.md        # Phase 1
├── quickstart.md        # Phase 1
├── contracts/           # Phase 1
├── spec.md
└── tasks.md             # /speckit-tasks (not produced by /speckit-plan)
```

### Source Code (repository root)

```text
vim_build/src/rs_emitter/
├── api_registry.rs       # lookup_xml_type, lookup_any_value_wrapper (PHF), tables
└── data_type_aware.rs    # DataTypeAware trait codegen

vim_rs/src/
├── xml/de.rs             # Streaming driver (primary implementation surface)
├── xml/soap.rs           # SOAP envelope / body extraction
├── core/client.rs        # unmarshal*, DataTypeAware bounds when xml enabled
└── types/api_field_registry.rs   # Generated (do not hand-edit)

vim_rs/tests/             # XML/SOAP and regression tests
```

**Structure Decision**: Feature work is **`vim_build`** (metadata + codegen) plus **`vim_rs/src/xml/de.rs`** + **`core/client`** bounds; no new top-level crate.

## Complexity Tracking

> Fill only if constitution gates need justified exceptions. None for this plan.

| Violation | Why needed | Simpler alternative rejected because |
|-----------|------------|-------------------------------------|
| — | — | — |
