use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Interface handling the Compliance aspects of entities.
#[derive(Clone)]
pub struct ProfileComplianceManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl ProfileComplianceManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn check_compliance_task(&self, profile: Option<&[crate::types::structs::ManagedObjectReference]>, entity: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CheckComplianceRequestType {profile, entity, };
        let bytes = self.client.invoke("", "ProfileComplianceManager", &self.mo_id, "CheckCompliance_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn clear_compliance_status(&self, profile: Option<&[crate::types::structs::ManagedObjectReference]>, entity: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<()> {
        let input = ClearComplianceStatusRequestType {profile, entity, };
        self.client.invoke_void("", "ProfileComplianceManager", &self.mo_id, "ClearComplianceStatus", Some(&input)).await
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
    pub async fn query_compliance_status(&self, profile: Option<&[crate::types::structs::ManagedObjectReference]>, entity: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<Option<Vec<crate::types::structs::ComplianceResult>>> {
        let input = QueryComplianceStatusRequestType {profile, entity, };
        let bytes_opt = self.client.invoke_optional("", "ProfileComplianceManager", &self.mo_id, "QueryComplianceStatus", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn query_expression_metadata(&self, expression_name: Option<&[String]>, profile: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<crate::types::structs::ProfileExpressionMetadata>>> {
        let input = QueryExpressionMetadataRequestType {expression_name, profile, };
        let bytes_opt = self.client.invoke_optional("", "ProfileComplianceManager", &self.mo_id, "QueryExpressionMetadata", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
}
struct CheckComplianceRequestType<'a> {
    profile: Option<&'a [crate::types::structs::ManagedObjectReference]>,
    entity: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for CheckComplianceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CheckComplianceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CheckComplianceRequestTypeSer<'b, 'a> {
    data: &'b CheckComplianceRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CheckComplianceRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CheckComplianceRequestType")),
                1 => {
                    let Some(ref val) = self.data.profile else { continue; };
                    return Some((std::borrow::Cow::Borrowed("profile"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.entity else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entity"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ClearComplianceStatusRequestType<'a> {
    profile: Option<&'a [crate::types::structs::ManagedObjectReference]>,
    entity: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for ClearComplianceStatusRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ClearComplianceStatusRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ClearComplianceStatusRequestTypeSer<'b, 'a> {
    data: &'b ClearComplianceStatusRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ClearComplianceStatusRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ClearComplianceStatusRequestType")),
                1 => {
                    let Some(ref val) = self.data.profile else { continue; };
                    return Some((std::borrow::Cow::Borrowed("profile"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.entity else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entity"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryComplianceStatusRequestType<'a> {
    profile: Option<&'a [crate::types::structs::ManagedObjectReference]>,
    entity: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for QueryComplianceStatusRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryComplianceStatusRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryComplianceStatusRequestTypeSer<'b, 'a> {
    data: &'b QueryComplianceStatusRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryComplianceStatusRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryComplianceStatusRequestType")),
                1 => {
                    let Some(ref val) = self.data.profile else { continue; };
                    return Some((std::borrow::Cow::Borrowed("profile"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.entity else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entity"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryExpressionMetadataRequestType<'a> {
    expression_name: Option<&'a [String]>,
    profile: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for QueryExpressionMetadataRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryExpressionMetadataRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryExpressionMetadataRequestTypeSer<'b, 'a> {
    data: &'b QueryExpressionMetadataRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryExpressionMetadataRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryExpressionMetadataRequestType")),
                1 => {
                    let Some(ref val) = self.data.expression_name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("expressionName"), val as &dyn miniserde::Serialize));
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
