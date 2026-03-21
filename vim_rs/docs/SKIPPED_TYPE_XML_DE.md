# Skipped-Type XML Deserialization Design

## Problem Statement

The vim_rs library has an optimization that **prunes** inheritance hierarchies for
`MethodFault` and `Event` types. Instead of generating hundreds of Rust data types
for each subtype, a single base struct is emitted with two special fields:

- `type_: Option<StructType>` — the actual type encountered in the payload
- `extra_fields_: HashMap<String, miniserde::json::Value>` — fields that belong to
  the subtype but not the base type

This works correctly for JSON because `miniserde::json::Value` captures full type
information (numbers, booleans, strings, arrays, objects). However, XML payloads
lack this metadata. The XML deserializer (`xml/de.rs`) feeds text through
`deliver_text()`, which probes Visitor methods in order: `string → nonnegative →
negative → float → boolean → null`. Since `miniserde::json::Value`'s visitor
accepts `string()` for any input, **all XML text becomes `Value::String`**,
producing incorrect types for integer, boolean, and float extra fields.

Additionally, the XML layer cannot distinguish a single-element field from a
one-element array without schema knowledge, because both appear as a single child
element. The `try_seq_or_single` function tries `seq()` first — `Value` accepts
it, so a single object field gets wrapped in an unnecessary array.

### Summary of failures with XML + pruned types today

| Scenario | Expected Value | Actual Value |
|---|---|---|
| `<key>42</key>` (i32 field) | `Value::Number(42)` | `Value::String("42")` |
| `<enabled>true</enabled>` (bool) | `Value::Bool(true)` | `Value::String("true")` |
| `<ratio>3.14</ratio>` (f64) | `Value::Number(3.14)` | `Value::String("3.14")` |
| `<config>...</config>` (single object) | `Value::Object(...)` | `Value::Array([Object(...)])` |

## Solution Overview

Three coordinated changes, **all behind `#[cfg(feature = "xml")]`**:

1. **API Field Registry** — a static array indexed by `StructType` (u32) providing
   field-name → type metadata for every struct type, enabling schema-guided
   deserialization with O(1) type lookup.
2. **Typed Value Visitors** — hand-written Visitor/Seq/Map implementations that
   write into `miniserde::json::Value` but accept only the correct Visitor methods
   based on registry metadata.
3. **Pruned-type deserializer integration** — generated code changes to
   `MethodFaultFields` / `EventFields` (and any future pruned types) that use the
   typed visitors for unknown fields when the XML feature is enabled.

No changes to `xml/de.rs` are required. The XML streaming layer already uses the
miniserde Visitor interface; the typed visitors steer it from the consumer side.

## Detailed Design

### 1. API Field Registry

#### Key Design Decision: `StructType`-indexed Array vs PHF

`StructType` is already a `#[repr(u32)]` enum with ~3900 contiguous variants. We
leverage this to use a **plain array indexed by `StructType as usize`** instead of
a PHF hash map with string keys:

| Aspect | PHF `&str` keys | Array indexed by `StructType` |
|---|---|---|
| Lookup cost | Hash string + 2 table lookups | Single array index |
| Key comparison | Variable-length string compare | None (direct offset) |
| Cache behavior | Scattered memory access | Dense, sequential, CPU-cache-friendly |
| Memory | Keys stored + displacement tables | Values only — no keys, no overhead |
| Practical speed | ~20-50ns per lookup | ~1-2ns per lookup |

The string → `StructType` conversion (`StructType::from_str`) happens **once** via
the existing `STRUCT_TYPE_MAP` PHF when `_typeName` is received. All subsequent
field lookups (multiple per object) use the u32 array index.

#### Location

- **Generated file**: `vim_rs/src/types/api_field_registry.rs`
- **Emitter**: new file `vim_build/src/rs_emitter/api_registry.rs`
- Entire file wrapped in `#![cfg(feature = "xml")]`

#### Data Structures (hand-written, in `vim_rs/src/types/api_field_types.rs`)

```rust
use super::struct_enum::StructType;

/// Primitive and composite field type descriptors for typed deserialization.
/// Used to guide XML (and potentially other) deserializers that lack
/// self-describing type information in the wire format.
///
/// All references are `'static` since they point into generated constant data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)] // `Debug` / `PartialEq` / `Eq` added in-tree for tests and ergonomics
pub enum ApiFieldType {
    Bool,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Str,                                 // String, DateTime, and enum types
    Binary,                              // base64-encoded binary
    Object(StructType),                  // struct type → direct array index
    Array(&'static ApiFieldType),        // element type (static ref to avoid alloc)
    Any,                                 // VimAny — polymorphic, resolved by xsi:type
}

/// Per-type metadata: own fields + optional parent link for chain lookup.
pub struct ApiTypeInfo {
    pub parent: Option<StructType>,
    pub fields: &'static [(&'static str, ApiFieldType)],
}
```

Note: `Object` stores `StructType` (4 bytes) instead of `&'static str` (16 bytes),
and `parent` is `Option<StructType>` (8 bytes) instead of `Option<&'static str>`
(24 bytes). This makes the registry both faster and more compact.

#### Registry Shape

```rust
#[cfg(feature = "xml")]
pub const STRUCT_TYPE_COUNT: usize = /* generated variant count */;

#[cfg(feature = "xml")]
static API_FIELD_TABLE: [ApiTypeInfo; STRUCT_TYPE_COUNT] = [
    /* index 0 = StructType::ManagedObjectReference */
    ApiTypeInfo {
        parent: None,
        fields: &[("type", ApiFieldType::Str), ("value", ApiFieldType::Str)],
    },
    /* index 1 = StructType::DataObject */
    ApiTypeInfo {
        parent: None,
        fields: &[],
    },
    // ... one entry per StructType variant, in enum order ...
];
```

