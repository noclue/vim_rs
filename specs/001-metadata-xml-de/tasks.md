# Tasks: Metadata-guided XML deserialization

**Input**: Design documents from `specs/001-metadata-xml-de/`  
**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/README.md](./contracts/README.md), [quickstart.md](./quickstart.md)

**Tests**: Verification tasks align with **SC-001**–**SC-004**; negative-path coverage supports **SC-002**. No mandatory TDD-only phase.

**Organization**: Phases follow user-story priorities (P1 → P3) after codegen groundwork. **Phase 8** is the **completion gate** (**FR-014**, **SC-003**, spec **Goal**): **zero** residual probing in `vim_rs/src/xml/de.rs`.

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Parallelizable (different files, no ordering dependency within the same checkpoint)
- **[Story]**: `[US1]` … `[US3]` map to [spec.md](./spec.md) user stories (user-story phases only)

## Path Conventions

Rust layout per [plan.md](./plan.md): `vim_rs/`, `vim_build/`, specs under `specs/001-metadata-xml-de/`.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Align implementers with authoritative design before touching code.

- [x] T001 Review implementation checklist against `vim_rs/docs/METADATA_DRIVEN_XML_DE.md` and `specs/001-metadata-xml-de/plan.md`
- [x] T002 [P] Inventory probe-first patterns to eliminate (`deliver_text`, `try_seq_or_single`, `stream_drive_legacy_probe`, ordered visitor trials) in `vim_rs/src/xml/de.rs` and note call sites in `vim_rs/src/xml/mod.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Generated **`DataTypeAware`** surface and a compiling **`vim_rs`** with `--features xml` before rewriting the XML driver.

**⚠️ CRITICAL**: User story phases must not start until this phase completes.

- [x] T003 Implement **`DataTypeAware`** trait emission (definition + struct/enum/`VimAny` / trait-object bases / primitives per [research.md](./research.md)) in `vim_build/src/rs_emitter/` and register it from `vim_build/src/generator.rs`
- [x] T004 Wire generated module exports so **`DataTypeAware`** is reachable from `vim_rs` (updates to `vim_build` printer paths and `vim_rs/src/types/mod.rs` emission stubs as needed—no hand-edits to final generated files except via regeneration)
- [x] T005 Run **`vim_build`** code generator (`vim_build/`, `cargo run --bin generate --release`) and iterate until `cargo check -p vim_rs --features xml` succeeds from repo root

**Checkpoint**: Foundation ready — **`DataTypeAware`** exists and **`vim_rs`** builds with **`xml`**.

---

## Phase 3: User Story 1 — Predictable typed values from XML (Priority: P1) 🎯 MVP

**Goal**: **`vim_rs/src/xml/de.rs`** walks XML using **`ApiFieldType`** (**`lookup_api_field`**, **`lookup_xml_type`**) with **no** shape guessing for sequences or **typed** leaves.

**Independent Test**: Representative structs/enums/polymorphic payloads deserialize consistently with the JSON path for logically equivalent data ([spec.md](./spec.md) US1).

### Implementation for User Story 1

- [x] T006 [US1] Rewrite recursive descent in `vim_rs/src/xml/de.rs` so object children resolve fields via `vim_rs::types::api_field_registry::lookup_api_field` using wire names (no Rust-field-name guessing)
- [x] T007 [US1] Implement **`xsi:type`** refinement and **`xsi:nil`** handling per `specs/001-metadata-xml-de/research.md` in `vim_rs/src/xml/de.rs` using `lookup_xml_type` / inheritance checks as specified in `vim_rs/docs/METADATA_DRIVEN_XML_DE.md`
- [x] T008 [US1] Deserialize arrays from **`ApiFieldType::Array`** inner types in `vim_rs/src/xml/de.rs`; apply duplicate sibling policy ([research.md](./research.md) R4)
- [x] T009 [US1] Update **`from_xml`** / **`from_xml_with`** in `vim_rs/src/xml/de.rs` (and re-exports in `vim_rs/src/xml/mod.rs`) to require **`DataTypeAware`** and start descent from **`T::data_type()`**

**Checkpoint**: Metadata-driven descent for non-probe refactor baseline is in place; **full US1** requires **Phase 8** (**FR-014**).

---

## Phase 4: User Story 2 — Explicit failures + observability (Priority: P2)

**Goal**: Unknown wire shapes fail fast; optional **`tracing`** aids debugging; **`Result`** stays unit-style ([spec.md](./spec.md) FR-009).

**Independent Test**: Negative fixtures fail without silent string fallbacks ([spec.md](./spec.md) US2).

### Implementation for User Story 2

- [x] T010 [US2] Add **`tracing`** diagnostics on failure paths in `vim_rs/src/xml/de.rs` using **`vim_rs::wire::soap`** (no secrets in logs; per constitution)
- [x] T011 [US2] Ensure unknown child tags and invalid **`xsi:type`** resolve to **`Err`** without string fallback in `vim_rs/src/xml/de.rs`; add or extend regression coverage under `vim_rs/tests/` as needed to lock **SC-002** behavior

**Checkpoint**: User Story 2 satisfied — failures are explicit; logs aid diagnosis.

---

## Phase 5: User Story 3 — Bindings & utilities (Priority: P3)

**Goal**: **`core::client`**, **`xml/soap`**, and generated **`mo`** stubs use tightened entry points; internal utilities compile ([spec.md](./spec.md) US3, **SC-004**).

**Independent Test**: Workspace builds with **`xml`**; representative **`unmarshal::<ConcreteType>`** call sites work ([spec.md](./spec.md) US3).

### Implementation for User Story 3

- [x] T012 [US3] Add **`cfg(feature = "xml")`** **`DataTypeAware`** bounds to **`unmarshal`**, **`unmarshal_array`**, and SOAP/XML helpers in `vim_rs/src/core/client.rs` and `vim_rs/src/xml/soap.rs` per `specs/001-metadata-xml-de/contracts/README.md` (re-verify **`contracts/README.md`** wording matches final rustdoc signatures)
- [x] T013 [US3] Regenerate managed-object stubs by running **`vim_build`** (`vim_build/`, `cargo run --bin generate --release`) so `vim_rs/src/mo/*.rs` matches new bounds; fix `vim_build` emitters if stubs fail type-check
- [x] T014 [US3] Update hand-written **`vim_rs`** utilities that call **`unmarshal`** / **`from_xml`** (search `vim_rs/src/` outside `vim_rs/src/mo/`—e.g. `vim_rs/src/core/`, `vim_rs/src/xml/client.rs`) for compatible explicit types

**Checkpoint**: User Story 3 satisfied — codegen + internal call sites aligned.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, changelog, and constitution-style validation.

- [x] T015 [P] Add rustdoc for **`DataTypeAware`** and XML entry-point contracts in `vim_rs/src/lib.rs` and/or `vim_rs/src/xml/mod.rs`
- [x] T016 [P] Record **`xml`** feature API/bound changes for advanced consumers in `CHANGELOG.md` (and pointer in `vim_rs/README.md` if present)
- [x] T017 Execute **`specs/001-metadata-xml-de/quickstart.md`**: `cargo test -p vim_rs --features xml`, `cargo check -p vim_rs --all-features`, and note build-time impact for release gate ([plan.md](./plan.md) Principle III)

---

## Phase 7: Registry & strict `Any` topology (FR-010–FR-013)

**Purpose**: PHF wrappers, **`lookup_xml_type`** order, enum-table removal, strict **`Any`** branching—prerequisites for deleting legacy probes.

- [x] T018 [P] In `vim_build/src/rs_emitter/api_registry.rs`, emit **`lookup_any_value_wrapper`** via **PHF** or equivalent **O(1)** static lookup per [research.md](./research.md) R9; regenerate `vim_rs/src/types/api_field_registry.rs`
- [x] T019 In `vim_build/src/rs_emitter/api_registry.rs`, reorder **`lookup_xml_type`** to **struct (`StructType`) → boxed wrapper → XSD primitives**; remove standalone enum-string table from codegen (**FR-013**); regenerate `vim_rs/src/types/api_field_registry.rs`
- [x] T020 [US1] In `vim_rs/src/xml/de.rs`, implement strict **`ApiFieldType::Any`** branching on **`lookup_xml_type`** (**FR-011**); enforce field-site **`Array`** vs **`ArrayOf*`** under **`Any`** (**FR-012**) via `stream_drive_any_typed` / typed paths

**Checkpoint**: Registry and **`Any`** topology are metadata-driven; **legacy probe helpers may still exist** until **Phase 8** removes them (**FR-014**).

---

## Phase 8: Completion gate — zero residual probing (FR-014, SC-003) 🎯 Feature done

**Purpose**: Meet spec **Goal**, **FR-014**, and **SC-003**: **no** ordered visitor trials, **no** `seq`-then-single structural probing, **no** legacy probe subdrivers in shipped **`vim_rs/src/xml/de.rs`**.

**Independent Test**: `rg "deliver_text\\(|stream_drive_legacy_probe|try_seq_or_single"` on `vim_rs/src/xml/de.rs` returns **no** matches (after renaming/refactoring, equivalent patterns must also be absent); `cargo test -p vim_rs --features xml` and `cargo check -p vim_rs --all-features` pass.

- [x] T021 [US1] Remove **`deliver_text`** (multi-visitor trial leaf coercion) from `vim_rs/src/xml/de.rs`; ensure every leaf uses **`deliver_text_typed`** or a single visitor path derived only from declared **`ApiFieldType`** (including **`accumulate_text`** / `#text` helpers that delegate without probing)
- [x] T022 [US1] Eliminate **`stream_drive_legacy_probe`** from `vim_rs/src/xml/de.rs` by routing **`ApiFieldType::Any`** and remaining **`Array`** edge cases through **`lookup_xml_type`** + **`ApiFieldType`** only (see **R11**, **R12** in [research.md](./research.md))
- [x] T023 [US1] Remove **`try_seq_or_single`** and reshape **`stream_children`** / **`stream_children_typed`** in `vim_rs/src/xml/de.rs` so sequence vs single-element handling follows **only** declared **`ApiFieldType::Array`** vs non-array—no structural probing
- [x] T024 [P] [US1] Extend or adjust tests in `vim_rs/tests/` and/or `#[cfg(test)]` in `vim_rs/src/xml/de.rs` so **`Any`** + array paths remain covered after probe removal (**SC-001**)
- [x] T025 [P] [US1] Update **`vim_rs/docs/METADATA_DRIVEN_XML_DE.md`** to document **zero probing** completion and remove any “legacy probe remains” wording; align **`specs/001-metadata-xml-de/quickstart.md`** Phase 8 verification line if needed

**Checkpoint**: Feature **complete** per **spec** — **SC-003** passes; **`vcsim_compat`** remains limited to documented finish/build swallowing only (**spec Goal**).

---

## Dependencies & Execution Order

### Phase Dependencies

| Phase | Depends on | Notes |
|-------|------------|--------|
| Phase 1 | — | Start immediately |
| Phase 2 | Phase 1 | Blocks all user stories |
| Phase 3–5 | Phase 2 | US1 → US2 → US3 (same files: prefer sequential or tightly coordinated) |
| Phase 6 | Phases 3–5 (desired scope) | Polish |
| Phase 7 | Phase 3 minimum | Registry work can overlap Phase 6 |
| **Phase 8** | **Phase 7** | **Deletes probes**; blocks declaring the feature **done** |

### User Story Dependencies

- **US1**: Phases 3 + 7 + **8** — **Phase 8** closes **FR-014** for `de.rs`.
- **US2**: Builds on US1-shaped **`de.rs`**.
- **US3**: Requires Phase 2 + working **`from_xml`** (Phase 3).

### Parallel Opportunities

- **T024** [P] parallel with **T025** after **T021**–**T023** land (different files).
- **T018**/**T019** were parallelizable with doc tasks during Phase 7.

