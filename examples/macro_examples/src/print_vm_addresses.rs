//! # Print VM Network Addresses Example
//!
//! This example demonstrates how to retrieve and display network information for virtual
//! machines, including MAC addresses, IP addresses, and network associations.
//!
//! ## Key Concepts
//!
//! **Guest Network Info vs Hardware Devices**: VMs have two sources of network information:
//! - `guest.net`: Network info reported by VMware Tools (IPs, MACs, network names)
//! - `config.hardware.device`: Virtual hardware configuration (NICs and MACs)
//!
//! The guest info is more complete (includes IPs) but requires VMware Tools to be running.
//! Hardware config is always available but only shows MACs and device keys.
//!
//! **Polymorphic Device Handling**: Virtual devices use trait objects (`Box<dyn VirtualDeviceTrait>`).
//! This example shows the CORRECT way to filter for ethernet cards using `CastInto` trait
//! to cast to `VirtualEthernetCardTrait`, which works for ALL NIC types (E1000, Vmxnet3, etc.).
//!
//! ## How It Works
//!
//! 1. Retrieve all VMs with `vim_retrievable!` fetching guest network and device properties
//! 2. First, try to display guest network info (preferred, has IPs)
//! 3. If no guest info, fall back to hardware device info (MACs only)
//! 4. Report VMs that have no network devices at all
//!
//! ## Use Cases
//!
//! - Network inventory and IP address management (IPAM)
//! - Validating VMware Tools installation status
//! - Auditing VM network configurations
//! - Troubleshooting network connectivity issues
//!
//! ## Output Format
//!
//! For each VM, displays:
//! - VM name
//! - MAC address(es)
//! - Virtual device key (hardware config ID)
//! - Connected network name
//! - IP address(es) assigned (if VMware Tools is running)

use log::info;
use utils::connect;
use vim_macros::vim_retrievable;
use vim_rs::core::pc_retrieve::ObjectRetriever;
use vim_rs::types::convert::CastInto;
use vim_rs::types::traits::VirtualEthernetCardTrait;

vim_retrievable!(
    struct Vm: VirtualMachine {
        name = "name",
        devices = "config.hardware.device",
        guest_network = "guest.net",
    }
);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;
    let retriever = ObjectRetriever::new(client.clone())?;

    let dash = "-".to_string();

    let objs: Vec<Vm> = retriever
        .retrieve_objects_from_container(&client.service_content().root_folder)
        .await?;
    for vm in objs {
        if let Some(guest_networks) = vm.guest_network {
            let guest = !guest_networks.is_empty();
            for guest_network in guest_networks {
                let mac_addr = guest_network.mac_address.unwrap_or("-".to_string());
                let network = guest_network.network.unwrap_or("-".to_string());
                let device = guest_network.device_config_id;
                let ip_addrs = guest_network
                    .ip_address
                    .unwrap_or(vec!["-".to_string()])
                    .join(", ");
                info!(
                    "VM: {:25}, MAC: {:18}, Device: {:6}, Network: {:12}, IPs: {}",
                    vm.name, mac_addr, device, network, ip_addrs
                );
            }
            if guest {
                continue;
            }
        }

        if let Some(devices) = vm.devices {
            let mut has_eth = false;
            for device in devices {
                let Some(eth): Option<&dyn VirtualEthernetCardTrait> = device.as_ref().into_ref()
                else {
                    continue;
                };
                let mac_addr = eth.mac_address.as_ref().unwrap_or(&dash);
                info!("VM: {:25}, MAC: {:18}, Device: {:6}", vm.name, mac_addr, eth.key);
                has_eth = true;
            }
            if has_eth {
                continue;
            }
        }

        info!(
            "VM: {:25} has no ethernet devices or guest networks",
            vm.name
        );
    }
    Ok(())
}
