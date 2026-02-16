//! Toggle Wake-on-LAN for Virtual Machine Network Adapters
//!
//! Demonstrates how to modify virtual machine hardware configuration by toggling the
//! Wake-on-LAN (WOL) setting on all ethernet network adapters. This example showcases
//! advanced device reconfiguration patterns including type-based filtering, polymorphic
//! device handling, and asynchronous task monitoring.
//!
//! # What is Wake-on-LAN?
//!
//! Wake-on-LAN is a hardware feature that allows a computer to be awakened from a
//! low-power state (suspended or powered off) by receiving a special network packet
//! called a "magic packet". For virtual machines, this setting is configured at the
//! virtual ethernet adapter level and allows remote power management of VMs.
//!
//! # Device Reconfiguration Workflow
//!
//! This example demonstrates the complete pattern for modifying existing virtual devices:
//!
//! 1. **Locate the VM** - Use `SearchIndex::find_by_inventory_path()` to find VM by path
//! 2. **Retrieve device list** - Use PropertyCollector with `vim_retrievable!` macro to fetch
//!    `config.hardware.device` property efficiently
//! 3. **Filter devices by type** - Use `StructType` enum to identify ethernet adapters without
//!    trial downcasting
//! 4. **Downcast to concrete types** - Convert `Box<dyn VirtualDeviceTrait>` to concrete
//!    ethernet card types (VirtualE1000, VirtualVmxnet3, etc.)
//! 5. **Modify device properties** - Toggle the `wake_on_lan_enabled` field
//! 6. **Build configuration spec** - Wrap modified devices in `VirtualDeviceConfigSpec`
//!    with operation `Edit`
//! 7. **Submit reconfiguration** - Call `VirtualMachine::reconfig_vm_task()` to apply changes
//! 8. **Monitor task completion** - Use `TaskTracker` to wait for task completion
//!
//! # Polymorphic Virtual Device Handling
//!
//! Virtual devices in vSphere are polymorphic - the API returns them as trait objects
//! (`Box<dyn VirtualDeviceTrait>`). This example demonstrates the recommended pattern
//! for handling polymorphic types:
//!
//! - **Type identification**: Use `device.data_type()` to get the `StructType` enum variant
//! - **Type filtering**: Check `obj_type.child_of(StructType::VirtualDevice)` for inheritance
//! - **Safe downcasting**: Use `as_any_box().downcast::<ConcreteType>()` to convert to concrete types
//!
//! ## Supported Ethernet Adapter Types
//!
//! The example handles all 9 virtual ethernet card types:
//! - `VirtualEthernetCard` (base type)
//! - `VirtualE1000` (Intel E1000)
//! - `VirtualE1000E` (Intel E1000e)
//! - `VirtualPcNet32` (AMD PCNet32)
//! - `VirtualVmxnet` (VMware Vmxnet)
//! - `VirtualVmxnet2` (VMware Vmxnet2)
//! - `VirtualVmxnet3` (VMware Vmxnet3 - most common)
//! - `VirtualVmxnet3Vrdma` (Vmxnet3 with RDMA)
//! - `VirtualSriovEthernetCard` (SR-IOV passthrough)
//!
//! # Example Output
//!
//! ```text
//! [INFO] Connected to VMware vCenter Server 8.0.3 build-24305161
//! [INFO] Searching for VM at path: /Home/vm/production/ubuntu_test__
//! [INFO] Found VM: /Home/vm/production/ubuntu_test__ (vm-7009)
//! [INFO] VM name: ubuntu_test__
//! [INFO] Processing 14 devices
//! [INFO] Wake-on-LAN will be disabled for the VirtualVmxnet3 adapter with key 4000
//! [INFO] Found 1 ethernet adapter(s), submitting reconfiguration
//! [INFO] Calling reconfig_vm_task...
//! [INFO] Reconfigure task created: task-247019
//! [INFO] Task in progress... (0%)
//! [INFO] ✅ Task completed successfully
//! [INFO] Successfully toggled Wake-on-LAN for 1 NIC(s)
//! ```
//!
//! # Environment Variables
//!
//! The following environment variables must be set (typically loaded from `.env` file):
//!
//! - `VIM_SERVER` - vCenter Server hostname or IP (e.g., "vcenter.example.com")
//! - `VIM_USERNAME` - vSphere username (e.g., "administrator@vsphere.local")
//! - `VIM_PASSWORD` - vSphere password
//! - `VM_INVENTORY_PATH` - Full inventory path to VM (e.g., "/Datacenter/vm/MyVM")
//!
//! # Required Permissions
//!
//! - `VirtualMachine.Config.Settings` - Required to modify VM configuration
//!
//! # Important Behavior Notes
//!
//! - **VM Power State**: This operation works regardless of VM power state (on, off, suspended)
//! - **Immediate Effect**: Configuration changes are applied immediately
//! - **WOL Functionality**: Wake-on-LAN only functions when VM is suspended or powered off,
//!   not when the VM is in "guest powered off" state
//! - **Guest OS**: Changes may require guest OS network driver restart to take full effect
//! - **Idempotent**: Running the example multiple times toggles WOL on/off each time
//!
//! # Key API Patterns Demonstrated
//!
//! - **SearchIndex** for locating managed objects by inventory path
//! - **PropertyCollector** via `vim_retrievable!` macro for efficient bulk property retrieval
//! - **ObjectRetriever** pattern with `retrieve_objects_from_list()`
//! - **Type-safe downcasting** using `StructType` enum and `downcast::<T>()`
//! - **Device reconfiguration** with `VirtualDeviceConfigSpec` and operation enum
//! - **Asynchronous task monitoring** with `TaskTracker`
//!
//! # Use Cases
//!
//! - **Compliance automation** - Enforce WOL policies across VM fleet
//! - **Power management** - Enable WOL for VMs that need remote wake capability
//! - **Configuration drift remediation** - Standardize network adapter settings
//! - **Learning resource** - Study advanced vim_rs patterns for device modification
//!
//! # See Also
//!
//! - `vm_rename.rs` - Example of simple VM reconfiguration without device changes
//! - `retrieve_host_info.rs` - Example of PropertyCollector usage patterns

