# Metadata-Driven XML Deserialization Design

## Goal

Rework `vim_rs/src/xml/de.rs` from a visitor-probing XML adapter into a
schema-guided recursive descent driver. The driver streams `quick_xml`
events into the same miniserde visitors that the JSON path uses, but every
shape decision (open a map, open a sequence, deliver text as bool / int /
string) comes from generated VIM type metadata.

**Scope.** This spec is about *XML → miniserde JSON-visitor driving only*.
It is not a redesign of vim_rs deserialization. In particular, the driver:

- Translates an XML element into the same sequence of `Visitor` /
  `Map` / `Seq` calls miniserde would receive from JSON. `xsi:type` is
  delivered as the synthetic `_typeName` key, attributes as `@name` keys,
  text as `#text` (or directly to a primitive visitor for typed leaves).
- Does not validate cardinality (required vs optional fields). That check
  happens in miniserde when the generated builder finishes the structure.
- Does not validate enum values. Enums travel as `ApiFieldType::Str` and
  any enum-name validation happens in the generated miniserde builder.
- Does not validate cross-field invariants, ranges beyond integer-fits, or
  semantic correctness — same delegation to miniserde.

The core contract preserved across the rewrite:

- XML is read with `quick_xml::reader::NsReader`.
- Rust values are built by driving `miniserde::de::Visitor`.
- `xsi:type` is the XML spelling of miniserde's synthetic `_typeName`.
- SOAP envelope extraction remains in `xml/soap.rs`.

The change is about the driver. Today `xml/de.rs` asks the consumer "will
you accept a sequence?", "will you accept this text as a string?", "will
this map build?" and infers shape from the answers. The target design is
that every descent step carries a concrete `ApiFieldType` from generated
metadata and calls exactly one visitor operation per element.

## Implementation status (in-tree, `feature = "xml"`)

The metadata-guided driver lives in `vim_rs/src/xml/de.rs`. `stream_drive` takes a declared
`ApiFieldType` and the miniserde visitor; roots use `DataTypeAware::data_type()`.

| Concern | Behavior |
|--------|----------|
| **`lookup_xml_type`** (generated [`api_field_registry.rs`](../src/types/api_field_registry.rs)) | Resolves `xsi:type` **local names** in order: **`StructType::from_str` → `ANY_VALUE_WRAPPER_MAP`** (PHF over boxed `Primitive*` / `ArrayOf*`) **→ `lookup_xml_primitive`** (XSD aliases). **No** standalone enum-name table for `xsi:type`; enum-like payloads use field typing as **`ApiFieldType::Str`** (same as JSON). |
| **Boxed wrappers** | `lookup_any_value_wrapper` is emitted as a compile-time **`phf::Map`** (`ANY_VALUE_WRAPPER_MAP`). |
| **`ApiFieldType::Any`** | Non-nil elements require **`xsi:type`**. After `lookup_xml_type`: **`Object(st)`** → struct descent; **primitive `ApiFieldType`** → **`typed_leaf_via_map`** (JSON-aligned `_typeName` / `#text` via **`deliver_text_typed`** only); **`Array(inner)`** (boxed `ArrayOf*`) → **`stream_drive_any_array_typed`** (`_typeName` + **`_value`** `seq`, homogeneous sibling elements as **`inner`**). |
| **Diagnostics** | [`log_xml_deser_failure`](../src/core/wire_log.rs) on log target **`vim_rs::wire::soap`**; public errors remain unit-style. |

## Current State

### Typed descent (shipped)

- `from_xml` / SOAP helpers require **`Deserialize + DataTypeAware`** and call **`stream_drive(reader, start, visitor, declared)`** with the appropriate **`ApiFieldType`** (root: `T::data_type()`).
- Object fields use **`lookup_api_field(StructType, wire_name)`**; primitives use **`deliver_text_typed`** (no probe order).
- **`ApiFieldType::Any`** uses **`stream_drive_any_typed`** / **`drive_empty_any_typed`** (strict `xsi:type` + `lookup_xml_type`), including boxed **`ArrayOf*`** via **`stream_drive_any_array_typed`** / **`drive_empty_any_array_typed`**.

Visitor probing helpers (**`deliver_text`**, **`try_seq_or_single`**, **`stream_children`**, **`stream_drive_legacy_probe`**) have been **removed** (**FR-014**, **SC-003**). **`vcsim_compat`** remains limited to **`finish_map_or_tolerate`** / documented tolerant boundaries — no wire-shape reinterpretation.

- `types/deserialize.rs` emits `TYPE_REGISTRY` and `lookup_type`. This maps VIM
  names such as `VirtualMachine`, `string`, `ManagedEntityStatus`, and
  `ArrayOfString` to `TypeInfo`. It is the correct source for dynamic named
  values and `VimAny`/`ValueElements`.
- `types/api_field_registry.rs` emits `lookup_api_field(StructType, field_name)`
  over `ApiFieldType`. This maps struct fields to primitive/object/array field
  metadata and walks the inheritance chain. A field declared as `Any` is better
  treated as an abstract dynamic slot that requires `xsi:type`, not as a
  concrete field type.

Those registries are complementary, not interchangeable. `ArrayOf*` values are
named value types in `TYPE_REGISTRY`, but arrays in object fields appear as
`ApiFieldType::Array(&inner)` in the field registry. `StructType` resolves only
object structs, not `ArrayOf*` value wrappers or primitive aliases.

## Non-Goals

- Do not build a DOM as the primary implementation. Some small buffering may be
  needed for specific ambiguous dynamic cases, but the normal path should remain
  streaming and O(depth).
- Do not merge `TYPE_REGISTRY` and `api_field_registry` into one table in the
  first implementation. Their keys and responsibilities differ.
- Do not make XML deserialization depend on generated Rust field names. The XML
  layer should use wire names from the API model.
- Do not hide producer ambiguity with more guessing. Unknown shape is an
  error. The only tolerated boundary is `vcsim_compat`, which drops a
  malformed *complete* element after the reader has cleanly consumed it;
  it never re-interprets a known element.

## Design Decisions

1. What root type does `from_xml::<T>` know?
   `miniserde::de::Deserialize` alone does not expose a VIM schema type, so
   the parser cannot start descent without help. The XML driver requires a
   second trait, `DataTypeAware`, generated alongside the miniserde
   `Deserialize` impls, exposing the type's declared `ApiFieldType`. The
   parser entry point is `from_xml<T: Deserialize + DataTypeAware>`. There is
   no untyped compatibility shell — the previous `from_xml<T: Deserialize>`
   shape is replaced.

   `unmarshal` in `core/client.rs` (the single entry point used by every
   generated managed-object stub) tightens its bound the same way under
   `feature = "xml"`. Concretely, `T: Deserialize + DataTypeAware` when XML
   is enabled, `T: Deserialize` otherwise. The conditional bound is sound
   because the JSON path does not consult `DataTypeAware` and the XML feature
   pulls in the generated `DataTypeAware` impls for every type that can
   appear at an `unmarshal` boundary.

   **This is a deliberate public API change** under `feature = "xml"`.
   `from_xml`, `from_xml_with`, `unmarshal`, `unmarshal_array`,
   `vim_response`, `vim_response_with`, and `vim_response_list` are all
   `pub`. Tightening their generic bound from `T: Deserialize` to
   `T: Deserialize + DataTypeAware` will reject downstream call sites
   that pass a type without a `DataTypeAware` impl. In practice that
   means:
   - **`Vec<T>` and `Option<T>` roots are no longer accepted.**
     Generated bindings already wrap arrays via `unmarshal_array<U>`
     (where `U: DataTypeAware` is the element type, not the container)
     and unwrap `Option` at the binding layer before calling
     `unmarshal`, so generated code is unaffected. Downstream code that
     calls `from_xml::<Vec<MyStruct>>` or `unmarshal::<Option<MyStruct>>`
     directly must switch to the element-typed form, or the surrounding
     wrapper helper. We are accepting this break in exchange for a
     monomorphic typed driver and the simpler `&'static ApiFieldType`
     storage that allowance enables.
   - **No re-exported untyped `from_xml` shim.** A shim that defaults
     `data_type()` to `Any` would re-introduce the dynamic-resolution
     path for every unparameterized root and defeat the schema-first
     design.
   - **Internal call sites are already covered.** The only in-tree
     consumers are `from_xml::<MethodFault>`, `from_xml::<String>` (via
     hand-written primitive impl), `from_xml::<VimAny>`, and
     `unmarshal::<ManagedObjectReference>` plus generated
     stub-internal calls — all of which gain a `DataTypeAware` impl in
     step 1.

   The step-0 spike compiles every workspace member (vim_rs library,
   examples crates, MCP server crates) with the new bound. Downstream
   public-API consumers outside the workspace are not visible to the
   spike; the breakage is documented in the next release notes.

