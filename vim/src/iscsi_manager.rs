use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::IscsiStatus;
use crate::types::IscsiPortInfo;
use crate::types::IscsiMigrationDependency;
/// This managed object provides interfaces for mapping VMkernel NIC to
/// iSCSI Host Bus Adapter.
pub struct IscsiManager {
    client: Arc<VimClient>,
    mo_id: String,
}
impl IscsiManager {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Bind a Virtual NIC to be used for an iSCSI adapter
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### i_scsi_hba_name
    /// iSCSI adapter name for which the Virtual NIC to
    /// be added.
    ///
    /// ### vnic_device
    /// Virtual NIC that is to be bound to the iSCSI HBA
    ///
    /// ## Errors:
    ///
    /// ***IscsiFaultVnicAlreadyBound***: The given Virtual NIC is already bound to the HBA.
    /// 
    /// ***IscsiFaultVnicHasNoUplinks***: The given Virtual NIC has no physical uplinks.
    /// 
    /// ***IscsiFaultVnicHasMultipleUplinks***: The given Virtual NIC has multiple uplinks.
    /// 
    /// ***IscsiFaultVnicHasWrongUplink***: The given Virtual NIC has the wrong uplink and
    /// it can't be used for iSCSI multi-pathing.
    /// 
    /// ***IscsiFaultVnicNotFound***: The given Virtual NIC is not present on the system.
    /// 
    /// ***IscsiFaultInvalidVnic***: The given Virtual NIC is not valid for the HBA.
    /// 
    /// ***PlatformConfigFault***: For platform error that occurs during the operation.
    /// 
    /// ***IscsiFault***: For any problem that is not handled with a more specific fault.
    /// 
    /// ***NotFound***: If the given HBA is not found
    pub async fn bind_vnic(&self, i_scsi_hba_name: &str, vnic_device: &str) -> Result<()> {
        let input = BindVnicRequestType {i_scsi_hba_name, vnic_device, };
        let path = format!("/IscsiManager/{moId}/BindVnic", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Query the list of Virtual NICs that are bound to a given iSCSI HBA.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### i_scsi_hba_name
    /// iSCSI adapter name for which the method to be
    /// applied.
    ///
    /// ## Returns:
    ///
    /// An array of *IscsiPortInfo* containing detailed
    /// information on the list of Virtual NICs bound to the adapter
    ///
    /// ## Errors:
    ///
    /// ***IscsiFault***: For any problem that is not handled with a more specific fault.
    /// 
    /// ***NotFound***: If the given HBA is not found
    pub async fn query_bound_vnics(&self, i_scsi_hba_name: &str) -> Result<Option<Vec<IscsiPortInfo>>> {
        let input = QueryBoundVnicsRequestType {i_scsi_hba_name, };
        let path = format!("/IscsiManager/{moId}/QueryBoundVnics", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Query the candidate Virtual NICs and Physical NICs that can be used
    /// for Port-Binding.
    /// 
    /// For dependent offload adapters, the Virtual NIC should be attached
    /// to the physical NIC associated with the hardware function.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### i_scsi_hba_name
    /// iSCSI Adapter name for which the method to be
    /// applied.
    ///
    /// ## Returns:
    ///
    /// Array of *IscsiPortInfo* containing detailed
    /// information on list of eligible Virtual NICs that can be bound
    /// to the adapter. This list will also include details on the
    /// eligible Physical NICs that are not associated with any
    /// Virtual NICs.
    ///
    /// ## Errors:
    ///
    /// ***IscsiFault***: For any problem that is not handled with a more specific fault.
    /// 
    /// ***NotFound***: If the given HBA is not found
    pub async fn query_candidate_nics(&self, i_scsi_hba_name: &str) -> Result<Option<Vec<IscsiPortInfo>>> {
        let input = QueryCandidateNicsRequestType {i_scsi_hba_name, };
        let path = format!("/IscsiManager/{moId}/QueryCandidateNics", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Query the dependency table for a migration operation of a given Physical
    /// NIC.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### pnic_device
    /// List of Physical NICs to be migrated
    ///
    /// ## Returns:
    ///
    /// Dependency table, as described in *IscsiMigrationDependency*,
    /// providing the user of all the Virtual NIC and iSCSI resources
    /// affected.
    pub async fn query_migration_dependencies(&self, pnic_device: &[String]) -> Result<IscsiMigrationDependency> {
        let input = QueryMigrationDependenciesRequestType {pnic_device, };
        let path = format!("/IscsiManager/{moId}/QueryMigrationDependencies", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Query if Physical NIC device is used for iSCSI.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### pnic_device
    /// Physical NIC device name to check the status for
    ///
    /// ## Returns:
    ///
    /// A status object, *IscsiStatus*, indicating
    /// whether Physical NIC is used by iSCSI or not.
    /// - Empty *IscsiStatus* (i.e reason unset)
    ///   if Physical NIC device is not used.
    /// - Fault code *IscsiFaultPnicInUse* if
    ///   Physical NIC is being used.
    ///
    /// ## Errors:
    ///
    /// ***IscsiFault***: For any problem that is not handled with a more specific fault.
    pub async fn query_pnic_status(&self, pnic_device: &str) -> Result<IscsiStatus> {
        let input = QueryPnicStatusRequestType {pnic_device, };
        let path = format!("/IscsiManager/{moId}/QueryPnicStatus", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Query the status of Virtual NIC association with the iSCSI.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### vnic_device
    /// Virtual NIC device to check the status for
    ///
    /// ## Returns:
    ///
    /// A status object *IscsiStatus*, containing
    /// list of the fault codes, providing the user with information as to
    /// whether Virtual NIC is used by iSCSI and list of compliance check
    /// failure codes if any. The returned *IscsiStatus*
    /// object will have an array of *MethodFault* objects providing
    /// following information:
    /// - Empty *IscsiStatus* (i.e reason unset)
    ///   if Virtual NIC device is not used.
    /// - Fault code *IscsiFaultVnicInUse* if Virtual
    ///   NIC is being used by iSCSI.
    /// - This will be followed with list of fault codes
    ///   corresponding to the compliance check failures.
    ///
    /// ## Errors:
    ///
    /// ***IscsiFault***: For any problem that is not handled with a more specific fault.
    pub async fn query_vnic_status(&self, vnic_device: &str) -> Result<IscsiStatus> {
        let input = QueryVnicStatusRequestType {vnic_device, };
        let path = format!("/IscsiManager/{moId}/QueryVnicStatus", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Unbind Virtual NIC binding from an iSCSI adapter.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### i_scsi_hba_name
    /// iSCSI adapter name for which the Virtual NIC to
    /// be removed.
    ///
    /// ### vnic_device
    /// Virtual NIC that is to be removed from the iSCSI HBA
    ///
    /// ### force
    /// -
    ///
    /// ## Errors:
    ///
    /// ***IscsiFaultVnicNotBound***: The given Virtual NIC is not bound to the adapter
    /// 
    /// ***IscsiFaultVnicHasActivePaths***: The given Virtual NIC is associated with "active" paths
    /// to the storage.
    /// 
    /// ***IscsiFaultVnicIsLastPath***: The given Virtual NIC is associated with "only" paths
    /// to the storage.
    /// 
    /// ***PlatformConfigFault***: For platform error that occurs during the operation.
    /// 
    /// ***IscsiFault***: For any problem that is not handled with a more specific fault.
    /// 
    /// ***NotFound***: If the given HBA is not found
    pub async fn unbind_vnic(&self, i_scsi_hba_name: &str, vnic_device: &str, force: bool) -> Result<()> {
        let input = UnbindVnicRequestType {i_scsi_hba_name, vnic_device, force, };
        let path = format!("/IscsiManager/{moId}/UnbindVnic", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct BindVnicRequestType<'a> {
    #[serde(rename = "iScsiHbaName")]
    i_scsi_hba_name: &'a str,
    #[serde(rename = "vnicDevice")]
    vnic_device: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryBoundVnicsRequestType<'a> {
    #[serde(rename = "iScsiHbaName")]
    i_scsi_hba_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryCandidateNicsRequestType<'a> {
    #[serde(rename = "iScsiHbaName")]
    i_scsi_hba_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryMigrationDependenciesRequestType<'a> {
    #[serde(rename = "pnicDevice")]
    pnic_device: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryPnicStatusRequestType<'a> {
    #[serde(rename = "pnicDevice")]
    pnic_device: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryVnicStatusRequestType<'a> {
    #[serde(rename = "vnicDevice")]
    vnic_device: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UnbindVnicRequestType<'a> {
    #[serde(rename = "iScsiHbaName")]
    i_scsi_hba_name: &'a str,
    #[serde(rename = "vnicDevice")]
    vnic_device: &'a str,
    force: bool,
}
