//! # Resolve a VM by inventory path and read guest IP (`vim_retrievable!`)
//!
//! 1. Connect with the shared [`connect`](snippets::connect) helper.
//! 2. Resolve `VM_INVENTORY_PATH` with [`SearchIndex::find_by_inventory_path`](vim_rs::mo::SearchIndex::find_by_inventory_path).
//! 3. Fetch `guest.ip_address` through `vim_retrievable!` and [`ObjectRetriever`](vim_rs::core::pc_retrieve::ObjectRetriever).
//!
//! ## Environment
//!
//! - `VM_INVENTORY_PATH` — full path (e.g. `/Datacenter/vm/Folder/MyVM`)
//!
//! Guest IP requires VMware Tools (or equivalent) reporting the address; absence is not always an error.

use std::env;

use anyhow::{Context, Result};
use log::{error, info};
use snippets::connect;
use vim_rs::core::pc_retrieve::ObjectRetriever;
use vim_rs::mo::SearchIndex;
use vim_rs::types::structs::ManagedObjectReference;
use vim_rs::vim_retrievable;

vim_retrievable! {
    struct VmIpInfo: VirtualMachine {
        ip_address = "guest.ip_address",
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;
    println!("Connected to {}", client.service_content().about.full_name);
    info!("Logged in");

    let inv_path = env::var("VM_INVENTORY_PATH")
        .context("VM_INVENTORY_PATH env var not set (e.g., '/Datacenter/vm/Prod/YourVM')")?;

    let search_index = SearchIndex::new(
        client.clone(),
        &client
            .service_content()
            .search_index
            .as_ref()
            .unwrap()
            .value,
    );
    let vm_moref_opt: Option<ManagedObjectReference> =
        match search_index.find_by_inventory_path(&inv_path).await? {
            Some(moref) => Some(moref),
            None => {
                error!("No object found at inventory path '{}'", inv_path);
                return Ok(());
            }
        };

    let vm_moref = vm_moref_opt.unwrap();
    info!("Found VM: {}", vm_moref.value);

    let retriever = ObjectRetriever::new(client.clone())?;

    let vm_ip = retriever
        .retrieve_objects_from_list::<VmIpInfo>(&[vm_moref])
        .await?;

    if let Some(vm_info) = vm_ip.first() {
        match &vm_info.ip_address {
            Some(ip) => println!("IP address of '{}' is {}", inv_path, ip),
            None => println!("Could not read guest IP. Maybe VMware Tools is not running?"),
        }
    } else {
        error!("Could not retrieve VM properties. The VM might have been deleted.");
    }

    Ok(())
}
