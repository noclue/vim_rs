//! Typed entity wrappers. Query builders land with the fetch implementation.

use std::marker::PhantomData;

use crate::perf::entity::{
    EntityScope, Scope, MASK_CLUSTER, MASK_DATACENTER, MASK_DATASTORE, MASK_HOST,
    MASK_RESOURCE_POOL, MASK_VM,
};
use crate::perf::error::PerfError;
use crate::types::enums::MoTypesEnum;
use crate::types::structs::ManagedObjectReference;

mod private {
    pub trait Sealed {}
}

/// A managed-object kind that can be named in a typed performance query.
///
/// Sealed. Implemented for `VirtualMachine`, `HostSystem`, `ClusterComputeResource`,
/// `ResourcePool`, `Datastore`, and `Datacenter`.
pub trait EntityKind: private::Sealed + 'static {
    /// VIM managed-object type.
    const MO_TYPE: MoTypesEnum;
    /// Scope bit from [`crate::perf::entity`].
    const BIT: u8;
}

macro_rules! entity_kind {
    ($($ty:ty, $mo:ident, $bit:expr);+ $(;)?) => {
        $(
            impl private::Sealed for $ty {}
            impl EntityKind for $ty {
                const MO_TYPE: MoTypesEnum = MoTypesEnum::$mo;
                const BIT: u8 = $bit;
            }
        )+
    };
}

entity_kind! {
    crate::mo::VirtualMachine, VirtualMachine, MASK_VM;
    crate::mo::HostSystem, HostSystem, MASK_HOST;
    crate::mo::ClusterComputeResource, ClusterComputeResource, MASK_CLUSTER;
    crate::mo::ResourcePool, ResourcePool, MASK_RESOURCE_POOL;
    crate::mo::Datastore, Datastore, MASK_DATASTORE;
    crate::mo::Datacenter, Datacenter, MASK_DATACENTER;
}

/// Implemented when scope `Self` includes entity kind `K`.
#[diagnostic::on_unimplemented(
    message = "metric scope `{Self}` does not include `{K}` in the vCenter 8.0.3 catalog",
    note = "if this counter is a Tier-3b inferred counter, call `metric.with_scope::<AnyEntity>()` or `.metric_unchecked(metric)`"
)]
pub trait Admits<K: EntityKind>: EntityScope {}

// One impl per kind. The fixed `true` is that kind's flag. The other five
// stay generic, so every combination that selects the kind is covered.
impl<const HOST: bool, const CLUSTER: bool, const RP: bool, const DS: bool, const DC: bool>
    Admits<crate::mo::VirtualMachine> for Scope<true, HOST, CLUSTER, RP, DS, DC>
{
}
impl<const VM: bool, const CLUSTER: bool, const RP: bool, const DS: bool, const DC: bool>
    Admits<crate::mo::HostSystem> for Scope<VM, true, CLUSTER, RP, DS, DC>
{
}
impl<const VM: bool, const HOST: bool, const RP: bool, const DS: bool, const DC: bool>
    Admits<crate::mo::ClusterComputeResource> for Scope<VM, HOST, true, RP, DS, DC>
{
}
impl<const VM: bool, const HOST: bool, const CLUSTER: bool, const DS: bool, const DC: bool>
    Admits<crate::mo::ResourcePool> for Scope<VM, HOST, CLUSTER, true, DS, DC>
{
}
impl<const VM: bool, const HOST: bool, const CLUSTER: bool, const RP: bool, const DC: bool>
    Admits<crate::mo::Datastore> for Scope<VM, HOST, CLUSTER, RP, true, DC>
{
}
impl<const VM: bool, const HOST: bool, const CLUSTER: bool, const RP: bool, const DS: bool>
    Admits<crate::mo::Datacenter> for Scope<VM, HOST, CLUSTER, RP, DS, true>
{
}

/// A managed object whose type is `K`.
///
/// [`TryFrom<ManagedObjectReference>`](std::convert::TryFrom) fails with
/// [`crate::perf::PerfError::EntityKindMismatch`] when `moref` is a different kind.
/// Use an untyped [`crate::perf::PerfQuery`] for `VirtualApp`.
pub struct Entity<K: EntityKind> {
    moref: ManagedObjectReference,
    _k: PhantomData<fn() -> K>,
}

impl<K: EntityKind> Entity<K> {
    /// Borrow the underlying reference.
    pub fn as_moref(&self) -> &ManagedObjectReference {
        &self.moref
    }

    /// Unwrap the reference.
    pub fn into_moref(self) -> ManagedObjectReference {
        self.moref
    }
}

impl<K: EntityKind> std::fmt::Debug for Entity<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entity")
            .field("type", &K::MO_TYPE)
            .field("value", &self.moref.value)
            .finish()
    }
}

impl<K: EntityKind> Clone for Entity<K> {
    fn clone(&self) -> Self {
        Self {
            moref: self.moref.clone(),
            _k: PhantomData,
        }
    }
}

impl<K: EntityKind> PartialEq for Entity<K> {
    fn eq(&self, other: &Self) -> bool {
        self.moref == other.moref
    }
}

impl<K: EntityKind> Eq for Entity<K> {}

impl<K: EntityKind> std::hash::Hash for Entity<K> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.moref.hash(state);
    }
}

impl<K: EntityKind> AsRef<ManagedObjectReference> for Entity<K> {
    fn as_ref(&self) -> &ManagedObjectReference {
        &self.moref
    }
}

impl<K: EntityKind> TryFrom<ManagedObjectReference> for Entity<K> {
    type Error = PerfError;

    fn try_from(moref: ManagedObjectReference) -> Result<Self, Self::Error> {
        if moref.r#type != K::MO_TYPE {
            return Err(PerfError::EntityKindMismatch {
                expected: K::MO_TYPE.clone(),
                actual: moref.r#type.clone(),
            });
        }
        Ok(Self {
            moref,
            _k: PhantomData,
        })
    }
}
