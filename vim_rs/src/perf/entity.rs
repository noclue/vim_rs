//! Entity scopes for generated counters.

use crate::types::enums::MoTypesEnum;

/// `VirtualMachine`.
pub const MASK_VM: u8 = 1 << 0;
/// `HostSystem`.
pub const MASK_HOST: u8 = 1 << 1;
/// `ClusterComputeResource`.
pub const MASK_CLUSTER: u8 = 1 << 2;
/// `ResourcePool` and `VirtualApp`.
pub const MASK_RESOURCE_POOL: u8 = 1 << 3;
/// `Datastore` and `StoragePod`.
pub const MASK_DATASTORE: u8 = 1 << 4;
/// `Datacenter`.
pub const MASK_DATACENTER: u8 = 1 << 5;
/// Every kind. Only [`crate::perf::metric::Metric::custom`] uses this mask.
pub const MASK_ALL: u8 = 0x3F;

/// Bit for a managed-object type. Unknown kinds are `0`.
pub const fn mo_type_bit(mo: &MoTypesEnum) -> u8 {
    match mo {
        MoTypesEnum::VirtualMachine => MASK_VM,
        MoTypesEnum::HostSystem => MASK_HOST,
        MoTypesEnum::ClusterComputeResource => MASK_CLUSTER,
        MoTypesEnum::ResourcePool | MoTypesEnum::VirtualApp => MASK_RESOURCE_POOL,
        MoTypesEnum::Datastore | MoTypesEnum::StoragePod => MASK_DATASTORE,
        MoTypesEnum::Datacenter => MASK_DATACENTER,
        _ => 0,
    }
}

/// A compile-time set of entity kinds.
pub trait EntityScope: Copy + 'static {
    /// Bitmask of admitted kinds.
    const MASK: u8;

    /// Whether `mo` is inside this scope. [`MASK_ALL`] admits every kind.
    fn admits(mo: &MoTypesEnum) -> bool {
        Self::MASK == MASK_ALL || (Self::MASK & mo_type_bit(mo)) != 0
    }
}

/// Compile-time set of entity kinds.
///
/// The const parameters are, in order: virtual machine, host, cluster,
/// resource pool, datastore, datacenter. `true` admits that kind.
/// [`EntityScope::MASK`] packs the same flags into the runtime bitmask.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Scope<
    const VM: bool,
    const HOST: bool,
    const CLUSTER: bool,
    const RP: bool,
    const DS: bool,
    const DC: bool,
>;

impl<
        const VM: bool,
        const HOST: bool,
        const CLUSTER: bool,
        const RP: bool,
        const DS: bool,
        const DC: bool,
    > EntityScope for Scope<VM, HOST, CLUSTER, RP, DS, DC>
{
    const MASK: u8 = (VM as u8) * MASK_VM
        | (HOST as u8) * MASK_HOST
        | (CLUSTER as u8) * MASK_CLUSTER
        | (RP as u8) * MASK_RESOURCE_POOL
        | (DS as u8) * MASK_DATASTORE
        | (DC as u8) * MASK_DATACENTER;
}

/// Virtual machine only.
pub type VmOnly = Scope<true, false, false, false, false, false>;
/// Host only.
pub type HostOnly = Scope<false, true, false, false, false, false>;
/// Virtual machine or host.
pub type VmOrHost = Scope<true, true, false, false, false, false>;
/// Cluster only.
pub type ClusterOnly = Scope<false, false, true, false, false, false>;
/// Host or cluster.
pub type HostCluster = Scope<false, true, true, false, false, false>;
/// Virtual machine, host, or cluster.
pub type VmHostCluster = Scope<true, true, true, false, false, false>;
/// Virtual machine or resource pool.
pub type VmOrPool = Scope<true, false, false, true, false, false>;
/// Cluster or resource pool.
pub type ClusterPool = Scope<false, false, true, true, false, false>;
/// Virtual machine, host, cluster, or resource pool.
pub type ComputeScope = Scope<true, true, true, true, false, false>;
/// Datastore only.
pub type DatastoreOnly = Scope<false, false, false, false, true, false>;
/// Datastore or virtual machine.
pub type DatastoreVm = Scope<true, false, false, false, true, false>;
/// Datastore, virtual machine, or host.
pub type DatastoreVmHost = Scope<true, true, false, false, true, false>;
/// Cluster or datacenter.
pub type ClusterDc = Scope<false, false, true, false, false, true>;
/// Every kind. Custom metrics only.
pub type AnyEntity = Scope<true, true, true, true, true, true>;
