# Type system

The VIM API is a classic object-oriented model. `Any` is the root. `DataObject` is the root of data structures. `MethodFault` is the root of errors. `Event` is the root of events. Rust has no inheritance, so `vim_rs` uses two representations side by side.

Deep hierarchies are traits plus compositional inheritance. Wide, shallow sums are enums. That split is the whole design: traits stay usable where a VIM value can be any of dozens of descendants, and enums stay cheap where the alternatives are a flat list of primitives and arrays.

`Any` is renamed to `VimAny`.

Day-to-day casting, property reads, and event decoding are in the [crate guide](../README.md#working-with-polymorphic-values). This document is the model those snippets rely on.

## Data structs

Every VIM structure type has a Rust struct. Child types **contain** the parent as a field and implement `Deref` / `DerefMut`. Callers still read inherited fields as if they were local.

A network card looks like this:

```rust
pub struct VirtualE1000 {
    // Parent field — VirtualEthernetCard fields, which themselves contain VirtualDevice
    pub virtual_ethernet_card_: VirtualEthernetCard,
}

impl Deref for VirtualE1000 {
    type Target = VirtualEthernetCard;
    fn deref(&self) -> &Self::Target { &self.virtual_ethernet_card_ }
}

impl DerefMut for VirtualE1000 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.virtual_ethernet_card_ }
}
```

Deref coercion walks the chain:

```rust
let e1000: &VirtualE1000 = /* ... */;
let key: i32 = e1000.key;                      // VirtualDevice (grandparent)
let mac: &Option<String> = &e1000.mac_address; // VirtualEthernetCard (parent)
```

The parent is the same shape one level up:

```rust
pub struct VirtualEthernetCard {
    pub virtual_device_: VirtualDevice,
    pub address_type: Option<String>,
    pub mac_address: Option<String>,
    pub wake_on_lan_enabled: Option<bool>,
    // ...
}
```

Field mapping from the OpenAPI model:

- Optional VIM fields are `Option<T>`. Required fields are `T`.
- Arrays are `Vec<T>`.
- Recursive fields and fields that would cycle are `Box<T>`.
- Polymorphic fields are `Box<dyn SomeTrait>` (often wrapped in `Option`).

Structs implement `miniserde` JSON serialization and `Debug`.

This compositional model replaced an earlier expansion that copied every ancestor field onto the child. Field access is the same, the generated source is smaller, and the structs support mutation when an update spec has to be built.

## Traits

A trait is generated for every VIM struct that has children. Every descendant implements that trait, so a field can accept or return the whole family. Upcasting to a trait the concrete type implements is ordinary Rust.

Rust has no built-in way to upcast or downcast `dyn Trait` to another trait or to a concrete struct. `vim_rs` adds two utilities.

**Concrete downcast.** Every VIM trait extends `AsAny`, which produces `&dyn Any`. From there, `downcast_ref` is fallible:

```rust
let e1000 = vd[0].as_any_ref().downcast_ref::<VirtualE1000>().unwrap();
```

**Trait-to-trait cast.** `CastInto` (and its mirror `CastFrom`) converts between trait objects. Reading the MAC of any NIC means turning `VirtualDeviceTrait` into `VirtualEthernetCardTrait`:

```rust
let eth: &dyn VirtualEthernetCardTrait = vd.as_ref().into_ref().unwrap();

let eth: Box<dyn VirtualEthernetCardTrait> = vd.into_box().unwrap();
```

Import `vim_rs::types::convert::CastInto` at the call site.

Child structs still reach parent fields through `Deref`, including across a trait object once you have a concrete reference or have deref'd to the parent struct. The trait objects themselves are the polymorphic holes in the tree; the structs behind them are the compositional chain.

## Enums

Two places stay enums because a trait-per-variant would be all cost and no hierarchy:

- **`VimAny`** — the VIM `Any` sum: a primitive `Value`, a typed object, or another dynamic form the property collector and similar APIs return.
- **`ValueElements`** — the flat inventory of boxed arrays and primitives used when a field is dynamically typed. `ValueElements::as_str()` returns the VIM type name (`"string"`, `"ArrayOfManagedObjectReference"`) for logging and discrimination.

An enum of enums, nested as deep as the VIM inheritance tree, is unusable at the call site. The hybrid model stops at the depth where that happens.

## Pruned types

`MethodFault` and `Event` descendants are not generated. The list is `PRUNED_TYPES` in [`vim_build/src/main.rs`](../../vim_build/src/main.rs). Skipping those trees cuts a large amount of generated code and compile time. The cost is that subtype-specific fields are not first-class Rust fields.

Both base structs carry:

- `type_: Option<StructType>` — the discriminator (`EventEx`, `NotFound`, …).
- `extra_fields_: HashMap<String, miniserde::json::Value>` — payload fields that belong to the subtype. Keys are the API’s camelCase names, not the snake_case names used everywhere else in `vim_rs`.

There is no trait for either hierarchy.

`StructType::child_of` tests whether a discriminator is a type or one of its descendants. A semantic event id looks like this:

```rust
fn get_event_type_id(event: &Event) -> String {
    let Some(type_) = event.type_ else {
        return "Event".to_string();
    };
    if type_.child_of(StructType::EventEx) || type_.child_of(StructType::ExtendedEvent) {
        if let Some(event_type_id) = event.extra_fields_["eventTypeId"].as_str() {
            return event_type_id.to_string();
        }
    }
    type_.as_str().to_string()
}
```

A dynamic field can be turned back into a generated binding with `vim_rs::types::mini_helpers::from_value`:

```rust
use vim_rs::types::mini_helpers::from_value;

let value = event.extra_fields_["managedObject"].clone();
let managed_object: ManagedObjectReference = from_value(value)?;
```

JSON keeps numbers, booleans, and arrays distinct inside `miniserde::json::Value`. XML text does not. How the SOAP deserializer reconstructs pruned fields is [SKIPPED_TYPE_XML_DE.md](SKIPPED_TYPE_XML_DE.md). The deserializer that drives miniserde from XML in general is [METADATA_DRIVEN_XML_DE.md](METADATA_DRIVEN_XML_DE.md).

## `defaults`

`Default` for every struct, enum, and trait object is behind the `defaults` feature. It exists so update specs can be built with struct-update syntax (`..Default::default()`). Generating it for the whole model adds a large amount of source and compile time, so it stays opt-in. Enum defaults use the first variant. `Box<dyn Trait>` defaults construct the base struct.

## Standard traits that are omitted

`PartialEq`, `Eq`, `Hash`, and `Clone` are not implemented on VIM structs. Deriving them across this model dominates compile time and binary size. The generated impls are the ones the client needs to speak the protocol: `Debug`, miniserde, and (opt-in) `Default`.

## Compile time

A clean build of the integration tests is typically a few minutes on a fast machine. Incremental builds inside an editor are often around a minute. Most of that time is `vim_rs/src/types/`, and `structs.rs` in particular. The hybrid model, pruned fault and event trees, and opt-in `Default` are the levers used to keep that in bounds.
