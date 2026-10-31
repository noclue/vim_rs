//! Fill the property path AND implement labeled_macs().
//! devices is Vec<Box<dyn VirtualDeviceTrait>>. The MAC lives on the Ethernet
//! adapter subtype; the label lives on the common/base device info. Only
//! Ethernet adapters should appear in the output.
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

/// (device label, MAC) for each Ethernet adapter in `t.devices`, device order.
pub fn labeled_macs(t: &Target) -> Vec<(String, String)> {
    let _ = t;
    Vec::new()
}
