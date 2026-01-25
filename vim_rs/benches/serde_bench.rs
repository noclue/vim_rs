use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vim_rs::types::boxed_types::ValueElements;
use vim_rs::types::structs::{VirtualDevice, VirtualE1000, VirtualEthernetCard, MethodFault};
use vim_rs::types::traits::VirtualEthernetCardTrait;

const JSON_VAPP_PROPERTY_FAULT: &str = r#"{
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
                },
                {
                    "_typeName": "KeyAnyValue",
                    "key": "vm.name",
                    "value": {
                        "_typeName": "string",
                        "_value": "test-vm"
                    }
                }
            ],
            "message": "Product Version: 1.0.0, VM Name: test-vm"
        }
    ],
    "id": "config.product.version",
    "category": "string",
    "label": "Product Version",
    "type": "string",
    "value": "1.0.0"
}"#;

// Test data setup functions
fn create_virtual_e1000() -> VirtualE1000 {
    VirtualE1000 {
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
    }
}

fn create_vapp_property_fault() -> MethodFault {
    serde_json::from_str(JSON_VAPP_PROPERTY_FAULT).unwrap()
}


// Benchmark: Plain structure (e1000) serialization
fn bench_e1000_serialize(c: &mut Criterion) {
    let e1000 = create_virtual_e1000();
    
    c.bench_function("e1000_serialize", |b| {
        b.iter(|| {
            let json = serde_json::to_string(black_box(&e1000)).unwrap();
            black_box(json);
        });
    });
}

// Benchmark: Plain structure (e1000) deserialization
fn bench_e1000_deserialize(c: &mut Criterion) {
    let json = r#"{"_typeName":"VirtualE1000","key":1000,"address_type":"Generated","mac_address":"00:50:56:aa:bb:cc","wake_on_lan_enabled":true,"upt_compatibility_enabled":false}"#;
    
    c.bench_function("e1000_deserialize", |b| {
        b.iter(|| {
            let e1000: VirtualE1000 = serde_json::from_str(black_box(json)).unwrap();
            black_box(e1000);
        });
    });
}

// Benchmark: Purged type (VAppPropertyFault) serialization
fn bench_vapp_property_fault_serialize(c: &mut Criterion) {
    let vapp_property_fault = create_vapp_property_fault();
    
    c.bench_function("vapp_property_fault_serialize", |b| {
        b.iter(|| {
            let json = serde_json::to_string(black_box(&vapp_property_fault)).unwrap();
            black_box(json);
        });
    });
}

// Benchmark: Purged type (VAppPropertyFault) deserialization
fn bench_vapp_property_fault_deserialize(c: &mut Criterion) {
    c.bench_function("vapp_property_fault_deserialize", |b| {
        b.iter(|| {
            let vapp_property_fault: MethodFault = serde_json::from_str(black_box(JSON_VAPP_PROPERTY_FAULT)).unwrap();
            black_box(vapp_property_fault);
        });
    });
}

// Benchmark: Polymorphic serialization (e1000 via VirtualEthernetCardTrait)
fn bench_polymorphic_serialize(c: &mut Criterion) {
    let e1000 = create_virtual_e1000();
    let trait_ref: &dyn VirtualEthernetCardTrait = &e1000;
    
    c.bench_function("polymorphic_e1000_serialize", |b| {
        b.iter(|| {
            let json = serde_json::to_string(black_box(trait_ref)).unwrap();
            black_box(json);
        });
    });
}

// Benchmark: Polymorphic deserialization (into Box<dyn VirtualEthernetCardTrait>)
fn bench_polymorphic_deserialize(c: &mut Criterion) {
    let json = r#"{"_typeName":"VirtualE1000","key":1000,"address_type":"Generated","mac_address":"00:50:56:aa:bb:cc","wake_on_lan_enabled":true,"upt_compatibility_enabled":false}"#;
    
    c.bench_function("polymorphic_e1000_deserialize", |b| {
        b.iter(|| {
            let e1000: Box<dyn VirtualEthernetCardTrait> = serde_json::from_str(black_box(json)).unwrap();
            black_box(e1000);
        });
    });
}

// Benchmark: Round-trip serialization for each pattern
fn bench_roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("roundtrip");
    
    // VirtualE1000 roundtrip (plain struct)
    group.bench_function("e1000", |b| {
        let e1000 = create_virtual_e1000();
        b.iter(|| {
            let json = serde_json::to_string(black_box(&e1000)).unwrap();
            let recovered: VirtualE1000 = serde_json::from_str(&json).unwrap();
            black_box(recovered);
        });
    });
    
    // MethodFault roundtrip (purged type)
    group.bench_function("method_fault", |b| {
        let fault = create_vapp_property_fault();
        b.iter(|| {
            let json = serde_json::to_string(black_box(&fault)).unwrap();
            let recovered: MethodFault = serde_json::from_str(&json).unwrap();
            black_box(recovered);
        });
    });
    
    // Polymorphic roundtrip (VirtualE1000 via VirtualEthernetCardTrait)
    group.bench_function("polymorphic_e1000", |b| {
        let e1000 = create_virtual_e1000();
        b.iter(|| {
            let trait_ref: &dyn VirtualEthernetCardTrait = black_box(&e1000);
            let json = serde_json::to_string(trait_ref).unwrap();
            let recovered: Box<dyn VirtualEthernetCardTrait> = serde_json::from_str(&json).unwrap();
            black_box(recovered);
        });
    });
    
    group.finish();
}

// Benchmark: Array deserialization
fn bench_array_deserialize(c: &mut Criterion) {
    let json = r#"{"_typeName":"ArrayOfVirtualEthernetCard","_value":[
        {"_typeName":"VirtualE1000","key":1000,"address_type":"Generated","mac_address":"00:50:56:aa:bb:cc","wake_on_lan_enabled":true},
        {"_typeName":"VirtualE1000","key":1001,"address_type":"Generated","mac_address":"00:50:56:aa:bb:dd","wake_on_lan_enabled":false},
        {"_typeName":"VirtualE1000","key":1002,"address_type":"Manual","mac_address":"00:50:56:aa:bb:ee","wake_on_lan_enabled":true}
    ]}"#;
    
    c.bench_function("array_of_virtual_ethernet_card_deserialize", |b| {
        b.iter(|| {
            let value: ValueElements = serde_json::from_str(black_box(json)).unwrap();
            black_box(value);
        });
    });
}

criterion_group!(
    benches,
    bench_e1000_serialize,
    bench_e1000_deserialize,
    bench_vapp_property_fault_serialize,
    bench_vapp_property_fault_deserialize,
    bench_polymorphic_serialize,
    bench_polymorphic_deserialize,
    bench_roundtrip,
    bench_array_deserialize,
);

criterion_main!(benches);
