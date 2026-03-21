use std::sync::Arc;
use crate::core::client::{VimClient, Result};
#[derive(Clone)]
pub struct ClusterProfileManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl ClusterProfileManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn create_profile(&self, create_spec: &dyn crate::types::traits::ProfileCreateSpecTrait) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateProfileRequestType {create_spec, };
        let bytes = self.client.invoke("", "ClusterProfileManager", &self.mo_id, "CreateProfile", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn find_associated_profile(&self, entity: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let input = FindAssociatedProfileRequestType {entity, };
        let bytes_opt = self.client.invoke_optional("", "ClusterProfileManager", &self.mo_id, "FindAssociatedProfile", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn query_policy_metadata(&self, policy_name: Option<&[String]>, profile: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<crate::types::structs::ProfilePolicyMetadata>>> {
        let input = QueryPolicyMetadataRequestType {policy_name, profile, };
        let bytes_opt = self.client.invoke_optional("", "ClusterProfileManager", &self.mo_id, "QueryPolicyMetadata", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// A list of profiles known to this ProfileManager.
    /// 
    /// ***Required privileges:*** Profile.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Profile*.
    pub async fn profile(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let bytes_opt = self.client.fetch_property_raw("", "ClusterProfileManager", &self.mo_id, "profile").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
}
struct CreateProfileRequestType<'a> {
    create_spec: &'a dyn crate::types::traits::ProfileCreateSpecTrait,
}

impl<'a> miniserde::Serialize for CreateProfileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateProfileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateProfileRequestTypeSer<'b, 'a> {
    data: &'b CreateProfileRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateProfileRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateProfileRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("createSpec"), &self.data.create_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct FindAssociatedProfileRequestType<'a> {
    entity: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for FindAssociatedProfileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(FindAssociatedProfileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct FindAssociatedProfileRequestTypeSer<'b, 'a> {
    data: &'b FindAssociatedProfileRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for FindAssociatedProfileRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"FindAssociatedProfileRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryPolicyMetadataRequestType<'a> {
    policy_name: Option<&'a [String]>,
    profile: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for QueryPolicyMetadataRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryPolicyMetadataRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryPolicyMetadataRequestTypeSer<'b, 'a> {
    data: &'b QueryPolicyMetadataRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryPolicyMetadataRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryPolicyMetadataRequestType")),
                1 => {
                    let Some(ref val) = self.data.policy_name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("policyName"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.profile else { continue; };
                    return Some((std::borrow::Cow::Borrowed("profile"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
