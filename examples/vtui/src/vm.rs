use vim_macros::vim_updatable;

vim_updatable!(
    struct VirtualMachine: VirtualMachine {
        name = "name",
        os = "summary.guest.guest_full_name",
        storage = "summary.storage",
        host_cpu = "summary.quick_stats.overall_cpu_usage",
        host_memory = "summary.quick_stats.host_memory_usage",
        status = "overall_status",
        power_state = "runtime.power_state",
    }
);





