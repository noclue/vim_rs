# Feature Specification: Metadata-guided XML deserialization

**Feature Branch**: `001-metadata-xml-de`  
**Created**: 2026-05-09  
**Status**: Draft  
**Input**: User description: "Improve the XML deserialization code in the library's XML module, especially the main deserializer, so it walks the XML tree while tracking data types from the API model—eliminating guessing and probing behavior. Design notes are captured in the repository documentation."

## Goal (non-negotiable)

The XML deserializer **must** drive miniserde visitors using **only** generated **`ApiFieldType`** metadata and the supported lookups (**`lookup_api_field`**, **`lookup_xml_type`**, **`StructType::from_str`** / **`child_of`**). It **must not** infer types, sequence shape, or leaf coercion by **probing** visitor methods in trial order, by **trying** `seq()` and falling back to a single element, or by any **legacy probe** subdriver. **All** such probing code paths are **in scope for complete removal** before this feature is **done**—not as optional follow-up. Compatibility features (**`vcsim_compat`**) may only apply the **already-documented** tolerance (e.g. swallowing `build`/`finish` errors at stream-safe boundaries); they **must not** reintroduce type or shape guessing.

## Clarifications

### Session 2026-05-09

- **Q**: What should failures expose to callers, given serializer constraints? → **A**: Public errors remain the existing unit-style deserialization failures (no structured detail in the error value). Debugging and observability rely on **logging** at appropriate levels where extra context is useful.
- **Q**: Who must migrate entry-point usage when XML interfaces tighten? → **A**: In practice the **only consumers** of XML deserialization are **generated managed-object bindings** and **occasional utility code inside vim_rs**. It is **in scope** to update code generation and those utilities so they use the XML entry points correctly—this is not primarily an open-ended external-integrator migration program.
- **Q**: How is the root API type known at deserialization boundaries? → **A**: Entry points such as the generic response unmarshalling helper are **generic over the result type**; the root logical type is fixed by the **Rust type parameter** at each call site (for example an explicit `let result: AgentConfigInfo = unmarshal(...)`), so the driver always has a declared root type without runtime guessing.

### Session 2026-05-09 (typed lookup & `_value` topology)

- **Q**: How should `lookup_any_value_wrapper` be implemented for thousands of keys? → **A**: Emit a **PHF (perfect-hash) map** at codegen time (same general approach as `StructType::from_str`) so boxed `Primitive*` / `ArrayOf*` names resolve in **O(1)** without a giant `match`.
- **Q**: When must the XML driver emit the JSON wrapper shape (`_typeName` + `_value`) vs a plain struct map? → **A**: If `xsi:type` (local name) resolves through **`lookup_xml_primitive`** or **`lookup_any_value_wrapper`**, the payload follows the **boxed / non-structure JSON topology**: drive **`_value`** (and the usual `_typeName`) like JSON `Any` values. If it resolves through **`StructType::from_str`** (a real structure type), the driver emits a **normal struct field map** for that subtree—**no** synthetic `_value` wrapper solely because the slot was `Any`; structure-shaped types use ordinary child keys.
- **Q**: Does `ApiFieldType::Array(inner)` always imply the `_value` array-wrapper shape? → **A**: **No.** **Field-site arrays** (declared array field on a struct: repeated XML siblings, plain JSON arrays) **must not** be conflated with **`ArrayOf*` boxed wrappers** reached under an **`Any`** slot. Only the latter carry the **`_typeName` / `_value`** wrapper convention parallel to JSON; hardware lists and similar **schema arrays** stay **direct sequences** under the field.
- **Q**: Should standalone enum names participate in `lookup_xml_type` via a sorted name list? → **A**: **No—remove that path.** The XML schema carries symbol names such as `DasVmPriority`; forms like `DasVmPriority_enum` are **not** legal `xsi:type` values. Enum-like values deserialize as **`Str`** via normal field typing; **do not** add a separate enum-string table to `lookup_xml_type`.
- **Q**: May `lookup_xml_type` try **`StructType` first**, then **boxed wrappers**, then **XSD primitives**? → **A**: **Yes**, this order is **preferred**: maximize specificity for API struct names first, then `Any` box wrappers, then XSD primitives—provided tests confirm **no intentional cross-bucket name collisions** (expected to hold for the shipped API surface).

### Session 2026-05-09 (zero residual probing)

