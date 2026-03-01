# MCP Tool Gap Analysis Report

This document captures the gap analysis performed to identify where MCP tool content has become stale after vim_rs's compositional inheritance, trait getter removal, and enum changes. It serves as motivation and context for implementers — if an unexpected complication requires a change of direction, this document explains the "why" behind each fix.

## GAP 1 (CRITICAL): Trait Getter Methods Are Stale

**What the MCP says:** When you call `get(id="VirtualDeviceTrait")`, it shows "Getter Methods" like `get_key()`, `get_backing()`, `get_connectable()`, etc. Similarly `get(id="VirtualEthernetCardTrait")` shows `get_mac_address()`, `get_address_type()`, etc.

**What the code actually has:** Traits now only have two methods — a Deref accessor pair:
- `get_<parent_struct>(&self) -> &StructType`
- `get_<parent_struct>_mut(&mut self) -> &mut StructType`

Fields are accessed directly via Deref coercion (e.g., `device.key`, `eth.mac_address`), not via getter methods.

**Root cause:** `mcp/data_processing/build_api_definitions/src/api_builder/traits.rs` lines 28-42 iterate struct fields and create `GetterEntry` items for each, but these per-field getters were removed from trait code generation.

**Impact:** LLMs generate `device.get_key()`, `eth.get_mac_address()` which won't compile.

---

## GAP 2 (CRITICAL): Trait Usage Examples Use Wrong Access Pattern

**What the MCP generates** (in `format_trait_doc()` at `api_database/src/lib.rs:1232-1234`):

```rust
let value = specialized.get_dynamic_property();  // WRONG
```

**Correct pattern:**

```rust
let value = &specialized.mac_address;  // Direct field access via Deref
```

**Root cause:** The `format_trait_doc()` function generates the usage example using `getter.name` from the (stale) getter entries.

---

## GAP 3 (CRITICAL): Structs Missing Parent Field for Construction

**What the MCP shows for `TraversalSpec`:** 4 fields only (`r#type`, `path`, `skip`, `select_set`)

**What the actual struct looks like:**

```rust
pub struct TraversalSpec {
    // Parent field
    pub selection_spec_: super::structs::SelectionSpec,
    // Fields of TraversalSpec
    pub r#type: String,
    pub path: String,
    pub skip: Option<bool>,
    pub select_set: Option<Vec<Box<dyn super::traits::SelectionSpecTrait>>>,
}
```

The parent field `selection_spec_: SelectionSpec` is **required** for construction but invisible to the LLM. Same applies to `VirtualEthernetCard` (`virtual_device_: VirtualDevice`), `VirtualE1000` (needs both `virtual_ethernet_card_` and transitive parent), and many others.

**Root cause:** `data_structures.rs` line 97 iterates only `s.fields` (the VIM model fields), but the parent field is added during Rust code generation and doesn't exist in the model.

**Impact:** LLMs cannot construct structs with parent types. The working `property_collector.rs` example shows the correct pattern but the MCP struct docs omit it entirely.

---

## GAP 4 (MODERATE): Enum `Other_` Variant Not Documented

**What the MCP shows for enums:** Only known variants (e.g., `Gray`, `Green`, `Yellow`, `Red` for `ManagedEntityStatusEnum`)

**What the actual code has:**

```rust
pub enum ManagedEntityStatusEnum {
    Gray,
    Green,
    Yellow,
    Red,
    /// This variant handles values not known at compile time.
    Other_(String),
}
```

**Root cause:** `enumerations.rs` builder only iterates `enum_def.variants` and doesn't add the `Other_(String)` catch-all.

**Impact:** LLMs will write exhaustive match blocks without a catch-all and get compiler errors. Also, `as_str()` returning `&str` (instance lifetime, not `&'static str`) for `Other_` variants isn't documented per-enum, and `from_str()` availability isn't documented.

---

## GAP 5 (MODERATE): Starter Guide Omits Construction-Side Deref Pattern

**What the guide covers (reading side — adequate):**
- "Access fields directly via Deref" — repeated many times with examples
- `device.key`, `eth.mac_address` — shows the end result
- "Structs use compositional inheritance with Deref" — one sentence

**What the guide completely omits (construction side — critical gap):**
- The actual mechanical pattern: child structs contain a `parent_name_: ParentType` field (underscore suffix convention)
- How to construct these structs in struct literals — you must initialize the parent field
- The multi-level Deref chain: `VirtualE1000` Derefs to `VirtualEthernetCard` which Derefs to `VirtualDevice`
- That `dyn VirtualDeviceTrait` Derefs to `VirtualDevice` struct, making field access work on trait objects too
- Zero construction examples for child structs anywhere in the guide

This is a novel Rust pattern — even experienced Rust developers consider Deref-as-inheritance unusual. LLMs will have almost no training data with this convention.

---

## GAP 6 (LOW): Managed Objects Don't Distinguish Property Accessors from Action Methods

**What the MCP shows:** `VirtualMachine` lists `name`, `config`, `runtime` alongside `power_on_vm_task` with no distinction.

**Reality:** Everything on managed objects is a remote method. Property accessors (like `name()`, `config()`) fetch a single property via SOAP call — useful when you need just one value quickly. Action methods (like `power_on_vm_task()`) initiate operations and return a Task reference.

**Important nuance:** Fetching multiple properties sequentially can be a recipe for disaster — the remote object is live and can change between calls, providing potentially internally incoherent data. For multiple properties or many objects, PropertyCollector and the vim_rs macros are the correct approach.

**Impact:** Quality-of-life improvement. The starter guide already explains the Task vs property distinction, but the per-MO listing doesn't annotate it.

---

## Key Evidence from Source Code

- **Actual `VirtualDeviceTrait`** (vim_rs/src/types/traits.rs:35582-35587): only has `get_virtual_device()` and `get_virtual_device_mut()`
- **Actual `VirtualEthernetCard`** (vim_rs/src/types/structs.rs:373576-373578): has `virtual_device_: VirtualDevice` parent field
- **Actual `TraversalSpec`** (vim_rs/src/types/structs.rs:470501-470503): has `selection_spec_: SelectionSpec` parent field
- **Actual `ManagedEntityStatusEnum`** (vim_rs/src/types/enums.rs:2902-2909): has `Other_(String)` variant
- **Actual `as_str()` impl** (vim_rs/src/types/enums.rs:23869-23877): returns `&str`, matches `Other_(s) => s` for runtime value