use anyhow::{Context, Result};
use log::info;
use tokio::time::sleep;
use vim_rs::types::convert::CastInto as _;
use std::env;
use std::time::Duration;
use utils::connect;
use vim_macros::vim_retrievable;
use vim_rs::core::tasks::TaskTracker;
use vim_rs::mo::{SearchIndex, VirtualMachine};
use vim_rs::types::enums::VirtualDeviceConfigSpecOperationEnum;
use vim_rs::types::structs::{
    VirtualDeviceConfigSpec, VirtualMachineConfigSpec,
};
use vim_rs::types::traits::{VirtualDeviceTrait, VirtualEthernetCardTrait};

vim_retrievable!(
    struct Vm: VirtualMachine {
        name = "name",
        devices = "config.hardware.device",
    }
);



async fn toggle_wol() -> Result<()> {
    // Connect to vCenter
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;
    info!("Connected to {}", client.service_content().about.full_name);

    let task_tracker = TaskTracker::new(client.clone());

    // Get VM inventory path from environment
    let vm_path = env::var("VM_INVENTORY_PATH")
        .context("VM_INVENTORY_PATH env var not set (e.g., '/Datacenter/vm/MyVM')")?;

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
        .context("VM not found at specified inventory path")?;

    info!("Found VM: {} ({})", vm_path, vm_moref.value);

    // Retrieve VM devices using PropertyCollector
    let retriever = vim_rs::core::pc_retrieve::ObjectRetriever::new(client.clone())?;
    let vms: Vec<Vm> = retriever
        .retrieve_objects_from_list(&[vm_moref.clone()])
        .await?;

    let vm = vms
        .into_iter()
        .next()
        .context("VM not found in retrieval results")?;

    info!("VM name: {}", vm.name);

    // Move devices out of the VM struct
    let mut devices = vm.devices.context("No devices found on VM")?;

    info!("Processing {} devices", devices.len());

    // Find all ethernet cards and toggle wake_on_lan_enabled
    let mut device_changes: Vec<VirtualDeviceConfigSpec> = Vec::new();
    let mut nic_count = 0;

    // Drain the devices vector to take ownership of each device
    for device in devices.drain(..) {
        let Ok(mut eth) : Result<Box<dyn VirtualEthernetCardTrait>,_> = device.into_box() else{
            continue;
        };

        let current_wol = eth.wake_on_lan_enabled.unwrap_or(false);
        eth.wake_on_lan_enabled = Some(!current_wol);

        let device: Box<dyn VirtualDeviceTrait> = eth.into_box().expect("trait cast should work");

        let type_: &'static str = device.data_type().into();
        if current_wol {
            info!("Wake-on-LAN will be disabled for the {} adapter with key {}", type_, device.key);
        } else {
            info!("Wake-on-LAN will be enabled for the {} adapter with key {}", type_, device.key);
        }

        device_changes.push(VirtualDeviceConfigSpec {
            operation: Some(VirtualDeviceConfigSpecOperationEnum::Edit),
            device: device,
            file_operation: None,
            profile: None,
            backing: None,
            filter_spec: None,
            change_mode: None,
        });
        nic_count += 1;
    }

    if nic_count == 0 {
        info!("No ethernet network adapters found on VM");
        return Ok(());
    }

    info!(
        "Found {} ethernet adapter(s), submitting reconfiguration",
        nic_count
    );

    // Create VirtualMachineConfigSpec with device changes
    // We only need to set device_change - all other fields default to None
    let config_spec = VirtualMachineConfigSpec {
        device_change: Some(
            device_changes
                .into_iter()
                .map(|spec| {
                    Box::new(spec) as Box<dyn vim_rs::types::traits::VirtualDeviceConfigSpecTrait>
                })
                .collect(),
        ),
        change_version: None,
        name: None,
        version: None,
        create_date: None,
        uuid: None,
        instance_uuid: None,
        npiv_node_world_wide_name: None,
        npiv_port_world_wide_name: None,
        npiv_world_wide_name_type: None,
        npiv_desired_node_wwns: None,
        npiv_desired_port_wwns: None,
        npiv_temporary_disabled: None,
        npiv_on_non_rdm_disks: None,
        npiv_world_wide_name_op: None,
        location_id: None,
        guest_id: None,
        alternate_guest_name: None,
        annotation: None,
        files: None,
        tools: None,
        flags: None,
        console_preferences: None,
        power_op_info: None,
        reboot_power_off: None,
        num_cp_us: None,
        vcpu_config: None,
        num_cores_per_socket: None,
        memory_mb: None,
        memory_hot_add_enabled: None,
        cpu_hot_add_enabled: None,
        cpu_hot_remove_enabled: None,
        virtual_ich_7_m_present: None,
        virtual_smc_present: None,
        cpu_allocation: None,
        memory_allocation: None,
        latency_sensitivity: None,
        cpu_affinity: None,
        memory_affinity: None,
        network_shaper: None,
        cpu_feature_mask: None,
        extra_config: None,
        swap_placement: None,
        boot_options: None,
        v_app_config: None,
        ft_info: None,
        rep_config: None,
        v_app_config_removed: None,
        v_asserts_enabled: None,
        change_tracking_enabled: None,
        firmware: None,
        max_mks_connections: None,
        guest_auto_lock_enabled: None,
        managed_by: None,
        memory_reservation_locked_to_max: None,
        nested_hv_enabled: None,
        v_pmc_enabled: None,
        scheduled_hardware_upgrade_info: None,
        vm_profile: None,
        message_bus_tunnel_enabled: None,
        crypto: None,
        migrate_encryption: None,
        sgx_info: None,
        ft_encryption_mode: None,
        guest_monitoring_mode_info: None,
        sev_enabled: None,
        virtual_numa: None,
        motherboard_layout: None,
        pmem_failover_enabled: None,
        vmx_stats_collection_enabled: None,
        vm_op_notification_to_app_enabled: None,
        vm_op_notification_timeout: None,
        device_swap: None,
        simultaneous_threads: None,
        pmem: None,
        device_groups: None,
        fixed_passthru_hot_plug_enabled: None,
        metro_ft_enabled: None,
        metro_ft_host_group: None,
        tdx_enabled: None,
        sev_snp_enabled: None,
    };

    // Reconfigure the VM
    let vm = VirtualMachine::new(client.clone(), &vm_moref.value);
    info!("Calling reconfig_vm_task...");
    let task_ref = vm.reconfig_vm_task(&config_spec).await?;
    info!("Reconfigure task created: {}", task_ref.value);

    // Wait for task completion
    task_tracker.wait::<()>(task_ref).await?;
    info!("✅ Task completed successfully");
    info!("Successfully toggled Wake-on-LAN for {} NIC(s)", nic_count);

    Ok(())
}


#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    toggle_wol().await?;
    // Yield to run async drop cleanup
    sleep(Duration::from_millis(10)).await;

    Ok(())
}