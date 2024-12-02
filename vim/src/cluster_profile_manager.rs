use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::ManagedObjectReference;
use crate::types::ProfileCreateSpecTrait;
use crate::types::ProfilePolicyMetadata;
pub struct ClusterProfileManager {
    client: Arc<VimClient>,
    mo_id: String,
}
impl ClusterProfileManager {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Create a profile from the specified CreateSpec.
    /// 
    /// ***Required privileges:*** Profile.Create
    ///
    /// ## Parameters:
    ///
    /// ### create_spec
    /// Specification for the profile being created.
    /// Usually a derived class CreateSpec can be used to create the Profile.
    ///
    /// ## Returns:
    ///
    /// Profile created from the specified createSpec.
    /// 
    /// Refers instance of *Profile*.
    ///
    /// ## Errors:
    ///
    /// ***DuplicateName***: If a profile with the specified name already
    /// exists.
    /// 
    /// ***InvalidProfileReferenceHost***: if the specified reference host is
    /// incompatible or no reference host has been specified.
    pub async fn create_profile(&self, create_spec: &dyn ProfileCreateSpecTrait) -> Result<ManagedObjectReference> {
        let input = CreateProfileRequestType {create_spec, };
        let path = format!("/ClusterProfileManager/{moId}/CreateProfile", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Get the profile(s) to which this entity is associated.
    /// 
    /// The list of profiles will only include profiles known to this
    /// profileManager.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// Entity for which profile is being looked up.
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Profile*.
    pub async fn find_associated_profile(&self, entity: &ManagedObjectReference) -> Result<Option<Vec<ManagedObjectReference>>> {
        let input = FindAssociatedProfileRequestType {entity, };
        let path = format!("/ClusterProfileManager/{moId}/FindAssociatedProfile", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Get the Metadata information for the policyNames.
    /// 
    /// PolicyNames are available with the defaultProfile obtained by invoking the
    /// method createDefaultProfile.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### policy_name
    /// Retrieve metadata for the specified policyNames.
    /// If policyName is not specified, metadata for all policies will be returned.
    ///
    /// ### profile
    /// Base profile whose context needs to be used during the operation
    /// 
    /// Refers instance of *Profile*.
    ///
    /// ## Returns:
    ///
    /// The metadata information for the policy.
    pub async fn query_policy_metadata(&self, policy_name: Option<&[String]>, profile: Option<&ManagedObjectReference>) -> Result<Option<Vec<ProfilePolicyMetadata>>> {
        let input = QueryPolicyMetadataRequestType {policy_name, profile, };
        let path = format!("/ClusterProfileManager/{moId}/QueryPolicyMetadata", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// A list of profiles known to this ProfileManager.
    /// 
    /// ***Required privileges:*** Profile.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Profile*.
    pub async fn profile(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/ClusterProfileManager/{moId}/profile", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateProfileRequestType<'a> {
    #[serde(rename = "createSpec")]
    create_spec: &'a dyn ProfileCreateSpecTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct FindAssociatedProfileRequestType<'a> {
    entity: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryPolicyMetadataRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "policyName")]
    policy_name: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a ManagedObjectReference>,
}
