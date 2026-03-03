use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The VFlash Manager object is used to configure vFlash resource
/// and vFlash cache on the ESX host.
#[derive(Clone)]
pub struct HostVFlashManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostVFlashManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Configure vFlash cache on the host.
    /// 
    /// ***Required privileges:*** Host.Config.AdvancedConfig
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// Specification for host cache configuration.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: If the swap cache cannot be configured on the host.
    /// 
    /// ***InaccessibleVFlashSource***: vFlash resource is not accessible.
    /// 
    /// ***ResourceInUse***: The contained VFFS volume is being used.
    pub async fn host_config_v_flash_cache(&self, spec: &crate::types::structs::HostVFlashManagerVFlashCacheConfigSpec) -> Result<()> {
        let input = HostConfigVFlashCacheRequestType {spec, };
        let path = format!("/HostVFlashManager/{moId}/HostConfigVFlashCache", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Configure vFlash resource on the host by attaching to a backend VFFS volume.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// the vFlash resource specification.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: If vFlash resource cannot be configured on the host
    /// 
    /// ***ResourceInUse***: The contained VFFS volume is being used.
    pub async fn host_configure_v_flash_resource(&self, spec: &crate::types::structs::HostVFlashManagerVFlashResourceConfigSpec) -> Result<()> {
        let input = HostConfigureVFlashResourceRequestType {spec, };
        let path = format!("/HostVFlashManager/{moId}/HostConfigureVFlashResource", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Configure vFlash resource on a list of SSD disks.
    /// 
    /// If the host does not have
    /// a VFFS volume, host will format the volume first and then extend the volume
    /// on the rest of the SSDs; otherwise host will extend the existing VFFS volume
    /// on the passed SSDs. Finally host will configure the vFlash resource on the
    /// VFFS volume.
    /// 
    /// It will return *HostVFlashResourceConfigurationResult*
    /// describing success or failure associated with each device.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### device_path
    /// An array of device path names that identify disks.
    /// See *ScsiDisk*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation. The *info.result* property in the
    /// *Task* contains *HostVFlashResourceConfigurationResult*
    /// describing success or failure associated with each device.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if batch operation fails on the host.
    /// Because the returned VFlashResourceConfigurationResult contains the configuration
    /// success or fault for each device, as of vSphere API 5.x, we won't throw fault when
    /// batch operation fails.
    pub async fn configure_v_flash_resource_ex_task(&self, device_path: Option<&[String]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ConfigureVFlashResourceExRequestType {device_path, };
        let path = format!("/HostVFlashManager/{moId}/ConfigureVFlashResourceEx_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Retrieve the default supported configuration for a given vFlash module
    /// 
    /// ***Required privileges:*** Host.Config.AdvancedConfig
    ///
    /// ## Parameters:
    ///
    /// ### v_flash_module
    /// Name of the vFlash module
    ///
    /// ## Returns:
    ///
    /// The supported default vFlash cache configuration
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If vFlash resource is not configured or the contained VFFS volume
    /// cannot be found on the host.
    /// 
    /// ***HostConfigFault***: If the default vFlash module configuration option cannot be
    /// retrieved.
    pub async fn host_get_v_flash_module_default_config(&self, v_flash_module: &str) -> Result<crate::types::structs::VirtualDiskVFlashCacheConfigInfo> {
        let input = HostGetVFlashModuleDefaultConfigRequestType {v_flash_module, };
        let path = format!("/HostVFlashManager/{moId}/HostGetVFlashModuleDefaultConfig", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::VirtualDiskVFlashCacheConfigInfo = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Remove vFlash resource on the host by destroying the contained VFFS volume.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If vFlash resource is not configured or the contained VFFS volume
    /// cannot be found on the host.
    /// 
    /// ***HostConfigFault***: If vFlash resource or the contained VFFS volume cannot
    /// be removed from the host.
    /// 
    /// ***ResourceInUse***: The contained VFFS volume is being used.
    pub async fn host_remove_v_flash_resource(&self) -> Result<()> {
        let path = format!("/HostVFlashManager/{moId}/HostRemoveVFlashResource", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Host vFlash configuration information.
    pub async fn v_flash_config_info(&self) -> Result<Option<crate::types::structs::HostVFlashManagerVFlashConfigInfo>> {
        let path = format!("/HostVFlashManager/{moId}/vFlashConfigInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::HostVFlashManagerVFlashConfigInfo>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
}
struct HostConfigVFlashCacheRequestType<'a> {
    spec: &'a crate::types::structs::HostVFlashManagerVFlashCacheConfigSpec,
}

impl<'a> miniserde::Serialize for HostConfigVFlashCacheRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostConfigVFlashCacheRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostConfigVFlashCacheRequestTypeSer<'b, 'a> {
    data: &'b HostConfigVFlashCacheRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostConfigVFlashCacheRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostConfigVFlashCacheRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct HostConfigureVFlashResourceRequestType<'a> {
    spec: &'a crate::types::structs::HostVFlashManagerVFlashResourceConfigSpec,
}

impl<'a> miniserde::Serialize for HostConfigureVFlashResourceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostConfigureVFlashResourceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostConfigureVFlashResourceRequestTypeSer<'b, 'a> {
    data: &'b HostConfigureVFlashResourceRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostConfigureVFlashResourceRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostConfigureVFlashResourceRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ConfigureVFlashResourceExRequestType<'a> {
    device_path: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for ConfigureVFlashResourceExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ConfigureVFlashResourceExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ConfigureVFlashResourceExRequestTypeSer<'b, 'a> {
    data: &'b ConfigureVFlashResourceExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ConfigureVFlashResourceExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ConfigureVFlashResourceExRequestType")),
                1 => {
                    let Some(ref val) = self.data.device_path else { continue; };
                    return Some((std::borrow::Cow::Borrowed("devicePath"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct HostGetVFlashModuleDefaultConfigRequestType<'a> {
    v_flash_module: &'a str,
}

impl<'a> miniserde::Serialize for HostGetVFlashModuleDefaultConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HostGetVFlashModuleDefaultConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HostGetVFlashModuleDefaultConfigRequestTypeSer<'b, 'a> {
    data: &'b HostGetVFlashModuleDefaultConfigRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HostGetVFlashModuleDefaultConfigRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HostGetVFlashModuleDefaultConfigRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vFlashModule"), &self.data.v_flash_module as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
