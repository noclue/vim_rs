//! # VM network addresses (`vim_retrievable!`)
//!
//! Walks all VMs under the root folder and prints NIC-oriented network data. It prefers VMware Tools
//! `guest.net` when present, and falls back to hardware devices using
//! [`CastInto`](vim_rs::types::convert::CastInto) to [`VirtualEthernetCardTrait`](vim_rs::types::traits::VirtualEthernetCardTrait).
//!
//! ## Typical uses
//!
//! IPAM-style inventory, Tools coverage checks, and network troubleshooting.

use log::info;
use snippets::connect;
use vim_rs::core::pc_retrieve::ObjectRetriever;
use vim_rs::types::convert::CastInto;
use vim_rs::types::traits::VirtualEthernetCardTrait;
use vim_rs::vim_retrievable;

vim_retrievable!(
    struct Vm: VirtualMachine {
        name = "name",
        devices = "config.hardware.device",
        guest_network = "guest.net",
    }
);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
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
