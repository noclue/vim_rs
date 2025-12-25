use vim_rs::types::boxed_types::ValueElements;
use vim_rs::types::convert::CastInto;
use vim_rs::types::structs::VirtualE1000;
use vim_rs::types::traits::VirtualEthernetCardTrait;
use vim_rs::types::vim_any::VimAny;

#[test]
fn narrow_vim_any_array_of_virtual_device() {
    // NOTE: In vim_rs the “VirtualDevice” base is modeled as a trait:
    // `Box<dyn VirtualDeviceTrait>`, not `Box<VirtualDevice>`.
    let any = VimAny::Value(ValueElements::ArrayOfVirtualDevice(vec![Box::new(
        create_virtual_e1000(),
    )]));

    let VimAny::Value(ValueElements::ArrayOfVirtualDevice(devices)) = any else {
        panic!("expected VimAny::Value(ArrayOfVirtualDevice)");
    };

    let e1000 = devices[0]
        .as_any_ref()
        .downcast_ref::<VirtualE1000>()
        .expect("first device should be VirtualE1000");

    assert_eq!(e1000.key, 1000);
}

#[test]
fn downcast_vim_any_object_to_concrete_type() {
    let any = VimAny::Object(Box::new(create_virtual_e1000()));

    let VimAny::Object(obj) = any else {
        panic!("expected VimAny::Object");
    };

    let e1000: Box<VirtualE1000> = obj.as_any_box().downcast().expect("downcast should work");
    assert_eq!(e1000.key, 1000);
}

#[test]
fn narrow_vim_any_object_to_trait_box() {
    let any = VimAny::Object(Box::new(create_virtual_e1000()));

    let VimAny::Object(obj) = any else {
        panic!("expected VimAny::Object");
    };

    // Trait narrowing without JSON: VimAny::Object(Box<dyn VimObjectTrait>) -> Box<dyn VirtualEthernetCardTrait>
    let eth: Box<dyn VirtualEthernetCardTrait> = obj.into_box().expect("trait cast should work");
    assert_eq!(eth.get_key(), 1000);
    assert_eq!(
        eth.get_mac_address().as_deref(),
        Some("00:50:56:aa:bb:cc")
    );
}


fn create_virtual_e1000() -> VirtualE1000 {
    VirtualE1000 {
        mac_address: Some("00:50:56:aa:bb:cc".to_string()),
        wake_on_lan_enabled: Some(true),
        address_type: Some("Generated".to_string()),
        numa_node: None,
        device_group_info: None,
        resource_allocation: None,
        upt_compatibility_enabled: Some(false),
        external_id: None,
        unit_number: None,
        backing: None,
        controller_key: None,
        slot_info: None,
        key: 1000,
        device_info: None,
        connectable: None,
        dynamic_property: None,
        subnet_id: None,
    }
}