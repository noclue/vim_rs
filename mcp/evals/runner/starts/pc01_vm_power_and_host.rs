//! Fill in the property-path strings.
//! power_state = "<path>", guest_host = "<path>".

use vim_rs::vim_retrievable;

vim_retrievable!(
    struct Target: VirtualMachine {
        power_state = "summary.runtime.power_state",
        guest_host = "name",
    }
);
