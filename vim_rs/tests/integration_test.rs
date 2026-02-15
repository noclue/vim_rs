use log::{debug, info};
use std::convert::AsRef;
use vim_rs::types::boxed_types::ValueElements;
use vim_rs::types::convert::CastInto;
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::structs::{VirtualDevice, VirtualE1000, VirtualEthernetCard};
use vim_rs::types::traits::VirtualDeviceTrait;
use vim_rs::types::traits::VirtualEthernetCardTrait;
use vim_rs::types::vim_any::VimAny;
use vim_rs::types::vim_object_trait::VimObjectTrait;

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn test_vim_any_box() {
    init();
    let e1000 = create_virtual_e1000();
    let s = miniserde::json::to_string(&e1000);
    debug!("{}", s);
    let any: VimAny = miniserde::json::from_str(&s).unwrap();
    info!("{:?}", any);
}

#[test]
fn test_eth_json() {
    init();
    let e1000 = create_virtual_e1000();
    let s = miniserde::json::to_string(&e1000);
    debug!("{}", s);
    let eth: VirtualE1000 = miniserde::json::from_str(&s).unwrap();
    info!("{:?}", eth);
}

#[test]
fn test_ethernet_box() {
    init();
    let e1000 = create_virtual_e1000();
    let s = miniserde::json::to_string(&e1000);
    debug!("{}", s);
    let vd: Box<dyn VirtualDeviceTrait> = miniserde::json::from_str(&s).unwrap();
    assert_eq!(vd.key, 1000);
    let eth: Box<dyn VirtualEthernetCardTrait> = vd.into_box().unwrap();
    assert_eq!(
        eth.mac_address,
        Some("00:50:56:aa:bb:cc".to_string())
    );
    info!("{:?}", eth);
}

#[test]
fn test_ethernet_ref() {
    init();
    let e1000 = &create_virtual_e1000();
    let vd: &dyn VirtualDeviceTrait = e1000;
    assert_eq!(vd.key, 1000);
    let eth: &dyn VirtualEthernetCardTrait = vd.into_ref().unwrap();
    assert_eq!(
        eth.mac_address,
        Some("00:50:56:aa:bb:cc".to_string())
    );
    info!("{:?}", eth);
}

#[test]
fn consume_vim_any_as_box() {
    init();
    let vim_any = eth_as_any();

    let e1000: Box<VirtualE1000> = match vim_any {
        VimAny::Object(e) => e.as_any_box().downcast().unwrap(),
        _ => panic!("Unexpected type"),
    };
    assert_eq!(e1000.key, 1000);
    info!("{:?}", e1000);
}

#[test]
fn consume_vim_any_as_ref() {
    init();
    let vim_any = eth_as_any();
    let VimAny::Object(vim_obj) = vim_any else {
        panic!("Unexpected type");
    };
    let e1000: &VirtualE1000 = vim_obj.as_any_ref().downcast_ref().unwrap();
    assert_eq!(e1000.key, 1000);
    info!("{:?}", e1000);
}

#[test]
fn consume_vim_any_as_trait_box() {
    init();
    let vim_any = eth_as_any();
    let VimAny::Object(vim_obj) = vim_any else {
        panic!("Unexpected type");
    };
    let eth: Box<dyn VirtualEthernetCardTrait> = vim_obj.into_box().unwrap();

    assert_eq!(eth.key, 1000);
    info!("{:?}", eth);
}

#[test]
fn consume_vim_any_as_trait_ref() {
    init();
    let vim_any = eth_as_any();
    let VimAny::Object(vim_obj) = vim_any else {
        panic!("Unexpected type");
    };
    let eth: &dyn VirtualEthernetCardTrait = vim_obj.as_ref().into_ref().unwrap();

    assert_eq!(eth.key, 1000);
    info!("{:?}", eth);
}

#[test]
fn consume_vim_any_array() {
    init();
    let vim_any = create_virtual_device_array();
    let VimAny::Value(ValueElements::ArrayOfVirtualDevice(vd)) = vim_any else {
        panic!("Unexpected type");
    };
    let e1000 = vd[0].as_any_ref().downcast_ref::<VirtualE1000>().unwrap();
    assert_eq!(e1000.key, 1000);
    info!("{:?}", e1000);
}

#[test]
fn enum_as_string() {
    init();
    assert_eq!(MoTypesEnum::VirtualMachine.as_str(), "VirtualMachine");
}

#[test]
fn enum_unknown_as_string() {
    init();
    let e = MoTypesEnum::Other_(String::from("Container"));
    assert_eq!(e.as_str(), "Container");
}

#[test]
fn type_name() {
    init();
    let e1000 = create_virtual_e1000();
    assert_eq!(e1000.data_type().as_str(), "VirtualE1000");
}

fn create_virtual_device_array() -> VimAny {
    VimAny::Value(ValueElements::ArrayOfVirtualDevice(vec![Box::new(
        create_virtual_e1000(),
    )]))
}

fn eth_as_any() -> VimAny {
    VimAny::Object(Box::new(create_virtual_e1000()))
}

fn create_virtual_e1000() -> VirtualE1000 {
    VirtualE1000 {
        virtual_ethernet_card_: VirtualEthernetCard {
            virtual_device_: VirtualDevice {
                backing: None,
                controller_key: None,
                slot_info: None,
                key: 1000,
                device_info: None,
                connectable: None,
                unit_number: None,
                numa_node: None,
                device_group_info: None,
            },
            mac_address: Some("00:50:56:aa:bb:cc".to_string()),
            wake_on_lan_enabled: Some(true),
            address_type: Some("Generated".to_string()),
            resource_allocation: None,
            upt_compatibility_enabled: Some(false),
            external_id: None,
            dynamic_property: None,
            subnet_id: None,
        },
    }
}
