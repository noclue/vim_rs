use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *DVSFeatureCapability* data object
/// represents the capabilities supported by a
/// *DistributedVirtualSwitch*.
/// 
/// These properties are read-only with
/// the exception of
/// *DVSFeatureCapability.vmDirectPathGen2Supported*.
pub trait DvsFeatureCapabilityTrait : super::data_object_trait::DataObjectTrait {
    /// Deprecated as of vSphere API 5.0, use
    /// <code>networkResourceManagementCapability</code>.*DVSNetworkResourceManagementCapability.networkResourceManagementSupported*.
    /// 
    /// Indicates whether network I/O control is
    /// supported on the vSphere Distributed Switch.
    fn get_network_resource_management_supported(&self) -> bool;
    /// Deprecated as of vSphere API 8.0. VMDirectPath Gen 2 is no longer supported and
    /// there is no replacement.
    /// 
    /// Indicates whether VMDirectPath Gen 2 is supported on the
    /// distributed virtual switch.
    /// 
    /// See
    /// *HostCapability*.*HostCapability.vmDirectPathGen2Supported*
    /// and *PhysicalNic*.*PhysicalNic.vmDirectPathGen2Supported*.
    /// 
    /// For a third-party distributed switch implementation, you can
    /// specify this property during switch creation or when you call the
    /// *DistributedVirtualSwitch.UpdateDvsCapability* method.
    /// 
    /// VMDirectPath Gen 2 is supported in
    /// vSphere Distributed Switch Version 4.1 or later.
    fn get_vm_direct_path_gen_2_supported(&self) -> Option<bool>;
    /// The available teaming modes for the vSphere Distributed Switch.
    /// 
    /// The
    /// value can be one or more of
    /// *DistributedVirtualSwitchNicTeamingPolicyMode_enum*.
    fn get_nic_teaming_policy(&self) -> &Option<Vec<String>>;
    /// Deprecated as of vSphere API 5.0, use
    /// <code>networkResourceManagementCapability</code>.*DVSNetworkResourceManagementCapability.networkResourcePoolHighShareValue*.
    /// 
    /// This is the value for *high*
    /// in *DVSNetworkResourcePoolAllocationInfo.shares*.
    /// 
    /// This
    /// implicitly defines the legal range of share values to be between 1 and this.
    /// This also defines values for other level types, such as
    /// *normal* being one half of this value and
    /// *low* being one fourth of this value.
    fn get_network_resource_pool_high_share_value(&self) -> Option<i32>;
    /// Network resource management capabilities supported by a
    /// distributed virtual switch.
    fn get_network_resource_management_capability(&self) -> &Option<DvsNetworkResourceManagementCapability>;
    /// Health check capabilities supported by a *VmwareDistributedVirtualSwitch*.
    fn get_health_check_capability(&self) -> &Option<Box<dyn super::dvs_health_check_capability_trait::DvsHealthCheckCapabilityTrait>>;
    /// Host rollback capability.
    /// 
    /// If <code>rollbackCapability</code>.*DVSRollbackCapability.rollbackSupported*
    /// is true, network operations that disconnect the the host are rolled back.
    fn get_rollback_capability(&self) -> &Option<DvsRollbackCapability>;
    /// Backup, restore, and rollback capabilities.
    /// 
    /// Backup and restore
    /// are supported only for *VmwareDistributedVirtualSwitch*.
    /// Rollback is supported for *VmwareDistributedVirtualSwitch*
    /// and *DistributedVirtualPortgroup*.
    /// For information about backup and restore, see the
    /// *DistributedVirtualSwitchManager* methods
    /// *DistributedVirtualSwitchManager.DVSManagerExportEntity_Task* and
    /// *DistributedVirtualSwitchManager.DVSManagerImportEntity_Task*.
    /// For information about rollback, see the
    /// *DistributedVirtualSwitch*.*DistributedVirtualSwitch.DVSRollback_Task*
    /// and *DistributedVirtualPortgroup*.*DistributedVirtualPortgroup.DVPortgroupRollback_Task*
    /// methods.
    fn get_backup_restore_capability(&self) -> &Option<DvsBackupRestoreCapability>;
    /// Indicates whether Network Filter feature is
    /// supported in vSphere Distributed Switch.
    fn get_network_filter_supported(&self) -> Option<bool>;
    /// Indicates whether MAC learning feature is
    /// supported in vSphere Distributed Switch.
    fn get_mac_learning_supported(&self) -> Option<bool>;
}
impl<'s> serde::Serialize for dyn DvsFeatureCapabilityTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DvsFeatureCapabilityTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DvsFeatureCapabilityVisitor)
            }
        }