- **Q**: May any datatype / visitor probing remain in `xml/de.rs` after the metadata driver lands? → **A**: **No.** The feature is **not complete** until **all** probing is removed: no ordered visitor trials for leaves, no structural `seq`-then-single probing, no legacy probe subdrivers—only **`ApiFieldType`**-guided descent (plus documented **`vcsim_compat`** semantics that do **not** reinterpret wire shape).

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Predictable typed values from XML (Priority: P1)

When someone uses the library to turn XML responses into API-shaped values, each element and text node is interpreted according to the **declared type** for that position in the API model (fields, polymorphic bases, and dynamic slots that require an explicit wire type name). The process does not pick numeric versus textual versus boolean interpretation by trial and error.

**Why this priority**: Wrong coercion (for example treating a number as text because text was tried first) corrupts application logic and is hard to debug.

**Independent Test**: Given fixtures whose XML shape matches the API model, deserialized values match the same logical results as the non-XML deserialization path for equivalent payloads, for representative structs, enums, polymorphic objects, and optional fields.

**Acceptance Scenarios**:

1. **Given** a field declared as a numeric type in the API model, **When** XML for that field is deserialized, **Then** the value is interpreted as that numeric type without relying on order-dependent probes.
2. **Given** a field declared as an array in the API model, **When** the wire carries repeated sibling elements for that field, **Then** those elements are assembled as a sequence according to the declared element type, not by probing for sequence acceptance.
3. **Given** a polymorphic field with a declared base type, **When** the wire names a concrete subtype, **Then** the concrete type is accepted only if it is consistent with that base; otherwise deserialization fails (same class of failure as today’s deserialization errors; finer detail **may** appear in logs, not in the returned error value).

---

### User Story 2 - Explicit failures instead of silent mis-shaping (Priority: P2)

When the wire shape or type annotations do not match what the API model allows at that position (unknown element names for the declared struct, unresolvable type names, incompatible type refinement), deserialization **fails**. It does not fall back to treating arbitrary content as strings or other guesses. The **returned error value** does not carry rich deserialization diagnostics (same practical limitation as today); **logging** **may** record contextual detail for debugging and observability.

**Why this priority**: Silent acceptance hides producer bugs and creates security and correctness risks.

**Independent Test**: Curated negative fixtures (unknown child tag, bad type name, wrong refinement) each produce a failed result without claiming success.

**Acceptance Scenarios**:

1. **Given** a child element name that is not a known field for the declared parent type, **When** deserialization runs, **Then** it fails rather than storing opaque string data.
2. **Given** a type annotation on the wire that cannot be resolved to a known type for that context, **When** deserialization runs, **Then** it fails (without requiring structured detail in the error value).

---

### User Story 3 - Generated bindings and internal utilities use XML entry points correctly (Priority: P3)

The **only** call sites that deserialize XML responses today are **generated managed-object bindings** and **small amounts of utility code in vim_rs**. Work **includes** updating **code generation** and those utilities so every path uses the metadata-aware XML interfaces consistently (including generic unmarshalling where the **Rust type parameter** supplies the root API type). Maintainer-facing notes **may** summarize contract changes for advanced users; broad third-party migration guides are **not** the focus.

**Why this priority**: Keeps the typed driver coherent without imposing speculative external migration scope.

**Independent Test**: The workspace builds with XML enabled; generated stubs and in-tree utilities compile against the tightened entry points; spot-check representative stubs (for example types explicitly unmarshalled in bindings) still deserialize correctly.

**Acceptance Scenarios**:

1. **Given** the code generator and internal utilities, **When** the XML entry-point contract is finalized, **Then** they are updated in-repo so no hand-written workaround is required for normal API usage.
2. **Given** a generic unmarshalling call with an explicit result type, **When** XML is deserialized, **Then** descent starts from that declared root type without an additional runtime root-type probe.

---

### Edge Cases

