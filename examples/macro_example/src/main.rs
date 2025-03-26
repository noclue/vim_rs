use vim_macros::vim_updatable;
//use vim_rs::types::structs::ObjectUpdate;
use anyhow::Result;
use vim_rs::types::enums::{ManagedEntityStatusEnum, MoTypesEnum, VirtualMachinePowerStateEnum};
use vim_rs::types::structs::{ManagedObjectReference, VirtualMachineStorageSummary};

vim_updatable!(
    struct VM: VirtualMachine {
        name = "name",
        os = "summary.guest.guest_full_name",
        storage = "summary.storage",
        host_cpu = "summary.quick_stats.overall_cpu_usage",
        host_memory = "summary.quick_stats.host_memory_usage",
        status = "summary.overall_status",
        power_state = "runtime.power_state",
        devices = "config.hardware.device",
        ft_info = "config.ft_info",
    }
);

fn main() -> Result<()> {
    //let r#type: String = Into::<&str>::into(vim_rs::types::enums::MoTypesEnum::VirtualMachine).to_string();
    // Create a property spec for VMs
    let prop_spec = VM::prop_spec();
    println!("Generated property spec: {:?}", prop_spec);

    let _test_vm = VM{
        id: ManagedObjectReference{
            r#type: MoTypesEnum::VirtualMachine,
            value: "vm-42".to_string(),
        },
        name: "My VM".to_string(),
        os: Some("Ubuntu 64-bit".to_string()),
        storage: Some(VirtualMachineStorageSummary{
            committed: 1024,
            uncommitted: 512,
            unshared: 256,
            timestamp: "".to_string(),
        }),
        host_cpu: Some(64000),
        host_memory: Some(1024),
        status: ManagedEntityStatusEnum::Gray,
        power_state: VirtualMachinePowerStateEnum::PoweredOff,
        devices: None,
        ft_info: None,
    };

    // Example of how we might use this in a real application
    // This would normally come from the vSphere API
    //let mock_update = create_mock_update();

    // Convert the update to our VirtualMachine struct
    // let vm = VM::try_from(&mock_update)?;
    // println!("VM ID: {}", vm.id());
    // println!("VM Name: {:?}", vm.name);
    // println!("VM OS: {:?}", vm.os);

    Ok(())
}

// Helper to create a mock update for demonstration
// fn create_mock_update() -> ObjectUpdate {
//     // Create a simplified mock ObjectUpdate
//     // In real code, this would come from the vSphere API
//     unimplemented!("This would create a mock ObjectUpdate")
// }