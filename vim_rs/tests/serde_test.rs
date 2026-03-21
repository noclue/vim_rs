use vim_rs::types::structs::{VirtualE1000, VirtualEthernetCard, VirtualDevice, MethodFault};
use vim_rs::types::traits::VirtualEthernetCardTrait;
use vim_rs::types::boxed_types::ValueElements;

// this is the format used for empty and unset array in VI/JSON. Note in XML
// there is no way to differentiate null, from empty from missing array.
// See the error test below where one of the errors has no arguments (empty
// arguments).
#[test]
fn test_json_update_set() {
    let json_text = r#"{"version":"2"}"#;
    let result: vim_rs::types::structs::UpdateSet = miniserde::json::from_str(&json_text).unwrap();
    assert_eq!(result.version, "2");
    assert!(result.truncated.is_none());
    assert!(result.filter_set.is_none());
}

// This deserializaiton fails need to debug. Not used in VI/JSON so it is not a high priority.
// #[test]
// fn test_json_null_update_set() {
//     let json_text = r#"{"version":"2", "filterSet":null }"#;
//     let result: vim_rs::types::structs::UpdateSet = miniserde::json::from_str(&json_text).unwrap();
//     assert_eq!(result.version, "2");
//     assert!(result.truncated.is_none());
//     assert!(result.filter_set.is_none());
// }

// This works with the current JSON code but is not used in VI/JSON.
#[test]
fn test_json_empty_update_set() {
    let json_text = r#"{"version":"2", "filterSet":[] }"#;
    let result: vim_rs::types::structs::UpdateSet = miniserde::json::from_str(&json_text).unwrap();
    assert_eq!(result.version, "2");
    assert!(result.truncated.is_none());
    assert!(result.filter_set.is_some());
    assert!(result.filter_set.unwrap().is_empty());
}


#[test]
fn test_e1000_roundtrip() {
    let e1000 = VirtualE1000 {
        virtual_ethernet_card_: VirtualEthernetCard {
            virtual_device_: VirtualDevice {
                numa_node: None,
                device_group_info: None,
                unit_number: None,
                backing: None,
                controller_key: None,
                slot_info: None,
                key: 1000,
                device_info: None,
                connectable: None,
            },
            dynamic_property: None,
            mac_address: Some("00:50:56:aa:bb:cc".to_string()),
            wake_on_lan_enabled: Some(true),
            address_type: Some("Generated".to_string()),
            subnet_id: None,
            resource_allocation: None,
            upt_compatibility_enabled: Some(false),
            external_id: None,
        },
    };

    let json = miniserde::json::to_string(&e1000);
    println!("E1000 JSON: {}", json);
    let recovered: VirtualE1000 = miniserde::json::from_str(&json).unwrap();
    assert_eq!(recovered.virtual_ethernet_card_.mac_address, Some("00:50:56:aa:bb:cc".to_string()));
}

#[test]
fn test_e1000_polymorphic_roundtrip() {
    let e1000 = VirtualE1000 {
        virtual_ethernet_card_: VirtualEthernetCard {
            virtual_device_: VirtualDevice {
                numa_node: None,
                device_group_info: None,
                unit_number: None,
                backing: None,
                controller_key: None,
                slot_info: None,
                key: 1000,
                device_info: None,
                connectable: None,
            },
            dynamic_property: None,
            mac_address: Some("00:50:56:aa:bb:cc".to_string()),
            wake_on_lan_enabled: Some(true),
            address_type: Some("Generated".to_string()),
            subnet_id: None,
            resource_allocation: None,
            upt_compatibility_enabled: Some(false),
            external_id: None,
        },
    };

    let trait_ref: &dyn VirtualEthernetCardTrait = &e1000;
    let json = miniserde::json::to_string(trait_ref);
    println!("Polymorphic E1000 JSON: {}", json);
    let recovered: Box<dyn VirtualEthernetCardTrait> = miniserde::json::from_str(&json).unwrap();
    // Can't easily assert on trait object, but at least verify it deserializes
    drop(recovered);
}

