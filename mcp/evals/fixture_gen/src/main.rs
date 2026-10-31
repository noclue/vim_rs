//! Generate VI-JSON fixture files for eval problems.

use std::fs;
use std::path::PathBuf;

use vim_rs::types::boxed_types::ValueElements;
use vim_rs::types::enums::{
    ManagedEntityStatusEnum, MoTypesEnum, VirtualMachinePowerStateEnum,
};
use vim_rs::types::structs::{
    Description, DynamicProperty, ManagedObjectReference, ObjectContent, RetrieveResult,
    VirtualCdrom, VirtualDevice, VirtualE1000, VirtualEthernetCard, VirtualVmxnet,
    VirtualVmxnet3,
};
use vim_rs::types::traits::{DescriptionTrait, VirtualDeviceTrait};
use vim_rs::types::vim_any::VimAny;

fn vm_mor(id: &str) -> ManagedObjectReference {
    ManagedObjectReference {
        r#type: MoTypesEnum::VirtualMachine,
        value: id.to_string(),
    }
}

fn host_mor(id: &str) -> ManagedObjectReference {
    ManagedObjectReference {
        r#type: MoTypesEnum::HostSystem,
        value: id.to_string(),
    }
}

fn power_prop(state: VirtualMachinePowerStateEnum) -> DynamicProperty {
    DynamicProperty {
        name: "runtime.powerState".to_string(),
        val: VimAny::Value(ValueElements::VirtualMachinePowerState(state)),
    }
}

fn guest_host_prop(host: &str) -> DynamicProperty {
    DynamicProperty {
        name: "guest.hostName".to_string(),
        val: VimAny::Value(ValueElements::PrimitiveString(host.to_string())),
    }
}

fn name_prop(name: &str) -> DynamicProperty {
    DynamicProperty {
        name: "name".to_string(),
        val: VimAny::Value(ValueElements::PrimitiveString(name.to_string())),
    }
}

fn num_cpu_prop(n: i32) -> DynamicProperty {
    DynamicProperty {
        name: "config.hardware.numCPU".to_string(),
        val: VimAny::Value(ValueElements::PrimitiveInt(n)),
    }
}

fn overall_status_prop(status: ManagedEntityStatusEnum) -> DynamicProperty {
    DynamicProperty {
        name: "summary.overallStatus".to_string(),
        val: VimAny::Value(ValueElements::ManagedEntityStatus(status)),
    }
}

fn product_full_name_prop(name: &str) -> DynamicProperty {
    DynamicProperty {
        name: "summary.config.product.fullName".to_string(),
        val: VimAny::Value(ValueElements::PrimitiveString(name.to_string())),
    }
}

