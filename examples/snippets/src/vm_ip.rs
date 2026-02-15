// ────────────────────────────────────────────────────────────────
//  examples/get_ip_from_inventory_path.rs
// ────────────────────────────────────────────────────────────────

use std::env;

use anyhow::{Context, Result};
use log::{error, info};
use utils::connect;

use vim_macros::vim_retrievable;
use vim_rs::{
    mo::SearchIndex, types::structs::ManagedObjectReference,
};

// Define a tiny struct that mirrors the `VirtualMachine` object but
// only pulls back the `guest.ipAddress` property.
vim_retrievable! {
    struct VmIpInfo: VirtualMachine {
        ip_address = "guest.ip_address",
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let client = connect("vm_ip_example", env!("CARGO_PKG_VERSION")).await?;
    println!("Connected to {}", client.service_content().about.full_name);
    info!("✅  Logged in");

    // ----- 2️⃣  Resolve inventory path ---------------------------------------
    // Example path – replace with the one you want to query.
    let inv_path = env::var("VM_INVENTORY_PATH")
        .context("VM_INVENTORY_PATH env var not set (e.g., '/Datacenter/vm/Prod/YourVM')")?;

    // The SearchIndex service is used for *FindByInventoryPath*.
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

    // ----- 3️⃣  Property collector via vim_retrievable! ---------------------
    // `ObjectRetriever` is a thin wrapper around the PropertyCollector
    // that knows how to turn our `VmIpInfo` struct into a filter spec.
    let retriever = vim_rs::core::pc_retrieve::ObjectRetriever::new(client.clone())?;

    // Retrieve *one* VM – the macro expands to a tiny property filter that asks
    // for `guest.ipAddress`.  The result is automatically deserialized into
    // `VmIpInfo`.
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
