# Migration Guide: vim_rs 0.3.0 → 0.4.0

This release contains several **breaking changes**. Work through each section that applies to your codebase. Most changes can be handled with straightforward search-and-replace.

---

## Table of Contents

1. [Compositional inheritance — struct field layout changed](#1-compositional-inheritance--struct-field-layout-changed)
2. [Trait getter methods removed](#2-trait-getter-methods-removed)
3. [Enum API changes](#3-enum-api-changes)
4. [DataObject trait methods removed](#4-dataobject-trait-methods-removed)
5. [miniserde replaces serde/serde_json](#5-miniserde-replaces-serdeserde_json)
6. [Cargo.toml dependency updates](#6-cargotoml-dependency-updates)
7. [Quick-reference cheat sheet](#7-quick-reference-cheat-sheet)

---

## 1. Compositional inheritance — struct field layout changed

In 0.3.0, all fields from ancestor structs were inlined into each child struct. In 0.4.0, each struct embeds only its **direct parent** as a named field, and implements `Deref`/`DerefMut` to it.

### What changed

```rust
// 0.3.0 — VirtualE1000 had all ancestor fields inlined
pub struct VirtualE1000 {
    pub key: i32,                          // from VirtualDevice
    pub device_info: Option<Description>,  // from VirtualDevice
    pub mac_address: Option<String>,       // from VirtualEthernetCard
    // ... many more ancestor fields
}

// 0.4.0 — VirtualE1000 embeds its direct parent only
pub struct VirtualE1000 {
    pub virtual_ethernet_card_: VirtualEthernetCard,  // parent field (note trailing _)
    // VirtualE1000-specific fields (if any)
}

impl Deref for VirtualE1000 {
    type Target = VirtualEthernetCard;
    fn deref(&self) -> &Self::Target { &self.virtual_ethernet_card_ }
}
```

### Field access — usually no change needed

Thanks to `Deref` coercion, **read access to fields works without changes**:

```rust
// Works the same in 0.3.0 and 0.4.0
println!("{:?}", e1000.mac_address);  // Deref chain resolves automatically
println!("{}", e1000.key);
```

### Construction — you must populate the parent field

```rust
// 0.3.0
let e1000 = VirtualE1000 {
    key: 100,
    mac_address: Some("00:50:56:aa:bb:cc".to_string()),
    // ... all other fields
    ..Default::default()
};

// 0.4.0 — with `defaults` feature enabled
let e1000 = VirtualE1000 {
    virtual_ethernet_card_: VirtualEthernetCard {
        virtual_device_: VirtualDevice {
            key: 100,
            ..Default::default()
        },
        mac_address: Some("00:50:56:aa:bb:cc".to_string()),
        ..Default::default()
    },
    ..Default::default()
};

// 0.4.0 — without `defaults` feature, set every field explicitly
```

> **Tip:** Enable the `defaults` feature to keep construction concise:
> ```toml
> vim_rs = { version = "0.4", features = ["defaults"] }
> ```

### Parent field naming convention

The parent field name is always the snake_case version of the parent type name followed by a trailing `_`:

| Parent type | Field name |
|---|---|
| `VirtualEthernetCard` | `virtual_ethernet_card_` |
| `VirtualDevice` | `virtual_device_` |
| `SelectionSpec` | `selection_spec_` |
| `EamObjectRuntimeInfo` | `eam_object_runtime_info_` |

For multi-level inheritance, nest the structs accordingly:

```rust
// VirtualE1000 → VirtualEthernetCard → VirtualDevice → DynamicData
let e1000 = VirtualE1000 {
    virtual_ethernet_card_: VirtualEthernetCard {
        virtual_device_: VirtualDevice {
            // VirtualDevice fields
            ..Default::default()
        },
        // VirtualEthernetCard fields
        ..Default::default()
    },
    // VirtualE1000 fields (if any)
    ..Default::default()
};
```

---

## 2. Trait getter methods removed

All `get_*()` accessor methods on traits (e.g. `VirtualDeviceTrait`, `VirtualEthernetCardTrait`) have been removed. Use direct field access via Deref coercion instead.

```rust
// 0.3.0
let key: i32 = device.get_key();
let mac: Option<&String> = eth.get_mac_address();
let backing = device.get_backing();

// 0.4.0 — direct field access (works through Deref automatically)
let key: i32 = device.key;
let mac: Option<&String> = eth.mac_address.as_ref();
let backing = device.backing.as_deref();
```

When working with trait objects (`&dyn VirtualDeviceTrait`, `Box<dyn VirtualEthernetCardTrait>`), Deref coercion still applies:

```rust
fn describe(device: &dyn VirtualDeviceTrait) {
    // 0.3.0
    println!("key={}", device.get_key());

    // 0.4.0
    println!("key={}", device.key);
}
```

---

## 3. Enum API changes

### 3.1 `as_str()` lifetime change

`as_str()` now returns `&str` bound to the **instance lifetime** instead of `&'static str`. This is required to support the `Other_` variant correctly.

```rust
// 0.3.0
fn get_name(e: &MyEnum) -> &'static str {
    e.as_str()
}

// 0.4.0 — lifetime tied to the enum value
fn get_name(e: &MyEnum) -> &str {
    e.as_str()
}
```

If you stored `&'static str` slices returned by `as_str()`, store `String` instead:

```rust
// 0.3.0
let s: &'static str = my_enum.as_str();

// 0.4.0
let s: &str = my_enum.as_str();               // borrow
let s: String = my_enum.as_str().to_owned();  // own
```

### 3.2 `From<Enum> for &'static str` removed

```rust
// 0.3.0
let s: &'static str = my_enum.into();          // From impl
let s: &'static str = <&str>::from(my_enum);

// 0.4.0 — use as_str() explicitly
let s: &str = my_enum.as_str();
let s: String = my_enum.to_string();  // Display is still implemented
```

### 3.3 `Other_` variant — forward-compatible enum matching

Every enum now has an `Other_(String)` variant for values not present in the OpenAPI spec at the time of code generation. When matching exhaustively, handle it:

```rust
match power_state {
    VirtualMachinePowerStateEnum::PoweredOn => { /* ... */ }
    VirtualMachinePowerStateEnum::PoweredOff => { /* ... */ }
    VirtualMachinePowerStateEnum::Suspended => { /* ... */ }
    VirtualMachinePowerStateEnum::Other_(s) => {
        eprintln!("Unknown power state from server: {s}");
    }
}
```

### 3.4 `from_str()` replaces strum

If you used `strum`'s `from_str` or `EnumString` to convert string → enum:

```rust
// 0.3.0 (strum)
use std::str::FromStr;
let e = VirtualMachinePowerStateEnum::from_str("poweredOn").unwrap();

// 0.4.0
let e = VirtualMachinePowerStateEnum::from_str("poweredOn");
// Returns the enum value or Other_("poweredOn".to_string()) — never panics
```

---

## 4. DataObject trait methods removed

`DataObjectTrait` no longer exposes getter methods. Empty descendant types (those with no fields of their own across the entire inheritance chain) are pruned from the type hierarchy.

If your code called methods on `&dyn DataObjectTrait` beyond type discrimination, switch to downcasting:

```rust
use vim_rs::types::as_any::AsAny;
use vim_rs::types::structs::SomeConcreteType;

// 0.3.0
let value = obj.get_some_field();

// 0.4.0 — downcast first, then access fields
if let Some(concrete) = obj.as_any_ref().downcast_ref::<SomeConcreteType>() {
    let value = &concrete.some_field;
}
```

---

## 5. miniserde replaces serde/serde_json

All generated types (`vim_rs::types::structs`, `::enums`, `::traits`) now derive `miniserde::Serialize`/`Deserialize` instead of `serde`. For most users this is invisible — the library's API methods handle all serialisation internally. You only need to act here if you were directly serialising or deserialising vim_rs types yourself.

### 5.1 Cargo.toml

```toml
# Before
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# After
[dependencies]
miniserde = "0.1"
# serde and serde_json are no longer needed unless used for non-vim_rs types
```

### 5.2 Parsing a response string

```rust
// Before
let response: MyVimType = serde_json::from_str(&body)?;

// After
let response: MyVimType = miniserde::json::from_str(&body)?;
```

### 5.3 Serialising to a string

```rust
// Before
let body = serde_json::to_string(&my_spec)?;

// After
let body = miniserde::json::to_string(&my_spec);
// Note: miniserde::json::to_string returns String, not Result<String>
```

### 5.4 Working with a parsed Value (replacing `serde_json::from_value`)

`miniserde` has no direct equivalent to `serde_json::from_value`. Use the helper provided:

```rust
use vim_rs::types::mini_helpers::from_value;

// Before
let mor: ManagedObjectReference = serde_json::from_value(raw_value.clone())?;

// After
let mor: ManagedObjectReference = from_value(&raw_value)?;
// The helper accepts &miniserde::json::Value and returns miniserde::Result<T>
```

### 5.5 Base64-encoded byte fields

If you were decoding `Vec<u8>` fields serialised as Base64 strings:

```rust
use vim_rs::types::mini_helpers::Base64;

// Before (serde with base64 feature or custom deserializer)
let bytes: Vec<u8> = base64::decode(&encoded_string)?;

// After — wrap the field in Base64; the helper handles decode/encode
let wrapper: Base64 = miniserde::json::from_str(&format!("\"{}\"", encoded_string))?;
let bytes: Vec<u8> = wrapper.0;
```

### 5.6 Derive macros on your own types

miniserde derive macros are simpler than serde's. Key differences:

| Feature | serde | miniserde |
|---|---|---|
| `#[derive(Serialize, Deserialize)]` | ✅ | ✅ |
| `#[serde(rename = "...")]` | ✅ | ❌ — not supported |
| `#[serde(skip_serializing_if)]` | ✅ | ❌ |
| `#[serde(flatten)]` | ✅ | ❌ |
| Untagged enums | ✅ | ❌ |
| `serde_json::Value` | ✅ | `miniserde::json::Value` |

If your own structs used `#[serde(rename)]` or other attributes to talk to vim_rs types, you will need to implement miniserde traits manually or adjust field names to match the wire format.

---

## 6. Cargo.toml dependency updates

### vim_rs version

```toml
# Before
vim_rs = "0.3"

# After
vim_rs = "0.4"

# With the defaults feature (recommended for struct construction)
vim_rs = { version = "0.4", features = ["defaults"] }
```

### Dependencies to remove

```toml
# These are no longer pulled in by vim_rs — remove if you only used them for vim_rs types:
serde = { version = "1", features = ["derive"] }
serde_json = "1"
erased-serde = "0.4"
strum = "0.26"
strum_macros = "0.26"
```

### Dependencies to add

```toml
miniserde = "0.1"
```

---

## 7. Quick-reference cheat sheet

| What you had (0.3.0) | What to use (0.4.0) |
|---|---|
| `device.get_key()` | `device.key` |
| `eth.get_mac_address()` | `eth.mac_address.as_ref()` |
| `VirtualE1000 { key: 0, mac_address: ..., .. }` | `VirtualE1000 { virtual_ethernet_card_: VirtualEthernetCard { virtual_device_: VirtualDevice { key: 0, .. }, mac_address: ..., .. }, .. }` |
| `my_enum.into()` → `&'static str` | `my_enum.as_str()` |
| `as_str()` returns `&'static str` | `as_str()` returns `&str` (instance lifetime) |
| `MyEnum::from_str("val").unwrap()` (strum) | `MyEnum::from_str("val")` (returns value or `Other_`) |
| `serde_json::from_str(&s)?` | `miniserde::json::from_str(&s)?` |
| `serde_json::to_string(&v)?` | `miniserde::json::to_string(&v)` |
| `serde_json::from_value(v)` | `vim_rs::types::mini_helpers::from_value(&v)?` |
| `use serde::{Serialize, Deserialize}` | `use miniserde::{Serialize, Deserialize}` |

---

## Need help?

- File issues at the vim_rs repository issue tracker.
- The `CHANGELOG.md` at the repo root documents every change with additional context.
- `docs/inheritance_model_notes.md` explains the compositional inheritance design in depth.
