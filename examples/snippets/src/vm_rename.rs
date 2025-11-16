//! # Virtual Machine Rename Example
//!
//! This example demonstrates how to rename a virtual machine using the vSphere API.
//!
//! The rename operation is performed through the ManagedEntity.Rename_Task method,
//! which is inherited by VirtualMachine. This is an asynchronous operation that
//! returns a Task object which can be monitored for completion.
//!
//! In this example:
//! 1. We connect to a vSphere server using credentials from environment variables
//! 2. We search for a VM by its inventory path using SearchIndex
//! 3. We create a VirtualMachine managed object proxy
//! 4. We call rename_task to rename the VM
//! 5. We wait for the task to complete
//!
//! **Important Notes:**
//! - Renaming a VM changes only the display name in the vCenter inventory
//! - It does NOT change the VM's hostname, guest OS computer name, or file names
//! - The VM can be powered on or off during the rename operation
//! - You need VirtualMachine.Config.Rename privilege to perform this operation
//!
//! **Environment Variables Required:**
//! - VIM_SERVER: vCenter/ESXi server URL
//! - VIM_USERNAME: Username for authentication
//! - VIM_PASSWORD: Password for authentication
//! - VM_INVENTORY_PATH: Full inventory path to the VM (e.g., "/Datacenter/vm/MyVM")
//! - NEW_VM_NAME: The new name for the virtual machine

use anyhow::{Context, Result};
use log::{debug, info};
use std::env;
use std::sync::Arc;
use utils::connect;
use vim_rs::core::Client;
use vim_rs::mo::{SearchIndex, Task, VirtualMachine};
use vim_rs::types::enums::TaskInfoStateEnum;
use vim_rs::types::structs::ManagedObjectReference;

/// Waits for a vSphere task to complete and returns the result.
///
/// This function polls the task state periodically until it reaches a terminal state
/// (Success or Error). It logs progress updates during execution.
///
/// # Arguments
/// * `client` - The vSphere client connection
/// * `task_ref` - Reference to the task to monitor
///
/// # Returns
/// * `Ok(())` if the task completes successfully
/// * `Err` if the task fails or encounters an error
async fn wait_for_task(client: Arc<Client>, task_ref: &ManagedObjectReference) -> Result<()> {
    let task = Task::new(client, &task_ref.value);

    loop {
        let task_info = task.info().await?;

        match task_info.state {
            TaskInfoStateEnum::Success => {
                debug!("✅ Task completed successfully");
                return Ok(());
            }
            TaskInfoStateEnum::Error => {
                let error_msg = task_info
                    .error
                    .map(|e| format!("{:?}", e))
                    .unwrap_or_else(|| "Unknown error".to_string());
                return Err(anyhow::anyhow!("Task failed: {}", error_msg));
            }
            TaskInfoStateEnum::Running => {
                debug!("Task in progress... ({}%)", task_info.progress.unwrap_or(0));
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
            TaskInfoStateEnum::Queued => {
                debug!("Task queued...");
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
            TaskInfoStateEnum::Other_(state) => {
                debug!("Task in unknown state: {}", state);
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Connect to vCenter
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;
    info!("Connected to {}", client.service_content().about.full_name);

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
    wait_for_task(client.clone(), &task_ref).await?;
    info!("VM successfully renamed to: {}", new_name);

    // Verify the new name
    let updated_name = vm.name().await?;
    info!("Verified new VM name: {}", updated_name);

    Ok(())
}