struct DvsFeatureCapabilityVisitor;

impl<'de> de::Visitor<'de> for DvsFeatureCapabilityVisitor {
    type Value = Box<dyn DvsFeatureCapabilityTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DvsFeatureCapabilityTrait JSON object with a _typeName field")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let deserializer = de::value::MapAccessDeserializer::new(&mut map);
        let any: VimAny = de::Deserialize::deserialize(deserializer)?;
        match any {
            VimAny::Object(obj) => Ok(CastFrom::from_box(obj)
                .map_err(|_| de::Error::custom("Internal error converting to trait type"))?),
            VimAny::Value(value) => Err(de::Error::custom(format!(
                "expected object not wrapped value: {:?}",
                value))),
        }
    }
}

impl DvsFeatureCapabilityTrait for DvsFeatureCapability {
    fn get_network_resource_management_supported(&self) -> bool { self.network_resource_management_supported }
    fn get_vm_direct_path_gen_2_supported(&self) -> Option<bool> { self.vm_direct_path_gen_2_supported }
    fn get_nic_teaming_policy(&self) -> &Option<Vec<String>> { &self.nic_teaming_policy }
    fn get_network_resource_pool_high_share_value(&self) -> Option<i32> { self.network_resource_pool_high_share_value }
    fn get_network_resource_management_capability(&self) -> &Option<DvsNetworkResourceManagementCapability> { &self.network_resource_management_capability }
    fn get_health_check_capability(&self) -> &Option<Box<dyn super::dvs_health_check_capability_trait::DvsHealthCheckCapabilityTrait>> { &self.health_check_capability }
    fn get_rollback_capability(&self) -> &Option<DvsRollbackCapability> { &self.rollback_capability }
    fn get_backup_restore_capability(&self) -> &Option<DvsBackupRestoreCapability> { &self.backup_restore_capability }
    fn get_network_filter_supported(&self) -> Option<bool> { self.network_filter_supported }
    fn get_mac_learning_supported(&self) -> Option<bool> { self.mac_learning_supported }
}
impl DvsFeatureCapabilityTrait for VMwareDvsFeatureCapability {
    fn get_network_resource_management_supported(&self) -> bool { self.network_resource_management_supported }
    fn get_vm_direct_path_gen_2_supported(&self) -> Option<bool> { self.vm_direct_path_gen_2_supported }
    fn get_nic_teaming_policy(&self) -> &Option<Vec<String>> { &self.nic_teaming_policy }
    fn get_network_resource_pool_high_share_value(&self) -> Option<i32> { self.network_resource_pool_high_share_value }
    fn get_network_resource_management_capability(&self) -> &Option<DvsNetworkResourceManagementCapability> { &self.network_resource_management_capability }
    fn get_health_check_capability(&self) -> &Option<Box<dyn super::dvs_health_check_capability_trait::DvsHealthCheckCapabilityTrait>> { &self.health_check_capability }
    fn get_rollback_capability(&self) -> &Option<DvsRollbackCapability> { &self.rollback_capability }
    fn get_backup_restore_capability(&self) -> &Option<DvsBackupRestoreCapability> { &self.backup_restore_capability }
    fn get_network_filter_supported(&self) -> Option<bool> { self.network_filter_supported }
    fn get_mac_learning_supported(&self) -> Option<bool> { self.mac_learning_supported }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DvsFeatureCapabilityTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsFeatureCapability => Some(from.as_any_ref().downcast_ref::<DvsFeatureCapability>()?),
            StructType::VMwareDvsFeatureCapability => Some(from.as_any_ref().downcast_ref::<VMwareDvsFeatureCapability>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsFeatureCapability => Ok(from.as_any_box().downcast::<DvsFeatureCapability>()?),
            StructType::VMwareDvsFeatureCapability => Ok(from.as_any_box().downcast::<VMwareDvsFeatureCapability>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
