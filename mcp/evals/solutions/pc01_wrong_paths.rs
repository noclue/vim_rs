use vim_rs::vim_retrievable;

vim_retrievable!(
    struct Target: VirtualMachine {
        // Wrong paths: valid types but keys won't match fixture propSet
        power_state = "summary.runtime.power_state",
        guest_host = "name",
    }
);
