use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// As vSAN VUM integration feature is mostly an automatic backend
/// feature, so currently only get and set config methods are available in its
/// control path.
/// 
/// The Managed Entity can be accessed through MOID of vsan-vum-system at
/// vCenter server side.
#[derive(Clone)]
pub struct VsanVumSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanVumSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Fetch and save support.broadcom.com cookie string for VMware ISO download
    /// authentication
    /// 
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### username
    /// Username for support.broadcom.com
    ///
    /// ### password
    /// Password for support.broadcom.com
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn fetch_iso_depot_cookie(&self, username: &str, password: &str) -> Result<()> {
        let input = FetchIsoDepotCookieRequestType {username, password, };
        let path = format!("/vsan/VsanVumSystem/{moId}/FetchIsoDepotCookie", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Fetch vSAN VUM integration configurable settings
    /// See *VsanVumSystemConfig* for more detail information
    /// about all configurable settings for vSAN VUM integration.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn get_vsan_vum_config(&self) -> Result<crate::types::structs::VsanVumSystemConfig> {
        let path = format!("/vsan/VsanVumSystem/{moId}/GetVsanVumConfig", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::VsanVumSystemConfig = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of vSphere API 8.0.
    /// 
    /// Perform full stack firmware update for the given vSAN host.
    /// 
    /// API look for the current firmware version on the host and
    /// perform a firmware update if a new firmware version is
    /// available.
    /// This API returns a task and caller should monitor the task
    /// to track firmware upgrade status. Upon successful completion
    /// of the returned *Task*, its *TaskInfo.result*
    /// field will be populated with a flag to notify whether host reboot
    /// is required.
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Target host
    /// 
    /// ***Required privileges:*** Host.Config.Firmware
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_host_update_firmware(&self, host: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanHostUpdateFirmwareRequestType {host, };
        let path = format!("/vsan/VsanVumSystem/{moId}/VsanHostUpdateFirmware", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Upload a release DB in JSON format.
    /// 
    /// The DB will replace any existing release DB, even
    /// if the existing DB may appear newer.
    /// 
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### db
    /// The new DB in JSON format
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***NotFound***: 
    /// 
    /// ***VsanFault***: 
    /// 
    /// ***NotSupported***:
    pub async fn vsan_vc_upload_release_db(&self, db: &str) -> Result<()> {
        let input = VsanVcUploadReleaseDbRequestType {db, };
        let path = format!("/vsan/VsanVumSystem/{moId}/VsanVcUploadReleaseDb", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
}
struct FetchIsoDepotCookieRequestType<'a> {
    username: &'a str,
    password: &'a str,
}

impl<'a> miniserde::Serialize for FetchIsoDepotCookieRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(FetchIsoDepotCookieRequestTypeSer { data: self, seq: 0 }))
    }
}

struct FetchIsoDepotCookieRequestTypeSer<'b, 'a> {
    data: &'b FetchIsoDepotCookieRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for FetchIsoDepotCookieRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"FetchIsoDepotCookieRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("username"), &self.data.username as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("password"), &self.data.password as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanHostUpdateFirmwareRequestType<'a> {
    host: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for VsanHostUpdateFirmwareRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanHostUpdateFirmwareRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanHostUpdateFirmwareRequestTypeSer<'b, 'a> {
    data: &'b VsanHostUpdateFirmwareRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanHostUpdateFirmwareRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanHostUpdateFirmwareRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("host"), &self.data.host as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanVcUploadReleaseDbRequestType<'a> {
    db: &'a str,
}

impl<'a> miniserde::Serialize for VsanVcUploadReleaseDbRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanVcUploadReleaseDbRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanVcUploadReleaseDbRequestTypeSer<'b, 'a> {
    data: &'b VsanVcUploadReleaseDbRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanVcUploadReleaseDbRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanVcUploadReleaseDbRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("db"), &self.data.db as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