Each entry's position matches its `StructType` discriminant. The `fields` slice
contains only the type's **own** fields (not inherited). Parent-chain traversal
recovers inherited fields.

#### Lookup Function

```rust
/// Look up a field's ApiFieldType by walking the type's inheritance chain.
/// Returns `None` if the field is not defined for the given type (or any ancestor).
pub fn lookup_api_field(st: StructType, field_name: &str) -> Option<ApiFieldType> {
    let mut current = st;
    loop {
        let info = &API_FIELD_TABLE[current as usize];
        // Linear scan of own fields; types typically have ≤15 own fields.
        for &(name, ft) in info.fields {
            if name == field_name {
                return Some(ft);
            }
        }
        current = info.parent?;
    }
}

/// Convert a type name string to StructType for registry lookup.
/// Thin wrapper around the existing STRUCT_TYPE_MAP PHF.
#[inline]
pub fn resolve_type(type_name: &str) -> Option<StructType> {
    StructType::from_str(type_name)
}
```

#### Scope — which types to include

The registry **must** include every struct type in the model (Emit, Prune, and
Skip alike) because:

- The pruned type's extra fields are deserialized as `miniserde::json::Value` trees.
  Nested object fields can reference **any** struct type in the model — not just
  types within the pruned hierarchy.
- The array has one entry per `StructType` variant — there are no gaps and no
  conditional inclusion logic. Every variant maps to exactly one `ApiTypeInfo`.

Only **object (struct) types** are needed. Value types (primitives, arrays) are
encoded inline via `ApiFieldType` enum variants and do not require separate
registry entries.

#### Mapping from `DataType` to `ApiFieldType`

| `DataType` | `ApiFieldType` |
|---|---|
| `Boolean` | `Bool` |
| `String` | `Str` |
| `Int8` | `I8` |
| `Int16` | `I16` |
| `Int32` | `I32` |
| `Int64` | `I64` |
| `Float` | `F32` |
| `Double` | `F64` |
| `DateTime` | `Str` |
| `Binary` | `Binary` |
| `Reference("Any")` | `Any` |
| `Reference(name)` where `name` is an enum | `Str` |
| `Reference(name)` where `name` is a struct | `Object(StructType::VariantName)` |
| `Reference(name)` otherwise (not struct or enum in model) | `Str` |
| `Array(inner)` | `Array(&inner_aft)` |

#### Static references for `Array` inner types

`ApiFieldType::Array` contains `&'static ApiFieldType`. The emitter generates
named `static` items for each distinct inner type:

```rust
static AFT_BOOL: ApiFieldType = ApiFieldType::Bool;
static AFT_STR: ApiFieldType = ApiFieldType::Str;
// Generated code uses `SCREAMING_SNAKE_CASE` via `convert_case::Case::UpperSnake`,
// so numeric primitives are typically `AFT_I_32`, `AFT_F_64`, etc. (not `AFT_i32`).
// ...
static AFT_OBJ_VIRTUAL_DEVICE: ApiFieldType =
    ApiFieldType::Object(StructType::VirtualDevice);
// ...
static AFT_ARR_BOOL: ApiFieldType = ApiFieldType::Array(&AFT_BOOL);
static AFT_ARR_STR: ApiFieldType = ApiFieldType::Array(&AFT_STR);
static AFT_ARR_VIRTUAL_DEVICE: ApiFieldType =
    ApiFieldType::Array(&AFT_OBJ_VIRTUAL_DEVICE);
```

The emitter deduplicates: one `static` per unique `ApiFieldType` tree. Primitive
statics are shared across all types; object statics are per referenced type.

#### Size Estimate

~3900 entries × (8 bytes parent + 16 bytes fields slice) = ~94 KB for the table
itself. Plus field-name slices: ~3900 types × ~5 own fields × 24 bytes = ~470 KB.
Total ~560 KB. No PHF overhead (no displacement tables, no key storage). Only
compiled when `xml` feature is enabled.

---

### 2. Typed Value Visitors

#### Location

- **Hand-written file**: `vim_rs/src/types/api_typed_visitor.rs`
- Gated with `#![cfg(feature = "xml")]`

#### `ApiTypedValueVisitor`

A `miniserde::de::Visitor` implementation that writes into an internal
`Option<miniserde::json::Value>`, guided by an `ApiFieldType`.

```rust
pub struct ApiTypedValueVisitor {
    field_type: ApiFieldType,
    out: Option<miniserde::json::Value>,
}

impl ApiTypedValueVisitor {
    pub fn new() -> Self {
        Self { field_type: ApiFieldType::Str, out: None }
    }

    /// Prepare for the next field. Called before each use.
    pub fn reset(&mut self, ft: ApiFieldType) {
        self.field_type = ft;
        self.out = None;
    }

    /// Extract the produced value after deserialization.
    pub fn take_value(&mut self) -> Option<miniserde::json::Value> {
        self.out.take()
    }
}
```

##### Visitor method routing

| `ApiFieldType` | Accepts `string()` | Accepts `nonnegative()` | Accepts `negative()` | Accepts `float()` | Accepts `boolean()` | Accepts `seq()` | Accepts `map()` |
|---|---|---|---|---|---|---|---|
| `Bool` | yes (parses "true"/"false") | no | no | no | yes | no | no |
| `I8..I64` | yes (parses integer) | yes | yes | no | no | no | no |
| `F32, F64` | yes (parses float) | yes | yes | yes | no | no | no |
| `Str` | yes (verbatim) | no | no | no | no | no | no |
| `Binary` | yes (verbatim base64) | no | no | no | no | no | no |
| `Object(_)` | no | no | no | no | no | no | yes |
| `Array(_)` | no (strict) | no | no | no | no | yes | no |
| `Any` | delegate to Value | delegate | delegate | delegate | delegate | delegate | delegate | delegate |

