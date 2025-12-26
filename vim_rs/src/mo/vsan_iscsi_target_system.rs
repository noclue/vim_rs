use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The VsanIscsiTargetSystem exposes interfaces from vCenter to perform
/// vSAN iSCSI target service related operations.
/// 
/// To access these APIs from vCenter, user should possess correct privileges of
/// the target cluster: cluster edit privilege is required for the adding,
/// editing and removal of target, LUN, initiator and initiator group, while
/// system read/view privilege is required for the view of these entities.
/// The Managed Entity can be accessed through MOID of
/// vsan-cluster-iscsi-target-system, through vSAN service at both vCenter
/// server and ESXi host side.
#[derive(Clone)]
pub struct VsanIscsiTargetSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanIscsiTargetSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Adds iSCSI initiator group to one cluster.  
    /// If the initiator group already exists, a VsanFault will be raised.
    /// 
    /// Some unhealthy status of the vSAN iSCSI service can also lead to this
    /// VsanFault exception, e.g., the unavailability of the home object.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### initiator_group_name
    /// : the iSCSI initiator group name.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_add_iscsi_initiator_group(&self, cluster: &crate::types::structs::ManagedObjectReference, initiator_group_name: &str) -> Result<()> {
        let input = VsanVitAddIscsiInitiatorGroupRequestType {cluster, initiator_group_name, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitAddIscsiInitiatorGroup", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Adds iSCSI initiators to one initiator group.  
    /// If the
    /// initiator group doesn't exist, or some of the initiators
    /// are already in the specified initiator
    /// group, a VsanFault will be raised.
    /// 
    /// Some unhealthy status of the vSAN iSCSI service can also lead to this
    /// VsanFault exception, e.g., the unavailability of the home object.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### initiator_group_name
    /// : the initiator group name.
    ///
    /// ### initiator_names
    /// : the iSCSI initiator name list.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_add_iscsi_initiators_to_group(&self, cluster: &crate::types::structs::ManagedObjectReference, initiator_group_name: &str, initiator_names: &[String]) -> Result<()> {
        let input = VsanVitAddIscsiInitiatorsToGroupRequestType {cluster, initiator_group_name, initiator_names, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitAddIscsiInitiatorsToGroup", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Adds iSCSI initiators or initiator groups to one target.  
    ///   
    /// If the target cannot be found or if there are
    /// initiators or initiator groups in the
    /// initiatorNames parameter are already associated with the specified target, a
    /// VsanFault will be raised.
    /// 
    /// Some unhealthy status of the vSAN iSCSI service can also lead to this
    /// VsanFault exception, e.g., the unavailability of the home object.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### target_alias
    /// : target alias.
    ///
    /// ### initiator_names
    /// : initiator or initiator group name list.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_add_iscsi_initiators_to_target(&self, cluster: &crate::types::structs::ManagedObjectReference, target_alias: &str, initiator_names: &[String]) -> Result<()> {
        let input = VsanVitAddIscsiInitiatorsToTargetRequestType {cluster, target_alias, initiator_names, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitAddIscsiInitiatorsToTarget", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Adds iSCSI LUN to specified target.  
    /// If the target
    /// cannot be found, a VsanFault exception is raised.
    /// 
    /// If the LUN
    /// id is provided, it should be in
    /// range \[0, 255\] and be unique in the specified target. The LUN size should be
    /// provided, its minimum size is 1MB, the maximum size is 62TB. If either of
    /// them is not met, a VsanFault will be raised.
    /// Some unhealthy status of the vSAN iSCSI service can also lead to this
    /// VsanFault exception, e.g., the unavailability of the home object.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### target_alias
    /// : target alias
    ///
    /// ### lun_spec
    /// : reference to iSCSI LUN specification.
    ///
    /// ## Returns:
    ///
    /// The related task.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_add_iscsi_lun(&self, cluster: &crate::types::structs::ManagedObjectReference, target_alias: &str, lun_spec: &crate::types::structs::VsanIscsiLunSpec) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let input = VsanVitAddIscsiLunRequestType {cluster, target_alias, lun_spec, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitAddIscsiLUN", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<crate::types::structs::ManagedObjectReference>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Adds iSCSI target to this cluster.  
    /// The alias in VsanIscsiTargetSpec
    /// must be provided and unique.
    /// 
    /// If iqn provided, it should be unique, if either
    /// of them is not met, a VsanFault will be raised.
    /// Some unhealthy status of the vSAN iSCSI service can also lead to this
    /// VsanFault exception, e.g., the unavailability of the home object.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### target_spec
    /// : reference to iSCSI target specification.
    ///
    /// ## Returns:
    ///
    /// The related task.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_add_iscsi_target(&self, cluster: &crate::types::structs::ManagedObjectReference, target_spec: &crate::types::structs::VsanIscsiTargetSpec) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let input = VsanVitAddIscsiTargetRequestType {cluster, target_spec, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitAddIscsiTarget", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<crate::types::structs::ManagedObjectReference>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Adds the iSCSI target to the specified initiator group.  
    /// If the initiator group is not found or the target is already in the
    /// accessible targets of the group, a VsanFault will be raised.
    /// 
    /// Some unhealthy status of the vSAN iSCSI service can also lead to this
    /// VsanFault exception, e.g., the unavailability of the home object.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### initiator_group_name
    /// : the initiator group name.
    ///
    /// ### target_alias
    /// : the iSCSI target alias.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_add_iscsi_target_to_group(&self, cluster: &crate::types::structs::ManagedObjectReference, initiator_group_name: &str, target_alias: &str) -> Result<()> {
        let input = VsanVitAddIscsiTargetToGroupRequestType {cluster, initiator_group_name, target_alias, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitAddIscsiTargetToGroup", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Edits iSCSI LUN in specified target.  
    /// All properties in
    /// <code>VsanIscsiLUNSpec</code> can be changed.
    /// 
    /// If new LUN id is provided, it
    /// should be in range \[0, 255\] and be unique in the specified target, if lunSize
    /// is provided, it should be greater than existing size, i.e., LUNs are only
    /// allowed to grow, if either of them are not met, a VsanFault exception will be
    /// raised. If storagePolicy is provided, it is set by calling
    /// <code>SetVsanObjectPolicy</code> in <code>VsanObjectSystem</code>.
    /// Some unhealthy status of the vSAN iSCSI service can also lead to this
    /// VsanFault exception, e.g., the unavailability of the home object.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### target_alias
    /// : iSCSI target alias.
    ///
    /// ### lun_spec
    /// : reference to iSCSI LUN specification.
    ///
    /// ## Returns:
    ///
    /// The related task.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_edit_iscsi_lun(&self, cluster: &crate::types::structs::ManagedObjectReference, target_alias: &str, lun_spec: &crate::types::structs::VsanIscsiLunSpec) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let input = VsanVitEditIscsiLunRequestType {cluster, target_alias, lun_spec, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitEditIscsiLUN", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<crate::types::structs::ManagedObjectReference>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Edits iSCSI target in this cluster.  
    /// All properties in
    /// VsanIscsiTargetSpec can be changed.
    /// 
    /// If authType changed, all existing
    /// connections are not effected, but the new connection to this target need to
    /// use the new authentication type, if port or network interface changed, it has
    /// the same impact to connection as authType change. If IQN or alias provided,
    /// it should be unique.  
    /// If IQN or alias is not unique, a VsanFault will be raised.
    /// Some unhealthy status of the vSAN iSCSI service can also lead to this
    /// VsanFault exception, e.g., the unavailability of the home object.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### target_spec
    /// : reference to iSCSI target specification.
    ///
    /// ## Returns:
    ///
    /// The related task.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_edit_iscsi_target(&self, cluster: &crate::types::structs::ManagedObjectReference, target_spec: &crate::types::structs::VsanIscsiTargetSpec) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let input = VsanVitEditIscsiTargetRequestType {cluster, target_spec, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitEditIscsiTarget", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<crate::types::structs::ManagedObjectReference>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Gets the home object which stores the metadata for
    /// vSAN iSCSI target service.
    /// 
    /// If the home object is not
    /// found, a VsanFault will be raised.  
    /// Please don't rely on this function to decide whether need
    /// to create the home object or not, because the home object
    /// may be in creation process.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// VsanObjectInformation for the home object.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_get_home_object(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::VsanObjectInformation> {
        let input = VsanVitGetHomeObjectRequestType {cluster, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitGetHomeObject", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::VsanObjectInformation = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Gets one iSCSI initiator group.  
    /// If the initiator group doesn't exists, a VsanFault will be raised.
    /// 
    /// Some unhealthy status of the vSAN iSCSI service can also lead to this
    /// VsanFault exception, e.g., the unavailability of the home object.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### initiator_group_name
    /// : the initiator group name.
    ///
    /// ## Returns:
    ///
    /// iSCSI initiator group.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_get_iscsi_initiator_group(&self, cluster: &crate::types::structs::ManagedObjectReference, initiator_group_name: &str) -> Result<Option<crate::types::structs::VsanIscsiInitiatorGroup>> {
        let input = VsanVitGetIscsiInitiatorGroupRequestType {cluster, initiator_group_name, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitGetIscsiInitiatorGroup", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<crate::types::structs::VsanIscsiInitiatorGroup>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Gets iSCSI initiator group list for one cluster.  
    /// If the operation fails, a VsanFault will be raised.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// iSCSI initiator group list.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_get_iscsi_initiator_groups(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<crate::types::structs::VsanIscsiInitiatorGroup>>> {
        let input = VsanVitGetIscsiInitiatorGroupsRequestType {cluster, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitGetIscsiInitiatorGroups", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::VsanIscsiInitiatorGroup>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Gets one iSCSI LUN for specified target.  
    /// If the target or the LUN cannot be found, a VsanFault will be raised.
    /// 
    /// Some unhealthy status of the vSAN iSCSI service can also lead to this
    /// VsanFault exception, e.g., the unavailability of the home object.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### target_alias
    /// : the target alias.
    ///
    /// ### lun_id
    /// : the LUN id.
    ///
    /// ## Returns:
    ///
    /// iSCSI LUN.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_get_iscsi_lun(&self, cluster: &crate::types::structs::ManagedObjectReference, target_alias: &str, lun_id: i32) -> Result<Option<crate::types::structs::VsanIscsiLun>> {
        let input = VsanVitGetIscsiLunRequestType {cluster, target_alias, lun_id, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitGetIscsiLUN", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<crate::types::structs::VsanIscsiLun>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Gets iSCSI LUN list for specified target list.  
    /// If some targets cannot be found, a VsanFault will be raised.
    /// 
    /// Some unhealthy status of the vSAN iSCSI service can also lead to this
    /// VsanFault exception, e.g., the unavailability of the home object.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### target_aliases
    /// : target alias list, if not specified, all targets are
    /// used here.
    ///
    /// ## Returns:
    ///
    /// iSCSI LUN list, it may be empty.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_get_iscsi_lu_ns(&self, cluster: &crate::types::structs::ManagedObjectReference, target_aliases: Option<&[String]>) -> Result<Option<Vec<crate::types::structs::VsanIscsiLun>>> {
        let input = VsanVitGetIscsiLuNsRequestType {cluster, target_aliases, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitGetIscsiLUNs", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::VsanIscsiLun>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Gets one iSCSI target.
    /// 
    ///   
    /// If the target cannot be found, a VsanFault will be raised.
    /// Some unhealthy status of the vSAN iSCSI service can also lead to this
    /// VsanFault exception, e.g., the unavailability of the home object.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### target_alias
    /// : target alias.
    ///
    /// ## Returns:
    ///
    /// iSCSI target detail.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_get_iscsi_target(&self, cluster: &crate::types::structs::ManagedObjectReference, target_alias: &str) -> Result<Option<crate::types::structs::VsanIscsiTarget>> {
        let input = VsanVitGetIscsiTargetRequestType {cluster, target_alias, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitGetIscsiTarget", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<crate::types::structs::VsanIscsiTarget>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Gets iSCSI target list for one cluster.
    /// 
    ///   
    /// If the operation fails, a VsanFault will be raised.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// iSCSI target list, it may be empty.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_get_iscsi_targets(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<crate::types::structs::VsanIscsiTarget>>> {
        let input = VsanVitGetIscsiTargetsRequestType {cluster, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitGetIscsiTargets", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::VsanIscsiTarget>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Queries iSCSI service version, if version file is not found, a NotFound
    /// exception is raised.
    /// 
    /// When it is invoked from an ESXi host on MO
    /// 'vsan-cluster-iscsi-target-system', it just returns the iSCSI version on
    /// this host.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_query_iscsi_target_service_version(&self) -> Result<String> {
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitQueryIscsiTargetServiceVersion", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: String = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Remediates the iSCSI LUNs which runtime status are not consistent
    /// with configuration.  
    /// The remediation result will be returned after the task is finished.  
    /// Only if the remediation succeed for all LUNs, the task state will be
    /// "success".
    /// 
    /// If the remediation fails with no exception raised, the task state will
    /// be "failure".
    /// If hit internal vSAN error, the task state will be "error" with error
    /// message in the fault field.
    /// Unhealthy status of the vSAN iSCSI service can lead to this
    /// vim.fault.VsanFault exception, e.g., the unavailability of the home object.
    /// Multiple entry of this function at the same time is not allowed.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// , reference to cluster resource.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// task.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_remediate_iscsi_luns_runtime_status(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanRemediateIscsiLunsRuntimeStatusRequestType {cluster, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanRemediateIscsiLunsRuntimeStatus", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::ManagedObjectReference = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Removes iSCSI initiator group from one cluster.  
    /// If there are initiators in the group, the initiators will be
    /// removed at the same time.
    /// 
    /// If the initiator
    /// group doesn't exist, a VsanFault will be raised.
    /// Some unhealthy status of the vSAN iSCSI service can also lead to this
    /// VsanFault exception, e.g., the unavailability of the home object.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### initiator_group_name
    /// : the initiator group name.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_remove_iscsi_initiator_group(&self, cluster: &crate::types::structs::ManagedObjectReference, initiator_group_name: &str) -> Result<()> {
        let input = VsanVitRemoveIscsiInitiatorGroupRequestType {cluster, initiator_group_name, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitRemoveIscsiInitiatorGroup", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Removes iSCSI initiators from one initiator group.  
    /// If the initiator group does not exist, or some of the initiators
    /// are not in the specified initiator group, a VsanFault will be raised.
    /// 
    /// Some unhealthy status of the vSAN iSCSI service can also lead to this
    /// VsanFault exception, e.g., the unavailability of the home object.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### initiator_group_name
    /// : the initiator group name.
    ///
    /// ### initiator_names
    /// : the initiator name list.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_remove_iscsi_initiators_from_group(&self, cluster: &crate::types::structs::ManagedObjectReference, initiator_group_name: &str, initiator_names: &[String]) -> Result<()> {
        let input = VsanVitRemoveIscsiInitiatorsFromGroupRequestType {cluster, initiator_group_name, initiator_names, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitRemoveIscsiInitiatorsFromGroup", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Removes iSCSI initiator or initiator groups from one target.  
    /// If the specified target cannot be found or if the initiators or initiator
    /// groups specified by the initiatorNames cannot be associated with the
    /// specified target, a VsanFault will be raised.
    /// 
    /// Some unhealthy status of the vSAN iSCSI service can also lead to this
    /// VsanFault exception, e.g., the unavailability of the home object.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### target_alias
    /// : target alias.
    ///
    /// ### initiator_names
    /// : initiator or initiator group name list.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_remove_iscsi_initiators_from_target(&self, cluster: &crate::types::structs::ManagedObjectReference, target_alias: &str, initiator_names: &[String]) -> Result<()> {
        let input = VsanVitRemoveIscsiInitiatorsFromTargetRequestType {cluster, target_alias, initiator_names, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitRemoveIscsiInitiatorsFromTarget", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Removes iSCSI LUN from this target.  
    /// If
    /// the target or the LUN cannot be
    /// found, a VsanFault will be raised.
    /// 
    /// Some unhealthy status of the vSAN iSCSI service can also lead to this
    /// VsanFault exception, e.g., the unavailability of the home object.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### target_alias
    /// : iSCSI target alias.
    ///
    /// ### lun_id
    /// : iSCSI LUN id.
    ///
    /// ## Returns:
    ///
    /// The related task.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_remove_iscsi_lun(&self, cluster: &crate::types::structs::ManagedObjectReference, target_alias: &str, lun_id: i32) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let input = VsanVitRemoveIscsiLunRequestType {cluster, target_alias, lun_id, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitRemoveIscsiLUN", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<crate::types::structs::ManagedObjectReference>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Removes iSCSI target from this cluster.  
    /// If the target cannot be found, a VsanFault will be raised.
    /// 
    /// Some unhealthy status of the vSAN iSCSI service can also lead to this
    /// VsanFault exception, e.g., the unavailability of the home object.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### target_alias
    /// : the target alias
    ///
    /// ## Returns:
    ///
    /// The related task.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_remove_iscsi_target(&self, cluster: &crate::types::structs::ManagedObjectReference, target_alias: &str) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let input = VsanVitRemoveIscsiTargetRequestType {cluster, target_alias, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitRemoveIscsiTarget", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<crate::types::structs::ManagedObjectReference>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Removes accessible iSCSI target from one initiator group.  
    /// If the initiator group is not found or the target is not in the accessible
    /// targets of the group, a VsanFault will be raised.
    /// 
    /// Some unhealthy status of the vSAN iSCSI service can also lead to this
    /// VsanFault exception, e.g., the unavailability of the home object.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// : reference to cluster resource.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### initiator_group_name
    /// : the initiator group name.
    ///
    /// ### target_alias
    /// : the iSCSI target alias.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vit_remove_iscsi_target_from_group(&self, cluster: &crate::types::structs::ManagedObjectReference, initiator_group_name: &str, target_alias: &str) -> Result<()> {
        let input = VsanVitRemoveIscsiTargetFromGroupRequestType {cluster, initiator_group_name, target_alias, };
        let path = format!("/vsan/VsanIscsiTargetSystem/{moId}/VsanVitRemoveIscsiTargetFromGroup", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVitAddIscsiInitiatorGroupRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "initiatorGroupName")]
    initiator_group_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVitAddIscsiInitiatorsToGroupRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "initiatorGroupName")]
    initiator_group_name: &'a str,
    #[serde(rename = "initiatorNames")]
    initiator_names: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVitAddIscsiInitiatorsToTargetRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "targetAlias")]
    target_alias: &'a str,
    #[serde(rename = "initiatorNames")]
    initiator_names: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(rename = "VsanVitAddIscsiLUNRequestType", tag = "_typeName")]
struct VsanVitAddIscsiLunRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "targetAlias")]
    target_alias: &'a str,
    #[serde(rename = "lunSpec")]
    lun_spec: &'a crate::types::structs::VsanIscsiLunSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVitAddIscsiTargetRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "targetSpec")]
    target_spec: &'a crate::types::structs::VsanIscsiTargetSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVitAddIscsiTargetToGroupRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "initiatorGroupName")]
    initiator_group_name: &'a str,
    #[serde(rename = "targetAlias")]
    target_alias: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "VsanVitEditIscsiLUNRequestType", tag = "_typeName")]
struct VsanVitEditIscsiLunRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "targetAlias")]
    target_alias: &'a str,
    #[serde(rename = "lunSpec")]
    lun_spec: &'a crate::types::structs::VsanIscsiLunSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVitEditIscsiTargetRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "targetSpec")]
    target_spec: &'a crate::types::structs::VsanIscsiTargetSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVitGetHomeObjectRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVitGetIscsiInitiatorGroupRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "initiatorGroupName")]
    initiator_group_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVitGetIscsiInitiatorGroupsRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(rename = "VsanVitGetIscsiLUNRequestType", tag = "_typeName")]
struct VsanVitGetIscsiLunRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "targetAlias")]
    target_alias: &'a str,
    #[serde(rename = "lunId")]
    lun_id: i32,
}
#[derive(serde::Serialize)]
#[serde(rename = "VsanVitGetIscsiLUNsRequestType", tag = "_typeName")]
struct VsanVitGetIscsiLuNsRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetAliases")]
    target_aliases: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVitGetIscsiTargetRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "targetAlias")]
    target_alias: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVitGetIscsiTargetsRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanRemediateIscsiLunsRuntimeStatusRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVitRemoveIscsiInitiatorGroupRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "initiatorGroupName")]
    initiator_group_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVitRemoveIscsiInitiatorsFromGroupRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "initiatorGroupName")]
    initiator_group_name: &'a str,
    #[serde(rename = "initiatorNames")]
    initiator_names: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVitRemoveIscsiInitiatorsFromTargetRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "targetAlias")]
    target_alias: &'a str,
    #[serde(rename = "initiatorNames")]
    initiator_names: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(rename = "VsanVitRemoveIscsiLUNRequestType", tag = "_typeName")]
struct VsanVitRemoveIscsiLunRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "targetAlias")]
    target_alias: &'a str,
    #[serde(rename = "lunId")]
    lun_id: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVitRemoveIscsiTargetRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "targetAlias")]
    target_alias: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVitRemoveIscsiTargetFromGroupRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "initiatorGroupName")]
    initiator_group_name: &'a str,
    #[serde(rename = "targetAlias")]
    target_alias: &'a str,
}