2. How are field values typed during descent?
   The parser starts from the root type's `ApiFieldType` and walks down
   schema-first. While inside an object, every child element's type comes
   from `lookup_api_field(struct_type, child_name)`. For example
   `Event.userName` is `Str` and `ServiceContent.rootFolder` is
   `Object(ManagedObjectReference)`, so the parser never has to guess
   element shape.

   The XML type registry (`lookup_xml_type`) is only consulted when an
   element carries `xsi:type`. The rules are:

   - **declared `Any`**: `xsi:type` is mandatory; resolve via
     `lookup_xml_type` and continue with the returned `ApiFieldType`.
   - **declared `Object(base)`**: `xsi:type` is optional. If absent, descend
     as `base`. If present, resolve to a `StructType` via
     `StructType::from_str` and verify it is `child_of(base)` (descendant
     or equal).
   - **declared primitive**: `xsi:type`, if present, is stripped to its
     local part and must resolve to the same primitive `ApiFieldType`
     via `lookup_xml_type`.
   - **declared `Array(inner)`**: `xsi:type` is normally absent for
     field-site arrays. Wrapper-style arrays (`<val xsi:type="ArrayOfX">`)
     are reached via `Any` resolution above and never appear at a struct
     field site.

   `TYPE_REGISTRY` is not used by the XML driver at all. It still owns
   miniserde's dynamic deserializer construction (`make_deserializer`,
   `from_value`); the XML driver builds shape decisions from `ApiFieldType`
   and never needs the callbacks.

3. Are non-adjacent repeated elements accepted?
   The driver uses **last-write-wins (LWW)**. Each adjacent run of
   repeated children with the same name is delivered as one
   `key(name) → seq() → … → seq.finish()` cycle. If the same name
   reappears later under the same parent, the driver simply opens a new
   `key(name) → seq() → … → seq.finish()` cycle; miniserde overwrites
   the field value. No state is kept between runs, no parse error is
   raised. This is the simplest robust policy and matches miniserde's
   behavior for duplicate JSON keys, so XML and JSON transports stay
   indistinguishable on this axis. Producers in practice keep array
   members adjacent; the LWW rule is just the formal answer for the
   "what if they don't" case.

4. What should happen when metadata is missing?
   Error out. If `lookup_api_field` returns `None` for a child element name,
   or `lookup_xml_type` returns `None` for an `xsi:type` value, the parse
   fails with a descriptive error. No silent stringification, no fallback
   probing.

5. Do we need XML aliases beyond current field names?
   `lookup_xml_type` carries the local-part forms of XML primitive type
   names — `string`, `int`, `long`, `boolean`, `dateTime`,
   `base64Binary`, etc. — because they differ from VIM struct names.
   The wire `xsd:`-prefixed forms (`xsd:string`, …) are stripped to
   their local part by the driver before lookup, so they are **not**
   keys in the registry. Field-name aliases are not added speculatively;
   only when real fixtures prove a producer uses one.

6. How do we want diagnostics to look?
   The public error stays **`miniserde::Error`**. Implementation detail:
   **`crate::core::wire_log::log_xml_deser_failure`** emits **debug** lines on log target
   **`vim_rs::wire::soap`** with safe context (element/type hints; truncated; never secrets).
   Rich structured errors in `Result` remain out of scope for the parser pivot.

7. Where does validation live?
   The XML driver only cares about *shape*: which visitor method to call
   for each element. Anything stronger is delegated:
   - **Required vs optional fields**: enforced by miniserde's generated
     `build()`/`finish()` when the structure shape closes.
   - **Enum membership**: enforced by the miniserde builder for the
     enum-typed Rust field. The XML driver delivers the raw string.
   - **Cross-field invariants**, exhaustive-match checks, semantic
     validity: not enforced by either layer; same as the JSON path.

   This keeps XML and JSON transports producing identical errors for the
   same payloads, and it lets the spec stay focused on the parser pivot.

8. How is `xsi:nil` handled?
   Schema-instance `nil="true"` (or `"1"`, case-insensitive after
   trim) is the XML idiom for an absent value. The driver treats it as
   one short-circuit at the start of `drive_element`: when the
   attribute is present and truthy, consume the element body (which
   per XSD must be empty) and call `visitor.null()`. The element ends
   immediately; no further attribute / text / child processing
   happens.

   `visitor.null()` is forwarded to whatever the field visitor is. For
   `Option<T>` fields the generated builder accepts it and stores
   `None`. For non-optional fields miniserde rejects null and the
   parse fails — which is the desired behavior, identical to the JSON
   path receiving an explicit `null` for a required field. This means
   `ApiFieldType` does not need an `Optional(_)` variant: optionality
   stays in the Rust type and miniserde enforces it.

   `xsi:nil="false"` is treated the same as the attribute being
   absent. Any other value is an error (the wire form is binary).

## Proposed Architecture

The XML parser should be a declared-type recursive descent parser. Every descent
step receives an `ApiFieldType` from generated metadata. There is no untyped
descent mode and no XML-specific context such as `Unknown`, `NamedValue`, or a
separate dynamic mode.

At each element, the parser starts from the declared type and optionally refines
it with `xsi:type`:

- Primitive declared type: parse the element text as that primitive. If
  `xsi:type` is present, it must be a compatible XML primitive alias.
- Object declared type: deserialize as the declared `StructType` unless
  `xsi:type` names a descendant struct. If `xsi:type` is present, resolve it to
  `StructType`, verify it is equal to or a child of the declared base, then use
  that effective `StructType` for field lookup.
- Array declared type: open a sequence and deserialize each array element using
  the array inner `ApiFieldType`. The same declared/refined rules apply to each
  element.
- Declared `Any`: not deserialized as `Any`. It is a marker that says the
  element must carry `xsi:type`. Resolve that type name through the XML type
  registry (see below) to a concrete `ApiFieldType` and continue descent with
  that.

`ManagedObjectReference` does not need its own `ApiFieldType` variant.
`ManagedObjectReference` is already a `StructType`, so an MOR field is just
`Object(StructType::ManagedObjectReference)`. Its compact XML shape
(`<x type="VirtualMachine">moid</x>`) is handled by the generated MOR
deserializer reading `@type` + `#text`, the same map shape every other struct
gets.

The key point: polymorphism is not a separate XML mode. `xsi:type` is simply the
standard refinement mechanism for a declared type. This is true for `Any`, but it
is equally true for ordinary base types such as `VirtualDevice`,
`ManagedEntity`, `Event`, or `MethodFault`.

### Root Type

The XML driver requires a generated `DataTypeAware` trait alongside the
existing miniserde `Deserialize` impls:

```rust
#[cfg(feature = "xml")]
pub trait DataTypeAware {
    fn data_type() -> ApiFieldType;
}
```

The set of impls that has to land is exactly the set of Rust types that
appear at an XML entry point (`unmarshal`, `unmarshal_array`, `from_xml`,
`vim_response*`). Concretely, the impl surface is the **method
return-type surface** — every Rust type the generated bindings can return
must implement `DataTypeAware`. That set includes:

- Each generated VIM struct returns `Object(StructType::Self_)`.
- Each generated VIM enum returns `Str`.
- `VimAny` returns `Any`.
- Each `Box<dyn FooTrait>` boundary type returns the trait's base
  `Object(StructType::Foo)`. vSphere methods often return the polymorphic
  base of an inheritance hierarchy (`Box<dyn IssueTrait>`,
  `Box<dyn ApplyProfileTrait>`, `Box<dyn VsanMountPrecheckResultTrait>`,
  `Box<dyn SmsProviderInfoTrait>`, ...), with `xsi:type` selecting the
  concrete subtype on the wire. The trait-level `DataTypeAware` impl just
  names the base struct; the driver's `Object(base)` path then refines via
  `xsi:type` element-by-element.
- Hand-written impls under the `xml` feature for the primitive Rust types
  matching `ApiFieldType` variants: `bool`, `i8`, `i16`, `i32`, `i64`,
  `f32`, `f64`, `String` (returning `Str`), and `Base64` (returning
  `Binary`).

In other words: if a type can appear in the return position of a
generated managed-object method, it must implement `DataTypeAware`. The
generator that emits managed-object stubs is the same generator that has
to emit `DataTypeAware` impls — they share the same return-type closure.

`Vec<T>` and `Option<T>` do not need `DataTypeAware` impls: `unmarshal_array<U>`
takes `U: DataTypeAware` and iterates the SOAP `<returnval>` list itself, and
`Option<_>` is unwrapped at the binding layer before `unmarshal` is called.
Avoiding container impls sidesteps the `&'static ApiFieldType` storage
problem for generic containers.

This gives the parser a real root schema without trying to infer Rust type
identity from `T: Deserialize`, which Rust cannot do through miniserde.

The entry point is concrete:

```rust
pub fn from_xml<T>(xml: &str) -> miniserde::Result<T>
where
    T: miniserde::Deserialize + DataTypeAware,
{
    // enter recursive descent with T::data_type()
}
```

