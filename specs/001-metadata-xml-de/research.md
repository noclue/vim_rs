# Research: Metadata-guided XML deserialization

**Feature**: [spec.md](./spec.md)  
**Date**: 2026-05-09  
**Primary reference**: `vim_rs/docs/METADATA_DRIVEN_XML_DE.md`

This document records decisions that resolve implementation forks. Each item follows: **Decision**, **Rationale**, **Alternatives considered**.

---

## R1 — Root type for XML entry points

**Decision**: Introduce a generated trait **`DataTypeAware`** (`fn data_type() -> ApiFieldType`) implemented for all types that can appear at XML unmarshalling boundaries. Entry points require **`T: Deserialize + DataTypeAware`** when `feature = "xml"` (conditional compilation preserves JSON-only builds).

**Rationale**: `Deserialize` alone does not expose VIM schema identity; the driver must start from a declared **`ApiFieldType`**. The generic parameter **`T`** at call sites (e.g. `unmarshal::<AgentConfigInfo>`) supplies the correct root type at compile time.

**Alternatives considered**:  
- Untyped `from_xml` shim defaulting to **`Any`** — **rejected** (reintroduces guessing for every root).  
- Runtime registry from `TypeId` — **rejected** (fragile, easy to drift from codegen).

---

## R2 — Which metadata drives XML shape?

**Decision**: Use **`api_field_registry`** / **`lookup_api_field(StructType, wire_field_name)`** for object fields and **`lookup_xml_type`** for **`xsi:type`** local names. Do **not** use **`TYPE_REGISTRY`** to drive XML element shape (it remains for JSON/miniserde dynamic deserializers).

**Rationale**: Field shapes and arrays are expressed as **`ApiFieldType`** on struct fields; **`ArrayOf*`** wrappers vs field-site arrays are distinguished in the design doc. Keeps a single source of truth aligned with OpenAPI field metadata.

**Alternatives considered**:  
- Merging registries — **deferred** (explicit non-goal for first iteration).

---

## R3 — Leaf text parsing

**Decision**: Parse leaf text using the **single** visitor path matching the declared **`ApiFieldType`** (e.g. bool vs string vs int); remove ordered probing (`deliver_text` style).

**Rationale**: Probe order can bind overly permissive visitors (e.g. string before int) and corrupt values.

**Alternatives considered**:  
- Retain probing as fallback — **rejected** by spec (FR-005).

---

## R4 — Repeated sibling elements

**Decision**: **Last-write-wins** for duplicate field names across non-adjacent runs: each run opens **`key → seq → finish`**; later runs overwrite—aligned with duplicate JSON keys and miniserde behavior.

**Rationale**: Predictable policy without buffering entire subtrees; matches design doc.

**Alternatives considered**:  
- Error on non-adjacent repeats — **rejected** (over-constrains real producers).  
- Full merge — **rejected** (complexity / buffering).

---

## R5 — Failure reporting

**Decision**: Keep **`miniserde::Error`** / existing **`Result`** shape for failures; add optional **`tracing`** on failure paths for context (path, types). No structured error payloads in this phase.

**Rationale**: Spec clarification; aligns with constitution errors guidance and build constraints.

**Alternatives considered**:  
- Rich error enums with spans — **out of scope** (spec / constitution defer).

---

## R6 — DOM vs streaming

**Decision**: Stay **streaming** with **`quick_xml::reader::NsReader`**; avoid DOM for the primary path; small local buffering only if an edge case requires it.

**Rationale**: Performance and memory; matches current architecture.

**Alternatives considered**:  
- Full DOM — **rejected** (non-goal).

---

## R7 — SOAP boundaries

**Decision**: Leave envelope stripping and transport framing in **`xml/soap.rs`**; **`de.rs`** consumes already-extracted element streams consistent with today.

**Rationale**: Separation of concerns (FR-008).

**Alternatives considered**: None material.

---

## R8 — Consumer scope

**Decision**: Update **`vim_build`** managed-object emitters and **in-repo** `vim_rs` utilities only; document breaking generic bounds in **CHANGELOG** for advanced downstream users.

**Rationale**: Spec clarification — no broad migration program.

**Alternatives considered**: Long-form external migration guide — **not** primary deliverable.

---

## R9 — `lookup_any_value_wrapper` scale

**Decision**: Emit **`lookup_any_value_wrapper`** as a **compile-time perfect-hash (PHF) map** or equivalent **O(1)** static lookup (same general strategy as **`StructType::from_str`**) so thousands of boxed **`Primitive*` / `ArrayOf*`** names do not ship as one enormous **`match`**.

**Rationale**: Runtime cost and compile-time size both scale poorly with a linear match over the full wrapper vocabulary; **FR-010** explicitly prefers PHF.

**Alternatives considered**:  
- Giant **`match`** — **acceptable interim** (already used for bootstrap); **not** the long-term solution.  
- Binary search on sorted static slice — **possible** but worse constants than PHF for hot paths.

---

## R10 — `lookup_xml_type` buckets and order

**Decision**: Resolve `xsi:type` local names in order **(1)** **`StructType::from_str`** (real API structs), **(2)** boxed wrapper map (**`lookup_any_value_wrapper`** / PHF), **(3)** XSD primitive aliases (**`lookup_xml_primitive`**). **Do not** add a separate enum-name table in **`lookup_xml_type`**—enum symbols are not legal standalone **`xsi:type`** refinements in the shipped schema; enum-like values use **`Str`** at field sites (**FR-013**, spec clarifications).

**Rationale**: Maximizes specificity for struct names first; keeps wrapper and primitive buckets disjoint by construction; avoids misleading accepts for pseudo-types like `*_enum`.

**Alternatives considered**:  
- Enum table — **rejected** (spec clarification).  
- Primitive-before-struct — **rejected** (risks shadowing struct names if collisions existed—tests must guard order).

---

## R11 — `Any` slot: struct map vs `_typeName`/`_value`

**Decision**: After **`lookup_xml_type`**, if the name resolves to a **structure** type, drive **normal struct-shaped** XML→visitor traffic (children per **`lookup_api_field`**). If it resolves only to a **primitive** or **boxed wrapper** type, synthesize the JSON-parallel **`_typeName` + `_value`** visitor shape. **Never** apply the boxed **`ArrayOf*`** **`_value`** convention to **field-site** **`ApiFieldType::Array`**—those stay repeated siblings → plain array (**FR-011**, **FR-012**).

**Rationale**: Matches JSON **`Any`** semantics and distinguishes schema arrays from dynamically boxed array values.

**Alternatives considered**:  
- Single probe path for all **`Any`** payloads — **rejected** (**FR-014**, **SC-003**).

---

## R12 — Zero residual probing (completion gate)

**Decision**: The XML driver **must not** ship ordered visitor trials for leaf text, **`seq()`-first fallback to infer arrays, or legacy probe subdrivers** (`stream_drive_legacy_probe`, `deliver_text` where it probes multiple visitor kinds, `try_seq_or_single`, probe-shaped `stream_children`). **`ApiFieldType::Any`** and all other slots dispatch **only** via **`lookup_xml_type` + `ApiFieldType`** (and **`lookup_api_field`** / **`child_of`** as specified). **`vcsim_compat`** keeps documented tolerance at safe boundaries only—no shape reinterpretation.

**Rationale**: Matches **spec Goal**, **FR-014**, and **SC-003**; eliminates nondeterministic coercion and aligns JSON/XML parity.

**Alternatives considered**:  
- Keep legacy probe until “later” — **rejected** (feature incomplete per **SC-003**).  
- Probe only under **`vcsim_compat`** — **rejected** (would violate **spec Goal**).