**Strict arrays (as implemented):** `Array(_)` never accepts `string()`. A
schema-declared array must deserialize through `seq()` (repeated XML child
elements). A future revision *could* optionally accept `string()` for primitive
element types to treat a single text node as a one-element array.

The critical design point: accepting `string()` for numeric/boolean types means
`deliver_text()` in `xml/de.rs` hits the `string()` path **first** and the typed
visitor parses the text according to the field type, producing correctly typed
`miniserde::json::Value`. No changes to `deliver_text()` are needed.

For JSON input, the JSON parser calls the correct visitor method directly
(`nonnegative()` for integers, `boolean()` for bools, etc.), and the typed visitor
accepts those too. Both protocols work through the same visitor.

##### `null()` handling

All field types accept `null()` → stores `Value::Null`. This covers optional
fields with no content in XML (empty/self-closing elements).

#### `ApiTypedSeqBuilder`

Implements `miniserde::de::Seq`. Returned by `ApiTypedValueVisitor::seq()` when
the field type is `Array(inner)`.

```rust
struct ApiTypedSeqBuilder<'a> {
    inner_type: ApiFieldType,
    items: Vec<miniserde::json::Value>,
    element_visitor: ApiTypedValueVisitor,
    out: &'a mut Option<miniserde::json::Value>,
}

impl Seq for ApiTypedSeqBuilder<'_> {
    fn element(&mut self) -> Result<&mut dyn Visitor> {
        if let Some(v) = self.element_visitor.take_value() {
            self.items.push(v);
        }
        self.element_visitor.reset(self.inner_type);
        Ok(&mut self.element_visitor)
    }

    fn finish(&mut self) -> Result<()> {
        if let Some(v) = self.element_visitor.take_value() {
            self.items.push(v);
        }
        *self.out = Some(Value::Array(std::mem::take(&mut self.items)));
        Ok(())
    }
}
```

#### `ApiTypedMapBuilder`

Implements `miniserde::de::Map`. Returned by `ApiTypedValueVisitor::map()` when
the field type is `Object(struct_type)`.

```rust
struct ApiTypedMapBuilder<'a> {
    declared_type: StructType,
    effective_type: Option<StructType>,    // overridden via _typeName / xsi:type
    fields: BTreeMap<String, miniserde::json::Value>,
    current_key: Option<String>,
    field_visitor: ApiTypedValueVisitor,
    out: &'a mut Option<miniserde::json::Value>,
}

impl Map for ApiTypedMapBuilder<'_> {
    fn key(&mut self, key: &str) -> Result<&mut dyn Visitor> {
        self.shift();
        self.current_key = Some(key.to_owned());

        if key == "_typeName" {
            self.field_visitor.reset(ApiFieldType::Str);
            return Ok(&mut self.field_visitor);
        }

        let lookup_type = self.effective_type.unwrap_or(self.declared_type);

        if let Some(ft) = lookup_api_field(lookup_type, key) {
            self.field_visitor.reset(ft);
        } else {
            // Unknown field — best effort: accept as string
            self.field_visitor.reset(ApiFieldType::Str);
        }
        Ok(&mut self.field_visitor)
    }

    fn finish(&mut self) -> Result<()> {
        self.shift();
        *self.out = Some(Value::Object(std::mem::take(&mut self.fields)));
        Ok(())
    }
}

impl ApiTypedMapBuilder<'_> {
    fn shift(&mut self) {
        if let (Some(k), Some(v)) = (self.current_key.take(), self.field_visitor.take_value()) {
            if k == "_typeName" {
                if let Value::String(ref s) = v {
                    self.effective_type = StructType::from_str(s);
                }
            }
            self.fields.insert(k, v);
        }
    }
}
```

This handles polymorphic inner objects: when `xsi:type` is present on a nested
element, the XML layer delivers `_typeName` first (from attributes). The builder
captures it, resolves to `StructType` via `from_str()`, and uses the effective
type for subsequent field lookups — all through O(1) array indexing.

---

### 3. Integration into Pruned Type Deserializers

#### Changes to generated `*Fields` structs

For each pruned type (`MethodFault`, `Event`), the generated Fields struct gains:

```rust
pub struct MethodFaultFields<'a> {
    f0: Option<LocalizedMethodFault>,        // faultCause (illustrative names)
    f1: Option<…>,                           // faultMessage — actual type from model (e.g. `LocalizableMessage` stack)
    type_: Option<struct_enum::StructType>,
    type_name: Option<String>,
    extra_fields_: HashMap<String, miniserde::json::Value>,
    current_extra_key: Option<String>,
    current_extra_value: Option<miniserde::json::Value>,
    #[cfg(feature = "xml")]
    resolved_type: Option<struct_enum::StructType>,
    #[cfg(feature = "xml")]
    api_extra_visitor: super::api_typed_visitor::ApiTypedValueVisitor,
    __out: Option<&'a mut Option<MethodFault>>,
}
```

The `resolved_type` field caches the `StructType` resolved from `type_name`. It is
set once (when `_typeName` is received) and reused for every subsequent field
lookup.

#### Changes to `_typeName` handling in `key()`

When `_typeName` is captured, eagerly resolve to `StructType`:

```rust
"_typeName" => {
    // existing: Ok(miniserde::Deserialize::begin(&mut self.type_name))
    // After type_name is populated, resolve_type is called in shift_extra or
    // lazily on first use. But for eager resolution, we set it in shift_extra.
    Ok(miniserde::Deserialize::begin(&mut self.type_name))
}
```