`unmarshal` (in `core/client.rs`) is the single entry point used by generated
managed-object stubs (e.g. `Folder::create_vm_task` calls
`unmarshal::<ManagedObjectReference>`). Its bound is tightened the same way:

```rust
pub fn unmarshal<T>(transport: Transport, bytes: &[u8]) -> Result<T>
where
    T: miniserde::Deserialize,
    #[cfg(feature = "xml")]
    T: crate::xml::DataTypeAware,
{ ... }
```

`unmarshal_array<U>` and the SOAP helpers (`vim_response*`,
`vim_response_list*`, `from_xml*`) take the same conditional bound. JSON-only
builds keep the looser bound; XML builds get root typing for free at every
call site.

`VimAny` is itself `DataTypeAware` (returning `Any`), so dynamic roots flow
through the same entry point: `from_xml::<VimAny>` enters with
`ApiFieldType::Any`, the parser reads the root `xsi:type`, resolves it
through `lookup_xml_type`, and recursive descent continues with the
resolved concrete `ApiFieldType`.

### Object Fields

When the declared type is `ApiFieldType::Object(declared_st)`, the parser should:

1. Resolve `effective_st`:
   - no `xsi:type`: `effective_st = declared_st`;
   - `xsi:type` present: strip the `xsi:type` value to its local part,
     resolve via `StructType::from_str`, verify
     `effective_st.child_of(declared_st)`, then use it.
2. Open `visitor.map()`.
3. Emit `key("_typeName")` first when `xsi:type` is present, using the
   stripped local-part value (`VirtualE1000`, not `vim:VirtualE1000`).
   This matches the JSON wrapper convention. Polymorphic builders
   (`ObjectPolyBuilder` / `PolyCore`) use the value to pick the concrete
   `FieldsBuilder`; non-polymorphic generated builders ignore
   `_typeName` (their type is statically known). The driver does not
   need to know which kind of builder is on the other side.
4. Emit each non-`xsi:type`, non-`xsi:nil` attribute as `key("@<local_name>")`,
   with the attribute value delivered as `Str`. This is the
   `ManagedObjectReference` and similar compact-XML pathway:
   `<obj type="VirtualMachine">vm-1</obj>` becomes `@type` → `VirtualMachine`.
5. If the element body contains text whose `trim()` is non-empty, emit
   `key("#text")` with the **raw** (un-trimmed) text as `Str`. Pretty
   XML usually has whitespace-only text between an opening tag and the
   first child element (`<foo>\n  <bar/>\n</foo>`); that whitespace is
   formatting, not content, and must not produce a `#text` key. The
   trim-test gate matches the existing `xml/de.rs` behavior and keeps
   compact `ManagedObjectReference` (`<obj>vm-1</obj>`) working without
   accidentally giving every pretty-printed object a bogus `#text`
   field. Once the gate decides to emit, the **raw** text is forwarded
   to the visitor — trimming would corrupt values whose meaningful
   payload includes leading/trailing whitespace.
6. Iterate child elements. For each child, look up the **local element
   name** via `lookup_api_field(effective_st, local_name)` (see
   "Element Names and Namespaces" below).
7. If the field is found, recurse using that field's `ApiFieldType`.
8. If no field is found, return an error. The metadata-driven parser
   should not silently stringify unknown fields.
9. `map.finish()`.

**Mixed content is not allowed.** Once the first child element has been
seen, any further non-whitespace text inside the same parent body is a
parse error. The XSD shapes vSphere produces are either text-only
(compact MOR), child-only (ordinary structs), or attribute-and-text
(MOR), never `text-child-text` interleavings. Treating tail text as an
error keeps the rule crisp and lines up with how vSphere serialises;
the alternative — silently dropping tail text — would mask producer
bugs. Whitespace-only text between children (pretty-print padding) is
ignored.

This `_typeName` / `@attr` / `#text` / child-keys shape is exactly what
the existing generated deserializers expect and what the JSON path
already uses for compact polymorphic values. `ManagedObjectReference`'s
generated builder reads `@type` and `#text` as its two fields.

The parser does not enforce required-vs-optional cardinality. Whether a
field is required is encoded in the generated miniserde builder: when the
builder's `finish()` runs, it checks for absent required fields and
returns an error. The XML driver only delivers the children that are
present on the wire.

For pruned hierarchies such as `MethodFault` and `Event`, the same rule
applies: `xsi:type` resolves the effective subtype `effective_st`, and
`lookup_api_field(effective_st, child_name)` resolves each child's
`ApiFieldType`. `API_FIELD_TABLE` is **complete** — it is a positional
array sized at `STRUCT_TYPE_COUNT` and indexed by `current as usize`, so
every variant of `StructType` (including pruned descendants such as
`EventEx`, `ExtendedEvent`, `VAppPropertyFault`) has an entry. The reason
those entries are not findable by grepping `EventEx` in
`vim_rs/src/types/api_field_registry.rs` is that the file emits entries
positionally without rendering the type name as a comment. There is no
missing data; the registry is the same source of truth as `StructType`
itself.

The parser delivers each pruned-subtype child as a typed JSON-visitor
field. The generated miniserde builder for the pruned base (`MethodFault`,
`Event`) is responsible for routing the typed value into the right place —
its declared base fields when the name matches, and `extra_fields_:
HashMap<String, Value>` (camelCase keyed) when it does not. The XML driver
does not need to know which child belongs in `extra_fields_`; that is a
miniserde-side concern.

The `vim_build` generator should be tightened to emit a `// EventEx` /
`// ExtendedEvent` comment line above each `ApiTypeInfo` entry so the file
is greppable by VIM type name. This is purely a developer-experience fix;
it does not change emitted runtime data.

### Element Names and Namespaces

The driver is **namespace-naive during typed descent**. This is the
simplest policy that is also robust against the multiple prefix forms
vSphere wire payloads use in practice (no prefix, default `xmlns=` on
`Body`, `vim:`, `urn:vim25`/`urn:internalvim25`, the occasional
producer-private prefix). Concretely:

- For child element names (the keys passed to `lookup_api_field`), use
  the **local part only**. Use `BytesStart::local_name()` (or the
  ns-resolver equivalent) and ignore the namespace URI. Today's helper
  `de::start_name` returns the full prefixed tag; the typed driver
  replaces it with a local-name helper at every call site that
  participates in field-name matching: **`stream_children_typed`** /
  **`dispatch_typed_field`**, the SOAP `find_start*` helpers, and the
  `vim_response_list*` `<returnval>` filter.
- For ordinary XML attribute names that map to generated `@name` keys,
  same rule: local part only.
- `xsi:type` is the **only** namespace-aware case in the typed driver.
  It is recognized by the schema-instance URI
  `http://www.w3.org/2001/XMLSchema-instance`, never by the literal
  prefix `xsi:`. Any attribute whose resolved namespace URI matches that
  string is the schema-instance type discriminator, regardless of the
  prefix the wire happened to use.
- `xsi:type` **values** are stripped to their local part before
  registry lookup. `xsi:type="xsd:string"` and `xsi:type="string"` both
  resolve to `lookup_xml_type("string")`. There is **one** canonical
  form in the registry — the local-part name. The `xsd:` form never
  appears as a registry key.

The SOAP envelope, header, and fault elements live in the SOAP
namespace and continue to be handled by `xml/soap.rs` outside the typed
driver.