- **Nil/absent elements**: Wire indicates an explicitly absent value where the schema allows absence; required fields still fail when absent, consistent with the non-XML path.
- **Repeated elements with the same name**: Policy for non-adjacent repeats is defined (consistent with duplicate-key behavior on the parallel JSON path) so behavior is predictable rather than implementation-defined.
- **Compatible transports**: Where both XML and non-XML deserialization exist for the same logical types, cardinality and duplicate-field behavior remain aligned unless explicitly documented otherwise.
- **Failure diagnostics**: Operators or developers rely on **logs** (where enabled) for pinpointing malformed XML or mismatched wire shape; the **`Result` error** remains a lightweight deserialization failure as today.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Deserialization **MUST** use the API model's declared type information at each step of the tree walk to decide structural shape (object, sequence, leaf typing, polymorphic refinement), rather than inferring shape solely from XML content or visitor trial responses.
- **FR-002**: For object-typed nodes, child elements **MUST** be mapped to fields using API-model field identity (wire names as defined by the model), not inferred from Rust-specific naming alone.
- **FR-003**: When the declared model requires a type name on the wire for a dynamic slot, that name **MUST** be resolved through the supported registry; missing or unknown names **MUST** cause failure.
- **FR-004**: When the declared model allows an optional concrete subtype name for a base object type, a present name **MUST** be validated as an allowed refinement of that base; incompatible names **MUST** cause failure.
- **FR-005**: Leaf values **MUST** be parsed according to the declared primitive or enumerated logical type for that field, without multi-step probing that could prefer a wrong interpretation.
- **FR-006**: Entry points used for response deserialization **MUST** supply the root API type through the **generic type parameter** (for example unmarshalling to a concrete struct or enum type at the call site), so the driver begins from that declared root without inferring it from the wire or from `Deserialize` alone.
- **FR-007**: Validation of required fields, optional presence, enum membership, and cross-field business rules **MAY** remain delegated to the same layer as today; the XML walk **MUST** focus on structural and lexical alignment with declared types.
- **FR-008**: SOAP envelope handling and separation of concerns **MUST** remain as today: envelope extraction stays outside the core tree driver scope described here.
- **FR-009**: Deserialization failures **MUST NOT** be required to embed rich structural detail in the public error type; **MAY** emit contextual information via **logging** for debugging and observability, consistent with project logging practices.
- **FR-010**: Code generation **SHOULD** implement **`lookup_any_value_wrapper`** with a **PHF map** (or equivalent **O(1)** static lookup) at compile time, analogous to **`StructType::from_str`**, so boxed type-name resolution stays fast at runtime.
- **FR-011**: For dynamic **`Any`** slots, resolution of `xsi:type` **MUST** branch on **`lookup_xml_type`** outcome: **structure types** (`StructType::from_str`) drive **normal struct-shaped** visitor traffic (child keys / attrs / `#text` per model); names resolved only via **primitive** or **boxed-wrapper** lookup **MUST** follow the **`_typeName` + `_value`** wrapper topology consistent with JSON **`Any`** value shapes—not the field-site array topology.
- **FR-012**: The driver **MUST** treat **field-site arrays** (`ApiFieldType::Array(inner)` at a declared struct field) as **repeated XML siblings** mapping to **plain JSON arrays** without imposing the boxed **`ArrayOf*` `_value`** convention; **`ArrayOf*` wrappers** apply to **`Any`**-boxed **value** shapes, not to ordinary schema array fields.
- **FR-013**: **`lookup_xml_type`** **MUST NOT** include a standalone **enum-name** fallback table; **MUST** use resolution order **struct (`StructType`) → boxed wrapper map → XSD primitives** unless profiling proves a different order is required—document any deviation.
- **FR-014**: The XML tree driver **MUST NOT** retain **datatype probing** or **visitor probing** implementation patterns in shipped code: no ordered trials over visitor methods for leaf text; no structural **`seq()`-first with fallback to a single element** to infer arrays; no **legacy probe subdrivers** (including helpers historically named or equivalent to **`deliver_text`**, **`try_seq_or_single`**, **`stream_drive_legacy_probe`**, **`stream_children`** when used only to probe shape). **`ApiFieldType::Any`** and every other slot **MUST** select visitor traffic solely via **`lookup_xml_type`** + **`ApiFieldType`** (and related registry lookups). **`vcsim_compat`** **MUST NOT** widen tolerances to reinterpret wire types or recover via probing.

### Key Entities *(include if feature involves data)*