And in `shift_extra()`, flush the **previous** extra field (if any) and refresh
`resolved_type` from `type_name` when still unset:

```rust
fn shift_extra(&mut self) {
    #[cfg(feature = "xml")]
    {
        if self.resolved_type.is_none() {
            if let Some(tn) = self.type_name.as_deref() {
                self.resolved_type = struct_enum::StructType::from_str(tn);
            }
        }
    }
    #[cfg(feature = "xml")]
    let value = self.current_extra_value.take().or_else(|| self.api_extra_visitor.take_value());
    #[cfg(not(feature = "xml"))]
    let value = self.current_extra_value.take();
    if let (Some(k), Some(v)) = (self.current_extra_key.take(), value) {
        self.extra_fields_.insert(k, v);
    }
}
```

#### Changes to the fallback arm in `key()`

The fallback arm (`_ =>`) uses the cached `StructType` for O(1) field lookup.

**Important:** `shift_extra()` is already called at the **beginning** of `key()` for
pruned types (before the `match`). The `_` arm must **not** call `shift_extra()`
again, or the new `current_extra_key` would be cleared immediately. Earlier
drafts of this document showed `shift_extra()` inside the `_` arm; the shipped
emitter only uses the leading call.

```rust
_ => {
    self.current_extra_key = Some(key.to_owned());

    #[cfg(feature = "xml")]
    {
        let st = self.resolved_type.or(self.type_);
        if let Some(st) = st {
            if let Some(ft) = super::api_field_registry::lookup_api_field(st, key) {
                self.api_extra_visitor.reset(ft);
                return Ok(&mut self.api_extra_visitor);
            }
        }
    }

    Ok(miniserde::Deserialize::begin(&mut self.current_extra_value))
}
```

**Fallback behavior**: If the type is not yet known (`resolved_type` and `type_`
are both `None` — which should not happen in XML because `xsi:type` is delivered
first from attributes), or the field is not found in the registry, the original
untyped `miniserde::json::Value` path is used. This preserves JSON compatibility.

#### Changes to constructors

Both `new()` and `with_output()` initialise the new fields:

```rust
#[cfg(feature = "xml")]
resolved_type: None,
#[cfg(feature = "xml")]
api_extra_visitor: super::api_typed_visitor::ApiTypedValueVisitor::new(),
```

For `new(type_: Option<StructType>)`, also seed `resolved_type`:

```rust
#[cfg(feature = "xml")]
resolved_type: type_,
```

---

### 4. `_typeName` Ordering Constraint

#### Current behavior

`PolyCore` (used by `VimObjectHolder` and `VimAny` deserialization) already
requires `_typeName` to be the first key — or a default type to be provided from
the schema. No buffering of preceding fields occurs.

For pruned types, `_typeName` is one of many match arms in `key()`. Its position
is not enforced. When XML is enabled, the typed visitor path in the `_` arm
naturally requires `resolved_type` to already be set. Since XML always delivers
`xsi:type` as an attribute (before child elements), this is satisfied.

For JSON, `_typeName` is customarily first in vCenter payloads. If it appears
later, the typed visitor path is simply not taken (`resolved_type` is still
`None`), and the untyped `Value` path handles it. **No regression.**

#### Optional strictness (future)

If strict `_typeName`-first enforcement is desired for both JSON and XML when the
xml feature is enabled, the `_` arm can error when the type is unknown:

```rust
#[cfg(feature = "xml")]
if self.resolved_type.is_none() && self.type_.is_none() {
    return Err(miniserde::Error);
}
```

This is NOT recommended for the initial implementation because it would break
JSON payloads where `_typeName` appears after some fields.

---

## vim_build Changes

### New file: `vim_build/src/rs_emitter/api_registry.rs`

A new emitter module that generates `vim_rs/src/types/api_field_registry.rs`.

#### Responsibilities

1. Emit `pub const STRUCT_TYPE_COUNT: usize = N;` from the model's struct count.
2. Iterate all struct types in the model **in `StructType` enum order** (the
   `struct_enum.rs` emitter and model ordering must be consistent).
3. For each type, emit an `ApiTypeInfo` array element:
   - `parent`: the type's parent as `Some(StructType::ParentName)`, or `None` if
     parent is "Any" or absent.
   - `fields`: only the type's OWN fields (not inherited), mapped from `DataType`
     to `ApiFieldType`.
4. Generate deduplicated `static` items for `ApiFieldType` values referenced by
   `Array(...)` entries.
5. Emit the array as `static API_FIELD_TABLE: [ApiTypeInfo; STRUCT_TYPE_COUNT]`.
6. Emit the `lookup_api_field()` and `resolve_type()` functions.
7. Wrap everything in `#[cfg(feature = "xml")]`.

#### DataType → ApiFieldType conversion

```rust
fn data_type_to_api_field_type(dt: &DataType, model: &Model) -> String {
    match dt {
        DataType::Boolean  => "ApiFieldType::Bool".into(),
        DataType::String   => "ApiFieldType::Str".into(),
        DataType::Int8     => "ApiFieldType::I8".into(),
        DataType::Int16    => "ApiFieldType::I16".into(),
        DataType::Int32    => "ApiFieldType::I32".into(),
        DataType::Int64    => "ApiFieldType::I64".into(),
        DataType::Float    => "ApiFieldType::F32".into(),
        DataType::Double   => "ApiFieldType::F64".into(),
        DataType::DateTime => "ApiFieldType::Str".into(),
        DataType::Binary   => "ApiFieldType::Binary".into(),
        DataType::Reference(name) if name == "Any" => "ApiFieldType::Any".into(),
        DataType::Reference(name) if model.enums.contains_key(name) =>
            "ApiFieldType::Str".into(),
        DataType::Reference(name) => format!(
            "ApiFieldType::Object(StructType::{})", to_type_name(name)
        ),
        DataType::Array(inner) => format!(
            "ApiFieldType::Array(&{})", static_name(inner)
        ),
    }
}
```

