use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The *DistributedVirtualSwitchManager* provides methods
/// that support the following operations:
/// - Backup and restore operations for *VmwareDistributedVirtualSwitch*
///   and associated *DistributedVirtualPortgroup* managed objects.
/// - Query operations for information about portgroups and distributed
///   virtual switches.
/// - Distributed virtual switch configuration update operations.
#[derive(Clone)]
pub struct DistributedVirtualSwitchManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl DistributedVirtualSwitchManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// This operation returns a list of compatibility results.
    /// 
    /// Each compatibility result
    /// is an object that has a host property and optionally a fault which would
    /// be populated only if that host is not compatible with a given dvsProductSpec.
    /// All filters in hostFilerSpecs are ANDed to derive the intersection of hosts against
    /// which compatibility is checked. If caller did not have view privileges on the
    /// host entity in an element of the CompatibilityResult array, then that entire
    /// element would be removed from the CompatibilityResult array.
    /// Typical uses:
    /// - For the createDVS situation, hostFilterSpec is of type HostDvsFilterSpec and DvsProductSpec
    ///   will have newSwitchProductSpec set.
    /// - For the Add-Host-To-DVS situation, you can use either HostDvsFilterSpec or
    ///   HostDvsMembershipFilter with inclusive being false, and pass the DVS in DvsProductSpec.
    /// - For the Upgrade-DVS situation, you can use either HostDvsFilterSpec or
    ///   HostDvsMembershipFilter with inclusive being true, and pass the new desired ProductSpec
    ///   for DVS in newSwitchProductSpec.
    ///   
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### host_container
    /// The container of hosts on which we check the compatibility.
    /// This container can be a datacenter, folder, or computeResource.
    /// We can also include all the hosts in the hierarchy with container
    /// as root of the tree.
    ///
    /// ### dvs_product_spec
    /// The productSpec of a DistributedVirtualSwitch. If not
    /// set, it is assumed to be the default one used for
    /// DistributedVirtualSwitch creation for current version.
    ///
    /// ### host_filter_spec
    /// The hosts against which to check compatibility. This is a
    /// filterSpec and users can use this to specify all hosts in a
    /// container (datacenter, folder, or computeResource), an array
    /// of hosts, or hosts that might or might not be a DVS member.
    pub async fn query_dvs_check_compatibility(&self, host_container: &crate::types::structs::DistributedVirtualSwitchManagerHostContainer, dvs_product_spec: Option<&crate::types::structs::DistributedVirtualSwitchManagerDvsProductSpec>, host_filter_spec: Option<&[Box<dyn crate::types::traits::DistributedVirtualSwitchManagerHostDvsFilterSpecTrait>]>) -> Result<Option<Vec<crate::types::structs::DistributedVirtualSwitchManagerCompatibilityResult>>> {
        let input = QueryDvsCheckCompatibilityRequestType {host_container, dvs_product_spec, host_filter_spec, };
        let bytes_opt = self.client.invoke_optional("", "DistributedVirtualSwitchManager", &self.mo_id, "QueryDvsCheckCompatibility", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Export the configuration for entities specified in the
    /// <code>selectionSet</code> parameter.
    /// 
    /// You can use this method only
    /// for a *VmwareDistributedVirtualSwitch* and its
    /// associated *DistributedVirtualPortgroup* objects.
    /// 
    /// Use the *DistributedVirtualSwitchManager.DVSManagerImportEntity_Task* method to restore the entity
    /// to the state represented by the exported configuration.
    /// You can also use the exported configuration to create
    /// a new switch or portgroup.
    ///
    /// ## Parameters:
    ///
    /// ### selection_set
    /// The selection criteria for a set of
    /// entities to export the configuration.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which
    /// to monitor the operation. After successful completion, the
    /// *Task*.*Task.info*.*TaskInfo.result*
    /// property contains the *EntityBackupConfig* object.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If entity in selectionSet doesn't exist.
    /// 
    /// ***BackupBlobWriteFailure***: if failed to create backup config blob.
    pub async fn dvs_manager_export_entity_task(&self, selection_set: &[Box<dyn crate::types::traits::SelectionSetTrait>]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = DvsManagerExportEntityRequestType {selection_set, };
        let bytes = self.client.invoke("", "DistributedVirtualSwitchManager", &self.mo_id, "DVSManagerExportEntity_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Import the configuration of entities specified in
    /// *EntityBackupConfig*.
    /// 
    /// You can restore the
    /// existing configuration to the state represented by the
    /// backup configuration. You can also use the backup
    /// configuration to create a new switch or portgroup.
    /// See *EntityImportType_enum*.
    ///
    /// ## Parameters:
    ///
    /// ### entity_backup
    /// Configuration of one or more entities to be imported.
    /// The entity backup configuration is returned
    /// by the *DistributedVirtualSwitchManager.DVSManagerExportEntity_Task* method.
    ///
    /// ### import_type
    /// Specifies whether to create a new configuration
    /// or restore a previous configuration. See *EntityImportType_enum* for valid values.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which
    /// to monitor the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If entity in *EntityBackupConfig.key* doesn't exist.
    /// 
    /// ***DvsFault***: if operation fails on any host.
    pub async fn dvs_manager_import_entity_task(&self, entity_backup: &[crate::types::structs::EntityBackupConfig], import_type: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = DvsManagerImportEntityRequestType {entity_backup, import_type, };
        let bytes = self.client.invoke("", "DistributedVirtualSwitchManager", &self.mo_id, "DVSManagerImportEntity_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Returns the portgroup identified by the key within the specified VDS
    /// identified by its UUID.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### switch_uuid
    /// The UUID of the *DistributedVirtualSwitch*.
    ///
    /// ### portgroup_key
    /// The key that identifies a
    /// *DistributedVirtualPortgroup*.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *DistributedVirtualPortgroup*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If the portgroup for the specified inputs was not found.
    /// 
    /// ***NotSupported***: If the operation is not supported.
    pub async fn dvs_manager_lookup_dv_port_group(&self, switch_uuid: &str, portgroup_key: &str) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let input = DvsManagerLookupDvPortGroupRequestType {switch_uuid, portgroup_key, };
        let bytes_opt = self.client.invoke_optional("", "DistributedVirtualSwitchManager", &self.mo_id, "DVSManagerLookupDvPortGroup", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// This operation returns a list of hosts that are compatible with
    /// the given DistributedVirtualSwitch product specification.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### container
    /// Where to look for hosts. Supported types of objects for
    /// this parameter are *Datacenter*,
    /// *ComputeResource* and *Folder*.
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ### recursive
    /// Whether to search for hosts in the subfolders,
    /// if applicable. In the case when container is a *Datacenter*,
    /// the recursive flag is applied to its HostFolder.
    ///
    /// ### dvs
    /// Search the host based on the specification published in the
    /// *DVSCapability.compatibleHostComponentProductInfo*
    /// of a *DistributedVirtualSwitch*. If not
    /// set, it is assumed to be the specification that a
    /// DistributedVirtualSwitch would have if it is created
    /// with the default *DistributedVirtualSwitchProductSpec*.
    /// 
    /// Refers instance of *DistributedVirtualSwitch*.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *HostSystem*.
    pub async fn query_compatible_host_for_existing_dvs(&self, container: &crate::types::structs::ManagedObjectReference, recursive: bool, dvs: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let input = QueryCompatibleHostForExistingDvsRequestType {container, recursive, dvs, };
        let bytes_opt = self.client.invoke_optional("", "DistributedVirtualSwitchManager", &self.mo_id, "QueryCompatibleHostForExistingDvs", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// This operation returns a list of hosts that are compatible with
    /// the given DistributedVirtualSwitch product specification.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### container
    /// Where to look for hosts. Supported types of objects for
    /// this parameter are *Datacenter*,
    /// *ComputeResource* and *Folder*.
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ### recursive
    /// Whether to search for hosts in the subfolders,
    /// if applicable. In the case when container is a *Datacenter*,
    /// the recursive flag is applied to its HostFolder.
    ///
    /// ### switch_product_spec
    /// The productSpec of a *DistributedVirtualSwitch*.
    /// If not set, it is assumed to be the default one used for
    /// DistributedVirtualSwitch creation.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *HostSystem*.
    pub async fn query_compatible_host_for_new_dvs(&self, container: &crate::types::structs::ManagedObjectReference, recursive: bool, switch_product_spec: Option<&crate::types::structs::DistributedVirtualSwitchProductSpec>) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let input = QueryCompatibleHostForNewDvsRequestType {container, recursive, switch_product_spec, };
        let bytes_opt = self.client.invoke_optional("", "DistributedVirtualSwitchManager", &self.mo_id, "QueryCompatibleHostForNewDvs", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// This operation returns a list of host product specifications that
    /// are compatible with the given DistributedVirtualSwitch product
    /// specification.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### switch_product_spec
    /// The productSpec of a *DistributedVirtualSwitch*.
    /// If not set, it is assumed to be the default one used for
    /// DistributedVirtualSwitch creation.
    pub async fn query_dvs_compatible_host_spec(&self, switch_product_spec: Option<&crate::types::structs::DistributedVirtualSwitchProductSpec>) -> Result<Option<Vec<crate::types::structs::DistributedVirtualSwitchHostProductSpec>>> {
        let input = QueryDvsCompatibleHostSpecRequestType {switch_product_spec, };
        let bytes_opt = self.client.invoke_optional("", "DistributedVirtualSwitchManager", &self.mo_id, "QueryDvsCompatibleHostSpec", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// This operation returns a list of vmnics which are compatible
    /// with the given DistributedVirtualSwitch product specification.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### hosts
    /// The array of hosts on which the query is to be made to
    /// fetch valid PhysicalNics on each host.
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ### dvs
    /// The distributed virtual switch on which the query is to be
    /// made.
    /// 
    /// Refers instance of *DistributedVirtualSwitch*.
    pub async fn query_compatible_vmnics_from_hosts(&self, hosts: Option<&[crate::types::structs::ManagedObjectReference]>, dvs: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<crate::types::structs::DvsManagerPhysicalNicsList>>> {
        let input = QueryCompatibleVmnicsFromHostsRequestType {hosts, dvs, };
        let bytes_opt = self.client.invoke_optional("", "DistributedVirtualSwitchManager", &self.mo_id, "QueryCompatibleVmnicsFromHosts", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// This operation returns the DistributedVirtualSwitch or
    /// DistributedVirtualPortgroup configuration target on a host.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// The host on which the query is to be made. If called
    /// directly on the host this parameter need not be specified.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### dvs
    /// The distributed virtual switch on which the query is to be
    /// made. If unspecified the config target will encompass all the
    /// distributed virtual switches available on the host.
    /// 
    /// Refers instance of *DistributedVirtualSwitch*.
    pub async fn query_dvs_config_target(&self, host: Option<&crate::types::structs::ManagedObjectReference>, dvs: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::DvsManagerDvsConfigTarget> {
        let input = QueryDvsConfigTargetRequestType {host, dvs, };
        let bytes = self.client.invoke("", "DistributedVirtualSwitchManager", &self.mo_id, "QueryDvsConfigTarget", Some(&input)).await?;
        let result: crate::types::structs::DvsManagerDvsConfigTarget = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// This operation indicates which version-specific DVS features are
    /// available for the given DistributedVirtualSwitch product specification.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### switch_product_spec
    /// The productSpec of a *DistributedVirtualSwitch*.
    /// If not set, it is assumed to be the default one used for
    /// DistributedVirtualSwitch creation.
    pub async fn query_dvs_feature_capability(&self, switch_product_spec: Option<&crate::types::structs::DistributedVirtualSwitchProductSpec>) -> Result<Option<Box<dyn crate::types::traits::DvsFeatureCapabilityTrait>>> {
        let input = QueryDvsFeatureCapabilityRequestType {switch_product_spec, };
        let bytes_opt = self.client.invoke_optional("", "DistributedVirtualSwitchManager", &self.mo_id, "QueryDvsFeatureCapability", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// This operation returns a list of network offload specifications that are
    /// compatible with the given DistributedVirtualSwitch product specification.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### switch_product_spec
    /// The productSpec of a
    /// *DistributedVirtualSwitch*.
    pub async fn query_supported_network_offload_spec(&self, switch_product_spec: &crate::types::structs::DistributedVirtualSwitchProductSpec) -> Result<Option<Vec<crate::types::structs::DistributedVirtualSwitchNetworkOffloadSpec>>> {
        let input = QuerySupportedNetworkOffloadSpecRequestType {switch_product_spec, };
        let bytes_opt = self.client.invoke_optional("", "DistributedVirtualSwitchManager", &self.mo_id, "QuerySupportedNetworkOffloadSpec", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// This operation returns a list of switch product specifications that
    /// are supported by the vCenter Server.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### recommended
    /// If set to true, return only the recommended versions.
    /// If set to false, return only the not recommended versions.
    /// If unset, return all supported versions.
    pub async fn query_available_dvs_spec(&self, recommended: Option<bool>) -> Result<Option<Vec<crate::types::structs::DistributedVirtualSwitchProductSpec>>> {
        let input = QueryAvailableDvsSpecRequestType {recommended, };
        let bytes_opt = self.client.invoke_optional("", "DistributedVirtualSwitchManager", &self.mo_id, "QueryAvailableDvsSpec", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// This operation returns a DistributedVirtualSwitch given a UUID.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### uuid
    /// -
    ///
    /// ## Returns:
    ///
    /// Refers instance of *DistributedVirtualSwitch*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If a switch with the UUID doesn't exist.
    pub async fn query_dvs_by_uuid(&self, uuid: &str) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let input = QueryDvsByUuidRequestType {uuid, };
        let bytes_opt = self.client.invoke_optional("", "DistributedVirtualSwitchManager", &self.mo_id, "QueryDvsByUuid", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Update the Distributed Switch configuration on the hosts to bring them in sync with the
    /// current configuration in vCenter Server.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### hosts
    /// The hosts to be rectified.
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// Returns a *Task* object with which to monitor the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***DvsFault***: if operation fails on any host or if there are other update failures.
    pub async fn rectify_dvs_on_host_task(&self, hosts: &[crate::types::structs::ManagedObjectReference]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RectifyDvsOnHostRequestType {hosts, };
        let bytes = self.client.invoke("", "DistributedVirtualSwitchManager", &self.mo_id, "RectifyDvsOnHost_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct QueryDvsCheckCompatibilityRequestType<'a> {
    host_container: &'a crate::types::structs::DistributedVirtualSwitchManagerHostContainer,
    dvs_product_spec: Option<&'a crate::types::structs::DistributedVirtualSwitchManagerDvsProductSpec>,
    host_filter_spec: Option<&'a [Box<dyn crate::types::traits::DistributedVirtualSwitchManagerHostDvsFilterSpecTrait>]>,
}

impl<'a> miniserde::Serialize for QueryDvsCheckCompatibilityRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryDvsCheckCompatibilityRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryDvsCheckCompatibilityRequestTypeSer<'b, 'a> {
    data: &'b QueryDvsCheckCompatibilityRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryDvsCheckCompatibilityRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryDvsCheckCompatibilityRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("hostContainer"), &self.data.host_container as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.dvs_product_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("dvsProductSpec"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.host_filter_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("hostFilterSpec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct DvsManagerExportEntityRequestType<'a> {
    selection_set: &'a [Box<dyn crate::types::traits::SelectionSetTrait>],
}

impl<'a> miniserde::Serialize for DvsManagerExportEntityRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DvsManagerExportEntityRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DvsManagerExportEntityRequestTypeSer<'b, 'a> {
    data: &'b DvsManagerExportEntityRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DvsManagerExportEntityRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DVSManagerExportEntityRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("selectionSet"), &self.data.selection_set as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DvsManagerImportEntityRequestType<'a> {
    entity_backup: &'a [crate::types::structs::EntityBackupConfig],
    import_type: &'a str,
}

impl<'a> miniserde::Serialize for DvsManagerImportEntityRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DvsManagerImportEntityRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DvsManagerImportEntityRequestTypeSer<'b, 'a> {
    data: &'b DvsManagerImportEntityRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DvsManagerImportEntityRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DVSManagerImportEntityRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("entityBackup"), &self.data.entity_backup as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("importType"), &self.data.import_type as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DvsManagerLookupDvPortGroupRequestType<'a> {
    switch_uuid: &'a str,
    portgroup_key: &'a str,
}

impl<'a> miniserde::Serialize for DvsManagerLookupDvPortGroupRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DvsManagerLookupDvPortGroupRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DvsManagerLookupDvPortGroupRequestTypeSer<'b, 'a> {
    data: &'b DvsManagerLookupDvPortGroupRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DvsManagerLookupDvPortGroupRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DVSManagerLookupDvPortGroupRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("switchUuid"), &self.data.switch_uuid as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("portgroupKey"), &self.data.portgroup_key as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryCompatibleHostForExistingDvsRequestType<'a> {
    container: &'a crate::types::structs::ManagedObjectReference,
    recursive: bool,
    dvs: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for QueryCompatibleHostForExistingDvsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryCompatibleHostForExistingDvsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryCompatibleHostForExistingDvsRequestTypeSer<'b, 'a> {
    data: &'b QueryCompatibleHostForExistingDvsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryCompatibleHostForExistingDvsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryCompatibleHostForExistingDvsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("container"), &self.data.container as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("recursive"), &self.data.recursive as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("dvs"), &self.data.dvs as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryCompatibleHostForNewDvsRequestType<'a> {
    container: &'a crate::types::structs::ManagedObjectReference,
    recursive: bool,
    switch_product_spec: Option<&'a crate::types::structs::DistributedVirtualSwitchProductSpec>,
}

impl<'a> miniserde::Serialize for QueryCompatibleHostForNewDvsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryCompatibleHostForNewDvsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryCompatibleHostForNewDvsRequestTypeSer<'b, 'a> {
    data: &'b QueryCompatibleHostForNewDvsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryCompatibleHostForNewDvsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryCompatibleHostForNewDvsRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("container"), &self.data.container as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("recursive"), &self.data.recursive as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.switch_product_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("switchProductSpec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryDvsCompatibleHostSpecRequestType<'a> {
    switch_product_spec: Option<&'a crate::types::structs::DistributedVirtualSwitchProductSpec>,
}

impl<'a> miniserde::Serialize for QueryDvsCompatibleHostSpecRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryDvsCompatibleHostSpecRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryDvsCompatibleHostSpecRequestTypeSer<'b, 'a> {
    data: &'b QueryDvsCompatibleHostSpecRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryDvsCompatibleHostSpecRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryDvsCompatibleHostSpecRequestType")),
                1 => {
                    let Some(ref val) = self.data.switch_product_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("switchProductSpec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryCompatibleVmnicsFromHostsRequestType<'a> {
    hosts: Option<&'a [crate::types::structs::ManagedObjectReference]>,
    dvs: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for QueryCompatibleVmnicsFromHostsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryCompatibleVmnicsFromHostsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryCompatibleVmnicsFromHostsRequestTypeSer<'b, 'a> {
    data: &'b QueryCompatibleVmnicsFromHostsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryCompatibleVmnicsFromHostsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryCompatibleVmnicsFromHostsRequestType")),
                1 => {
                    let Some(ref val) = self.data.hosts else { continue; };
                    return Some((std::borrow::Cow::Borrowed("hosts"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("dvs"), &self.data.dvs as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct QueryDvsConfigTargetRequestType<'a> {
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
    dvs: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for QueryDvsConfigTargetRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryDvsConfigTargetRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryDvsConfigTargetRequestTypeSer<'b, 'a> {
    data: &'b QueryDvsConfigTargetRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryDvsConfigTargetRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryDvsConfigTargetRequestType")),
                1 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.dvs else { continue; };
                    return Some((std::borrow::Cow::Borrowed("dvs"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryDvsFeatureCapabilityRequestType<'a> {
    switch_product_spec: Option<&'a crate::types::structs::DistributedVirtualSwitchProductSpec>,
}

impl<'a> miniserde::Serialize for QueryDvsFeatureCapabilityRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryDvsFeatureCapabilityRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryDvsFeatureCapabilityRequestTypeSer<'b, 'a> {
    data: &'b QueryDvsFeatureCapabilityRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryDvsFeatureCapabilityRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryDvsFeatureCapabilityRequestType")),
                1 => {
                    let Some(ref val) = self.data.switch_product_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("switchProductSpec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QuerySupportedNetworkOffloadSpecRequestType<'a> {
    switch_product_spec: &'a crate::types::structs::DistributedVirtualSwitchProductSpec,
}

impl<'a> miniserde::Serialize for QuerySupportedNetworkOffloadSpecRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QuerySupportedNetworkOffloadSpecRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QuerySupportedNetworkOffloadSpecRequestTypeSer<'b, 'a> {
    data: &'b QuerySupportedNetworkOffloadSpecRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QuerySupportedNetworkOffloadSpecRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QuerySupportedNetworkOffloadSpecRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("switchProductSpec"), &self.data.switch_product_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryAvailableDvsSpecRequestType {
    recommended: Option<bool>,
}

impl miniserde::Serialize for QueryAvailableDvsSpecRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryAvailableDvsSpecRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryAvailableDvsSpecRequestTypeSer<'b> {
    data: &'b QueryAvailableDvsSpecRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for QueryAvailableDvsSpecRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryAvailableDvsSpecRequestType")),
                1 => {
                    let Some(ref val) = self.data.recommended else { continue; };
                    return Some((std::borrow::Cow::Borrowed("recommended"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryDvsByUuidRequestType<'a> {
    uuid: &'a str,
}

impl<'a> miniserde::Serialize for QueryDvsByUuidRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryDvsByUuidRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryDvsByUuidRequestTypeSer<'b, 'a> {
    data: &'b QueryDvsByUuidRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryDvsByUuidRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryDvsByUuidRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("uuid"), &self.data.uuid as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RectifyDvsOnHostRequestType<'a> {
    hosts: &'a [crate::types::structs::ManagedObjectReference],
}

impl<'a> miniserde::Serialize for RectifyDvsOnHostRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RectifyDvsOnHostRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RectifyDvsOnHostRequestTypeSer<'b, 'a> {
    data: &'b RectifyDvsOnHostRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RectifyDvsOnHostRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RectifyDvsOnHostRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("hosts"), &self.data.hosts as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
