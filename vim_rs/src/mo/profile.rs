use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The <code>Profile</code> managed object is the base class for host and cluster
/// profiles.
#[derive(Clone)]
pub struct Profile {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl Profile {
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
        let path = format!("/Profile/{moId}/AssociateProfile", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
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
    pub async fn check_profile_compliance_task(&self, entity: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CheckProfileComplianceRequestType {entity, };
        let path = format!("/Profile/{moId}/CheckProfileCompliance_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Destroy the profile.
    /// 
    /// ***Required privileges:*** Profile.Delete
    pub async fn destroy_profile(&self) -> Result<()> {
        let path = format!("/Profile/{moId}/DestroyProfile", moId = &self.mo_id);
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
    pub async fn dissociate_profile(&self, entity: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<()> {
        let input = DissociateProfileRequestType {entity, };
        let path = format!("/Profile/{moId}/DissociateProfile", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
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
        let path = format!("/Profile/{moId}/ExportProfile", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: String = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
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
        let path = format!("/Profile/{moId}/RetrieveDescription", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::ProfileDescription>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Overall compliance of entities associated with this profile.
    /// 
    /// If one of the entities is out of compliance, the profile is <code>nonCompliant</code>.
    /// If all entities are in compliance, the profile is <code>compliant</code>.
    /// If the compliance status of one of the entities is not known, compliance status
    /// of the profile is <code>unknown</code>.
    /// See *ComplianceResultStatus_enum*.
    pub async fn compliance_status(&self) -> Result<String> {
        let path = format!("/Profile/{moId}/complianceStatus", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: String = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Configuration data for the profile.
    /// 
    /// ***Required privileges:*** Profile.Edit
    pub async fn config(&self) -> Result<Box<dyn crate::types::traits::ProfileConfigInfoTrait>> {
        let path = format!("/Profile/{moId}/config", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: Box<dyn crate::types::traits::ProfileConfigInfoTrait> = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Time at which the profile was created.
    pub async fn created_time(&self) -> Result<String> {
        let path = format!("/Profile/{moId}/createdTime", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: String = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of vSphere API 5.0. use *Profile.RetrieveDescription* instead.
    /// 
    /// Localizable description of the profile
    pub async fn description(&self) -> Result<Option<crate::types::structs::ProfileDescription>> {
        let path = format!("/Profile/{moId}/description", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::ProfileDescription>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// List of managed entities associated with the profile.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *ManagedEntity*.
    pub async fn entity(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let path = format!("/Profile/{moId}/entity", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::ManagedObjectReference>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Time at which the profile was last modified.
    pub async fn modified_time(&self) -> Result<String> {
        let path = format!("/Profile/{moId}/modifiedTime", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: String = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Name of the profile.
    pub async fn name(&self) -> Result<String> {
        let path = format!("/Profile/{moId}/name", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: String = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
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

impl miniserde::ser::Map for AssociateProfileRequestTypeSer<'_, '_> {
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

impl miniserde::ser::Map for CheckProfileComplianceRequestTypeSer<'_, '_> {
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

impl miniserde::ser::Map for DissociateProfileRequestTypeSer<'_, '_> {
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
