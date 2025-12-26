use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The *PbmComplianceManager* provides methods to verify the compliance
/// of virtual machine and virtual disk requirement profiles.
/// 
/// When you provision
/// a virtual machine on a matching datastore, the Server sends the profile
/// to the storage provider. When you perform a compliance check, the storage
/// provider compares the requirements with its capabilities,
/// returns the results to the Server, and the Server returns the results
/// to your client. The Server maintains the compliance results for retrieval
/// at a later time.
/// 
/// You can check the compliance of one or more virtual machines and/or virtual disks.
/// You can also perform a rollup compliance check, in which the Server checks the
/// compliance of a virtual machine and all of its virtual disks.
#[derive(Clone)]
pub struct PbmComplianceManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl PbmComplianceManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Checks compliance of the profiles associated with one or more
    /// virtual machines and/or virtual disks.
    /// 
    /// The Server stores the compliance results for all of the storage entities
    /// associated with the virtual machines and disks. You can call the
    /// *PbmComplianceManager.PbmFetchComplianceResult* method
    /// to retrieve the stored results. However, for storage entities placed on vSAN,
    /// both fetchComplianceResult and checkCompliance methods have the same
    /// behaviour of recomputing the compliance.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### entities
    /// One or more references to storage entities.
    /// You can specify virtual machines and virtual disks
    /// A maximum of 1000 virtual machines and/or virtual disks can be specified
    /// in a call. The results of calling the checkCompliance API with
    /// more than a 1000 entities is undefined.
    /// - If the list of entities also contains datastores, the Server
    ///   will ignore the datastores.
    /// - If the list contains valid and invalid entities, the Server ignores
    ///   the invalid entities and returns results for the valid entities.
    ///   Invalid entities are entities that are not in the vCenter inventory.
    /// - If the list contains only datastores, the method throws
    ///   an <code>InvalidArgument</code> fault.
    /// - If the list contains virtual machines and disks and the entities
    ///   are invalid or have been deleted by the time of the request, the method
    ///   throws an <code>InvalidArgument</code> fault.
    ///   
    /// If an entity does not have an associated storage profile, the entity
    /// is removed from the list.
    ///
    /// ### profile
    /// Not used. If specified, the Server ignores the value.
    /// The Server uses the profiles associated with the specified entities.
    ///
    /// ## Returns:
    ///
    /// Result of the compliance check. The returned array contains one
    /// result object for each entity specified in the method call.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If one of the following situations occurs:
    /// - You do not specify an entity.
    /// - You specify only datastores.
    /// - All of the specified storage entities are invalid.
    /// 
    /// ***PbmFault***: If there is an internal server error.
    pub async fn pbm_check_compliance(&self, entities: &[crate::types::structs::PbmServerObjectRef], profile: Option<&crate::types::structs::PbmProfileId>) -> Result<Option<Vec<crate::types::structs::PbmComplianceResult>>> {
        let input = PbmCheckComplianceRequestType {entities, profile, };
        let path = format!("/pbm/PbmComplianceManager/{moId}/PbmCheckCompliance", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::PbmComplianceResult>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Checks rollup compliance of virtual machines and returns the results to your
    /// client.
    /// 
    /// For a specified virtual machine, a rollup compliance check verifies
    /// the storage requirements of the virtual machine and its virtual disks as
    /// compared with the storage provider capabilities.
    /// 
    /// The Server stores the compliance results for all of the storage entities
    /// associated with the virtual machines. You can call the
    /// *PbmComplianceManager.PbmFetchRollupComplianceResult* method
    /// to retrieve the stored results. However, for storage entities placed on vSAN,
    /// both fetchRollupComplianceResult and checkRollupCompliance methods have the
    /// same behaviour of recomputing the compliance.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// One or more references to virtual machines.
    /// A maximum of 1000 virtual machines can be specified
    /// in a call. The results of calling the checkRollupCompliance API with
    /// more than a 1000 entities is undefined.
    ///
    /// ## Returns:
    ///
    /// Result of the rollup compliance check. The returned array
    /// contains one rollup compliance result for each virtual machine.
    /// A rollup compliance result object includes the overall compliance
    /// status that represents the collective compliance status for the
    /// virtual machine and its virtual disks.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If one of the following situations occurs:
    /// - You do not specify any entities.
    /// - You specify only datastores or virtual disks.
    /// - All of the specified virtual machines are invalid.
    /// 
    /// ***PbmFault***: If there is an internal server error.
    pub async fn pbm_check_rollup_compliance(&self, entity: &[crate::types::structs::PbmServerObjectRef]) -> Result<Option<Vec<crate::types::structs::PbmRollupComplianceResult>>> {
        let input = PbmCheckRollupComplianceRequestType {entity, };
        let path = format!("/pbm/PbmComplianceManager/{moId}/PbmCheckRollupCompliance", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::PbmRollupComplianceResult>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Retrieves the latest version of *PbmComplianceResult* objects that are
    /// available for the specified entities.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### entities
    /// One or more references to storage entities.
    /// A maximum of 1000 virtual machines and/or virtual disks can be specified
    /// in a call. The results of calling the fetchComplianceResult API with
    /// more than a 1000 entities is undefined.
    /// - If the list of entities also contains datastores, the Server
    ///   will ignore the datastores.
    /// - If the list contains valid and invalid entities, the Server ignores
    ///   the invalid entities and returns results for the valid entities.
    ///   Invalid entities are entities that are not in the vCenter inventory.
    /// - If the list contains only datastores, the method throws
    ///   an <code>InvalidArgument</code> fault.
    ///
    /// ### profile
    /// Not used. if specified, the Server ignores the value.
    /// The Server uses the profiles associated with the specified entities.
    ///
    /// ## Returns:
    ///
    /// Array of compliance results. The returned array contains
    /// one result object for each entity specified in the method call.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If one of the following situations occurs:
    /// - You do not specify an entity.
    /// - You specify only datastores.
    /// - All of the specified storage entities are invalid.
    /// 
    /// ***PbmFault***: If there is an internal server error.
    pub async fn pbm_fetch_compliance_result(&self, entities: &[crate::types::structs::PbmServerObjectRef], profile: Option<&crate::types::structs::PbmProfileId>) -> Result<Option<Vec<crate::types::structs::PbmComplianceResult>>> {
        let input = PbmFetchComplianceResultRequestType {entities, profile, };
        let path = format!("/pbm/PbmComplianceManager/{moId}/PbmFetchComplianceResult", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::PbmComplianceResult>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Retrieves the rollup compliance (*PbmRollupComplianceResult*)
    /// of the given virtual machines if present.
    /// 
    /// The returned rollup compliance
    /// result may be old. Invoke checkRollupCompliance API to compute and retrieve the
    /// latest rollup compliance result. For storage entities placed on vSAN the
    /// returned compliance is always the latest.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// One or more virtual machines.
    /// A maximum of 1000 virtual machines can be specified
    /// in a call. The results of calling the fetchRollupComplianceResult API with
    /// more than a 1000 entity objects is undefined.
    ///
    /// ## Returns:
    ///
    /// Rollup compliance results for the given virtual machines.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If one of the following situations occurs:
    /// - There is no profile associated with the virtual machine
    ///   or its virtual disks.
    /// - You specify only datastores or virtual disks.
    /// - All of the specified virtual machines are invalid.
    /// 
    /// ***PbmFault***: If there is an internal server error.
    pub async fn pbm_fetch_rollup_compliance_result(&self, entity: &[crate::types::structs::PbmServerObjectRef]) -> Result<Option<Vec<crate::types::structs::PbmRollupComplianceResult>>> {
        let input = PbmFetchRollupComplianceResultRequestType {entity, };
        let path = format!("/pbm/PbmComplianceManager/{moId}/PbmFetchRollupComplianceResult", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::PbmRollupComplianceResult>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Returns the virtual machines for the given rollup compliance status.
    /// 
    /// ***Required privileges:*** StorageProfile.View
    ///
    /// ## Parameters:
    ///
    /// ### status
    /// *PbmComplianceStatus_enum*
    ///
    /// ## Returns:
    ///
    /// Array of VirtualMachine entities
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If the given status parameter is invalid or incorrect format.
    /// 
    /// ***PbmFault***: If there is an internal service error.
    pub async fn pbm_query_by_rollup_compliance_status(&self, status: &str) -> Result<Option<Vec<crate::types::structs::PbmServerObjectRef>>> {
        let input = PbmQueryByRollupComplianceStatusRequestType {status, };
        let path = format!("/pbm/PbmComplianceManager/{moId}/PbmQueryByRollupComplianceStatus", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::PbmServerObjectRef>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmCheckComplianceRequestType<'a> {
    entities: &'a [crate::types::structs::PbmServerObjectRef],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a crate::types::structs::PbmProfileId>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmCheckRollupComplianceRequestType<'a> {
    entity: &'a [crate::types::structs::PbmServerObjectRef],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmFetchComplianceResultRequestType<'a> {
    entities: &'a [crate::types::structs::PbmServerObjectRef],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a crate::types::structs::PbmProfileId>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmFetchRollupComplianceResultRequestType<'a> {
    entity: &'a [crate::types::structs::PbmServerObjectRef],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PbmQueryByRollupComplianceStatusRequestType<'a> {
    status: &'a str,
}
