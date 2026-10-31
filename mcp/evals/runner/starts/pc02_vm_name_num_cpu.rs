//! Fill in the property-path strings.
//! name = "<path>", num_cpu = "<path>".

use vim_rs::vim_retrievable;

vim_retrievable!(
    struct Target: VirtualMachine {
        name = "guest.host_name",
        num_cpu = "name",
    }
);
