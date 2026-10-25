//! Generated from `vim_build/data/perf-catalog.json`. Do not edit.
//!
//! Counter constants and per-kind re-exports. Scopes come from the catalog
//! availability rows and the tier 3a / 3b tables.

pub mod cluster_services {
    use super::super::*;

    /// `clusterServices.clusterDrsScore.latest` (key 276) · percent (hundredths) · level 1, per_device_level 1 · Tier 3a: Host, VM
    pub const CLUSTER_DRS_SCORE_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "clusterServices",
        "clusterDrsScore",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `clusterServices.cpufairness.latest` (key 226) · number · level 1, per_device_level 3 · Tier 1: Host
    pub const CPUFAIRNESS_LATEST: Metric<Count, HostOnly> = Metric::new(
        "clusterServices",
        "cpufairness",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `clusterServices.effectivecpu.average` (key 272) · megaHertz · level 1, per_device_level 3 · Tier 3a: Host, VM
    pub const EFFECTIVECPU_AVERAGE: Metric<Frequency, VmOrHost> = Metric::new(
        "clusterServices",
        "effectivecpu",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `clusterServices.effectivemem.average` (key 273) · megaBytes · level 1, per_device_level 3 · Tier 3a: Host, VM
    pub const EFFECTIVEMEM_AVERAGE: Metric<Memory, VmOrHost> = Metric::new(
        "clusterServices",
        "effectivemem",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `clusterServices.failover.latest` (key 277) · number · level 1, per_device_level 3 · Tier 3a: Host, VM
    pub const FAILOVER_LATEST: Metric<Count, VmOrHost> = Metric::new(
        "clusterServices",
        "failover",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `clusterServices.memfairness.latest` (key 227) · number · level 1, per_device_level 3 · Tier 1: Host
    pub const MEMFAIRNESS_LATEST: Metric<Count, HostOnly> = Metric::new(
        "clusterServices",
        "memfairness",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `clusterServices.vmDrsScore.latest` (key 225) · percent (hundredths) · level 1, per_device_level 1 · Tier 1: VM
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

    /// `cpu.cache.l3.occupancy.average` (key 365) · kiloBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CACHE_L3_OCCUPANCY_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "cpu",
        "cache.l3.occupancy",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.capacity.contention.average` (key 19) · percent (hundredths) · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_CONTENTION_AVERAGE: Metric<Ratio, ComputeScope> = Metric::new(
        "cpu",
        "capacity.contention",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.capacity.demand.average` (key 18) · megaHertz · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_DEMAND_AVERAGE: Metric<Frequency, ComputeScope> = Metric::new(
        "cpu",
        "capacity.demand",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.capacity.entitlement.average` (key 16) · megaHertz · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_ENTITLEMENT_AVERAGE: Metric<Frequency, ComputeScope> = Metric::new(
        "cpu",
        "capacity.entitlement",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.capacity.provisioned.average` (key 15) · megaHertz · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_PROVISIONED_AVERAGE: Metric<Frequency, ComputeScope> = Metric::new(
        "cpu",
        "capacity.provisioned",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.capacity.usage.average` (key 17) · megaHertz · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_USAGE_AVERAGE: Metric<Frequency, ComputeScope> = Metric::new(
        "cpu",
        "capacity.usage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.corecount.contention.average` (key 22) · percent (hundredths) · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CORECOUNT_CONTENTION_AVERAGE: Metric<Ratio, ComputeScope> = Metric::new(
        "cpu",
        "corecount.contention",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.corecount.provisioned.average` (key 20) · number · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CORECOUNT_PROVISIONED_AVERAGE: Metric<Count, ComputeScope> = Metric::new(
        "cpu",
        "corecount.provisioned",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.corecount.provisioned.latest` (key 364) · number · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CORECOUNT_PROVISIONED_LATEST: Metric<Count, ComputeScope> = Metric::new(
        "cpu",
        "corecount.provisioned",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.corecount.usage.average` (key 21) · number · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CORECOUNT_USAGE_AVERAGE: Metric<Count, ComputeScope> = Metric::new(
        "cpu",
        "corecount.usage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.corecount.usage.latest` (key 366) · number · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CORECOUNT_USAGE_LATEST: Metric<Count, ComputeScope> = Metric::new(
        "cpu",
        "corecount.usage",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.coreUtilization.average` (key 441) · percent (hundredths) · level 2, per_device_level 3 · Tier 1: Host
    pub const CORE_UTILIZATION_AVERAGE: Metric<Ratio, HostOnly> = Metric::new(
        "cpu",
        "coreUtilization",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `cpu.coreUtilization.maximum` (key 442) · percent (hundredths) · level 4, per_device_level 4 · Tier 2: Host
    pub const CORE_UTILIZATION_MAXIMUM: Metric<Ratio, HostOnly> = Metric::new(
        "cpu",
        "coreUtilization",
        Rollup::Maximum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.coreUtilization.minimum` (key 443) · percent (hundredths) · level 4, per_device_level 4 · Tier 2: Host
    pub const CORE_UTILIZATION_MINIMUM: Metric<Ratio, HostOnly> = Metric::new(
        "cpu",
        "coreUtilization",
        Rollup::Minimum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.coreUtilization.none` (key 440) · percent (hundredths) · level 4, per_device_level 4 · Tier 2: Host
    pub const CORE_UTILIZATION_NONE: Metric<Ratio, HostOnly> = Metric::new(
        "cpu",
        "coreUtilization",
        Rollup::None,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.costop.summation` (key 448) · millisecond · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const COSTOP_SUMMATION: Metric<Time, VmOrHost> = Metric::new(
        "cpu",
        "costop",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `cpu.cpuentitlement.latest` (key 223) · megaHertz · level 2, per_device_level 3 · Tier 1: Pool, VM
    pub const CPUENTITLEMENT_LATEST: Metric<Frequency, VmOrPool> = Metric::new(
        "cpu",
        "cpuentitlement",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `cpu.demand.average` (key 447) · megaHertz · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const DEMAND_AVERAGE: Metric<Frequency, VmOrHost> = Metric::new(
        "cpu",
        "demand",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `cpu.demandEntitlementRatio.latest` (key 452) · percent (hundredths) · level 4, per_device_level 4 · Tier 1: VM
    pub const DEMAND_ENTITLEMENT_RATIO_LATEST: Metric<Ratio, VmOnly> = Metric::new(
        "cpu",
        "demandEntitlementRatio",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.entitlement.latest` (key 446) · megaHertz · level 2, per_device_level 3 · Tier 1: VM
    pub const ENTITLEMENT_LATEST: Metric<Frequency, VmOnly> = Metric::new(
        "cpu",
        "entitlement",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `cpu.idle.summation` (key 13) · millisecond · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const IDLE_SUMMATION: Metric<Time, VmOrHost> = Metric::new(
        "cpu",
        "idle",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `cpu.latency.average` (key 445) · percent (hundredths) · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const LATENCY_AVERAGE: Metric<Ratio, VmOrHost> = Metric::new(
        "cpu",
        "latency",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `cpu.load.avg15min.latest` (key 369) · percent (hundredths) · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const LOAD_AVG15MIN_LATEST: Metric<Ratio, ComputeScope> = Metric::new(
        "cpu",
        "load.avg15min",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.load.avg1min.latest` (key 367) · percent (hundredths) · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const LOAD_AVG1MIN_LATEST: Metric<Ratio, ComputeScope> = Metric::new(
        "cpu",
        "load.avg1min",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.load.avg5min.latest` (key 368) · percent (hundredths) · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const LOAD_AVG5MIN_LATEST: Metric<Ratio, ComputeScope> = Metric::new(
        "cpu",
        "load.avg5min",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.maxlimited.summation` (key 449) · millisecond · level 2, per_device_level 3 · Tier 1: VM
    pub const MAXLIMITED_SUMMATION: Metric<Time, VmOnly> = Metric::new(
        "cpu",
        "maxlimited",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `cpu.overlap.summation` (key 450) · millisecond · level 3, per_device_level 3 · Tier 1: VM
    pub const OVERLAP_SUMMATION: Metric<Time, VmOnly> = Metric::new(
        "cpu",
        "overlap",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `cpu.partnerBusyTime.average` (key 362) · percent (hundredths) · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const PARTNER_BUSY_TIME_AVERAGE: Metric<Ratio, ComputeScope> = Metric::new(
        "cpu",
        "partnerBusyTime",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.readiness.average` (key 453) · percent (hundredths) · level 4, per_device_level 4 · Tier 1: Host, VM
    pub const READINESS_AVERAGE: Metric<Ratio, VmOrHost> = Metric::new(
        "cpu",
        "readiness",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// `cpu.ready.summation` (key 12) · millisecond · level 1, per_device_level 3 · Tier 1: Host, VM
    pub const READY_SUMMATION: Metric<Time, VmOrHost> = Metric::new(
        "cpu",
        "ready",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `cpu.reservedCapacity.average` (key 9) · megaHertz · level 2, per_device_level 3 · Tier 1: Cluster, Host
    pub const RESERVED_CAPACITY_AVERAGE: Metric<Frequency, HostCluster> = Metric::new(
        "cpu",
        "reservedCapacity",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `cpu.run.summation` (key 451) · millisecond · level 2, per_device_level 3 · Tier 1: VM
    pub const RUN_SUMMATION: Metric<Time, VmOnly> = Metric::new(
        "cpu",
        "run",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `cpu.swapwait.summation` (key 436) · millisecond · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const SWAPWAIT_SUMMATION: Metric<Time, VmOrHost> = Metric::new(
        "cpu",
        "swapwait",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `cpu.system.summation` (key 10) · millisecond · level 3, per_device_level 3 · Tier 1: VM
    pub const SYSTEM_SUMMATION: Metric<Time, VmOnly> = Metric::new(
        "cpu",
        "system",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `cpu.totalmhz.average` (key 274) · megaHertz · level 1, per_device_level 3 · Tier 3a: Cluster, Host, Pool, VM
    pub const TOTALMHZ_AVERAGE: Metric<Frequency, ComputeScope> = Metric::new(
        "cpu",
        "totalmhz",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `cpu.totalCapacity.average` (key 444) · megaHertz · level 2, per_device_level 3 · Tier 1: Host
    pub const TOTAL_CAPACITY_AVERAGE: Metric<Frequency, HostOnly> = Metric::new(
        "cpu",
        "totalCapacity",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `cpu.usagemhz.average` (key 6) · megaHertz · level 1, per_device_level 3 · Tier 1: Cluster, Host, Pool, VM
    pub const USAGEMHZ_AVERAGE: Metric<Frequency, ComputeScope> = Metric::new(
        "cpu",
        "usagemhz",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `cpu.usagemhz.maximum` (key 8) · megaHertz · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const USAGEMHZ_MAXIMUM: Metric<Frequency, ComputeScope> = Metric::new(
        "cpu",
        "usagemhz",
        Rollup::Maximum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.usagemhz.minimum` (key 7) · megaHertz · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const USAGEMHZ_MINIMUM: Metric<Frequency, ComputeScope> = Metric::new(
        "cpu",
        "usagemhz",
        Rollup::Minimum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.usagemhz.none` (key 5) · megaHertz · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const USAGEMHZ_NONE: Metric<Frequency, ComputeScope> = Metric::new(
        "cpu",
        "usagemhz",
        Rollup::None,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.usage.average` (key 2) · percent (hundredths) · level 1, per_device_level 3 · Tier 1: Cluster, Host, VM
    pub const USAGE_AVERAGE: Metric<Ratio, VmHostCluster> = Metric::new(
        "cpu",
        "usage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `cpu.usage.maximum` (key 4) · percent (hundredths) · level 4, per_device_level 4 · Tier 2: Cluster, Host, VM
    pub const USAGE_MAXIMUM: Metric<Ratio, VmHostCluster> = Metric::new(
        "cpu",
        "usage",
        Rollup::Maximum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.usage.minimum` (key 3) · percent (hundredths) · level 4, per_device_level 4 · Tier 2: Cluster, Host, VM
    pub const USAGE_MINIMUM: Metric<Ratio, VmHostCluster> = Metric::new(
        "cpu",
        "usage",
        Rollup::Minimum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.usage.none` (key 1) · percent (hundredths) · level 4, per_device_level 4 · Tier 2: Cluster, Host, VM
    pub const USAGE_NONE: Metric<Ratio, VmHostCluster> = Metric::new(
        "cpu",
        "usage",
        Rollup::None,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.usage.vcpus.average` (key 454) · percent (hundredths) · level 4, per_device_level 4 · Tier 1: VM
    pub const USAGE_VCPUS_AVERAGE: Metric<Ratio, VmOnly> = Metric::new(
        "cpu",
        "usage.vcpus",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.used.summation` (key 14) · millisecond · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const USED_SUMMATION: Metric<Time, VmOrHost> = Metric::new(
        "cpu",
        "used",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `cpu.utilization.average` (key 363) · percent (hundredths) · level 2, per_device_level 3 · Tier 1: Host
    pub const UTILIZATION_AVERAGE: Metric<Ratio, HostOnly> = Metric::new(
        "cpu",
        "utilization",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `cpu.utilization.maximum` (key 438) · percent (hundredths) · level 4, per_device_level 4 · Tier 2: Host
    pub const UTILIZATION_MAXIMUM: Metric<Ratio, HostOnly> = Metric::new(
        "cpu",
        "utilization",
        Rollup::Maximum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.utilization.minimum` (key 439) · percent (hundredths) · level 4, per_device_level 4 · Tier 2: Host
    pub const UTILIZATION_MINIMUM: Metric<Ratio, HostOnly> = Metric::new(
        "cpu",
        "utilization",
        Rollup::Minimum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.utilization.none` (key 437) · percent (hundredths) · level 4, per_device_level 4 · Tier 2: Host
    pub const UTILIZATION_NONE: Metric<Ratio, HostOnly> = Metric::new(
        "cpu",
        "utilization",
        Rollup::None,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `cpu.wait.summation` (key 11) · millisecond · level 3, per_device_level 3 · Tier 1: Host, VM
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

    /// `datastore.busResets.summation` (key 196) · number · level 2, per_device_level 2 · Tier 3a: Datastore, Host, VM
    pub const BUS_RESETS_SUMMATION: Metric<Count, DatastoreVmHost> = Metric::new(
        "datastore",
        "busResets",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(2u8),
        false,
    );

    /// `datastore.commandsAborted.summation` (key 197) · number · level 2, per_device_level 2 · Tier 3a: Datastore, Host, VM
    pub const COMMANDS_ABORTED_SUMMATION: Metric<Count, DatastoreVmHost> = Metric::new(
        "datastore",
        "commandsAborted",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(2u8),
        false,
    );

    /// `datastore.datastoreIops.average` (key 192) · number · level 1, per_device_level 3 · Tier 1: Host
    pub const DATASTORE_IOPS_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreIops",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `datastore.datastoreMaxQueueDepth.latest` (key 584) · number · level 1, per_device_level 3 · Tier 1: Host
    pub const DATASTORE_MAX_QUEUE_DEPTH_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreMaxQueueDepth",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `datastore.datastoreNormalReadLatency.latest` (key 353) · number · level 2, per_device_level 2 · Tier 1: Host
    pub const DATASTORE_NORMAL_READ_LATENCY_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreNormalReadLatency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// `datastore.datastoreNormalWriteLatency.latest` (key 354) · number · level 2, per_device_level 2 · Tier 1: Host
    pub const DATASTORE_NORMAL_WRITE_LATENCY_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreNormalWriteLatency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// `datastore.datastoreReadBytes.latest` (key 347) · number · level 2, per_device_level 2 · Tier 1: Host
    pub const DATASTORE_READ_BYTES_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreReadBytes",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// `datastore.datastoreReadIops.latest` (key 349) · number · level 1, per_device_level 3 · Tier 1: Host
    pub const DATASTORE_READ_IOPS_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreReadIops",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `datastore.datastoreReadLoadMetric.latest` (key 355) · number · level 4, per_device_level 4 · Tier 1: Host
    pub const DATASTORE_READ_LOAD_METRIC_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreReadLoadMetric",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// `datastore.datastoreReadOIO.latest` (key 351) · number · level 1, per_device_level 3 · Tier 1: Host
    pub const DATASTORE_READ_OIO_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreReadOIO",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `datastore.datastoreVMObservedLatency.latest` (key 357) · microsecond · level 1, per_device_level 3 · Tier 1: Host
    pub const DATASTORE_VMOBSERVED_LATENCY_LATEST: Metric<Time, HostOnly> = Metric::new(
        "datastore",
        "datastoreVMObservedLatency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `datastore.datastoreWriteBytes.latest` (key 348) · number · level 2, per_device_level 2 · Tier 1: Host
    pub const DATASTORE_WRITE_BYTES_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreWriteBytes",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// `datastore.datastoreWriteIops.latest` (key 350) · number · level 1, per_device_level 3 · Tier 1: Host
    pub const DATASTORE_WRITE_IOPS_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreWriteIops",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `datastore.datastoreWriteLoadMetric.latest` (key 356) · number · level 4, per_device_level 4 · Tier 1: Host
    pub const DATASTORE_WRITE_LOAD_METRIC_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreWriteLoadMetric",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// `datastore.datastoreWriteOIO.latest` (key 352) · number · level 1, per_device_level 3 · Tier 1: Host
    pub const DATASTORE_WRITE_OIO_LATEST: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "datastoreWriteOIO",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `datastore.maxTotalLatency.latest` (key 191) · millisecond · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const MAX_TOTAL_LATENCY_LATEST: Metric<Time, VmOrHost> = Metric::new(
        "datastore",
        "maxTotalLatency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `datastore.numberReadAveraged.average` (key 185) · number · level 1, per_device_level 3 · Tier 1: Datastore, Host, VM
    pub const NUMBER_READ_AVERAGED_AVERAGE: Metric<Count, DatastoreVmHost> = Metric::new(
        "datastore",
        "numberReadAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `datastore.numberWriteAveraged.average` (key 186) · number · level 1, per_device_level 3 · Tier 1: Datastore, Host, VM
    pub const NUMBER_WRITE_AVERAGED_AVERAGE: Metric<Count, DatastoreVmHost> = Metric::new(
        "datastore",
        "numberWriteAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `datastore.read.average` (key 187) · kiloBytesPerSecond · level 2, per_device_level 2 · Tier 1: Datastore, Host, VM
    pub const READ_AVERAGE: Metric<Throughput, DatastoreVmHost> = Metric::new(
        "datastore",
        "read",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// `datastore.siocActiveTimePercentage.average` (key 198) · percent (hundredths) · level 1, per_device_level 3 · Tier 1: Host
    pub const SIOC_ACTIVE_TIME_PERCENTAGE_AVERAGE: Metric<Ratio, HostOnly> = Metric::new(
        "datastore",
        "siocActiveTimePercentage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `datastore.sizeNormalizedDatastoreLatency.average` (key 193) · microsecond · level 1, per_device_level 3 · Tier 1: Host
    pub const SIZE_NORMALIZED_DATASTORE_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "datastore",
        "sizeNormalizedDatastoreLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `datastore.throughput.contention.average` (key 195) · millisecond · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const THROUGHPUT_CONTENTION_AVERAGE: Metric<Time, DatastoreVmHost> = Metric::new(
        "datastore",
        "throughput.contention",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `datastore.throughput.usage.average` (key 194) · kiloBytesPerSecond · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const THROUGHPUT_USAGE_AVERAGE: Metric<Throughput, DatastoreVmHost> = Metric::new(
        "datastore",
        "throughput.usage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `datastore.totalReadLatency.average` (key 189) · millisecond · level 1, per_device_level 3 · Tier 1: Host, VM
    pub const TOTAL_READ_LATENCY_AVERAGE: Metric<Time, VmOrHost> = Metric::new(
        "datastore",
        "totalReadLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `datastore.totalWriteLatency.average` (key 190) · millisecond · level 1, per_device_level 3 · Tier 1: Host, VM
    pub const TOTAL_WRITE_LATENCY_AVERAGE: Metric<Time, VmOrHost> = Metric::new(
        "datastore",
        "totalWriteLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `datastore.unmapIOs.summation` (key 586) · number · level 4, per_device_level 4 · Tier 1: Host
    pub const UNMAP_IOS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "datastore",
        "unmapIOs",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// `datastore.unmapSize.summation` (key 585) · megaBytes · level 4, per_device_level 4 · Tier 1: Host
    pub const UNMAP_SIZE_SUMMATION: Metric<Memory, HostOnly> = Metric::new(
        "datastore",
        "unmapSize",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// `datastore.vmfs.apdCount.latest` (key 948) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const VMFS_APD_COUNT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "datastore",
        "vmfs.apdCount",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `datastore.vmfs.apdState.latest` (key 947) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const VMFS_APD_STATE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "datastore",
        "vmfs.apdState",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `datastore.vmfs.cancelledTxn.latest` (key 946) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const VMFS_CANCELLED_TXN_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "datastore",
        "vmfs.cancelledTxn",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `datastore.vmfs.totalTxn.latest` (key 945) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const VMFS_TOTAL_TXN_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "datastore",
        "vmfs.totalTxn",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `datastore.write.average` (key 188) · kiloBytesPerSecond · level 2, per_device_level 2 · Tier 1: Datastore, Host, VM
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

    /// `disk.actualused.latest` (key 282) · megaBytes · level 2, per_device_level 3 · Tier 3a: Datastore, Host, VM
    pub const ACTUALUSED_LATEST: Metric<Memory, DatastoreVmHost> = Metric::new(
        "disk",
        "actualused",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `disk.busResets.summation` (key 142) · number · level 2, per_device_level 3 · Tier 1: Datastore, Host, VM
    pub const BUS_RESETS_SUMMATION: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "busResets",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `disk.capacity.contention.average` (key 286) · percent (hundredths) · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const CAPACITY_CONTENTION_AVERAGE: Metric<Ratio, DatastoreVmHost> = Metric::new(
        "disk",
        "capacity.contention",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.capacity.latest` (key 280) · kiloBytes · level 1, per_device_level 3 · Tier 1: Datastore
    pub const CAPACITY_LATEST: Metric<Memory, DatastoreOnly> = Metric::new(
        "disk",
        "capacity",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `disk.capacity.provisioned.average` (key 284) · kiloBytes · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const CAPACITY_PROVISIONED_AVERAGE: Metric<Memory, DatastoreVmHost> = Metric::new(
        "disk",
        "capacity.provisioned",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.capacity.usage.average` (key 285) · kiloBytes · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const CAPACITY_USAGE_AVERAGE: Metric<Memory, DatastoreVmHost> = Metric::new(
        "disk",
        "capacity.usage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.commandsAborted.summation` (key 141) · number · level 2, per_device_level 3 · Tier 1: Datastore, Host, VM
    pub const COMMANDS_ABORTED_SUMMATION: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "commandsAborted",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `disk.commands.active.latest` (key 908) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const COMMANDS_ACTIVE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "commands.active",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.commandsAveraged.average` (key 509) · number · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const COMMANDS_AVERAGED_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "disk",
        "commandsAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `disk.commands.failed.latest` (key 906) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const COMMANDS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "commands.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.commands.queued.latest` (key 907) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const COMMANDS_QUEUED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "commands.queued",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.commands.success.latest` (key 905) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const COMMANDS_SUCCESS_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "commands.success",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.commands.summation` (key 496) · number · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const COMMANDS_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "disk",
        "commands",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `disk.deltaused.latest` (key 283) · kiloBytes · level 2, per_device_level 3 · Tier 3a: Datastore, Host, VM
    pub const DELTAUSED_LATEST: Metric<Memory, DatastoreVmHost> = Metric::new(
        "disk",
        "deltaused",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `disk.deviceLatency.average` (key 505) · millisecond · level 1, per_device_level 3 · Tier 1: Host
    pub const DEVICE_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "deviceLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `disk.deviceReadLatency.average` (key 497) · millisecond · level 2, per_device_level 3 · Tier 1: Host
    pub const DEVICE_READ_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "deviceReadLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `disk.deviceWriteLatency.average` (key 501) · millisecond · level 2, per_device_level 3 · Tier 1: Host
    pub const DEVICE_WRITE_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "deviceWriteLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `disk.kernelLatency.average` (key 506) · millisecond · level 2, per_device_level 3 · Tier 1: Host
    pub const KERNEL_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "kernelLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `disk.kernelReadLatency.average` (key 498) · millisecond · level 2, per_device_level 3 · Tier 1: Host
    pub const KERNEL_READ_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "kernelReadLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `disk.kernelWriteLatency.average` (key 502) · millisecond · level 2, per_device_level 3 · Tier 1: Host
    pub const KERNEL_WRITE_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "kernelWriteLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `disk.latency.davg.latest` (key 926) · microsecond · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const LATENCY_DAVG_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "disk",
        "latency.davg",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.latency.gavg.latest` (key 928) · microsecond · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const LATENCY_GAVG_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "disk",
        "latency.gavg",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.latency.kavg.latest` (key 927) · microsecond · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const LATENCY_KAVG_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "disk",
        "latency.kavg",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.latency.qavg.latest` (key 925) · microsecond · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const LATENCY_QAVG_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "disk",
        "latency.qavg",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.maxQueueDepth.average` (key 508) · number · level 1, per_device_level 3 · Tier 1: Host
    pub const MAX_QUEUE_DEPTH_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "disk",
        "maxQueueDepth",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `disk.maxTotalLatency.latest` (key 140) · millisecond · level 1, per_device_level 3 · Tier 1: Host, VM
    pub const MAX_TOTAL_LATENCY_LATEST: Metric<Time, VmOrHost> = Metric::new(
        "disk",
        "maxTotalLatency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `disk.numberReadAveraged.average` (key 143) · number · level 1, per_device_level 3 · Tier 1: Datastore, Host, VM
    pub const NUMBER_READ_AVERAGED_AVERAGE: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "numberReadAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `disk.numberRead.summation` (key 135) · number · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const NUMBER_READ_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "disk",
        "numberRead",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `disk.numberWriteAveraged.average` (key 144) · number · level 1, per_device_level 3 · Tier 1: Datastore, Host, VM
    pub const NUMBER_WRITE_AVERAGED_AVERAGE: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "numberWriteAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `disk.numberWrite.summation` (key 136) · number · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const NUMBER_WRITE_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "disk",
        "numberWrite",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `disk.provisioned.latest` (key 279) · kiloBytes · level 1, per_device_level 1 · Tier 1: Datastore, VM
    pub const PROVISIONED_LATEST: Metric<Memory, DatastoreVm> = Metric::new(
        "disk",
        "provisioned",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `disk.queueLatency.average` (key 507) · millisecond · level 2, per_device_level 3 · Tier 1: Host
    pub const QUEUE_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "queueLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `disk.queueReadLatency.average` (key 500) · millisecond · level 2, per_device_level 3 · Tier 1: Host
    pub const QUEUE_READ_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "queueReadLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `disk.queueWriteLatency.average` (key 504) · millisecond · level 2, per_device_level 3 · Tier 1: Host
    pub const QUEUE_WRITE_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "queueWriteLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `disk.read.average` (key 137) · kiloBytesPerSecond · level 2, per_device_level 3 · Tier 1: Datastore, Host, VM
    pub const READ_AVERAGE: Metric<Throughput, DatastoreVmHost> = Metric::new(
        "disk",
        "read",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `disk.readFailed.latest` (key 902) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const READ_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "readFailed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.read.latest` (key 901) · number · level 4, per_device_level 4 · Tier 2: Datastore, Host, VM
    pub const READ_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "read",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.scsiReservationCnflctsPct.average` (key 358) · percent (hundredths) · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const SCSI_RESERVATION_CNFLCTS_PCT_AVERAGE: Metric<Ratio, DatastoreVmHost> = Metric::new(
        "disk",
        "scsiReservationCnflctsPct",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.scsiReservationCnflctsPct.average` (key 148) · percent (hundredths) · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
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

    /// `disk.scsiReservationConflicts.summation` (key 147) · number · level 2, per_device_level 2 · Tier 1: Host, VM
    pub const SCSI_RESERVATION_CONFLICTS_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "disk",
        "scsiReservationConflicts",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// `disk.state.latest` (key 909) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const STATE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "state",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.throughput.contention.average` (key 146) · millisecond · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const THROUGHPUT_CONTENTION_AVERAGE: Metric<Time, DatastoreVmHost> = Metric::new(
        "disk",
        "throughput.contention",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.throughput.usage.average` (key 145) · kiloBytesPerSecond · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const THROUGHPUT_USAGE_AVERAGE: Metric<Throughput, DatastoreVmHost> = Metric::new(
        "disk",
        "throughput.usage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.TM.abortFailed.latest` (key 912) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const TM_ABORT_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.abortFailed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.TM.abort.latest` (key 910) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const TM_ABORT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.abort",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.TM.abortRetry.latest` (key 911) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const TM_ABORT_RETRY_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.abortRetry",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.TM.busResetFailed.latest` (key 924) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const TM_BUS_RESET_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.busResetFailed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.TM.busReset.latest` (key 922) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const TM_BUS_RESET_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.busReset",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.TM.busResetRetry.latest` (key 923) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const TM_BUS_RESET_RETRY_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.busResetRetry",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.TM.deviceResetFailed.latest` (key 921) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const TM_DEVICE_RESET_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.deviceResetFailed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.TM.deviceReset.latest` (key 919) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const TM_DEVICE_RESET_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.deviceReset",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.TM.deviceResetRetry.latest` (key 920) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const TM_DEVICE_RESET_RETRY_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.deviceResetRetry",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.TM.lunResetFailed.latest` (key 918) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const TM_LUN_RESET_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.lunResetFailed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.TM.lunReset.latest` (key 916) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const TM_LUN_RESET_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.lunReset",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.TM.lunResetRetry.latest` (key 917) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const TM_LUN_RESET_RETRY_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.lunResetRetry",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.TM.virtResetFailed.latest` (key 915) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const TM_VIRT_RESET_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.virtResetFailed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.TM.virtReset.latest` (key 913) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const TM_VIRT_RESET_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.virtReset",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.TM.virtResetRetry.latest` (key 914) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const TM_VIRT_RESET_RETRY_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "TM.virtResetRetry",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.totalLatency.average` (key 139) · millisecond · level 3, per_device_level 3 · Tier 1: Host
    pub const TOTAL_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "totalLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `disk.totalReadLatency.average` (key 499) · millisecond · level 2, per_device_level 3 · Tier 1: Host
    pub const TOTAL_READ_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "totalReadLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `disk.totalWriteLatency.average` (key 503) · millisecond · level 2, per_device_level 3 · Tier 1: Host
    pub const TOTAL_WRITE_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "disk",
        "totalWriteLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `disk.unshared.latest` (key 281) · kiloBytes · level 1, per_device_level 1 · Tier 1: VM
    pub const UNSHARED_LATEST: Metric<Memory, VmOnly> = Metric::new(
        "disk",
        "unshared",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `disk.usage.average` (key 132) · kiloBytesPerSecond · level 1, per_device_level 3 · Tier 1: Host, VM
    pub const USAGE_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "disk",
        "usage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `disk.usage.maximum` (key 134) · kiloBytesPerSecond · level 4, per_device_level 4 · Tier 2: Host, VM
    pub const USAGE_MAXIMUM: Metric<Throughput, VmOrHost> = Metric::new(
        "disk",
        "usage",
        Rollup::Maximum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.usage.minimum` (key 133) · kiloBytesPerSecond · level 4, per_device_level 4 · Tier 2: Host, VM
    pub const USAGE_MINIMUM: Metric<Throughput, VmOrHost> = Metric::new(
        "disk",
        "usage",
        Rollup::Minimum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.usage.none` (key 131) · kiloBytesPerSecond · level 4, per_device_level 4 · Tier 2: Host, VM
    pub const USAGE_NONE: Metric<Throughput, VmOrHost> = Metric::new(
        "disk",
        "usage",
        Rollup::None,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.used.latest` (key 278) · kiloBytes · level 1, per_device_level 1 · Tier 1: Datastore, VM
    pub const USED_LATEST: Metric<Memory, DatastoreVm> = Metric::new(
        "disk",
        "used",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        true,
    );

    /// `disk.write.average` (key 138) · kiloBytesPerSecond · level 2, per_device_level 3 · Tier 1: Datastore, Host, VM
    pub const WRITE_AVERAGE: Metric<Throughput, DatastoreVmHost> = Metric::new(
        "disk",
        "write",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `disk.writeFailed.latest` (key 904) · number · level 4, per_device_level 4 · Tier 3a: Datastore, Host, VM
    pub const WRITE_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "disk",
        "writeFailed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `disk.write.latest` (key 903) · number · level 4, per_device_level 4 · Tier 2: Datastore, Host, VM
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

    /// `gpu.mem.reserved.gb.latest` (key 1018) · gigaBytes · level 3, per_device_level 3 · Tier 3a: Cluster, Pool
    pub const MEM_RESERVED_GB_LATEST: Metric<Memory, ClusterPool> = Metric::new(
        "gpu",
        "mem.reserved.gb",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `gpu.mem.reserved.latest` (key 1014) · kiloBytes · level 1, per_device_level 3 · Tier 1: Cluster, Pool
    pub const MEM_RESERVED_LATEST: Metric<Memory, ClusterPool> = Metric::new(
        "gpu",
        "mem.reserved",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `gpu.mem.total.gb.latest` (key 1019) · gigaBytes · level 3, per_device_level 3 · Tier 3a: Cluster, Pool
    pub const MEM_TOTAL_GB_LATEST: Metric<Memory, ClusterPool> = Metric::new(
        "gpu",
        "mem.total.gb",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `gpu.mem.total.latest` (key 1016) · kiloBytes · level 1, per_device_level 3 · Tier 1: Cluster, Pool
    pub const MEM_TOTAL_LATEST: Metric<Memory, ClusterPool> = Metric::new(
        "gpu",
        "mem.total",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `gpu.mem.usage.average` (key 622) · percent (hundredths) · level 4, per_device_level 4 · Tier 3a: Cluster, Pool
    pub const MEM_USAGE_AVERAGE: Metric<Ratio, ClusterPool> = Metric::new(
        "gpu",
        "mem.usage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `gpu.mem.usage.maximum` (key 623) · percent (hundredths) · level 4, per_device_level 4 · Tier 3a: Cluster, Pool
    pub const MEM_USAGE_MAXIMUM: Metric<Ratio, ClusterPool> = Metric::new(
        "gpu",
        "mem.usage",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `gpu.mem.usage.minimum` (key 624) · percent (hundredths) · level 4, per_device_level 4 · Tier 3a: Cluster, Pool
    pub const MEM_USAGE_MINIMUM: Metric<Ratio, ClusterPool> = Metric::new(
        "gpu",
        "mem.usage",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `gpu.mem.usage.none` (key 621) · percent (hundredths) · level 4, per_device_level 4 · Tier 3a: Cluster, Pool
    pub const MEM_USAGE_NONE: Metric<Ratio, ClusterPool> = Metric::new(
        "gpu",
        "mem.usage",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `gpu.mem.used.average` (key 618) · kiloBytes · level 1, per_device_level 3 · Tier 1: Cluster, Pool
    pub const MEM_USED_AVERAGE: Metric<Memory, ClusterPool> = Metric::new(
        "gpu",
        "mem.used",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `gpu.mem.used.gb.latest` (key 1017) · gigaBytes · level 3, per_device_level 3 · Tier 3a: Cluster, Pool
    pub const MEM_USED_GB_LATEST: Metric<Memory, ClusterPool> = Metric::new(
        "gpu",
        "mem.used.gb",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `gpu.mem.used.maximum` (key 619) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Pool
    pub const MEM_USED_MAXIMUM: Metric<Memory, ClusterPool> = Metric::new(
        "gpu",
        "mem.used",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `gpu.mem.used.minimum` (key 620) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Pool
    pub const MEM_USED_MINIMUM: Metric<Memory, ClusterPool> = Metric::new(
        "gpu",
        "mem.used",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `gpu.mem.used.none` (key 617) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Pool
    pub const MEM_USED_NONE: Metric<Memory, ClusterPool> = Metric::new(
        "gpu",
        "mem.used",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `gpu.power.used.latest` (key 1015) · watt · level 1, per_device_level 3 · Tier 1: Cluster, Pool
    pub const POWER_USED_LATEST: Metric<Power, ClusterPool> = Metric::new(
        "gpu",
        "power.used",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `gpu.temperature.average` (key 625) · celsius · level 1, per_device_level 3 · Tier 1: Cluster, Pool
    pub const TEMPERATURE_AVERAGE: Metric<Temperature, ClusterPool> = Metric::new(
        "gpu",
        "temperature",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `gpu.utilization.average` (key 614) · percent (hundredths) · level 1, per_device_level 3 · Tier 1: Cluster, Pool
    pub const UTILIZATION_AVERAGE: Metric<Ratio, ClusterPool> = Metric::new(
        "gpu",
        "utilization",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `gpu.utilization.maximum` (key 615) · percent (hundredths) · level 4, per_device_level 4 · Tier 2: Cluster, Pool
    pub const UTILIZATION_MAXIMUM: Metric<Ratio, ClusterPool> = Metric::new(
        "gpu",
        "utilization",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `gpu.utilization.minimum` (key 616) · percent (hundredths) · level 4, per_device_level 4 · Tier 2: Cluster, Pool
    pub const UTILIZATION_MINIMUM: Metric<Ratio, ClusterPool> = Metric::new(
        "gpu",
        "utilization",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `gpu.utilization.none` (key 613) · percent (hundredths) · level 4, per_device_level 4 · Tier 2: Cluster, Pool
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

    /// `hbr.hbrDiskReadLatency.average` (key 591) · millisecond · level 4, per_device_level 4 · Tier 1: Host
    pub const HBR_DISK_READ_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "hbr",
        "hbrDiskReadLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `hbr.hbrDiskStallLatency.average` (key 592) · millisecond · level 4, per_device_level 4 · Tier 1: Host
    pub const HBR_DISK_STALL_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "hbr",
        "hbrDiskStallLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `hbr.hbrDiskTransferBytes.average` (key 595) · kiloBytes · level 4, per_device_level 4 · Tier 3a: Host
    pub const HBR_DISK_TRANSFER_BYTES_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "hbr",
        "hbrDiskTransferBytes",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `hbr.hbrDiskTransferIdle.average` (key 594) · millisecond · level 4, per_device_level 4 · Tier 3a: Host
    pub const HBR_DISK_TRANSFER_IDLE_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "hbr",
        "hbrDiskTransferIdle",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `hbr.hbrDiskTransferSuccess.average` (key 593) · millisecond · level 4, per_device_level 4 · Tier 3a: Host
    pub const HBR_DISK_TRANSFER_SUCCESS_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "hbr",
        "hbrDiskTransferSuccess",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `hbr.hbrNetLatency.average` (key 590) · millisecond · level 4, per_device_level 4 · Tier 3a: Host
    pub const HBR_NET_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "hbr",
        "hbrNetLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `hbr.hbrNetRx.average` (key 588) · kiloBytesPerSecond · level 4, per_device_level 4 · Tier 1: Host
    pub const HBR_NET_RX_AVERAGE: Metric<Throughput, HostOnly> = Metric::new(
        "hbr",
        "hbrNetRx",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `hbr.hbrNetTx.average` (key 589) · kiloBytesPerSecond · level 4, per_device_level 4 · Tier 1: Host
    pub const HBR_NET_TX_AVERAGE: Metric<Throughput, HostOnly> = Metric::new(
        "hbr",
        "hbrNetTx",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `hbr.hbrNumVms.average` (key 587) · number · level 4, per_device_level 4 · Tier 1: Host
    pub const HBR_NUM_VMS_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "hbr",
        "hbrNumVms",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `hbr.vms.latest` (key 402) · number · level 4, per_device_level 4 · Tier 3a: Host
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

    /// `lwd.numSnapshots.latest` (key 408) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host, VM
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

    /// `managementAgent.cpuUsage.average` (key 569) · megaHertz · level 3, per_device_level 3 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const CPU_USAGE_AVERAGE: Metric<Frequency, HostOnly> = Metric::new(
        "managementAgent",
        "cpuUsage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `managementAgent.memUsed.average` (key 567) · kiloBytes · level 3, per_device_level 3 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const MEM_USED_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "managementAgent",
        "memUsed",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `managementAgent.swapIn.average` (key 88) · kiloBytesPerSecond · level 3, per_device_level 3 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const SWAP_IN_AVERAGE: Metric<Throughput, HostOnly> = Metric::new(
        "managementAgent",
        "swapIn",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `managementAgent.swapOut.average` (key 87) · kiloBytesPerSecond · level 3, per_device_level 3 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const SWAP_OUT_AVERAGE: Metric<Throughput, HostOnly> = Metric::new(
        "managementAgent",
        "swapOut",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `managementAgent.swapUsed.average` (key 568) · kiloBytes · level 3, per_device_level 3 · tier 3b: inferred scope (no active samples in lab dump) · Host
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

    /// `mem.activewrite.average` (key 467) · kiloBytes · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const ACTIVEWRITE_AVERAGE: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "activewrite",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.active.average` (key 33) · kiloBytes · level 2, per_device_level 3 · Tier 1: Cluster, Host, Pool, VM
    pub const ACTIVE_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "active",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.active.maximum` (key 35) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const ACTIVE_MAXIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "active",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.active.minimum` (key 34) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const ACTIVE_MINIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "active",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.active.none` (key 32) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const ACTIVE_NONE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "active",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.bandwidth.read.latest` (key 124) · kiloBytesPerSecond · level 2, per_device_level 2 · Tier 3a: Cluster, Host, Pool, VM
    pub const BANDWIDTH_READ_LATEST: Metric<Throughput, ComputeScope> = Metric::new(
        "mem",
        "bandwidth.read",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        false,
    );

    /// `mem.bandwidth.total.latest` (key 126) · kiloBytesPerSecond · level 1, per_device_level 1 · Tier 3a: Cluster, Host, Pool, VM
    pub const BANDWIDTH_TOTAL_LATEST: Metric<Throughput, ComputeScope> = Metric::new(
        "mem",
        "bandwidth.total",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `mem.bandwidth.write.latest` (key 125) · kiloBytesPerSecond · level 2, per_device_level 2 · Tier 3a: Cluster, Host, Pool, VM
    pub const BANDWIDTH_WRITE_LATEST: Metric<Throughput, ComputeScope> = Metric::new(
        "mem",
        "bandwidth.write",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        false,
    );

    /// `mem.capacity.active.latest` (key 802) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_ACTIVE_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.active",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.capacity.consumed.latest` (key 801) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_CONSUMED_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.consumed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.capacity.contention.average` (key 112) · percent (hundredths) · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_CONTENTION_AVERAGE: Metric<Ratio, ComputeScope> = Metric::new(
        "mem",
        "capacity.contention",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.capacity.entitlement.average` (key 109) · kiloBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_ENTITLEMENT_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.entitlement",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.capacity.overhead.average` (key 394) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_OVERHEAD_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.overhead",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.capacity.overheadResv.average` (key 395) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_OVERHEAD_RESV_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.overheadResv",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.capacity.provisioned.average` (key 108) · kiloBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_PROVISIONED_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.provisioned",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.capacity.provisioned.latest` (key 370) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_PROVISIONED_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.provisioned",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.capacity.usable.average` (key 110) · kiloBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_USABLE_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.usable",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.capacity.usage.average` (key 111) · kiloBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_USAGE_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.usage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.capacity.usage.userworld.average` (key 116) · kiloBytes · level 2, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_USAGE_USERWORLD_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.usage.userworld",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(4u8),
        false,
    );

    /// `mem.capacity.usage.vmkOvrhd.average` (key 115) · kiloBytes · level 2, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_USAGE_VMK_OVRHD_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.usage.vmkOvrhd",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(4u8),
        false,
    );

    /// `mem.capacity.usage.vm.average` (key 113) · kiloBytes · level 2, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_USAGE_VM_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.usage.vm",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(4u8),
        false,
    );

    /// `mem.capacity.usage.vmOvrhd.average` (key 114) · kiloBytes · level 2, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_USAGE_VM_OVRHD_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "capacity.usage.vmOvrhd",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(4u8),
        false,
    );

    /// `mem.compressed.average` (key 105) · kiloBytes · level 2, per_device_level 3 · Tier 1: Cluster, Host, Pool, VM
    pub const COMPRESSED_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "compressed",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.compressionRate.average` (key 106) · kiloBytesPerSecond · level 2, per_device_level 3 · Tier 1: Cluster, Host, Pool, VM
    pub const COMPRESSION_RATE_AVERAGE: Metric<Throughput, ComputeScope> = Metric::new(
        "mem",
        "compressionRate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.consumed.average` (key 98) · kiloBytes · level 1, per_device_level 3 · Tier 1: Cluster, Host, Pool, VM
    pub const CONSUMED_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "consumed",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `mem.consumed.maximum` (key 100) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const CONSUMED_MAXIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "consumed",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.consumed.minimum` (key 99) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const CONSUMED_MINIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "consumed",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.consumed.none` (key 97) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const CONSUMED_NONE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "consumed",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.consumed.userworlds.average` (key 123) · kiloBytes · level 2, per_device_level 4 · Tier 1: Host
    pub const CONSUMED_USERWORLDS_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "consumed.userworlds",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(4u8),
        false,
    );

    /// `mem.consumed.vms.average` (key 122) · kiloBytes · level 2, per_device_level 4 · Tier 1: Host
    pub const CONSUMED_VMS_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "consumed.vms",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(4u8),
        true,
    );

    /// `mem.decompressionRate.average` (key 107) · kiloBytesPerSecond · level 2, per_device_level 3 · Tier 1: Cluster, Host, Pool, VM
    pub const DECOMPRESSION_RATE_AVERAGE: Metric<Throughput, ComputeScope> = Metric::new(
        "mem",
        "decompressionRate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.entitlement.average` (key 473) · kiloBytes · level 2, per_device_level 3 · Tier 1: VM
    pub const ENTITLEMENT_AVERAGE: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "entitlement",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.granted.average` (key 29) · kiloBytes · level 2, per_device_level 3 · Tier 1: Cluster, Host, Pool, VM
    pub const GRANTED_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "granted",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.granted.maximum` (key 31) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const GRANTED_MAXIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "granted",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.granted.minimum` (key 30) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const GRANTED_MINIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "granted",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.granted.none` (key 28) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const GRANTED_NONE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "granted",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.health.reservationState.latest` (key 393) · number · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const HEALTH_RESERVATION_STATE_LATEST: Metric<Count, ComputeScope> = Metric::new(
        "mem",
        "health.reservationState",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.heapfree.average` (key 65) · kiloBytes · level 4, per_device_level 4 · Tier 1: Host
    pub const HEAPFREE_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "heapfree",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.heapfree.maximum` (key 67) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const HEAPFREE_MAXIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "heapfree",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.heapfree.minimum` (key 66) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const HEAPFREE_MINIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "heapfree",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.heapfree.none` (key 64) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const HEAPFREE_NONE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "heapfree",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.heap.average` (key 61) · kiloBytes · level 4, per_device_level 4 · Tier 1: Host
    pub const HEAP_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "heap",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.heap.maximum` (key 63) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const HEAP_MAXIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "heap",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.heap.minimum` (key 62) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const HEAP_MINIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "heap",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.heap.none` (key 60) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const HEAP_NONE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "heap",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.kernel.managed.latest` (key 378) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const KERNEL_MANAGED_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "kernel.managed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.kernel.minfree.latest` (key 379) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const KERNEL_MINFREE_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "kernel.minfree",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.kernel.unreserved.latest` (key 380) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const KERNEL_UNRESERVED_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "kernel.unreserved",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.latency.average` (key 472) · percent (hundredths) · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const LATENCY_AVERAGE: Metric<Ratio, VmOrHost> = Metric::new(
        "mem",
        "latency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.latency.read.latest` (key 129) · nanosecond · level 3, per_device_level 3 · Tier 3a: Cluster, Host, Pool, VM
    pub const LATENCY_READ_LATEST: Metric<Time, ComputeScope> = Metric::new(
        "mem",
        "latency.read",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `mem.latency.write.latest` (key 130) · nanosecond · level 3, per_device_level 3 · Tier 3a: Cluster, Host, Pool, VM
    pub const LATENCY_WRITE_LATEST: Metric<Time, ComputeScope> = Metric::new(
        "mem",
        "latency.write",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `mem.llSwapIn.average` (key 483) · kiloBytes · level 4, per_device_level 4 · Tier 1: Host
    pub const LL_SWAP_IN_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "llSwapIn",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.llSwapIn.maximum` (key 484) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const LL_SWAP_IN_MAXIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "llSwapIn",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.llSwapIn.minimum` (key 485) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const LL_SWAP_IN_MINIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "llSwapIn",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.llSwapIn.none` (key 482) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const LL_SWAP_IN_NONE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "llSwapIn",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.llSwapInRate.average` (key 476) · kiloBytesPerSecond · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const LL_SWAP_IN_RATE_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "mem",
        "llSwapInRate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.llSwapOut.average` (key 487) · kiloBytes · level 4, per_device_level 4 · Tier 1: Host
    pub const LL_SWAP_OUT_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "llSwapOut",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.llSwapOut.maximum` (key 488) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const LL_SWAP_OUT_MAXIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "llSwapOut",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.llSwapOut.minimum` (key 489) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const LL_SWAP_OUT_MINIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "llSwapOut",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.llSwapOut.none` (key 486) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const LL_SWAP_OUT_NONE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "llSwapOut",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.llSwapOutRate.average` (key 477) · kiloBytesPerSecond · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const LL_SWAP_OUT_RATE_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "mem",
        "llSwapOutRate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.llSwapUsed.average` (key 479) · kiloBytes · level 4, per_device_level 4 · Tier 1: Host, VM
    pub const LL_SWAP_USED_AVERAGE: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "llSwapUsed",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.llSwapUsed.maximum` (key 480) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host, VM
    pub const LL_SWAP_USED_MAXIMUM: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "llSwapUsed",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.llSwapUsed.minimum` (key 481) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host, VM
    pub const LL_SWAP_USED_MINIMUM: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "llSwapUsed",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.llSwapUsed.none` (key 475) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host, VM
    pub const LL_SWAP_USED_NONE: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "llSwapUsed",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.lowfreethreshold.average` (key 474) · kiloBytes · level 2, per_device_level 3 · Tier 1: Host
    pub const LOWFREETHRESHOLD_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "lowfreethreshold",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.memctl.current.latest` (key 390) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const MEMCTL_CURRENT_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "memctl.current",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.memctl.max.latest` (key 392) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const MEMCTL_MAX_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "memctl.max",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.memctl.target.latest` (key 391) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const MEMCTL_TARGET_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "memctl.target",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.mementitlement.latest` (key 224) · megaBytes · level 2, per_device_level 3 · Tier 1: Pool, VM
    pub const MEMENTITLEMENT_LATEST: Metric<Memory, VmOrPool> = Metric::new(
        "mem",
        "mementitlement",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.missrate.latest` (key 128) · percent (hundredths) · level 2, per_device_level 2 · Tier 3a: Cluster, Host, Pool, VM
    pub const MISSRATE_LATEST: Metric<Ratio, ComputeScope> = Metric::new(
        "mem",
        "missrate",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        false,
    );

    /// `mem.overcommit.avg15min.latest` (key 374) · number · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const OVERCOMMIT_AVG15MIN_LATEST: Metric<Count, ComputeScope> = Metric::new(
        "mem",
        "overcommit.avg15min",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.overcommit.avg1min.latest` (key 372) · number · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const OVERCOMMIT_AVG1MIN_LATEST: Metric<Count, ComputeScope> = Metric::new(
        "mem",
        "overcommit.avg1min",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.overcommit.avg5min.latest` (key 373) · number · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const OVERCOMMIT_AVG5MIN_LATEST: Metric<Count, ComputeScope> = Metric::new(
        "mem",
        "overcommit.avg5min",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.overhead.average` (key 102) · kiloBytes · level 1, per_device_level 1 · Tier 1: Cluster, Host, Pool, VM
    pub const OVERHEAD_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "overhead",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `mem.overhead.maximum` (key 104) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const OVERHEAD_MAXIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "overhead",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.overheadMax.average` (key 468) · kiloBytes · level 2, per_device_level 3 · Tier 1: VM
    pub const OVERHEAD_MAX_AVERAGE: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "overheadMax",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.overhead.minimum` (key 103) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const OVERHEAD_MINIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "overhead",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.overhead.none` (key 101) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const OVERHEAD_NONE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "overhead",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.overheadTouched.average` (key 478) · kiloBytes · level 4, per_device_level 4 · Tier 1: VM
    pub const OVERHEAD_TOUCHED_AVERAGE: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "overheadTouched",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.physical.free.latest` (key 377) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const PHYSICAL_FREE_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "physical.free",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.physical.total.latest` (key 375) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const PHYSICAL_TOTAL_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "physical.total",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.physical.user.latest` (key 376) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const PHYSICAL_USER_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "physical.user",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.pshare.common.latest` (key 382) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const PSHARE_COMMON_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "pshare.common",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.pshare.shared.latest` (key 381) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const PSHARE_SHARED_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "pshare.shared",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.pshare.sharedSave.latest` (key 383) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const PSHARE_SHARED_SAVE_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "pshare.sharedSave",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.reservedCapacity.average` (key 27) · megaBytes · level 2, per_device_level 3 · Tier 1: Cluster, Host
    pub const RESERVED_CAPACITY_AVERAGE: Metric<Memory, HostCluster> = Metric::new(
        "mem",
        "reservedCapacity",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.reservedCapacityPct.average` (key 121) · percent (hundredths) · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const RESERVED_CAPACITY_PCT_AVERAGE: Metric<Ratio, ComputeScope> = Metric::new(
        "mem",
        "reservedCapacityPct",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.reservedCapacityPct.latest` (key 371) · percent (hundredths) · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const RESERVED_CAPACITY_PCT_LATEST: Metric<Ratio, ComputeScope> = Metric::new(
        "mem",
        "reservedCapacityPct",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.reservedCapacity.userworld.average` (key 120) · kiloBytes · level 2, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const RESERVED_CAPACITY_USERWORLD_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "reservedCapacity.userworld",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(4u8),
        false,
    );

    /// `mem.reservedCapacity.vmkOvrhd.average` (key 119) · kiloBytes · level 2, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const RESERVED_CAPACITY_VMK_OVRHD_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "reservedCapacity.vmkOvrhd",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(4u8),
        false,
    );

    /// `mem.reservedCapacity.vm.average` (key 117) · kiloBytes · level 2, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const RESERVED_CAPACITY_VM_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "reservedCapacity.vm",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(4u8),
        false,
    );

    /// `mem.reservedCapacity.vmOvhd.average` (key 118) · kiloBytes · level 2, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const RESERVED_CAPACITY_VM_OVHD_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "reservedCapacity.vmOvhd",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(4u8),
        false,
    );

    /// `mem.sharedcommon.average` (key 57) · kiloBytes · level 2, per_device_level 3 · Tier 1: Host
    pub const SHAREDCOMMON_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "sharedcommon",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.sharedcommon.maximum` (key 59) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const SHAREDCOMMON_MAXIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "sharedcommon",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.sharedcommon.minimum` (key 58) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const SHAREDCOMMON_MINIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "sharedcommon",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.sharedcommon.none` (key 56) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const SHAREDCOMMON_NONE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "sharedcommon",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.shared.average` (key 37) · kiloBytes · level 2, per_device_level 3 · Tier 1: Cluster, Host, Pool, VM
    pub const SHARED_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "shared",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.shared.maximum` (key 39) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const SHARED_MAXIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "shared",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.shared.minimum` (key 38) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const SHARED_MINIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "shared",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.shared.none` (key 36) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const SHARED_NONE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "shared",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.state.latest` (key 68) · number · level 2, per_device_level 3 · Tier 1: Host
    pub const STATE_LATEST: Metric<Count, HostOnly> = Metric::new(
        "mem",
        "state",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.swapin.average` (key 456) · kiloBytes · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const SWAPIN_AVERAGE: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "swapin",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.swapin.maximum` (key 457) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host, VM
    pub const SWAPIN_MAXIMUM: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "swapin",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapin.minimum` (key 458) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host, VM
    pub const SWAPIN_MINIMUM: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "swapin",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapin.none` (key 455) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host, VM
    pub const SWAPIN_NONE: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "swapin",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapinRate.average` (key 85) · kiloBytesPerSecond · level 1, per_device_level 3 · Tier 1: Host, VM
    pub const SWAPIN_RATE_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "mem",
        "swapinRate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `mem.swapout.average` (key 460) · kiloBytes · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const SWAPOUT_AVERAGE: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "swapout",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.swapout.maximum` (key 461) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host, VM
    pub const SWAPOUT_MAXIMUM: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "swapout",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapout.minimum` (key 462) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host, VM
    pub const SWAPOUT_MINIMUM: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "swapout",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapout.none` (key 459) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host, VM
    pub const SWAPOUT_NONE: Metric<Memory, VmOrHost> = Metric::new(
        "mem",
        "swapout",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapoutRate.average` (key 86) · kiloBytesPerSecond · level 1, per_device_level 3 · Tier 1: Host, VM
    pub const SWAPOUT_RATE_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "mem",
        "swapoutRate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `mem.swapped.average` (key 70) · kiloBytes · level 2, per_device_level 3 · Tier 1: Pool, VM
    pub const SWAPPED_AVERAGE: Metric<Memory, VmOrPool> = Metric::new(
        "mem",
        "swapped",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.swapped.maximum` (key 72) · kiloBytes · level 4, per_device_level 4 · Tier 2: Pool, VM
    pub const SWAPPED_MAXIMUM: Metric<Memory, VmOrPool> = Metric::new(
        "mem",
        "swapped",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapped.minimum` (key 71) · kiloBytes · level 4, per_device_level 4 · Tier 2: Pool, VM
    pub const SWAPPED_MINIMUM: Metric<Memory, VmOrPool> = Metric::new(
        "mem",
        "swapped",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapped.none` (key 69) · kiloBytes · level 4, per_device_level 4 · Tier 2: Pool, VM
    pub const SWAPPED_NONE: Metric<Memory, VmOrPool> = Metric::new(
        "mem",
        "swapped",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swaptarget.average` (key 74) · kiloBytes · level 2, per_device_level 3 · Tier 1: VM
    pub const SWAPTARGET_AVERAGE: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "swaptarget",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.swaptarget.maximum` (key 76) · kiloBytes · level 4, per_device_level 4 · Tier 2: VM
    pub const SWAPTARGET_MAXIMUM: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "swaptarget",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swaptarget.minimum` (key 75) · kiloBytes · level 4, per_device_level 4 · Tier 2: VM
    pub const SWAPTARGET_MINIMUM: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "swaptarget",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swaptarget.none` (key 73) · kiloBytes · level 4, per_device_level 4 · Tier 2: VM
    pub const SWAPTARGET_NONE: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "swaptarget",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapunreserved.average` (key 53) · kiloBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const SWAPUNRESERVED_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapunreserved",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapunreserved.maximum` (key 55) · kiloBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const SWAPUNRESERVED_MAXIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapunreserved",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapunreserved.minimum` (key 54) · kiloBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const SWAPUNRESERVED_MINIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapunreserved",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapunreserved.none` (key 52) · kiloBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const SWAPUNRESERVED_NONE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapunreserved",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapused.average` (key 49) · kiloBytes · level 2, per_device_level 3 · Tier 1: Cluster, Host
    pub const SWAPUSED_AVERAGE: Metric<Memory, HostCluster> = Metric::new(
        "mem",
        "swapused",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.swapused.maximum` (key 51) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host
    pub const SWAPUSED_MAXIMUM: Metric<Memory, HostCluster> = Metric::new(
        "mem",
        "swapused",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapused.minimum` (key 50) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host
    pub const SWAPUSED_MINIMUM: Metric<Memory, HostCluster> = Metric::new(
        "mem",
        "swapused",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapused.none` (key 48) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host
    pub const SWAPUSED_NONE: Metric<Memory, HostCluster> = Metric::new(
        "mem",
        "swapused",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swap.current.latest` (key 384) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const SWAP_CURRENT_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swap.current",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapIn.average` (key 78) · kiloBytes · level 2, per_device_level 3 · Tier 3a: Cluster, Host, Pool, VM
    pub const SWAP_IN_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapIn",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.swapIn.maximum` (key 80) · kiloBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const SWAP_IN_MAXIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapIn",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapIn.minimum` (key 79) · kiloBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const SWAP_IN_MINIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapIn",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapIn.none` (key 77) · kiloBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const SWAP_IN_NONE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapIn",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapOut.average` (key 82) · kiloBytes · level 2, per_device_level 3 · Tier 3a: Cluster, Host, Pool, VM
    pub const SWAP_OUT_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapOut",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.swapOut.maximum` (key 84) · kiloBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const SWAP_OUT_MAXIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapOut",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapOut.minimum` (key 83) · kiloBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const SWAP_OUT_MINIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapOut",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swapOut.none` (key 81) · kiloBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const SWAP_OUT_NONE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swapOut",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swap.readrate.average` (key 386) · megaBytesPerSecond · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const SWAP_READRATE_AVERAGE: Metric<Throughput, ComputeScope> = Metric::new(
        "mem",
        "swap.readrate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swap.target.latest` (key 385) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const SWAP_TARGET_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "swap.target",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.swap.writerate.average` (key 387) · megaBytesPerSecond · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const SWAP_WRITERATE_AVERAGE: Metric<Throughput, ComputeScope> = Metric::new(
        "mem",
        "swap.writerate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.sysUsage.average` (key 464) · kiloBytes · level 2, per_device_level 3 · Tier 1: Host
    pub const SYS_USAGE_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "sysUsage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.sysUsage.maximum` (key 465) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const SYS_USAGE_MAXIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "sysUsage",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.sysUsage.minimum` (key 466) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const SYS_USAGE_MINIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "sysUsage",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.sysUsage.none` (key 463) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const SYS_USAGE_NONE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "sysUsage",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.totalmb.average` (key 275) · megaBytes · level 1, per_device_level 3 · Tier 3a: Cluster, Host, Pool, VM
    pub const TOTALMB_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "totalmb",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `mem.totalCapacity.average` (key 469) · megaBytes · level 2, per_device_level 3 · Tier 1: Host
    pub const TOTAL_CAPACITY_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "totalCapacity",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.unreserved.average` (key 45) · kiloBytes · level 2, per_device_level 3 · Tier 1: Host
    pub const UNRESERVED_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "unreserved",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.unreserved.maximum` (key 47) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const UNRESERVED_MAXIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "unreserved",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.unreserved.minimum` (key 46) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const UNRESERVED_MINIMUM: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "unreserved",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.unreserved.none` (key 44) · kiloBytes · level 4, per_device_level 4 · Tier 2: Host
    pub const UNRESERVED_NONE: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "unreserved",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.usage.average` (key 24) · percent (hundredths) · level 1, per_device_level 3 · Tier 1: Cluster, Host, VM
    pub const USAGE_AVERAGE: Metric<Ratio, VmHostCluster> = Metric::new(
        "mem",
        "usage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `mem.usage.maximum` (key 26) · percent (hundredths) · level 4, per_device_level 4 · Tier 2: Cluster, Host, VM
    pub const USAGE_MAXIMUM: Metric<Ratio, VmHostCluster> = Metric::new(
        "mem",
        "usage",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.usage.minimum` (key 25) · percent (hundredths) · level 4, per_device_level 4 · Tier 2: Cluster, Host, VM
    pub const USAGE_MINIMUM: Metric<Ratio, VmHostCluster> = Metric::new(
        "mem",
        "usage",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.usage.none` (key 23) · percent (hundredths) · level 4, per_device_level 4 · Tier 2: Cluster, Host, VM
    pub const USAGE_NONE: Metric<Ratio, VmHostCluster> = Metric::new(
        "mem",
        "usage",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.vmfs.pbc.capMissRatio.latest` (key 495) · percent (hundredths) · level 4, per_device_level 4 · Tier 1: Host
    pub const VMFS_PBC_CAP_MISS_RATIO_LATEST: Metric<Ratio, HostOnly> = Metric::new(
        "mem",
        "vmfs.pbc.capMissRatio",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.vmfs.pbc.overhead.latest` (key 494) · kiloBytes · level 4, per_device_level 4 · Tier 1: Host
    pub const VMFS_PBC_OVERHEAD_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "vmfs.pbc.overhead",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.vmfs.pbc.size.latest` (key 490) · megaBytes · level 4, per_device_level 4 · Tier 1: Host
    pub const VMFS_PBC_SIZE_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "vmfs.pbc.size",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.vmfs.pbc.sizeMax.latest` (key 491) · megaBytes · level 4, per_device_level 4 · Tier 1: Host
    pub const VMFS_PBC_SIZE_MAX_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "vmfs.pbc.sizeMax",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.vmfs.pbc.workingSet.latest` (key 492) · teraBytes · level 4, per_device_level 4 · Tier 1: Host
    pub const VMFS_PBC_WORKING_SET_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "vmfs.pbc.workingSet",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.vmfs.pbc.workingSetMax.latest` (key 493) · teraBytes · level 4, per_device_level 4 · Tier 1: Host
    pub const VMFS_PBC_WORKING_SET_MAX_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "mem",
        "vmfs.pbc.workingSetMax",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.vmmemctltarget.average` (key 94) · kiloBytes · level 2, per_device_level 3 · Tier 1: VM
    pub const VMMEMCTLTARGET_AVERAGE: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "vmmemctltarget",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.vmmemctltarget.maximum` (key 96) · kiloBytes · level 4, per_device_level 4 · Tier 2: VM
    pub const VMMEMCTLTARGET_MAXIMUM: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "vmmemctltarget",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.vmmemctltarget.minimum` (key 95) · kiloBytes · level 4, per_device_level 4 · Tier 2: VM
    pub const VMMEMCTLTARGET_MINIMUM: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "vmmemctltarget",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.vmmemctltarget.none` (key 93) · kiloBytes · level 4, per_device_level 4 · Tier 2: VM
    pub const VMMEMCTLTARGET_NONE: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "vmmemctltarget",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.vmmemctl.average` (key 90) · kiloBytes · level 1, per_device_level 3 · Tier 1: Cluster, Host, Pool, VM
    pub const VMMEMCTL_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "vmmemctl",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `mem.vmmemctl.maximum` (key 92) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const VMMEMCTL_MAXIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "vmmemctl",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.vmmemctl.minimum` (key 91) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const VMMEMCTL_MINIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "vmmemctl",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.vmmemctl.none` (key 89) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const VMMEMCTL_NONE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "vmmemctl",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.vm.bandwidth.read.latest` (key 127) · kiloBytesPerSecond · level 2, per_device_level 2 · Tier 3a: Cluster, Host, Pool, VM
    pub const VM_BANDWIDTH_READ_LATEST: Metric<Throughput, ComputeScope> = Metric::new(
        "mem",
        "vm.bandwidth.read",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        false,
    );

    /// `mem.zero.average` (key 41) · kiloBytes · level 2, per_device_level 3 · Tier 1: Cluster, Host, Pool, VM
    pub const ZERO_AVERAGE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "zero",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.zero.maximum` (key 43) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const ZERO_MAXIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "zero",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.zero.minimum` (key 42) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const ZERO_MINIMUM: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "zero",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.zero.none` (key 40) · kiloBytes · level 4, per_device_level 4 · Tier 2: Cluster, Host, Pool, VM
    pub const ZERO_NONE: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "zero",
        Rollup::None,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.zipped.latest` (key 470) · kiloBytes · level 2, per_device_level 3 · Tier 1: VM
    pub const ZIPPED_LATEST: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "zipped",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.zipSaved.latest` (key 471) · kiloBytes · level 2, per_device_level 3 · Tier 1: VM
    pub const ZIPSAVED_LATEST: Metric<Memory, VmOnly> = Metric::new(
        "mem",
        "zipSaved",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `mem.zip.saved.latest` (key 389) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const ZIP_SAVED_LATEST: Metric<Memory, ComputeScope> = Metric::new(
        "mem",
        "zip.saved",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `mem.zip.zipped.latest` (key 388) · megaBytes · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
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

    /// `net.broadcastRx.summation` (key 514) · number · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const BROADCAST_RX_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "broadcastRx",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `net.broadcastTx.summation` (key 515) · number · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const BROADCAST_TX_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "broadcastTx",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `net.bytesRx.average` (key 512) · kiloBytesPerSecond · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const BYTES_RX_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "bytesRx",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `net.bytesTx.average` (key 513) · kiloBytesPerSecond · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const BYTES_TX_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "bytesTx",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `net.droppedRx.summation` (key 510) · number · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const DROPPED_RX_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "droppedRx",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `net.droppedTx.summation` (key 511) · number · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const DROPPED_TX_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "droppedTx",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `net.errorsRx.summation` (key 518) · number · level 2, per_device_level 3 · Tier 1: Host
    pub const ERRORS_RX_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "net",
        "errorsRx",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `net.errorsTx.summation` (key 519) · number · level 2, per_device_level 3 · Tier 1: Host
    pub const ERRORS_TX_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "net",
        "errorsTx",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `net.latency.hbr.outbound.latest` (key 407) · millisecond · level 4, per_device_level 4 · Tier 3a: Host, VM
    pub const LATENCY_HBR_OUTBOUND_LATEST: Metric<Time, VmOrHost> = Metric::new(
        "net",
        "latency.hbr.outbound",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `net.multicastRx.summation` (key 516) · number · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const MULTICAST_RX_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "multicastRx",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `net.multicastTx.summation` (key 517) · number · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const MULTICAST_TX_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "multicastTx",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `net.packetsRx.summation` (key 153) · number · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const PACKETS_RX_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "packetsRx",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `net.packetsTx.summation` (key 154) · number · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const PACKETS_TX_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "packetsTx",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `net.pnicBytesRx.average` (key 521) · kiloBytesPerSecond · level 4, per_device_level 4 · Tier 1: VM
    pub const PNIC_BYTES_RX_AVERAGE: Metric<Throughput, VmOnly> = Metric::new(
        "net",
        "pnicBytesRx",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// `net.pnicBytesTx.average` (key 522) · kiloBytesPerSecond · level 4, per_device_level 4 · Tier 1: VM
    pub const PNIC_BYTES_TX_AVERAGE: Metric<Throughput, VmOnly> = Metric::new(
        "net",
        "pnicBytesTx",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// `net.received.average` (key 155) · kiloBytesPerSecond · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const RECEIVED_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "received",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `net.throughput.contention.summation` (key 160) · number · level 4, per_device_level 4 · Tier 3a: Host, VM
    pub const THROUGHPUT_CONTENTION_SUMMATION: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.contention",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `net.throughput.droppedRx.average` (key 235) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_DROPPED_RX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.droppedRx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.droppedTx.average` (key 234) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_DROPPED_TX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.droppedTx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.hbr.inbound.average` (key 403) · kiloBytesPerSecond · level 4, per_device_level 4 · Tier 3a: Host, VM
    pub const THROUGHPUT_HBR_INBOUND_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.hbr.inbound",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `net.throughput.hbr.outbound.average` (key 404) · kiloBytesPerSecond · level 4, per_device_level 4 · Tier 3a: Host, VM
    pub const THROUGHPUT_HBR_OUTBOUND_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.hbr.outbound",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `net.throughput.packetsPerSec.average` (key 161) · number · level 4, per_device_level 4 · Tier 3a: Host, VM
    pub const THROUGHPUT_PACKETS_PER_SEC_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.packetsPerSec",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `net.throughput.pktsRx.average` (key 231) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_PKTS_RX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.pktsRx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.pktsRxBroadcast.average` (key 233) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_PKTS_RX_BROADCAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.pktsRxBroadcast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.pktsRxMulticast.average` (key 232) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_PKTS_RX_MULTICAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.pktsRxMulticast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.pktsTx.average` (key 228) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_PKTS_TX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.pktsTx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.pktsTxBroadcast.average` (key 230) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_PKTS_TX_BROADCAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.pktsTxBroadcast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.pktsTxMulticast.average` (key 229) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_PKTS_TX_MULTICAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.pktsTxMulticast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.provisioned.average` (key 157) · kiloBytesPerSecond · level 4, per_device_level 4 · Tier 3a: Host, VM
    pub const THROUGHPUT_PROVISIONED_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.provisioned",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `net.throughput.usable.average` (key 158) · kiloBytesPerSecond · level 4, per_device_level 4 · Tier 3a: Host, VM
    pub const THROUGHPUT_USABLE_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.usable",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `net.throughput.usage.average` (key 159) · kiloBytesPerSecond · level 4, per_device_level 4 · Tier 3a: Host, VM
    pub const THROUGHPUT_USAGE_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.usage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `net.throughput.usage.ft.average` (key 215) · kiloBytesPerSecond · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_USAGE_FT_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.usage.ft",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.usage.hbr.average` (key 217) · kiloBytesPerSecond · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_USAGE_HBR_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.usage.hbr",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.usage.iscsi.average` (key 216) · kiloBytesPerSecond · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_USAGE_ISCSI_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.usage.iscsi",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.usage.nfs.average` (key 213) · kiloBytesPerSecond · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_USAGE_NFS_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.usage.nfs",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.usage.vmotion.average` (key 214) · kiloBytesPerSecond · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_USAGE_VMOTION_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.usage.vmotion",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.usage.vm.average` (key 212) · kiloBytesPerSecond · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_USAGE_VM_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "throughput.usage.vm",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.vds.droppedRx.average` (key 243) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_VDS_DROPPED_RX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.droppedRx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.vds.droppedTx.average` (key 242) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_VDS_DROPPED_TX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.droppedTx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.vds.lagDropRx.average` (key 251) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_VDS_LAG_DROP_RX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.lagDropRx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.vds.lagDropTx.average` (key 250) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_VDS_LAG_DROP_TX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.lagDropTx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.vds.lagRx.average` (key 247) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_VDS_LAG_RX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.lagRx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.vds.lagRxBcast.average` (key 249) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_VDS_LAG_RX_BCAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.lagRxBcast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.vds.lagRxMcast.average` (key 248) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_VDS_LAG_RX_MCAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.lagRxMcast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.vds.lagTx.average` (key 244) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_VDS_LAG_TX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.lagTx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.vds.lagTxBcast.average` (key 246) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_VDS_LAG_TX_BCAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.lagTxBcast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.vds.lagTxMcast.average` (key 245) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_VDS_LAG_TX_MCAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.lagTxMcast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.vds.pktsRx.average` (key 239) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_VDS_PKTS_RX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.pktsRx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.vds.pktsRxBcast.average` (key 241) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_VDS_PKTS_RX_BCAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.pktsRxBcast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.vds.pktsRxMcast.average` (key 240) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_VDS_PKTS_RX_MCAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.pktsRxMcast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.vds.pktsTx.average` (key 236) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_VDS_PKTS_TX_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.pktsTx",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.vds.pktsTxBcast.average` (key 238) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_VDS_PKTS_TX_BCAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.pktsTxBcast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.throughput.vds.pktsTxMcast.average` (key 237) · number · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const THROUGHPUT_VDS_PKTS_TX_MCAST_AVERAGE: Metric<Count, VmOrHost> = Metric::new(
        "net",
        "throughput.vds.pktsTxMcast",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `net.transmitted.average` (key 156) · kiloBytesPerSecond · level 2, per_device_level 3 · Tier 1: Host, VM
    pub const TRANSMITTED_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "transmitted",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `net.unknownProtos.summation` (key 520) · number · level 2, per_device_level 3 · Tier 1: Host
    pub const UNKNOWN_PROTOS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "net",
        "unknownProtos",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `net.usage.average` (key 150) · kiloBytesPerSecond · level 1, per_device_level 3 · Tier 1: Host, VM
    pub const USAGE_AVERAGE: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "usage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `net.usage.maximum` (key 152) · kiloBytesPerSecond · level 4, per_device_level 4 · Tier 2: Host, VM
    pub const USAGE_MAXIMUM: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "usage",
        Rollup::Maximum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `net.usage.minimum` (key 151) · kiloBytesPerSecond · level 4, per_device_level 4 · Tier 2: Host, VM
    pub const USAGE_MINIMUM: Metric<Throughput, VmOrHost> = Metric::new(
        "net",
        "usage",
        Rollup::Minimum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `net.usage.none` (key 149) · kiloBytesPerSecond · level 4, per_device_level 4 · Tier 2: Host, VM
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

    /// `nfs.apdState.latest` (key 932) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const APD_STATE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "nfs",
        "apdState",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `nfs.createFailed.latest` (key 943) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const CREATE_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "nfs",
        "createFailed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `nfs.ioRequestsQueued.latest` (key 941) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const IO_REQUESTS_QUEUED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "nfs",
        "ioRequestsQueued",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `nfs.readsFailed.latest` (key 936) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const READS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "nfs",
        "readsFailed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `nfs.readIssueTime.latest` (key 933) · microsecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const READ_ISSUE_TIME_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "nfs",
        "readIssueTime",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `nfs.readTime.latest` (key 939) · microsecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const READ_TIME_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "nfs",
        "readTime",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `nfs.socketBufferFull.latest` (key 944) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const SOCKET_BUFFER_FULL_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "nfs",
        "socketBufferFull",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `nfs.totalCreate.latest` (key 942) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const TOTAL_CREATE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "nfs",
        "totalCreate",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `nfs.totalReads.latest` (key 935) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const TOTAL_READS_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "nfs",
        "totalReads",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `nfs.totalWrites.latest` (key 937) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const TOTAL_WRITES_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "nfs",
        "totalWrites",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `nfs.writesFailed.latest` (key 938) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const WRITES_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "nfs",
        "writesFailed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `nfs.writeIssueTime.latest` (key 934) · microsecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const WRITE_ISSUE_TIME_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "nfs",
        "writeIssueTime",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `nfs.writeTime.latest` (key 940) · microsecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
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

    /// `pmem.available.reservation.latest` (key 626) · megaBytes · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host, VM
    pub const AVAILABLE_RESERVATION_LATEST: Metric<Memory, VmOrHost> = Metric::new(
        "pmem",
        "available.reservation",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `pmem.drsmanaged.reservation.latest` (key 627) · megaBytes · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host, VM
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

    /// `power.capacity.usable.average` (key 218) · watt · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_USABLE_AVERAGE: Metric<Power, ComputeScope> = Metric::new(
        "power",
        "capacity.usable",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `power.capacity.usage.average` (key 219) · watt · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_USAGE_AVERAGE: Metric<Power, ComputeScope> = Metric::new(
        "power",
        "capacity.usage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `power.capacity.usageCpu.average` (key 929) · watt · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_USAGE_CPU_AVERAGE: Metric<Power, ComputeScope> = Metric::new(
        "power",
        "capacity.usageCpu",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `power.capacity.usageIdle.average` (key 220) · watt · level 2, per_device_level 3 · Tier 1: Host
    pub const CAPACITY_USAGE_IDLE_AVERAGE: Metric<Power, HostOnly> = Metric::new(
        "power",
        "capacity.usageIdle",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `power.capacity.usageMem.average` (key 930) · watt · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_USAGE_MEM_AVERAGE: Metric<Power, ComputeScope> = Metric::new(
        "power",
        "capacity.usageMem",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `power.capacity.usageOther.average` (key 931) · watt · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_USAGE_OTHER_AVERAGE: Metric<Power, ComputeScope> = Metric::new(
        "power",
        "capacity.usageOther",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `power.capacity.usagePct.average` (key 167) · percent (hundredths) · level 4, per_device_level 4 · Tier 3a: Cluster, Host, Pool, VM
    pub const CAPACITY_USAGE_PCT_AVERAGE: Metric<Ratio, ComputeScope> = Metric::new(
        "power",
        "capacity.usagePct",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `power.capacity.usageStatic.average` (key 701) · watt · level 2, per_device_level 3 · Tier 1: VM
    pub const CAPACITY_USAGE_STATIC_AVERAGE: Metric<Power, VmOnly> = Metric::new(
        "power",
        "capacity.usageStatic",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `power.capacity.usageSystem.average` (key 221) · watt · level 2, per_device_level 3 · Tier 1: Host
    pub const CAPACITY_USAGE_SYSTEM_AVERAGE: Metric<Power, HostOnly> = Metric::new(
        "power",
        "capacity.usageSystem",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `power.capacity.usageVm.average` (key 222) · watt · level 2, per_device_level 3 · Tier 1: Host
    pub const CAPACITY_USAGE_VM_AVERAGE: Metric<Power, HostOnly> = Metric::new(
        "power",
        "capacity.usageVm",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `power.energy.summation` (key 166) · joule · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const ENERGY_SUMMATION: Metric<Energy, VmOrHost> = Metric::new(
        "power",
        "energy",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `power.power.average` (key 164) · watt · level 2, per_device_level 3 · Tier 1: Cluster, Host, Pool, VM
    pub const POWER_AVERAGE: Metric<Power, ComputeScope> = Metric::new(
        "power",
        "power",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(3u8),
        false,
    );

    /// `power.powerCap.average` (key 165) · watt · level 3, per_device_level 3 · Tier 1: Host
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

    /// `rescpu.actav15.latest` (key 556) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const ACTAV15_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "actav15",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `rescpu.actav1.latest` (key 346) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const ACTAV1_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "actav1",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `rescpu.actav5.latest` (key 553) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const ACTAV5_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "actav5",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `rescpu.actpk15.latest` (key 557) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const ACTPK15_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "actpk15",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `rescpu.actpk1.latest` (key 551) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const ACTPK1_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "actpk1",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `rescpu.actpk5.latest` (key 554) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const ACTPK5_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "actpk5",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `rescpu.maxLimited15.latest` (key 564) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const MAX_LIMITED15_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "maxLimited15",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `rescpu.maxLimited1.latest` (key 560) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const MAX_LIMITED1_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "maxLimited1",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `rescpu.maxLimited5.latest` (key 562) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const MAX_LIMITED5_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "maxLimited5",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `rescpu.runav15.latest` (key 558) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const RUNAV15_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "runav15",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `rescpu.runav1.latest` (key 552) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const RUNAV1_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "runav1",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `rescpu.runav5.latest` (key 555) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const RUNAV5_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "runav5",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `rescpu.runpk15.latest` (key 563) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const RUNPK15_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "runpk15",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `rescpu.runpk1.latest` (key 559) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const RUNPK1_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "runpk1",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `rescpu.runpk5.latest` (key 561) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const RUNPK5_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "rescpu",
        "runpk5",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `rescpu.sampleCount.latest` (key 565) · number · level 3, per_device_level 3 · Tier 1: Host, VM
    pub const SAMPLE_COUNT_LATEST: Metric<Count, VmOrHost> = Metric::new(
        "rescpu",
        "sampleCount",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `rescpu.samplePeriod.latest` (key 566) · millisecond · level 3, per_device_level 3 · Tier 1: Host, VM
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

    /// `storageAdapter.commandsAveraged.average` (key 168) · number · level 2, per_device_level 2 · Tier 1: Host
    pub const COMMANDS_AVERAGED_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "storageAdapter",
        "commandsAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// `storageAdapter.maxTotalLatency.latest` (key 175) · millisecond · level 3, per_device_level 3 · Tier 1: Host
    pub const MAX_TOTAL_LATENCY_LATEST: Metric<Time, HostOnly> = Metric::new(
        "storageAdapter",
        "maxTotalLatency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `storageAdapter.numberReadAveraged.average` (key 169) · number · level 2, per_device_level 2 · Tier 1: Host
    pub const NUMBER_READ_AVERAGED_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "storageAdapter",
        "numberReadAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// `storageAdapter.numberWriteAveraged.average` (key 170) · number · level 2, per_device_level 2 · Tier 1: Host
    pub const NUMBER_WRITE_AVERAGED_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "storageAdapter",
        "numberWriteAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// `storageAdapter.OIOsPct.average` (key 177) · percent (hundredths) · level 4, per_device_level 4 · Tier 3a: Host
    pub const OIOS_PCT_AVERAGE: Metric<Ratio, HostOnly> = Metric::new(
        "storageAdapter",
        "OIOsPct",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `storageAdapter.outstandingIOs.average` (key 204) · number · level 2, per_device_level 2 · Tier 3a: Host
    pub const OUTSTANDING_IOS_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "storageAdapter",
        "outstandingIOs",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        false,
    );

    /// `storageAdapter.outstandingIOs.latest` (key 359) · number · level 4, per_device_level 4 · Tier 3a: Host
    pub const OUTSTANDING_IOS_LATEST: Metric<Count, HostOnly> = Metric::new(
        "storageAdapter",
        "outstandingIOs",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `storageAdapter.queued.average` (key 205) · number · level 2, per_device_level 2 · Tier 3a: Host
    pub const QUEUED_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "storageAdapter",
        "queued",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        false,
    );

    /// `storageAdapter.queued.latest` (key 360) · number · level 4, per_device_level 4 · Tier 3a: Host
    pub const QUEUED_LATEST: Metric<Count, HostOnly> = Metric::new(
        "storageAdapter",
        "queued",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `storageAdapter.queueDepth.average` (key 206) · number · level 2, per_device_level 2 · Tier 3a: Host
    pub const QUEUE_DEPTH_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "storageAdapter",
        "queueDepth",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        false,
    );

    /// `storageAdapter.queueDepth.latest` (key 361) · number · level 4, per_device_level 4 · Tier 3a: Host
    pub const QUEUE_DEPTH_LATEST: Metric<Count, HostOnly> = Metric::new(
        "storageAdapter",
        "queueDepth",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `storageAdapter.queueLatency.average` (key 207) · millisecond · level 2, per_device_level 2 · Tier 1: Host
    pub const QUEUE_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "storageAdapter",
        "queueLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// `storageAdapter.read.average` (key 171) · kiloBytesPerSecond · level 2, per_device_level 2 · Tier 1: Host
    pub const READ_AVERAGE: Metric<Throughput, HostOnly> = Metric::new(
        "storageAdapter",
        "read",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// `storageAdapter.throughput.cont.average` (key 176) · millisecond · level 4, per_device_level 4 · Tier 3a: Host
    pub const THROUGHPUT_CONT_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "storageAdapter",
        "throughput.cont",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `storageAdapter.throughput.usag.average` (key 208) · kiloBytesPerSecond · level 4, per_device_level 4 · Tier 3a: Host
    pub const THROUGHPUT_USAG_AVERAGE: Metric<Throughput, HostOnly> = Metric::new(
        "storageAdapter",
        "throughput.usag",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `storageAdapter.totalReadLatency.average` (key 173) · millisecond · level 2, per_device_level 2 · Tier 1: Host
    pub const TOTAL_READ_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "storageAdapter",
        "totalReadLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// `storageAdapter.totalWriteLatency.average` (key 174) · millisecond · level 2, per_device_level 2 · Tier 1: Host
    pub const TOTAL_WRITE_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "storageAdapter",
        "totalWriteLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// `storageAdapter.write.average` (key 172) · kiloBytesPerSecond · level 2, per_device_level 2 · Tier 1: Host
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

    /// `storagePath.busResets.summation` (key 209) · number · level 2, per_device_level 3 · Tier 1: Host
    pub const BUS_RESETS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "storagePath",
        "busResets",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `storagePath.commandsAborted.summation` (key 210) · number · level 2, per_device_level 3 · Tier 1: Host
    pub const COMMANDS_ABORTED_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "storagePath",
        "commandsAborted",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(3u8),
        true,
    );

    /// `storagePath.commandsAveraged.average` (key 570) · number · level 3, per_device_level 3 · Tier 1: Host
    pub const COMMANDS_AVERAGED_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "storagePath",
        "commandsAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `storagePath.maxTotalLatency.latest` (key 200) · millisecond · level 3, per_device_level 3 · Tier 1: Host
    pub const MAX_TOTAL_LATENCY_LATEST: Metric<Time, HostOnly> = Metric::new(
        "storagePath",
        "maxTotalLatency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `storagePath.numberReadAveraged.average` (key 571) · number · level 3, per_device_level 3 · Tier 1: Host
    pub const NUMBER_READ_AVERAGED_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "storagePath",
        "numberReadAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `storagePath.numberWriteAveraged.average` (key 572) · number · level 3, per_device_level 3 · Tier 1: Host
    pub const NUMBER_WRITE_AVERAGED_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "storagePath",
        "numberWriteAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `storagePath.read.average` (key 573) · kiloBytesPerSecond · level 3, per_device_level 3 · Tier 1: Host
    pub const READ_AVERAGE: Metric<Throughput, HostOnly> = Metric::new(
        "storagePath",
        "read",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `storagePath.throughput.cont.average` (key 199) · millisecond · level 4, per_device_level 4 · Tier 3a: Host
    pub const THROUGHPUT_CONT_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "storagePath",
        "throughput.cont",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `storagePath.throughput.usage.average` (key 211) · kiloBytesPerSecond · level 4, per_device_level 4 · Tier 3a: Host
    pub const THROUGHPUT_USAGE_AVERAGE: Metric<Throughput, HostOnly> = Metric::new(
        "storagePath",
        "throughput.usage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `storagePath.totalReadLatency.average` (key 575) · millisecond · level 3, per_device_level 3 · Tier 1: Host
    pub const TOTAL_READ_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "storagePath",
        "totalReadLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `storagePath.totalWriteLatency.average` (key 576) · millisecond · level 3, per_device_level 3 · Tier 1: Host
    pub const TOTAL_WRITE_LATENCY_AVERAGE: Metric<Time, HostOnly> = Metric::new(
        "storagePath",
        "totalWriteLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `storagePath.write.average` (key 574) · kiloBytesPerSecond · level 3, per_device_level 3 · Tier 1: Host
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

    /// `sys.diskUsage.latest` (key 524) · percent (hundredths) · level 3, per_device_level 3 · Tier 3a: Host, VM
    pub const DISK_USAGE_LATEST: Metric<Ratio, VmOrHost> = Metric::new(
        "sys",
        "diskUsage",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `sys.heartbeat.latest` (key 523) · number · level 4, per_device_level 4 · Tier 1: VM
    pub const HEARTBEAT_LATEST: Metric<Count, VmOnly> = Metric::new(
        "sys",
        "heartbeat",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `sys.heartbeat.summation` (key 163) · number · level 1, per_device_level 3 · Tier 2: VM
    pub const HEARTBEAT_SUMMATION: Metric<Count, VmOnly> = Metric::new(
        "sys",
        "heartbeat",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `sys.osUptime.latest` (key 548) · second · level 4, per_device_level 4 · Tier 1: VM
    pub const OS_UPTIME_LATEST: Metric<Time, VmOnly> = Metric::new(
        "sys",
        "osUptime",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `sys.resourceCpuAct1.latest` (key 537) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host
    pub const RESOURCE_CPU_ACT1_LATEST: Metric<Ratio, HostOnly> = Metric::new(
        "sys",
        "resourceCpuAct1",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `sys.resourceCpuAct5.latest` (key 540) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host
    pub const RESOURCE_CPU_ACT5_LATEST: Metric<Ratio, HostOnly> = Metric::new(
        "sys",
        "resourceCpuAct5",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `sys.resourceCpuAllocMax.latest` (key 543) · megaHertz · level 3, per_device_level 3 · Tier 1: Host
    pub const RESOURCE_CPU_ALLOC_MAX_LATEST: Metric<Frequency, HostOnly> = Metric::new(
        "sys",
        "resourceCpuAllocMax",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `sys.resourceCpuAllocMin.latest` (key 542) · megaHertz · level 3, per_device_level 3 · Tier 1: Host
    pub const RESOURCE_CPU_ALLOC_MIN_LATEST: Metric<Frequency, HostOnly> = Metric::new(
        "sys",
        "resourceCpuAllocMin",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `sys.resourceCpuAllocShares.latest` (key 544) · number · level 3, per_device_level 3 · Tier 1: Host
    pub const RESOURCE_CPU_ALLOC_SHARES_LATEST: Metric<Count, HostOnly> = Metric::new(
        "sys",
        "resourceCpuAllocShares",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `sys.resourceCpuMaxLimited1.latest` (key 538) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host
    pub const RESOURCE_CPU_MAX_LIMITED1_LATEST: Metric<Ratio, HostOnly> = Metric::new(
        "sys",
        "resourceCpuMaxLimited1",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `sys.resourceCpuMaxLimited5.latest` (key 541) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host
    pub const RESOURCE_CPU_MAX_LIMITED5_LATEST: Metric<Ratio, HostOnly> = Metric::new(
        "sys",
        "resourceCpuMaxLimited5",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `sys.resourceCpuRun1.latest` (key 536) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host
    pub const RESOURCE_CPU_RUN1_LATEST: Metric<Ratio, HostOnly> = Metric::new(
        "sys",
        "resourceCpuRun1",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `sys.resourceCpuRun5.latest` (key 539) · percent (hundredths) · level 3, per_device_level 3 · Tier 1: Host
    pub const RESOURCE_CPU_RUN5_LATEST: Metric<Ratio, HostOnly> = Metric::new(
        "sys",
        "resourceCpuRun5",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `sys.resourceCpuUsage.average` (key 526) · megaHertz · level 3, per_device_level 3 · Tier 1: Host
    pub const RESOURCE_CPU_USAGE_AVERAGE: Metric<Frequency, HostOnly> = Metric::new(
        "sys",
        "resourceCpuUsage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `sys.resourceCpuUsage.maximum` (key 527) · megaHertz · level 4, per_device_level 4 · Tier 2: Host
    pub const RESOURCE_CPU_USAGE_MAXIMUM: Metric<Frequency, HostOnly> = Metric::new(
        "sys",
        "resourceCpuUsage",
        Rollup::Maximum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `sys.resourceCpuUsage.minimum` (key 528) · megaHertz · level 4, per_device_level 4 · Tier 2: Host
    pub const RESOURCE_CPU_USAGE_MINIMUM: Metric<Frequency, HostOnly> = Metric::new(
        "sys",
        "resourceCpuUsage",
        Rollup::Minimum,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `sys.resourceCpuUsage.none` (key 525) · megaHertz · level 4, per_device_level 4 · Tier 2: Host
    pub const RESOURCE_CPU_USAGE_NONE: Metric<Frequency, HostOnly> = Metric::new(
        "sys",
        "resourceCpuUsage",
        Rollup::None,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `sys.resourceFdUsage.latest` (key 550) · number · level 4, per_device_level 4 · Tier 1: Host
    pub const RESOURCE_FD_USAGE_LATEST: Metric<Count, HostOnly> = Metric::new(
        "sys",
        "resourceFdUsage",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// `sys.resourceMemAllocMax.latest` (key 546) · kiloBytes · level 3, per_device_level 3 · Tier 1: Host
    pub const RESOURCE_MEM_ALLOC_MAX_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "sys",
        "resourceMemAllocMax",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `sys.resourceMemAllocMin.latest` (key 545) · kiloBytes · level 3, per_device_level 3 · Tier 1: Host
    pub const RESOURCE_MEM_ALLOC_MIN_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "sys",
        "resourceMemAllocMin",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `sys.resourceMemAllocShares.latest` (key 547) · number · level 3, per_device_level 3 · Tier 1: Host
    pub const RESOURCE_MEM_ALLOC_SHARES_LATEST: Metric<Count, HostOnly> = Metric::new(
        "sys",
        "resourceMemAllocShares",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `sys.resourceMemConsumed.latest` (key 549) · kiloBytes · level 4, per_device_level 4 · Tier 1: Host
    pub const RESOURCE_MEM_CONSUMED_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "sys",
        "resourceMemConsumed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// `sys.resourceMemCow.latest` (key 534) · kiloBytes · level 3, per_device_level 3 · Tier 1: Host
    pub const RESOURCE_MEM_COW_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "sys",
        "resourceMemCow",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `sys.resourceMemMapped.latest` (key 530) · kiloBytes · level 3, per_device_level 3 · Tier 1: Host
    pub const RESOURCE_MEM_MAPPED_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "sys",
        "resourceMemMapped",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `sys.resourceMemOverhead.latest` (key 533) · kiloBytes · level 3, per_device_level 3 · Tier 1: Host
    pub const RESOURCE_MEM_OVERHEAD_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "sys",
        "resourceMemOverhead",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `sys.resourceMemShared.latest` (key 531) · kiloBytes · level 3, per_device_level 3 · Tier 1: Host
    pub const RESOURCE_MEM_SHARED_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "sys",
        "resourceMemShared",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `sys.resourceMemSwapped.latest` (key 532) · kiloBytes · level 3, per_device_level 3 · Tier 1: Host
    pub const RESOURCE_MEM_SWAPPED_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "sys",
        "resourceMemSwapped",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `sys.resourceMemTouched.latest` (key 529) · kiloBytes · level 3, per_device_level 3 · Tier 1: Host
    pub const RESOURCE_MEM_TOUCHED_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "sys",
        "resourceMemTouched",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `sys.resourceMemZero.latest` (key 535) · kiloBytes · level 3, per_device_level 3 · Tier 1: Host
    pub const RESOURCE_MEM_ZERO_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "sys",
        "resourceMemZero",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        true,
    );

    /// `sys.uptime.latest` (key 162) · second · level 1, per_device_level 3 · Tier 1: Host, VM
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

    /// `vcDebugInfo.activationlatencystats.maximum` (key 287) · millisecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const ACTIVATIONLATENCYSTATS_MAXIMUM: Metric<Time, HostOnly> = Metric::new(
        "vcDebugInfo",
        "activationlatencystats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.activationlatencystats.minimum` (key 288) · millisecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const ACTIVATIONLATENCYSTATS_MINIMUM: Metric<Time, HostOnly> = Metric::new(
        "vcDebugInfo",
        "activationlatencystats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.activationlatencystats.summation` (key 289) · millisecond · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const ACTIVATIONLATENCYSTATS_SUMMATION: Metric<Time, HostOnly> = Metric::new(
        "vcDebugInfo",
        "activationlatencystats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcDebugInfo.activationstats.maximum` (key 290) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const ACTIVATIONSTATS_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "activationstats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.activationstats.minimum` (key 291) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const ACTIVATIONSTATS_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "activationstats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.activationstats.summation` (key 292) · number · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const ACTIVATIONSTATS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "activationstats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcDebugInfo.hostsynclatencystats.maximum` (key 300) · millisecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const HOSTSYNCLATENCYSTATS_MAXIMUM: Metric<Time, HostOnly> = Metric::new(
        "vcDebugInfo",
        "hostsynclatencystats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.hostsynclatencystats.minimum` (key 301) · millisecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const HOSTSYNCLATENCYSTATS_MINIMUM: Metric<Time, HostOnly> = Metric::new(
        "vcDebugInfo",
        "hostsynclatencystats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.hostsynclatencystats.summation` (key 302) · millisecond · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const HOSTSYNCLATENCYSTATS_SUMMATION: Metric<Time, HostOnly> = Metric::new(
        "vcDebugInfo",
        "hostsynclatencystats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcDebugInfo.hostsyncstats.maximum` (key 303) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const HOSTSYNCSTATS_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "hostsyncstats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.hostsyncstats.minimum` (key 304) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const HOSTSYNCSTATS_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "hostsyncstats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.hostsyncstats.summation` (key 305) · number · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const HOSTSYNCSTATS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "hostsyncstats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcDebugInfo.inventorystats.maximum` (key 306) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const INVENTORYSTATS_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "inventorystats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.inventorystats.minimum` (key 307) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const INVENTORYSTATS_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "inventorystats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.inventorystats.summation` (key 308) · number · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const INVENTORYSTATS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "inventorystats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcDebugInfo.lockstats.maximum` (key 309) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const LOCKSTATS_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "lockstats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.lockstats.minimum` (key 310) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const LOCKSTATS_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "lockstats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.lockstats.summation` (key 311) · number · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const LOCKSTATS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "lockstats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcDebugInfo.lrostats.maximum` (key 312) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const LROSTATS_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "lrostats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.lrostats.minimum` (key 313) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const LROSTATS_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "lrostats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.lrostats.summation` (key 314) · number · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const LROSTATS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "lrostats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcDebugInfo.miscstats.maximum` (key 315) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const MISCSTATS_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "miscstats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.miscstats.minimum` (key 316) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const MISCSTATS_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "miscstats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.miscstats.summation` (key 317) · number · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const MISCSTATS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "miscstats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcDebugInfo.morefregstats.maximum` (key 318) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const MOREFREGSTATS_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "morefregstats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.morefregstats.minimum` (key 319) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const MOREFREGSTATS_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "morefregstats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.morefregstats.summation` (key 320) · number · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const MOREFREGSTATS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "morefregstats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcDebugInfo.scoreboard.maximum` (key 327) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const SCOREBOARD_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "scoreboard",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.scoreboard.minimum` (key 328) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const SCOREBOARD_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "scoreboard",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.scoreboard.summation` (key 329) · number · level 3, per_device_level 3 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const SCOREBOARD_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "scoreboard",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(3u8),
        Some(3u8),
        false,
    );

    /// `vcDebugInfo.sessionstats.maximum` (key 330) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const SESSIONSTATS_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "sessionstats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.sessionstats.minimum` (key 331) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const SESSIONSTATS_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "sessionstats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.sessionstats.summation` (key 332) · number · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const SESSIONSTATS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "sessionstats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcDebugInfo.systemstats.maximum` (key 334) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const SYSTEMSTATS_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "systemstats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.systemstats.minimum` (key 335) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const SYSTEMSTATS_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "systemstats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.systemstats.summation` (key 336) · number · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const SYSTEMSTATS_SUMMATION: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "systemstats",
        Rollup::Summation,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcDebugInfo.vcservicestats.maximum` (key 338) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const VCSERVICESTATS_MAXIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "vcservicestats",
        Rollup::Maximum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.vcservicestats.minimum` (key 339) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const VCSERVICESTATS_MINIMUM: Metric<Count, HostOnly> = Metric::new(
        "vcDebugInfo",
        "vcservicestats",
        Rollup::Minimum,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcDebugInfo.vcservicestats.summation` (key 340) · number · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
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

    /// `vcResources.buffersz.average` (key 293) · kiloBytes · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const BUFFERSZ_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "vcResources",
        "buffersz",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcResources.cachesz.average` (key 294) · kiloBytes · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const CACHESZ_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "vcResources",
        "cachesz",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcResources.ctxswitchesrate.average` (key 295) · number · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const CTXSWITCHESRATE_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "vcResources",
        "ctxswitchesrate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcResources.diskreadsectorrate.average` (key 296) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const DISKREADSECTORRATE_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "vcResources",
        "diskreadsectorrate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcResources.diskreadsrate.average` (key 297) · number · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const DISKREADSRATE_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "vcResources",
        "diskreadsrate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcResources.diskwritesectorrate.average` (key 298) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const DISKWRITESECTORRATE_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "vcResources",
        "diskwritesectorrate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vcResources.diskwritesrate.average` (key 299) · number · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const DISKWRITESRATE_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "vcResources",
        "diskwritesrate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcResources.packetrecvrate.average` (key 321) · number · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const PACKETRECVRATE_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "vcResources",
        "packetrecvrate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcResources.packetsentrate.average` (key 322) · number · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const PACKETSENTRATE_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "vcResources",
        "packetsentrate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcResources.pagefaultrate.average` (key 324) · number · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const PAGEFAULTRATE_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "vcResources",
        "pagefaultrate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcResources.physicalmemusage.average` (key 325) · kiloBytes · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const PHYSICALMEMUSAGE_AVERAGE: Metric<Memory, HostOnly> = Metric::new(
        "vcResources",
        "physicalmemusage",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcResources.priviledgedcpuusage.average` (key 326) · percent (hundredths) · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const PRIVILEDGEDCPUUSAGE_AVERAGE: Metric<Ratio, HostOnly> = Metric::new(
        "vcResources",
        "priviledgedcpuusage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcResources.syscallsrate.average` (key 333) · number · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const SYSCALLSRATE_AVERAGE: Metric<Count, HostOnly> = Metric::new(
        "vcResources",
        "syscallsrate",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcResources.systemcpuusage.average` (key 323) · percent (hundredths) · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const SYSTEMCPUUSAGE_AVERAGE: Metric<Ratio, HostOnly> = Metric::new(
        "vcResources",
        "systemcpuusage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcResources.usercpuusage.average` (key 337) · percent (hundredths) · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const USERCPUUSAGE_AVERAGE: Metric<Ratio, HostOnly> = Metric::new(
        "vcResources",
        "usercpuusage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(1u8),
        false,
    );

    /// `vcResources.virtualmemusage.average` (key 341) · kiloBytes · level 1, per_device_level 1 · tier 3b: inferred scope (no active samples in lab dump) · Host
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

    /// `vflashModule.numActiveVMDKs.latest` (key 596) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
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

    /// `virtualDisk.busResets.summation` (key 203) · number · level 2, per_device_level 4 · Tier 1: VM
    pub const BUS_RESETS_SUMMATION: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "busResets",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(4u8),
        true,
    );

    /// `virtualDisk.commandsAborted.summation` (key 202) · number · level 2, per_device_level 4 · Tier 1: VM
    pub const COMMANDS_ABORTED_SUMMATION: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "commandsAborted",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(2u8),
        Some(4u8),
        true,
    );

    /// `virtualDisk.hbr.readLatencyMS.latest` (key 405) · millisecond · level 4, per_device_level 4 · Tier 3a: VM
    pub const HBR_READ_LATENCY_MS_LATEST: Metric<Time, VmOnly> = Metric::new(
        "virtualDisk",
        "hbr.readLatencyMS",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `virtualDisk.hbr.stallLatencyMS.latest` (key 406) · millisecond · level 4, per_device_level 4 · Tier 3a: VM
    pub const HBR_STALL_LATENCY_MS_LATEST: Metric<Time, VmOnly> = Metric::new(
        "virtualDisk",
        "hbr.stallLatencyMS",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `virtualDisk.largeSeeks.latest` (key 581) · number · level 4, per_device_level 4 · Tier 1: VM
    pub const LARGE_SEEKS_LATEST: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "largeSeeks",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// `virtualDisk.mediumSeeks.latest` (key 580) · number · level 4, per_device_level 4 · Tier 1: VM
    pub const MEDIUM_SEEKS_LATEST: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "mediumSeeks",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// `virtualDisk.numberReadAveraged.average` (key 178) · number · level 1, per_device_level 3 · Tier 1: VM
    pub const NUMBER_READ_AVERAGED_AVERAGE: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "numberReadAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `virtualDisk.numberWriteAveraged.average` (key 179) · number · level 1, per_device_level 3 · Tier 1: VM
    pub const NUMBER_WRITE_AVERAGED_AVERAGE: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "numberWriteAveraged",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `virtualDisk.read.average` (key 180) · kiloBytesPerSecond · level 2, per_device_level 2 · Tier 1: VM
    pub const READ_AVERAGE: Metric<Throughput, VmOnly> = Metric::new(
        "virtualDisk",
        "read",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// `virtualDisk.readIOSize.latest` (key 577) · number · level 4, per_device_level 4 · Tier 1: VM
    pub const READ_IOSIZE_LATEST: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "readIOSize",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// `virtualDisk.readLatencyUS.latest` (key 582) · microsecond · level 4, per_device_level 4 · Tier 1: VM
    pub const READ_LATENCY_US_LATEST: Metric<Time, VmOnly> = Metric::new(
        "virtualDisk",
        "readLatencyUS",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// `virtualDisk.readLoadMetric.latest` (key 344) · number · level 2, per_device_level 2 · Tier 1: VM
    pub const READ_LOAD_METRIC_LATEST: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "readLoadMetric",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// `virtualDisk.readOIO.latest` (key 342) · number · level 2, per_device_level 2 · Tier 1: VM
    pub const READ_OIO_LATEST: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "readOIO",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// `virtualDisk.smallSeeks.latest` (key 579) · number · level 4, per_device_level 4 · Tier 1: VM
    pub const SMALL_SEEKS_LATEST: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "smallSeeks",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// `virtualDisk.throughput.cont.average` (key 184) · millisecond · level 4, per_device_level 4 · Tier 3a: VM
    pub const THROUGHPUT_CONT_AVERAGE: Metric<Time, VmOnly> = Metric::new(
        "virtualDisk",
        "throughput.cont",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `virtualDisk.throughput.usage.average` (key 201) · kiloBytesPerSecond · level 4, per_device_level 4 · Tier 3a: VM
    pub const THROUGHPUT_USAGE_AVERAGE: Metric<Throughput, VmOnly> = Metric::new(
        "virtualDisk",
        "throughput.usage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `virtualDisk.totalReadLatency.average` (key 182) · millisecond · level 1, per_device_level 3 · Tier 1: VM
    pub const TOTAL_READ_LATENCY_AVERAGE: Metric<Time, VmOnly> = Metric::new(
        "virtualDisk",
        "totalReadLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `virtualDisk.totalWriteLatency.average` (key 183) · millisecond · level 1, per_device_level 3 · Tier 1: VM
    pub const TOTAL_WRITE_LATENCY_AVERAGE: Metric<Time, VmOnly> = Metric::new(
        "virtualDisk",
        "totalWriteLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        true,
    );

    /// `virtualDisk.write.average` (key 181) · kiloBytesPerSecond · level 2, per_device_level 2 · Tier 1: VM
    pub const WRITE_AVERAGE: Metric<Throughput, VmOnly> = Metric::new(
        "virtualDisk",
        "write",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// `virtualDisk.writeIOSize.latest` (key 578) · number · level 4, per_device_level 4 · Tier 1: VM
    pub const WRITE_IOSIZE_LATEST: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "writeIOSize",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// `virtualDisk.writeLatencyUS.latest` (key 583) · microsecond · level 4, per_device_level 4 · Tier 1: VM
    pub const WRITE_LATENCY_US_LATEST: Metric<Time, VmOnly> = Metric::new(
        "virtualDisk",
        "writeLatencyUS",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        true,
    );

    /// `virtualDisk.writeLoadMetric.latest` (key 345) · number · level 2, per_device_level 2 · Tier 1: VM
    pub const WRITE_LOAD_METRIC_LATEST: Metric<Count, VmOnly> = Metric::new(
        "virtualDisk",
        "writeLoadMetric",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(2u8),
        Some(2u8),
        true,
    );

    /// `virtualDisk.writeOIO.latest` (key 343) · number · level 2, per_device_level 2 · Tier 1: VM
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

    /// `vmop.numChangeDS.latest` (key 267) · number · level 1, per_device_level 3 · Tier 1: Cluster, Datacenter
    pub const NUM_CHANGE_DS_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numChangeDS",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `vmop.numChangeHostDS.latest` (key 268) · number · level 1, per_device_level 3 · Tier 1: Cluster, Datacenter
    pub const NUM_CHANGE_HOST_DS_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numChangeHostDS",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `vmop.numChangeHost.latest` (key 266) · number · level 1, per_device_level 3 · Tier 1: Cluster, Datacenter
    pub const NUM_CHANGE_HOST_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numChangeHost",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `vmop.numClone.latest` (key 264) · number · level 1, per_device_level 3 · Tier 1: Cluster, Datacenter
    pub const NUM_CLONE_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numClone",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `vmop.numCreate.latest` (key 259) · number · level 1, per_device_level 3 · Tier 1: Cluster, Datacenter
    pub const NUM_CREATE_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numCreate",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `vmop.numDeploy.latest` (key 265) · number · level 1, per_device_level 3 · Tier 1: Cluster, Datacenter
    pub const NUM_DEPLOY_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numDeploy",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `vmop.numDestroy.latest` (key 260) · number · level 1, per_device_level 3 · Tier 1: Cluster, Datacenter
    pub const NUM_DESTROY_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numDestroy",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `vmop.numPoweroff.latest` (key 253) · number · level 1, per_device_level 3 · Tier 1: Cluster, Datacenter
    pub const NUM_POWEROFF_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numPoweroff",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `vmop.numPoweron.latest` (key 252) · number · level 1, per_device_level 3 · Tier 1: Cluster, Datacenter
    pub const NUM_POWERON_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numPoweron",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `vmop.numRebootGuest.latest` (key 256) · number · level 1, per_device_level 3 · Tier 1: Cluster, Datacenter
    pub const NUM_REBOOT_GUEST_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numRebootGuest",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `vmop.numReconfigure.latest` (key 263) · number · level 1, per_device_level 3 · Tier 1: Cluster, Datacenter
    pub const NUM_RECONFIGURE_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numReconfigure",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `vmop.numRegister.latest` (key 261) · number · level 1, per_device_level 3 · Tier 1: Cluster, Datacenter
    pub const NUM_REGISTER_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numRegister",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `vmop.numReset.latest` (key 255) · number · level 1, per_device_level 3 · Tier 1: Cluster, Datacenter
    pub const NUM_RESET_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numReset",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `vmop.numShutdownGuest.latest` (key 258) · number · level 1, per_device_level 3 · Tier 1: Cluster, Datacenter
    pub const NUM_SHUTDOWN_GUEST_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numShutdownGuest",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `vmop.numStandbyGuest.latest` (key 257) · number · level 1, per_device_level 3 · Tier 1: Cluster, Datacenter
    pub const NUM_STANDBY_GUEST_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numStandbyGuest",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `vmop.numSuspend.latest` (key 254) · number · level 1, per_device_level 3 · Tier 1: Cluster, Datacenter
    pub const NUM_SUSPEND_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numSuspend",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `vmop.numSVMotion.latest` (key 270) · number · level 1, per_device_level 3 · Tier 1: Cluster, Datacenter
    pub const NUM_SVMOTION_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numSVMotion",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `vmop.numUnregister.latest` (key 262) · number · level 1, per_device_level 3 · Tier 1: Cluster, Datacenter
    pub const NUM_UNREGISTER_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numUnregister",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `vmop.numVMotion.latest` (key 269) · number · level 1, per_device_level 3 · Tier 1: Cluster, Datacenter
    pub const NUM_VMOTION_LATEST: Metric<Count, ClusterDc> = Metric::new(
        "vmop",
        "numVMotion",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(1u8),
        Some(3u8),
        false,
    );

    /// `vmop.numXVMotion.latest` (key 271) · number · level 1, per_device_level 3 · Tier 1: Cluster, Datacenter
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

    /// `vmotion.downtime.latest` (key 397) · microsecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const DOWNTIME_LATEST: Metric<Time, HostOnly> = Metric::new(
        "vmotion",
        "downtime",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmotion.dst.migration.time.latest` (key 400) · second · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const DST_MIGRATION_TIME_LATEST: Metric<Time, HostOnly> = Metric::new(
        "vmotion",
        "dst.migration.time",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmotion.mem.sizemb.latest` (key 401) · megaBytes · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const MEM_SIZEMB_LATEST: Metric<Memory, HostOnly> = Metric::new(
        "vmotion",
        "mem.sizemb",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmotion.precopy.time.latest` (key 398) · microsecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const PRECOPY_TIME_LATEST: Metric<Time, HostOnly> = Metric::new(
        "vmotion",
        "precopy.time",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmotion.rtt.latest` (key 399) · microsecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
    pub const RTT_LATEST: Metric<Time, HostOnly> = Metric::new(
        "vmotion",
        "rtt",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmotion.vmkernel.downtime.latest` (key 396) · microsecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Host
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

    /// `vmx.cpuSpeed.latest` (key 413) · megaHertz · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · VM
    pub const CPU_SPEED_LATEST: Metric<Frequency, VmOnly> = Metric::new(
        "vmx",
        "cpuSpeed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmx.numVCPUs.latest` (key 409) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · VM
    pub const NUM_VCPUS_LATEST: Metric<Count, VmOnly> = Metric::new(
        "vmx",
        "numVCPUs",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmx.overheadMemSizeMax.latest` (key 415) · megaBytes · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · VM
    pub const OVERHEAD_MEM_SIZE_MAX_LATEST: Metric<Memory, VmOnly> = Metric::new(
        "vmx",
        "overheadMemSizeMax",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmx.overheadMemSizeMin.latest` (key 414) · megaBytes · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · VM
    pub const OVERHEAD_MEM_SIZE_MIN_LATEST: Metric<Memory, VmOnly> = Metric::new(
        "vmx",
        "overheadMemSizeMin",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmx.poll.itersPerS.latest` (key 1021) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · VM
    pub const POLL_ITERS_PER_S_LATEST: Metric<Count, VmOnly> = Metric::new(
        "vmx",
        "poll.itersPerS",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmx.userRpc.opsPerS.latest` (key 1022) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · VM
    pub const USER_RPC_OPS_PER_S_LATEST: Metric<Count, VmOnly> = Metric::new(
        "vmx",
        "userRpc.opsPerS",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmx.vcpusMhzMax.latest` (key 411) · megaHertz · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · VM
    pub const VCPUS_MHZ_MAX_LATEST: Metric<Frequency, VmOnly> = Metric::new(
        "vmx",
        "vcpusMhzMax",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmx.vcpusMhzMean.latest` (key 412) · megaHertz · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · VM
    pub const VCPUS_MHZ_MEAN_LATEST: Metric<Frequency, VmOnly> = Metric::new(
        "vmx",
        "vcpusMhzMean",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmx.vcpusMhzMin.latest` (key 410) · megaHertz · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · VM
    pub const VCPUS_MHZ_MIN_LATEST: Metric<Frequency, VmOnly> = Metric::new(
        "vmx",
        "vcpusMhzMin",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vmx.vigor.opsTotal.latest` (key 1020) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · VM
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

    /// `vsanDomObj.readAvgLatency.average` (key 599) · millisecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const READ_AVG_LATENCY_AVERAGE: Metric<Time, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "readAvgLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vsanDomObj.readCacheHitRate.latest` (key 601) · percent (hundredths) · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const READ_CACHE_HIT_RATE_LATEST: Metric<Ratio, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "readCacheHitRate",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vsanDomObj.readCongestion.average` (key 602) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const READ_CONGESTION_AVERAGE: Metric<Count, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "readCongestion",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vsanDomObj.readIops.average` (key 597) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const READ_IOPS_AVERAGE: Metric<Count, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "readIops",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vsanDomObj.readMaxLatency.latest` (key 600) · millisecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const READ_MAX_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "readMaxLatency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vsanDomObj.readThroughput.average` (key 598) · kiloBytesPerSecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const READ_THROUGHPUT_AVERAGE: Metric<Throughput, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "readThroughput",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vsanDomObj.recoveryWriteAvgLatency.average` (key 610) · millisecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const RECOVERY_WRITE_AVG_LATENCY_AVERAGE: Metric<Time, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "recoveryWriteAvgLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vsanDomObj.recoveryWriteCongestion.average` (key 612) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const RECOVERY_WRITE_CONGESTION_AVERAGE: Metric<Count, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "recoveryWriteCongestion",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vsanDomObj.recoveryWriteIops.average` (key 608) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const RECOVERY_WRITE_IOPS_AVERAGE: Metric<Count, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "recoveryWriteIops",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vsanDomObj.recoveryWriteMaxLatency.latest` (key 611) · millisecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const RECOVERY_WRITE_MAX_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "recoveryWriteMaxLatency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vsanDomObj.recoveryWriteThroughput.average` (key 609) · kiloBytesPerSecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const RECOVERY_WRITE_THROUGHPUT_AVERAGE: Metric<Throughput, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "recoveryWriteThroughput",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vsanDomObj.writeAvgLatency.average` (key 605) · millisecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const WRITE_AVG_LATENCY_AVERAGE: Metric<Time, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "writeAvgLatency",
        Rollup::Average,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vsanDomObj.writeCongestion.average` (key 607) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const WRITE_CONGESTION_AVERAGE: Metric<Count, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "writeCongestion",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vsanDomObj.writeIops.average` (key 603) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const WRITE_IOPS_AVERAGE: Metric<Count, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "writeIops",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vsanDomObj.writeMaxLatency.latest` (key 606) · millisecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const WRITE_MAX_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vsanDomObj",
        "writeMaxLatency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vsanDomObj.writeThroughput.average` (key 604) · kiloBytesPerSecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
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

    /// `vvol.container.issue.latency.latest` (key 988) · microsecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const CONTAINER_ISSUE_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.issue.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.container.readsissued.failed.latest` (key 980) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const CONTAINER_READSISSUED_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.readsissued.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.container.reads.done.latest` (key 974) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const CONTAINER_READS_DONE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.reads.done",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.container.reads.failed.latest` (key 983) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const CONTAINER_READS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.reads.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.container.reads.sent.latest` (key 977) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const CONTAINER_READS_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.reads.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.container.read.latency.latest` (key 986) · microsecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const CONTAINER_READ_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.read.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.container.totalissued.failed.latest` (key 982) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const CONTAINER_TOTALISSUED_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.totalissued.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.container.total.done.latest` (key 976) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const CONTAINER_TOTAL_DONE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.total.done",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.container.total.failed.latest` (key 985) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const CONTAINER_TOTAL_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.total.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.container.total.latency.latest` (key 989) · microsecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const CONTAINER_TOTAL_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.total.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.container.total.sent.latest` (key 979) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const CONTAINER_TOTAL_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.total.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.container.writesissued.failed.latest` (key 981) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const CONTAINER_WRITESISSUED_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.writesissued.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.container.writes.done.latest` (key 975) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const CONTAINER_WRITES_DONE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.writes.done",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.container.writes.failed.latest` (key 984) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const CONTAINER_WRITES_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.writes.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.container.writes.sent.latest` (key 978) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const CONTAINER_WRITES_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.writes.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.container.write.latency.latest` (key 987) · microsecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const CONTAINER_WRITE_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "container.write.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.cancel.failed.latest` (key 1007) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_CANCEL_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.cancel.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.cancel.sent.latest` (key 1006) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_CANCEL_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.cancel.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.deviceresets.failed.latest` (key 1009) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_DEVICERESETS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.deviceresets.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.deviceresets.sent.latest` (key 1008) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_DEVICERESETS_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.deviceresets.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.issue.latency.latest` (key 1004) · microsecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_ISSUE_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.issue.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.readsissued.failed.latest` (key 996) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_READSISSUED_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.readsissued.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.reads.done.latest` (key 990) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_READS_DONE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.reads.done",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.reads.failed.latest` (key 999) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_READS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.reads.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.reads.sent.latest` (key 993) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_READS_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.reads.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.read.latency.latest` (key 1002) · microsecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_READ_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.read.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.resets.failed.latest` (key 1011) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_RESETS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.resets.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.resets.sent.latest` (key 1010) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_RESETS_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.resets.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.totalissued.failed.latest` (key 998) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_TOTALISSUED_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.totalissued.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.total.done.latest` (key 992) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_TOTAL_DONE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.total.done",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.total.failed.latest` (key 1001) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_TOTAL_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.total.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.total.latency.latest` (key 1005) · microsecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_TOTAL_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.total.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.total.sent.latest` (key 995) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_TOTAL_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.total.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.unmaps.failed.latest` (key 1013) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_UNMAPS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.unmaps.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.unmaps.sent.latest` (key 1012) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_UNMAPS_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.unmaps.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.writesissued.failed.latest` (key 997) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_WRITESISSUED_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.writesissued.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.writes.done.latest` (key 991) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_WRITES_DONE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.writes.done",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.writes.failed.latest` (key 1000) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_WRITES_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.writes.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.writes.sent.latest` (key 994) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_WRITES_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.writes.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.device.write.latency.latest` (key 1003) · microsecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const DEVICE_WRITE_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "device.write.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.cancel.failed.latest` (key 967) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_CANCEL_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.cancel.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.cancel.sent.latest` (key 966) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_CANCEL_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.cancel.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.deviceresets.failed.latest` (key 969) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_DEVICERESETS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.deviceresets.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.deviceresets.sent.latest` (key 968) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_DEVICERESETS_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.deviceresets.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.isaccessible.latest` (key 949) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_ISACCESSIBLE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.isaccessible",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.issue.latency.latest` (key 964) · microsecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_ISSUE_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.issue.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.readsissued.failed.latest` (key 956) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_READSISSUED_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.readsissued.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.reads.done.latest` (key 950) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_READS_DONE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.reads.done",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.reads.failed.latest` (key 959) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_READS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.reads.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.reads.sent.latest` (key 953) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_READS_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.reads.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.read.latency.latest` (key 962) · microsecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_READ_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.read.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.resets.failed.latest` (key 971) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_RESETS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.resets.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.resets.sent.latest` (key 970) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_RESETS_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.resets.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.totalissued.failed.latest` (key 958) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_TOTALISSUED_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.totalissued.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.total.done.latest` (key 952) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_TOTAL_DONE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.total.done",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.total.failed.latest` (key 961) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_TOTAL_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.total.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.total.latency.latest` (key 965) · microsecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_TOTAL_LATENCY_LATEST: Metric<Time, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.total.latency",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.total.sent.latest` (key 955) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_TOTAL_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.total.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.unmaps.failed.latest` (key 973) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_UNMAPS_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.unmaps.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.unmaps.sent.latest` (key 972) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_UNMAPS_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.unmaps.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.writesissued.failed.latest` (key 957) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_WRITESISSUED_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.writesissued.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.writes.done.latest` (key 951) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_WRITES_DONE_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.writes.done",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.writes.failed.latest` (key 960) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_WRITES_FAILED_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.writes.failed",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.writes.sent.latest` (key 954) · number · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
    pub const PE_WRITES_SENT_LATEST: Metric<Count, DatastoreVmHost> = Metric::new(
        "vvol",
        "pe.writes.sent",
        Rollup::Latest,
        Some(StatsKind::Absolute),
        Some(4u8),
        Some(4u8),
        false,
    );

    /// `vvol.pe.write.latency.latest` (key 963) · microsecond · level 4, per_device_level 4 · tier 3b: inferred scope (no active samples in lab dump) · Datastore, Host, VM
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
            pub use crate::perf::metrics::cluster_services::CLUSTER_DRS_SCORE_LATEST;
            pub use crate::perf::metrics::cluster_services::EFFECTIVECPU_AVERAGE;
            pub use crate::perf::metrics::cluster_services::EFFECTIVEMEM_AVERAGE;
            pub use crate::perf::metrics::cluster_services::FAILOVER_LATEST;
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
            pub use crate::perf::metrics::cluster_services::CLUSTER_DRS_SCORE_LATEST;
            pub use crate::perf::metrics::cluster_services::CPUFAIRNESS_LATEST;
            pub use crate::perf::metrics::cluster_services::EFFECTIVECPU_AVERAGE;
            pub use crate::perf::metrics::cluster_services::EFFECTIVEMEM_AVERAGE;
            pub use crate::perf::metrics::cluster_services::FAILOVER_LATEST;
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