- **API model type**: The logical type of a value (struct, enum, array, primitive, polymorphic base, dynamic slot) as defined by the generated API metadata—not the wire format alone.
- **XML tree position**: A location in the document (element, attributes, text) being consumed relative to a parent declared type.
- **Type refinement**: Optional wire-level subtype information that narrows a declared base type to a concrete subtype when allowed by the model.
- **Deserialization entry point**: A supported API for turning XML bytes or strings into a value of a named root logical type.
- **Field-site array**: An array declared on a struct field in the API model—XML uses repeated siblings; JSON uses a normal array without `ArrayOf*` boxing.
- **Boxed `Any` value**: A `Primitive*` / `ArrayOf*` (etc.) shape stored under `Any`—JSON uses `_typeName` + `_value`; XML follows the parallel convention when `xsi:type` resolves to those names.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: For a maintained suite of round-trip and fixture tests covering structs, enums, lists, polymorphism, and fault payloads, **100%** of positive fixtures deserialize without shape guessing; golden expectations remain stable relative to the non-XML path except where the old path was demonstrably wrong.
- **SC-002**: For negative fixtures representing unknown fields, bad type names, and illegal refinements, **100%** fail without silent string fallbacks; **test or log review** **may** confirm the failure mode, recognizing that the **public error** stays unit-style.
- **SC-003**: The feature is **not complete** until **`vim_rs::xml::de`** contains **zero** probe-first patterns: no ordered visitor trials for leaves, no sequence-vs-single structural probing, and **no** legacy probe helpers (**FR-014**). Verification is **mandatory** via code review and/or automated checks (for example `rg`/lint rules or tests that fail if listed symbols or equivalent patterns reappear). Partial removal or “`Any` still uses legacy probe” states **fail** this criterion.
- **SC-004**: Generated bindings and in-repo utilities compile and behave correctly under the updated contract; brief maintainer or changelog notes capture any user-visible entry-point change for advanced consumers.

## Assumptions

- **Mandatory probe removal (completion gate)**: Implementation **must** finish **all** of: (1) **PHF** (or equivalent) for **`lookup_any_value_wrapper`** in `vim_build`. (2) **`lookup_xml_type`** per **FR-013** with **enum-name** table **removed**. (3) **`ApiFieldType::Any`** in `de.rs` driven **only** by **`xsi:type` → `lookup_xml_type`**: **`Object(StructType)`** → normal struct descent; **primitive** or **boxed wrapper** → **`_typeName` + `_value`** per **FR-011**; **`Array`** uses **wrapper** topology **only** for boxed **`ArrayOf*`** under **`Any`**, never for **field-site arrays** (**FR-012**). (4) **Delete** probing helpers per **FR-014**—no residual callers. (5) Fixtures cover **`Any`**: struct, **`ArrayOf*`**, primitive, and **field-site arrays** outside **`Any`**. This list is **blocking** for **SC-003**; it is **not** optional cleanup after merge.

- The API metadata already distinguishes field types, inheritance, array fields vs boxed array value types, and dynamic slots; the XML driver consumes that metadata rather than duplicating a parallel schema.
- The JSON-oriented deserialization path remains the behavioral reference for optional vs required enforcement and enum parsing; this feature does not broaden XML-specific semantic validation beyond structural/type alignment unless explicitly specified later.
- A compatibility mode for known-broken test doubles may exist but must not reinterpret known good elements; scope is limited to dropping or skipping explicitly marked malformed fragments as today.
- Performance targets match current streaming behavior: no requirement to materialize the full document as a tree for the common case.
- **Consumers**: XML deserialization is exercised primarily by **generated bindings** and **vim_rs utilities**; aligning those layers with any new trait or bounds is expected work, not optional follow-up.
- **Errors**: The stack continues to surface **miniserde-style unit errors** to callers; richer failure context is out-of-band (**logging**), not an expanded `Error` type in this effort.

## Out of Scope

- Leaving **any** probe-first or legacy-probe code paths in production XML deserialization “for later”; **full removal** per **FR-014** / **SC-003** is **in scope** and **required** for completion.
- Replacing the underlying XML reader library or the value-building strategy used with the deserialization API, except where required to implement metadata-driven decisions.
- Rich, structured **error values** (source spans, path traces, or nested causes in the public `Result`) as a dedicated deliverable; **logging** covers operational and developer diagnostics for this phase.
- Merging distinct metadata registries into a single table in the first iteration, if the project currently keeps separate responsibilities.
- Organization-wide or open-ecosystem migration programs beyond updating **this repository’s** generator output and internal helpers.
