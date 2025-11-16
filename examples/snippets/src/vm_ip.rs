// ────────────────────────────────────────────────────────────────
//  examples/get_ip_from_inventory_path.rs
// ────────────────────────────────────────────────────────────────

use anyhow::Result;
use log::{error, info};
use std::env;

use vim_macros::vim_retrievable;
use vim_rs::{
    core::client::ClientBuilder, mo::SearchIndex, types::structs::ManagedObjectReference,
};

// Define a tiny struct that mirrors the `VirtualMachine` object but
// only pulls back the `guest.ipAddress` property.
vim_retrievable! {
    struct VmIpInfo: VirtualMachine {
        ip_address = "guest.ip_address",
    }
}

/// Helper that reads vCenter connection details from env vars
/// (or you can change this to read from a CLI argument / config file).
fn get_vsphere_cfg() -> anyhow::Result<(String, String, String)> {
    let url = env::var("VSPHERE_URL").map_err(|_| anyhow::anyhow!("Missing VSPHERE_URL"))?;
    let user = env::var("VSPHERE_USER").map_err(|_| anyhow::anyhow!("Missing VSPHERE_USER"))?;
    let password =
        env::var("VSPHERE_PASSWORD").map_err(|_| anyhow::anyhow!("Missing VSPHERE_PASSWORD"))?;
    Ok((url, user, password))
}

#[tokio::main]
async fn main() -> Result<()> {
    // ----- 1️⃣  Connection ----------------------------------------------------
    env_logger::init();

    let (vsphere_url, username, password) = get_vsphere_cfg()?;

    info!("Connecting to vCenter: {}", vsphere_url);
    let client = ClientBuilder::new(vsphere_url.as_str())
        .insecure(true)
        .basic_authn(username.as_str(), password.as_str())
        //.app_details(app_name, app_version)
        .build()
        .await?;
    info!("✅  Logged in as {}", username);

    // ----- 2️⃣  Resolve inventory path ---------------------------------------
    // Example path – replace with the one you want to query.
    let inv_path = "/Datacenter/vm/Prod/YourVM";

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
        match search_index.find_by_inventory_path(inv_path).await? {
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

    // ----- 4️⃣  Display the IP address --------------------------------------
    match &vm_ip[0].ip_address {
        Some(ip) => println!("IP address of '{}' is {}", inv_path, ip),
        None => println!("Could not read guest IP. Maybe VMware Tools is not running?"),
    }

    Ok(())
}
