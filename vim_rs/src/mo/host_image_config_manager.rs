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
        let bytes_opt = self.client.invoke_optional("", "HostImageConfigManager", &self.mo_id, "fetchSoftwarePackages", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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
        let bytes = self.client.invoke("", "HostImageConfigManager", &self.mo_id, "installDate", None).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
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
        let bytes = self.client.invoke("", "HostImageConfigManager", &self.mo_id, "HostImageConfigGetAcceptance", None).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Queries the current host image profile information.
    /// 
    /// See also *HostImageProfileSummary*If there is no host image profile, then the value "&lt;Unknown&gt;" will
    /// populate both name and vendor..
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn host_image_config_get_profile(&self) -> Result<crate::types::structs::HostImageProfileSummary> {
        let bytes = self.client.invoke("", "HostImageConfigManager", &self.mo_id, "HostImageConfigGetProfile", None).await?;
        let result: crate::types::structs::HostImageProfileSummary = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
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
        self.client.invoke_void("", "HostImageConfigManager", &self.mo_id, "UpdateHostImageAcceptanceLevel", Some(&input)).await
    }
}
struct UpdateHostImageAcceptanceLevelRequestType<'a> {
    new_acceptance_level: &'a str,
}

impl<'a> miniserde::Serialize for UpdateHostImageAcceptanceLevelRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateHostImageAcceptanceLevelRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateHostImageAcceptanceLevelRequestTypeSer<'b, 'a> {
    data: &'b UpdateHostImageAcceptanceLevelRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateHostImageAcceptanceLevelRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateHostImageAcceptanceLevelRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("newAcceptanceLevel"), &self.data.new_acceptance_level as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
