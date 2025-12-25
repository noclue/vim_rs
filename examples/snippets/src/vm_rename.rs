//! # VM Rename (ManagedEntity::Rename_Task)
//!
//! Renames a VM’s **inventory display name** by calling `ManagedEntity::Rename_Task` (via
//! `VirtualMachine::rename_task`) and waiting for the returned `Task` to complete using
//! `TaskTracker`.
//!
//! ## Required environment variables
//! - `VIM_SERVER`: vCenter/ESXi URL (e.g. `https://vc.example.com`)
//! - `VIM_USERNAME`: username
//! - `VIM_PASSWORD`: password
//! - `VM_INVENTORY_PATH`: full inventory path (e.g. `"/Datacenter/vm/MyVM"`)
//! - `NEW_VM_NAME`: new inventory name for the VM
//!
//! ## Notes
//! - This **only** changes the vCenter inventory name (UI display name).
//! - It does **not** change guest OS hostname, DNS, or datastore file/folder names.
//! - Requires `VirtualMachine.Config.Rename` privilege.

use anyhow::{Context, Result};
use log::info;
use tokio::time::sleep;
use std::env;
use std::time::Duration;
use utils::connect;
use vim_rs::mo::{SearchIndex, VirtualMachine};
use vim_rs::core::tasks::tracker::TaskTracker;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Connect to vCenter
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;
    info!("Connected to {}", client.service_content().about.full_name);

    let task_tracker = TaskTracker::new(client.clone());

    // Get VM inventory path and new name from environment
    let vm_path = env::var("VM_INVENTORY_PATH")
        .context("VM_INVENTORY_PATH env var not set (e.g., '/Datacenter/vm/MyVM')")?;
    let new_name = env::var("NEW_VM_NAME").context("NEW_VM_NAME env var not set")?;

    // Find the VM using SearchIndex
    let search_index = SearchIndex::new(
        client.clone(),
        &client
            .service_content()
            .search_index
            .as_ref()
            .context("SearchIndex not available")?
            .value,
    );

    info!("Searching for VM at path: {}", vm_path);
    let vm_moref = search_index
        .find_by_inventory_path(&vm_path)
        .await?
        .context(format!(
            "VM not found at specified inventory path: {}",
            vm_path
        ))?;

    info!("Found VM: {} ({})", vm_path, vm_moref.value);

    // Create VirtualMachine managed object proxy
    let vm = VirtualMachine::new(client.clone(), &vm_moref.value);

    // Get current VM name to display
    let current_name = vm.name().await?;
    info!("Current VM name: {}", current_name);

    // Perform the rename operation
    info!("Renaming VM from '{}' to '{}'", current_name, new_name);
    let task_ref = vm.rename_task(&new_name).await?;
    info!("Rename task created: {}", task_ref.value);

    // Wait for the task to complete
    task_tracker.wait::<()>(task_ref).await?;
    info!("VM successfully renamed to: {}", new_name);

    // Verify the new name
    let updated_name = vm.name().await?;
    info!("Verified new VM name: {}", updated_name);

    drop(task_tracker);
    drop(client);
    sleep(Duration::from_secs(1)).await;
    Ok(())
}
