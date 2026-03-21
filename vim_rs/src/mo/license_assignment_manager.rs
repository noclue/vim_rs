use std::sync::Arc;
use crate::core::client::{VimClient, Result};
#[derive(Clone)]
pub struct LicenseAssignmentManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl LicenseAssignmentManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Get information about all the licenses associated with an entity
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### entity_id
    /// ID of the entity. E.g. HostSystem.
    pub async fn query_assigned_licenses(&self, entity_id: Option<&str>) -> Result<Option<Vec<crate::types::structs::LicenseAssignmentManagerLicenseAssignment>>> {
        let input = QueryAssignedLicensesRequestType {entity_id, };
        let bytes_opt = self.client.invoke_optional("", "LicenseAssignmentManager", &self.mo_id, "QueryAssignedLicenses", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Remove licenses associated with an entity
    /// 
    /// ***Required privileges:*** Global.Licenses
    ///
    /// ## Parameters:
    ///
    /// ### entity_id
    /// ID of the entity. E.g. HostSystem.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn remove_assigned_license(&self, entity_id: &str) -> Result<()> {
        let input = RemoveAssignedLicenseRequestType {entity_id, };
        self.client.invoke_void("", "LicenseAssignmentManager", &self.mo_id, "RemoveAssignedLicense", Some(&input)).await
    }
    /// Update the license associated with an entity
    /// 
    /// ***Required privileges:*** Global.Licenses
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// ID of the entity. E.g. HostSystem.
    ///
    /// ### license_key
    /// A license.
    ///
    /// ### entity_display_name
    /// Display name for the entity
    ///
    /// ## Returns:
    ///
    /// Returns information about the license specified in licenseKey
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn update_assigned_license(&self, entity: &str, license_key: &str, entity_display_name: Option<&str>) -> Result<crate::types::structs::LicenseManagerLicenseInfo> {
        let input = UpdateAssignedLicenseRequestType {entity, license_key, entity_display_name, };
        let bytes = self.client.invoke("", "LicenseAssignmentManager", &self.mo_id, "UpdateAssignedLicense", Some(&input)).await?;
        let result: crate::types::structs::LicenseManagerLicenseInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct QueryAssignedLicensesRequestType<'a> {
    entity_id: Option<&'a str>,
}

impl<'a> miniserde::Serialize for QueryAssignedLicensesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryAssignedLicensesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryAssignedLicensesRequestTypeSer<'b, 'a> {
    data: &'b QueryAssignedLicensesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryAssignedLicensesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryAssignedLicensesRequestType")),
                1 => {
                    let Some(ref val) = self.data.entity_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entityId"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RemoveAssignedLicenseRequestType<'a> {
    entity_id: &'a str,
}

impl<'a> miniserde::Serialize for RemoveAssignedLicenseRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveAssignedLicenseRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveAssignedLicenseRequestTypeSer<'b, 'a> {
    data: &'b RemoveAssignedLicenseRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveAssignedLicenseRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveAssignedLicenseRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("entityId"), &self.data.entity_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateAssignedLicenseRequestType<'a> {
    entity: &'a str,
    license_key: &'a str,
    entity_display_name: Option<&'a str>,
}

impl<'a> miniserde::Serialize for UpdateAssignedLicenseRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateAssignedLicenseRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateAssignedLicenseRequestTypeSer<'b, 'a> {
    data: &'b UpdateAssignedLicenseRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateAssignedLicenseRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateAssignedLicenseRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("licenseKey"), &self.data.license_key as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.entity_display_name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entityDisplayName"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