---

## Parallel Example: Phase 8

After **T021**–**T023** complete:

```bash
# Parallel:
# - T024 tests under vim_rs/tests/ or vim_rs/src/xml/de.rs
# - T025 METADATA_DRIVEN_XML_DE.md + quickstart touch-up
```

---

## Implementation Strategy

### MVP (historical)

Phases 1–3 delivered metadata-aware descent; Phases 4–7 added observability, client bounds, registry PHF, and strict **`Any`** topology.

### Current focus — finish the feature

1. Complete **Phase 8** (**T021**–**T025**) until **SC-003** / **FR-014** are satisfied.
2. Run **`cargo test -p vim_rs --features xml`** and **`cargo check -p vim_rs --all-features`** (**T017** pattern).
3. Release notes / changelog patch if behavior-visible (**SC-004**).

---

## Task Summary

| Scope | Task IDs | Count |
|-------|----------|------:|
| Setup | T001–T002 | 2 |
| Foundational | T003–T005 | 3 |
| US1 (Phase 3) | T006–T009 | 4 |
| US2 | T010–T011 | 2 |
| US3 | T012–T014 | 3 |
| Polish | T015–T017 | 3 |
| Registry / strict Any | T018–T020 | 3 |
| **Completion gate (FR-014)** | **T021–T025** | **5** |
| **Total** | | **25** |

### Task counts mapped to user stories

| Story | Task IDs | Notes |
|-------|----------|--------|
| US1 | T006–T009, T020, **T021–T024** | **T021–T024** = probe removal + tests |
| US2 | T010–T011 | |
| US3 | T012–T014 | |

---

## Notes

- Do **not** hand-edit generated **`vim_rs/src/types/*.rs`** or **`vim_rs/src/mo/*.rs`** except via **`vim_build`** regeneration (constitution Principle I).
- **`lookup_xml_type`** / field registry behavior must stay consistent with **`vim_rs/docs/METADATA_DRIVEN_XML_DE.md`**; resolve conflicts by updating **`vim_build`** emission (`vim_build/src/rs_emitter/api_registry.rs`), not one-off patches in generated output.
