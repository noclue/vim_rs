use std::sync::Arc;
use crate::core::client::{Client, Result};
/// This interface is responsible for managing DirectPath profiles in vCenter.
/// 
/// ***Since:*** vSphere API Release 9.0.0.0
#[derive(Clone)]
pub struct DirectPathProfileManager {
    client: Arc<Client>,
    mo_id: String,
}
impl DirectPathProfileManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Create a DirectPath profile from the specified CreateSpec.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    /// 
    /// ***Required privileges:*** DirectPathProfileManager.Manage
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// -
    ///
    /// ## Returns:
    ///
    /// Unique identifier of the DirectPath profile created.
    ///
    /// ## Errors:
    ///
    /// ***AlreadyExists***: If a DirectPath profile with the attributes in the
    /// createSpec already exists in the target vCenter. There cannot be two
    /// DirectPath profiles with the same name or the same device details.
    /// 
    /// ***InvalidArgument***: If the spec argument does not meet the constraints
    /// specified in *DirectPathProfileManagerCreateSpec*.
    pub async fn direct_path_profile_manager_create(&self, spec: &crate::types::structs::DirectPathProfileManagerCreateSpec) -> Result<String> {
        let input = DirectPathProfileManagerCreateRequestType {spec, };
        let path = format!("/DirectPathProfileManager/{moId}/DirectPathProfileManagerCreate", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Delete a DirectPath profile.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    /// 
    /// ***Required privileges:*** DirectPathProfileManager.Manage
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// Unique identifier of the DirectPath profile to be deleted.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If there is no DirectPath profile found with the
    /// specified identifier.
    /// 
    /// ***ResourceInUse***: If the DirectPath profile with the specified identifier
    /// is being used by a VM or associated with a VM resource profile.
    pub async fn direct_path_profile_manager_delete(&self, id: &str) -> Result<()> {
        let input = DirectPathProfileManagerDeleteRequestType {id, };
        let path = format!("/DirectPathProfileManager/{moId}/DirectPathProfileManagerDelete", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// List DirectPath profiles in this vCenter that match the specified
    /// filtering criteria.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### filter_spec
    /// -
    ///
    /// ## Returns:
    ///
    /// Information about DirectPath profiles matching the attributes
    /// specified in the input *DirectPathProfileManagerFilterSpec*.
    /// If an empty filterSpec is specified, then all the DirectPath profiles in
    /// the target vCenter are returned. If none of the DirectPath profiles match
    /// the attributes specified in the filterSpec, then an empty list is
    /// returned.
    pub async fn direct_path_profile_manager_list(&self, filter_spec: &crate::types::structs::DirectPathProfileManagerFilterSpec) -> Result<Option<Vec<crate::types::structs::DirectPathProfileInfo>>> {
        let input = DirectPathProfileManagerListRequestType {filter_spec, };
        let path = format!("/DirectPathProfileManager/{moId}/DirectPathProfileManagerList", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Query capacity of DirectPath profiles against a compute resource.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    ///
    /// ## Parameters:
    ///
    /// ### target
    /// specifies the compute resource for which the capacity
    /// needs to be computed. See *DirectPathProfileManagerTargetEntity*. A null or an invalid
    /// target will cause an exception.
    ///
    /// ### query_spec
    /// specifies a list of *DirectPathProfileManagerCapacityQuerySpec*, where each
    /// of them specifies the information about the DirectPath profile for which
    /// capacity needs to be computed.
    ///
    /// ## Returns:
    ///
    /// List of *DirectPathProfileManagerCapacityResult* when target is valid. The
    /// content of the list is subjected to the following conditions:
    /// \- Content of the return list will be in the same order as content of the
    /// list in querySpec argument, except when querySpec is a null or empty
    /// list.
    /// \- If the query specification is of type *DirectPathProfileManagerCapacityQueryById*, then
    /// the capacity of a DirectPath profile with the matching ID is returned.
    /// \- If the query specification is of type *DirectPathProfileManagerCapacityQueryByName*, then
    /// the capacity of a DirectPath profile with the matching name is returned.
    /// \- If the query specification is of type
    /// *DirectPathProfileManagerCapacityQueryByDeviceConfig*, then the capacity of a device with
    /// the specified configuration in the specified Target will be returned.
    /// \- When querySpec is null or an empty list, the returned list contains
    /// *DirectPathProfileManagerCapacityInfo* of all the DirectPath profiles available in the
    /// specified target.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: when target is null or contains invalid content
    /// such as invalid host or cluster.
    pub async fn direct_path_profile_manager_query_capacity(&self, target: &dyn crate::types::traits::DirectPathProfileManagerTargetEntityTrait, query_spec: Option<&[Box<dyn crate::types::traits::DirectPathProfileManagerCapacityQuerySpecTrait>]>) -> Result<Option<Vec<Box<dyn crate::types::traits::DirectPathProfileManagerCapacityResultTrait>>>> {
        let input = DirectPathProfileManagerQueryCapacityRequestType {target, query_spec, };
        let path = format!("/DirectPathProfileManager/{moId}/DirectPathProfileManagerQueryCapacity", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Update a DirectPath profile based on the specified *DirectPathProfileManagerUpdateSpec*.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    /// 
    /// ***Required privileges:*** DirectPathProfileManager.Manage
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// Unique identifier of the DirectPath profile being updated.
    ///
    /// ### spec
    /// Specification for the DirectPath device being updated.
    ///
    /// ## Errors:
    ///
    /// ***AlreadyExists***: If the desired name specified in the spec is
    /// already assigned to a different DirectPath profile.
    /// 
    /// ***InvalidArgument***: If the spec argument does not meet the constraints
    /// specified in *DirectPathProfileManagerUpdateSpec*.
    /// 
    /// ***NotFound***: If there is no DirectPath profile found with the
    /// specified identifier.
    pub async fn direct_path_profile_manager_update(&self, id: &str, spec: &crate::types::structs::DirectPathProfileManagerUpdateSpec) -> Result<()> {
        let input = DirectPathProfileManagerUpdateRequestType {id, spec, };
        let path = format!("/DirectPathProfileManager/{moId}/DirectPathProfileManagerUpdate", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DirectPathProfileManagerCreateRequestType<'a> {
    spec: &'a crate::types::structs::DirectPathProfileManagerCreateSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DirectPathProfileManagerDeleteRequestType<'a> {
    id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DirectPathProfileManagerListRequestType<'a> {
    #[serde(rename = "filterSpec")]
    filter_spec: &'a crate::types::structs::DirectPathProfileManagerFilterSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DirectPathProfileManagerQueryCapacityRequestType<'a> {
    target: &'a dyn crate::types::traits::DirectPathProfileManagerTargetEntityTrait,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "querySpec")]
    query_spec: Option<&'a [Box<dyn crate::types::traits::DirectPathProfileManagerCapacityQuerySpecTrait>]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DirectPathProfileManagerUpdateRequestType<'a> {
    id: &'a str,
    spec: &'a crate::types::structs::DirectPathProfileManagerUpdateSpec,
}
