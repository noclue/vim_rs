use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::ProfileDescription;
pub struct ClusterProfile {
    client: Arc<Client>,
    mo_id: String,
}
impl ClusterProfile {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Associate a profile with a managed entity.
    /// 
    /// You can check the compliance of
    /// entities associated with a profile by calling the
    /// *Profile.CheckProfileCompliance_Task* method.
    /// 
    /// ***Required privileges:*** Profile.Edit
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// The entity(s) to associate with the profile.
    /// If an entity is already associated with the profile, the association is
    /// maintained and the vCenter Server does not perform any action.
    /// 
    /// Refers instances of *ManagedEntity*.
    pub async fn associate_profile(&self, entity: &[ManagedObjectReference]) -> Result<()> {
        let input = AssociateProfileRequestType {entity, };
        let path = format!("/ClusterProfile/{moId}/AssociateProfile", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Check compliance of an entity against a Profile.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// If specified, the compliance check is performed on this entity.
    /// If the entity is not specified, the vCenter Server runs a compliance check on all the
    /// entities associated with the profile. The entity does not have to be associated with the
    /// profile.
    /// 
    /// Refers instances of *ManagedEntity*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation.
    /// 
    /// Refers instance of *Task*.
    pub async fn check_profile_compliance_task(&self, entity: Option<&[ManagedObjectReference]>) -> Result<ManagedObjectReference> {
        let input = CheckProfileComplianceRequestType {entity, };
        let path = format!("/ClusterProfile/{moId}/CheckProfileCompliance_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Destroy the profile.
    /// 
    /// ***Required privileges:*** Profile.Delete
    pub async fn destroy_profile(&self) -> Result<()> {
        let path = format!("/ClusterProfile/{moId}/DestroyProfile", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Remove the association between a profile and a managed entity.
    /// 
    /// ***Required privileges:*** Profile.Edit
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// List of entities. The vCenter Server will remove the associations
    /// that the profile has with the entities in the list. If unset,
    /// the Server removes all the associations that the profile has with any
    /// managed entities in the inventory.
    /// If the specified entity is not associated with the profile,
    /// the Server does not perform any action.
    /// 
    /// Refers instances of *ManagedEntity*.
    pub async fn dissociate_profile(&self, entity: Option<&[ManagedObjectReference]>) -> Result<()> {
        let input = DissociateProfileRequestType {entity, };
        let path = format!("/ClusterProfile/{moId}/DissociateProfile", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Export the profile in a serialized form.
    /// 
    /// To use the serialized string to create a profile,
    /// specify a *ProfileSerializedCreateSpec* when you call the
    /// *HostProfileManager*.*ProfileManager.CreateProfile*
    /// method.
    /// 
    /// ***Required privileges:*** Profile.Export
    ///
    /// ## Returns:
    ///
    /// Serialized form of the profile.
    pub async fn export_profile(&self) -> Result<String> {
        let path = format!("/ClusterProfile/{moId}/ExportProfile", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute(req).await
    }
    /// Returns the localizable description for the profile.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Profile divided into sections containing element descriptions and messages.
    pub async fn retrieve_description(&self) -> Result<Option<ProfileDescription>> {
        let path = format!("/ClusterProfile/{moId}/RetrieveDescription", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_option(req).await
    }
    /// Update the ClusterProfile with the specified config.
    /// 
    /// ***Required privileges:*** Profile.Edit
    ///
    /// ## Parameters:
    ///
    /// ### config
    /// Specification which describes the changes.
    ///
    /// ## Errors:
    ///
    /// ***DuplicateName***: If the profile with the new name already exists.
    pub async fn update_cluster_profile(&self, config: &dyn crate::types::traits::ClusterProfileConfigSpecTrait) -> Result<()> {
        let input = UpdateClusterProfileRequestType {config, };
        let path = format!("/ClusterProfile/{moId}/UpdateClusterProfile", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Overall compliance of entities associated with this profile.
    /// 
    /// If one of the entities is out of compliance, the profile is <code>nonCompliant</code>.
    /// If all entities are in compliance, the profile is <code>compliant</code>.
    /// If the compliance status of one of the entities is not known, compliance status
    /// of the profile is <code>unknown</code>.
    /// See *ComplianceResultStatus_enum*.
    pub async fn compliance_status(&self) -> Result<String> {
        let path = format!("/ClusterProfile/{moId}/complianceStatus", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// Configuration data for the profile.
    /// 
    /// ***Required privileges:*** Profile.Edit
    pub async fn config(&self) -> Result<Box<dyn crate::types::traits::ProfileConfigInfoTrait>> {
        let path = format!("/ClusterProfile/{moId}/config", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// Time at which the profile was created.
    pub async fn created_time(&self) -> Result<String> {
        let path = format!("/ClusterProfile/{moId}/createdTime", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// Deprecated as of vSphere API 5.0. use *Profile.RetrieveDescription* instead.
    /// 
    /// Localizable description of the profile
    pub async fn description(&self) -> Result<Option<ProfileDescription>> {
        let path = format!("/ClusterProfile/{moId}/description", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// List of managed entities associated with the profile.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *ManagedEntity*.
    pub async fn entity(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/ClusterProfile/{moId}/entity", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// Time at which the profile was last modified.
    pub async fn modified_time(&self) -> Result<String> {
        let path = format!("/ClusterProfile/{moId}/modifiedTime", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// Name of the profile.
    pub async fn name(&self) -> Result<String> {
        let path = format!("/ClusterProfile/{moId}/name", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AssociateProfileRequestType<'a> {
    entity: &'a [ManagedObjectReference],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CheckProfileComplianceRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entity: Option<&'a [ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DissociateProfileRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entity: Option<&'a [ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateClusterProfileRequestType<'a> {
    config: &'a dyn crate::types::traits::ClusterProfileConfigSpecTrait,
}
