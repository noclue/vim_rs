use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::ComplianceResult;
use crate::types::ManagedObjectReference;
use crate::types::ProfileExpressionMetadata;
/// Interface handling the Compliance aspects of entities.
pub struct ProfileComplianceManager {
    client: Arc<VimClient>,
    mo_id: String,
}
impl ProfileComplianceManager {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Check compliance of an entity against a Profile.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### profile
    /// If specified, check compliance against the specified profiles.
    /// If not specified, use the profiles associated with the entities.
    /// If both Profiles and Entities are specified, Check the compliance of each
    /// Entity against each of the profile specified.
    ///   
    /// For more information, look at the KMap below.
    ///   
    /// P represents if Profile is specified.
    ///   
    /// E represents if Entity is specified.
    /// 
    ///                 P                        ^P
    ///       ---------------------------------------------------
    ///       | Check compliance      |  Profiles associated    |
    ///      E|  of each entity       |   with the specified    |
    ///       |  against each of the  |   entity will be used   |
    ///       |  profiles specified.  |   for checking          |
    ///       |                       |   compliance.           |
    ///       |                       |                         |
    ///       |                       |                         |
    ///       ---------------------------------------------------
    ///       | All entities          |   InvalidArgument       |
    ///       |  associated with the  |   Exception is thrown.  |
    ///       |  profile are checked. |                         |
    ///     ^E|                       |                         |
    ///       |                       |                         |
    ///       |                       |                         |
    ///       |                       |                         |
    ///       ---------------------------------------------------
    /// 
    /// Refers instances of *Profile*.
    ///
    /// ### entity
    /// If specified, the compliance check is done against this entity.
    /// 
    /// Refers instances of *ManagedEntity*.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    pub async fn check_compliance_task(&self, profile: Option<&[ManagedObjectReference]>, entity: Option<&[ManagedObjectReference]>) -> Result<ManagedObjectReference> {
        let input = CheckComplianceRequestType {profile, entity, };
        let path = format!("/ProfileComplianceManager/{moId}/CheckCompliance_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Clear the saved ComplianceResult based on profile and entity filtering criteria.
    /// 
    /// ***Required privileges:*** Profile.Clear
    ///
    /// ## Parameters:
    ///
    /// ### profile
    /// If specified, clear the ComplianceResult related to the Profile.
    /// 
    /// Refers instances of *Profile*.
    ///
    /// ### entity
    /// If specified, clear the ComplianceResult related to the entity.
    /// If profile and entity are not specified, all the ComplianceResults will be cleared.
    /// 
    /// Refers instances of *ManagedEntity*.
    pub async fn clear_compliance_status(&self, profile: Option<&[ManagedObjectReference]>, entity: Option<&[ManagedObjectReference]>) -> Result<()> {
        let input = ClearComplianceStatusRequestType {profile, entity, };
        let path = format!("/ProfileComplianceManager/{moId}/ClearComplianceStatus", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Query the compliance status based on Profile and Entity filter.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### profile
    /// If specified, compliance result for the specified profiles will be
    /// returned. This acts like a filtering criteria for the ComplianceResults based on
    /// specified profiles.
    /// 
    /// Refers instances of *Profile*.
    ///
    /// ### entity
    /// If specified, compliance results for these entities will be returned.
    /// This acts like a filtering criteria for the ComplianceResults based on entities.
    /// 
    /// Refers instances of *ManagedEntity*.
    ///
    /// ## Returns:
    ///
    /// ComplianceResult. ComplianceResult information may not be
    /// available for all the entities. If the ComplianceResult is not available already,
    /// a new ComplianceCheck will not be triggered.
    pub async fn query_compliance_status(&self, profile: Option<&[ManagedObjectReference]>, entity: Option<&[ManagedObjectReference]>) -> Result<Option<Vec<ComplianceResult>>> {
        let input = QueryComplianceStatusRequestType {profile, entity, };
        let path = format!("/ProfileComplianceManager/{moId}/QueryComplianceStatus", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Query the metadata for the expressions.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### expression_name
    /// Names of the Expressions for which metadata is requested.
    /// If expressionNames are not specified, metadata for all known expressions is returned
    ///
    /// ### profile
    /// Base profile whose context needs to be used during the operation
    /// 
    /// Refers instance of *Profile*.
    pub async fn query_expression_metadata(&self, expression_name: Option<&[String]>, profile: Option<&ManagedObjectReference>) -> Result<Option<Vec<ProfileExpressionMetadata>>> {
        let input = QueryExpressionMetadataRequestType {expression_name, profile, };
        let path = format!("/ProfileComplianceManager/{moId}/QueryExpressionMetadata", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CheckComplianceRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a [ManagedObjectReference]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entity: Option<&'a [ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ClearComplianceStatusRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a [ManagedObjectReference]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entity: Option<&'a [ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryComplianceStatusRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a [ManagedObjectReference]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entity: Option<&'a [ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryExpressionMetadataRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expressionName")]
    expression_name: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a ManagedObjectReference>,
}