#### Integration into generator pipeline

In `vim_build/src/rs_emitter/mod.rs` (or `generator.rs`), add a call to the new
emitter after the existing emitters:

```rust
// In generate_bindings():
ApiRegistryEmitter::new(&vim_model, &mut printer).emit()?;
```

### Modified file: `vim_build/src/rs_emitter/structs.rs`

The `emit_deserialize()` method needs changes for pruned types:

1. **Fields struct**: Add `#[cfg(feature = "xml")]` fields for `resolved_type`
   and the typed visitor.
2. **Constructors** (`new()`, `with_output()`): Initialize the new fields.
3. **`shift_extra()`**: Resolve type eagerly; merge value from either
   `current_extra_value` or the API visitor.
4. **`key()` fallback arm**: Before falling through to untyped Value, use
   `resolved_type` for O(1) registry lookup and route to the typed visitor.

All additions are guarded by `#[cfg(feature = "xml")]` in the emitted code. The
emitter should have a helper like:

```rust
fn emit_cfg_xml(&mut self, code: &str) -> Result<()> {
    self.printer.println("#[cfg(feature = \"xml\")]")?;
    self.printer.println(code)?;
    Ok(())
}
```

Or emit blocks:

```rust
self.printer.println("#[cfg(feature = \"xml\")]")?;
self.printer.println("{")?;
self.printer.indent();
// ... xml-specific code ...
self.printer.dedent();
self.printer.println("}")?;
```

### Modified file: `vim_build/src/rs_emitter/deser.rs` (optional)

No changes strictly required. The `ValueElementsFields` slow path (buffering
`_value` as `miniserde::json::Value`) is only used when `_typeName` appears
**after** `_value`. In XML, attributes are always delivered first, so the fast
path (`PolyCore`) is always taken. If desired, the slow path can be disabled
behind `#[cfg(not(feature = "xml"))]` in a future iteration.

---

## vim_rs Changes

### New file: `vim_rs/src/types/api_field_types.rs` (hand-written)

Contains the `ApiFieldType` and `ApiTypeInfo` type definitions shown above.
Gated with `#[cfg(feature = "xml")]`.

### New file: `vim_rs/src/types/api_typed_visitor.rs` (hand-written)

Contains `ApiTypedValueVisitor`, `ApiTypedSeqBuilder`, `ApiTypedMapBuilder`.
Gated with `#[cfg(feature = "xml")]`.

### New file: `vim_rs/src/types/api_field_registry.rs` (generated)

The static array table, `lookup_api_field()`, and `resolve_type()` functions.
Generated by `api_registry.rs` emitter. Gated with `#[cfg(feature = "xml")]`.

### Generated file: `vim_rs/src/types/mod.rs` (via `emit_mod_rs`)

The crate root’s `types/mod.rs` is **emitted** by `vim_build` (`emit_mod_rs` in
[`generator.rs`](../../vim_build/src/generator.rs)), not maintained by hand. It
includes:

```rust
#[cfg(feature = "xml")]
pub mod api_field_types;
#[cfg(feature = "xml")]
pub mod api_field_registry;
#[cfg(feature = "xml")]
pub mod api_typed_visitor;
```

(`api_field_registry` before `api_typed_visitor`, since the visitor imports the registry.)

### Generated file: `vim_rs/src/types/structs.rs`

Pruned type Fields structs and their impl blocks gain `#[cfg(feature = "xml")]`
additions as described in Section 3.

---

## How It All Fits Together — Walkthrough

### XML deserialization of a pruned type

Given this XML payload for a `SecurityError` (subtype of `MethodFault`):

```xml
<fault xsi:type="SecurityError">
    <faultMessage>Access denied</faultMessage>
    <privilegeId>System.Admin</privilegeId>
</fault>
```

(Real vSphere payloads usually use structured `LocalizableMessage` children under
`faultMessage`; this minimal example is for the control flow only.)

