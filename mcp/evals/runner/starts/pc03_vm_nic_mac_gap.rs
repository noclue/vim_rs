//! Fill the property path AND implement ethernet_macs().
//! devices is Vec<Box<dyn VirtualDeviceTrait>>; only Ethernet adapter
//! subtypes carry a MAC — you must downcast, not read a base field.
//!
//! vim_retrievable!(struct Target: VirtualMachine {
//!     devices = "<path>",
//! });

use vim_rs::vim_retrievable;

vim_retrievable!(
    struct Target: VirtualMachine {
        devices = "name",
    }
);

/// MAC address of each Ethernet adapter in `t.devices`, in device order.
pub fn ethernet_macs(t: &Target) -> Vec<String> {
    let _ = t;
    Vec::new()
}
