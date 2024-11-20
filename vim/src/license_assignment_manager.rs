use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::LicenseAssignmentManagerLicenseAssignment;
use crate::types::LicenseManagerLicenseInfo;
pub struct LicenseAssignmentManager {
    client: Arc<VimClient>,
    mo_id: String,
}
impl LicenseAssignmentManager {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
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
    pub async fn query_assigned_licenses(&self, entity_id: Option<&str>) -> Result<Option<Vec<LicenseAssignmentManagerLicenseAssignment>>> {
        let input = QueryAssignedLicensesRequestType {entity_id, };
        let path = format!("/LicenseAssignmentManager/{moId}/QueryAssignedLicenses", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
        let path = format!("/LicenseAssignmentManager/{moId}/RemoveAssignedLicense", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn update_assigned_license(&self, entity: &str, license_key: &str, entity_display_name: Option<&str>) -> Result<LicenseManagerLicenseInfo> {
        let input = UpdateAssignedLicenseRequestType {entity, license_key, entity_display_name, };
        let path = format!("/LicenseAssignmentManager/{moId}/UpdateAssignedLicense", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryAssignedLicensesRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "entityId")]
    entity_id: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveAssignedLicenseRequestType<'a> {
    #[serde(rename = "entityId")]
    entity_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateAssignedLicenseRequestType<'a> {
    entity: &'a str,
    #[serde(rename = "licenseKey")]
    license_key: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "entityDisplayName")]
    entity_display_name: Option<&'a str>,
}