1. **XML layer** (`stream_drive`): extracts attributes, finds `xsi:type="SecurityError"`.
2. **`emit_attrs_to_map`**: delivers `_typeName` → `"SecurityError"` to `MethodFaultFields::key("_typeName")`.
3. **`MethodFaultFields::key("_typeName")`**: stores `"SecurityError"` in `self.type_name`.
4. **`MethodFaultFields::key("faultMessage")`**: matched arm, stores in `self.f1`. But first, `shift_extra()` resolves `type_name` → `StructType::SecurityError` and caches in `resolved_type`.
5. **`MethodFaultFields::key("privilegeId")`**: falls to `_` arm.
   - `resolved_type` is `Some(StructType::SecurityError)`.
   - `API_FIELD_TABLE[SecurityError as usize]` → search own fields → not found.
   - Follow `parent` → `StructType::MethodFault` → search → not found either (it's not a MethodFault base field in the registry since `key()` already handles those).
   - Actually, `privilegeId` IS SecurityError's own field → found: `ApiFieldType::Str`.
   - `api_extra_visitor.reset(Str)` → returns `&mut api_extra_visitor`.
6. **XML layer** (`deliver_text`): calls `visitor.string("System.Admin")`.
   - `ApiTypedValueVisitor` with `Str` accepts → stores `Value::String("System.Admin")`.
7. **`MethodFaultFields::finish()`**: calls `shift_extra()` → moves
   `("privilegeId", Value::String("System.Admin"))` into `extra_fields_`.
8. **Build**: produces `MethodFault { type_: Some(StructType::SecurityError), extra_fields_: {"privilegeId": "System.Admin"}, ... }`.

### For a numeric extra field

If SecurityError had `errorCode: i32` and the XML was `<errorCode>403</errorCode>`:

1. `key("errorCode")` → `lookup_api_field(StructType::SecurityError, "errorCode")` → `ApiFieldType::I32`.
2. `api_extra_visitor.reset(I32)`.
3. `deliver_text("403", visitor)` → calls `visitor.string("403")`.
4. `ApiTypedValueVisitor` with `I32`: parses "403" as i64 → `Value::Number(403)`. ✓

### For an array extra field

If the subtype had `affectedEntities: Vec<ManagedObjectReference>`:

```xml
<affectedEntities type="VirtualMachine">vm-1</affectedEntities>
<affectedEntities type="VirtualMachine">vm-2</affectedEntities>
```

1. First `<affectedEntities>`: `key("affectedEntities")` → `lookup_api_field` → `ApiFieldType::Array(&Object(StructType::ManagedObjectReference))`.
2. `api_extra_visitor.reset(Array(...))`.
3. XML layer calls `visitor.seq()` → typed visitor accepts (it's an Array) → returns `ApiTypedSeqBuilder` with inner `Object(StructType::ManagedObjectReference)`.
4. `seq.element()` → returns typed visitor for `Object(...)`.
5. XML layer calls `visitor.map()` → returns `ApiTypedMapBuilder` for `ManagedObjectReference`.
6. Map builder routes `@type` → Str, `#text` → Str (based on MOR's field types, looked up via `API_FIELD_TABLE[ManagedObjectReference as usize]`).
7. Second `<affectedEntities>`: XML layer calls `seq.element()` again.
8. `seq.finish()` → stores `Value::Array([Object, Object])`.

### JSON deserialization (unchanged path)

For JSON like `{"_typeName":"SecurityError","faultMessage":"...","privilegeId":"System.Admin"}`:

1. JSON parser calls `key("_typeName")` → stored, resolved to `StructType`.
2. `key("faultMessage")` → matched arm.
3. `key("privilegeId")` → `_` arm.
   - `resolved_type` is `Some(StructType::SecurityError)`.
   - `lookup_api_field(SecurityError, "privilegeId")` → `ApiFieldType::Str`.
   - `api_extra_visitor.reset(Str)`.
4. JSON parser calls `visitor.string("System.Admin")` → typed visitor accepts. ✓

If `_typeName` were NOT first in JSON (edge case), `resolved_type` would be `None`,
the typed path would not be taken, and the original untyped `Value` path handles it.
**No regression.**

---

## Test Plan

### Target test scenario: VAppPropertyFault (MethodFault hierarchy)

`VAppPropertyFault` extends `VmConfigFault` → `VimFault` → `MethodFault` and
has five own fields: `id` (string), `category` (string), `label` (string),
`type` (string), `value` (string). It has children such as `InvalidPropertyType`
and `NotUserConfigurableProperty`. This makes it an ideal test subject — a
real-world skipped type with string extra fields and faultMessage arrays.

The JSON tests in `serde_test.rs` (`test_vapp_property_fault_with_args`) already
validate this scenario for JSON. The XML tests below are the corresponding
validation for XML deserialization.

### Test cases for `vim_rs/src/xml/de.rs` (or `vim_rs/src/xml/mod.rs` tests)

#### T1: VAppPropertyFault — simple extra fields (string only)

```rust
#[test]
fn test_xml_vapp_property_fault_simple() {
    let xml = r#"<fault xsi:type="VAppPropertyFault">
        <faultMessage>
            <key>config.product.version</key>
            <message>Product Version: 1.0.0</message>
        </faultMessage>
        <id>config.product.version</id>
        <category>string</category>
        <label>Product Version</label>
        <type>string</type>
        <value>1.0.0</value>
    </fault>"#;

    let fault: MethodFault = from_xml(xml).unwrap();
    assert_eq!(fault.type_, Some(StructType::VAppPropertyFault));
    assert!(fault.fault_message.is_some());
    // Extra fields from VAppPropertyFault (not in base MethodFault)
    assert_eq!(
        fault.extra_fields_.get("id"),
        Some(&Value::String("config.product.version".into()))
    );
    assert_eq!(
        fault.extra_fields_.get("label"),
        Some(&Value::String("Product Version".into()))
    );
    assert_eq!(
        fault.extra_fields_.get("value"),
        Some(&Value::String("1.0.0".into()))
    );
}
```

#### T2: VAppPropertyFault — faultMessage with KeyAnyValue args

This is the XML equivalent of `test_vapp_property_fault_with_args`:

```rust
#[test]
fn test_xml_vapp_property_fault_with_args() {
    // faultMessage contains LocalizableMessage with arg array of KeyAnyValue
    let xml = r#"<fault xsi:type="VAppPropertyFault">
        <faultMessage>
            <key>config.product.version</key>
            <arg xsi:type="KeyAnyValue">
                <key>config.product.version</key>
                <value xsi:type="xsd:string">1.0.0</value>
            </arg>
            <message>Product Version: 1.0.0</message>
        </faultMessage>
        <id>config.product.version</id>
        <category>string</category>
        <label>Product Version</label>
        <type>string</type>
        <value>1.0.0</value>
    </fault>"#;

    let fault: MethodFault = from_xml(xml).unwrap();
    assert_eq!(fault.type_, Some(StructType::VAppPropertyFault));
    assert!(fault.fault_message.is_some());
    assert!(fault.extra_fields_.get("label").is_some());
}
```

#### T3: Event with EventEx-style extra fields (string + array + int)

Modeled after the `eventster.rs` example's `get_event_type_id` pattern, where
`EventEx` has extra fields `eventTypeId` (string), `severity` (string),
`message` (string), and `arguments` (array of KeyAnyValue):

```rust
#[test]
fn test_xml_event_ex_extra_fields() {
    let xml = r#"<event xsi:type="EventEx">
        <key>12345</key>
        <chainId>100</chainId>
        <createdTime>2024-06-15T10:30:00Z</createdTime>
        <userName>admin</userName>
        <eventTypeId>com.vmware.example.event</eventTypeId>
        <severity>info</severity>
        <message>Something happened</message>
    </event>"#;

    let event: Event = from_xml(xml).unwrap();
    assert_eq!(event.type_, Some(StructType::EventEx));
    assert_eq!(event.key, 12345);
    assert_eq!(event.user_name, "admin");

    // Extra fields from EventEx (not in base Event)
    assert_eq!(
        event.extra_fields_.get("eventTypeId"),
        Some(&Value::String("com.vmware.example.event".into()))
    );
    assert_eq!(
        event.extra_fields_.get("severity"),
        Some(&Value::String("info".into()))
    );
}
```

#### T4: Numeric extra field — validates int is not stored as string

Uses a fault subtype that has an integer (and ideally a bool) field. This is the
core bug the design fixes.

**Draft sketch (MOR on `MigrationFault`):** `MigrationFault` in the schema is not
the type that owns a top-level `source` reference; that pattern fits **child**
fault types instead. The snippet below is illustrative of “object not wrapped in
`Value::Array`” more than of `MigrationFault` literally.

```rust
// Illustrative only — pick a fault type that actually declares `source` in the model.
let xml = r#"<fault xsi:type="???">
        <source type="VirtualMachine">vm-1</source>
    </fault>"#;
// … assert Value::Object, not Value::Array …
```

**As implemented in-tree:** `test_xml_fault_readonly_disks_numeric_extra` uses
`ReadOnlyDisksWithLegacyDestination` with `roDiskCount` (`i32`) and
`timeoutDanger` (`bool`), `xmlns:xsi` on the root element, and pattern-matching
on `Value` (no `PartialEq`).

```rust
#[test]
fn test_xml_fault_numeric_extra_field() {
    let xml = r#"<fault xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ReadOnlyDisksWithLegacyDestination">
        <roDiskCount>2</roDiskCount>
        <timeoutDanger>false</timeoutDanger>
    </fault>"#;

    let fault: MethodFault = from_xml(xml).unwrap();
    assert_eq!(
        fault.type_,
        Some(StructType::ReadOnlyDisksWithLegacyDestination)
    );
    match fault.extra_fields_.get("roDiskCount") {
        Some(Value::Number(Number::I64(n))) => assert_eq!(*n, 2),
        o => panic!("roDiskCount: {:?}", o),
    }
    match fault.extra_fields_.get("timeoutDanger") {
        Some(Value::Bool(b)) => assert!(!b),
        o => panic!("timeoutDanger: {:?}", o),
    }
}
```

#### T5: Array extra field — validates repeated elements become an array

```rust
#[test]
fn test_xml_event_arguments_array() {
    let xml = r#"<event xsi:type="EventEx">
        <key>1</key>
        <chainId>1</chainId>
        <createdTime>2024-01-01T00:00:00Z</createdTime>
        <userName>system</userName>
        <eventTypeId>test.event</eventTypeId>
        <arguments xsi:type="KeyAnyValue">
            <key>arg1</key>
            <value xsi:type="xsd:string">val1</value>
        </arguments>
        <arguments xsi:type="KeyAnyValue">
            <key>arg2</key>
            <value xsi:type="xsd:string">val2</value>
        </arguments>
    </event>"#;

    let event: Event = from_xml(xml).unwrap();
    let args = event.extra_fields_.get("arguments");
    assert!(args.is_some());
    if let Some(Value::Array(arr)) = args {
        assert_eq!(arr.len(), 2);
    } else {
        panic!("arguments should be Value::Array, got {:?}", args);
    }
}
```

#### T6: Single object field — validates it is NOT wrapped in an array

```rust
#[test]
fn test_xml_single_object_not_array() {
    // EventEx with a single 'arguments' element — must be Array([one_item]),
    // NOT a scalar, because the schema declares it as an array field.
    let xml = r#"<event xsi:type="EventEx">
        <key>1</key>
        <chainId>1</chainId>
        <createdTime>2024-01-01T00:00:00Z</createdTime>
        <userName>system</userName>
        <eventTypeId>test.event</eventTypeId>
        <arguments xsi:type="KeyAnyValue">
            <key>only_arg</key>
            <value xsi:type="xsd:string">only_val</value>
        </arguments>
    </event>"#;

    let event: Event = from_xml(xml).unwrap();
    let args = event.extra_fields_.get("arguments");
    // Even though there's only one <arguments> element, the schema says it's
    // an array, so the typed visitor accepts seq() and produces a 1-element array.
    if let Some(Value::Array(arr)) = args {
        assert_eq!(arr.len(), 1);
    } else {
        panic!("single arguments element should still be Array, got {:?}", args);
    }
}
```

### JSON regression tests

Run the existing tests without the `xml` feature to confirm no regression:

```bash
cargo test                         # default features (no xml)
cargo test --features xml          # with xml
```

The `serde_test.rs` tests (`test_vapp_property_fault_simple`,
`test_vapp_property_fault_with_args`, etc.) must continue to pass in both modes.

**Where the XML tests live:** T1–T6 are implemented in the `#[cfg(all(test, feature = "xml"))]`
`tests` module in [`src/xml/mod.rs`](../src/xml/mod.rs) (not in `xml/de.rs`).

**Assertions and `Value`:** `miniserde::json::Value` does not implement `PartialEq`, so
tests use `match` / pattern matching on `extra_fields_.get(...)` rather than
`assert_eq!(..., Some(&Value::...))`.

**T4 (numeric / bool extras) as shipped:** The design draft used `MigrationFault` with a
`source` `ManagedObjectReference`; in the VMware schema, `MigrationFault` itself does not
own that shape, so the checked-in test uses `ReadOnlyDisksWithLegacyDestination`
(`roDiskCount: i32`, `timeoutDanger: bool`) under `xsi:type="ReadOnlyDisksWithLegacyDestination"`.

**T5 / T6 (`EventEx`):** The in-tree tests include a `message` child where needed so the
payload matches typical `EventEx` usage (optional in schema but keeps deserialization
unambiguous alongside other extras).

---

## Implementation notes (repository)

These details reflect how the design is **wired in this codebase**; they supplement
the normative sections above.

| Topic | Detail |
|--------|--------|
| **`types/mod.rs`** | **Generated** by [`vim_build/src/generator.rs`](../../vim_build/src/generator.rs) (`emit_mod_rs`). Edit `emit_mod_rs`, not the checked-in file alone, or the next `vim_build` run will overwrite manual changes. |
| **Module order** | After `struct_enum`: `api_field_types`, `api_field_registry`, `api_typed_visitor` (visitor depends on registry). |
| **Registry `static` names** | Each deduplicated `ApiFieldType` tree gets `static AFT_<NAME>` where `<NAME>` is `UpperSnake` from a normalized signature (e.g. `i32` → `AFT_I_32`, `obj:Foo` → `AFT_OBJ_FOO`, nested arrays → `AFT_ARR_…`). Satisfies Rust’s `non_upper_case_globals` lint. |
| **Topological order** | `static` items are emitted in ascending “array nesting depth” so `Array(&INNER)` references only earlier `static`s. |
| **`shift_extra` / `key()`** | For pruned maps, `shift_extra()` runs at the **start** of `key()`; the `_` fallback arm does **not** call `shift_extra()` again (see Section 3). |
| **Strict `Array`** | No `string()` on `Array(_)`; see visitor routing table above. |
| **Enum / index parity** | Registry rows are emitted in the same `vim_model.structs` iteration order as the [`struct_enum` emitter](../../vim_build/src/rs_emitter/struct_enum.rs) (skip only `Any`). No extra compile-time assertion is checked in yet; a small test over known `(StructType, field)` pairs is a possible follow-up. |
| **`changeset_xml_test`** | Not wired as a Cargo integration test in all checkouts; run whatever XML/fixture tests exist in your workspace when validating SOAP paths. |

The schematic `MethodFaultFields` snippet in Section 3 uses illustrative `f0`/`f1` types;
the generated field indices and Rust types follow the live OpenAPI model (e.g.
`fault_message` deserializes via the real `LocalizableMessage` / holder types).

---

## File Inventory

| File | Type | Feature Gate | Description |
|---|---|---|---|
| `vim_build/src/rs_emitter/api_registry.rs` | New (emitter) | N/A (build tool) | Generates the API field registry array |
| `vim_build/src/rs_emitter/mod.rs` | Modified | N/A | Register new emitter module |
| `vim_build/src/rs_emitter/structs.rs` | Modified | N/A | Emit `#[cfg(xml)]` additions to pruned types |
| `vim_rs/src/types/api_field_types.rs` | New (hand-written) | `#[cfg(feature = "xml")]` | `ApiFieldType`, `ApiTypeInfo` definitions |
| `vim_rs/src/types/api_typed_visitor.rs` | New (hand-written) | `#[cfg(feature = "xml")]` | Visitor/Seq/Map for typed Value construction |
| `vim_rs/src/types/api_field_registry.rs` | New (generated) | `#[cfg(feature = "xml")]` | Static array table + `lookup_api_field()` |
| `vim_rs/src/types/mod.rs` | Generated | — | Module list from `vim_build` `emit_mod_rs` (includes xml-gated `api_*` mods) |
| `vim_rs/src/types/structs.rs` | Modified (generated) | Additions gated | Pruned type Fields changes |

---

## Implementation Order

1. **Define types** (`api_field_types.rs`) — `ApiFieldType` and `ApiTypeInfo`.
2. **Write the emitter** (`api_registry.rs`) — generates the registry array.
3. **Run code generation** — verify the registry compiles.
4. **Implement typed visitors** (`api_typed_visitor.rs`) — `ApiTypedValueVisitor`,
   `ApiTypedSeqBuilder`, `ApiTypedMapBuilder`.
5. **Unit test visitors** — test each visitor with mock field types.
6. **Modify structs emitter** — add `#[cfg(xml)]` paths to pruned type
   deserializers.
7. **Re-generate** — verify compilation with both `--features xml` and without.
8. **Integration test** — run existing `changeset_xml_test` fixtures, add new
   tests (T1–T6 above) with mixed-type extra fields.
9. **Verify JSON** — run existing `serde_test` and integration tests without xml
   feature to confirm no regression.

---

## Risks and Mitigations

| Risk | Mitigation |
|---|---|
| Registry size (~560KB) bloats binary | Only compiled when `xml` feature enabled; array is denser than PHF |
| Registry stale after spec update | Generated by code generator — always in sync with `StructType` enum |
| Unknown fields in nested objects | Fall back to `ApiFieldType::Str` — lossy but safe |
| Polymorphic nested objects | `ApiTypedMapBuilder` resolves `_typeName` → `StructType` via `from_str()` and switches lookup type |
| JSON regression from typed visitor | Fallback to untyped Value when `resolved_type` is unknown; typed visitor accepts all JSON-native method calls |
| Compile-time increase | Array literal is faster to compile than PHF codegen |
| Enum ordering mismatch | Emitter iterates model structs in same order as `struct_enum.rs` emitter; add assertion in tests |
