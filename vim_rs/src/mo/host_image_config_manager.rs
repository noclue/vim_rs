use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object is the interface for
/// configuration of the ESX software image, including
/// properties such as acceptance level.
/// 
/// It is currently designed to be host agent specific.
#[derive(Clone)]
pub struct HostImageConfigManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostImageConfigManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Reports the set of software packages installed.
    /// 
    /// The
    /// CLI command is: esxcli software vib get
    /// 
    /// ***Required privileges:*** Host.Config.Image
    pub async fn fetch_software_packages(&self) -> Result<Option<Vec<crate::types::structs::SoftwarePackage>>> {
        let path = format!("/HostImageConfigManager/{moId}/fetchSoftwarePackages", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::SoftwarePackage>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Reports the UTC time stamp when this system was first installed.
    /// 
    /// The
    /// CLI command is: esxcli system stats installtime get
    /// 
    /// ***Required privileges:*** Host.Config.Image
    pub async fn install_date(&self) -> Result<String> {
        let path = format!("/HostImageConfigManager/{moId}/installDate", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: String = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Queries the current host acceptance level setting.
    /// 
    /// See also *HostImageAcceptanceLevel_enum*If this has not been configured yet, then a default value will be
    /// returned..
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if the host acceptance setting is invalid.
    pub async fn host_image_config_get_acceptance(&self) -> Result<String> {
        let path = format!("/HostImageConfigManager/{moId}/HostImageConfigGetAcceptance", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: String = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Queries the current host image profile information.
    /// 
    /// See also *HostImageProfileSummary*If there is no host image profile, then the value "&lt;Unknown&gt;" will
    /// populate both name and vendor..
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn host_image_config_get_profile(&self) -> Result<crate::types::structs::HostImageProfileSummary> {
        let path = format!("/HostImageConfigManager/{moId}/HostImageConfigGetProfile", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::HostImageProfileSummary = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Sets the acceptance level of the host image profile.
    /// 
    /// See also *HostImageAcceptanceLevel_enum*.
    /// 
    /// ***Required privileges:*** Host.Config.Image
    ///
    /// ## Parameters:
    ///
    /// ### new_acceptance_level
    /// the new AcceptanceLevel to set.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if the acceptance level is raised and there are
    /// VIB packages that are not permitted by the
    /// higher acceptance level.
    pub async fn update_host_image_acceptance_level(&self, new_acceptance_level: &str) -> Result<()> {
        let input = UpdateHostImageAcceptanceLevelRequestType {new_acceptance_level, };
        let path = format!("/HostImageConfigManager/{moId}/UpdateHostImageAcceptanceLevel", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateHostImageAcceptanceLevelRequestType<'a> {
    #[serde(rename = "newAcceptanceLevel")]
    new_acceptance_level: &'a str,
}
