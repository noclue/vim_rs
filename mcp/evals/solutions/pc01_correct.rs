use vim_rs::vim_retrievable;

vim_retrievable!(
    struct Target: VirtualMachine {
        power_state = "runtime.power_state",
        guest_host = "guest.host_name",
    }
);