#[test]
fn test_vapp_property_fault_simple() {
    // Simple version without nested KeyAnyValue
    let json = r#"{
        "_typeName": "VAppPropertyFault",
        "id": "config.product.version",
        "category": "string",
        "label": "Product Version",
        "type": "string",
        "value": "1.0.0"
    }"#;
    
    let fault: MethodFault = miniserde::json::from_str(json).unwrap();
    println!("Fault type: {:?}", fault.type_);
}

#[test]
fn test_vapp_property_fault_with_message() {
    // Version with LocalizableMessage but no args
    let json = r#"{
        "_typeName": "VAppPropertyFault",
        "faultMessage": [
            {
                "_typeName": "LocalizableMessage",
                "key": "config.product.version",
                "message": "Product Version: 1.0.0"
            }
        ],
        "id": "config.product.version",
        "category": "string",
        "label": "Product Version",
        "type": "string",
        "value": "1.0.0"
    }"#;
    
    let fault: MethodFault = miniserde::json::from_str(json).unwrap();
    println!("Fault with message: {:?}", fault);
    assert!(fault.fault_message.is_some());
}

#[test]
fn test_vapp_property_fault_with_args() {
    // Full version with KeyAnyValue in args
    let json = r#"{
        "_typeName": "VAppPropertyFault",
        "faultMessage": [
            {
                "_typeName": "LocalizableMessage",
                "key": "config.product.version",
                "arg": [
                    {
                        "_typeName": "KeyAnyValue",
                        "key": "config.product.version",
                        "value": {
                            "_typeName": "string",
                            "_value": "1.0.0"
                        }
                    }
                ],
                "message": "Product Version: 1.0.0"
            }
        ],
        "id": "config.product.version",
        "category": "string",
        "label": "Product Version",
        "type": "string",
        "value": "1.0.0"
    }"#;
    
    let result = miniserde::json::from_str::<MethodFault>(json);
    match result {
        Ok(fault) => {
            println!("Successfully parsed fault with args: {:?}", fault);
            assert!(fault.extra_fields_.get("label").is_some());
        }
        Err(e) => {
            println!("Failed to parse: {:?}", e);
            panic!("Should have parsed successfully");
        }
    }
}

#[test]
fn test_array_of_virtual_ethernet_card() {
    let json = r#"{
        "_typeName": "ArrayOfVirtualEthernetCard",
        "_value": [
            {
                "_typeName": "VirtualE1000",
                "key": 1000,
                "address_type": "Generated",
                "mac_address": "00:50:56:aa:bb:cc",
                "wake_on_lan_enabled": true
            },
            {
                "_typeName": "VirtualE1000",
                "key": 1001,
                "address_type": "Generated",
                "mac_address": "00:50:56:aa:bb:dd",
                "wake_on_lan_enabled": false
            }
        ]
    }"#;
    
    let value: ValueElements = miniserde::json::from_str(json).unwrap();
    println!("Array deserialized successfully");
    
    // Verify it's the right variant
    if let ValueElements::ArrayOfVirtualEthernetCard(cards) = value {
        assert_eq!(cards.len(), 2);
    } else {
        panic!("Expected ArrayOfVirtualEthernetCard variant");
    }
}


#[test]
fn test_array_of_virtual_ethernet_card_empty() {
    let json = r#"{
        "_typeName": "ArrayOfVirtualEthernetCard",
        "_value": []
    }"#;
    
    let value: ValueElements = miniserde::json::from_str(json).unwrap();
    println!("Array deserialized successfully");
    
    // Verify it's the right variant
    if let ValueElements::ArrayOfVirtualEthernetCard(cards) = value {
        assert!(cards.is_empty());
    } else {
        panic!("Expected ArrayOfVirtualEthernetCard variant");
    }
}