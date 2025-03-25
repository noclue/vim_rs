use vim_macros::vim_updatable;
//use vim_rs::types::structs::ObjectUpdate;
use anyhow::Result;
use vim_rs::types::enums::{ManagedEntityStatusEnum, VirtualMachinePowerStateEnum};

vim_updatable!(
    struct VM: VirtualMachine {
        name = "name",
        os = "summary.guest.guestFullName",
        used_space = "summary.storage.committed",
        host_cpu = "summary.quickStats.overallCpuUsage",
        host_memory = "summary.quickStats.hostMemoryUsage",
        status = "summary.overallStatus",
        power_state = "runtime.powerState",
    }
);

fn main() -> Result<()> {
    //let r#type: String = Into::<&str>::into(vim_rs::types::enums::MoTypesEnum::VirtualMachine).to_string();
    // Create a property spec for VMs
    let prop_spec = VM::prop_spec();
    println!("Generated property spec: {:?}", prop_spec);

    let _test_vm = VM{
        id: "365".to_string(),
        name: "My VM".to_string(),
        os: Some("Ubuntu 64-bit".to_string()),
        used_space: Some(1024),
        host_cpu: Some(64000),
        host_memory: Some(1024),
        status: ManagedEntityStatusEnum::Gray,
        power_state: VirtualMachinePowerStateEnum::PoweredOff,
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