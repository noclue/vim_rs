use vim_rs::vim_retrievable;

vim_retrievable!(
    struct Target: VirtualMachine {
        name = "name",
        num_cpu = "config.hardware.num_cpu",
    }
);
