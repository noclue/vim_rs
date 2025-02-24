use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::DistributedVirtualSwitchHostProductSpec;
use crate::types::structs::DistributedVirtualSwitchManagerCompatibilityResult;
use crate::types::structs::DistributedVirtualSwitchManagerDvsProductSpec;
use crate::types::structs::DistributedVirtualSwitchManagerHostContainer;
use crate::types::structs::DistributedVirtualSwitchNetworkOffloadSpec;
use crate::types::structs::DistributedVirtualSwitchProductSpec;
use crate::types::structs::DvsManagerDvsConfigTarget;
use crate::types::structs::DvsManagerPhysicalNicsList;
use crate::types::structs::EntityBackupConfig;
use crate::types::structs::ManagedObjectReference;
/// The *DistributedVirtualSwitchManager* provides methods
/// that support the following operations:
/// - Backup and restore operations for *VmwareDistributedVirtualSwitch*
///   and associated *DistributedVirtualPortgroup* managed objects.
/// - Query operations for information about portgroups and distributed
///   virtual switches.
/// - Distributed virtual switch configuration update operations.
pub struct DistributedVirtualSwitchManager {
    client: Arc<Client>,
    mo_id: String,
}
impl DistributedVirtualSwitchManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
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
    pub async fn query_dvs_check_compatibility(&self, host_container: &DistributedVirtualSwitchManagerHostContainer, dvs_product_spec: Option<&DistributedVirtualSwitchManagerDvsProductSpec>, host_filter_spec: Option<&[Box<dyn crate::types::distributed_virtual_switch_manager_host_dvs_filter_spec_trait::DistributedVirtualSwitchManagerHostDvsFilterSpecTrait>]>) -> Result<Option<Vec<DistributedVirtualSwitchManagerCompatibilityResult>>> {
        let input = QueryDvsCheckCompatibilityRequestType {host_container, dvs_product_spec, host_filter_spec, };
        let path = format!("/DistributedVirtualSwitchManager/{moId}/QueryDvsCheckCompatibility", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn dvs_manager_export_entity_task(&self, selection_set: &[Box<dyn crate::types::selection_set_trait::SelectionSetTrait>]) -> Result<ManagedObjectReference> {
        let input = DvsManagerExportEntityRequestType {selection_set, };
        let path = format!("/DistributedVirtualSwitchManager/{moId}/DVSManagerExportEntity_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn dvs_manager_import_entity_task(&self, entity_backup: &[EntityBackupConfig], import_type: &str) -> Result<ManagedObjectReference> {
        let input = DvsManagerImportEntityRequestType {entity_backup, import_type, };
        let path = format!("/DistributedVirtualSwitchManager/{moId}/DVSManagerImportEntity_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn dvs_manager_lookup_dv_port_group(&self, switch_uuid: &str, portgroup_key: &str) -> Result<Option<ManagedObjectReference>> {
        let input = DvsManagerLookupDvPortGroupRequestType {switch_uuid, portgroup_key, };
        let path = format!("/DistributedVirtualSwitchManager/{moId}/DVSManagerLookupDvPortGroup", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn query_compatible_host_for_existing_dvs(&self, container: &ManagedObjectReference, recursive: bool, dvs: &ManagedObjectReference) -> Result<Option<Vec<ManagedObjectReference>>> {
        let input = QueryCompatibleHostForExistingDvsRequestType {container, recursive, dvs, };
        let path = format!("/DistributedVirtualSwitchManager/{moId}/QueryCompatibleHostForExistingDvs", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn query_compatible_host_for_new_dvs(&self, container: &ManagedObjectReference, recursive: bool, switch_product_spec: Option<&DistributedVirtualSwitchProductSpec>) -> Result<Option<Vec<ManagedObjectReference>>> {
        let input = QueryCompatibleHostForNewDvsRequestType {container, recursive, switch_product_spec, };
        let path = format!("/DistributedVirtualSwitchManager/{moId}/QueryCompatibleHostForNewDvs", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn query_dvs_compatible_host_spec(&self, switch_product_spec: Option<&DistributedVirtualSwitchProductSpec>) -> Result<Option<Vec<DistributedVirtualSwitchHostProductSpec>>> {
        let input = QueryDvsCompatibleHostSpecRequestType {switch_product_spec, };
        let path = format!("/DistributedVirtualSwitchManager/{moId}/QueryDvsCompatibleHostSpec", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn query_compatible_vmnics_from_hosts(&self, hosts: Option<&[ManagedObjectReference]>, dvs: &ManagedObjectReference) -> Result<Option<Vec<DvsManagerPhysicalNicsList>>> {
        let input = QueryCompatibleVmnicsFromHostsRequestType {hosts, dvs, };
        let path = format!("/DistributedVirtualSwitchManager/{moId}/QueryCompatibleVmnicsFromHosts", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn query_dvs_config_target(&self, host: Option<&ManagedObjectReference>, dvs: Option<&ManagedObjectReference>) -> Result<DvsManagerDvsConfigTarget> {
        let input = QueryDvsConfigTargetRequestType {host, dvs, };
        let path = format!("/DistributedVirtualSwitchManager/{moId}/QueryDvsConfigTarget", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn query_dvs_feature_capability(&self, switch_product_spec: Option<&DistributedVirtualSwitchProductSpec>) -> Result<Option<Box<dyn crate::types::dvs_feature_capability_trait::DvsFeatureCapabilityTrait>>> {
        let input = QueryDvsFeatureCapabilityRequestType {switch_product_spec, };
        let path = format!("/DistributedVirtualSwitchManager/{moId}/QueryDvsFeatureCapability", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn query_supported_network_offload_spec(&self, switch_product_spec: &DistributedVirtualSwitchProductSpec) -> Result<Option<Vec<DistributedVirtualSwitchNetworkOffloadSpec>>> {
        let input = QuerySupportedNetworkOffloadSpecRequestType {switch_product_spec, };
        let path = format!("/DistributedVirtualSwitchManager/{moId}/QuerySupportedNetworkOffloadSpec", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn query_available_dvs_spec(&self, recommended: Option<bool>) -> Result<Option<Vec<DistributedVirtualSwitchProductSpec>>> {
        let input = QueryAvailableDvsSpecRequestType {recommended, };
        let path = format!("/DistributedVirtualSwitchManager/{moId}/QueryAvailableDvsSpec", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn query_dvs_by_uuid(&self, uuid: &str) -> Result<Option<ManagedObjectReference>> {
        let input = QueryDvsByUuidRequestType {uuid, };
        let path = format!("/DistributedVirtualSwitchManager/{moId}/QueryDvsByUuid", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn rectify_dvs_on_host_task(&self, hosts: &[ManagedObjectReference]) -> Result<ManagedObjectReference> {
        let input = RectifyDvsOnHostRequestType {hosts, };
        let path = format!("/DistributedVirtualSwitchManager/{moId}/RectifyDvsOnHost_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryDvsCheckCompatibilityRequestType<'a> {
    #[serde(rename = "hostContainer")]
    host_container: &'a DistributedVirtualSwitchManagerHostContainer,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dvsProductSpec")]
    dvs_product_spec: Option<&'a DistributedVirtualSwitchManagerDvsProductSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hostFilterSpec")]
    host_filter_spec: Option<&'a [Box<dyn crate::types::distributed_virtual_switch_manager_host_dvs_filter_spec_trait::DistributedVirtualSwitchManagerHostDvsFilterSpecTrait>]>,
}
#[derive(serde::Serialize)]
#[serde(rename = "DVSManagerExportEntityRequestType", tag = "_typeName")]
struct DvsManagerExportEntityRequestType<'a> {
    #[serde(rename = "selectionSet")]
    selection_set: &'a [Box<dyn crate::types::selection_set_trait::SelectionSetTrait>],
}
#[derive(serde::Serialize)]
#[serde(rename = "DVSManagerImportEntityRequestType", tag = "_typeName")]
struct DvsManagerImportEntityRequestType<'a> {
    #[serde(rename = "entityBackup")]
    entity_backup: &'a [EntityBackupConfig],
    #[serde(rename = "importType")]
    import_type: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "DVSManagerLookupDvPortGroupRequestType", tag = "_typeName")]
struct DvsManagerLookupDvPortGroupRequestType<'a> {
    #[serde(rename = "switchUuid")]
    switch_uuid: &'a str,
    #[serde(rename = "portgroupKey")]
    portgroup_key: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryCompatibleHostForExistingDvsRequestType<'a> {
    container: &'a ManagedObjectReference,
    recursive: bool,
    dvs: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryCompatibleHostForNewDvsRequestType<'a> {
    container: &'a ManagedObjectReference,
    recursive: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "switchProductSpec")]
    switch_product_spec: Option<&'a DistributedVirtualSwitchProductSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryDvsCompatibleHostSpecRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "switchProductSpec")]
    switch_product_spec: Option<&'a DistributedVirtualSwitchProductSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryCompatibleVmnicsFromHostsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hosts: Option<&'a [ManagedObjectReference]>,
    dvs: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryDvsConfigTargetRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    dvs: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryDvsFeatureCapabilityRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "switchProductSpec")]
    switch_product_spec: Option<&'a DistributedVirtualSwitchProductSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QuerySupportedNetworkOffloadSpecRequestType<'a> {
    #[serde(rename = "switchProductSpec")]
    switch_product_spec: &'a DistributedVirtualSwitchProductSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryAvailableDvsSpecRequestType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    recommended: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryDvsByUuidRequestType<'a> {
    uuid: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RectifyDvsOnHostRequestType<'a> {
    hosts: &'a [ManagedObjectReference],
}