/// A device to place in a VM's `config.hardware.device` array.
/// Ethernet variants carry a MAC; non-ethernet (Cdrom) do not — so the
/// grader task must trait-downcast to `VirtualEthernetCardTrait` and filter,
/// not read a field off the base `VirtualDevice`.
enum Dev {
    E1000(&'static str),
    Vmxnet3(&'static str),
    Cdrom,
}

fn base_device(key: i32) -> VirtualDevice {
    VirtualDevice {
        numa_node: None,
        device_group_info: None,
        unit_number: None,
        backing: None,
        controller_key: None,
        slot_info: None,
        key,
        device_info: None,
        connectable: None,
    }
}

fn eth_card(mac: &str, key: i32) -> VirtualEthernetCard {
    VirtualEthernetCard {
        virtual_device_: base_device(key),
        dynamic_property: None,
        mac_address: Some(mac.to_string()),
        wake_on_lan_enabled: None,
        address_type: None,
        subnet_id: None,
        resource_allocation: None,
        upt_compatibility_enabled: None,
        external_id: None,
    }
}

/// Build a mixed `config.hardware.device` array in the given order. Keys are
/// assigned by position so device order is deterministic.
fn device_array_prop(specs: &[Dev]) -> DynamicProperty {
    let devices: Vec<Box<dyn VirtualDeviceTrait>> = specs
        .iter()
        .enumerate()
        .map(|(i, spec)| {
            let key = 1000 + i as i32;
            let dev: Box<dyn VirtualDeviceTrait> = match spec {
                Dev::E1000(mac) => Box::new(VirtualE1000 {
                    virtual_ethernet_card_: eth_card(mac, key),
                }),
                Dev::Vmxnet3(mac) => Box::new(VirtualVmxnet3 {
                    virtual_vmxnet_: VirtualVmxnet {
                        virtual_ethernet_card_: eth_card(mac, key),
                    },
                    uptv_2_enabled: None,
                    strict_latency_config: None,
                }),
                Dev::Cdrom => Box::new(VirtualCdrom {
                    virtual_device_: base_device(key),
                }),
            };
            dev
        })
        .collect();
    DynamicProperty {
        name: "config.hardware.device".to_string(),
        val: VimAny::Value(ValueElements::ArrayOfVirtualDevice(devices)),
    }
}

// --- pc05: devices carry a label in the base VirtualDevice.deviceInfo ---
// The label lives on the *base* type (VirtualDevice.device_info: Description),
// reachable from an ethernet card only via the supertrait, and device_info is
// itself polymorphic (Box<dyn DescriptionTrait>).

/// A labeled device: ethernet variants carry (mac, label); Cdrom carries a
/// label but no MAC (must be filtered out).
enum LDev {
    E1000(&'static str, &'static str),
    Vmxnet3(&'static str, &'static str),
    Cdrom(&'static str),
}

fn labeled_device(key: i32, label: &str) -> VirtualDevice {
    VirtualDevice {
        device_info: Some(Box::new(Description {
            label: label.to_string(),
            summary: format!("{label} — summary"),
        }) as Box<dyn DescriptionTrait>),
        ..base_device(key)
    }
}

fn labeled_eth_card(mac: &str, label: &str, key: i32) -> VirtualEthernetCard {
    VirtualEthernetCard {
        virtual_device_: labeled_device(key, label),
        ..eth_card(mac, key)
    }
}

fn labeled_device_array_prop(specs: &[LDev]) -> DynamicProperty {
    let devices: Vec<Box<dyn VirtualDeviceTrait>> = specs
        .iter()
        .enumerate()
        .map(|(i, spec)| {
            let key = 1000 + i as i32;
            let dev: Box<dyn VirtualDeviceTrait> = match spec {
                LDev::E1000(mac, label) => Box::new(VirtualE1000 {
                    virtual_ethernet_card_: labeled_eth_card(mac, label, key),
                }),
                LDev::Vmxnet3(mac, label) => Box::new(VirtualVmxnet3 {
                    virtual_vmxnet_: VirtualVmxnet {
                        virtual_ethernet_card_: labeled_eth_card(mac, label, key),
                    },
                    uptv_2_enabled: None,
                    strict_latency_config: None,
                }),
                LDev::Cdrom(label) => Box::new(VirtualCdrom {
                    virtual_device_: labeled_device(key, label),
                }),
            };
            dev
        })
        .collect();
    DynamicProperty {
        name: "config.hardware.device".to_string(),
        val: VimAny::Value(ValueElements::ArrayOfVirtualDevice(devices)),
    }
}

fn write_fixture(dir: &PathBuf, name: &str, result: &RetrieveResult) {
    let path = dir.join(name);
    let json = miniserde::json::to_string(result);
    fs::write(path, json).expect("write fixture");
}

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixtures = root.join("../fixtures");
    fs::create_dir_all(&fixtures).expect("fixtures dir");

    // pc01
    write_fixture(
        &fixtures,
        "pc01_a.json",
        &RetrieveResult {
            token: None,
            objects: vec![ObjectContent {
                obj: vm_mor("vm-101"),
                prop_set: Some(vec![
                    power_prop(VirtualMachinePowerStateEnum::PoweredOn),
                    guest_host_prop("web01.eval"),
                ]),
                missing_set: None,
            }],
        },
    );
    write_fixture(
        &fixtures,
        "pc01_b.json",
        &RetrieveResult {
            token: None,
            objects: vec![
                ObjectContent {
                    obj: vm_mor("vm-101"),
                    prop_set: Some(vec![
                        power_prop(VirtualMachinePowerStateEnum::PoweredOff),
                    ]),
                    missing_set: None,
                },
                ObjectContent {
                    obj: vm_mor("vm-102"),
                    prop_set: Some(vec![
                        power_prop(VirtualMachinePowerStateEnum::PoweredOn),
                        guest_host_prop("db02.eval"),
                    ]),
                    missing_set: None,
                },
            ],
        },
    );

    // pc02
    write_fixture(
        &fixtures,
        "pc02_a.json",
        &RetrieveResult {
            token: None,
            objects: vec![ObjectContent {
                obj: vm_mor("vm-201"),
                prop_set: Some(vec![
                    name_prop("app-tier-7"),
                    num_cpu_prop(8),
                ]),
                missing_set: None,
            }],
        },
    );
    write_fixture(
        &fixtures,
        "pc02_b.json",
        &RetrieveResult {
            token: None,
            objects: vec![
                ObjectContent {
                    obj: vm_mor("vm-201"),
                    prop_set: Some(vec![name_prop("cache-01"), num_cpu_prop(4)]),
                    missing_set: None,
                },
                ObjectContent {
                    obj: vm_mor("vm-202"),
                    prop_set: Some(vec![name_prop("cache-02"), num_cpu_prop(16)]),
                    missing_set: None,
                },
            ],
        },
    );

    // pc03
    write_fixture(
        &fixtures,
        "pc03_a.json",
        &RetrieveResult {
            token: None,
            objects: vec![ObjectContent {
                obj: vm_mor("vm-301"),
                // Cdrom (no MAC) before the NIC: naive "read mac off any device" fails.
                prop_set: Some(vec![device_array_prop(&[
                    Dev::Cdrom,
                    Dev::E1000("00:aa:11:22:33:44"),
                ])]),
                missing_set: None,
            }],
        },
    );
    write_fixture(
        &fixtures,
        "pc03_b.json",
        &RetrieveResult {
            token: None,
            objects: vec![
                ObjectContent {
                    obj: vm_mor("vm-301"),
                    prop_set: Some(vec![device_array_prop(&[
                        Dev::E1000("00:bb:22:33:44:55"),
                        Dev::Cdrom,
                    ])]),
                    missing_set: None,
                },
                ObjectContent {
                    obj: vm_mor("vm-302"),
                    // Mixed NIC subtypes (Vmxnet3 + E1000) with a Cdrom interspersed:
                    // matching a single concrete type or skipping the downcast both fail.
                    prop_set: Some(vec![device_array_prop(&[
                        Dev::Cdrom,
                        Dev::Vmxnet3("00:cc:33:44:55:66"),
                        Dev::E1000("00:dd:44:55:66:77"),
                    ])]),
                    missing_set: None,
                },
            ],
        },
    );

    // pc04
    write_fixture(
        &fixtures,
        "pc04_a.json",
        &RetrieveResult {
            token: None,
            objects: vec![ObjectContent {
                obj: host_mor("host-401"),
                prop_set: Some(vec![
                    overall_status_prop(ManagedEntityStatusEnum::Green),
                    product_full_name_prop("VMware ESXi 8.0.2 build-22380479"),
                ]),
                missing_set: None,
            }],
        },
    );
    write_fixture(
        &fixtures,
        "pc04_b.json",
        &RetrieveResult {
            token: None,
            objects: vec![
                ObjectContent {
                    obj: host_mor("host-401"),
                    prop_set: Some(vec![
                        overall_status_prop(ManagedEntityStatusEnum::Yellow),
                        product_full_name_prop("VMware ESXi 8.0.2 build-22380479"),
                    ]),
                    missing_set: None,
                },
                ObjectContent {
                    obj: host_mor("host-402"),
                    prop_set: Some(vec![
                        overall_status_prop(ManagedEntityStatusEnum::Green),
                        product_full_name_prop("VMware ESXi 7.0.3 build-19193900"),
                    ]),
                    missing_set: None,
                },
            ],
        },
    );

    // pc05 — (label, mac) pairs: label is a base VirtualDevice.deviceInfo field
    // reached via the supertrait; mac is on the ethernet subtype.
    write_fixture(
        &fixtures,
        "pc05_a.json",
        &RetrieveResult {
            token: None,
            objects: vec![ObjectContent {
                obj: vm_mor("vm-501"),
                prop_set: Some(vec![labeled_device_array_prop(&[
                    LDev::Cdrom("CD/DVD drive 1"),
                    LDev::E1000("00:aa:11:22:33:44", "Network adapter 1"),
                ])]),
                missing_set: None,
            }],
        },
    );
    write_fixture(
        &fixtures,
        "pc05_b.json",
        &RetrieveResult {
            token: None,
            objects: vec![
                ObjectContent {
                    obj: vm_mor("vm-501"),
                    prop_set: Some(vec![labeled_device_array_prop(&[
                        LDev::E1000("00:bb:22:33:44:55", "Network adapter 1"),
                        LDev::Cdrom("CD/DVD drive 1"),
                    ])]),
                    missing_set: None,
                },
                ObjectContent {
                    obj: vm_mor("vm-502"),
                    prop_set: Some(vec![labeled_device_array_prop(&[
                        LDev::Cdrom("CD/DVD drive 1"),
                        LDev::Vmxnet3("00:cc:33:44:55:66", "Network adapter 1"),
                        LDev::E1000("00:dd:44:55:66:77", "Network adapter 2"),
                    ])]),
                    missing_set: None,
                },
            ],
        },
    );

    eprintln!("fixtures written to {}", fixtures.display());
}
