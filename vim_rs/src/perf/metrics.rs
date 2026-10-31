//! Generated from vim_build/data/perf-catalog.json (vCenter 8.0.3) and
//! vim_build/data/esxi-perf-catalog.json (ESXi 8.0.3 availability). Do not edit.
//!
//! Each constant is one performance counter. The comment above it reads:
//!
//! ```text
//! {catalog summary}
//! `group.name.rollup` · {unit} · {vCenter and ESX | vCenter only}
//! Level: {1-4 | unset}, Per-device level: {1-4 | unset} · Tier: {how Types was chosen}
//! Types: {managed object types}
//! ```
//!
//! The host assigns the numeric counter id ([`PerfCounterInfo.key`][counter-info]).
//! This file names a counter by `group.name.rollup`.
//!
//! **Level** is the lowest vCenter statistics collection level (1-4) that stores
//! the aggregate series. A historical interval stores counters at its collection
//! level and every lower level. The four built-in intervals collect at level 1
//! until that interval is raised, so `Level: 4` is stored only at collection
//! level 4.
//!
//! **Per-device level** is the lowest collection level that stores the same
//! counter split by device, such as `vmnic0` or `scsi0:0`. `Per-device level: 4`
//! means that split is stored only when the interval's collection level is 4.
//! The per-device level is equal to Level or higher: collecting the device
//! series includes the aggregate. `unset` means the catalog left that level empty.
//!
//! **Rollup** is the last part of the name (`average`, `maximum`, `minimum`,
//! `latest`, `summation`, or `none`). The `StatsKind` on the constant is
//! `absolute` (a point-in-time value), `delta` (change since the previous sample),
//! or `rate`.
//!
//! **Unit** is the catalog unit. `percent (hundredths)` counts hundredths of a
//! percent: 10000 is 100%.
//!
//! **vCenter and ESX** means the counter is in both catalogs. **vCenter only**
//! means the counter appears in the vCenter catalog alone.
//!
//! **Tier** is this crate's note on how **Types** was chosen. `Direct (1)` was
//! seen on sample entities. `Sibling Rollup (2)` was copied from another rollup
//! of the same group and name. `Group Union (3a)` is the group's combined set.
//! `Inferred (3b)` is a guess for a group with no samples. [`by_entity`] re-exports
//! Direct, Sibling Rollup, and Group Union. `ResourcePool` includes `VirtualApp`.
//!
//! [counter-info]: https://developer.broadcom.com/xapis/vsphere-web-services-api/latest/vim.PerformanceManager.CounterInfo.html

pub mod cluster_services {
    use super::super::*;

