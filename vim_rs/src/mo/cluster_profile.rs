use std::sync::Arc;
use crate::core::client::{VimClient, Result};
#[derive(Clone)]
pub struct ClusterProfile {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl ClusterProfile {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn associate_profile(&self, entity: &[crate::types::structs::ManagedObjectReference]) -> Result<()> {
        let input = AssociateProfileRequestType {entity, };
        self.client.invoke_void("", "ClusterProfile", &self.mo_id, "AssociateProfile", Some(&input)).await
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
    pub async fn check_profile_compliance_task(&self, entity: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CheckProfileComplianceRequestType {entity, };
        let bytes = self.client.invoke("", "ClusterProfile", &self.mo_id, "CheckProfileCompliance_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Destroy the profile.
    /// 
    /// ***Required privileges:*** Profile.Delete
    pub async fn destroy_profile(&self) -> Result<()> {
        self.client.invoke_void("", "ClusterProfile", &self.mo_id, "DestroyProfile", None).await
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
    pub async fn dissociate_profile(&self, entity: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<()> {
        let input = DissociateProfileRequestType {entity, };
        self.client.invoke_void("", "ClusterProfile", &self.mo_id, "DissociateProfile", Some(&input)).await
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
        let bytes = self.client.invoke("", "ClusterProfile", &self.mo_id, "ExportProfile", None).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Returns the localizable description for the profile.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Profile divided into sections containing element descriptions and messages.
    pub async fn retrieve_description(&self) -> Result<Option<crate::types::structs::ProfileDescription>> {
        let bytes_opt = self.client.invoke_optional("", "ClusterProfile", &self.mo_id, "RetrieveDescription", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
        self.client.invoke_void("", "ClusterProfile", &self.mo_id, "UpdateClusterProfile", Some(&input)).await
    }
    /// Overall compliance of entities associated with this profile.
    /// 
    /// If one of the entities is out of compliance, the profile is <code>nonCompliant</code>.
    /// If all entities are in compliance, the profile is <code>compliant</code>.
    /// If the compliance status of one of the entities is not known, compliance status
    /// of the profile is <code>unknown</code>.
    /// See *ComplianceResultStatus_enum*.
    pub async fn compliance_status(&self) -> Result<String> {
        let bytes_opt = self.client.fetch_property_raw("", "ClusterProfile", &self.mo_id, "complianceStatus").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property complianceStatus was empty".to_string()))?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Configuration data for the profile.
    /// 
    /// ***Required privileges:*** Profile.Edit
    pub async fn config(&self) -> Result<Box<dyn crate::types::traits::ProfileConfigInfoTrait>> {
        let bytes_opt = self.client.fetch_property_raw("", "ClusterProfile", &self.mo_id, "config").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property config was empty".to_string()))?;
        let result: Box<dyn crate::types::traits::ProfileConfigInfoTrait> = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Time at which the profile was created.
    pub async fn created_time(&self) -> Result<String> {
        let bytes_opt = self.client.fetch_property_raw("", "ClusterProfile", &self.mo_id, "createdTime").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property createdTime was empty".to_string()))?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Deprecated as of vSphere API 5.0. use *Profile.RetrieveDescription* instead.
    /// 
    /// Localizable description of the profile
    pub async fn description(&self) -> Result<Option<crate::types::structs::ProfileDescription>> {
        let bytes_opt = self.client.fetch_property_raw("", "ClusterProfile", &self.mo_id, "description").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// List of managed entities associated with the profile.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *ManagedEntity*.
    pub async fn entity(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let bytes_opt = self.client.fetch_property_raw("", "ClusterProfile", &self.mo_id, "entity").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Time at which the profile was last modified.
    pub async fn modified_time(&self) -> Result<String> {
        let bytes_opt = self.client.fetch_property_raw("", "ClusterProfile", &self.mo_id, "modifiedTime").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property modifiedTime was empty".to_string()))?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Name of the profile.
    pub async fn name(&self) -> Result<String> {
        let bytes_opt = self.client.fetch_property_raw("", "ClusterProfile", &self.mo_id, "name").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property name was empty".to_string()))?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct AssociateProfileRequestType<'a> {
    entity: &'a [crate::types::structs::ManagedObjectReference],
}

impl<'a> miniserde::Serialize for AssociateProfileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AssociateProfileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AssociateProfileRequestTypeSer<'b, 'a> {
    data: &'b AssociateProfileRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AssociateProfileRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AssociateProfileRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CheckProfileComplianceRequestType<'a> {
    entity: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for CheckProfileComplianceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CheckProfileComplianceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CheckProfileComplianceRequestTypeSer<'b, 'a> {
    data: &'b CheckProfileComplianceRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CheckProfileComplianceRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CheckProfileComplianceRequestType")),
                1 => {
                    let Some(ref val) = self.data.entity else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entity"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct DissociateProfileRequestType<'a> {
    entity: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for DissociateProfileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DissociateProfileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DissociateProfileRequestTypeSer<'b, 'a> {
    data: &'b DissociateProfileRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DissociateProfileRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DissociateProfileRequestType")),
                1 => {
                    let Some(ref val) = self.data.entity else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entity"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct UpdateClusterProfileRequestType<'a> {
    config: &'a dyn crate::types::traits::ClusterProfileConfigSpecTrait,
}

impl<'a> miniserde::Serialize for UpdateClusterProfileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateClusterProfileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateClusterProfileRequestTypeSer<'b, 'a> {
    data: &'b UpdateClusterProfileRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateClusterProfileRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateClusterProfileRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("config"), &self.data.config as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