**On `lookup_api_field` errors as a namespace check.** Because field
lookup is strict (`None` is an error, see Decision #4), a foreign
element whose local name does not collide with a VIM field name will
fail the lookup and the parse fails — which is the desired outcome.
Foreign-namespace local-name collisions with VIM fields are theoretical
on the vSphere wire format; if they ever appear, "non-VIM namespaces are
foreign" can be added as a separate URI gate. Until then, the simple
local-name policy stays.

### Scalars

When the declared type is a primitive `ApiFieldType`, the driver parses
the element text once and calls exactly one visitor method. There is no
fallback probe path. The rules below are decisions, not options:

- **Whitespace handling.** Apply the XSD `collapse` whitespace facet to
  every non-string primitive: trim leading and trailing ASCII
  whitespace before parsing. `Str` (and `Binary`, which travels as
  base64 text) is delivered **as-is** — no trim, no normalization. This
  matches the XSD default `preserve` facet for `xs:string`.
- **`Bool`.** Accept `true`, `false`, `1`, `0` (case-insensitive after
  trim). Anything else is an error. This is the same surface the
  existing `ApiTypedValueVisitor::parse_bool_str` already accepts, so
  XML and JSON behave identically.
- **`I8` / `I16` / `I32` / `I64`.** Parse trimmed text as `i64`,
  range-check against the declared bound, then call
  `negative` / `nonnegative` accordingly. Out-of-range or non-integer
  text is an error.
- **`F32` / `F64`.** Parse trimmed text as `f64`, reject non-finite
  values (`NaN`, `+Inf`, `-Inf`) — XML traffic in vSphere never carries
  these, and accepting them silently hides corruption. For `F32` the
  parsed `f64` is additionally cast and the cast must remain finite.
- **`Str`.** Deliver the raw element text as-is (no trim) via
  `string()`. Empty string is a valid value.
- **`Binary`.** Forward the raw base64 text via `string()` to whatever
  visitor the field visitor exposes. The driver does **not** decode
  base64 itself; the generated `Base64` wrapper's
  `Visitor::string()` already decodes (see
  `vim_rs/src/types/mini_helpers.rs:64`). For dynamic sites under `Any`
  with `xsi:type="base64Binary"`, `lookup_xml_type` returns
  `ApiFieldType::Binary` and the driver still forwards the raw string —
  the consumer (a `Base64`-aware visitor or the typed JSON `Value`)
  decides whether to decode.

**Enums travel as `Str`.** The schema collapses enum-typed fields to
`ApiFieldType::Str`. **`lookup_xml_type` does not use VIM enum type names as keys:**
at an `Any` site, `xsi:type` resolves only to structs, boxed wrappers, or XSD primitives.
Enum-like strings appear as normal field text typed `Str`. The driver does not validate
that the text is a valid enum member;
it just delivers it as a string. Whatever miniserde builder consumes that
string is responsible for enum validation — same as the JSON path. This
is intentional: the spec is about driving the JSON visitor, not enforcing
schema correctness, and keeping the same validation rules for both
transports avoids surprises.

### Arrays

There are two array shapes on the wire. The visitor-call contract is
different for each because the visitor at the array site is different.

#### Field-site arrays

A struct field declared `ApiFieldType::Array(inner)` is a field-site
array. The wire form is one or more adjacent sibling elements with the
field name. The visitor for the field is a sequence-accepting visitor
(no `_typeName` wrapping needed — the type is statically known to be an
array of `inner`). The driver:

1. takes `field_visitor = parent_map.key(local_name)`;
2. opens `field_visitor.seq()`;
3. for each adjacent sibling with that local name, drives one
   `seq.element()` using `inner` (and `inner` may be refined per element
   by `xsi:type`);
4. calls `seq.finish()`;
5. processes the next sibling.

Example — `EventEx.arguments` declared `Array(Object(KeyAnyValue))`:

```xml
<arguments xsi:type="KeyAnyValue">
  <key>a</key>
  <value xsi:type="xsd:string">1</value>
</arguments>
<arguments xsi:type="KeyAnyValue">
  <key>b</key>
  <value xsi:type="xsd:string">2</value>
</arguments>
```

Visitor calls:

```text
parent_map.key("arguments") -> seq()
  seq.element() -> map(); key("_typeName")="KeyAnyValue";
                   key("key")="a"; key("value")=map(...); finish
  seq.element() -> map(); key("_typeName")="KeyAnyValue";
                   key("key")="b"; key("value")=map(...); finish
  seq.finish()
```

Per-element `xsi:type` rules for the `inner` type:

- `inner = Str | Bool | I*/F* | Binary`: optional. If present, must
  resolve to the same primitive via `lookup_xml_type`. Otherwise the
  child text is parsed as that primitive.
- `inner = Object(base)`: optional. If present, resolve via
  `StructType::from_str` and verify `child_of(base)`. This is the
  polymorphic case (e.g. `Array(Object(VirtualDevice))` whose elements
  are concrete subtypes).
- `inner = Any`: mandatory. Each element follows the `Any` resolution
  contract below.

LWW handles non-adjacent runs: if `<arguments>` reappears later under
the same parent (after some other child), the driver simply runs another
`key("arguments") → seq() → … → seq.finish()`, and miniserde overwrites
the field's value with the second run.

#### Wrapper arrays (under `Any`)

A wrapper array appears as a single element with `xsi:type="ArrayOfX"`
whose children are the array items. Wrapper arrays only show up at
declared-`Any` sites, where the visitor is a polymorphic map visitor
(`VimAny` / `PolyCore`) that requires the `_typeName` + `_value`
protocol described in "Dynamic `xsi:type` Values" below. The driver:

1. opens `visitor.map()`;
2. emits `key("_typeName")` with the **stripped local-part** of the
   `xsi:type` value (e.g. `"ArrayOfString"`, never
   `"vim:ArrayOfString"`). This is the same form `PolyCore` uses to
   resolve through `TYPE_REGISTRY`;
3. resolves `lookup_xml_type("ArrayOfString")` → `Array(inner)`;
4. emits `key("_value") -> seq()` and drives each child as one
   `seq.element()` of type `inner` (per-element `xsi:type` rules from
   the field-site list apply). Wire child element names (`<string>`,
   `<HostSystem>`, etc.) are decoration and ignored — only the
   sequence shape matters.
5. calls `seq.finish()`;
6. calls `map.finish()`.

`lookup_xml_type` mappings for common wrappers:

| `xsi:type` value         | `lookup_xml_type` returns         |
|--------------------------|-----------------------------------|
| `ArrayOfString`          | `Array(Str)`                      |
| `ArrayOfInt`             | `Array(I32)`                      |
| `ArrayOfHostSystem`      | `Array(Object(HostSystem))`       |
| `ArrayOfVirtualDevice`   | `Array(Object(VirtualDevice))`    |
| `ArrayOfAnyType`         | `Array(Any)`                      |

Example — `<val xsi:type="ArrayOfString">` reached at an `Any` site:

```xml
<val xsi:type="ArrayOfString">
  <string xsi:type="xsd:string">a</string>
  <string>b</string>
</val>
```

Visitor calls:

```text
visitor.map()
  key("_typeName") <- "ArrayOfString"
  key("_value") -> seq()
    seq.element() <- "a"
    seq.element() <- "b"
    seq.finish()
  map.finish()
```

Example — `<val xsi:type="ArrayOfVirtualDevice">` with mixed concrete
subtypes:

```xml
<val xsi:type="ArrayOfVirtualDevice">
  <VirtualDevice xsi:type="VirtualE1000">
    <key>4000</key>
    <macAddress>00:50:56:00:00:01</macAddress>
  </VirtualDevice>
  <VirtualDevice xsi:type="VirtualLsiLogicController">
    <key>1000</key>
  </VirtualDevice>
</val>
```

Visitor calls:

```text
visitor.map()
  key("_typeName") <- "ArrayOfVirtualDevice"
  key("_value") -> seq()
    seq.element() -> map(); key("_typeName")="VirtualE1000";
                     key("key")=4000; key("macAddress")="00:50:56:..."; finish
    seq.element() -> map(); key("_typeName")="VirtualLsiLogicController";
                     key("key")=1000; finish
    seq.finish()
  map.finish()
```

Empty wrapper arrays (`<val xsi:type="ArrayOfFoo"/>` or
`<val xsi:type="ArrayOfFoo"></val>`) produce `map() → _typeName →
_value:seq()→seq.finish() → map.finish()` with zero elements. This
matches the existing `try_deliver_empty_value` helper in `xml/de.rs`,
which initializes an empty `Vec<T>` inside `ValuePolyBuilder`.

### Dynamic `xsi:type` Values

`VimAny`, `PropertyChange.val`, declared `Any` fields, and other dynamic
sites all share one descent contract. The visitor at the site is a
polymorphic map (`VimAny` / `PolyCore` / `Box<dyn FooTrait>`); it does
not accept raw `string()` / `seq()` / object fields directly. The driver
must therefore always wrap the resolved value in the polymorphic
protocol:

**As implemented** (`stream_drive_any_typed` in `xml/de.rs`): after `lookup_xml_type`,
**`Object(st)`** uses **`drive_object_element`**; **primitive `ApiFieldType`** uses
**`typed_leaf_via_map`** (`_typeName` / `#text` via **`deliver_text_typed`**, aligned with JSON scalar `Any` values);
**`Array(inner)`** (boxed `ArrayOf*`) uses **`stream_drive_any_array_typed`** / **`drive_empty_any_array_typed`**
(`_typeName` + **`_value`** `seq` of **`inner`**). Steps 1–5 below describe the PolyCore
visitor sequence (inverse of JSON wrappers); primitives use `#text` under `_typeName`+map as implemented.

1. `xsi:type` is **mandatory** at the element. Missing `xsi:type` is a
   parse error.
2. Open `visitor.map()`.
3. Emit `key("_typeName")` with the **stripped local-part** type name
   (e.g. `"string"`, `"ArrayOfString"`, `"VirtualE1000"`) — this is what
   `PolyCore` uses to resolve to a builder via `TYPE_REGISTRY`. The
   normalization is the same as today's `xsi_type_to_type_name` and
   matches the canonical registry-key form (see "Single source of
   truth" below).
4. Resolve `lookup_xml_type(stripped_name)` → `ApiFieldType`. Continue
   based on the resolved type:
   - **Primitive** (`Str` / `Bool` / `I*` / `F*` / `Binary`):
     `key("_value")` and deliver the element text via the matching
     visitor method on the value visitor (`string`, `boolean`,
     `nonnegative` / `negative`, `float`).
   - **`Object(st)`**: emit each child element as its own key directly
     on the open map. Child types are resolved via
     `lookup_api_field(st, local_name)`. The map's underlying
     `ObjectPolyBuilder` already routes those keys into the right
     `FieldsBuilder`.
   - **`Array(inner)`**: `key("_value") -> seq()`, each child element
     drives one `seq.element()` of type `inner` (per-element `xsi:type`
     rules from "Arrays" apply).
   - **`Any`** (only reachable when `lookup_xml_type` returns `Any`,
     e.g. as the `inner` of `Array(Any)`): error. `Any` cannot be
     directly resolvable; only concrete shapes are.
5. Call `map.finish()`.

This is the inverse of the existing JSON wrapper format
`{"_typeName": "X", "_value": ...}` for value types and `{"_typeName":
"X", ...fields...}` for object types. The XML driver delivers the same
visitor sequence either way; the JSON path's wrapper encoding is the
reference shape.

For object subtype refinement (`Object(base)` with `xsi:type` naming a
descendant), the same mechanics apply: open the visitor map, emit
`_typeName`, then emit child fields. Whether the underlying builder is
polymorphic (`ObjectPolyBuilder` / `PolyCore`) or a plain
`FieldsBuilder` is invisible to the driver — both accept `_typeName`
(the latter ignores it) and route field keys the same way.

Every shape that dynamic resolution can produce is already an
`ApiFieldType`: a struct entry resolves to `Object(StructType::X)`; a
primitive alias resolves to `Bool` / `I32` / `Str` / ...; **enum symbols are not a
separate `lookup_xml_type` bucket** — enum fields are `Str` at the field site; an
`ArrayOf*` wrapper resolves to `Array(&inner)` (including `Array(&Any)` for
`ArrayOfAnyType`);
`ManagedObjectReference` resolves to
`Object(StructType::ManagedObjectReference)`. There is no need for
parallel `XmlTypeInfo` / `XmlValueType` / `XmlDeclaredType` /
`XmlPrimitiveType` enums.

The XML tables live in generated [`api_field_registry.rs`](../src/types/api_field_registry.rs)
(`feature = "xml"`). They are produced by
[`vim_build/src/rs_emitter/api_registry.rs`](../../vim_build/src/rs_emitter/api_registry.rs)
from the same OpenAPI / `vim_model` sources as `TYPE_REGISTRY` and struct metadata.

**Canonical key form.** `lookup_xml_type` is keyed on **local-part**
type names: `string`, `int`, `long`, `boolean`, `dateTime`,
`base64Binary`, `VirtualE1000`, `ArrayOfString`, `ArrayOfVirtualDevice`,
`ManagedObjectReference`, etc. There is **one** canonical form. The
`xsd:` prefix is wire-side decoration and never appears as a key. The
driver always strips the prefix before lookup. This matches today's
`xsi_type_to_type_name` and the wire spelling that `TYPE_REGISTRY` /
`PolyCore` already use.

**Single source of truth in codegen.** `lookup_xml_type`,
`StructType::from_str`, and `lookup_api_field` must agree on type and
field names because they are consulted in the same descent. For any
type name `N` that names a generated struct, both
`StructType::from_str(N)` and `lookup_xml_type(N)` must succeed, and the
former's `StructType` must match the latter's `Object(StructType::...)`.
This is achieved in the generator by deriving all three tables from the
same `vim_model::Model` walk in a single `vim_build` pass; the
`api_registry` emitter and the XML registry emitter use shared helpers
for type-name normalization, so renames cannot drift between tables.

`TYPE_REGISTRY` is not replaced by this. It still owns dynamic
deserializer construction for miniserde (`make_deserializer`,
`from_value`); the XML driver just doesn't need those callbacks because
it builds shape decisions from `ApiFieldType` instead.

## Metadata Extensions

The current `ApiFieldType` is the right input type for the recursive descent
as-is:

```rust
pub enum ApiFieldType {
    Bool,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Str,
    Binary,
    Object(StructType),
    Array(&'static ApiFieldType),
    Any,
}
```

No new variants are needed:

- Enums stay collapsed to `Str`. That matches the miniserde model and avoids
  adding validation behavior during the parser rewrite.
- `ManagedObjectReference` is not a distinct primitive; it is already
  `StructType::ManagedObjectReference`. The generated MOR deserializer reads
  the compact `<x type="...">moid</x>` form via `@type` + `#text` like any
  other struct.
- `ArrayOf*` wrappers do not need new metadata — they are dynamic and resolve
  through the XML type registry to `ApiFieldType::Array(&inner)`.
- `Any` is a marker. It says "this slot is dynamic; require `xsi:type` and
  resolve through the XML type registry". The parser never tries to
  deserialize `Any` directly.
- Optionality / cardinality are tracked at the field site by miniserde's
  generated builders; the parser does not need them in `ApiTypeInfo`.
- XML attribute names and aliases are not represented yet; add only when
  fixtures prove they are real.

Do not add XML descent contexts such as `Unknown`, `NamedValue`, or a separate
dynamic mode.

Generated tables:

- `lookup_api_field(StructType, field_name) -> Option<ApiFieldType>` — exists
  and is **complete**. The backing `API_FIELD_TABLE` is positionally indexed
  by `StructType as usize` and sized at `STRUCT_TYPE_COUNT`, so every
  `StructType` variant — including pruned descendants of `MethodFault` /
  `Event` such as `EventEx`, `ExtendedEvent`, `VAppPropertyFault` — already
  has an entry. The DX wart is that those entries are not greppable by VIM
  type name; the generator should be tightened to emit a `// <VimTypeName>`
  comment line per entry.
- `StructType::from_str(name: &str) -> Option<StructType>` — exists, used
  to resolve an `xsi:type` value to a concrete struct.
- `StructType::child_of(base: StructType) -> bool` — exists, used to verify
  the resolved subtype is a descendant of (or equal to) the declared base.
- `lookup_xml_type(type_name) -> Option<ApiFieldType>` — emitted in
  [`api_field_registry.rs`](../src/types/api_field_registry.rs) under `feature = "xml"`.
  Keys are the local-part forms of XSD primitives, every generated VIM **struct** name
  (via `StructType` / `resolve_type`), every boxed **`Primitive*` / `ArrayOf*`** wrapper
  (PHF map), **not** a separate table of VIM enum type names for `xsi:type`. Enum-typed
  **fields** use `ApiFieldType::Str` in `lookup_api_field`; the wire sends strings and
  miniserde validates. The driver strips the wire `xsd:` / `xsi:` prefix before lookup
  where applicable; `xsd:` does not appear as a registry key.

Generator sources:

- `vim_build/src/rs_emitter/api_registry.rs` already maps `DataType` to
  `ApiFieldType`. The same pass can emit the XML type registry.
- `vim_build/src/rs_emitter/deser.rs` already iterates `any_value_types` and
  has the `BoxType.property_type` needed to emit array entries.
- `vim_build/src/vim_model/types.rs` and `loader.rs` carry field optionality,
  array shape, struct parentage, discriminator values, and enum identity.

## Parser Algorithm

The new driver has one central recursive function, `drive_element`. Its
input is the XML element, the declared `ApiFieldType` from the schema,
and the visitor for the target value. Both input and output of the
dynamic-type lookup are `ApiFieldType`, so there is no separate
`DeclaredType` enum and no `normalize_declared_type` helper.

`xsi:type` is inspected once at the start of `drive_element` and
**stripped to its local part** before any registry lookup
(`xsd:string` → `string`). The branch on the declared type then chooses
the visitor protocol:

- declared primitive: deliver text directly to the value visitor with
  the matching method (`string`, `boolean`, `nonnegative`, `negative`,
  `float`). Optional `xsi:type` must resolve via `lookup_xml_type` to
  the same primitive.
- declared `Object(base)`: open `visitor.map()`, emit `_typeName` if
  `xsi:type` is present, then iterate children resolving each via
  `lookup_api_field(effective_st, local_name)`. `effective_st` is the
  `xsi:type`-resolved subtype if present (verified `child_of(base)`),
  otherwise `base`.
- declared `Array(inner)`: open `visitor.seq()` directly, iterate the
  adjacent same-name siblings as `seq.element()` of type `inner`,
  `seq.finish()`. There is no `_typeName`/`_value` wrapping at field-site
  arrays.
- declared `Any`: `xsi:type` is mandatory. The driver opens
  `visitor.map()`, emits `_typeName` with the stripped name, resolves
  `lookup_xml_type(stripped_name)` → `ApiFieldType`, then dispatches:
  primitive → `key("_value")` + value-visitor method; object → emit
  child fields directly on the map; array → `key("_value") -> seq()`.
  Finally `map.finish()`.

```rust
fn drive_element(
    reader: &mut NsReader<&[u8]>,
    start: BytesStart,
    is_empty: bool,
    declared: ApiFieldType,
    visitor: &mut dyn Visitor,
) -> Result<()> {
    let attrs = extract_attrs(reader.resolver(), &start)?;

    // xsi:nil short-circuit. Truthy nil consumes the element body and
    // emits null; the visitor decides whether null is acceptable.
    if schema_instance_nil(&attrs)? {
        skip_element_body(reader, start, is_empty)?;
        return visitor.null();
    }

    let xsi = schema_instance_local(&attrs)?; // xsi:type, already stripped

    match declared {
        ApiFieldType::Bool
        | ApiFieldType::I8 | ApiFieldType::I16
        | ApiFieldType::I32 | ApiFieldType::I64
        | ApiFieldType::F32 | ApiFieldType::F64
        | ApiFieldType::Str | ApiFieldType::Binary => {
            if let Some(name) = xsi {
                let resolved = lookup_xml_type(name).ok_or(Error)?;
                if resolved != declared { return Err(Error); }
            }
            drive_primitive(reader, start, is_empty, declared, visitor)
        }

        ApiFieldType::Object(base) => {
            let effective = match xsi {
                None => base,
                Some(name) => {
                    let st = StructType::from_str(name).ok_or(Error)?;
                    if !st.child_of(base) { return Err(Error); }
                    st
                }
            };
            drive_struct(reader, start, is_empty, attrs, xsi, effective, visitor)
        }

        ApiFieldType::Array(inner) => {
            // Field-site arrays. The caller (drive_struct) reaches this only
            // for the first sibling of an array run; the run is consumed by
            // drive_adjacent_array_run. A standalone Array(inner) at a
            // non-field-site is not produced by the schema.
            unreachable!("Array handled at the parent struct site");
        }

        ApiFieldType::Any => {
            let name = xsi.ok_or(Error)?;
            let resolved = lookup_xml_type(name).ok_or(Error)?;
            drive_any(reader, start, is_empty, attrs, name, resolved, visitor)
        }
    }
}
```

`Object(StructType::ManagedObjectReference)` falls into the `Object` arm
unchanged — its compact attribute/text shape is delivered by the same
attribute + `#text` pattern every other struct uses, and the generated
`ManagedObjectReference` deserializer handles the rest.

Struct descent is field lookup and recursion, with field-site arrays
inlined so the per-field `key("name") → seq() → … → finish` cycle stays
contained in the loop body. LWW means the loop simply runs another full
cycle if the same field name reappears later. The `accumulate_text +
first_child` step combines text accumulation with locating the first
child element so the compact `ManagedObjectReference` shape
(text-only body, no children) is handled by the same code path as
ordinary structs:

```rust
fn drive_struct(
    reader: &mut NsReader<&[u8]>,
    start: BytesStart,
    is_empty: bool,
    attrs: Vec<XmlAttr>,
    xsi_local: Option<&str>,
    st: StructType,
    visitor: &mut dyn Visitor,
) -> Result<()> {
    let mut map = visitor.map()?;

    if let Some(name) = xsi_local {
        deliver_string(name, map.key("_typeName")?)?;
    }
    for attr in non_xsi_attrs(&attrs) {
        // attr.local_name is the @-key without prefix
        deliver_string(&attr.value, map.key(&format!("@{}", attr.local_name))?)?;
    }

    // accumulate_text reads up to the first child Start/Empty (or End).
    // A compact MOR-shaped element has text and no children — both branches
    // below handle that correctly: we emit #text and skip the child loop.
    // Whitespace-only text is pretty-print padding and ignored; non-empty
    // text is forwarded raw.
    let (text, mut next) = accumulate_text_until_child(reader, start, is_empty)?;
    if !text.trim().is_empty() {
        deliver_string(&text, map.key("#text")?)?;
    }

    while let Some(child) = next {
        let name = local_name(&child.start)?;
        let field_type = lookup_api_field(st, name).ok_or(Error)?;

        if let ApiFieldType::Array(inner) = field_type {
            next = drive_adjacent_array_run(
                reader, child, name, *inner, map.key(name)?,
            )?;
        } else {
            drive_element(
                reader, child.start, child.is_empty, field_type, map.key(name)?,
            )?;
            next = read_next_child(reader)?;
        }
    }

    map.finish()
}
```

`read_next_child` (and the equivalent step inside
`drive_adjacent_array_run`) skip whitespace text between children and
error on non-whitespace text — see "Mixed content is not allowed" above.

Adjacent-run consumption is unchanged from the prior pseudocode and is
the only place a one-event lookahead is kept (to hand the
different-named sibling back to the parent loop):

```rust
fn drive_adjacent_array_run(
    reader: &mut NsReader<&[u8]>,
    first: Child,
    field_name: &str,
    inner: ApiFieldType,
    visitor: &mut dyn Visitor,
) -> Result<Option<Child>> {
    let mut seq = visitor.seq()?;
    let mut current = Some(first);

    while let Some(child) = current {
        if local_name(&child.start)? != field_name {
            seq.finish()?;
            return Ok(Some(child));
        }
        drive_element(reader, child.start, child.is_empty, inner, seq.element()?)?;
        current = read_next_child(reader)?;
    }

    seq.finish()?;
    Ok(None)
}
```

`Any` descent always wraps the resolved value in the polymorphic
protocol. The branch on `resolved` mirrors the prose contract. The
Object branch must emit non-`xsi:type` attributes and accumulated text
in addition to child fields, otherwise compact-XML payloads such as a
`VimAny` carrying a `ManagedObjectReference`
(`<val xsi:type="ManagedObjectReference" type="VirtualMachine">vm-1</val>`)
would lose `@type` and `#text`:

```rust
fn drive_any(
    reader: &mut NsReader<&[u8]>,
    start: BytesStart,
    is_empty: bool,
    attrs: Vec<XmlAttr>,           // includes the xsi:type attribute
    type_local: &str,              // e.g. "string", "VirtualE1000", "ArrayOfString"
    resolved: ApiFieldType,
    visitor: &mut dyn Visitor,
) -> Result<()> {
    let mut map = visitor.map()?;
    deliver_string(type_local, map.key("_typeName")?)?;

    match resolved {
        ApiFieldType::Bool
        | ApiFieldType::I8 | ApiFieldType::I16
        | ApiFieldType::I32 | ApiFieldType::I64
        | ApiFieldType::F32 | ApiFieldType::F64
        | ApiFieldType::Str | ApiFieldType::Binary => {
            let text = read_text_until_end(reader, start, is_empty)?;
            deliver_primitive(&text, resolved, map.key("_value")?)?;
        }

        ApiFieldType::Object(st) => {
            // Compact-shape support: emit any non-xsi:type attributes (e.g.
            // MOR's @type) and accumulated text (e.g. the moid in #text)
            // before walking child fields. Whitespace-only text is
            // pretty-print padding and dropped; non-empty text is forwarded
            // raw (same gate as drive_struct).
            for attr in non_xsi_attrs(&attrs) {
                deliver_string(&attr.value, map.key(&format!("@{}", attr.local_name))?)?;
            }
            let (text, mut next) = accumulate_text_until_child(reader, start, is_empty)?;
            if !text.trim().is_empty() {
                deliver_string(&text, map.key("#text")?)?;
            }
            while let Some(child) = next {
                let name = local_name(&child.start)?;
                let field_type = lookup_api_field(st, name).ok_or(Error)?;
                if let ApiFieldType::Array(inner) = field_type {
                    next = drive_adjacent_array_run(
                        reader, child, name, *inner, map.key(name)?,
                    )?;
                } else {
                    drive_element(
                        reader, child.start, child.is_empty, field_type, map.key(name)?,
                    )?;
                    next = read_next_child(reader)?;
                }
            }
        }

        ApiFieldType::Array(inner) => {
            let value_visitor = map.key("_value")?;
            let mut seq = value_visitor.seq()?;
            let mut next = first_child(reader, start, is_empty)?;
            while let Some(child) = next {
                drive_element(
                    reader, child.start, child.is_empty, *inner, seq.element()?,
                )?;
                next = read_next_child(reader)?;
            }
            seq.finish()?;
        }

        ApiFieldType::Any => return Err(Error), // lookup must yield concrete
    }

    map.finish()
}
```

The same compact-shape rule applies to `drive_struct`, which already
walks attrs and text in the pseudocode above; the two paths share the
`non_xsi_attrs + accumulate_text_until_child + child loop` structure so
that any `Object(_)` site (declared or `Any`-resolved) can absorb a
text-only body, an attribute-only body, child-element body, or any
combination.

Primitive descent never probes visitor methods in a preferred order; it
parses according to the declared primitive and calls exactly the
corresponding miniserde visitor method. XML primitive aliases
(`xsd:string`, `xsd:int`, etc.) are accepted via the same
`lookup_xml_type` table after stripping the prefix.

Finish maps/sequences at the same stream-safe boundaries used today.
Preserve `DeserializeOptions::tolerate_build_errors` only where the
reader has already consumed the offending element.

### Lookahead

The current parser carries lookahead for three purposes. The typed
parser drops the first one entirely, keeps the second two with their
heuristic role removed:

- **Sequence detection** (legacy probe `seq()` then fall back to single) —
  **removed**. Field-site arrays use **`dispatch_typed_field`** (`ApiFieldType::Array`) only;
  boxed **`Any`** arrays use **`stream_drive_any_array_typed`** (homogeneous sibling elements).
- **Text-then-first-child accumulation** (today's `accumulate_text`,
  which returns "all text up to the first child Start/Empty, plus that
  event") — **kept, but no longer heuristic**. Object descent needs it
  for the compact MOR shape (`<obj type="…">moid</obj>` is text + zero
  children) and for any future XML producer that mixes content inside
  an object body. The typed driver no longer uses the result to decide
  *what* the element is — that is fixed by the declared `ApiFieldType`
  before the call. It just collects `#text` when present and feeds the
  child loop afterwards. Primitive descent uses a simpler "read text
  until End" variant that does not buffer a child event.
- **One-event buffer at array run boundaries** (the `Option<Child>` in
  `drive_adjacent_array_run`) — **kept**. When a run ends because the
  next sibling has a different name, that event has to flow back to the
  parent struct loop. This is forward iteration of a run-terminated
  sequence, not the heuristic kind of lookahead the current parser
  uses.

The old **`deliver_text`** probe order is gone (**FR-014**); primitives use
**`deliver_text_typed`** only.

### SOAP Integration and `stream_drive`

The current `stream_drive` is the shared entry the SOAP layer uses to
push a single XML element into a miniserde visitor. It takes the reader,
the start element, and the visitor:

```rust
pub(crate) fn stream_drive(
    reader: &mut NsReader<&[u8]>,
    start: &BytesStart<'_>,
    visitor: &mut dyn Visitor,
) -> Result<()>;
```

In the typed driver this signature gains an `ApiFieldType` parameter:

```rust
pub(crate) fn stream_drive(
    reader: &mut NsReader<&[u8]>,
    start: &BytesStart<'_>,
    declared: ApiFieldType,
    visitor: &mut dyn Visitor,
) -> Result<()>;
```

Internally `stream_drive` is the thin entry that calls `drive_element`
(or `drive_empty` for self-closing tags) with the declared type. Every
existing caller threads `T::data_type()` (or, for the SOAP envelope
helpers that already know they are decoding a `Vec<T>`, the per-item
`T::data_type()`):

| Call site                                       | Source file        | Declared type passed in            |
|-------------------------------------------------|--------------------|------------------------------------|
| `from_xml::<T>` root drive                      | `xml/de.rs`        | `T::data_type()`                   |
| `vim_response::<T>` `<returnval>` drive         | `xml/soap.rs`      | `T::data_type()`                   |
| `vim_response_list::<T>` per-`<returnval>` loop | `xml/soap.rs`      | `T::data_type()` (per element)     |
| `vim_response_list_tolerant::<T>` per-`<returnval>` loop | `xml/soap.rs` | `T::data_type()` (per element)  |

`vim_response_list` does not pass `Vec<T>::data_type()` because there is
no such impl (see "Root Type"). The list shape is handled by the SOAP
helper iterating siblings; each `<returnval>` is one typed root drive.

The other SOAP helpers (`find_start`, `find_start_returning`,
`find_start_any`, `vim_response_is_empty`, `skip_element`) do not invoke
`stream_drive` and need no signature change. They only walk the SOAP
envelope structure.

PropertyCollector paths that produce `PropertyValue::Parsed(VimAny)`
extract a `<val>` element and push it through the typed driver with
`ApiFieldType::Any`; the parser then resolves `xsi:type` on `<val>`
against `lookup_xml_type` and continues normally.

## Implementation Plan

There is no compatibility shell. The legacy `from_xml<T: Deserialize>`
signature is replaced in place; every XML call site picks up the
`DataTypeAware` bound at the same time. The work breaks into six
landings. Steps 00 and 0 are gating: they must complete and merge
before steps 1–4 begin, because their outputs (fixtures and the
`DataTypeAware` impl surface) constrain the rewrite.

### 00. Wire fixture capture

The current XML driver is heuristic but is, for vCenter and ESXi
production traffic, **working**. Before any rewrite happens, capture a
representative sample of real wire payloads under the existing driver
and lock them in as test fixtures. The new typed driver must reproduce
the same `Value`-equivalent visitor sequence on every captured payload.

The infrastructure already exists. `ClientBuilder::wire_logging` (see
`vim_rs/src/core/client.rs`) takes a `WireLoggingMode::Detailed`
setting that emits full request and response bodies through the `log`
crate at `Trace` level (`Debug` for summaries). Set the env logger
appropriately and run the existing examples. Concretely:

- Run **every** binary in `examples/snippets/` against a live vCenter
  with `WireLoggingMode::Detailed` and `RUST_LOG=trace,vim_rs=trace`.
  Each binary exercises a focused slice of the API
  (event streaming, VM rename, env-browser walk, property collection,
  task tracking, etc.). Save the captured `wire=soap mode=detailed
  phase=response … body=…` lines as fixtures keyed by
  `<binary_name>__<method_name>.xml`.
- Run `examples/vtui/` interactively to sweep large surfaces:
  inventory walk, host configuration, VM device trees, datastore
  browsing, alarm and event histories. These payloads are the largest
  realistic responses the driver will see and will exercise wrapper
  arrays, `Box<dyn FooTrait>` polymorphism, pruned `Event` /
  `MethodFault` subtypes, and `VimAny` carrying `ManagedObjectReference`,
  `ArrayOfManagedObjectReference`, and `ArrayOfAnyType`.
- Capture at least one `vcsim_compat`-only payload (the documented
  `HostConfigInfo.optionDef`-without-`optionType` quirk, or any other
  fixture currently relying on `tolerate_build_errors`) so the new
  driver can prove it preserves that boundary.
- Land the fixtures under `vim_rs/tests/fixtures/xml/` with a small
  manifest mapping each fixture to (a) the originating example
  binary / vtui screen, (b) the request method, (c) the expected
  Rust return type's `data_type()` (used as the `declared` parameter
  in step 2 tests).

The point is that step 2's typed driver lands against a fixed,
reviewable corpus rather than against synthetic XML. The corpus
becomes the regression suite: the new driver must produce
byte-identical `Value` output for every captured response.

### 0. `DataTypeAware` surface spike

Before doing the parser rewrite, add `DataTypeAware` (with a placeholder
implementation that always returns `ApiFieldType::Any`) and tighten the
`unmarshal` / `unmarshal_array` / `vim_response*` / `from_xml*` bounds
to `T: DataTypeAware` under the `xml` feature. Compile every feature
combination (`--no-default-features`, `--features xml`,
`--features xml,vcsim_compat`, `--features defaults`,
`--features xml,defaults`, all examples, and the MCP server crates).

The point of the spike is to flush out **every** type that reaches an
XML root. Beyond the obvious surface (generated structs, generated
enums, `VimAny`, `Box<dyn FooTrait>` boundary types, primitive Rust
types), the bound will catch:

- PropertyCollector extraction paths that materialize `VimAny` /
  `PropertyValue::Parsed`.
- `TaskTracker` result types (`TaskTracker::wait::<T>`,
  `wait_any`, etc.) — both the typed and `VimAny` shapes.
- SOAP list helpers' element types (`Vec<T>` decoded as a sequence of
  `T` roots).
- Generated managed-object methods returning trait objects, optional
  values, and arrays of trait objects.
- Any handwritten helper crate path (the examples crates, MCP server
  data-transformer) that calls into XML deserialization directly.

Each compile error is one missing impl; fixing them in step 0 with
placeholder `Any` returns produces a complete inventory before the real
generation pass runs in step 1. Step 1 then replaces the placeholders
with the schema-derived values.

### 1. Metadata generation

- Replace the placeholder `DataTypeAware` impls from step 0 with the
  real schema-derived ones for generated VIM structs (`Object(Self_)`),
  enums (`Str`), `VimAny` (`Any`), and `Box<dyn FooTrait>` boundary
  types (`Object(base)`). Hand-write the primitive Rust impls.
- Emit `lookup_xml_type(type_name) -> Option<ApiFieldType>` in
  [`vim_rs/src/types/api_field_registry.rs`](../src/types/api_field_registry.rs)
  (same module as `lookup_api_field`). Keys: XSD primitive local names, PHF map of
  boxed `Primitive*` / `ArrayOf*`, and struct names via `StructType::from_str` /
  `resolve_type`. **Do not** emit a standalone enum-name table for `xsi:type`.
  No `xsd:`-prefixed keys.
- Tighten the `api_registry` emitter in `vim_build` to render a `// <VimTypeName>`
  comment line above each `ApiTypeInfo` entry, so `api_field_registry.rs` is
  greppable by VIM type name.
- Add invariant tests that `STRUCT_TYPE_COUNT == API_FIELD_TABLE.len()` and
  that every `StructType` discriminant resolves through `lookup_api_field`,
  including pruned descendants. Add unit tests for `lookup_xml_type` keys
  covering `string`, `ManagedEntityStatus`, `ArrayOfString`,
  `ArrayOfStoragePort`, and `ManagedObjectReference`.

### 2. Typed driver

- **Largely implemented** in `xml/de.rs` (`stream_drive`, `dispatch_typed_field`,
  `stream_drive_any_typed`, **`stream_drive_any_array_typed`**, etc.). Long-form
  pseudocode names in "Parser Algorithm" remain as documentation.
- **Done**: removed `try_seq_or_single`, seq-vs-single probing, and **`deliver_text`**
  (**FR-014**). `typed_leaf_via_map` uses **`deliver_text_typed`** only.
- **Done**: `from_xml` / `from_xml_with` require `T: Deserialize + DataTypeAware`.

### 3. Entry-point wiring

- Thread the root `ApiFieldType` from the SOAP wrappers into the typed
  driver via the new `stream_drive(reader, start, declared, visitor)`
  signature (see "SOAP Integration and `stream_drive`"). SOAP property
  paths preserve dynamic `PropertyValue::Parsed(VimAny)` extraction by
  calling the typed driver with `ApiFieldType::Any`.
- Verify `vcsim_compat` tolerance still drops only complete elements (the
  three documented `finish_map_or_tolerate` boundaries).

### 4. Cleanup

- **Done**: probing helpers removed from **`xml/de.rs`** (**FR-014**, **SC-003**).
- Re-verify the existing `xml/mod.rs` test fixtures pass against the typed
  driver and add the negative cases listed in "Test Matrix".

## Test Matrix

Required positive fixtures:

- Primitive scalar fields at struct sites: bool, signed integer,
  unsigned-compatible integer, float, string, enum string, datetime
  string, binary.
- `ManagedObjectReference` compact XML:
  `<obj type="VirtualMachine">vm-1</obj>`.
- `VimAny` typed leaves under `Any`: `xsi:type="xsd:string"` (stripped
  to `string`) and `xsi:type` naming an enum. Verify the visitor
  sequence is `map() → _typeName=string → _value=text → finish`.
- `VimAny` carrying `ManagedObjectReference`:
  `<val xsi:type="ManagedObjectReference" type="VirtualMachine">vm-1</val>`
  must produce `map() → _typeName=ManagedObjectReference →
  @type=VirtualMachine → #text=vm-1 → finish`. This is the canonical
  test for the compact-XML pathway in `drive_any`'s `Object` branch
  (attrs + text + zero children).
- `VimAny` carrying an arbitrary struct
  (e.g. `<val xsi:type="VirtualE1000"><key>4000</key>…</val>`):
  visitor sequence `map() → _typeName=VirtualE1000 → key=4000 → … →
  finish`. Same branch, but with children instead of text.
- `VimAny` carrying base64 binary:
  `<val xsi:type="xsd:base64Binary">SGVsbG8=</val>` must produce
  `map() → _typeName=base64Binary → _value="SGVsbG8=" → finish`, and
  the resulting `ValueElements` payload must match the existing
  JSON / `ValuePolyBuilder` decode path byte-for-byte. The driver
  forwards the raw base64 string; the underlying `Place<Base64>`
  visitor (or the `ValuePolyBuilder` constructed from
  `make_deserializer()`) decodes. This is the critical regression
  test for the "driver does not decode base64" rule.
- `VimAny` wrapper arrays under `Any`: `ArrayOfString` (primitive
  inner), `ArrayOfHostSystem` (`Object` inner, single subtype), and
  `ArrayOfVirtualDevice` (`Object(base)` inner with mixed concrete
  subtypes). Verify the visitor sequence wraps the seq in `_value`.
- Empty wrapper arrays: `<val xsi:type="ArrayOfFoo"/>` and explicit
  open/close form. Verify the result is an empty array (the existing
  `try_deliver_empty_value` shape).
- Field-site arrays with one and many adjacent siblings:
  `EventEx.arguments`, no outer wrapping.
- LWW ordering: `<arguments/><other/><arguments/>` — the second
  `<arguments>` run wins. miniserde overwrites the field with the
  second value; the first value is discarded.
- Pruned-subtype extras: a `MethodFault` / `Event` payload with
  `xsi:type` naming a pruned descendant (e.g. `EventEx`); verify each
  child field is delivered as a typed visitor key and the pruned
  builder routes both declared-base fields and `extra_fields_` keys
  correctly.
- Namespace prefixes on element names: same payload with no prefix,
  with `vim:` prefix, and with `urn:vim25` default namespace must
  produce identical results.
- Pretty-printed XML whitespace must not produce spurious `#text`:
  `<obj>\n  <key>foo</key>\n</obj>` must emit `key("key")="foo"` and
  no `#text`. The compact-MOR fixture
  `<obj type="VirtualMachine">vm-1</obj>` must emit `#text="vm-1"`.
  Both pathways share the same `text.trim().is_empty()` gate.
- `xsi:nil="true"` (and `="1"`) on optional fields: the field visitor
  receives `null()` and the parent struct's required-field check still
  passes for required siblings. Verify both bare `xsi:nil="true"` and
  `xsi:nil="true"` combined with content-empty bodies. Wrong-namespace
  `nil="true"` (no `xsi:`) must be ignored, treated as a regular
  attribute, and the `lookup_api_field("@nil")` lookup must produce a
  parse error.
- Alternate schema-instance prefixes: `xsi:type`, `p:type`, and an
  inline-`xmlns:foo="…XMLSchema-instance"` `foo:type` must all be
  recognized as the schema-instance type. A `p:type` whose URI is **not**
  the schema-instance namespace must not be treated as `xsi:type`.
- SOAP `returnval` list handling and `vcsim_compat` tolerant item-drop
  behavior.

Negative tests (each must fail with a descriptive error):

- Numeric overflow by declared field type.
- Array-typed field where a non-array shape is provided (e.g. a single
  child where the field expects `seq()` but the visitor rejects it
  because the schema says it isn't an array — this should not happen
  given declared-typed descent, but is a guard rail).
- Unknown field under a strict known object (`lookup_api_field` returns
  `None`).
- `xsi:type` that resolves neither via `lookup_xml_type` nor
  `StructType::from_str` (after local-part stripping).
- `xsi:type` on an `Object(base)` element whose value is not
  `child_of(base)`.
- Declared `Any` element with no `xsi:type` attribute.
- `Array(Any)` element whose item has no `xsi:type` attribute.
- Foreign element inside a typed object whose local name is not a
  registered field (covers the namespace policy via local-name lookup
  failure).
- `xsi:nil="true"` on a non-optional field: miniserde's generated
  builder rejects `null()`, the parse fails. The error must come from
  the visitor / builder (the driver itself does not type-check
  optionality).
- `xsi:nil` value other than `true`/`1`/`false`/`0` (after trim,
  case-insensitive) is a parse error.
- Mixed content inside an object body: `<obj><key>a</key>tail</obj>`
  must error. Whitespace between children is fine; non-whitespace tail
  text is not. Same rule for array bodies.

## Risks

- `DataTypeAware` must be generated for every type that reaches an XML
  entry point. Missing impls are compile errors at `unmarshal` /
  `from_xml` / `vim_response*` call sites — they surface as a wave when
  the bound lands. **Step 0 must be done before the parser rewrite is
  designed any further**: it is the single thing most likely to surface
  unexpected root types (PropertyCollector materialization paths,
  `TaskTracker::wait*`, trait-object returns, examples crates, MCP
  data-transformer). Until the spike compiles green across all feature
  combinations, the surface is hypothetical and the rewrite is
  unconstrained. The spike's placeholder `data_type() -> Any` impl is
  intentionally trivial so that compile errors are about call-site
  bounds, not about the schema-derived values that come in step 1.
- `Any`-as-placeholder is a deliberate over-approximation during step 0.
  At runtime it would force every root through the dynamic `xsi:type`
  resolution path, which would be wrong for non-`VimAny` roots. That is
  fine because step 0 is compile-only — no runtime path is exercised
  until step 2 wires the new driver. By then step 1 has replaced the
  placeholders with real schema-derived `data_type()` values.
- Strict object-field behavior can surface producer quirks the old
  parser silently absorbed. This is desirable long term; the
  implementation lands in step 2, with `vcsim_compat` still available
  for tolerant boundaries.
- The new XML registry adds generated code under the `xml` feature.
  Entries are compact (one `ApiFieldType` per key) and carry no
  callbacks, so the build-time cost is small relative to
  `TYPE_REGISTRY`.
- Diagnostics stay on `miniserde::Error` for now; richer XML errors are
  a follow-up after the metadata path is working.