    /// DRS score of the cluster
    /// `clusterServices.clusterDrsScore.latest` · percent (hundredths) · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Known Scope (3k)
    /// Types: ClusterComputeResource
    pub const CLUSTER_DRS_SCORE_LATEST: Metric<Ratio, ClusterOnly> = Metric::new(
        "clusterServices",
        "clusterDrsScore",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// Fairness of distributed CPU resource allocation
    /// `clusterServices.cpufairness.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const CPUFAIRNESS_LATEST: Metric<Count, HostOnly> = Metric::new(
        "clusterServices",
        "cpufairness",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Total available CPU resources of all hosts within a cluster
    /// `clusterServices.effectivecpu.average` · megaHertz · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Known Scope (3k)
    /// Types: ClusterComputeResource
    pub const EFFECTIVECPU_AVERAGE: Metric<Frequency, ClusterOnly> = Metric::new(
        "clusterServices",
        "effectivecpu",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Total amount of machine memory of all hosts in the cluster that is available for use for virtual machine memory and overhead memory
    /// `clusterServices.effectivemem.average` · megaBytes · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Known Scope (3k)
    /// Types: ClusterComputeResource
    pub const EFFECTIVEMEM_AVERAGE: Metric<Memory, ClusterOnly> = Metric::new(
        "clusterServices",
        "effectivemem",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// vSphere HA number of failures that can be tolerated
    /// `clusterServices.failover.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Known Scope (3k)
    /// Types: ClusterComputeResource
    pub const FAILOVER_LATEST: Metric<Count, ClusterOnly> = Metric::new(
        "clusterServices",
        "failover",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Aggregate available memory resources of all the hosts within a cluster
    /// `clusterServices.memfairness.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const MEMFAIRNESS_LATEST: Metric<Count, HostOnly> = Metric::new(
        "clusterServices",
        "memfairness",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// DRS score of the virtual machine
    /// `clusterServices.vmDrsScore.latest` · percent (hundredths) · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const VM_DRS_SCORE_LATEST: Metric<Ratio, VmOnly> = Metric::new(
        "clusterServices",
        "vmDrsScore",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );
}

pub mod cpu {
    use super::super::*;

    /// The amount of L3 cache the VM uses
    /// `cpu.cache.l3.occupancy.average` · kiloBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CACHE_L3_OCCUPANCY_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "cpu",
        "cache.l3.occupancy",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Percent of time the VM is unable to run because it is contending for access to the physical CPU(s)
    /// `cpu.capacity.contention.average` · percent (hundredths) · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_CONTENTION_AVERAGE: Metric<Ratio, ComputeScope> = Metric::new(
        "cpu",
        "capacity.contention",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The amount of CPU resources a VM would use if there were no CPU contention
    /// `cpu.capacity.demand.average` · megaHertz · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_DEMAND_AVERAGE: Metric<Frequency, ComputeScope> = Metric::new(
        "cpu",
        "capacity.demand",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// CPU resources devoted by the ESXi scheduler to the virtual machines and resource pools
    /// `cpu.capacity.entitlement.average` · megaHertz · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_ENTITLEMENT_AVERAGE: Metric<Frequency, ComputeScope> = Metric::new(
        "cpu",
        "capacity.entitlement",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Capacity in MHz of the physical CPU cores
    /// `cpu.capacity.provisioned.average` · megaHertz · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_PROVISIONED_AVERAGE: Metric<Frequency, ComputeScope> = Metric::new(
        "cpu",
        "capacity.provisioned",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// CPU usage as a percent during the interval
    /// `cpu.capacity.usage.average` · megaHertz · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_USAGE_AVERAGE: Metric<Frequency, ComputeScope> = Metric::new(
        "cpu",
        "capacity.usage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Time the VM vCPU is ready to run, but is unable to run due to co-scheduling constraints
    /// `cpu.corecount.contention.average` · percent (hundredths) · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CORECOUNT_CONTENTION_AVERAGE: Metric<Ratio, ComputeScope> = Metric::new(
        "cpu",
        "corecount.contention",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The number of virtual processors provisioned to the entity
    /// `cpu.corecount.provisioned.average` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CORECOUNT_PROVISIONED_AVERAGE: Metric<Count, ComputeScope> = Metric::new(
        "cpu",
        "corecount.provisioned",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The number of virtual processors provisioned to the entity
    /// `cpu.corecount.provisioned.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CORECOUNT_PROVISIONED_LATEST: Metric<Count, ComputeScope> = Metric::new(
        "cpu",
        "corecount.provisioned",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The number of virtual processors running on the host
    /// `cpu.corecount.usage.average` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CORECOUNT_USAGE_AVERAGE: Metric<Count, ComputeScope> = Metric::new(
        "cpu",
        "corecount.usage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The number of virtual processors running on the host
    /// `cpu.corecount.usage.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CORECOUNT_USAGE_LATEST: Metric<Count, ComputeScope> = Metric::new(
        "cpu",
        "corecount.usage",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// CPU utilization of the corresponding core (if hyper-threading is enabled) as a percentage during the interval (A core is utilized if either or both of its logical CPUs are utilized)
    /// `cpu.coreUtilization.average` · percent (hundredths) · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const CORE_UTILIZATION_AVERAGE: Metric<Ratio, HostOnly> = Metric::new(
        "cpu",
        "coreUtilization",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// CPU utilization of the corresponding core (if hyper-threading is enabled) as a percentage during the interval (A core is utilized if either or both of its logical CPUs are utilized)
    /// `cpu.coreUtilization.maximum` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const CORE_UTILIZATION_MAXIMUM: Metric<Ratio, HostOnly> = Metric::new(
        "cpu",
        "coreUtilization",
        Rollup::Maximum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// CPU utilization of the corresponding core (if hyper-threading is enabled) as a percentage during the interval (A core is utilized if either or both of its logical CPUs are utilized)
    /// `cpu.coreUtilization.minimum` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const CORE_UTILIZATION_MINIMUM: Metric<Ratio, HostOnly> = Metric::new(
        "cpu",
        "coreUtilization",
        Rollup::Minimum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// CPU utilization of the corresponding core (if hyper-threading is enabled) as a percentage during the interval (A core is utilized if either or both of its logical CPUs are utilized)
    /// `cpu.coreUtilization.none` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const CORE_UTILIZATION_NONE: Metric<Ratio, HostOnly> = Metric::new(
        "cpu",
        "coreUtilization",
        Rollup::None,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Time the virtual machine is ready to run, but is unable to run due to co-scheduling constraints
    /// `cpu.costop.summation` · millisecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const COSTOP_SUMMATION: Metric<Time, VmOrHost> = Metric::new(
        "cpu",
        "costop",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Amount of CPU resources allocated to the virtual machine or resource pool, based on the total cluster capacity and the resource configuration of the resource hierarchy
    /// `cpu.cpuentitlement.latest` · megaHertz · vCenter only
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: ResourcePool, VirtualMachine
    pub const CPUENTITLEMENT_LATEST: Metric<Frequency, VmOrPool> = Metric::new(
        "cpu",
        "cpuentitlement",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// The amount of CPU resources a virtual machine would use if there were no CPU contention or CPU limit
    /// `cpu.demand.average` · megaHertz · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const DEMAND_AVERAGE: Metric<Frequency, VmOrHost> = Metric::new(
        "cpu",
        "demand",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// CPU resource entitlement to CPU demand ratio (in percents)
    /// `cpu.demandEntitlementRatio.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const DEMAND_ENTITLEMENT_RATIO_LATEST: Metric<Ratio, VmOnly> = Metric::new(
        "cpu",
        "demandEntitlementRatio",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// CPU resources devoted by the ESX scheduler
    /// `cpu.entitlement.latest` · megaHertz · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const ENTITLEMENT_LATEST: Metric<Frequency, VmOnly> = Metric::new(
        "cpu",
        "entitlement",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Total time that the CPU spent in an idle state
    /// `cpu.idle.summation` · millisecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const IDLE_SUMMATION: Metric<Time, VmOrHost> = Metric::new(
        "cpu",
        "idle",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Percent of time the virtual machine is unable to run because it is contending for access to the physical CPU(s)
    /// `cpu.latency.average` · percent (hundredths) · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const LATENCY_AVERAGE: Metric<Ratio, VmOrHost> = Metric::new(
        "cpu",
        "latency",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// CPU load average over the past 15 minutes, sampled on every 6 seconds
    /// `cpu.load.avg15min.latest` · percent (hundredths) · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const LOAD_AVG15MIN_LATEST: Metric<Ratio, ComputeScope> = Metric::new(
        "cpu",
        "load.avg15min",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// CPU load average over the past 1 minute, sampled on every 6 seconds
    /// `cpu.load.avg1min.latest` · percent (hundredths) · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const LOAD_AVG1MIN_LATEST: Metric<Ratio, ComputeScope> = Metric::new(
        "cpu",
        "load.avg1min",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// CPU load average over the past 5 minutes, sampled on every 6 seconds
    /// `cpu.load.avg5min.latest` · percent (hundredths) · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const LOAD_AVG5MIN_LATEST: Metric<Ratio, ComputeScope> = Metric::new(
        "cpu",
        "load.avg5min",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Time the virtual machine is ready to run, but is not run due to maxing out its CPU limit setting
    /// `cpu.maxlimited.summation` · millisecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const MAXLIMITED_SUMMATION: Metric<Time, VmOnly> = Metric::new(
        "cpu",
        "maxlimited",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Time the virtual machine was interrupted to perform system services on behalf of itself or other virtual machines
    /// `cpu.overlap.summation` · millisecond · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const OVERLAP_SUMMATION: Metric<Time, VmOnly> = Metric::new(
        "cpu",
        "overlap",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// The percentage HT partner usage per physical CPU
    /// `cpu.partnerBusyTime.average` · percent (hundredths) · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const PARTNER_BUSY_TIME_AVERAGE: Metric<Ratio, ComputeScope> = Metric::new(
        "cpu",
        "partnerBusyTime",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Percentage of time that the virtual machine was ready, but could not get scheduled to run on the physical CPU
    /// `cpu.readiness.average` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const READINESS_AVERAGE: Metric<Ratio, VmOrHost> = Metric::new(
        "cpu",
        "readiness",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// Time that the virtual machine was ready, but could not get scheduled to run on the physical CPU during last measurement interval
    /// `cpu.ready.summation` · millisecond · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const READY_SUMMATION: Metric<Time, VmOrHost> = Metric::new(
        "cpu",
        "ready",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// Total CPU capacity reserved by virtual machines
    /// `cpu.reservedCapacity.average` · megaHertz · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, HostSystem
    pub const RESERVED_CAPACITY_AVERAGE: Metric<Frequency, HostCluster> = Metric::new(
        "cpu",
        "reservedCapacity",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Time the virtual machine is scheduled to run
    /// `cpu.run.summation` · millisecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const RUN_SUMMATION: Metric<Time, VmOnly> = Metric::new(
        "cpu",
        "run",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// CPU time spent waiting for swap-in
    /// `cpu.swapwait.summation` · millisecond · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const SWAPWAIT_SUMMATION: Metric<Time, VmOrHost> = Metric::new(
        "cpu",
        "swapwait",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Amount of time spent on system processes on each virtual CPU in the virtual machine
    /// `cpu.system.summation` · millisecond · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const SYSTEM_SUMMATION: Metric<Time, VmOnly> = Metric::new(
        "cpu",
        "system",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Total amount of CPU resources of all hosts in the cluster
    /// `cpu.totalmhz.average` · megaHertz · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const TOTALMHZ_AVERAGE: Metric<Frequency, ComputeScope> = Metric::new(
        "cpu",
        "totalmhz",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Total CPU capacity reserved by and available for virtual machines
    /// `cpu.totalCapacity.average` · megaHertz · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const TOTAL_CAPACITY_AVERAGE: Metric<Frequency, HostOnly> = Metric::new(
        "cpu",
        "totalCapacity",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// CPU usage in megahertz during the interval
    /// `cpu.usagemhz.average` · megaHertz · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const USAGEMHZ_AVERAGE: Metric<Frequency, ComputeScope> = Metric::new(
        "cpu",
        "usagemhz",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// CPU usage in megahertz during the interval
    /// `cpu.usagemhz.maximum` · megaHertz · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const USAGEMHZ_MAXIMUM: Metric<Frequency, ComputeScope> = Metric::new(
        "cpu",
        "usagemhz",
        Rollup::Maximum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// CPU usage in megahertz during the interval
    /// `cpu.usagemhz.minimum` · megaHertz · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const USAGEMHZ_MINIMUM: Metric<Frequency, ComputeScope> = Metric::new(
        "cpu",
        "usagemhz",
        Rollup::Minimum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// CPU usage in megahertz during the interval
    /// `cpu.usagemhz.none` · megaHertz · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const USAGEMHZ_NONE: Metric<Frequency, ComputeScope> = Metric::new(
        "cpu",
        "usagemhz",
        Rollup::None,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// CPU usage as a percentage during the interval
    /// `cpu.usage.average` · percent (hundredths) · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, HostSystem, VirtualMachine
    pub const USAGE_AVERAGE: Metric<Ratio, VmHostCluster> = Metric::new(
        "cpu",
        "usage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// CPU usage as a percentage during the interval
    /// `cpu.usage.maximum` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, VirtualMachine
    pub const USAGE_MAXIMUM: Metric<Ratio, VmHostCluster> = Metric::new(
        "cpu",
        "usage",
        Rollup::Maximum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// CPU usage as a percentage during the interval
    /// `cpu.usage.minimum` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, VirtualMachine
    pub const USAGE_MINIMUM: Metric<Ratio, VmHostCluster> = Metric::new(
        "cpu",
        "usage",
        Rollup::Minimum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// CPU usage as a percentage during the interval
    /// `cpu.usage.none` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, VirtualMachine
    pub const USAGE_NONE: Metric<Ratio, VmHostCluster> = Metric::new(
        "cpu",
        "usage",
        Rollup::None,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Virtual CPU usage as a percentage during the interval
    /// `cpu.usage.vcpus.average` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const USAGE_VCPUS_AVERAGE: Metric<Ratio, VmOnly> = Metric::new(
        "cpu",
        "usage.vcpus",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total CPU usage
    /// `cpu.used.summation` · millisecond · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const USED_SUMMATION: Metric<Time, VmOrHost> = Metric::new(
        "cpu",
        "used",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// CPU utilization as a percentage during the interval (CPU usage and CPU utilization might be different due to power management technologies or hyper-threading)
    /// `cpu.utilization.average` · percent (hundredths) · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const UTILIZATION_AVERAGE: Metric<Ratio, HostOnly> = Metric::new(
        "cpu",
        "utilization",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// CPU utilization as a percentage during the interval (CPU usage and CPU utilization might be different due to power management technologies or hyper-threading)
    /// `cpu.utilization.maximum` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const UTILIZATION_MAXIMUM: Metric<Ratio, HostOnly> = Metric::new(
        "cpu",
        "utilization",
        Rollup::Maximum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// CPU utilization as a percentage during the interval (CPU usage and CPU utilization might be different due to power management technologies or hyper-threading)
    /// `cpu.utilization.minimum` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const UTILIZATION_MINIMUM: Metric<Ratio, HostOnly> = Metric::new(
        "cpu",
        "utilization",
        Rollup::Minimum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// CPU utilization as a percentage during the interval (CPU usage and CPU utilization might be different due to power management technologies or hyper-threading)
    /// `cpu.utilization.none` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const UTILIZATION_NONE: Metric<Ratio, HostOnly> = Metric::new(
        "cpu",
        "utilization",
        Rollup::None,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total CPU time spent in wait state
    /// `cpu.wait.summation` · millisecond · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const WAIT_SUMMATION: Metric<Time, VmOrHost> = Metric::new(
        "cpu",
        "wait",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(3u8),
        Some(3u8),
        true,
    );
}

pub mod datastore {
    use super::super::*;

    /// `datastore.busResets.summation` · number · vCenter only
    /// Level: 2, Per-device level: 2 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const BUS_RESETS_SUMMATION: Metric<Count, DatastoreVmHost> = Metric::new(
        "datastore",
        "busResets",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(2u8),
        false,
    );

    /// `datastore.commandsAborted.summation` · number · vCenter only
    /// Level: 2, Per-device level: 2 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const COMMANDS_ABORTED_SUMMATION: Metric<Count, DatastoreVmHost> = Metric::new(
        "datastore",
        "commandsAborted",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(2u8),
        false,
    );

    /// Storage I/O Control aggregated IOPS
    /// `datastore.datastoreIops.average` · number · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const DATASTORE_IOPS_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreIops",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// Storage I/O Control datastore maximum queue depth
    /// `datastore.datastoreMaxQueueDepth.latest` · number · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const DATASTORE_MAX_QUEUE_DEPTH_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreMaxQueueDepth",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// Storage DRS datastore normalized read latency
    /// `datastore.datastoreNormalReadLatency.latest` · number · vCenter and ESX
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: HostSystem
    pub const DATASTORE_NORMAL_READ_LATENCY_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreNormalReadLatency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// Storage DRS datastore normalized write latency
    /// `datastore.datastoreNormalWriteLatency.latest` · number · vCenter and ESX
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: HostSystem
    pub const DATASTORE_NORMAL_WRITE_LATENCY_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreNormalWriteLatency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// Storage DRS datastore bytes read
    /// `datastore.datastoreReadBytes.latest` · number · vCenter and ESX
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: HostSystem
    pub const DATASTORE_READ_BYTES_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreReadBytes",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// Storage DRS datastore read I/O rate
    /// `datastore.datastoreReadIops.latest` · number · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const DATASTORE_READ_IOPS_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreReadIops",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// Storage DRS datastore metric for read workload model
    /// `datastore.datastoreReadLoadMetric.latest` · number · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const DATASTORE_READ_LOAD_METRIC_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreReadLoadMetric",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// Storage DRS datastore outstanding read requests
    /// `datastore.datastoreReadOIO.latest` · number · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const DATASTORE_READ_OIO_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreReadOIO",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// The average datastore latency as seen by virtual machines
    /// `datastore.datastoreVMObservedLatency.latest` · microsecond · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const DATASTORE_VMOBSERVED_LATENCY_LATEST: Metric<Time, HostOnly> = Metric::new(
        "datastore",
        "datastoreVMObservedLatency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// Storage DRS datastore bytes written
    /// `datastore.datastoreWriteBytes.latest` · number · vCenter and ESX
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: HostSystem
    pub const DATASTORE_WRITE_BYTES_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreWriteBytes",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// Storage DRS datastore write I/O rate
    /// `datastore.datastoreWriteIops.latest` · number · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const DATASTORE_WRITE_IOPS_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreWriteIops",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// Storage DRS datastore metric for write workload model
    /// `datastore.datastoreWriteLoadMetric.latest` · number · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const DATASTORE_WRITE_LOAD_METRIC_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreWriteLoadMetric",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// Storage DRS datastore outstanding write requests
    /// `datastore.datastoreWriteOIO.latest` · number · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const DATASTORE_WRITE_OIO_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreWriteOIO",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// Highest latency value across all datastores used by the host
    /// `datastore.maxTotalLatency.latest` · millisecond · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const MAX_TOTAL_LATENCY_LATEST: Metric<Time, VmOrHost> = Metric::new(
        "datastore",
        "maxTotalLatency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Average number of read commands issued per second to the datastore during the collection interval
    /// `datastore.numberReadAveraged.average` · number · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const NUMBER_READ_AVERAGED_AVERAGE: Metric<Count, DatastoreVmHost> = Metric::new(
        "datastore",
        "numberReadAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// Average number of write commands issued per second to the datastore during the collection interval
    /// `datastore.numberWriteAveraged.average` · number · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const NUMBER_WRITE_AVERAGED_AVERAGE: Metric<Count, DatastoreVmHost> = Metric::new(
        "datastore",
        "numberWriteAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// Rate of reading data from the datastore
    /// `datastore.read.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const READ_AVERAGE: Metric<Throughput, DatastoreVmHost> = Metric::new(
        "datastore",
        "read",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// Percentage of time Storage I/O Control actively controlled datastore latency
    /// `datastore.siocActiveTimePercentage.average` · percent (hundredths) · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const SIOC_ACTIVE_TIME_PERCENTAGE_AVERAGE: Metric<Ratio, HostOnly> = Metric::new(
        "datastore",
        "siocActiveTimePercentage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// Storage I/O Control size-normalized I/O latency
    /// `datastore.sizeNormalizedDatastoreLatency.average` · microsecond · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const SIZE_NORMALIZED_DATASTORE_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "datastore",
        "sizeNormalizedDatastoreLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `datastore.throughput.contention.average` · millisecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const THROUGHPUT_CONTENTION_AVERAGE: Metric<Time, DatastoreVmHost> = Metric::new(
        "datastore",
        "throughput.contention",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `datastore.throughput.usage.average` · kiloBytesPerSecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const THROUGHPUT_USAGE_AVERAGE: Metric<Throughput, DatastoreVmHost> = Metric::new(
        "datastore",
        "throughput.usage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The average time a read from the datastore takes
    /// `datastore.totalReadLatency.average` · millisecond · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const TOTAL_READ_LATENCY_AVERAGE: Metric<Time, VmOrHost> = Metric::new(
        "datastore",
        "totalReadLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// The average time a write to the datastore takes
    /// `datastore.totalWriteLatency.average` · millisecond · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const TOTAL_WRITE_LATENCY_AVERAGE: Metric<Time, VmOrHost> = Metric::new(
        "datastore",
        "totalWriteLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// Number of unmap IOs issued
    /// `datastore.unmapIOs.summation` · number · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const UNMAP_IOS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "unmapIOs",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// Unmapped size in MB
    /// `datastore.unmapSize.summation` · megaBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const UNMAP_SIZE_SUMMATION: Metric<Memory, HostOnly> = Metric::new(
        "datastore",
        "unmapSize",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// Total apd timeout events received on the VMFS volume
    /// `datastore.vmfs.apdCount.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const VMFS_APD_COUNT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "datastore",
        "vmfs.apdCount",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Current APD state of the VMFS volume
    /// `datastore.vmfs.apdState.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const VMFS_APD_STATE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "datastore",
        "vmfs.apdState",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total cancelled journal transactions on VMFS volume
    /// `datastore.vmfs.cancelledTxn.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const VMFS_CANCELLED_TXN_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "datastore",
        "vmfs.cancelledTxn",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total journal transactions on VMFS volume
    /// `datastore.vmfs.totalTxn.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const VMFS_TOTAL_TXN_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "datastore",
        "vmfs.totalTxn",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Rate of writing data to the datastore
    /// `datastore.write.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const WRITE_AVERAGE: Metric<Throughput, DatastoreVmHost> = Metric::new(
        "datastore",
        "write",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(2u8),
        true,
    );
}

pub mod disk {
    use super::super::*;

    /// Amount of disk actually used on the datastore
    /// `disk.actualused.latest` · megaBytes · vCenter only
    /// Level: 2, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const ACTUALUSED_LATEST: Metric<Memory, DatastoreVmHost> = Metric::new(
        "disk",
        "actualused",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Number of Storage-bus reset commands issued during the collection interval
    /// `disk.busResets.summation` · number · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const BUS_RESETS_SUMMATION: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "busResets",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// The amount of storage capacity overcommitment for the entity, measured in percent.
    /// `disk.capacity.contention.average` · percent (hundredths) · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const CAPACITY_CONTENTION_AVERAGE: Metric<Ratio, DatastoreVmHost> = Metric::new(
        "disk",
        "capacity.contention",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Configured size of the datastore
    /// `disk.capacity.latest` · kiloBytes · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: Datastore
    pub const CAPACITY_LATEST: Metric<Memory, DatastoreOnly> = Metric::new(
        "disk",
        "capacity",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Virtual disk: The maximum capacity size of the virtual disk. Virtual machine: The provisioned size of all virtual disks plus snapshot files and the swap file, if the VM is running. Datastore: The maximum capacity of the datastore. POD: The maximum capacity of all datastores in the POD.
    /// `disk.capacity.provisioned.average` · kiloBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const CAPACITY_PROVISIONED_AVERAGE: Metric<Memory, DatastoreVmHost> = Metric::new(
        "disk",
        "capacity.provisioned",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The amount of storage capacity currently being consumed by the entity or on the entity.
    /// `disk.capacity.usage.average` · kiloBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const CAPACITY_USAGE_AVERAGE: Metric<Memory, DatastoreVmHost> = Metric::new(
        "disk",
        "capacity.usage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Number of Storage commands aborted during the collection interval
    /// `disk.commandsAborted.summation` · number · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const COMMANDS_ABORTED_SUMMATION: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "commandsAborted",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Number of active commands on the disk
    /// `disk.commands.active.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const COMMANDS_ACTIVE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "commands.active",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average number of Storage commands issued per second during the collection interval
    /// `disk.commandsAveraged.average` · number · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const COMMANDS_AVERAGED_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "disk",
        "commandsAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Number of failed commands on the disk
    /// `disk.commands.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const COMMANDS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "commands.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Number of queued commands on the disk
    /// `disk.commands.queued.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const COMMANDS_QUEUED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "commands.queued",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Number of successful commands on the disk
    /// `disk.commands.success.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const COMMANDS_SUCCESS_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "commands.success",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Number of Storage commands issued during the collection interval
    /// `disk.commands.summation` · number · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const COMMANDS_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "disk",
        "commands",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Storage overhead of a virtual machine or a datastore due to delta disk backings
    /// `disk.deltaused.latest` · kiloBytes · vCenter only
    /// Level: 2, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DELTAUSED_LATEST: Metric<Memory, DatastoreVmHost> = Metric::new(
        "disk",
        "deltaused",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Average amount of time, in milliseconds, to complete a Storage command from the physical device
    /// `disk.deviceLatency.average` · millisecond · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const DEVICE_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "deviceLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// Average amount of time, in milliseconds, to read from the physical device
    /// `disk.deviceReadLatency.average` · millisecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const DEVICE_READ_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "deviceReadLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Average amount of time, in milliseconds, to write to the physical device
    /// `disk.deviceWriteLatency.average` · millisecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const DEVICE_WRITE_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "deviceWriteLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Average amount of time, in milliseconds, spent by VMkernel to process each Storage command
    /// `disk.kernelLatency.average` · millisecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const KERNEL_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "kernelLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Average amount of time, in milliseconds, spent by VMkernel to process each Storage read command
    /// `disk.kernelReadLatency.average` · millisecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const KERNEL_READ_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "kernelReadLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Average amount of time, in milliseconds, spent by VMkernel to process each Storage write command
    /// `disk.kernelWriteLatency.average` · millisecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const KERNEL_WRITE_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "kernelWriteLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Average time, in microseconds, spent by Device to process each Storage command
    /// `disk.latency.davg.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const LATENCY_DAVG_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "disk",
        "latency.davg",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average time, in microseconds, spent by Guest to process each Storage command
    /// `disk.latency.gavg.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const LATENCY_GAVG_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "disk",
        "latency.gavg",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average time, in microseconds, spent by kernel to process each Storage command
    /// `disk.latency.kavg.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const LATENCY_KAVG_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "disk",
        "latency.kavg",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average time, in microseconds, spent by Queue to process each Storage command
    /// `disk.latency.qavg.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const LATENCY_QAVG_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "disk",
        "latency.qavg",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Maximum queue depth
    /// `disk.maxQueueDepth.average` · number · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const MAX_QUEUE_DEPTH_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "disk",
        "maxQueueDepth",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// Highest latency value across all disks used by the host
    /// `disk.maxTotalLatency.latest` · millisecond · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const MAX_TOTAL_LATENCY_LATEST: Metric<Time, VmOrHost> = Metric::new(
        "disk",
        "maxTotalLatency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Average number of disk reads per second during the collection interval
    /// `disk.numberReadAveraged.average` · number · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const NUMBER_READ_AVERAGED_AVERAGE: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "numberReadAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// Number of disk reads during the collection interval
    /// `disk.numberRead.summation` · number · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const NUMBER_READ_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "disk",
        "numberRead",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Average number of disk writes per second during the collection interval
    /// `disk.numberWriteAveraged.average` · number · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const NUMBER_WRITE_AVERAGED_AVERAGE: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "numberWriteAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// Number of disk writes during the collection interval
    /// `disk.numberWrite.summation` · number · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const NUMBER_WRITE_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "disk",
        "numberWrite",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Amount of storage set aside for use by a datastore or a virtual machine
    /// `disk.provisioned.latest` · kiloBytes · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Direct (1)
    /// Types: Datastore, VirtualMachine
    pub const PROVISIONED_LATEST: Metric<Memory, DatastoreVm> = Metric::new(
        "disk",
        "provisioned",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// Average amount of time spent in the VMkernel queue, per Storage command, during the collection interval
    /// `disk.queueLatency.average` · millisecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const QUEUE_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "queueLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Average amount of time spent in the VMkernel queue, per Storage read command, during the collection interval
    /// `disk.queueReadLatency.average` · millisecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const QUEUE_READ_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "queueReadLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Average amount of time spent in the VMkernel queue, per Storage write command, during the collection interval
    /// `disk.queueWriteLatency.average` · millisecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const QUEUE_WRITE_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "queueWriteLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Average number of kilobytes read from the disk each second during the collection interval
    /// `disk.read.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const READ_AVERAGE: Metric<Throughput, DatastoreVmHost> = Metric::new(
        "disk",
        "read",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Number of failed reads on the disk
    /// `disk.readFailed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const READ_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "readFailed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average number of kilobytes read from the disk each second during the collection interval
    /// `disk.read.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const READ_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "read",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Number of Storage reservation conflicts for the LUN as a percent of total commands during the collection interval
    /// `disk.scsiReservationCnflctsPct.average` · percent (hundredths) · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const SCSI_RESERVATION_CNFLCTS_PCT_AVERAGE: Metric<Ratio, DatastoreVmHost> = Metric::new(
        "disk",
        "scsiReservationCnflctsPct",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Number of Storage reservation conflicts for the LUN as a percent of total commands during the collection interval
    /// `disk.scsiReservationCnflctsPct.average` · percent (hundredths) · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const SCSI_RESERVATION_CNFLCTS_PCT_AVERAGE_ABSOLUTE: Metric<Ratio, DatastoreVmHost> =
        Metric::new(
            "disk",
            "scsiReservationCnflctsPct",
            Rollup::Average,
            Some(StatsKind::Absolute),
            Some(4u8),
            Some(4u8),
            false,
        );

    /// Number of Storage reservation conflicts for the LUN during the collection interval
    /// `disk.scsiReservationConflicts.summation` · number · vCenter only
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const SCSI_RESERVATION_CONFLICTS_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "disk",
        "scsiReservationConflicts",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// Current state of device
    /// `disk.state.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const STATE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "state",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average amount of time for an I/O operation to complete successfully
    /// `disk.throughput.contention.average` · millisecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const THROUGHPUT_CONTENTION_AVERAGE: Metric<Time, DatastoreVmHost> = Metric::new(
        "disk",
        "throughput.contention",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Aggregated disk I/O rate, including the rates for all virtual machines running on the host during the collection interval
    /// `disk.throughput.usage.average` · kiloBytesPerSecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const THROUGHPUT_USAGE_AVERAGE: Metric<Throughput, DatastoreVmHost> = Metric::new(
        "disk",
        "throughput.usage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total number of failed aborts on a disk
    /// `disk.TM.abortFailed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const TM_ABORT_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.abortFailed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total number of aborts on a disk
    /// `disk.TM.abort.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const TM_ABORT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.abort",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total number of aborts retries on a disk
    /// `disk.TM.abortRetry.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const TM_ABORT_RETRY_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.abortRetry",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total number of failed bus-resets
    /// `disk.TM.busResetFailed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const TM_BUS_RESET_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.busResetFailed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total number of bus resets
    /// `disk.TM.busReset.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const TM_BUS_RESET_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.busReset",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total number of bus-reset retries
    /// `disk.TM.busResetRetry.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const TM_BUS_RESET_RETRY_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.busResetRetry",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total number of failed device-resets
    /// `disk.TM.deviceResetFailed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const TM_DEVICE_RESET_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.deviceResetFailed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total number of device resets
    /// `disk.TM.deviceReset.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const TM_DEVICE_RESET_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.deviceReset",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total number of device-reset retries
    /// `disk.TM.deviceResetRetry.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const TM_DEVICE_RESET_RETRY_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.deviceResetRetry",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total number of failed lun-resets
    /// `disk.TM.lunResetFailed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const TM_LUN_RESET_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.lunResetFailed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total number of lun resets
    /// `disk.TM.lunReset.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const TM_LUN_RESET_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.lunReset",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total number of lun-reset retries
    /// `disk.TM.lunResetRetry.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const TM_LUN_RESET_RETRY_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.lunResetRetry",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total number of failed virt-resets
    /// `disk.TM.virtResetFailed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const TM_VIRT_RESET_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.virtResetFailed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total number of virt resets
    /// `disk.TM.virtReset.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const TM_VIRT_RESET_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.virtReset",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total number of virt-reset retries
    /// `disk.TM.virtResetRetry.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const TM_VIRT_RESET_RETRY_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.virtResetRetry",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average amount of time taken during the collection interval to process a Storage command issued by the guest OS to the virtual machine
    /// `disk.totalLatency.average` · millisecond · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const TOTAL_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "totalLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Average amount of time taken during the collection interval to process a Storage read command issued from the guest OS to the virtual machine
    /// `disk.totalReadLatency.average` · millisecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const TOTAL_READ_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "totalReadLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Average amount of time taken during the collection interval to process a Storage write command issued by the guest OS to the virtual machine
    /// `disk.totalWriteLatency.average` · millisecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const TOTAL_WRITE_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "totalWriteLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Amount of space associated exclusively with a virtual machine
    /// `disk.unshared.latest` · kiloBytes · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const UNSHARED_LATEST: Metric<Memory, VmOnly> = Metric::new(
        "disk",
        "unshared",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// Aggregated disk I/O rate. For hosts, this metric includes the rates for all virtual machines running on the host during the collection interval.
    /// `disk.usage.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const USAGE_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "disk",
        "usage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Aggregated disk I/O rate. For hosts, this metric includes the rates for all virtual machines running on the host during the collection interval.
    /// `disk.usage.maximum` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem, VirtualMachine
    pub const USAGE_MAXIMUM: Metric<Throughput, VmOrHost> = Metric::new(
        "disk",
        "usage",
        Rollup::Maximum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Aggregated disk I/O rate. For hosts, this metric includes the rates for all virtual machines running on the host during the collection interval.
    /// `disk.usage.minimum` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem, VirtualMachine
    pub const USAGE_MINIMUM: Metric<Throughput, VmOrHost> = Metric::new(
        "disk",
        "usage",
        Rollup::Minimum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Aggregated disk I/O rate. For hosts, this metric includes the rates for all virtual machines running on the host during the collection interval.
    /// `disk.usage.none` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem, VirtualMachine
    pub const USAGE_NONE: Metric<Throughput, VmOrHost> = Metric::new(
        "disk",
        "usage",
        Rollup::None,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of space actually used by the virtual machine or the datastore
    /// `disk.used.latest` · kiloBytes · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Direct (1)
    /// Types: Datastore, VirtualMachine
    pub const USED_LATEST: Metric<Memory, DatastoreVm> = Metric::new(
        "disk",
        "used",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        true,
    );

    /// Average number of kilobytes written to disk each second during the collection interval
    /// `disk.write.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const WRITE_AVERAGE: Metric<Throughput, DatastoreVmHost> = Metric::new(
        "disk",
        "write",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Number of failed writes on the disk
    /// `disk.writeFailed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const WRITE_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "writeFailed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average number of kilobytes written to disk each second during the collection interval
    /// `disk.write.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const WRITE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "write",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );
}

pub mod gpu {
    use super::super::*;

    /// The amount of GPU memory reserved in gigabytes
    /// `gpu.mem.reserved.gb.latest` · gigaBytes · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, ResourcePool
    pub const MEM_RESERVED_GB_LATEST: Metric<Memory, ClusterPool> = Metric::new(
        "gpu",
        "mem.reserved.gb",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The amount of GPU memory reserved in kilobytes
    /// `gpu.mem.reserved.latest` · kiloBytes · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, ResourcePool
    pub const MEM_RESERVED_LATEST: Metric<Memory, ClusterPool> = Metric::new(
        "gpu",
        "mem.reserved",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// The total amount of GPU memory in gigabytes
    /// `gpu.mem.total.gb.latest` · gigaBytes · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, ResourcePool
    pub const MEM_TOTAL_GB_LATEST: Metric<Memory, ClusterPool> = Metric::new(
        "gpu",
        "mem.total.gb",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The total amount of GPU memory in kilobytes
    /// `gpu.mem.total.latest` · kiloBytes · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, ResourcePool
    pub const MEM_TOTAL_LATEST: Metric<Memory, ClusterPool> = Metric::new(
        "gpu",
        "mem.total",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// The amount of GPU memory used in percentages of the total available
    /// `gpu.mem.usage.average` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, ResourcePool
    pub const MEM_USAGE_AVERAGE: Metric<Ratio, ClusterPool> = Metric::new(
        "gpu",
        "mem.usage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The amount of GPU memory used in percentages of the total available
    /// `gpu.mem.usage.maximum` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, ResourcePool
    pub const MEM_USAGE_MAXIMUM: Metric<Ratio, ClusterPool> = Metric::new(
        "gpu",
        "mem.usage",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The amount of GPU memory used in percentages of the total available
    /// `gpu.mem.usage.minimum` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, ResourcePool
    pub const MEM_USAGE_MINIMUM: Metric<Ratio, ClusterPool> = Metric::new(
        "gpu",
        "mem.usage",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The amount of GPU memory used in percentages of the total available
    /// `gpu.mem.usage.none` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, ResourcePool
    pub const MEM_USAGE_NONE: Metric<Ratio, ClusterPool> = Metric::new(
        "gpu",
        "mem.usage",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The amount of GPU memory used in kilobytes
    /// `gpu.mem.used.average` · kiloBytes · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, ResourcePool
    pub const MEM_USED_AVERAGE: Metric<Memory, ClusterPool> = Metric::new(
        "gpu",
        "mem.used",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// The amount of GPU memory used in gigabytes
    /// `gpu.mem.used.gb.latest` · gigaBytes · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, ResourcePool
    pub const MEM_USED_GB_LATEST: Metric<Memory, ClusterPool> = Metric::new(
        "gpu",
        "mem.used.gb",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The amount of GPU memory used in kilobytes
    /// `gpu.mem.used.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, ResourcePool
    pub const MEM_USED_MAXIMUM: Metric<Memory, ClusterPool> = Metric::new(
        "gpu",
        "mem.used",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The amount of GPU memory used in kilobytes
    /// `gpu.mem.used.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, ResourcePool
    pub const MEM_USED_MINIMUM: Metric<Memory, ClusterPool> = Metric::new(
        "gpu",
        "mem.used",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The amount of GPU memory used in kilobytes
    /// `gpu.mem.used.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, ResourcePool
    pub const MEM_USED_NONE: Metric<Memory, ClusterPool> = Metric::new(
        "gpu",
        "mem.used",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The power used by a GPU in watts
    /// `gpu.power.used.latest` · watt · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, ResourcePool
    pub const POWER_USED_LATEST: Metric<Power, ClusterPool> = Metric::new(
        "gpu",
        "power.used",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// The temperature of a GPU in degrees celsius
    /// `gpu.temperature.average` · celsius · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, ResourcePool
    pub const TEMPERATURE_AVERAGE: Metric<Temperature, ClusterPool> = Metric::new(
        "gpu",
        "temperature",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// The compute utilization of a GPU in percentages
    /// `gpu.utilization.average` · percent (hundredths) · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, ResourcePool
    pub const UTILIZATION_AVERAGE: Metric<Ratio, ClusterPool> = Metric::new(
        "gpu",
        "utilization",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// The compute utilization of a GPU in percentages
    /// `gpu.utilization.maximum` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, ResourcePool
    pub const UTILIZATION_MAXIMUM: Metric<Ratio, ClusterPool> = Metric::new(
        "gpu",
        "utilization",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The compute utilization of a GPU in percentages
    /// `gpu.utilization.minimum` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, ResourcePool
    pub const UTILIZATION_MINIMUM: Metric<Ratio, ClusterPool> = Metric::new(
        "gpu",
        "utilization",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The compute utilization of a GPU in percentages
    /// `gpu.utilization.none` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, ResourcePool
    pub const UTILIZATION_NONE: Metric<Ratio, ClusterPool> = Metric::new(
        "gpu",
        "utilization",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );
}

pub mod hbr {
    use super::super::*;

    /// Average disk read latency seen by vSphere Replication
    /// `hbr.hbrDiskReadLatency.average` · millisecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const HBR_DISK_READ_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "hbr",
        "hbrDiskReadLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average guest I/O stall introduced by vSphere Replication
    /// `hbr.hbrDiskStallLatency.average` · millisecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const HBR_DISK_STALL_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "hbr",
        "hbrDiskStallLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average amount of data in KB successfully transferred per disk
    /// `hbr.hbrDiskTransferBytes.average` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem
    pub const HBR_DISK_TRANSFER_BYTES_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "hbr",
        "hbrDiskTransferBytes",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average amount of idle time per disk
    /// `hbr.hbrDiskTransferIdle.average` · millisecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem
    pub const HBR_DISK_TRANSFER_IDLE_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "hbr",
        "hbrDiskTransferIdle",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average amount of successful transfer time per disk
    /// `hbr.hbrDiskTransferSuccess.average` · millisecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem
    pub const HBR_DISK_TRANSFER_SUCCESS_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "hbr",
        "hbrDiskTransferSuccess",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average network latency seen by vSphere Replication
    /// `hbr.hbrNetLatency.average` · millisecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem
    pub const HBR_NET_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "hbr",
        "hbrNetLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average amount of data received per second
    /// `hbr.hbrNetRx.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const HBR_NET_RX_AVERAGE: Metric<Throughput, HostOnly> = Metric::new(
        "hbr",
        "hbrNetRx",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average amount of data transmitted per second
    /// `hbr.hbrNetTx.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const HBR_NET_TX_AVERAGE: Metric<Throughput, HostOnly> = Metric::new(
        "hbr",
        "hbrNetTx",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Current number of replicated virtual machines
    /// `hbr.hbrNumVms.average` · number · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const HBR_NUM_VMS_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "hbr",
        "hbrNumVms",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Current number of replicated virtual machines
    /// `hbr.vms.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem
    pub const VMS_LATEST: Metric<Count, HostOnly> = Metric::new(
        "hbr",
        "vms",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );
}

pub mod lwd {
    use super::super::*;

    /// Number of Lightweight Delta (LWD) snapshots taken
    /// `lwd.numSnapshots.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem, VirtualMachine
    pub const NUM_SNAPSHOTS_LATEST: Metric<Count, VmOrHost> = Metric::new(
        "lwd",
        "numSnapshots",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );
}

pub mod management_agent {
    use super::super::*;

    /// Amount of Service Console CPU usage
    /// `managementAgent.cpuUsage.average` · megaHertz · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const CPU_USAGE_AVERAGE: Metric<Frequency, HostOnly> = Metric::new(
        "managementAgent",
        "cpuUsage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Amount of total configured memory that is available for use
    /// `managementAgent.memUsed.average` · kiloBytes · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const MEM_USED_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "managementAgent",
        "memUsed",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Amount of memory that is swapped in for the Service Console
    /// `managementAgent.swapIn.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const SWAP_IN_AVERAGE: Metric<Throughput, HostOnly> = Metric::new(
        "managementAgent",
        "swapIn",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Amount of memory that is swapped out for the Service Console
    /// `managementAgent.swapOut.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const SWAP_OUT_AVERAGE: Metric<Throughput, HostOnly> = Metric::new(
        "managementAgent",
        "swapOut",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Sum of the memory swapped by all powered-on virtual machines on the host
    /// `managementAgent.swapUsed.average` · kiloBytes · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const SWAP_USED_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "managementAgent",
        "swapUsed",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );
}

pub mod mem {
    use super::super::*;

    /// Amount of guest physical memory that is being actively written by guest. Activeness is estimated by ESXi
    /// `mem.activewrite.average` · kiloBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const ACTIVEWRITE_AVERAGE: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "activewrite",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Amount of guest physical memory that is being actively read or written by guest. Activeness is estimated by ESXi
    /// `mem.active.average` · kiloBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const ACTIVE_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "active",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Amount of guest physical memory that is being actively read or written by guest. Activeness is estimated by ESXi
    /// `mem.active.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const ACTIVE_MAXIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "active",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory that is being actively read or written by guest. Activeness is estimated by ESXi
    /// `mem.active.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const ACTIVE_MINIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "active",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory that is being actively read or written by guest. Activeness is estimated by ESXi
    /// `mem.active.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const ACTIVE_NONE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "active",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Current read bandwidth of this memory type
    /// `mem.bandwidth.read.latest` · kiloBytesPerSecond · vCenter only
    /// Level: 2, Per-device level: 2 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const BANDWIDTH_READ_LATEST: Metric<Throughput, ComputeScope> = Metric::new(
        "mem",
        "bandwidth.read",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        false,
    );

    /// Total read and write bandwidth of this memory type
    /// `mem.bandwidth.total.latest` · kiloBytesPerSecond · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const BANDWIDTH_TOTAL_LATEST: Metric<Throughput, ComputeScope> = Metric::new(
        "mem",
        "bandwidth.total",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// Current write bandwidth of this memory type
    /// `mem.bandwidth.write.latest` · kiloBytesPerSecond · vCenter only
    /// Level: 2, Per-device level: 2 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const BANDWIDTH_WRITE_LATEST: Metric<Throughput, ComputeScope> = Metric::new(
        "mem",
        "bandwidth.write",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        false,
    );

    /// Per tier active memory. This value is expressed in megabytes
    /// `mem.capacity.active.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_ACTIVE_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.active",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Per tier consumed memory. This value is expressed in megabytes
    /// `mem.capacity.consumed.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_CONSUMED_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.consumed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Percentage of time VMs are waiting to access swapped, compressed or ballooned memory
    /// `mem.capacity.contention.average` · percent (hundredths) · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_CONTENTION_AVERAGE: Metric<Ratio, ComputeScope> = Metric::new(
        "mem",
        "capacity.contention",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of host physical memory the VM is entitled to, as determined by the ESXi scheduler
    /// `mem.capacity.entitlement.average` · kiloBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_ENTITLEMENT_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.entitlement",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of Overhead memory actively used
    /// `mem.capacity.overhead.average` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_OVERHEAD_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.overhead",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of OverheadResv memory
    /// `mem.capacity.overheadResv.average` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_OVERHEAD_RESV_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.overheadResv",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total amount of memory available to the host
    /// `mem.capacity.provisioned.average` · kiloBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_PROVISIONED_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.provisioned",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total amount of memory available to the host
    /// `mem.capacity.provisioned.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_PROVISIONED_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.provisioned",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of physical memory available for use by virtual machines on this host
    /// `mem.capacity.usable.average` · kiloBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_USABLE_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.usable",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of physical memory actively used
    /// `mem.capacity.usage.average` · kiloBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_USAGE_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.usage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.capacity.usage.userworld.average` · kiloBytes · vCenter only
    /// Level: 2, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_USAGE_USERWORLD_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.usage.userworld",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(4u8),
        false,
    );

    /// `mem.capacity.usage.vmkOvrhd.average` · kiloBytes · vCenter only
    /// Level: 2, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_USAGE_VMK_OVRHD_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.usage.vmkOvrhd",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(4u8),
        false,
    );

    /// `mem.capacity.usage.vm.average` · kiloBytes · vCenter only
    /// Level: 2, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_USAGE_VM_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.usage.vm",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(4u8),
        false,
    );

    /// `mem.capacity.usage.vmOvrhd.average` · kiloBytes · vCenter only
    /// Level: 2, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_USAGE_VM_OVRHD_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.usage.vmOvrhd",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(4u8),
        false,
    );

    /// Guest physical memory pages that have undergone memory compression
    /// `mem.compressed.average` · kiloBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const COMPRESSED_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "compressed",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Rate of guest physical memory page compression by ESXi
    /// `mem.compressionRate.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const COMPRESSION_RATE_AVERAGE: Metric<Throughput, ComputeScope> = Metric::new(
        "mem",
        "compressionRate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Amount of host physical memory consumed for backing up guest physical memory pages
    /// `mem.consumed.average` · kiloBytes · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CONSUMED_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "consumed",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Amount of host physical memory consumed for backing up guest physical memory pages
    /// `mem.consumed.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CONSUMED_MAXIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "consumed",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of host physical memory consumed for backing up guest physical memory pages
    /// `mem.consumed.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CONSUMED_MINIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "consumed",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of host physical memory consumed for backing up guest physical memory pages
    /// `mem.consumed.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CONSUMED_NONE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "consumed",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of physical memory consumed by userworlds on this host
    /// `mem.consumed.userworlds.average` · kiloBytes · vCenter only
    /// Level: 2, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const CONSUMED_USERWORLDS_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "consumed.userworlds",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(4u8),
        false,
    );

    /// Amount of physical memory consumed by VMs on this host
    /// `mem.consumed.vms.average` · kiloBytes · vCenter only
    /// Level: 2, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const CONSUMED_VMS_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "consumed.vms",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(4u8),
        true,
    );

    /// Rate of guest physical memory decompression
    /// `mem.decompressionRate.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const DECOMPRESSION_RATE_AVERAGE: Metric<Throughput, ComputeScope> = Metric::new(
        "mem",
        "decompressionRate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Amount of host physical memory the virtual machine deserves, as determined by ESXi
    /// `mem.entitlement.average` · kiloBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const ENTITLEMENT_AVERAGE: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "entitlement",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Amount of host physical memory or physical memory that is mapped for a virtual machine or a host
    /// `mem.granted.average` · kiloBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const GRANTED_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "granted",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Amount of host physical memory or physical memory that is mapped for a virtual machine or a host
    /// `mem.granted.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const GRANTED_MAXIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "granted",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of host physical memory or physical memory that is mapped for a virtual machine or a host
    /// `mem.granted.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const GRANTED_MINIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "granted",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of host physical memory or physical memory that is mapped for a virtual machine or a host
    /// `mem.granted.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const GRANTED_NONE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "granted",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Memory reservation health state, 2->Red, 1->Green
    /// `mem.health.reservationState.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const HEALTH_RESERVATION_STATE_LATEST: Metric<Count, ComputeScope> = Metric::new(
        "mem",
        "health.reservationState",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Free address space in the heap of ESXi. This is less than or equal to Heap
    /// `mem.heapfree.average` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const HEAPFREE_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "heapfree",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Free address space in the heap of ESXi. This is less than or equal to Heap
    /// `mem.heapfree.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const HEAPFREE_MAXIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "heapfree",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Free address space in the heap of ESXi. This is less than or equal to Heap
    /// `mem.heapfree.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const HEAPFREE_MINIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "heapfree",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Free address space in the heap of ESXi. This is less than or equal to Heap
    /// `mem.heapfree.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const HEAPFREE_NONE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "heapfree",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Virtual address space of ESXi that is dedicated to its heap
    /// `mem.heap.average` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const HEAP_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "heap",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Virtual address space of ESXi that is dedicated to its heap
    /// `mem.heap.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const HEAP_MAXIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "heap",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Virtual address space of ESXi that is dedicated to its heap
    /// `mem.heap.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const HEAP_MINIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "heap",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Virtual address space of ESXi that is dedicated to its heap
    /// `mem.heap.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const HEAP_NONE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "heap",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total amount of machine memory managed by VMkernel
    /// `mem.kernel.managed.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const KERNEL_MANAGED_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "kernel.managed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Mininum amount of machine memory that VMkernel likes to keep free
    /// `mem.kernel.minfree.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const KERNEL_MINFREE_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "kernel.minfree",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of machine memory that is currently unreserved
    /// `mem.kernel.unreserved.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const KERNEL_UNRESERVED_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "kernel.unreserved",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Percentage of time the virtual machine spent waiting to swap in or decompress guest physical memory
    /// `mem.latency.average` · percent (hundredths) · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const LATENCY_AVERAGE: Metric<Ratio, VmOrHost> = Metric::new(
        "mem",
        "latency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Get the current read latency of this memory type
    /// `mem.latency.read.latest` · nanosecond · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const LATENCY_READ_LATEST: Metric<Time, ComputeScope> = Metric::new(
        "mem",
        "latency.read",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Get the current write latency of this memory type
    /// `mem.latency.write.latest` · nanosecond · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const LATENCY_WRITE_LATEST: Metric<Time, ComputeScope> = Metric::new(
        "mem",
        "latency.write",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Amount of guest physical memory swapped in from host cache
    /// `mem.llSwapIn.average` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const LL_SWAP_IN_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "llSwapIn",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory swapped in from host cache
    /// `mem.llSwapIn.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const LL_SWAP_IN_MAXIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "llSwapIn",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory swapped in from host cache
    /// `mem.llSwapIn.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const LL_SWAP_IN_MINIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "llSwapIn",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory swapped in from host cache
    /// `mem.llSwapIn.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const LL_SWAP_IN_NONE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "llSwapIn",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Rate at which guest physical memory is swapped in from the host swap cache
    /// `mem.llSwapInRate.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const LL_SWAP_IN_RATE_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "mem",
        "llSwapInRate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Amount of guest physical memory swapped out to the host swap cache
    /// `mem.llSwapOut.average` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const LL_SWAP_OUT_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "llSwapOut",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory swapped out to the host swap cache
    /// `mem.llSwapOut.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const LL_SWAP_OUT_MAXIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "llSwapOut",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory swapped out to the host swap cache
    /// `mem.llSwapOut.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const LL_SWAP_OUT_MINIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "llSwapOut",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory swapped out to the host swap cache
    /// `mem.llSwapOut.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const LL_SWAP_OUT_NONE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "llSwapOut",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Rate at which guest physical memory is swapped out to the host swap cache
    /// `mem.llSwapOutRate.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const LL_SWAP_OUT_RATE_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "mem",
        "llSwapOutRate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Storage space consumed on the host swap cache for storing swapped guest physical memory pages
    /// `mem.llSwapUsed.average` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const LL_SWAP_USED_AVERAGE: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "llSwapUsed",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Storage space consumed on the host swap cache for storing swapped guest physical memory pages
    /// `mem.llSwapUsed.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem, VirtualMachine
    pub const LL_SWAP_USED_MAXIMUM: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "llSwapUsed",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Storage space consumed on the host swap cache for storing swapped guest physical memory pages
    /// `mem.llSwapUsed.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem, VirtualMachine
    pub const LL_SWAP_USED_MINIMUM: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "llSwapUsed",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Storage space consumed on the host swap cache for storing swapped guest physical memory pages
    /// `mem.llSwapUsed.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem, VirtualMachine
    pub const LL_SWAP_USED_NONE: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "llSwapUsed",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Threshold of free host physical memory below which ESXi will begin actively reclaiming memory from virtual machines by swapping, compression and ballooning
    /// `mem.lowfreethreshold.average` · kiloBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const LOWFREETHRESHOLD_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "lowfreethreshold",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Total amount of physical memory reclaimed using the vmmemctl modules
    /// `mem.memctl.current.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const MEMCTL_CURRENT_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "memctl.current",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Maximum amount of physical memory ESXi can reclaim using the vmmemctl modules
    /// `mem.memctl.max.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const MEMCTL_MAX_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "memctl.max",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total amount of physical memory ESXi would like to reclaim using the vmmemctl modules
    /// `mem.memctl.target.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const MEMCTL_TARGET_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "memctl.target",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Memory allocation as calculated by the VMkernel scheduler based on current estimated demand and reservation, limit, and shares policies set for all virtual machines and resource pools in the host or cluster
    /// `mem.mementitlement.latest` · megaBytes · vCenter only
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: ResourcePool, VirtualMachine
    pub const MEMENTITLEMENT_LATEST: Metric<Memory, VmOrPool> = Metric::new(
        "mem",
        "mementitlement",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Get the current miss rate of this memory type
    /// `mem.missrate.latest` · percent (hundredths) · vCenter only
    /// Level: 2, Per-device level: 2 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const MISSRATE_LATEST: Metric<Ratio, ComputeScope> = Metric::new(
        "mem",
        "missrate",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        false,
    );

    /// Ratio of total requested memory and the managed memory minus 1 over the past 15 minutes
    /// `mem.overcommit.avg15min.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const OVERCOMMIT_AVG15MIN_LATEST: Metric<Count, ComputeScope> = Metric::new(
        "mem",
        "overcommit.avg15min",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Ratio of total requested memory and the managed memory minus 1 over the past 1 minute
    /// `mem.overcommit.avg1min.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const OVERCOMMIT_AVG1MIN_LATEST: Metric<Count, ComputeScope> = Metric::new(
        "mem",
        "overcommit.avg1min",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Ratio of total requested memory and the managed memory minus 1 over the past 5 minutes
    /// `mem.overcommit.avg5min.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const OVERCOMMIT_AVG5MIN_LATEST: Metric<Count, ComputeScope> = Metric::new(
        "mem",
        "overcommit.avg5min",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Host physical memory consumed by ESXi data structures for running the virtual machines
    /// `mem.overhead.average` · kiloBytes · vCenter and ESX
    /// Level: 1, Per-device level: 1 · Tier: Direct (1)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const OVERHEAD_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "overhead",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// Host physical memory consumed by ESXi data structures for running the virtual machines
    /// `mem.overhead.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const OVERHEAD_MAXIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "overhead",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Host physical memory reserved by ESXi, for its data structures, for running the virtual machine
    /// `mem.overheadMax.average` · kiloBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const OVERHEAD_MAX_AVERAGE: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "overheadMax",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Host physical memory consumed by ESXi data structures for running the virtual machines
    /// `mem.overhead.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const OVERHEAD_MINIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "overhead",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Host physical memory consumed by ESXi data structures for running the virtual machines
    /// `mem.overhead.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const OVERHEAD_NONE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "overhead",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Estimate of the host physical memory, from Overhead consumed, that is actively read or written to by ESXi
    /// `mem.overheadTouched.average` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const OVERHEAD_TOUCHED_AVERAGE: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "overheadTouched",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of machine memory that is free on the ESXi host
    /// `mem.physical.free.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const PHYSICAL_FREE_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "physical.free",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total amount of machine memory on the ESXi host
    /// `mem.physical.total.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const PHYSICAL_TOTAL_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "physical.total",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of machine memory being used by everything other than VMkernel
    /// `mem.physical.user.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const PHYSICAL_USER_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "physical.user",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of machine memory that is common across World(s)
    /// `mem.pshare.common.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const PSHARE_COMMON_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "pshare.common",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of physical memory that is being shared
    /// `mem.pshare.shared.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const PSHARE_SHARED_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "pshare.shared",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of machine memory saved due to page-sharing
    /// `mem.pshare.sharedSave.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const PSHARE_SHARED_SAVE_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "pshare.sharedSave",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Memory reservation consumed by powered-on virtual machines
    /// `mem.reservedCapacity.average` · megaBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, HostSystem
    pub const RESERVED_CAPACITY_AVERAGE: Metric<Memory, HostCluster> = Metric::new(
        "mem",
        "reservedCapacity",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Percent of memory that has been reserved either through VMkernel use, by userworlds or due to VM memory reservations
    /// `mem.reservedCapacityPct.average` · percent (hundredths) · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const RESERVED_CAPACITY_PCT_AVERAGE: Metric<Ratio, ComputeScope> = Metric::new(
        "mem",
        "reservedCapacityPct",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Percent of memory that has been reserved either through VMkernel use, by userworlds or due to VM memory reservations
    /// `mem.reservedCapacityPct.latest` · percent (hundredths) · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const RESERVED_CAPACITY_PCT_LATEST: Metric<Ratio, ComputeScope> = Metric::new(
        "mem",
        "reservedCapacityPct",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.reservedCapacity.userworld.average` · kiloBytes · vCenter only
    /// Level: 2, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const RESERVED_CAPACITY_USERWORLD_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "reservedCapacity.userworld",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(4u8),
        false,
    );

    /// `mem.reservedCapacity.vmkOvrhd.average` · kiloBytes · vCenter only
    /// Level: 2, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const RESERVED_CAPACITY_VMK_OVRHD_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "reservedCapacity.vmkOvrhd",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(4u8),
        false,
    );

    /// `mem.reservedCapacity.vm.average` · kiloBytes · vCenter only
    /// Level: 2, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const RESERVED_CAPACITY_VM_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "reservedCapacity.vm",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(4u8),
        false,
    );

    /// `mem.reservedCapacity.vmOvhd.average` · kiloBytes · vCenter only
    /// Level: 2, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const RESERVED_CAPACITY_VM_OVHD_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "reservedCapacity.vmOvhd",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(4u8),
        false,
    );

    /// Amount of host physical memory that backs shared guest physical memory (Shared)
    /// `mem.sharedcommon.average` · kiloBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const SHAREDCOMMON_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "sharedcommon",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Amount of host physical memory that backs shared guest physical memory (Shared)
    /// `mem.sharedcommon.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const SHAREDCOMMON_MAXIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "sharedcommon",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of host physical memory that backs shared guest physical memory (Shared)
    /// `mem.sharedcommon.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const SHAREDCOMMON_MINIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "sharedcommon",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of host physical memory that backs shared guest physical memory (Shared)
    /// `mem.sharedcommon.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const SHAREDCOMMON_NONE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "sharedcommon",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory that is shared within a single virtual machine or across virtual machines
    /// `mem.shared.average` · kiloBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const SHARED_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "shared",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Amount of guest physical memory that is shared within a single virtual machine or across virtual machines
    /// `mem.shared.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const SHARED_MAXIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "shared",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory that is shared within a single virtual machine or across virtual machines
    /// `mem.shared.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const SHARED_MINIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "shared",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory that is shared within a single virtual machine or across virtual machines
    /// `mem.shared.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const SHARED_NONE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "shared",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Current memory availability state of ESXi. Possible values are high, clear, soft, hard, low. The state value determines the techniques used for memory reclamation from virtual machines
    /// `mem.state.latest` · number · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const STATE_LATEST: Metric<Count, HostOnly> = Metric::new(
        "mem",
        "state",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Amount of guest physical memory that is swapped in from the swap space since the virtual machine has been powered on. This value is less than or equal to the 'Swap out' counter
    /// `mem.swapin.average` · kiloBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const SWAPIN_AVERAGE: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "swapin",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Amount of guest physical memory that is swapped in from the swap space since the virtual machine has been powered on. This value is less than or equal to the 'Swap out' counter
    /// `mem.swapin.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem, VirtualMachine
    pub const SWAPIN_MAXIMUM: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "swapin",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory that is swapped in from the swap space since the virtual machine has been powered on. This value is less than or equal to the 'Swap out' counter
    /// `mem.swapin.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem, VirtualMachine
    pub const SWAPIN_MINIMUM: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "swapin",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory that is swapped in from the swap space since the virtual machine has been powered on. This value is less than or equal to the 'Swap out' counter
    /// `mem.swapin.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem, VirtualMachine
    pub const SWAPIN_NONE: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "swapin",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Rate at which guest physical memory is swapped in from the swap space
    /// `mem.swapinRate.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const SWAPIN_RATE_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "mem",
        "swapinRate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Amount of guest physical memory that is swapped out from the virtual machine to its swap space since it has been powered on.
    /// `mem.swapout.average` · kiloBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const SWAPOUT_AVERAGE: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "swapout",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Amount of guest physical memory that is swapped out from the virtual machine to its swap space since it has been powered on.
    /// `mem.swapout.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem, VirtualMachine
    pub const SWAPOUT_MAXIMUM: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "swapout",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory that is swapped out from the virtual machine to its swap space since it has been powered on.
    /// `mem.swapout.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem, VirtualMachine
    pub const SWAPOUT_MINIMUM: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "swapout",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory that is swapped out from the virtual machine to its swap space since it has been powered on.
    /// `mem.swapout.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem, VirtualMachine
    pub const SWAPOUT_NONE: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "swapout",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Rate at which guest physical memory is swapped out to the swap space
    /// `mem.swapoutRate.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const SWAPOUT_RATE_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "mem",
        "swapoutRate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Amount of guest physical memory that is swapped out to the swap space
    /// `mem.swapped.average` · kiloBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: ResourcePool, VirtualMachine
    pub const SWAPPED_AVERAGE: Metric<Memory, VmOrPool> = Metric::new(
        "mem",
        "swapped",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Amount of guest physical memory that is swapped out to the swap space
    /// `mem.swapped.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ResourcePool, VirtualMachine
    pub const SWAPPED_MAXIMUM: Metric<Memory, VmOrPool> = Metric::new(
        "mem",
        "swapped",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory that is swapped out to the swap space
    /// `mem.swapped.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ResourcePool, VirtualMachine
    pub const SWAPPED_MINIMUM: Metric<Memory, VmOrPool> = Metric::new(
        "mem",
        "swapped",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory that is swapped out to the swap space
    /// `mem.swapped.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ResourcePool, VirtualMachine
    pub const SWAPPED_NONE: Metric<Memory, VmOrPool> = Metric::new(
        "mem",
        "swapped",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of memory that ESXi needs to reclaim by swapping
    /// `mem.swaptarget.average` · kiloBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const SWAPTARGET_AVERAGE: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "swaptarget",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Amount of memory that ESXi needs to reclaim by swapping
    /// `mem.swaptarget.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: VirtualMachine
    pub const SWAPTARGET_MAXIMUM: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "swaptarget",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of memory that ESXi needs to reclaim by swapping
    /// `mem.swaptarget.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: VirtualMachine
    pub const SWAPTARGET_MINIMUM: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "swaptarget",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of memory that ESXi needs to reclaim by swapping
    /// `mem.swaptarget.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: VirtualMachine
    pub const SWAPTARGET_NONE: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "swaptarget",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapunreserved.average` · kiloBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const SWAPUNRESERVED_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapunreserved",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapunreserved.maximum` · kiloBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const SWAPUNRESERVED_MAXIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapunreserved",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapunreserved.minimum` · kiloBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const SWAPUNRESERVED_MINIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapunreserved",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapunreserved.none` · kiloBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const SWAPUNRESERVED_NONE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapunreserved",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Swap storage space consumed
    /// `mem.swapused.average` · kiloBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, HostSystem
    pub const SWAPUSED_AVERAGE: Metric<Memory, HostCluster> = Metric::new(
        "mem",
        "swapused",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Swap storage space consumed
    /// `mem.swapused.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem
    pub const SWAPUSED_MAXIMUM: Metric<Memory, HostCluster> = Metric::new(
        "mem",
        "swapused",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Swap storage space consumed
    /// `mem.swapused.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem
    pub const SWAPUSED_MINIMUM: Metric<Memory, HostCluster> = Metric::new(
        "mem",
        "swapused",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Swap storage space consumed
    /// `mem.swapused.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem
    pub const SWAPUSED_NONE: Metric<Memory, HostCluster> = Metric::new(
        "mem",
        "swapused",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Current swap usage
    /// `mem.swap.current.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const SWAP_CURRENT_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swap.current",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapIn.average` · kiloBytes · vCenter only
    /// Level: 2, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const SWAP_IN_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapIn",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.swapIn.maximum` · kiloBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const SWAP_IN_MAXIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapIn",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapIn.minimum` · kiloBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const SWAP_IN_MINIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapIn",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapIn.none` · kiloBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const SWAP_IN_NONE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapIn",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapOut.average` · kiloBytes · vCenter only
    /// Level: 2, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const SWAP_OUT_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapOut",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.swapOut.maximum` · kiloBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const SWAP_OUT_MAXIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapOut",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapOut.minimum` · kiloBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const SWAP_OUT_MINIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapOut",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapOut.none` · kiloBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const SWAP_OUT_NONE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapOut",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Rate at which memory is swapped in by ESXi from disk
    /// `mem.swap.readrate.average` · megaBytesPerSecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const SWAP_READRATE_AVERAGE: Metric<Throughput, ComputeScope> = Metric::new(
        "mem",
        "swap.readrate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Where ESXi expects the reclaimed memory using swapping and compression to be
    /// `mem.swap.target.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const SWAP_TARGET_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swap.target",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Rate at which memory is swapped to disk by the ESXi
    /// `mem.swap.writerate.average` · megaBytesPerSecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const SWAP_WRITERATE_AVERAGE: Metric<Throughput, ComputeScope> = Metric::new(
        "mem",
        "swap.writerate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of host physical memory consumed by VMkernel
    /// `mem.sysUsage.average` · kiloBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const SYS_USAGE_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "sysUsage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Amount of host physical memory consumed by VMkernel
    /// `mem.sysUsage.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const SYS_USAGE_MAXIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "sysUsage",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of host physical memory consumed by VMkernel
    /// `mem.sysUsage.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const SYS_USAGE_MINIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "sysUsage",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of host physical memory consumed by VMkernel
    /// `mem.sysUsage.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const SYS_USAGE_NONE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "sysUsage",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total amount of host physical memory of all hosts in the cluster that is available for virtual machine memory (physical memory for use by the guest OS) and virtual machine overhead memory
    /// `mem.totalmb.average` · megaBytes · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const TOTALMB_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "totalmb",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Total reservation, available and consumed, for powered-on virtual machines
    /// `mem.totalCapacity.average` · megaBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const TOTAL_CAPACITY_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "totalCapacity",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Amount by which reservation can be raised
    /// `mem.unreserved.average` · kiloBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const UNRESERVED_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "unreserved",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Amount by which reservation can be raised
    /// `mem.unreserved.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const UNRESERVED_MAXIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "unreserved",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount by which reservation can be raised
    /// `mem.unreserved.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const UNRESERVED_MINIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "unreserved",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount by which reservation can be raised
    /// `mem.unreserved.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const UNRESERVED_NONE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "unreserved",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Percentage of host physical memory that has been consumed
    /// `mem.usage.average` · percent (hundredths) · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, HostSystem, VirtualMachine
    pub const USAGE_AVERAGE: Metric<Ratio, VmHostCluster> = Metric::new(
        "mem",
        "usage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Percentage of host physical memory that has been consumed
    /// `mem.usage.maximum` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, VirtualMachine
    pub const USAGE_MAXIMUM: Metric<Ratio, VmHostCluster> = Metric::new(
        "mem",
        "usage",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Percentage of host physical memory that has been consumed
    /// `mem.usage.minimum` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, VirtualMachine
    pub const USAGE_MINIMUM: Metric<Ratio, VmHostCluster> = Metric::new(
        "mem",
        "usage",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Percentage of host physical memory that has been consumed
    /// `mem.usage.none` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, VirtualMachine
    pub const USAGE_NONE: Metric<Ratio, VmHostCluster> = Metric::new(
        "mem",
        "usage",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Trailing average of the ratio of capacity misses to compulsory misses for the VMFS PB Cache
    /// `mem.vmfs.pbc.capMissRatio.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const VMFS_PBC_CAP_MISS_RATIO_LATEST: Metric<Ratio, HostOnly> = Metric::new(
        "mem",
        "vmfs.pbc.capMissRatio",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of VMFS heap used by the VMFS PB Cache
    /// `mem.vmfs.pbc.overhead.latest` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const VMFS_PBC_OVERHEAD_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "vmfs.pbc.overhead",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Space used for holding VMFS Pointer Blocks in memory
    /// `mem.vmfs.pbc.size.latest` · megaBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const VMFS_PBC_SIZE_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "vmfs.pbc.size",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Maximum size the VMFS Pointer Block Cache can grow to
    /// `mem.vmfs.pbc.sizeMax.latest` · megaBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const VMFS_PBC_SIZE_MAX_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "vmfs.pbc.sizeMax",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of file blocks whose addresses are cached in the VMFS PB Cache
    /// `mem.vmfs.pbc.workingSet.latest` · teraBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const VMFS_PBC_WORKING_SET_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "vmfs.pbc.workingSet",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Maximum amount of file blocks whose addresses are cached in the VMFS PB Cache
    /// `mem.vmfs.pbc.workingSetMax.latest` · teraBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const VMFS_PBC_WORKING_SET_MAX_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "vmfs.pbc.workingSetMax",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Desired amount of guest physical memory the balloon driver needs to reclaim, as determined by ESXi
    /// `mem.vmmemctltarget.average` · kiloBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const VMMEMCTLTARGET_AVERAGE: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "vmmemctltarget",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Desired amount of guest physical memory the balloon driver needs to reclaim, as determined by ESXi
    /// `mem.vmmemctltarget.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: VirtualMachine
    pub const VMMEMCTLTARGET_MAXIMUM: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "vmmemctltarget",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Desired amount of guest physical memory the balloon driver needs to reclaim, as determined by ESXi
    /// `mem.vmmemctltarget.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: VirtualMachine
    pub const VMMEMCTLTARGET_MINIMUM: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "vmmemctltarget",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Desired amount of guest physical memory the balloon driver needs to reclaim, as determined by ESXi
    /// `mem.vmmemctltarget.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: VirtualMachine
    pub const VMMEMCTLTARGET_NONE: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "vmmemctltarget",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory reclaimed from the virtual machine by the balloon driver in the guest
    /// `mem.vmmemctl.average` · kiloBytes · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const VMMEMCTL_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "vmmemctl",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Amount of guest physical memory reclaimed from the virtual machine by the balloon driver in the guest
    /// `mem.vmmemctl.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const VMMEMCTL_MAXIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "vmmemctl",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory reclaimed from the virtual machine by the balloon driver in the guest
    /// `mem.vmmemctl.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const VMMEMCTL_MINIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "vmmemctl",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory reclaimed from the virtual machine by the balloon driver in the guest
    /// `mem.vmmemctl.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const VMMEMCTL_NONE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "vmmemctl",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.vm.bandwidth.read.latest` · kiloBytesPerSecond · vCenter only
    /// Level: 2, Per-device level: 2 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const VM_BANDWIDTH_READ_LATEST: Metric<Throughput, ComputeScope> = Metric::new(
        "mem",
        "vm.bandwidth.read",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        false,
    );

    /// Guest physical memory pages whose content is 0x00
    /// `mem.zero.average` · kiloBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const ZERO_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "zero",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Guest physical memory pages whose content is 0x00
    /// `mem.zero.maximum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const ZERO_MAXIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "zero",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Guest physical memory pages whose content is 0x00
    /// `mem.zero.minimum` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const ZERO_MINIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "zero",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Guest physical memory pages whose content is 0x00
    /// `mem.zero.none` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const ZERO_NONE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "zero",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of guest physical memory pages compressed by ESXi
    /// `mem.zipped.latest` · kiloBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const ZIPPED_LATEST: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "zipped",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Host physical memory, reclaimed from a virtual machine, by memory compression. This value is less than the value of 'Compressed' memory
    /// `mem.zipSaved.latest` · kiloBytes · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const ZIPSAVED_LATEST: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "zipSaved",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Saved memory by compression
    /// `mem.zip.saved.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const ZIP_SAVED_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "zip.saved",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total compressed physical memory
    /// `mem.zip.zipped.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const ZIP_ZIPPED_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "zip.zipped",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );
}

pub mod net {
    use super::super::*;

    /// Number of broadcast packets received during the sampling interval
    /// `net.broadcastRx.summation` · number · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const BROADCAST_RX_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "broadcastRx",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Number of broadcast packets transmitted during the sampling interval
    /// `net.broadcastTx.summation` · number · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const BROADCAST_TX_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "broadcastTx",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Average amount of data received per second
    /// `net.bytesRx.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const BYTES_RX_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "bytesRx",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Average amount of data transmitted per second
    /// `net.bytesTx.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const BYTES_TX_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "bytesTx",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Number of receives dropped
    /// `net.droppedRx.summation` · number · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const DROPPED_RX_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "droppedRx",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Number of transmits dropped
    /// `net.droppedTx.summation` · number · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const DROPPED_TX_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "droppedTx",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Number of packets with errors received during the sampling interval
    /// `net.errorsRx.summation` · number · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const ERRORS_RX_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "net",
        "errorsRx",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Number of packets with errors transmitted during the sampling interval
    /// `net.errorsTx.summation` · number · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const ERRORS_TX_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "net",
        "errorsTx",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Average latency seen by vSphere Replication
    /// `net.latency.hbr.outbound.latest` · millisecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const LATENCY_HBR_OUTBOUND_LATEST: Metric<Time, VmOrHost> = Metric::new(
        "net",
        "latency.hbr.outbound",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Number of multicast packets received during the sampling interval
    /// `net.multicastRx.summation` · number · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const MULTICAST_RX_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "multicastRx",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Number of multicast packets transmitted during the sampling interval
    /// `net.multicastTx.summation` · number · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const MULTICAST_TX_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "multicastTx",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Number of packets received during the interval
    /// `net.packetsRx.summation` · number · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const PACKETS_RX_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "packetsRx",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Number of packets transmitted during the interval
    /// `net.packetsTx.summation` · number · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const PACKETS_TX_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "packetsTx",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Average amount of data received per second by a pNic
    /// `net.pnicBytesRx.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const PNIC_BYTES_RX_AVERAGE: Metric<Throughput, VmOnly> = Metric::new(
        "net",
        "pnicBytesRx",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// Average amount of data transmitted per second through a pNic
    /// `net.pnicBytesTx.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const PNIC_BYTES_TX_AVERAGE: Metric<Throughput, VmOnly> = Metric::new(
        "net",
        "pnicBytesTx",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// Average rate at which data was received during the interval
    /// `net.received.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const RECEIVED_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "received",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// The aggregate network droppped packets for the host
    /// `net.throughput.contention.summation` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_CONTENTION_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.contention",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Count of dropped received packets for this VDS
    /// `net.throughput.droppedRx.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_DROPPED_RX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.droppedRx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Count of dropped transmitted packets for this VDS
    /// `net.throughput.droppedTx.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_DROPPED_TX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.droppedTx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Average amount of data received per second
    /// `net.throughput.hbr.inbound.average` · kiloBytesPerSecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_HBR_INBOUND_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.hbr.inbound",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average amount of data transmitted per second
    /// `net.throughput.hbr.outbound.average` · kiloBytesPerSecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_HBR_OUTBOUND_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.hbr.outbound",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average rate of packets received and transmitted per second
    /// `net.throughput.packetsPerSec.average` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_PACKETS_PER_SEC_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.packetsPerSec",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The rate of received packets for this vDS
    /// `net.throughput.pktsRx.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_PKTS_RX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.pktsRx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The rate of received Broadcast packets for this VDS
    /// `net.throughput.pktsRxBroadcast.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_PKTS_RX_BROADCAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.pktsRxBroadcast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The rate of received Multicast packets for this VDS
    /// `net.throughput.pktsRxMulticast.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_PKTS_RX_MULTICAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.pktsRxMulticast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The rate of transmitted packets for this VDS
    /// `net.throughput.pktsTx.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_PKTS_TX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.pktsTx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The rate of transmitted Broadcast packets for this VDS
    /// `net.throughput.pktsTxBroadcast.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_PKTS_TX_BROADCAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.pktsTxBroadcast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The rate of transmitted Multicast packets for this VDS
    /// `net.throughput.pktsTxMulticast.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_PKTS_TX_MULTICAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.pktsTxMulticast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The maximum network bandwidth for the host
    /// `net.throughput.provisioned.average` · kiloBytesPerSecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_PROVISIONED_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.provisioned",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The current available network bandwidth for the host
    /// `net.throughput.usable.average` · kiloBytesPerSecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_USABLE_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.usable",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The current network bandwidth usage for the host
    /// `net.throughput.usage.average` · kiloBytesPerSecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_USAGE_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.usage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average pNic I/O rate for FT
    /// `net.throughput.usage.ft.average` · kiloBytesPerSecond · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_USAGE_FT_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.usage.ft",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Average pNic I/O rate for HBR
    /// `net.throughput.usage.hbr.average` · kiloBytesPerSecond · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_USAGE_HBR_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.usage.hbr",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Average pNic I/O rate for iSCSI
    /// `net.throughput.usage.iscsi.average` · kiloBytesPerSecond · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_USAGE_ISCSI_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.usage.iscsi",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Average pNic I/O rate for NFS
    /// `net.throughput.usage.nfs.average` · kiloBytesPerSecond · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_USAGE_NFS_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.usage.nfs",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Average pNic I/O rate for vMotion
    /// `net.throughput.usage.vmotion.average` · kiloBytesPerSecond · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_USAGE_VMOTION_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.usage.vmotion",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Average pNic I/O rate for VMs
    /// `net.throughput.usage.vm.average` · kiloBytesPerSecond · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_USAGE_VM_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.usage.vm",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Count of dropped received packets for this DVPort
    /// `net.throughput.vds.droppedRx.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_VDS_DROPPED_RX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.droppedRx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Count of dropped transmitted packets for this DVPort
    /// `net.throughput.vds.droppedTx.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_VDS_DROPPED_TX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.droppedTx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Count of dropped received packets for this LAG
    /// `net.throughput.vds.lagDropRx.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_VDS_LAG_DROP_RX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.lagDropRx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Count of dropped transmitted packets for this LAG
    /// `net.throughput.vds.lagDropTx.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_VDS_LAG_DROP_TX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.lagDropTx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The rate of received packets for this LAG
    /// `net.throughput.vds.lagRx.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_VDS_LAG_RX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.lagRx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The rate of received Broadcast packets for this LAG
    /// `net.throughput.vds.lagRxBcast.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_VDS_LAG_RX_BCAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.lagRxBcast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The rate of received multicast packets for this LAG
    /// `net.throughput.vds.lagRxMcast.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_VDS_LAG_RX_MCAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.lagRxMcast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The rate of transmitted packets for this LAG
    /// `net.throughput.vds.lagTx.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_VDS_LAG_TX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.lagTx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The rate of transmitted Broadcast packets for this LAG
    /// `net.throughput.vds.lagTxBcast.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_VDS_LAG_TX_BCAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.lagTxBcast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The rate of transmitted Multicast packets for this LAG
    /// `net.throughput.vds.lagTxMcast.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_VDS_LAG_TX_MCAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.lagTxMcast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The rate of received packets for this DVPort
    /// `net.throughput.vds.pktsRx.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_VDS_PKTS_RX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.pktsRx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The rate of received broadcast packets for this DVPort
    /// `net.throughput.vds.pktsRxBcast.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_VDS_PKTS_RX_BCAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.pktsRxBcast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The rate of received multicast packets for this DVPort
    /// `net.throughput.vds.pktsRxMcast.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_VDS_PKTS_RX_MCAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.pktsRxMcast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The rate of transmitted packets for this DVPort
    /// `net.throughput.vds.pktsTx.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_VDS_PKTS_TX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.pktsTx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The rate of transmitted broadcast packets for this DVPort
    /// `net.throughput.vds.pktsTxBcast.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_VDS_PKTS_TX_BCAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.pktsTxBcast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The rate of transmitted multicast packets for this DVPort
    /// `net.throughput.vds.pktsTxMcast.average` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const THROUGHPUT_VDS_PKTS_TX_MCAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.pktsTxMcast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Average rate at which data was transmitted during the interval
    /// `net.transmitted.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const TRANSMITTED_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "transmitted",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Number of frames with unknown protocol received during the sampling interval
    /// `net.unknownProtos.summation` · number · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const UNKNOWN_PROTOS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "net",
        "unknownProtos",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Network utilization (combined transmit-rates and receive-rates) during the interval
    /// `net.usage.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const USAGE_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "usage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// Network utilization (combined transmit-rates and receive-rates) during the interval
    /// `net.usage.maximum` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem, VirtualMachine
    pub const USAGE_MAXIMUM: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "usage",
        Rollup::Maximum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Network utilization (combined transmit-rates and receive-rates) during the interval
    /// `net.usage.minimum` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem, VirtualMachine
    pub const USAGE_MINIMUM: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "usage",
        Rollup::Minimum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Network utilization (combined transmit-rates and receive-rates) during the interval
    /// `net.usage.none` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem, VirtualMachine
    pub const USAGE_NONE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "usage",
        Rollup::None,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );
}

pub mod nfs {
    use super::super::*;

    /// APD state of the nfs volume
    /// `nfs.apdState.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const APD_STATE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "nfs",
        "apdState",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total create calls failed on NFS volume
    /// `nfs.createFailed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const CREATE_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "nfs",
        "createFailed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total IO requests queued in NFS volume
    /// `nfs.ioRequestsQueued.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const IO_REQUESTS_QUEUED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "nfs",
        "ioRequestsQueued",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total reads failed on NFS volume
    /// `nfs.readsFailed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const READS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "nfs",
        "readsFailed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Cumulative read issue time on NFS volume
    /// `nfs.readIssueTime.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const READ_ISSUE_TIME_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "nfs",
        "readIssueTime",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Cumulative readTime on NFS volume
    /// `nfs.readTime.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const READ_TIME_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "nfs",
        "readTime",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Number of times we hit into socket buffer out of space condition for NFS volume
    /// `nfs.socketBufferFull.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const SOCKET_BUFFER_FULL_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "nfs",
        "socketBufferFull",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total create calls on NFS volume
    /// `nfs.totalCreate.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const TOTAL_CREATE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "nfs",
        "totalCreate",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total reads on NFS volume
    /// `nfs.totalReads.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const TOTAL_READS_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "nfs",
        "totalReads",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total writes on NFS volume
    /// `nfs.totalWrites.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const TOTAL_WRITES_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "nfs",
        "totalWrites",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total writes failed on NFS volume
    /// `nfs.writesFailed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const WRITES_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "nfs",
        "writesFailed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Cumulative write issue time on NFS volume
    /// `nfs.writeIssueTime.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const WRITE_ISSUE_TIME_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "nfs",
        "writeIssueTime",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Cumulative writeTime on NFS volume
    /// `nfs.writeTime.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const WRITE_TIME_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "nfs",
        "writeTime",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );
}

pub mod pmem {
    use super::super::*;

    /// Persistent memory available reservation on a host.
    /// `pmem.available.reservation.latest` · megaBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem, VirtualMachine
    pub const AVAILABLE_RESERVATION_LATEST: Metric<Memory, VmOrHost> = Metric::new(
        "pmem",
        "available.reservation",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Persistent memory reservation managed by DRS on a host.
    /// `pmem.drsmanaged.reservation.latest` · megaBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem, VirtualMachine
    pub const DRSMANAGED_RESERVATION_LATEST: Metric<Memory, VmOrHost> = Metric::new(
        "pmem",
        "drsmanaged.reservation",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );
}

pub mod power {
    use super::super::*;

    /// Current maximum allowed power usage
    /// `power.capacity.usable.average` · watt · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_USABLE_AVERAGE: Metric<Power, ComputeScope> = Metric::new(
        "power",
        "capacity.usable",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Current power usage
    /// `power.capacity.usage.average` · watt · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_USAGE_AVERAGE: Metric<Power, ComputeScope> = Metric::new(
        "power",
        "capacity.usage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Current CPU power usage
    /// `power.capacity.usageCpu.average` · watt · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_USAGE_CPU_AVERAGE: Metric<Power, ComputeScope> = Metric::new(
        "power",
        "capacity.usageCpu",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Power usage due to host idleness
    /// `power.capacity.usageIdle.average` · watt · vCenter only
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const CAPACITY_USAGE_IDLE_AVERAGE: Metric<Power, HostOnly> = Metric::new(
        "power",
        "capacity.usageIdle",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Current memory power usage
    /// `power.capacity.usageMem.average` · watt · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_USAGE_MEM_AVERAGE: Metric<Power, ComputeScope> = Metric::new(
        "power",
        "capacity.usageMem",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Current other power usage
    /// `power.capacity.usageOther.average` · watt · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_USAGE_OTHER_AVERAGE: Metric<Power, ComputeScope> = Metric::new(
        "power",
        "capacity.usageOther",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Current power usage as a percentage of maximum allowed power
    /// `power.capacity.usagePct.average` · percent (hundredths) · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const CAPACITY_USAGE_PCT_AVERAGE: Metric<Ratio, ComputeScope> = Metric::new(
        "power",
        "capacity.usagePct",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Static power usage of VM
    /// `power.capacity.usageStatic.average` · watt · vCenter only
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const CAPACITY_USAGE_STATIC_AVERAGE: Metric<Power, VmOnly> = Metric::new(
        "power",
        "capacity.usageStatic",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Power usage due to non-VM activities
    /// `power.capacity.usageSystem.average` · watt · vCenter only
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const CAPACITY_USAGE_SYSTEM_AVERAGE: Metric<Power, HostOnly> = Metric::new(
        "power",
        "capacity.usageSystem",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Power usage due to VM workloads
    /// `power.capacity.usageVm.average` · watt · vCenter only
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const CAPACITY_USAGE_VM_AVERAGE: Metric<Power, HostOnly> = Metric::new(
        "power",
        "capacity.usageVm",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Total energy used since last stats reset
    /// `power.energy.summation` · joule · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const ENERGY_SUMMATION: Metric<Energy, VmOrHost> = Metric::new(
        "power",
        "energy",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Current power usage
    /// `power.power.average` · watt · vCenter and ESX
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, HostSystem, ResourcePool, VirtualMachine
    pub const POWER_AVERAGE: Metric<Power, ComputeScope> = Metric::new(
        "power",
        "power",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// Maximum allowed power usage
    /// `power.powerCap.average` · watt · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const POWER_CAP_AVERAGE: Metric<Power, HostOnly> = Metric::new(
        "power",
        "powerCap",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );
}

pub mod rescpu {
    use super::super::*;

    /// CPU active average over 15 minutes
    /// `rescpu.actav15.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const ACTAV15_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "actav15",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// CPU active average over 1 minute
    /// `rescpu.actav1.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const ACTAV1_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "actav1",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// CPU active average over 5 minutes
    /// `rescpu.actav5.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const ACTAV5_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "actav5",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// CPU active peak over 15 minutes
    /// `rescpu.actpk15.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const ACTPK15_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "actpk15",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// CPU active peak over 1 minute
    /// `rescpu.actpk1.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const ACTPK1_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "actpk1",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// CPU active peak over 5 minutes
    /// `rescpu.actpk5.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const ACTPK5_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "actpk5",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Amount of CPU resources over the limit that were refused, average over 15 minutes
    /// `rescpu.maxLimited15.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const MAX_LIMITED15_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "maxLimited15",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Amount of CPU resources over the limit that were refused, average over 1 minute
    /// `rescpu.maxLimited1.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const MAX_LIMITED1_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "maxLimited1",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Amount of CPU resources over the limit that were refused, average over 5 minutes
    /// `rescpu.maxLimited5.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const MAX_LIMITED5_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "maxLimited5",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// CPU running average over 15 minutes
    /// `rescpu.runav15.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const RUNAV15_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "runav15",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// CPU running average over 1 minute
    /// `rescpu.runav1.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const RUNAV1_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "runav1",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// CPU running average over 5 minutes
    /// `rescpu.runav5.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const RUNAV5_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "runav5",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// CPU running peak over 15 minutes
    /// `rescpu.runpk15.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const RUNPK15_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "runpk15",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// CPU running peak over 1 minute
    /// `rescpu.runpk1.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const RUNPK1_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "runpk1",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// CPU running peak over 5 minutes
    /// `rescpu.runpk5.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const RUNPK5_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "runpk5",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Group CPU sample count
    /// `rescpu.sampleCount.latest` · number · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const SAMPLE_COUNT_LATEST: Metric<Count, VmOrHost> = Metric::new(
        "rescpu",
        "sampleCount",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Group CPU sample period
    /// `rescpu.samplePeriod.latest` · millisecond · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const SAMPLE_PERIOD_LATEST: Metric<Time, VmOrHost> = Metric::new(
        "rescpu",
        "samplePeriod",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );
}

pub mod storage_adapter {
    use super::super::*;

    /// Average number of commands issued per second by the storage adapter during the collection interval
    /// `storageAdapter.commandsAveraged.average` · number · vCenter and ESX
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: HostSystem
    pub const COMMANDS_AVERAGED_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "storageAdapter",
        "commandsAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// Highest latency value across all storage adapters used by the host
    /// `storageAdapter.maxTotalLatency.latest` · millisecond · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const MAX_TOTAL_LATENCY_LATEST: Metric<Time, HostOnly> = Metric::new(
        "storageAdapter",
        "maxTotalLatency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Average number of read commands issued per second by the storage adapter during the collection interval
    /// `storageAdapter.numberReadAveraged.average` · number · vCenter and ESX
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: HostSystem
    pub const NUMBER_READ_AVERAGED_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "storageAdapter",
        "numberReadAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// Average number of write commands issued per second by the storage adapter during the collection interval
    /// `storageAdapter.numberWriteAveraged.average` · number · vCenter and ESX
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: HostSystem
    pub const NUMBER_WRITE_AVERAGED_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "storageAdapter",
        "numberWriteAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// The percent of I/Os that have been issued but have not yet completed
    /// `storageAdapter.OIOsPct.average` · percent (hundredths) · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem
    pub const OIOS_PCT_AVERAGE: Metric<Ratio, HostOnly> = Metric::new(
        "storageAdapter",
        "OIOsPct",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The number of I/Os that have been issued but have not yet completed
    /// `storageAdapter.outstandingIOs.average` · number · vCenter only
    /// Level: 2, Per-device level: 2 · Tier: Group Union (3a)
    /// Types: HostSystem
    pub const OUTSTANDING_IOS_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "storageAdapter",
        "outstandingIOs",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        false,
    );

    /// The number of I/Os that have been issued but have not yet completed
    /// `storageAdapter.outstandingIOs.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem
    pub const OUTSTANDING_IOS_LATEST: Metric<Count, HostOnly> = Metric::new(
        "storageAdapter",
        "outstandingIOs",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The current number of I/Os that are waiting to be issued
    /// `storageAdapter.queued.average` · number · vCenter only
    /// Level: 2, Per-device level: 2 · Tier: Group Union (3a)
    /// Types: HostSystem
    pub const QUEUED_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "storageAdapter",
        "queued",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        false,
    );

    /// The current number of I/Os that are waiting to be issued
    /// `storageAdapter.queued.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem
    pub const QUEUED_LATEST: Metric<Count, HostOnly> = Metric::new(
        "storageAdapter",
        "queued",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The maximum number of I/Os that can be outstanding at a given time
    /// `storageAdapter.queueDepth.average` · number · vCenter only
    /// Level: 2, Per-device level: 2 · Tier: Group Union (3a)
    /// Types: HostSystem
    pub const QUEUE_DEPTH_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "storageAdapter",
        "queueDepth",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        false,
    );

    /// The maximum number of I/Os that can be outstanding at a given time
    /// `storageAdapter.queueDepth.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem
    pub const QUEUE_DEPTH_LATEST: Metric<Count, HostOnly> = Metric::new(
        "storageAdapter",
        "queueDepth",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average amount of time spent in the VMkernel queue, per Storage command, during the collection interval
    /// `storageAdapter.queueLatency.average` · millisecond · vCenter only
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: HostSystem
    pub const QUEUE_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "storageAdapter",
        "queueLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// Rate of reading data by the storage adapter
    /// `storageAdapter.read.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: HostSystem
    pub const READ_AVERAGE: Metric<Throughput, HostOnly> = Metric::new(
        "storageAdapter",
        "read",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// Average amount of time for an I/O operation to complete successfully
    /// `storageAdapter.throughput.cont.average` · millisecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem
    pub const THROUGHPUT_CONT_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "storageAdapter",
        "throughput.cont",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The storage adapter's I/O rate
    /// `storageAdapter.throughput.usag.average` · kiloBytesPerSecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem
    pub const THROUGHPUT_USAG_AVERAGE: Metric<Throughput, HostOnly> = Metric::new(
        "storageAdapter",
        "throughput.usag",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The average time a read by the storage adapter takes
    /// `storageAdapter.totalReadLatency.average` · millisecond · vCenter and ESX
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: HostSystem
    pub const TOTAL_READ_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "storageAdapter",
        "totalReadLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// The average time a write by the storage adapter takes
    /// `storageAdapter.totalWriteLatency.average` · millisecond · vCenter and ESX
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: HostSystem
    pub const TOTAL_WRITE_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "storageAdapter",
        "totalWriteLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// Rate of writing data by the storage adapter
    /// `storageAdapter.write.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: HostSystem
    pub const WRITE_AVERAGE: Metric<Throughput, HostOnly> = Metric::new(
        "storageAdapter",
        "write",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(2u8),
        true,
    );
}

pub mod storage_path {
    use super::super::*;

    /// Number of Storage-bus reset commands issued during the collection interval
    /// `storagePath.busResets.summation` · number · vCenter only
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const BUS_RESETS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "storagePath",
        "busResets",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Number of Storage commands terminated during the collection interval
    /// `storagePath.commandsAborted.summation` · number · vCenter only
    /// Level: 2, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const COMMANDS_ABORTED_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "storagePath",
        "commandsAborted",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// Average number of commands issued per second on the storage path during the collection interval
    /// `storagePath.commandsAveraged.average` · number · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const COMMANDS_AVERAGED_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "storagePath",
        "commandsAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Highest latency value across all storage paths used by the host
    /// `storagePath.maxTotalLatency.latest` · millisecond · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const MAX_TOTAL_LATENCY_LATEST: Metric<Time, HostOnly> = Metric::new(
        "storagePath",
        "maxTotalLatency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Average number of read commands issued per second on the storage path during the collection interval
    /// `storagePath.numberReadAveraged.average` · number · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const NUMBER_READ_AVERAGED_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "storagePath",
        "numberReadAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Average number of write commands issued per second on the storage path during the collection interval
    /// `storagePath.numberWriteAveraged.average` · number · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const NUMBER_WRITE_AVERAGED_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "storagePath",
        "numberWriteAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Rate of reading data on the storage path
    /// `storagePath.read.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const READ_AVERAGE: Metric<Throughput, HostOnly> = Metric::new(
        "storagePath",
        "read",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Average amount of time for an I/O operation to complete successfully
    /// `storagePath.throughput.cont.average` · millisecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem
    pub const THROUGHPUT_CONT_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "storagePath",
        "throughput.cont",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Storage path I/O rate
    /// `storagePath.throughput.usage.average` · kiloBytesPerSecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: HostSystem
    pub const THROUGHPUT_USAGE_AVERAGE: Metric<Throughput, HostOnly> = Metric::new(
        "storagePath",
        "throughput.usage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The average time a read issued on the storage path takes
    /// `storagePath.totalReadLatency.average` · millisecond · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const TOTAL_READ_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "storagePath",
        "totalReadLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// The average time a write issued on the storage path takes
    /// `storagePath.totalWriteLatency.average` · millisecond · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const TOTAL_WRITE_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "storagePath",
        "totalWriteLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Rate of writing data on the storage path
    /// `storagePath.write.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const WRITE_AVERAGE: Metric<Throughput, HostOnly> = Metric::new(
        "storagePath",
        "write",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        true,
    );
}

pub mod sys {
    use super::super::*;

    /// Amount of disk space usage for each mount point
    /// `sys.diskUsage.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Group Union (3a)
    /// Types: HostSystem, VirtualMachine
    pub const DISK_USAGE_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "sys",
        "diskUsage",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// Number of heartbeats issued per virtual machine during the interval
    /// `sys.heartbeat.latest` · number · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const HEARTBEAT_LATEST: Metric<Count, VmOnly> = Metric::new(
        "sys",
        "heartbeat",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Number of heartbeats issued per virtual machine during the interval
    /// `sys.heartbeat.summation` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Sibling Rollup (2)
    /// Types: VirtualMachine
    pub const HEARTBEAT_SUMMATION: Metric<Count, VmOnly> = Metric::new(
        "sys",
        "heartbeat",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Total time elapsed, in seconds, since last operating system boot-up
    /// `sys.osUptime.latest` · second · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const OS_UPTIME_LATEST: Metric<Time, VmOnly> = Metric::new(
        "sys",
        "osUptime",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// CPU active average over 1 minute of the system resource group
    /// `sys.resourceCpuAct1.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_CPU_ACT1_LATEST: Metric<Ratio, HostOnly> = Metric::new(
        "sys",
        "resourceCpuAct1",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// CPU active average over 5 minutes of the system resource group
    /// `sys.resourceCpuAct5.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_CPU_ACT5_LATEST: Metric<Ratio, HostOnly> = Metric::new(
        "sys",
        "resourceCpuAct5",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// CPU allocation limit (in MHz) of the system resource group
    /// `sys.resourceCpuAllocMax.latest` · megaHertz · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_CPU_ALLOC_MAX_LATEST: Metric<Frequency, HostOnly> = Metric::new(
        "sys",
        "resourceCpuAllocMax",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// CPU allocation reservation (in MHz) of the system resource group
    /// `sys.resourceCpuAllocMin.latest` · megaHertz · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_CPU_ALLOC_MIN_LATEST: Metric<Frequency, HostOnly> = Metric::new(
        "sys",
        "resourceCpuAllocMin",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// CPU allocation shares of the system resource group
    /// `sys.resourceCpuAllocShares.latest` · number · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_CPU_ALLOC_SHARES_LATEST: Metric<Count, HostOnly> = Metric::new(
        "sys",
        "resourceCpuAllocShares",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// CPU maximum limited over 1 minute of the system resource group
    /// `sys.resourceCpuMaxLimited1.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_CPU_MAX_LIMITED1_LATEST: Metric<Ratio, HostOnly> = Metric::new(
        "sys",
        "resourceCpuMaxLimited1",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// CPU maximum limited over 5 minutes of the system resource group
    /// `sys.resourceCpuMaxLimited5.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_CPU_MAX_LIMITED5_LATEST: Metric<Ratio, HostOnly> = Metric::new(
        "sys",
        "resourceCpuMaxLimited5",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// CPU running average over 1 minute of the system resource group
    /// `sys.resourceCpuRun1.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_CPU_RUN1_LATEST: Metric<Ratio, HostOnly> = Metric::new(
        "sys",
        "resourceCpuRun1",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// CPU running average over 5 minutes of the system resource group
    /// `sys.resourceCpuRun5.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_CPU_RUN5_LATEST: Metric<Ratio, HostOnly> = Metric::new(
        "sys",
        "resourceCpuRun5",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Amount of CPU used by the Service Console and other applications during the interval
    /// `sys.resourceCpuUsage.average` · megaHertz · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_CPU_USAGE_AVERAGE: Metric<Frequency, HostOnly> = Metric::new(
        "sys",
        "resourceCpuUsage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Amount of CPU used by the Service Console and other applications during the interval
    /// `sys.resourceCpuUsage.maximum` · megaHertz · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const RESOURCE_CPU_USAGE_MAXIMUM: Metric<Frequency, HostOnly> = Metric::new(
        "sys",
        "resourceCpuUsage",
        Rollup::Maximum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of CPU used by the Service Console and other applications during the interval
    /// `sys.resourceCpuUsage.minimum` · megaHertz · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const RESOURCE_CPU_USAGE_MINIMUM: Metric<Frequency, HostOnly> = Metric::new(
        "sys",
        "resourceCpuUsage",
        Rollup::Minimum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Amount of CPU used by the Service Console and other applications during the interval
    /// `sys.resourceCpuUsage.none` · megaHertz · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Sibling Rollup (2)
    /// Types: HostSystem
    pub const RESOURCE_CPU_USAGE_NONE: Metric<Frequency, HostOnly> = Metric::new(
        "sys",
        "resourceCpuUsage",
        Rollup::None,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Number of file descriptors used by the system resource group
    /// `sys.resourceFdUsage.latest` · number · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_FD_USAGE_LATEST: Metric<Count, HostOnly> = Metric::new(
        "sys",
        "resourceFdUsage",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// Memory allocation limit (in KB) of the system resource group
    /// `sys.resourceMemAllocMax.latest` · kiloBytes · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_MEM_ALLOC_MAX_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "sys",
        "resourceMemAllocMax",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Memory allocation reservation (in KB) of the system resource group
    /// `sys.resourceMemAllocMin.latest` · kiloBytes · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_MEM_ALLOC_MIN_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "sys",
        "resourceMemAllocMin",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Memory allocation shares of the system resource group
    /// `sys.resourceMemAllocShares.latest` · number · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_MEM_ALLOC_SHARES_LATEST: Metric<Count, HostOnly> = Metric::new(
        "sys",
        "resourceMemAllocShares",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Memory consumed by the system resource group
    /// `sys.resourceMemConsumed.latest` · kiloBytes · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_MEM_CONSUMED_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "sys",
        "resourceMemConsumed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// Memory shared by the system resource group
    /// `sys.resourceMemCow.latest` · kiloBytes · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_MEM_COW_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "sys",
        "resourceMemCow",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Memory mapped by the system resource group
    /// `sys.resourceMemMapped.latest` · kiloBytes · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_MEM_MAPPED_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "sys",
        "resourceMemMapped",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Overhead memory consumed by the system resource group
    /// `sys.resourceMemOverhead.latest` · kiloBytes · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_MEM_OVERHEAD_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "sys",
        "resourceMemOverhead",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Memory saved due to sharing by the system resource group
    /// `sys.resourceMemShared.latest` · kiloBytes · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_MEM_SHARED_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "sys",
        "resourceMemShared",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Memory swapped out by the system resource group
    /// `sys.resourceMemSwapped.latest` · kiloBytes · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_MEM_SWAPPED_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "sys",
        "resourceMemSwapped",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Memory touched by the system resource group
    /// `sys.resourceMemTouched.latest` · kiloBytes · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_MEM_TOUCHED_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "sys",
        "resourceMemTouched",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Zero filled memory used by the system resource group
    /// `sys.resourceMemZero.latest` · kiloBytes · vCenter and ESX
    /// Level: 3, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem
    pub const RESOURCE_MEM_ZERO_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "sys",
        "resourceMemZero",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// Total time elapsed, in seconds, since last system startup
    /// `sys.uptime.latest` · second · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: HostSystem, VirtualMachine
    pub const UPTIME_LATEST: Metric<Time, VmOrHost> = Metric::new(
        "sys",
        "uptime",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );
}

pub mod vc_debug_info {
    use super::super::*;

    /// The latency of an activation operation in vCenter Server
    /// `vcDebugInfo.activationlatencystats.maximum` · millisecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const ACTIVATIONLATENCYSTATS_MAXIMUM: Metric<Time, HostOnly> = Metric::new(
        "vcDebugInfo",
        "activationlatencystats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The latency of an activation operation in vCenter Server
    /// `vcDebugInfo.activationlatencystats.minimum` · millisecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const ACTIVATIONLATENCYSTATS_MINIMUM: Metric<Time, HostOnly> = Metric::new(
        "vcDebugInfo",
        "activationlatencystats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The latency of an activation operation in vCenter Server
    /// `vcDebugInfo.activationlatencystats.summation` · millisecond · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const ACTIVATIONLATENCYSTATS_SUMMATION: Metric<Time, HostOnly> = Metric::new(
        "vcDebugInfo",
        "activationlatencystats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// Activation operations in vCenter Server
    /// `vcDebugInfo.activationstats.maximum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const ACTIVATIONSTATS_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "activationstats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Activation operations in vCenter Server
    /// `vcDebugInfo.activationstats.minimum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const ACTIVATIONSTATS_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "activationstats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Activation operations in vCenter Server
    /// `vcDebugInfo.activationstats.summation` · number · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const ACTIVATIONSTATS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "activationstats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// The latency of a host sync operation in vCenter Server
    /// `vcDebugInfo.hostsynclatencystats.maximum` · millisecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const HOSTSYNCLATENCYSTATS_MAXIMUM: Metric<Time, HostOnly> = Metric::new(
        "vcDebugInfo",
        "hostsynclatencystats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The latency of a host sync operation in vCenter Server
    /// `vcDebugInfo.hostsynclatencystats.minimum` · millisecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const HOSTSYNCLATENCYSTATS_MINIMUM: Metric<Time, HostOnly> = Metric::new(
        "vcDebugInfo",
        "hostsynclatencystats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The latency of a host sync operation in vCenter Server
    /// `vcDebugInfo.hostsynclatencystats.summation` · millisecond · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const HOSTSYNCLATENCYSTATS_SUMMATION: Metric<Time, HostOnly> = Metric::new(
        "vcDebugInfo",
        "hostsynclatencystats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// The number of host sync operations in vCenter Server
    /// `vcDebugInfo.hostsyncstats.maximum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const HOSTSYNCSTATS_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "hostsyncstats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The number of host sync operations in vCenter Server
    /// `vcDebugInfo.hostsyncstats.minimum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const HOSTSYNCSTATS_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "hostsyncstats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The number of host sync operations in vCenter Server
    /// `vcDebugInfo.hostsyncstats.summation` · number · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const HOSTSYNCSTATS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "hostsyncstats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// vCenter Server inventory statistics
    /// `vcDebugInfo.inventorystats.maximum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const INVENTORYSTATS_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "inventorystats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// vCenter Server inventory statistics
    /// `vcDebugInfo.inventorystats.minimum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const INVENTORYSTATS_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "inventorystats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// vCenter Server inventory statistics
    /// `vcDebugInfo.inventorystats.summation` · number · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const INVENTORYSTATS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "inventorystats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// vCenter Server locking statistics
    /// `vcDebugInfo.lockstats.maximum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const LOCKSTATS_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "lockstats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// vCenter Server locking statistics
    /// `vcDebugInfo.lockstats.minimum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const LOCKSTATS_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "lockstats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// vCenter Server locking statistics
    /// `vcDebugInfo.lockstats.summation` · number · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const LOCKSTATS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "lockstats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// vCenter Server LRO statistics
    /// `vcDebugInfo.lrostats.maximum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const LROSTATS_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "lrostats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// vCenter Server LRO statistics
    /// `vcDebugInfo.lrostats.minimum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const LROSTATS_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "lrostats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// vCenter Server LRO statistics
    /// `vcDebugInfo.lrostats.summation` · number · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const LROSTATS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "lrostats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// Miscellaneous statistics
    /// `vcDebugInfo.miscstats.maximum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const MISCSTATS_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "miscstats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Miscellaneous statistics
    /// `vcDebugInfo.miscstats.minimum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const MISCSTATS_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "miscstats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Miscellaneous statistics
    /// `vcDebugInfo.miscstats.summation` · number · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const MISCSTATS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "miscstats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// Managed object reference counts in vCenter Server
    /// `vcDebugInfo.morefregstats.maximum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const MOREFREGSTATS_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "morefregstats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Managed object reference counts in vCenter Server
    /// `vcDebugInfo.morefregstats.minimum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const MOREFREGSTATS_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "morefregstats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Managed object reference counts in vCenter Server
    /// `vcDebugInfo.morefregstats.summation` · number · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const MOREFREGSTATS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "morefregstats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// Object counts in vCenter Server
    /// `vcDebugInfo.scoreboard.maximum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const SCOREBOARD_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "scoreboard",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Object counts in vCenter Server
    /// `vcDebugInfo.scoreboard.minimum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const SCOREBOARD_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "scoreboard",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Object counts in vCenter Server
    /// `vcDebugInfo.scoreboard.summation` · number · vCenter only
    /// Level: 3, Per-device level: 3 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const SCOREBOARD_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "scoreboard",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// The statistics of client sessions connected to vCenter Server
    /// `vcDebugInfo.sessionstats.maximum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const SESSIONSTATS_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "sessionstats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The statistics of client sessions connected to vCenter Server
    /// `vcDebugInfo.sessionstats.minimum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const SESSIONSTATS_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "sessionstats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The statistics of client sessions connected to vCenter Server
    /// `vcDebugInfo.sessionstats.summation` · number · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const SESSIONSTATS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "sessionstats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// The statistics of vCenter Server as a running system such as thread statistics and heap statistics
    /// `vcDebugInfo.systemstats.maximum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const SYSTEMSTATS_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "systemstats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The statistics of vCenter Server as a running system such as thread statistics and heap statistics
    /// `vcDebugInfo.systemstats.minimum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const SYSTEMSTATS_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "systemstats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The statistics of vCenter Server as a running system such as thread statistics and heap statistics
    /// `vcDebugInfo.systemstats.summation` · number · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const SYSTEMSTATS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "systemstats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// vCenter service statistics such as events, alarms, and tasks
    /// `vcDebugInfo.vcservicestats.maximum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const VCSERVICESTATS_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "vcservicestats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// vCenter service statistics such as events, alarms, and tasks
    /// `vcDebugInfo.vcservicestats.minimum` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const VCSERVICESTATS_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "vcservicestats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// vCenter service statistics such as events, alarms, and tasks
    /// `vcDebugInfo.vcservicestats.summation` · number · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const VCSERVICESTATS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "vcservicestats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );
}

pub mod vc_resources {
    use super::super::*;

    /// Total size of in-memory cache of blocks (buffer cache) read in from block devices (i.e., disk devices) on the system where vCenter Server is running
    /// `vcResources.buffersz.average` · kiloBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const BUFFERSZ_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "vcResources",
        "buffersz",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total size of in-memory caches of pages (page cache) for files from on-disk and in-memory filesystems on the system where vCenter Server is running
    /// `vcResources.cachesz.average` · kiloBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const CACHESZ_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "vcResources",
        "cachesz",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Number of context switches per second on the system where vCenter Server is running
    /// `vcResources.ctxswitchesrate.average` · number · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const CTXSWITCHESRATE_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "vcResources",
        "ctxswitchesrate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// Disk sectors read per second over last sampling interval (typically 60 seconds) on the system where vCenter Server is running
    /// `vcResources.diskreadsectorrate.average` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const DISKREADSECTORRATE_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "vcResources",
        "diskreadsectorrate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Number of disk reads per second on the system where vCenter Server is running
    /// `vcResources.diskreadsrate.average` · number · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const DISKREADSRATE_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "vcResources",
        "diskreadsrate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// Disk sectors written per second over last sampling interval (typically 60 seconds) on the system where vCenter Server is running
    /// `vcResources.diskwritesectorrate.average` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const DISKWRITESECTORRATE_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "vcResources",
        "diskwritesectorrate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Number of disk writes per second on the system where vCenter Server is running
    /// `vcResources.diskwritesrate.average` · number · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const DISKWRITESRATE_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "vcResources",
        "diskwritesrate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// Rate of the number of total packets received per second on the system where vCenter Server is running
    /// `vcResources.packetrecvrate.average` · number · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const PACKETRECVRATE_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "vcResources",
        "packetrecvrate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// Number of total packets sent per second on the system where vCenter Server is running
    /// `vcResources.packetsentrate.average` · number · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const PACKETSENTRATE_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "vcResources",
        "packetsentrate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// Number of page faults per second on the system where vCenter Server is running
    /// `vcResources.pagefaultrate.average` · number · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const PAGEFAULTRATE_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "vcResources",
        "pagefaultrate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// Physical memory used by vCenter
    /// `vcResources.physicalmemusage.average` · kiloBytes · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const PHYSICALMEMUSAGE_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "vcResources",
        "physicalmemusage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// CPU used by vCenter Server in privileged mode
    /// `vcResources.priviledgedcpuusage.average` · percent (hundredths) · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const PRIVILEDGEDCPUUSAGE_AVERAGE: Metric<Ratio, HostOnly> = Metric::new(
        "vcResources",
        "priviledgedcpuusage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// Number of systems calls made per second on the system where vCenter Server is running
    /// `vcResources.syscallsrate.average` · number · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const SYSCALLSRATE_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "vcResources",
        "syscallsrate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// Total system CPU used on the system where vCenter Server in running
    /// `vcResources.systemcpuusage.average` · percent (hundredths) · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const SYSTEMCPUUSAGE_AVERAGE: Metric<Ratio, HostOnly> = Metric::new(
        "vcResources",
        "systemcpuusage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// CPU used by vCenter Server in user mode
    /// `vcResources.usercpuusage.average` · percent (hundredths) · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const USERCPUUSAGE_AVERAGE: Metric<Ratio, HostOnly> = Metric::new(
        "vcResources",
        "usercpuusage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// Virtual memory used by vCenter Server
    /// `vcResources.virtualmemusage.average` · kiloBytes · vCenter only
    /// Level: 1, Per-device level: 1 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const VIRTUALMEMUSAGE_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "vcResources",
        "virtualmemusage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );
}

pub mod vflash_module {
    use super::super::*;

    /// Number of caches controlled by the virtual flash module
    /// `vflashModule.numActiveVMDKs.latest` · number · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const NUM_ACTIVE_VMDKS_LATEST: Metric<Count, HostOnly> = Metric::new(
        "vflashModule",
        "numActiveVMDKs",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );
}

pub mod virtual_disk {
    use super::super::*;

    /// Number of resets to a virtual disk
    /// `virtualDisk.busResets.summation` · number · vCenter only
    /// Level: 2, Per-device level: 4 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const BUS_RESETS_SUMMATION: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "busResets",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(4u8),
        true,
    );

    /// Number of terminations to a virtual disk
    /// `virtualDisk.commandsAborted.summation` · number · vCenter only
    /// Level: 2, Per-device level: 4 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const COMMANDS_ABORTED_SUMMATION: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "commandsAborted",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(4u8),
        true,
    );

    /// Average disk read latency seen by vSphere Replication
    /// `virtualDisk.hbr.readLatencyMS.latest` · millisecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: VirtualMachine
    pub const HBR_READ_LATENCY_MS_LATEST: Metric<Time, VmOnly> = Metric::new(
        "virtualDisk",
        "hbr.readLatencyMS",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average guest I/O stall introduced by vSphere Replication
    /// `virtualDisk.hbr.stallLatencyMS.latest` · millisecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: VirtualMachine
    pub const HBR_STALL_LATENCY_MS_LATEST: Metric<Time, VmOnly> = Metric::new(
        "virtualDisk",
        "hbr.stallLatencyMS",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Number of seeks during the interval that were greater than 8192 LBNs apart
    /// `virtualDisk.largeSeeks.latest` · number · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const LARGE_SEEKS_LATEST: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "largeSeeks",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// Number of seeks during the interval that were between 64 and 8192 LBNs apart
    /// `virtualDisk.mediumSeeks.latest` · number · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const MEDIUM_SEEKS_LATEST: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "mediumSeeks",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// Average number of read commands issued per second to the virtual disk during the collection interval
    /// `virtualDisk.numberReadAveraged.average` · number · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const NUMBER_READ_AVERAGED_AVERAGE: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "numberReadAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// Average number of write commands issued per second to the virtual disk during the collection interval
    /// `virtualDisk.numberWriteAveraged.average` · number · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const NUMBER_WRITE_AVERAGED_AVERAGE: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "numberWriteAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// Rate of reading data from the virtual disk
    /// `virtualDisk.read.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const READ_AVERAGE: Metric<Throughput, VmOnly> = Metric::new(
        "virtualDisk",
        "read",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// Average read request size in bytes
    /// `virtualDisk.readIOSize.latest` · number · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const READ_IOSIZE_LATEST: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "readIOSize",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// Read latency in microseconds
    /// `virtualDisk.readLatencyUS.latest` · microsecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const READ_LATENCY_US_LATEST: Metric<Time, VmOnly> = Metric::new(
        "virtualDisk",
        "readLatencyUS",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// Storage DRS virtual disk metric for the read workload model
    /// `virtualDisk.readLoadMetric.latest` · number · vCenter and ESX
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const READ_LOAD_METRIC_LATEST: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "readLoadMetric",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// Average number of outstanding read requests to the virtual disk during the collection interval
    /// `virtualDisk.readOIO.latest` · number · vCenter and ESX
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const READ_OIO_LATEST: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "readOIO",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// Number of seeks during the interval that were less than 64 LBNs apart
    /// `virtualDisk.smallSeeks.latest` · number · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const SMALL_SEEKS_LATEST: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "smallSeeks",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// Average amount of time for an I/O operation to complete successfully
    /// `virtualDisk.throughput.cont.average` · millisecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: VirtualMachine
    pub const THROUGHPUT_CONT_AVERAGE: Metric<Time, VmOnly> = Metric::new(
        "virtualDisk",
        "throughput.cont",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Virtual disk I/O rate
    /// `virtualDisk.throughput.usage.average` · kiloBytesPerSecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Group Union (3a)
    /// Types: VirtualMachine
    pub const THROUGHPUT_USAGE_AVERAGE: Metric<Throughput, VmOnly> = Metric::new(
        "virtualDisk",
        "throughput.usage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// The average time a read from the virtual disk takes
    /// `virtualDisk.totalReadLatency.average` · millisecond · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const TOTAL_READ_LATENCY_AVERAGE: Metric<Time, VmOnly> = Metric::new(
        "virtualDisk",
        "totalReadLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// The average time a write to the virtual disk takes
    /// `virtualDisk.totalWriteLatency.average` · millisecond · vCenter and ESX
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const TOTAL_WRITE_LATENCY_AVERAGE: Metric<Time, VmOnly> = Metric::new(
        "virtualDisk",
        "totalWriteLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// Rate of writing data to the virtual disk
    /// `virtualDisk.write.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const WRITE_AVERAGE: Metric<Throughput, VmOnly> = Metric::new(
        "virtualDisk",
        "write",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// Average write request size in bytes
    /// `virtualDisk.writeIOSize.latest` · number · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const WRITE_IOSIZE_LATEST: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "writeIOSize",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// Write latency in microseconds
    /// `virtualDisk.writeLatencyUS.latest` · microsecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const WRITE_LATENCY_US_LATEST: Metric<Time, VmOnly> = Metric::new(
        "virtualDisk",
        "writeLatencyUS",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// Storage DRS virtual disk metric for the write workload model
    /// `virtualDisk.writeLoadMetric.latest` · number · vCenter and ESX
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const WRITE_LOAD_METRIC_LATEST: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "writeLoadMetric",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// Average number of outstanding write requests to the virtual disk during the collection interval
    /// `virtualDisk.writeOIO.latest` · number · vCenter and ESX
    /// Level: 2, Per-device level: 2 · Tier: Direct (1)
    /// Types: VirtualMachine
    pub const WRITE_OIO_LATEST: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "writeOIO",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );
}

pub mod vmop {
    use super::super::*;

    /// Number of datastore change operations for powered-off and suspended virtual machines
    /// `vmop.numChangeDS.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, Datacenter
    pub const NUM_CHANGE_DS_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numChangeDS",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Number of host and datastore change operations for powered-off and suspended virtual machines
    /// `vmop.numChangeHostDS.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, Datacenter
    pub const NUM_CHANGE_HOST_DS_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numChangeHostDS",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Number of host change operations for powered-off and suspended VMs
    /// `vmop.numChangeHost.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, Datacenter
    pub const NUM_CHANGE_HOST_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numChangeHost",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Number of virtual machine clone operations
    /// `vmop.numClone.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, Datacenter
    pub const NUM_CLONE_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numClone",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Number of virtual machine create operations
    /// `vmop.numCreate.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, Datacenter
    pub const NUM_CREATE_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numCreate",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Number of virtual machine template deploy operations
    /// `vmop.numDeploy.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, Datacenter
    pub const NUM_DEPLOY_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numDeploy",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Number of virtual machine delete operations
    /// `vmop.numDestroy.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, Datacenter
    pub const NUM_DESTROY_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numDestroy",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Number of virtual machine power off operations
    /// `vmop.numPoweroff.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, Datacenter
    pub const NUM_POWEROFF_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numPoweroff",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Number of virtual machine power on operations
    /// `vmop.numPoweron.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, Datacenter
    pub const NUM_POWERON_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numPoweron",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Number of virtual machine guest reboot operations
    /// `vmop.numRebootGuest.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, Datacenter
    pub const NUM_REBOOT_GUEST_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numRebootGuest",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Number of virtual machine reconfigure operations
    /// `vmop.numReconfigure.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, Datacenter
    pub const NUM_RECONFIGURE_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numReconfigure",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Number of virtual machine register operations
    /// `vmop.numRegister.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, Datacenter
    pub const NUM_REGISTER_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numRegister",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Number of virtual machine reset operations
    /// `vmop.numReset.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, Datacenter
    pub const NUM_RESET_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numReset",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Number of virtual machine guest shutdown operations
    /// `vmop.numShutdownGuest.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, Datacenter
    pub const NUM_SHUTDOWN_GUEST_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numShutdownGuest",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Number of virtual machine standby guest operations
    /// `vmop.numStandbyGuest.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, Datacenter
    pub const NUM_STANDBY_GUEST_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numStandbyGuest",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Number of virtual machine suspend operations
    /// `vmop.numSuspend.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, Datacenter
    pub const NUM_SUSPEND_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numSuspend",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Number of migrations with Storage vMotion (datastore change operations for powered-on VMs)
    /// `vmop.numSVMotion.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, Datacenter
    pub const NUM_SVMOTION_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numSVMotion",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Number of virtual machine unregister operations
    /// `vmop.numUnregister.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, Datacenter
    pub const NUM_UNREGISTER_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numUnregister",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Number of migrations with vMotion (host change operations for powered-on VMs)
    /// `vmop.numVMotion.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, Datacenter
    pub const NUM_VMOTION_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numVMotion",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// Number of host and datastore change operations for powered-on and suspended virtual machines
    /// `vmop.numXVMotion.latest` · number · vCenter only
    /// Level: 1, Per-device level: 3 · Tier: Direct (1)
    /// Types: ClusterComputeResource, Datacenter
    pub const NUM_XVMOTION_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numXVMotion",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );
}

pub mod vmotion {
    use super::super::*;

    /// `vmotion.downtime.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const DOWNTIME_LATEST: Metric<Time, HostOnly> = Metric::new(
        "vmotion",
        "downtime",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmotion.dst.migration.time.latest` · second · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const DST_MIGRATION_TIME_LATEST: Metric<Time, HostOnly> = Metric::new(
        "vmotion",
        "dst.migration.time",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmotion.mem.sizemb.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const MEM_SIZEMB_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "vmotion",
        "mem.sizemb",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmotion.precopy.time.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const PRECOPY_TIME_LATEST: Metric<Time, HostOnly> = Metric::new(
        "vmotion",
        "precopy.time",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmotion.rtt.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const RTT_LATEST: Metric<Time, HostOnly> = Metric::new(
        "vmotion",
        "rtt",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmotion.vmkernel.downtime.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: HostSystem
    pub const VMKERNEL_DOWNTIME_LATEST: Metric<Time, HostOnly> = Metric::new(
        "vmotion",
        "vmkernel.downtime",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );
}

pub mod vmx {
    use super::super::*;

    /// Actual clock speed of host CPU
    /// `vmx.cpuSpeed.latest` · megaHertz · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: VirtualMachine
    pub const CPU_SPEED_LATEST: Metric<Frequency, VmOnly> = Metric::new(
        "vmx",
        "cpuSpeed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total count of virtual CPUs in VM
    /// `vmx.numVCPUs.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: VirtualMachine
    pub const NUM_VCPUS_LATEST: Metric<Count, VmOnly> = Metric::new(
        "vmx",
        "numVCPUs",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Maximum overhead heap memory usage since the VM started running
    /// `vmx.overheadMemSizeMax.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: VirtualMachine
    pub const OVERHEAD_MEM_SIZE_MAX_LATEST: Metric<Memory, VmOnly> = Metric::new(
        "vmx",
        "overheadMemSizeMax",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Minimum overhead heap memory usage since the VM started running
    /// `vmx.overheadMemSizeMin.latest` · megaBytes · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: VirtualMachine
    pub const OVERHEAD_MEM_SIZE_MIN_LATEST: Metric<Memory, VmOnly> = Metric::new(
        "vmx",
        "overheadMemSizeMin",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmx.poll.itersPerS.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: VirtualMachine
    pub const POLL_ITERS_PER_S_LATEST: Metric<Count, VmOnly> = Metric::new(
        "vmx",
        "poll.itersPerS",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmx.userRpc.opsPerS.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: VirtualMachine
    pub const USER_RPC_OPS_PER_S_LATEST: Metric<Count, VmOnly> = Metric::new(
        "vmx",
        "userRpc.opsPerS",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Maximum clock speed of the vCPUs during last stats interval
    /// `vmx.vcpusMhzMax.latest` · megaHertz · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: VirtualMachine
    pub const VCPUS_MHZ_MAX_LATEST: Metric<Frequency, VmOnly> = Metric::new(
        "vmx",
        "vcpusMhzMax",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average clock speed of the vCPUs during last stats interval
    /// `vmx.vcpusMhzMean.latest` · megaHertz · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: VirtualMachine
    pub const VCPUS_MHZ_MEAN_LATEST: Metric<Frequency, VmOnly> = Metric::new(
        "vmx",
        "vcpusMhzMean",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Minimum clock speed of the vCPUs during last stats interval
    /// `vmx.vcpusMhzMin.latest` · megaHertz · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: VirtualMachine
    pub const VCPUS_MHZ_MIN_LATEST: Metric<Frequency, VmOnly> = Metric::new(
        "vmx",
        "vcpusMhzMin",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmx.vigor.opsTotal.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: VirtualMachine
    pub const VIGOR_OPS_TOTAL_LATEST: Metric<Count, VmOnly> = Metric::new(
        "vmx",
        "vigor.opsTotal",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );
}

pub mod vsan_dom_obj {
    use super::super::*;

    /// Average read latency in ms
    /// `vsanDomObj.readAvgLatency.average` · millisecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const READ_AVG_LATENCY_AVERAGE: Metric<Time, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "readAvgLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Cache hit rate percentage
    /// `vsanDomObj.readCacheHitRate.latest` · percent (hundredths) · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const READ_CACHE_HIT_RATE_LATEST: Metric<Ratio, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "readCacheHitRate",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Read congestion
    /// `vsanDomObj.readCongestion.average` · number · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const READ_CONGESTION_AVERAGE: Metric<Count, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "readCongestion",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Read IOPS
    /// `vsanDomObj.readIops.average` · number · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const READ_IOPS_AVERAGE: Metric<Count, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "readIops",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Max read latency in ms
    /// `vsanDomObj.readMaxLatency.latest` · millisecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const READ_MAX_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "readMaxLatency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Read throughput in kBps
    /// `vsanDomObj.readThroughput.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const READ_THROUGHPUT_AVERAGE: Metric<Throughput, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "readThroughput",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average recovery write latency in ms
    /// `vsanDomObj.recoveryWriteAvgLatency.average` · millisecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const RECOVERY_WRITE_AVG_LATENCY_AVERAGE: Metric<Time, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "recoveryWriteAvgLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Recovery write congestion
    /// `vsanDomObj.recoveryWriteCongestion.average` · number · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const RECOVERY_WRITE_CONGESTION_AVERAGE: Metric<Count, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "recoveryWriteCongestion",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Recovery write IOPS
    /// `vsanDomObj.recoveryWriteIops.average` · number · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const RECOVERY_WRITE_IOPS_AVERAGE: Metric<Count, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "recoveryWriteIops",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Max recovery write latency in ms
    /// `vsanDomObj.recoveryWriteMaxLatency.latest` · millisecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const RECOVERY_WRITE_MAX_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "recoveryWriteMaxLatency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Recovery write through-put in kBps
    /// `vsanDomObj.recoveryWriteThroughput.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const RECOVERY_WRITE_THROUGHPUT_AVERAGE: Metric<Throughput, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "recoveryWriteThroughput",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Average write latency in ms
    /// `vsanDomObj.writeAvgLatency.average` · millisecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const WRITE_AVG_LATENCY_AVERAGE: Metric<Time, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "writeAvgLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Write congestion
    /// `vsanDomObj.writeCongestion.average` · number · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const WRITE_CONGESTION_AVERAGE: Metric<Count, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "writeCongestion",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Write IOPS
    /// `vsanDomObj.writeIops.average` · number · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const WRITE_IOPS_AVERAGE: Metric<Count, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "writeIops",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Max write latency in ms
    /// `vsanDomObj.writeMaxLatency.latest` · millisecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const WRITE_MAX_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "writeMaxLatency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Write throughput in kBps
    /// `vsanDomObj.writeThroughput.average` · kiloBytesPerSecond · vCenter and ESX
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const WRITE_THROUGHPUT_AVERAGE: Metric<Throughput, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "writeThroughput",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );
}

pub mod vvol {
    use super::super::*;

    /// Cumulative latency of cmds that failed before issue by vVol Container
    /// `vvol.container.issue.latency.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const CONTAINER_ISSUE_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.issue.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// No. of read cmds issued by vVol Container that failed
    /// `vvol.container.readsissued.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const CONTAINER_READSISSUED_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.readsissued.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of read cmds done by vVol Container
    /// `vvol.container.reads.done.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const CONTAINER_READS_DONE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.reads.done",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of read cmds failed by vVol Container
    /// `vvol.container.reads.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const CONTAINER_READS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.reads.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of read cmds sent by vVol Container
    /// `vvol.container.reads.sent.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const CONTAINER_READS_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.reads.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Cumulative latency of successful reads by vVol Container
    /// `vvol.container.read.latency.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const CONTAINER_READ_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.read.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of cmds issued by vVol Container that failed
    /// `vvol.container.totalissued.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const CONTAINER_TOTALISSUED_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.totalissued.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of cmds done by vVol Container
    /// `vvol.container.total.done.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const CONTAINER_TOTAL_DONE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.total.done",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of cmds failed by vVol Container
    /// `vvol.container.total.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const CONTAINER_TOTAL_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.total.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Cumulative latency of all issued cmds by vVol Container
    /// `vvol.container.total.latency.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const CONTAINER_TOTAL_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.total.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of cmds sent by vVol Container
    /// `vvol.container.total.sent.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const CONTAINER_TOTAL_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.total.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// No. of write cmds issued by vVol Container that failed
    /// `vvol.container.writesissued.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const CONTAINER_WRITESISSUED_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.writesissued.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of write cmds done by vVol Container
    /// `vvol.container.writes.done.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const CONTAINER_WRITES_DONE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.writes.done",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Container:Total no. of write cmds failed by vVol Container
    /// `vvol.container.writes.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const CONTAINER_WRITES_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.writes.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of write cmds sent by vVol Container
    /// `vvol.container.writes.sent.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const CONTAINER_WRITES_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.writes.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Cumulative latency of successful writes by vVol Container
    /// `vvol.container.write.latency.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const CONTAINER_WRITE_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.write.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of cancel cmds failed by vVol Device
    /// `vvol.device.cancel.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_CANCEL_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.cancel.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of cancel cmds sent by vVol Device
    /// `vvol.device.cancel.sent.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_CANCEL_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.cancel.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of device reset cmds failed by vVol Device
    /// `vvol.device.deviceresets.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_DEVICERESETS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.deviceresets.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of device reset cmds sent by vVol Device
    /// `vvol.device.deviceresets.sent.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_DEVICERESETS_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.deviceresets.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Cumulative latency of cmds that failed before issue by vVol Device
    /// `vvol.device.issue.latency.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_ISSUE_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.issue.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// No. of read cmds issued by vVol Device that failed
    /// `vvol.device.readsissued.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_READSISSUED_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.readsissued.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of read cmds done by vVol Device
    /// `vvol.device.reads.done.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_READS_DONE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.reads.done",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of read cmds failed by vVol Device
    /// `vvol.device.reads.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_READS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.reads.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of read cmds sent by vVol Device
    /// `vvol.device.reads.sent.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_READS_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.reads.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Cumulative latency of successful reads by vVol Device
    /// `vvol.device.read.latency.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_READ_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.read.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of reset cmds failed by vVol Device
    /// `vvol.device.resets.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_RESETS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.resets.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of reset cmds sent by vVol Device
    /// `vvol.device.resets.sent.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_RESETS_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.resets.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of cmds issued by vVol Device that failed
    /// `vvol.device.totalissued.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_TOTALISSUED_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.totalissued.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of cmds done by vVol Device
    /// `vvol.device.total.done.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_TOTAL_DONE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.total.done",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of cmds failed by vVol Device
    /// `vvol.device.total.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_TOTAL_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.total.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Cumulative latency of all issued cmds by vVol Device
    /// `vvol.device.total.latency.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_TOTAL_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.total.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of cmds sent by vVol Device
    /// `vvol.device.total.sent.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_TOTAL_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.total.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of unmap cmds failed by vVol Device
    /// `vvol.device.unmaps.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_UNMAPS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.unmaps.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of unmap cmds sent by vVol Device
    /// `vvol.device.unmaps.sent.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_UNMAPS_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.unmaps.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// No. of write cmds issued by vVol Device that failed
    /// `vvol.device.writesissued.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_WRITESISSUED_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.writesissued.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of write cmds done by vVol Device
    /// `vvol.device.writes.done.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_WRITES_DONE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.writes.done",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of write cmds failed by vVol Device
    /// `vvol.device.writes.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_WRITES_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.writes.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of write cmds sent by vVol Device
    /// `vvol.device.writes.sent.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_WRITES_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.writes.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Cumulative latency of successful writes by vVol Device
    /// `vvol.device.write.latency.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const DEVICE_WRITE_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.write.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of cancel cmds failed on vVol PE
    /// `vvol.pe.cancel.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_CANCEL_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.cancel.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of cancel cmds sent on vVol PE
    /// `vvol.pe.cancel.sent.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_CANCEL_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.cancel.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of device reset cmds failed on vVol PE
    /// `vvol.pe.deviceresets.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_DEVICERESETS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.deviceresets.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of device reset cmds sent on vVol PE
    /// `vvol.pe.deviceresets.sent.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_DEVICERESETS_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.deviceresets.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// vVol PE is accessible
    /// `vvol.pe.isaccessible.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_ISACCESSIBLE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.isaccessible",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Cumulative latency of cmds that failed before issue on vVol PE
    /// `vvol.pe.issue.latency.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_ISSUE_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.issue.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// No. of read cmds issued on vVol PE that failed
    /// `vvol.pe.readsissued.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_READSISSUED_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.readsissued.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of read cmds done on vVol PE
    /// `vvol.pe.reads.done.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_READS_DONE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.reads.done",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of read cmds failed on vVol PE
    /// `vvol.pe.reads.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_READS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.reads.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of read cmds sent on vVol PE
    /// `vvol.pe.reads.sent.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_READS_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.reads.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Cumulative latency of successful reads on vVol PE
    /// `vvol.pe.read.latency.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_READ_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.read.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of reset cmds failed on vVol PE
    /// `vvol.pe.resets.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_RESETS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.resets.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of reset cmds sent on vVol PE
    /// `vvol.pe.resets.sent.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_RESETS_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.resets.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of cmds issued on vVol PE that failed
    /// `vvol.pe.totalissued.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_TOTALISSUED_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.totalissued.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of cmds done on vVol PE
    /// `vvol.pe.total.done.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_TOTAL_DONE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.total.done",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of cmds failed on vVol PE
    /// `vvol.pe.total.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_TOTAL_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.total.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Cumulative latency of all issued cmds on vVol PE
    /// `vvol.pe.total.latency.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_TOTAL_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.total.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of cmds sent on vVol PE
    /// `vvol.pe.total.sent.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_TOTAL_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.total.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of unmap cmds failed on vVol PE
    /// `vvol.pe.unmaps.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_UNMAPS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.unmaps.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of unmap cmds sent on vVol PE
    /// `vvol.pe.unmaps.sent.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_UNMAPS_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.unmaps.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// No. of write cmds issued on vVol PE that failed
    /// `vvol.pe.writesissued.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_WRITESISSUED_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.writesissued.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of write cmds done on vVol PE
    /// `vvol.pe.writes.done.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_WRITES_DONE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.writes.done",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of write cmds failed on vVol PE
    /// `vvol.pe.writes.failed.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_WRITES_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.writes.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Total no. of write cmds sent on vVol PE
    /// `vvol.pe.writes.sent.latest` · number · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_WRITES_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.writes.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// Cumulative latency of successful writes on vVol PE
    /// `vvol.pe.write.latency.latest` · microsecond · vCenter only
    /// Level: 4, Per-device level: 4 · Tier: Inferred (3b)
    /// Types: Datastore, HostSystem, VirtualMachine
    pub const PE_WRITE_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.write.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );
}

pub mod by_entity {
    pub mod virtual_machine {
        pub mod cluster_services {
            pub use crate::perf::metrics::cluster_services::VM_DRS_SCORE_LATEST;
        }

        pub mod cpu {
            pub use crate::perf::metrics::cpu::CACHE_L3_OCCUPANCY_AVERAGE;
            pub use crate::perf::metrics::cpu::CAPACITY_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::cpu::CAPACITY_DEMAND_AVERAGE;
            pub use crate::perf::metrics::cpu::CAPACITY_ENTITLEMENT_AVERAGE;
            pub use crate::perf::metrics::cpu::CAPACITY_PROVISIONED_AVERAGE;
            pub use crate::perf::metrics::cpu::CAPACITY_USAGE_AVERAGE;
            pub use crate::perf::metrics::cpu::CORECOUNT_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::cpu::CORECOUNT_PROVISIONED_AVERAGE;
            pub use crate::perf::metrics::cpu::CORECOUNT_PROVISIONED_LATEST;
            pub use crate::perf::metrics::cpu::CORECOUNT_USAGE_AVERAGE;
            pub use crate::perf::metrics::cpu::CORECOUNT_USAGE_LATEST;
            pub use crate::perf::metrics::cpu::COSTOP_SUMMATION;
            pub use crate::perf::metrics::cpu::CPUENTITLEMENT_LATEST;
            pub use crate::perf::metrics::cpu::DEMAND_AVERAGE;
            pub use crate::perf::metrics::cpu::DEMAND_ENTITLEMENT_RATIO_LATEST;
            pub use crate::perf::metrics::cpu::ENTITLEMENT_LATEST;
            pub use crate::perf::metrics::cpu::IDLE_SUMMATION;
            pub use crate::perf::metrics::cpu::LATENCY_AVERAGE;
            pub use crate::perf::metrics::cpu::LOAD_AVG15MIN_LATEST;
            pub use crate::perf::metrics::cpu::LOAD_AVG1MIN_LATEST;
            pub use crate::perf::metrics::cpu::LOAD_AVG5MIN_LATEST;
            pub use crate::perf::metrics::cpu::MAXLIMITED_SUMMATION;
            pub use crate::perf::metrics::cpu::OVERLAP_SUMMATION;
            pub use crate::perf::metrics::cpu::PARTNER_BUSY_TIME_AVERAGE;
            pub use crate::perf::metrics::cpu::READINESS_AVERAGE;
            pub use crate::perf::metrics::cpu::READY_SUMMATION;
            pub use crate::perf::metrics::cpu::RUN_SUMMATION;
            pub use crate::perf::metrics::cpu::SWAPWAIT_SUMMATION;
            pub use crate::perf::metrics::cpu::SYSTEM_SUMMATION;
            pub use crate::perf::metrics::cpu::TOTALMHZ_AVERAGE;
            pub use crate::perf::metrics::cpu::USAGEMHZ_AVERAGE;
            pub use crate::perf::metrics::cpu::USAGEMHZ_MAXIMUM;
            pub use crate::perf::metrics::cpu::USAGEMHZ_MINIMUM;
            pub use crate::perf::metrics::cpu::USAGEMHZ_NONE;
            pub use crate::perf::metrics::cpu::USAGE_AVERAGE;
            pub use crate::perf::metrics::cpu::USAGE_MAXIMUM;
            pub use crate::perf::metrics::cpu::USAGE_MINIMUM;
            pub use crate::perf::metrics::cpu::USAGE_NONE;
            pub use crate::perf::metrics::cpu::USAGE_VCPUS_AVERAGE;
            pub use crate::perf::metrics::cpu::USED_SUMMATION;
            pub use crate::perf::metrics::cpu::WAIT_SUMMATION;
        }

        pub mod datastore {
            pub use crate::perf::metrics::datastore::BUS_RESETS_SUMMATION;
            pub use crate::perf::metrics::datastore::COMMANDS_ABORTED_SUMMATION;
            pub use crate::perf::metrics::datastore::MAX_TOTAL_LATENCY_LATEST;
            pub use crate::perf::metrics::datastore::NUMBER_READ_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::datastore::NUMBER_WRITE_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::datastore::READ_AVERAGE;
            pub use crate::perf::metrics::datastore::THROUGHPUT_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::datastore::THROUGHPUT_USAGE_AVERAGE;
            pub use crate::perf::metrics::datastore::TOTAL_READ_LATENCY_AVERAGE;
            pub use crate::perf::metrics::datastore::TOTAL_WRITE_LATENCY_AVERAGE;
            pub use crate::perf::metrics::datastore::VMFS_APD_COUNT_LATEST;
            pub use crate::perf::metrics::datastore::VMFS_APD_STATE_LATEST;
            pub use crate::perf::metrics::datastore::VMFS_CANCELLED_TXN_LATEST;
            pub use crate::perf::metrics::datastore::VMFS_TOTAL_TXN_LATEST;
            pub use crate::perf::metrics::datastore::WRITE_AVERAGE;
        }

        pub mod disk {
            pub use crate::perf::metrics::disk::ACTUALUSED_LATEST;
            pub use crate::perf::metrics::disk::BUS_RESETS_SUMMATION;
            pub use crate::perf::metrics::disk::CAPACITY_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::disk::CAPACITY_PROVISIONED_AVERAGE;
            pub use crate::perf::metrics::disk::CAPACITY_USAGE_AVERAGE;
            pub use crate::perf::metrics::disk::COMMANDS_ABORTED_SUMMATION;
            pub use crate::perf::metrics::disk::COMMANDS_ACTIVE_LATEST;
            pub use crate::perf::metrics::disk::COMMANDS_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::disk::COMMANDS_FAILED_LATEST;
            pub use crate::perf::metrics::disk::COMMANDS_QUEUED_LATEST;
            pub use crate::perf::metrics::disk::COMMANDS_SUCCESS_LATEST;
            pub use crate::perf::metrics::disk::COMMANDS_SUMMATION;
            pub use crate::perf::metrics::disk::DELTAUSED_LATEST;
            pub use crate::perf::metrics::disk::LATENCY_DAVG_LATEST;
            pub use crate::perf::metrics::disk::LATENCY_GAVG_LATEST;
            pub use crate::perf::metrics::disk::LATENCY_KAVG_LATEST;
            pub use crate::perf::metrics::disk::LATENCY_QAVG_LATEST;
            pub use crate::perf::metrics::disk::MAX_TOTAL_LATENCY_LATEST;
            pub use crate::perf::metrics::disk::NUMBER_READ_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::disk::NUMBER_READ_SUMMATION;
            pub use crate::perf::metrics::disk::NUMBER_WRITE_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::disk::NUMBER_WRITE_SUMMATION;
            pub use crate::perf::metrics::disk::PROVISIONED_LATEST;
            pub use crate::perf::metrics::disk::READ_AVERAGE;
            pub use crate::perf::metrics::disk::READ_FAILED_LATEST;
            pub use crate::perf::metrics::disk::READ_LATEST;
            pub use crate::perf::metrics::disk::SCSI_RESERVATION_CNFLCTS_PCT_AVERAGE;
            pub use crate::perf::metrics::disk::SCSI_RESERVATION_CNFLCTS_PCT_AVERAGE_ABSOLUTE;
            pub use crate::perf::metrics::disk::SCSI_RESERVATION_CONFLICTS_SUMMATION;
            pub use crate::perf::metrics::disk::STATE_LATEST;
            pub use crate::perf::metrics::disk::THROUGHPUT_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::disk::THROUGHPUT_USAGE_AVERAGE;
            pub use crate::perf::metrics::disk::TM_ABORT_FAILED_LATEST;
            pub use crate::perf::metrics::disk::TM_ABORT_LATEST;
            pub use crate::perf::metrics::disk::TM_ABORT_RETRY_LATEST;
            pub use crate::perf::metrics::disk::TM_BUS_RESET_FAILED_LATEST;
            pub use crate::perf::metrics::disk::TM_BUS_RESET_LATEST;
            pub use crate::perf::metrics::disk::TM_BUS_RESET_RETRY_LATEST;
            pub use crate::perf::metrics::disk::TM_DEVICE_RESET_FAILED_LATEST;
            pub use crate::perf::metrics::disk::TM_DEVICE_RESET_LATEST;
            pub use crate::perf::metrics::disk::TM_DEVICE_RESET_RETRY_LATEST;
            pub use crate::perf::metrics::disk::TM_LUN_RESET_FAILED_LATEST;
            pub use crate::perf::metrics::disk::TM_LUN_RESET_LATEST;
            pub use crate::perf::metrics::disk::TM_LUN_RESET_RETRY_LATEST;
            pub use crate::perf::metrics::disk::TM_VIRT_RESET_FAILED_LATEST;
            pub use crate::perf::metrics::disk::TM_VIRT_RESET_LATEST;
            pub use crate::perf::metrics::disk::TM_VIRT_RESET_RETRY_LATEST;
            pub use crate::perf::metrics::disk::UNSHARED_LATEST;
            pub use crate::perf::metrics::disk::USAGE_AVERAGE;
            pub use crate::perf::metrics::disk::USAGE_MAXIMUM;
            pub use crate::perf::metrics::disk::USAGE_MINIMUM;
            pub use crate::perf::metrics::disk::USAGE_NONE;
            pub use crate::perf::metrics::disk::USED_LATEST;
            pub use crate::perf::metrics::disk::WRITE_AVERAGE;
            pub use crate::perf::metrics::disk::WRITE_FAILED_LATEST;
            pub use crate::perf::metrics::disk::WRITE_LATEST;
        }

        pub mod mem {
            pub use crate::perf::metrics::mem::ACTIVEWRITE_AVERAGE;
            pub use crate::perf::metrics::mem::ACTIVE_AVERAGE;
            pub use crate::perf::metrics::mem::ACTIVE_MAXIMUM;
            pub use crate::perf::metrics::mem::ACTIVE_MINIMUM;
            pub use crate::perf::metrics::mem::ACTIVE_NONE;
            pub use crate::perf::metrics::mem::BANDWIDTH_READ_LATEST;
            pub use crate::perf::metrics::mem::BANDWIDTH_TOTAL_LATEST;
            pub use crate::perf::metrics::mem::BANDWIDTH_WRITE_LATEST;
            pub use crate::perf::metrics::mem::CAPACITY_ACTIVE_LATEST;
            pub use crate::perf::metrics::mem::CAPACITY_CONSUMED_LATEST;
            pub use crate::perf::metrics::mem::CAPACITY_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_ENTITLEMENT_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_OVERHEAD_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_OVERHEAD_RESV_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_PROVISIONED_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_PROVISIONED_LATEST;
            pub use crate::perf::metrics::mem::CAPACITY_USABLE_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_USAGE_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_USAGE_USERWORLD_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_USAGE_VMK_OVRHD_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_USAGE_VM_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_USAGE_VM_OVRHD_AVERAGE;
            pub use crate::perf::metrics::mem::COMPRESSED_AVERAGE;
            pub use crate::perf::metrics::mem::COMPRESSION_RATE_AVERAGE;
            pub use crate::perf::metrics::mem::CONSUMED_AVERAGE;
            pub use crate::perf::metrics::mem::CONSUMED_MAXIMUM;
            pub use crate::perf::metrics::mem::CONSUMED_MINIMUM;
            pub use crate::perf::metrics::mem::CONSUMED_NONE;
            pub use crate::perf::metrics::mem::DECOMPRESSION_RATE_AVERAGE;
            pub use crate::perf::metrics::mem::ENTITLEMENT_AVERAGE;
            pub use crate::perf::metrics::mem::GRANTED_AVERAGE;
            pub use crate::perf::metrics::mem::GRANTED_MAXIMUM;
            pub use crate::perf::metrics::mem::GRANTED_MINIMUM;
            pub use crate::perf::metrics::mem::GRANTED_NONE;
            pub use crate::perf::metrics::mem::HEALTH_RESERVATION_STATE_LATEST;
            pub use crate::perf::metrics::mem::KERNEL_MANAGED_LATEST;
            pub use crate::perf::metrics::mem::KERNEL_MINFREE_LATEST;
            pub use crate::perf::metrics::mem::KERNEL_UNRESERVED_LATEST;
            pub use crate::perf::metrics::mem::LATENCY_AVERAGE;
            pub use crate::perf::metrics::mem::LATENCY_READ_LATEST;
            pub use crate::perf::metrics::mem::LATENCY_WRITE_LATEST;
            pub use crate::perf::metrics::mem::LL_SWAP_IN_RATE_AVERAGE;
            pub use crate::perf::metrics::mem::LL_SWAP_OUT_RATE_AVERAGE;
            pub use crate::perf::metrics::mem::LL_SWAP_USED_AVERAGE;
            pub use crate::perf::metrics::mem::LL_SWAP_USED_MAXIMUM;
            pub use crate::perf::metrics::mem::LL_SWAP_USED_MINIMUM;
            pub use crate::perf::metrics::mem::LL_SWAP_USED_NONE;
            pub use crate::perf::metrics::mem::MEMCTL_CURRENT_LATEST;
            pub use crate::perf::metrics::mem::MEMCTL_MAX_LATEST;
            pub use crate::perf::metrics::mem::MEMCTL_TARGET_LATEST;
            pub use crate::perf::metrics::mem::MEMENTITLEMENT_LATEST;
            pub use crate::perf::metrics::mem::MISSRATE_LATEST;
            pub use crate::perf::metrics::mem::OVERCOMMIT_AVG15MIN_LATEST;
            pub use crate::perf::metrics::mem::OVERCOMMIT_AVG1MIN_LATEST;
            pub use crate::perf::metrics::mem::OVERCOMMIT_AVG5MIN_LATEST;
            pub use crate::perf::metrics::mem::OVERHEAD_AVERAGE;
            pub use crate::perf::metrics::mem::OVERHEAD_MAXIMUM;
            pub use crate::perf::metrics::mem::OVERHEAD_MAX_AVERAGE;
            pub use crate::perf::metrics::mem::OVERHEAD_MINIMUM;
            pub use crate::perf::metrics::mem::OVERHEAD_NONE;
            pub use crate::perf::metrics::mem::OVERHEAD_TOUCHED_AVERAGE;
            pub use crate::perf::metrics::mem::PHYSICAL_FREE_LATEST;
            pub use crate::perf::metrics::mem::PHYSICAL_TOTAL_LATEST;
            pub use crate::perf::metrics::mem::PHYSICAL_USER_LATEST;
            pub use crate::perf::metrics::mem::PSHARE_COMMON_LATEST;
            pub use crate::perf::metrics::mem::PSHARE_SHARED_LATEST;
            pub use crate::perf::metrics::mem::PSHARE_SHARED_SAVE_LATEST;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_PCT_AVERAGE;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_PCT_LATEST;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_USERWORLD_AVERAGE;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_VMK_OVRHD_AVERAGE;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_VM_AVERAGE;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_VM_OVHD_AVERAGE;
            pub use crate::perf::metrics::mem::SHARED_AVERAGE;
            pub use crate::perf::metrics::mem::SHARED_MAXIMUM;
            pub use crate::perf::metrics::mem::SHARED_MINIMUM;
            pub use crate::perf::metrics::mem::SHARED_NONE;
            pub use crate::perf::metrics::mem::SWAPIN_AVERAGE;
            pub use crate::perf::metrics::mem::SWAPIN_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAPIN_MINIMUM;
            pub use crate::perf::metrics::mem::SWAPIN_NONE;
            pub use crate::perf::metrics::mem::SWAPIN_RATE_AVERAGE;
            pub use crate::perf::metrics::mem::SWAPOUT_AVERAGE;
            pub use crate::perf::metrics::mem::SWAPOUT_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAPOUT_MINIMUM;
            pub use crate::perf::metrics::mem::SWAPOUT_NONE;
            pub use crate::perf::metrics::mem::SWAPOUT_RATE_AVERAGE;
            pub use crate::perf::metrics::mem::SWAPPED_AVERAGE;
            pub use crate::perf::metrics::mem::SWAPPED_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAPPED_MINIMUM;
            pub use crate::perf::metrics::mem::SWAPPED_NONE;
            pub use crate::perf::metrics::mem::SWAPTARGET_AVERAGE;
            pub use crate::perf::metrics::mem::SWAPTARGET_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAPTARGET_MINIMUM;
            pub use crate::perf::metrics::mem::SWAPTARGET_NONE;
            pub use crate::perf::metrics::mem::SWAPUNRESERVED_AVERAGE;
            pub use crate::perf::metrics::mem::SWAPUNRESERVED_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAPUNRESERVED_MINIMUM;
            pub use crate::perf::metrics::mem::SWAPUNRESERVED_NONE;
            pub use crate::perf::metrics::mem::SWAP_CURRENT_LATEST;
            pub use crate::perf::metrics::mem::SWAP_IN_AVERAGE;
            pub use crate::perf::metrics::mem::SWAP_IN_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAP_IN_MINIMUM;
            pub use crate::perf::metrics::mem::SWAP_IN_NONE;
            pub use crate::perf::metrics::mem::SWAP_OUT_AVERAGE;
            pub use crate::perf::metrics::mem::SWAP_OUT_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAP_OUT_MINIMUM;
            pub use crate::perf::metrics::mem::SWAP_OUT_NONE;
            pub use crate::perf::metrics::mem::SWAP_READRATE_AVERAGE;
            pub use crate::perf::metrics::mem::SWAP_TARGET_LATEST;
            pub use crate::perf::metrics::mem::SWAP_WRITERATE_AVERAGE;
            pub use crate::perf::metrics::mem::TOTALMB_AVERAGE;
            pub use crate::perf::metrics::mem::USAGE_AVERAGE;
            pub use crate::perf::metrics::mem::USAGE_MAXIMUM;
            pub use crate::perf::metrics::mem::USAGE_MINIMUM;
            pub use crate::perf::metrics::mem::USAGE_NONE;
            pub use crate::perf::metrics::mem::VMMEMCTLTARGET_AVERAGE;
            pub use crate::perf::metrics::mem::VMMEMCTLTARGET_MAXIMUM;
            pub use crate::perf::metrics::mem::VMMEMCTLTARGET_MINIMUM;
            pub use crate::perf::metrics::mem::VMMEMCTLTARGET_NONE;
            pub use crate::perf::metrics::mem::VMMEMCTL_AVERAGE;
            pub use crate::perf::metrics::mem::VMMEMCTL_MAXIMUM;
            pub use crate::perf::metrics::mem::VMMEMCTL_MINIMUM;
            pub use crate::perf::metrics::mem::VMMEMCTL_NONE;
            pub use crate::perf::metrics::mem::VM_BANDWIDTH_READ_LATEST;
            pub use crate::perf::metrics::mem::ZERO_AVERAGE;
            pub use crate::perf::metrics::mem::ZERO_MAXIMUM;
            pub use crate::perf::metrics::mem::ZERO_MINIMUM;
            pub use crate::perf::metrics::mem::ZERO_NONE;
            pub use crate::perf::metrics::mem::ZIPPED_LATEST;
            pub use crate::perf::metrics::mem::ZIPSAVED_LATEST;
            pub use crate::perf::metrics::mem::ZIP_SAVED_LATEST;
            pub use crate::perf::metrics::mem::ZIP_ZIPPED_LATEST;
        }

        pub mod net {
            pub use crate::perf::metrics::net::BROADCAST_RX_SUMMATION;
            pub use crate::perf::metrics::net::BROADCAST_TX_SUMMATION;
            pub use crate::perf::metrics::net::BYTES_RX_AVERAGE;
            pub use crate::perf::metrics::net::BYTES_TX_AVERAGE;
            pub use crate::perf::metrics::net::DROPPED_RX_SUMMATION;
            pub use crate::perf::metrics::net::DROPPED_TX_SUMMATION;
            pub use crate::perf::metrics::net::LATENCY_HBR_OUTBOUND_LATEST;
            pub use crate::perf::metrics::net::MULTICAST_RX_SUMMATION;
            pub use crate::perf::metrics::net::MULTICAST_TX_SUMMATION;
            pub use crate::perf::metrics::net::PACKETS_RX_SUMMATION;
            pub use crate::perf::metrics::net::PACKETS_TX_SUMMATION;
            pub use crate::perf::metrics::net::PNIC_BYTES_RX_AVERAGE;
            pub use crate::perf::metrics::net::PNIC_BYTES_TX_AVERAGE;
            pub use crate::perf::metrics::net::RECEIVED_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_CONTENTION_SUMMATION;
            pub use crate::perf::metrics::net::THROUGHPUT_DROPPED_RX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_DROPPED_TX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_HBR_INBOUND_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_HBR_OUTBOUND_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_PACKETS_PER_SEC_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_PKTS_RX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_PKTS_RX_BROADCAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_PKTS_RX_MULTICAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_PKTS_TX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_PKTS_TX_BROADCAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_PKTS_TX_MULTICAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_PROVISIONED_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_USABLE_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_USAGE_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_USAGE_FT_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_USAGE_HBR_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_USAGE_ISCSI_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_USAGE_NFS_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_USAGE_VMOTION_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_USAGE_VM_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_DROPPED_RX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_DROPPED_TX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_LAG_DROP_RX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_LAG_DROP_TX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_LAG_RX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_LAG_RX_BCAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_LAG_RX_MCAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_LAG_TX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_LAG_TX_BCAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_LAG_TX_MCAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_PKTS_RX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_PKTS_RX_BCAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_PKTS_RX_MCAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_PKTS_TX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_PKTS_TX_BCAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_PKTS_TX_MCAST_AVERAGE;
            pub use crate::perf::metrics::net::TRANSMITTED_AVERAGE;
            pub use crate::perf::metrics::net::USAGE_AVERAGE;
            pub use crate::perf::metrics::net::USAGE_MAXIMUM;
            pub use crate::perf::metrics::net::USAGE_MINIMUM;
            pub use crate::perf::metrics::net::USAGE_NONE;
        }

        pub mod power {
            pub use crate::perf::metrics::power::CAPACITY_USABLE_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_CPU_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_MEM_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_OTHER_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_PCT_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_STATIC_AVERAGE;
            pub use crate::perf::metrics::power::ENERGY_SUMMATION;
            pub use crate::perf::metrics::power::POWER_AVERAGE;
        }

        pub mod rescpu {
            pub use crate::perf::metrics::rescpu::ACTAV15_LATEST;
            pub use crate::perf::metrics::rescpu::ACTAV1_LATEST;
            pub use crate::perf::metrics::rescpu::ACTAV5_LATEST;
            pub use crate::perf::metrics::rescpu::ACTPK15_LATEST;
            pub use crate::perf::metrics::rescpu::ACTPK1_LATEST;
            pub use crate::perf::metrics::rescpu::ACTPK5_LATEST;
            pub use crate::perf::metrics::rescpu::MAX_LIMITED15_LATEST;
            pub use crate::perf::metrics::rescpu::MAX_LIMITED1_LATEST;
            pub use crate::perf::metrics::rescpu::MAX_LIMITED5_LATEST;
            pub use crate::perf::metrics::rescpu::RUNAV15_LATEST;
            pub use crate::perf::metrics::rescpu::RUNAV1_LATEST;
            pub use crate::perf::metrics::rescpu::RUNAV5_LATEST;
            pub use crate::perf::metrics::rescpu::RUNPK15_LATEST;
            pub use crate::perf::metrics::rescpu::RUNPK1_LATEST;
            pub use crate::perf::metrics::rescpu::RUNPK5_LATEST;
            pub use crate::perf::metrics::rescpu::SAMPLE_COUNT_LATEST;
            pub use crate::perf::metrics::rescpu::SAMPLE_PERIOD_LATEST;
        }

        pub mod sys {
            pub use crate::perf::metrics::sys::DISK_USAGE_LATEST;
            pub use crate::perf::metrics::sys::HEARTBEAT_LATEST;
            pub use crate::perf::metrics::sys::HEARTBEAT_SUMMATION;
            pub use crate::perf::metrics::sys::OS_UPTIME_LATEST;
            pub use crate::perf::metrics::sys::UPTIME_LATEST;
        }

        pub mod virtual_disk {
            pub use crate::perf::metrics::virtual_disk::BUS_RESETS_SUMMATION;
            pub use crate::perf::metrics::virtual_disk::COMMANDS_ABORTED_SUMMATION;
            pub use crate::perf::metrics::virtual_disk::HBR_READ_LATENCY_MS_LATEST;
            pub use crate::perf::metrics::virtual_disk::HBR_STALL_LATENCY_MS_LATEST;
            pub use crate::perf::metrics::virtual_disk::LARGE_SEEKS_LATEST;
            pub use crate::perf::metrics::virtual_disk::MEDIUM_SEEKS_LATEST;
            pub use crate::perf::metrics::virtual_disk::NUMBER_READ_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::virtual_disk::NUMBER_WRITE_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::virtual_disk::READ_AVERAGE;
            pub use crate::perf::metrics::virtual_disk::READ_IOSIZE_LATEST;
            pub use crate::perf::metrics::virtual_disk::READ_LATENCY_US_LATEST;
            pub use crate::perf::metrics::virtual_disk::READ_LOAD_METRIC_LATEST;
            pub use crate::perf::metrics::virtual_disk::READ_OIO_LATEST;
            pub use crate::perf::metrics::virtual_disk::SMALL_SEEKS_LATEST;
            pub use crate::perf::metrics::virtual_disk::THROUGHPUT_CONT_AVERAGE;
            pub use crate::perf::metrics::virtual_disk::THROUGHPUT_USAGE_AVERAGE;
            pub use crate::perf::metrics::virtual_disk::TOTAL_READ_LATENCY_AVERAGE;
            pub use crate::perf::metrics::virtual_disk::TOTAL_WRITE_LATENCY_AVERAGE;
            pub use crate::perf::metrics::virtual_disk::WRITE_AVERAGE;
            pub use crate::perf::metrics::virtual_disk::WRITE_IOSIZE_LATEST;
            pub use crate::perf::metrics::virtual_disk::WRITE_LATENCY_US_LATEST;
            pub use crate::perf::metrics::virtual_disk::WRITE_LOAD_METRIC_LATEST;
            pub use crate::perf::metrics::virtual_disk::WRITE_OIO_LATEST;
        }
    }

    pub mod host_system {
        pub mod cluster_services {
            pub use crate::perf::metrics::cluster_services::CPUFAIRNESS_LATEST;
            pub use crate::perf::metrics::cluster_services::MEMFAIRNESS_LATEST;
        }

        pub mod cpu {
            pub use crate::perf::metrics::cpu::CACHE_L3_OCCUPANCY_AVERAGE;
            pub use crate::perf::metrics::cpu::CAPACITY_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::cpu::CAPACITY_DEMAND_AVERAGE;
            pub use crate::perf::metrics::cpu::CAPACITY_ENTITLEMENT_AVERAGE;
            pub use crate::perf::metrics::cpu::CAPACITY_PROVISIONED_AVERAGE;
            pub use crate::perf::metrics::cpu::CAPACITY_USAGE_AVERAGE;
            pub use crate::perf::metrics::cpu::CORECOUNT_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::cpu::CORECOUNT_PROVISIONED_AVERAGE;
            pub use crate::perf::metrics::cpu::CORECOUNT_PROVISIONED_LATEST;
            pub use crate::perf::metrics::cpu::CORECOUNT_USAGE_AVERAGE;
            pub use crate::perf::metrics::cpu::CORECOUNT_USAGE_LATEST;
            pub use crate::perf::metrics::cpu::CORE_UTILIZATION_AVERAGE;
            pub use crate::perf::metrics::cpu::CORE_UTILIZATION_MAXIMUM;
            pub use crate::perf::metrics::cpu::CORE_UTILIZATION_MINIMUM;
            pub use crate::perf::metrics::cpu::CORE_UTILIZATION_NONE;
            pub use crate::perf::metrics::cpu::COSTOP_SUMMATION;
            pub use crate::perf::metrics::cpu::DEMAND_AVERAGE;
            pub use crate::perf::metrics::cpu::IDLE_SUMMATION;
            pub use crate::perf::metrics::cpu::LATENCY_AVERAGE;
            pub use crate::perf::metrics::cpu::LOAD_AVG15MIN_LATEST;
            pub use crate::perf::metrics::cpu::LOAD_AVG1MIN_LATEST;
            pub use crate::perf::metrics::cpu::LOAD_AVG5MIN_LATEST;
            pub use crate::perf::metrics::cpu::PARTNER_BUSY_TIME_AVERAGE;
            pub use crate::perf::metrics::cpu::READINESS_AVERAGE;
            pub use crate::perf::metrics::cpu::READY_SUMMATION;
            pub use crate::perf::metrics::cpu::RESERVED_CAPACITY_AVERAGE;
            pub use crate::perf::metrics::cpu::SWAPWAIT_SUMMATION;
            pub use crate::perf::metrics::cpu::TOTALMHZ_AVERAGE;
            pub use crate::perf::metrics::cpu::TOTAL_CAPACITY_AVERAGE;
            pub use crate::perf::metrics::cpu::USAGEMHZ_AVERAGE;
            pub use crate::perf::metrics::cpu::USAGEMHZ_MAXIMUM;
            pub use crate::perf::metrics::cpu::USAGEMHZ_MINIMUM;
            pub use crate::perf::metrics::cpu::USAGEMHZ_NONE;
            pub use crate::perf::metrics::cpu::USAGE_AVERAGE;
            pub use crate::perf::metrics::cpu::USAGE_MAXIMUM;
            pub use crate::perf::metrics::cpu::USAGE_MINIMUM;
            pub use crate::perf::metrics::cpu::USAGE_NONE;
            pub use crate::perf::metrics::cpu::USED_SUMMATION;
            pub use crate::perf::metrics::cpu::UTILIZATION_AVERAGE;
            pub use crate::perf::metrics::cpu::UTILIZATION_MAXIMUM;
            pub use crate::perf::metrics::cpu::UTILIZATION_MINIMUM;
            pub use crate::perf::metrics::cpu::UTILIZATION_NONE;
            pub use crate::perf::metrics::cpu::WAIT_SUMMATION;
        }

        pub mod datastore {
            pub use crate::perf::metrics::datastore::BUS_RESETS_SUMMATION;
            pub use crate::perf::metrics::datastore::COMMANDS_ABORTED_SUMMATION;
            pub use crate::perf::metrics::datastore::DATASTORE_IOPS_AVERAGE;
            pub use crate::perf::metrics::datastore::DATASTORE_MAX_QUEUE_DEPTH_LATEST;
            pub use crate::perf::metrics::datastore::DATASTORE_NORMAL_READ_LATENCY_LATEST;
            pub use crate::perf::metrics::datastore::DATASTORE_NORMAL_WRITE_LATENCY_LATEST;
            pub use crate::perf::metrics::datastore::DATASTORE_READ_BYTES_LATEST;
            pub use crate::perf::metrics::datastore::DATASTORE_READ_IOPS_LATEST;
            pub use crate::perf::metrics::datastore::DATASTORE_READ_LOAD_METRIC_LATEST;
            pub use crate::perf::metrics::datastore::DATASTORE_READ_OIO_LATEST;
            pub use crate::perf::metrics::datastore::DATASTORE_VMOBSERVED_LATENCY_LATEST;
            pub use crate::perf::metrics::datastore::DATASTORE_WRITE_BYTES_LATEST;
            pub use crate::perf::metrics::datastore::DATASTORE_WRITE_IOPS_LATEST;
            pub use crate::perf::metrics::datastore::DATASTORE_WRITE_LOAD_METRIC_LATEST;
            pub use crate::perf::metrics::datastore::DATASTORE_WRITE_OIO_LATEST;
            pub use crate::perf::metrics::datastore::MAX_TOTAL_LATENCY_LATEST;
            pub use crate::perf::metrics::datastore::NUMBER_READ_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::datastore::NUMBER_WRITE_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::datastore::READ_AVERAGE;
            pub use crate::perf::metrics::datastore::SIOC_ACTIVE_TIME_PERCENTAGE_AVERAGE;
            pub use crate::perf::metrics::datastore::SIZE_NORMALIZED_DATASTORE_LATENCY_AVERAGE;
            pub use crate::perf::metrics::datastore::THROUGHPUT_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::datastore::THROUGHPUT_USAGE_AVERAGE;
            pub use crate::perf::metrics::datastore::TOTAL_READ_LATENCY_AVERAGE;
            pub use crate::perf::metrics::datastore::TOTAL_WRITE_LATENCY_AVERAGE;
            pub use crate::perf::metrics::datastore::UNMAP_IOS_SUMMATION;
            pub use crate::perf::metrics::datastore::UNMAP_SIZE_SUMMATION;
            pub use crate::perf::metrics::datastore::VMFS_APD_COUNT_LATEST;
            pub use crate::perf::metrics::datastore::VMFS_APD_STATE_LATEST;
            pub use crate::perf::metrics::datastore::VMFS_CANCELLED_TXN_LATEST;
            pub use crate::perf::metrics::datastore::VMFS_TOTAL_TXN_LATEST;
            pub use crate::perf::metrics::datastore::WRITE_AVERAGE;
        }

        pub mod disk {
            pub use crate::perf::metrics::disk::ACTUALUSED_LATEST;
            pub use crate::perf::metrics::disk::BUS_RESETS_SUMMATION;
            pub use crate::perf::metrics::disk::CAPACITY_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::disk::CAPACITY_PROVISIONED_AVERAGE;
            pub use crate::perf::metrics::disk::CAPACITY_USAGE_AVERAGE;
            pub use crate::perf::metrics::disk::COMMANDS_ABORTED_SUMMATION;
            pub use crate::perf::metrics::disk::COMMANDS_ACTIVE_LATEST;
            pub use crate::perf::metrics::disk::COMMANDS_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::disk::COMMANDS_FAILED_LATEST;
            pub use crate::perf::metrics::disk::COMMANDS_QUEUED_LATEST;
            pub use crate::perf::metrics::disk::COMMANDS_SUCCESS_LATEST;
            pub use crate::perf::metrics::disk::COMMANDS_SUMMATION;
            pub use crate::perf::metrics::disk::DELTAUSED_LATEST;
            pub use crate::perf::metrics::disk::DEVICE_LATENCY_AVERAGE;
            pub use crate::perf::metrics::disk::DEVICE_READ_LATENCY_AVERAGE;
            pub use crate::perf::metrics::disk::DEVICE_WRITE_LATENCY_AVERAGE;
            pub use crate::perf::metrics::disk::KERNEL_LATENCY_AVERAGE;
            pub use crate::perf::metrics::disk::KERNEL_READ_LATENCY_AVERAGE;
            pub use crate::perf::metrics::disk::KERNEL_WRITE_LATENCY_AVERAGE;
            pub use crate::perf::metrics::disk::LATENCY_DAVG_LATEST;
            pub use crate::perf::metrics::disk::LATENCY_GAVG_LATEST;
            pub use crate::perf::metrics::disk::LATENCY_KAVG_LATEST;
            pub use crate::perf::metrics::disk::LATENCY_QAVG_LATEST;
            pub use crate::perf::metrics::disk::MAX_QUEUE_DEPTH_AVERAGE;
            pub use crate::perf::metrics::disk::MAX_TOTAL_LATENCY_LATEST;
            pub use crate::perf::metrics::disk::NUMBER_READ_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::disk::NUMBER_READ_SUMMATION;
            pub use crate::perf::metrics::disk::NUMBER_WRITE_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::disk::NUMBER_WRITE_SUMMATION;
            pub use crate::perf::metrics::disk::QUEUE_LATENCY_AVERAGE;
            pub use crate::perf::metrics::disk::QUEUE_READ_LATENCY_AVERAGE;
            pub use crate::perf::metrics::disk::QUEUE_WRITE_LATENCY_AVERAGE;
            pub use crate::perf::metrics::disk::READ_AVERAGE;
            pub use crate::perf::metrics::disk::READ_FAILED_LATEST;
            pub use crate::perf::metrics::disk::READ_LATEST;
            pub use crate::perf::metrics::disk::SCSI_RESERVATION_CNFLCTS_PCT_AVERAGE;
            pub use crate::perf::metrics::disk::SCSI_RESERVATION_CNFLCTS_PCT_AVERAGE_ABSOLUTE;
            pub use crate::perf::metrics::disk::SCSI_RESERVATION_CONFLICTS_SUMMATION;
            pub use crate::perf::metrics::disk::STATE_LATEST;
            pub use crate::perf::metrics::disk::THROUGHPUT_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::disk::THROUGHPUT_USAGE_AVERAGE;
            pub use crate::perf::metrics::disk::TM_ABORT_FAILED_LATEST;
            pub use crate::perf::metrics::disk::TM_ABORT_LATEST;
            pub use crate::perf::metrics::disk::TM_ABORT_RETRY_LATEST;
            pub use crate::perf::metrics::disk::TM_BUS_RESET_FAILED_LATEST;
            pub use crate::perf::metrics::disk::TM_BUS_RESET_LATEST;
            pub use crate::perf::metrics::disk::TM_BUS_RESET_RETRY_LATEST;
            pub use crate::perf::metrics::disk::TM_DEVICE_RESET_FAILED_LATEST;
            pub use crate::perf::metrics::disk::TM_DEVICE_RESET_LATEST;
            pub use crate::perf::metrics::disk::TM_DEVICE_RESET_RETRY_LATEST;
            pub use crate::perf::metrics::disk::TM_LUN_RESET_FAILED_LATEST;
            pub use crate::perf::metrics::disk::TM_LUN_RESET_LATEST;
            pub use crate::perf::metrics::disk::TM_LUN_RESET_RETRY_LATEST;
            pub use crate::perf::metrics::disk::TM_VIRT_RESET_FAILED_LATEST;
            pub use crate::perf::metrics::disk::TM_VIRT_RESET_LATEST;
            pub use crate::perf::metrics::disk::TM_VIRT_RESET_RETRY_LATEST;
            pub use crate::perf::metrics::disk::TOTAL_LATENCY_AVERAGE;
            pub use crate::perf::metrics::disk::TOTAL_READ_LATENCY_AVERAGE;
            pub use crate::perf::metrics::disk::TOTAL_WRITE_LATENCY_AVERAGE;
            pub use crate::perf::metrics::disk::USAGE_AVERAGE;
            pub use crate::perf::metrics::disk::USAGE_MAXIMUM;
            pub use crate::perf::metrics::disk::USAGE_MINIMUM;
            pub use crate::perf::metrics::disk::USAGE_NONE;
            pub use crate::perf::metrics::disk::WRITE_AVERAGE;
            pub use crate::perf::metrics::disk::WRITE_FAILED_LATEST;
            pub use crate::perf::metrics::disk::WRITE_LATEST;
        }

        pub mod hbr {
            pub use crate::perf::metrics::hbr::HBR_DISK_READ_LATENCY_AVERAGE;
            pub use crate::perf::metrics::hbr::HBR_DISK_STALL_LATENCY_AVERAGE;
            pub use crate::perf::metrics::hbr::HBR_DISK_TRANSFER_BYTES_AVERAGE;
            pub use crate::perf::metrics::hbr::HBR_DISK_TRANSFER_IDLE_AVERAGE;
            pub use crate::perf::metrics::hbr::HBR_DISK_TRANSFER_SUCCESS_AVERAGE;
            pub use crate::perf::metrics::hbr::HBR_NET_LATENCY_AVERAGE;
            pub use crate::perf::metrics::hbr::HBR_NET_RX_AVERAGE;
            pub use crate::perf::metrics::hbr::HBR_NET_TX_AVERAGE;
            pub use crate::perf::metrics::hbr::HBR_NUM_VMS_AVERAGE;
            pub use crate::perf::metrics::hbr::VMS_LATEST;
        }

        pub mod mem {
            pub use crate::perf::metrics::mem::ACTIVEWRITE_AVERAGE;
            pub use crate::perf::metrics::mem::ACTIVE_AVERAGE;
            pub use crate::perf::metrics::mem::ACTIVE_MAXIMUM;
            pub use crate::perf::metrics::mem::ACTIVE_MINIMUM;
            pub use crate::perf::metrics::mem::ACTIVE_NONE;
            pub use crate::perf::metrics::mem::BANDWIDTH_READ_LATEST;
            pub use crate::perf::metrics::mem::BANDWIDTH_TOTAL_LATEST;
            pub use crate::perf::metrics::mem::BANDWIDTH_WRITE_LATEST;
            pub use crate::perf::metrics::mem::CAPACITY_ACTIVE_LATEST;
            pub use crate::perf::metrics::mem::CAPACITY_CONSUMED_LATEST;
            pub use crate::perf::metrics::mem::CAPACITY_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_ENTITLEMENT_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_OVERHEAD_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_OVERHEAD_RESV_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_PROVISIONED_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_PROVISIONED_LATEST;
            pub use crate::perf::metrics::mem::CAPACITY_USABLE_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_USAGE_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_USAGE_USERWORLD_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_USAGE_VMK_OVRHD_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_USAGE_VM_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_USAGE_VM_OVRHD_AVERAGE;
            pub use crate::perf::metrics::mem::COMPRESSED_AVERAGE;
            pub use crate::perf::metrics::mem::COMPRESSION_RATE_AVERAGE;
            pub use crate::perf::metrics::mem::CONSUMED_AVERAGE;
            pub use crate::perf::metrics::mem::CONSUMED_MAXIMUM;
            pub use crate::perf::metrics::mem::CONSUMED_MINIMUM;
            pub use crate::perf::metrics::mem::CONSUMED_NONE;
            pub use crate::perf::metrics::mem::CONSUMED_USERWORLDS_AVERAGE;
            pub use crate::perf::metrics::mem::CONSUMED_VMS_AVERAGE;
            pub use crate::perf::metrics::mem::DECOMPRESSION_RATE_AVERAGE;
            pub use crate::perf::metrics::mem::GRANTED_AVERAGE;
            pub use crate::perf::metrics::mem::GRANTED_MAXIMUM;
            pub use crate::perf::metrics::mem::GRANTED_MINIMUM;
            pub use crate::perf::metrics::mem::GRANTED_NONE;
            pub use crate::perf::metrics::mem::HEALTH_RESERVATION_STATE_LATEST;
            pub use crate::perf::metrics::mem::HEAPFREE_AVERAGE;
            pub use crate::perf::metrics::mem::HEAPFREE_MAXIMUM;
            pub use crate::perf::metrics::mem::HEAPFREE_MINIMUM;
            pub use crate::perf::metrics::mem::HEAPFREE_NONE;
            pub use crate::perf::metrics::mem::HEAP_AVERAGE;
            pub use crate::perf::metrics::mem::HEAP_MAXIMUM;
            pub use crate::perf::metrics::mem::HEAP_MINIMUM;
            pub use crate::perf::metrics::mem::HEAP_NONE;
            pub use crate::perf::metrics::mem::KERNEL_MANAGED_LATEST;
            pub use crate::perf::metrics::mem::KERNEL_MINFREE_LATEST;
            pub use crate::perf::metrics::mem::KERNEL_UNRESERVED_LATEST;
            pub use crate::perf::metrics::mem::LATENCY_AVERAGE;
            pub use crate::perf::metrics::mem::LATENCY_READ_LATEST;
            pub use crate::perf::metrics::mem::LATENCY_WRITE_LATEST;
            pub use crate::perf::metrics::mem::LL_SWAP_IN_AVERAGE;
            pub use crate::perf::metrics::mem::LL_SWAP_IN_MAXIMUM;
            pub use crate::perf::metrics::mem::LL_SWAP_IN_MINIMUM;
            pub use crate::perf::metrics::mem::LL_SWAP_IN_NONE;
            pub use crate::perf::metrics::mem::LL_SWAP_IN_RATE_AVERAGE;
            pub use crate::perf::metrics::mem::LL_SWAP_OUT_AVERAGE;
            pub use crate::perf::metrics::mem::LL_SWAP_OUT_MAXIMUM;
            pub use crate::perf::metrics::mem::LL_SWAP_OUT_MINIMUM;
            pub use crate::perf::metrics::mem::LL_SWAP_OUT_NONE;
            pub use crate::perf::metrics::mem::LL_SWAP_OUT_RATE_AVERAGE;
            pub use crate::perf::metrics::mem::LL_SWAP_USED_AVERAGE;
            pub use crate::perf::metrics::mem::LL_SWAP_USED_MAXIMUM;
            pub use crate::perf::metrics::mem::LL_SWAP_USED_MINIMUM;
            pub use crate::perf::metrics::mem::LL_SWAP_USED_NONE;
            pub use crate::perf::metrics::mem::LOWFREETHRESHOLD_AVERAGE;
            pub use crate::perf::metrics::mem::MEMCTL_CURRENT_LATEST;
            pub use crate::perf::metrics::mem::MEMCTL_MAX_LATEST;
            pub use crate::perf::metrics::mem::MEMCTL_TARGET_LATEST;
            pub use crate::perf::metrics::mem::MISSRATE_LATEST;
            pub use crate::perf::metrics::mem::OVERCOMMIT_AVG15MIN_LATEST;
            pub use crate::perf::metrics::mem::OVERCOMMIT_AVG1MIN_LATEST;
            pub use crate::perf::metrics::mem::OVERCOMMIT_AVG5MIN_LATEST;
            pub use crate::perf::metrics::mem::OVERHEAD_AVERAGE;
            pub use crate::perf::metrics::mem::OVERHEAD_MAXIMUM;
            pub use crate::perf::metrics::mem::OVERHEAD_MINIMUM;
            pub use crate::perf::metrics::mem::OVERHEAD_NONE;
            pub use crate::perf::metrics::mem::PHYSICAL_FREE_LATEST;
            pub use crate::perf::metrics::mem::PHYSICAL_TOTAL_LATEST;
            pub use crate::perf::metrics::mem::PHYSICAL_USER_LATEST;
            pub use crate::perf::metrics::mem::PSHARE_COMMON_LATEST;
            pub use crate::perf::metrics::mem::PSHARE_SHARED_LATEST;
            pub use crate::perf::metrics::mem::PSHARE_SHARED_SAVE_LATEST;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_AVERAGE;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_PCT_AVERAGE;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_PCT_LATEST;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_USERWORLD_AVERAGE;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_VMK_OVRHD_AVERAGE;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_VM_AVERAGE;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_VM_OVHD_AVERAGE;
            pub use crate::perf::metrics::mem::SHAREDCOMMON_AVERAGE;
            pub use crate::perf::metrics::mem::SHAREDCOMMON_MAXIMUM;
            pub use crate::perf::metrics::mem::SHAREDCOMMON_MINIMUM;
            pub use crate::perf::metrics::mem::SHAREDCOMMON_NONE;
            pub use crate::perf::metrics::mem::SHARED_AVERAGE;
            pub use crate::perf::metrics::mem::SHARED_MAXIMUM;
            pub use crate::perf::metrics::mem::SHARED_MINIMUM;
            pub use crate::perf::metrics::mem::SHARED_NONE;
            pub use crate::perf::metrics::mem::STATE_LATEST;
            pub use crate::perf::metrics::mem::SWAPIN_AVERAGE;
            pub use crate::perf::metrics::mem::SWAPIN_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAPIN_MINIMUM;
            pub use crate::perf::metrics::mem::SWAPIN_NONE;
            pub use crate::perf::metrics::mem::SWAPIN_RATE_AVERAGE;
            pub use crate::perf::metrics::mem::SWAPOUT_AVERAGE;
            pub use crate::perf::metrics::mem::SWAPOUT_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAPOUT_MINIMUM;
            pub use crate::perf::metrics::mem::SWAPOUT_NONE;
            pub use crate::perf::metrics::mem::SWAPOUT_RATE_AVERAGE;
            pub use crate::perf::metrics::mem::SWAPUNRESERVED_AVERAGE;
            pub use crate::perf::metrics::mem::SWAPUNRESERVED_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAPUNRESERVED_MINIMUM;
            pub use crate::perf::metrics::mem::SWAPUNRESERVED_NONE;
            pub use crate::perf::metrics::mem::SWAPUSED_AVERAGE;
            pub use crate::perf::metrics::mem::SWAPUSED_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAPUSED_MINIMUM;
            pub use crate::perf::metrics::mem::SWAPUSED_NONE;
            pub use crate::perf::metrics::mem::SWAP_CURRENT_LATEST;
            pub use crate::perf::metrics::mem::SWAP_IN_AVERAGE;
            pub use crate::perf::metrics::mem::SWAP_IN_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAP_IN_MINIMUM;
            pub use crate::perf::metrics::mem::SWAP_IN_NONE;
            pub use crate::perf::metrics::mem::SWAP_OUT_AVERAGE;
            pub use crate::perf::metrics::mem::SWAP_OUT_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAP_OUT_MINIMUM;
            pub use crate::perf::metrics::mem::SWAP_OUT_NONE;
            pub use crate::perf::metrics::mem::SWAP_READRATE_AVERAGE;
            pub use crate::perf::metrics::mem::SWAP_TARGET_LATEST;
            pub use crate::perf::metrics::mem::SWAP_WRITERATE_AVERAGE;
            pub use crate::perf::metrics::mem::SYS_USAGE_AVERAGE;
            pub use crate::perf::metrics::mem::SYS_USAGE_MAXIMUM;
            pub use crate::perf::metrics::mem::SYS_USAGE_MINIMUM;
            pub use crate::perf::metrics::mem::SYS_USAGE_NONE;
            pub use crate::perf::metrics::mem::TOTALMB_AVERAGE;
            pub use crate::perf::metrics::mem::TOTAL_CAPACITY_AVERAGE;
            pub use crate::perf::metrics::mem::UNRESERVED_AVERAGE;
            pub use crate::perf::metrics::mem::UNRESERVED_MAXIMUM;
            pub use crate::perf::metrics::mem::UNRESERVED_MINIMUM;
            pub use crate::perf::metrics::mem::UNRESERVED_NONE;
            pub use crate::perf::metrics::mem::USAGE_AVERAGE;
            pub use crate::perf::metrics::mem::USAGE_MAXIMUM;
            pub use crate::perf::metrics::mem::USAGE_MINIMUM;
            pub use crate::perf::metrics::mem::USAGE_NONE;
            pub use crate::perf::metrics::mem::VMFS_PBC_CAP_MISS_RATIO_LATEST;
            pub use crate::perf::metrics::mem::VMFS_PBC_OVERHEAD_LATEST;
            pub use crate::perf::metrics::mem::VMFS_PBC_SIZE_LATEST;
            pub use crate::perf::metrics::mem::VMFS_PBC_SIZE_MAX_LATEST;
            pub use crate::perf::metrics::mem::VMFS_PBC_WORKING_SET_LATEST;
            pub use crate::perf::metrics::mem::VMFS_PBC_WORKING_SET_MAX_LATEST;
            pub use crate::perf::metrics::mem::VMMEMCTL_AVERAGE;
            pub use crate::perf::metrics::mem::VMMEMCTL_MAXIMUM;
            pub use crate::perf::metrics::mem::VMMEMCTL_MINIMUM;
            pub use crate::perf::metrics::mem::VMMEMCTL_NONE;
            pub use crate::perf::metrics::mem::VM_BANDWIDTH_READ_LATEST;
            pub use crate::perf::metrics::mem::ZERO_AVERAGE;
            pub use crate::perf::metrics::mem::ZERO_MAXIMUM;
            pub use crate::perf::metrics::mem::ZERO_MINIMUM;
            pub use crate::perf::metrics::mem::ZERO_NONE;
            pub use crate::perf::metrics::mem::ZIP_SAVED_LATEST;
            pub use crate::perf::metrics::mem::ZIP_ZIPPED_LATEST;
        }

        pub mod net {
            pub use crate::perf::metrics::net::BROADCAST_RX_SUMMATION;
            pub use crate::perf::metrics::net::BROADCAST_TX_SUMMATION;
            pub use crate::perf::metrics::net::BYTES_RX_AVERAGE;
            pub use crate::perf::metrics::net::BYTES_TX_AVERAGE;
            pub use crate::perf::metrics::net::DROPPED_RX_SUMMATION;
            pub use crate::perf::metrics::net::DROPPED_TX_SUMMATION;
            pub use crate::perf::metrics::net::ERRORS_RX_SUMMATION;
            pub use crate::perf::metrics::net::ERRORS_TX_SUMMATION;
            pub use crate::perf::metrics::net::LATENCY_HBR_OUTBOUND_LATEST;
            pub use crate::perf::metrics::net::MULTICAST_RX_SUMMATION;
            pub use crate::perf::metrics::net::MULTICAST_TX_SUMMATION;
            pub use crate::perf::metrics::net::PACKETS_RX_SUMMATION;
            pub use crate::perf::metrics::net::PACKETS_TX_SUMMATION;
            pub use crate::perf::metrics::net::RECEIVED_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_CONTENTION_SUMMATION;
            pub use crate::perf::metrics::net::THROUGHPUT_DROPPED_RX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_DROPPED_TX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_HBR_INBOUND_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_HBR_OUTBOUND_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_PACKETS_PER_SEC_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_PKTS_RX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_PKTS_RX_BROADCAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_PKTS_RX_MULTICAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_PKTS_TX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_PKTS_TX_BROADCAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_PKTS_TX_MULTICAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_PROVISIONED_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_USABLE_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_USAGE_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_USAGE_FT_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_USAGE_HBR_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_USAGE_ISCSI_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_USAGE_NFS_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_USAGE_VMOTION_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_USAGE_VM_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_DROPPED_RX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_DROPPED_TX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_LAG_DROP_RX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_LAG_DROP_TX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_LAG_RX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_LAG_RX_BCAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_LAG_RX_MCAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_LAG_TX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_LAG_TX_BCAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_LAG_TX_MCAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_PKTS_RX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_PKTS_RX_BCAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_PKTS_RX_MCAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_PKTS_TX_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_PKTS_TX_BCAST_AVERAGE;
            pub use crate::perf::metrics::net::THROUGHPUT_VDS_PKTS_TX_MCAST_AVERAGE;
            pub use crate::perf::metrics::net::TRANSMITTED_AVERAGE;
            pub use crate::perf::metrics::net::UNKNOWN_PROTOS_SUMMATION;
            pub use crate::perf::metrics::net::USAGE_AVERAGE;
            pub use crate::perf::metrics::net::USAGE_MAXIMUM;
            pub use crate::perf::metrics::net::USAGE_MINIMUM;
            pub use crate::perf::metrics::net::USAGE_NONE;
        }

        pub mod power {
            pub use crate::perf::metrics::power::CAPACITY_USABLE_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_CPU_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_IDLE_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_MEM_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_OTHER_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_PCT_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_SYSTEM_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_VM_AVERAGE;
            pub use crate::perf::metrics::power::ENERGY_SUMMATION;
            pub use crate::perf::metrics::power::POWER_AVERAGE;
            pub use crate::perf::metrics::power::POWER_CAP_AVERAGE;
        }

        pub mod rescpu {
            pub use crate::perf::metrics::rescpu::ACTAV15_LATEST;
            pub use crate::perf::metrics::rescpu::ACTAV1_LATEST;
            pub use crate::perf::metrics::rescpu::ACTAV5_LATEST;
            pub use crate::perf::metrics::rescpu::ACTPK15_LATEST;
            pub use crate::perf::metrics::rescpu::ACTPK1_LATEST;
            pub use crate::perf::metrics::rescpu::ACTPK5_LATEST;
            pub use crate::perf::metrics::rescpu::MAX_LIMITED15_LATEST;
            pub use crate::perf::metrics::rescpu::MAX_LIMITED1_LATEST;
            pub use crate::perf::metrics::rescpu::MAX_LIMITED5_LATEST;
            pub use crate::perf::metrics::rescpu::RUNAV15_LATEST;
            pub use crate::perf::metrics::rescpu::RUNAV1_LATEST;
            pub use crate::perf::metrics::rescpu::RUNAV5_LATEST;
            pub use crate::perf::metrics::rescpu::RUNPK15_LATEST;
            pub use crate::perf::metrics::rescpu::RUNPK1_LATEST;
            pub use crate::perf::metrics::rescpu::RUNPK5_LATEST;
            pub use crate::perf::metrics::rescpu::SAMPLE_COUNT_LATEST;
            pub use crate::perf::metrics::rescpu::SAMPLE_PERIOD_LATEST;
        }

        pub mod storage_adapter {
            pub use crate::perf::metrics::storage_adapter::COMMANDS_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::storage_adapter::MAX_TOTAL_LATENCY_LATEST;
            pub use crate::perf::metrics::storage_adapter::NUMBER_READ_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::storage_adapter::NUMBER_WRITE_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::storage_adapter::OIOS_PCT_AVERAGE;
            pub use crate::perf::metrics::storage_adapter::OUTSTANDING_IOS_AVERAGE;
            pub use crate::perf::metrics::storage_adapter::OUTSTANDING_IOS_LATEST;
            pub use crate::perf::metrics::storage_adapter::QUEUED_AVERAGE;
            pub use crate::perf::metrics::storage_adapter::QUEUED_LATEST;
            pub use crate::perf::metrics::storage_adapter::QUEUE_DEPTH_AVERAGE;
            pub use crate::perf::metrics::storage_adapter::QUEUE_DEPTH_LATEST;
            pub use crate::perf::metrics::storage_adapter::QUEUE_LATENCY_AVERAGE;
            pub use crate::perf::metrics::storage_adapter::READ_AVERAGE;
            pub use crate::perf::metrics::storage_adapter::THROUGHPUT_CONT_AVERAGE;
            pub use crate::perf::metrics::storage_adapter::THROUGHPUT_USAG_AVERAGE;
            pub use crate::perf::metrics::storage_adapter::TOTAL_READ_LATENCY_AVERAGE;
            pub use crate::perf::metrics::storage_adapter::TOTAL_WRITE_LATENCY_AVERAGE;
            pub use crate::perf::metrics::storage_adapter::WRITE_AVERAGE;
        }

        pub mod storage_path {
            pub use crate::perf::metrics::storage_path::BUS_RESETS_SUMMATION;
            pub use crate::perf::metrics::storage_path::COMMANDS_ABORTED_SUMMATION;
            pub use crate::perf::metrics::storage_path::COMMANDS_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::storage_path::MAX_TOTAL_LATENCY_LATEST;
            pub use crate::perf::metrics::storage_path::NUMBER_READ_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::storage_path::NUMBER_WRITE_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::storage_path::READ_AVERAGE;
            pub use crate::perf::metrics::storage_path::THROUGHPUT_CONT_AVERAGE;
            pub use crate::perf::metrics::storage_path::THROUGHPUT_USAGE_AVERAGE;
            pub use crate::perf::metrics::storage_path::TOTAL_READ_LATENCY_AVERAGE;
            pub use crate::perf::metrics::storage_path::TOTAL_WRITE_LATENCY_AVERAGE;
            pub use crate::perf::metrics::storage_path::WRITE_AVERAGE;
        }

        pub mod sys {
            pub use crate::perf::metrics::sys::DISK_USAGE_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_CPU_ACT1_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_CPU_ACT5_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_CPU_ALLOC_MAX_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_CPU_ALLOC_MIN_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_CPU_ALLOC_SHARES_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_CPU_MAX_LIMITED1_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_CPU_MAX_LIMITED5_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_CPU_RUN1_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_CPU_RUN5_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_CPU_USAGE_AVERAGE;
            pub use crate::perf::metrics::sys::RESOURCE_CPU_USAGE_MAXIMUM;
            pub use crate::perf::metrics::sys::RESOURCE_CPU_USAGE_MINIMUM;
            pub use crate::perf::metrics::sys::RESOURCE_CPU_USAGE_NONE;
            pub use crate::perf::metrics::sys::RESOURCE_FD_USAGE_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_MEM_ALLOC_MAX_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_MEM_ALLOC_MIN_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_MEM_ALLOC_SHARES_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_MEM_CONSUMED_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_MEM_COW_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_MEM_MAPPED_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_MEM_OVERHEAD_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_MEM_SHARED_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_MEM_SWAPPED_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_MEM_TOUCHED_LATEST;
            pub use crate::perf::metrics::sys::RESOURCE_MEM_ZERO_LATEST;
            pub use crate::perf::metrics::sys::UPTIME_LATEST;
        }
    }

    pub mod cluster_compute_resource {
        pub mod cluster_services {
            pub use crate::perf::metrics::cluster_services::CLUSTER_DRS_SCORE_LATEST;
            pub use crate::perf::metrics::cluster_services::EFFECTIVECPU_AVERAGE;
            pub use crate::perf::metrics::cluster_services::EFFECTIVEMEM_AVERAGE;
            pub use crate::perf::metrics::cluster_services::FAILOVER_LATEST;
        }

        pub mod cpu {
            pub use crate::perf::metrics::cpu::CACHE_L3_OCCUPANCY_AVERAGE;
            pub use crate::perf::metrics::cpu::CAPACITY_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::cpu::CAPACITY_DEMAND_AVERAGE;
            pub use crate::perf::metrics::cpu::CAPACITY_ENTITLEMENT_AVERAGE;
            pub use crate::perf::metrics::cpu::CAPACITY_PROVISIONED_AVERAGE;
            pub use crate::perf::metrics::cpu::CAPACITY_USAGE_AVERAGE;
            pub use crate::perf::metrics::cpu::CORECOUNT_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::cpu::CORECOUNT_PROVISIONED_AVERAGE;
            pub use crate::perf::metrics::cpu::CORECOUNT_PROVISIONED_LATEST;
            pub use crate::perf::metrics::cpu::CORECOUNT_USAGE_AVERAGE;
            pub use crate::perf::metrics::cpu::CORECOUNT_USAGE_LATEST;
            pub use crate::perf::metrics::cpu::LOAD_AVG15MIN_LATEST;
            pub use crate::perf::metrics::cpu::LOAD_AVG1MIN_LATEST;
            pub use crate::perf::metrics::cpu::LOAD_AVG5MIN_LATEST;
            pub use crate::perf::metrics::cpu::PARTNER_BUSY_TIME_AVERAGE;
            pub use crate::perf::metrics::cpu::RESERVED_CAPACITY_AVERAGE;
            pub use crate::perf::metrics::cpu::TOTALMHZ_AVERAGE;
            pub use crate::perf::metrics::cpu::USAGEMHZ_AVERAGE;
            pub use crate::perf::metrics::cpu::USAGEMHZ_MAXIMUM;
            pub use crate::perf::metrics::cpu::USAGEMHZ_MINIMUM;
            pub use crate::perf::metrics::cpu::USAGEMHZ_NONE;
            pub use crate::perf::metrics::cpu::USAGE_AVERAGE;
            pub use crate::perf::metrics::cpu::USAGE_MAXIMUM;
            pub use crate::perf::metrics::cpu::USAGE_MINIMUM;
            pub use crate::perf::metrics::cpu::USAGE_NONE;
        }

        pub mod gpu {
            pub use crate::perf::metrics::gpu::MEM_RESERVED_GB_LATEST;
            pub use crate::perf::metrics::gpu::MEM_RESERVED_LATEST;
            pub use crate::perf::metrics::gpu::MEM_TOTAL_GB_LATEST;
            pub use crate::perf::metrics::gpu::MEM_TOTAL_LATEST;
            pub use crate::perf::metrics::gpu::MEM_USAGE_AVERAGE;
            pub use crate::perf::metrics::gpu::MEM_USAGE_MAXIMUM;
            pub use crate::perf::metrics::gpu::MEM_USAGE_MINIMUM;
            pub use crate::perf::metrics::gpu::MEM_USAGE_NONE;
            pub use crate::perf::metrics::gpu::MEM_USED_AVERAGE;
            pub use crate::perf::metrics::gpu::MEM_USED_GB_LATEST;
            pub use crate::perf::metrics::gpu::MEM_USED_MAXIMUM;
            pub use crate::perf::metrics::gpu::MEM_USED_MINIMUM;
            pub use crate::perf::metrics::gpu::MEM_USED_NONE;
            pub use crate::perf::metrics::gpu::POWER_USED_LATEST;
            pub use crate::perf::metrics::gpu::TEMPERATURE_AVERAGE;
            pub use crate::perf::metrics::gpu::UTILIZATION_AVERAGE;
            pub use crate::perf::metrics::gpu::UTILIZATION_MAXIMUM;
            pub use crate::perf::metrics::gpu::UTILIZATION_MINIMUM;
            pub use crate::perf::metrics::gpu::UTILIZATION_NONE;
        }

        pub mod mem {
            pub use crate::perf::metrics::mem::ACTIVE_AVERAGE;
            pub use crate::perf::metrics::mem::ACTIVE_MAXIMUM;
            pub use crate::perf::metrics::mem::ACTIVE_MINIMUM;
            pub use crate::perf::metrics::mem::ACTIVE_NONE;
            pub use crate::perf::metrics::mem::BANDWIDTH_READ_LATEST;
            pub use crate::perf::metrics::mem::BANDWIDTH_TOTAL_LATEST;
            pub use crate::perf::metrics::mem::BANDWIDTH_WRITE_LATEST;
            pub use crate::perf::metrics::mem::CAPACITY_ACTIVE_LATEST;
            pub use crate::perf::metrics::mem::CAPACITY_CONSUMED_LATEST;
            pub use crate::perf::metrics::mem::CAPACITY_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_ENTITLEMENT_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_OVERHEAD_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_OVERHEAD_RESV_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_PROVISIONED_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_PROVISIONED_LATEST;
            pub use crate::perf::metrics::mem::CAPACITY_USABLE_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_USAGE_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_USAGE_USERWORLD_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_USAGE_VMK_OVRHD_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_USAGE_VM_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_USAGE_VM_OVRHD_AVERAGE;
            pub use crate::perf::metrics::mem::COMPRESSED_AVERAGE;
            pub use crate::perf::metrics::mem::COMPRESSION_RATE_AVERAGE;
            pub use crate::perf::metrics::mem::CONSUMED_AVERAGE;
            pub use crate::perf::metrics::mem::CONSUMED_MAXIMUM;
            pub use crate::perf::metrics::mem::CONSUMED_MINIMUM;
            pub use crate::perf::metrics::mem::CONSUMED_NONE;
            pub use crate::perf::metrics::mem::DECOMPRESSION_RATE_AVERAGE;
            pub use crate::perf::metrics::mem::GRANTED_AVERAGE;
            pub use crate::perf::metrics::mem::GRANTED_MAXIMUM;
            pub use crate::perf::metrics::mem::GRANTED_MINIMUM;
            pub use crate::perf::metrics::mem::GRANTED_NONE;
            pub use crate::perf::metrics::mem::HEALTH_RESERVATION_STATE_LATEST;
            pub use crate::perf::metrics::mem::KERNEL_MANAGED_LATEST;
            pub use crate::perf::metrics::mem::KERNEL_MINFREE_LATEST;
            pub use crate::perf::metrics::mem::KERNEL_UNRESERVED_LATEST;
            pub use crate::perf::metrics::mem::LATENCY_READ_LATEST;
            pub use crate::perf::metrics::mem::LATENCY_WRITE_LATEST;
            pub use crate::perf::metrics::mem::MEMCTL_CURRENT_LATEST;
            pub use crate::perf::metrics::mem::MEMCTL_MAX_LATEST;
            pub use crate::perf::metrics::mem::MEMCTL_TARGET_LATEST;
            pub use crate::perf::metrics::mem::MISSRATE_LATEST;
            pub use crate::perf::metrics::mem::OVERCOMMIT_AVG15MIN_LATEST;
            pub use crate::perf::metrics::mem::OVERCOMMIT_AVG1MIN_LATEST;
            pub use crate::perf::metrics::mem::OVERCOMMIT_AVG5MIN_LATEST;
            pub use crate::perf::metrics::mem::OVERHEAD_AVERAGE;
            pub use crate::perf::metrics::mem::OVERHEAD_MAXIMUM;
            pub use crate::perf::metrics::mem::OVERHEAD_MINIMUM;
            pub use crate::perf::metrics::mem::OVERHEAD_NONE;
            pub use crate::perf::metrics::mem::PHYSICAL_FREE_LATEST;
            pub use crate::perf::metrics::mem::PHYSICAL_TOTAL_LATEST;
            pub use crate::perf::metrics::mem::PHYSICAL_USER_LATEST;
            pub use crate::perf::metrics::mem::PSHARE_COMMON_LATEST;
            pub use crate::perf::metrics::mem::PSHARE_SHARED_LATEST;
            pub use crate::perf::metrics::mem::PSHARE_SHARED_SAVE_LATEST;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_AVERAGE;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_PCT_AVERAGE;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_PCT_LATEST;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_USERWORLD_AVERAGE;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_VMK_OVRHD_AVERAGE;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_VM_AVERAGE;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_VM_OVHD_AVERAGE;
            pub use crate::perf::metrics::mem::SHARED_AVERAGE;
            pub use crate::perf::metrics::mem::SHARED_MAXIMUM;
            pub use crate::perf::metrics::mem::SHARED_MINIMUM;
            pub use crate::perf::metrics::mem::SHARED_NONE;
            pub use crate::perf::metrics::mem::SWAPUNRESERVED_AVERAGE;
            pub use crate::perf::metrics::mem::SWAPUNRESERVED_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAPUNRESERVED_MINIMUM;
            pub use crate::perf::metrics::mem::SWAPUNRESERVED_NONE;
            pub use crate::perf::metrics::mem::SWAPUSED_AVERAGE;
            pub use crate::perf::metrics::mem::SWAPUSED_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAPUSED_MINIMUM;
            pub use crate::perf::metrics::mem::SWAPUSED_NONE;
            pub use crate::perf::metrics::mem::SWAP_CURRENT_LATEST;
            pub use crate::perf::metrics::mem::SWAP_IN_AVERAGE;
            pub use crate::perf::metrics::mem::SWAP_IN_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAP_IN_MINIMUM;
            pub use crate::perf::metrics::mem::SWAP_IN_NONE;
            pub use crate::perf::metrics::mem::SWAP_OUT_AVERAGE;
            pub use crate::perf::metrics::mem::SWAP_OUT_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAP_OUT_MINIMUM;
            pub use crate::perf::metrics::mem::SWAP_OUT_NONE;
            pub use crate::perf::metrics::mem::SWAP_READRATE_AVERAGE;
            pub use crate::perf::metrics::mem::SWAP_TARGET_LATEST;
            pub use crate::perf::metrics::mem::SWAP_WRITERATE_AVERAGE;
            pub use crate::perf::metrics::mem::TOTALMB_AVERAGE;
            pub use crate::perf::metrics::mem::USAGE_AVERAGE;
            pub use crate::perf::metrics::mem::USAGE_MAXIMUM;
            pub use crate::perf::metrics::mem::USAGE_MINIMUM;
            pub use crate::perf::metrics::mem::USAGE_NONE;
            pub use crate::perf::metrics::mem::VMMEMCTL_AVERAGE;
            pub use crate::perf::metrics::mem::VMMEMCTL_MAXIMUM;
            pub use crate::perf::metrics::mem::VMMEMCTL_MINIMUM;
            pub use crate::perf::metrics::mem::VMMEMCTL_NONE;
            pub use crate::perf::metrics::mem::VM_BANDWIDTH_READ_LATEST;
            pub use crate::perf::metrics::mem::ZERO_AVERAGE;
            pub use crate::perf::metrics::mem::ZERO_MAXIMUM;
            pub use crate::perf::metrics::mem::ZERO_MINIMUM;
            pub use crate::perf::metrics::mem::ZERO_NONE;
            pub use crate::perf::metrics::mem::ZIP_SAVED_LATEST;
            pub use crate::perf::metrics::mem::ZIP_ZIPPED_LATEST;
        }

        pub mod power {
            pub use crate::perf::metrics::power::CAPACITY_USABLE_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_CPU_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_MEM_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_OTHER_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_PCT_AVERAGE;
            pub use crate::perf::metrics::power::POWER_AVERAGE;
        }

        pub mod vmop {
            pub use crate::perf::metrics::vmop::NUM_CHANGE_DS_LATEST;
            pub use crate::perf::metrics::vmop::NUM_CHANGE_HOST_DS_LATEST;
            pub use crate::perf::metrics::vmop::NUM_CHANGE_HOST_LATEST;
            pub use crate::perf::metrics::vmop::NUM_CLONE_LATEST;
            pub use crate::perf::metrics::vmop::NUM_CREATE_LATEST;
            pub use crate::perf::metrics::vmop::NUM_DEPLOY_LATEST;
            pub use crate::perf::metrics::vmop::NUM_DESTROY_LATEST;
            pub use crate::perf::metrics::vmop::NUM_POWEROFF_LATEST;
            pub use crate::perf::metrics::vmop::NUM_POWERON_LATEST;
            pub use crate::perf::metrics::vmop::NUM_REBOOT_GUEST_LATEST;
            pub use crate::perf::metrics::vmop::NUM_RECONFIGURE_LATEST;
            pub use crate::perf::metrics::vmop::NUM_REGISTER_LATEST;
            pub use crate::perf::metrics::vmop::NUM_RESET_LATEST;
            pub use crate::perf::metrics::vmop::NUM_SHUTDOWN_GUEST_LATEST;
            pub use crate::perf::metrics::vmop::NUM_STANDBY_GUEST_LATEST;
            pub use crate::perf::metrics::vmop::NUM_SUSPEND_LATEST;
            pub use crate::perf::metrics::vmop::NUM_SVMOTION_LATEST;
            pub use crate::perf::metrics::vmop::NUM_UNREGISTER_LATEST;
            pub use crate::perf::metrics::vmop::NUM_VMOTION_LATEST;
            pub use crate::perf::metrics::vmop::NUM_XVMOTION_LATEST;
        }
    }

    pub mod resource_pool {
        pub mod cpu {
            pub use crate::perf::metrics::cpu::CACHE_L3_OCCUPANCY_AVERAGE;
            pub use crate::perf::metrics::cpu::CAPACITY_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::cpu::CAPACITY_DEMAND_AVERAGE;
            pub use crate::perf::metrics::cpu::CAPACITY_ENTITLEMENT_AVERAGE;
            pub use crate::perf::metrics::cpu::CAPACITY_PROVISIONED_AVERAGE;
            pub use crate::perf::metrics::cpu::CAPACITY_USAGE_AVERAGE;
            pub use crate::perf::metrics::cpu::CORECOUNT_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::cpu::CORECOUNT_PROVISIONED_AVERAGE;
            pub use crate::perf::metrics::cpu::CORECOUNT_PROVISIONED_LATEST;
            pub use crate::perf::metrics::cpu::CORECOUNT_USAGE_AVERAGE;
            pub use crate::perf::metrics::cpu::CORECOUNT_USAGE_LATEST;
            pub use crate::perf::metrics::cpu::CPUENTITLEMENT_LATEST;
            pub use crate::perf::metrics::cpu::LOAD_AVG15MIN_LATEST;
            pub use crate::perf::metrics::cpu::LOAD_AVG1MIN_LATEST;
            pub use crate::perf::metrics::cpu::LOAD_AVG5MIN_LATEST;
            pub use crate::perf::metrics::cpu::PARTNER_BUSY_TIME_AVERAGE;
            pub use crate::perf::metrics::cpu::TOTALMHZ_AVERAGE;
            pub use crate::perf::metrics::cpu::USAGEMHZ_AVERAGE;
            pub use crate::perf::metrics::cpu::USAGEMHZ_MAXIMUM;
            pub use crate::perf::metrics::cpu::USAGEMHZ_MINIMUM;
            pub use crate::perf::metrics::cpu::USAGEMHZ_NONE;
        }

        pub mod gpu {
            pub use crate::perf::metrics::gpu::MEM_RESERVED_GB_LATEST;
            pub use crate::perf::metrics::gpu::MEM_RESERVED_LATEST;
            pub use crate::perf::metrics::gpu::MEM_TOTAL_GB_LATEST;
            pub use crate::perf::metrics::gpu::MEM_TOTAL_LATEST;
            pub use crate::perf::metrics::gpu::MEM_USAGE_AVERAGE;
            pub use crate::perf::metrics::gpu::MEM_USAGE_MAXIMUM;
            pub use crate::perf::metrics::gpu::MEM_USAGE_MINIMUM;
            pub use crate::perf::metrics::gpu::MEM_USAGE_NONE;
            pub use crate::perf::metrics::gpu::MEM_USED_AVERAGE;
            pub use crate::perf::metrics::gpu::MEM_USED_GB_LATEST;
            pub use crate::perf::metrics::gpu::MEM_USED_MAXIMUM;
            pub use crate::perf::metrics::gpu::MEM_USED_MINIMUM;
            pub use crate::perf::metrics::gpu::MEM_USED_NONE;
            pub use crate::perf::metrics::gpu::POWER_USED_LATEST;
            pub use crate::perf::metrics::gpu::TEMPERATURE_AVERAGE;
            pub use crate::perf::metrics::gpu::UTILIZATION_AVERAGE;
            pub use crate::perf::metrics::gpu::UTILIZATION_MAXIMUM;
            pub use crate::perf::metrics::gpu::UTILIZATION_MINIMUM;
            pub use crate::perf::metrics::gpu::UTILIZATION_NONE;
        }

        pub mod mem {
            pub use crate::perf::metrics::mem::ACTIVE_AVERAGE;
            pub use crate::perf::metrics::mem::ACTIVE_MAXIMUM;
            pub use crate::perf::metrics::mem::ACTIVE_MINIMUM;
            pub use crate::perf::metrics::mem::ACTIVE_NONE;
            pub use crate::perf::metrics::mem::BANDWIDTH_READ_LATEST;
            pub use crate::perf::metrics::mem::BANDWIDTH_TOTAL_LATEST;
            pub use crate::perf::metrics::mem::BANDWIDTH_WRITE_LATEST;
            pub use crate::perf::metrics::mem::CAPACITY_ACTIVE_LATEST;
            pub use crate::perf::metrics::mem::CAPACITY_CONSUMED_LATEST;
            pub use crate::perf::metrics::mem::CAPACITY_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_ENTITLEMENT_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_OVERHEAD_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_OVERHEAD_RESV_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_PROVISIONED_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_PROVISIONED_LATEST;
            pub use crate::perf::metrics::mem::CAPACITY_USABLE_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_USAGE_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_USAGE_USERWORLD_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_USAGE_VMK_OVRHD_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_USAGE_VM_AVERAGE;
            pub use crate::perf::metrics::mem::CAPACITY_USAGE_VM_OVRHD_AVERAGE;
            pub use crate::perf::metrics::mem::COMPRESSED_AVERAGE;
            pub use crate::perf::metrics::mem::COMPRESSION_RATE_AVERAGE;
            pub use crate::perf::metrics::mem::CONSUMED_AVERAGE;
            pub use crate::perf::metrics::mem::CONSUMED_MAXIMUM;
            pub use crate::perf::metrics::mem::CONSUMED_MINIMUM;
            pub use crate::perf::metrics::mem::CONSUMED_NONE;
            pub use crate::perf::metrics::mem::DECOMPRESSION_RATE_AVERAGE;
            pub use crate::perf::metrics::mem::GRANTED_AVERAGE;
            pub use crate::perf::metrics::mem::GRANTED_MAXIMUM;
            pub use crate::perf::metrics::mem::GRANTED_MINIMUM;
            pub use crate::perf::metrics::mem::GRANTED_NONE;
            pub use crate::perf::metrics::mem::HEALTH_RESERVATION_STATE_LATEST;
            pub use crate::perf::metrics::mem::KERNEL_MANAGED_LATEST;
            pub use crate::perf::metrics::mem::KERNEL_MINFREE_LATEST;
            pub use crate::perf::metrics::mem::KERNEL_UNRESERVED_LATEST;
            pub use crate::perf::metrics::mem::LATENCY_READ_LATEST;
            pub use crate::perf::metrics::mem::LATENCY_WRITE_LATEST;
            pub use crate::perf::metrics::mem::MEMCTL_CURRENT_LATEST;
            pub use crate::perf::metrics::mem::MEMCTL_MAX_LATEST;
            pub use crate::perf::metrics::mem::MEMCTL_TARGET_LATEST;
            pub use crate::perf::metrics::mem::MEMENTITLEMENT_LATEST;
            pub use crate::perf::metrics::mem::MISSRATE_LATEST;
            pub use crate::perf::metrics::mem::OVERCOMMIT_AVG15MIN_LATEST;
            pub use crate::perf::metrics::mem::OVERCOMMIT_AVG1MIN_LATEST;
            pub use crate::perf::metrics::mem::OVERCOMMIT_AVG5MIN_LATEST;
            pub use crate::perf::metrics::mem::OVERHEAD_AVERAGE;
            pub use crate::perf::metrics::mem::OVERHEAD_MAXIMUM;
            pub use crate::perf::metrics::mem::OVERHEAD_MINIMUM;
            pub use crate::perf::metrics::mem::OVERHEAD_NONE;
            pub use crate::perf::metrics::mem::PHYSICAL_FREE_LATEST;
            pub use crate::perf::metrics::mem::PHYSICAL_TOTAL_LATEST;
            pub use crate::perf::metrics::mem::PHYSICAL_USER_LATEST;
            pub use crate::perf::metrics::mem::PSHARE_COMMON_LATEST;
            pub use crate::perf::metrics::mem::PSHARE_SHARED_LATEST;
            pub use crate::perf::metrics::mem::PSHARE_SHARED_SAVE_LATEST;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_PCT_AVERAGE;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_PCT_LATEST;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_USERWORLD_AVERAGE;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_VMK_OVRHD_AVERAGE;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_VM_AVERAGE;
            pub use crate::perf::metrics::mem::RESERVED_CAPACITY_VM_OVHD_AVERAGE;
            pub use crate::perf::metrics::mem::SHARED_AVERAGE;
            pub use crate::perf::metrics::mem::SHARED_MAXIMUM;
            pub use crate::perf::metrics::mem::SHARED_MINIMUM;
            pub use crate::perf::metrics::mem::SHARED_NONE;
            pub use crate::perf::metrics::mem::SWAPPED_AVERAGE;
            pub use crate::perf::metrics::mem::SWAPPED_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAPPED_MINIMUM;
            pub use crate::perf::metrics::mem::SWAPPED_NONE;
            pub use crate::perf::metrics::mem::SWAPUNRESERVED_AVERAGE;
            pub use crate::perf::metrics::mem::SWAPUNRESERVED_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAPUNRESERVED_MINIMUM;
            pub use crate::perf::metrics::mem::SWAPUNRESERVED_NONE;
            pub use crate::perf::metrics::mem::SWAP_CURRENT_LATEST;
            pub use crate::perf::metrics::mem::SWAP_IN_AVERAGE;
            pub use crate::perf::metrics::mem::SWAP_IN_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAP_IN_MINIMUM;
            pub use crate::perf::metrics::mem::SWAP_IN_NONE;
            pub use crate::perf::metrics::mem::SWAP_OUT_AVERAGE;
            pub use crate::perf::metrics::mem::SWAP_OUT_MAXIMUM;
            pub use crate::perf::metrics::mem::SWAP_OUT_MINIMUM;
            pub use crate::perf::metrics::mem::SWAP_OUT_NONE;
            pub use crate::perf::metrics::mem::SWAP_READRATE_AVERAGE;
            pub use crate::perf::metrics::mem::SWAP_TARGET_LATEST;
            pub use crate::perf::metrics::mem::SWAP_WRITERATE_AVERAGE;
            pub use crate::perf::metrics::mem::TOTALMB_AVERAGE;
            pub use crate::perf::metrics::mem::VMMEMCTL_AVERAGE;
            pub use crate::perf::metrics::mem::VMMEMCTL_MAXIMUM;
            pub use crate::perf::metrics::mem::VMMEMCTL_MINIMUM;
            pub use crate::perf::metrics::mem::VMMEMCTL_NONE;
            pub use crate::perf::metrics::mem::VM_BANDWIDTH_READ_LATEST;
            pub use crate::perf::metrics::mem::ZERO_AVERAGE;
            pub use crate::perf::metrics::mem::ZERO_MAXIMUM;
            pub use crate::perf::metrics::mem::ZERO_MINIMUM;
            pub use crate::perf::metrics::mem::ZERO_NONE;
            pub use crate::perf::metrics::mem::ZIP_SAVED_LATEST;
            pub use crate::perf::metrics::mem::ZIP_ZIPPED_LATEST;
        }

        pub mod power {
            pub use crate::perf::metrics::power::CAPACITY_USABLE_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_CPU_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_MEM_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_OTHER_AVERAGE;
            pub use crate::perf::metrics::power::CAPACITY_USAGE_PCT_AVERAGE;
            pub use crate::perf::metrics::power::POWER_AVERAGE;
        }
    }

    pub mod datastore {
        pub mod datastore {
            pub use crate::perf::metrics::datastore::BUS_RESETS_SUMMATION;
            pub use crate::perf::metrics::datastore::COMMANDS_ABORTED_SUMMATION;
            pub use crate::perf::metrics::datastore::NUMBER_READ_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::datastore::NUMBER_WRITE_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::datastore::READ_AVERAGE;
            pub use crate::perf::metrics::datastore::THROUGHPUT_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::datastore::THROUGHPUT_USAGE_AVERAGE;
            pub use crate::perf::metrics::datastore::VMFS_APD_COUNT_LATEST;
            pub use crate::perf::metrics::datastore::VMFS_APD_STATE_LATEST;
            pub use crate::perf::metrics::datastore::VMFS_CANCELLED_TXN_LATEST;
            pub use crate::perf::metrics::datastore::VMFS_TOTAL_TXN_LATEST;
            pub use crate::perf::metrics::datastore::WRITE_AVERAGE;
        }

        pub mod disk {
            pub use crate::perf::metrics::disk::ACTUALUSED_LATEST;
            pub use crate::perf::metrics::disk::BUS_RESETS_SUMMATION;
            pub use crate::perf::metrics::disk::CAPACITY_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::disk::CAPACITY_LATEST;
            pub use crate::perf::metrics::disk::CAPACITY_PROVISIONED_AVERAGE;
            pub use crate::perf::metrics::disk::CAPACITY_USAGE_AVERAGE;
            pub use crate::perf::metrics::disk::COMMANDS_ABORTED_SUMMATION;
            pub use crate::perf::metrics::disk::COMMANDS_ACTIVE_LATEST;
            pub use crate::perf::metrics::disk::COMMANDS_FAILED_LATEST;
            pub use crate::perf::metrics::disk::COMMANDS_QUEUED_LATEST;
            pub use crate::perf::metrics::disk::COMMANDS_SUCCESS_LATEST;
            pub use crate::perf::metrics::disk::DELTAUSED_LATEST;
            pub use crate::perf::metrics::disk::LATENCY_DAVG_LATEST;
            pub use crate::perf::metrics::disk::LATENCY_GAVG_LATEST;
            pub use crate::perf::metrics::disk::LATENCY_KAVG_LATEST;
            pub use crate::perf::metrics::disk::LATENCY_QAVG_LATEST;
            pub use crate::perf::metrics::disk::NUMBER_READ_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::disk::NUMBER_WRITE_AVERAGED_AVERAGE;
            pub use crate::perf::metrics::disk::PROVISIONED_LATEST;
            pub use crate::perf::metrics::disk::READ_AVERAGE;
            pub use crate::perf::metrics::disk::READ_FAILED_LATEST;
            pub use crate::perf::metrics::disk::READ_LATEST;
            pub use crate::perf::metrics::disk::SCSI_RESERVATION_CNFLCTS_PCT_AVERAGE;
            pub use crate::perf::metrics::disk::SCSI_RESERVATION_CNFLCTS_PCT_AVERAGE_ABSOLUTE;
            pub use crate::perf::metrics::disk::STATE_LATEST;
            pub use crate::perf::metrics::disk::THROUGHPUT_CONTENTION_AVERAGE;
            pub use crate::perf::metrics::disk::THROUGHPUT_USAGE_AVERAGE;
            pub use crate::perf::metrics::disk::TM_ABORT_FAILED_LATEST;
            pub use crate::perf::metrics::disk::TM_ABORT_LATEST;
            pub use crate::perf::metrics::disk::TM_ABORT_RETRY_LATEST;
            pub use crate::perf::metrics::disk::TM_BUS_RESET_FAILED_LATEST;
            pub use crate::perf::metrics::disk::TM_BUS_RESET_LATEST;
            pub use crate::perf::metrics::disk::TM_BUS_RESET_RETRY_LATEST;
            pub use crate::perf::metrics::disk::TM_DEVICE_RESET_FAILED_LATEST;
            pub use crate::perf::metrics::disk::TM_DEVICE_RESET_LATEST;
            pub use crate::perf::metrics::disk::TM_DEVICE_RESET_RETRY_LATEST;
            pub use crate::perf::metrics::disk::TM_LUN_RESET_FAILED_LATEST;
            pub use crate::perf::metrics::disk::TM_LUN_RESET_LATEST;
            pub use crate::perf::metrics::disk::TM_LUN_RESET_RETRY_LATEST;
            pub use crate::perf::metrics::disk::TM_VIRT_RESET_FAILED_LATEST;
            pub use crate::perf::metrics::disk::TM_VIRT_RESET_LATEST;
            pub use crate::perf::metrics::disk::TM_VIRT_RESET_RETRY_LATEST;
            pub use crate::perf::metrics::disk::USED_LATEST;
            pub use crate::perf::metrics::disk::WRITE_AVERAGE;
            pub use crate::perf::metrics::disk::WRITE_FAILED_LATEST;
            pub use crate::perf::metrics::disk::WRITE_LATEST;
        }
    }

    pub mod datacenter {
        pub mod vmop {
            pub use crate::perf::metrics::vmop::NUM_CHANGE_DS_LATEST;
            pub use crate::perf::metrics::vmop::NUM_CHANGE_HOST_DS_LATEST;
            pub use crate::perf::metrics::vmop::NUM_CHANGE_HOST_LATEST;
            pub use crate::perf::metrics::vmop::NUM_CLONE_LATEST;
            pub use crate::perf::metrics::vmop::NUM_CREATE_LATEST;
            pub use crate::perf::metrics::vmop::NUM_DEPLOY_LATEST;
            pub use crate::perf::metrics::vmop::NUM_DESTROY_LATEST;
            pub use crate::perf::metrics::vmop::NUM_POWEROFF_LATEST;
            pub use crate::perf::metrics::vmop::NUM_POWERON_LATEST;
            pub use crate::perf::metrics::vmop::NUM_REBOOT_GUEST_LATEST;
            pub use crate::perf::metrics::vmop::NUM_RECONFIGURE_LATEST;
            pub use crate::perf::metrics::vmop::NUM_REGISTER_LATEST;
            pub use crate::perf::metrics::vmop::NUM_RESET_LATEST;
            pub use crate::perf::metrics::vmop::NUM_SHUTDOWN_GUEST_LATEST;
            pub use crate::perf::metrics::vmop::NUM_STANDBY_GUEST_LATEST;
            pub use crate::perf::metrics::vmop::NUM_SUSPEND_LATEST;
            pub use crate::perf::metrics::vmop::NUM_SVMOTION_LATEST;
            pub use crate::perf::metrics::vmop::NUM_UNREGISTER_LATEST;
            pub use crate::perf::metrics::vmop::NUM_VMOTION_LATEST;
            pub use crate::perf::metrics::vmop::NUM_XVMOTION_LATEST;
        }
    }
}
