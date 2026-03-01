#![cfg(feature = "defaults")]

//! Tests for the `defaults` feature: Default implementations on structs, enums, and trait objects.

use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::structs::{ManagedObjectReference, VirtualDevice, VirtualE1000};
use vim_rs::types::traits::VirtualDeviceTrait;

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn struct_default_virtual_device() {
    init();
    let vd = VirtualDevice::default();
    assert_eq!(vd.key, 0);
    assert!(vd.backing.is_none());
    assert!(vd.controller_key.is_none());
}

#[test]
fn struct_default_managed_object_reference() {
    init();
    let mor = ManagedObjectReference::default();
    assert_eq!(mor.value, "");
}

#[test]
fn enum_default_first_variant() {
    init();
    let e = MoTypesEnum::default();
    assert_eq!(e.as_str(), "Alarm");
}

#[test]
fn trait_object_default_creates_base_struct() {
    init();
    let vd: Box<dyn VirtualDeviceTrait> = Default::default();
    assert_eq!(vd.key, 0);
}

#[test]
fn nested_composition_default() {
    init();
    let e1000 = VirtualE1000::default();
    assert_eq!(e1000.key, 0);
    assert_eq!(e1000.mac_address, None);
}

#[test]
fn struct_update_syntax() {
    init();
    let vd = VirtualDevice {
        key: 42,
        ..Default::default()
    };
    assert_eq!(vd.key, 42);
    assert_eq!(vd.controller_key, None);
}
