use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::HostVFlashManagerVFlashCacheConfigSpec;
use crate::types::HostVFlashManagerVFlashConfigInfo;
use crate::types::HostVFlashManagerVFlashResourceConfigSpec;
use crate::types::ManagedObjectReference;
use crate::types::VirtualDiskVFlashCacheConfigInfo;
/// The VFlash Manager object is used to configure vFlash resource
/// and vFlash cache on the ESX host.
pub struct HostVFlashManager {
    client: Arc<VimClient>,
    mo_id: String,
}
impl HostVFlashManager {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
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
    pub async fn host_config_v_flash_cache(&self, spec: &HostVFlashManagerVFlashCacheConfigSpec) -> Result<()> {
        let input = HostConfigVFlashCacheRequestType {spec, };
        let path = format!("/HostVFlashManager/{moId}/HostConfigVFlashCache", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn host_configure_v_flash_resource(&self, spec: &HostVFlashManagerVFlashResourceConfigSpec) -> Result<()> {
        let input = HostConfigureVFlashResourceRequestType {spec, };
        let path = format!("/HostVFlashManager/{moId}/HostConfigureVFlashResource", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn configure_v_flash_resource_ex_task(&self, device_path: Option<&[String]>) -> Result<ManagedObjectReference> {
        let input = ConfigureVFlashResourceExRequestType {device_path, };
        let path = format!("/HostVFlashManager/{moId}/ConfigureVFlashResourceEx_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn host_get_v_flash_module_default_config(&self, v_flash_module: &str) -> Result<VirtualDiskVFlashCacheConfigInfo> {
        let input = HostGetVFlashModuleDefaultConfigRequestType {v_flash_module, };
        let path = format!("/HostVFlashManager/{moId}/HostGetVFlashModuleDefaultConfig", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
        Ok(self.client.execute_void(req).await?)
    }
    /// Host vFlash configuration information.
    pub async fn v_flash_config_info(&self) -> Result<HostVFlashManagerVFlashConfigInfo> {
        let path = format!("/HostVFlashManager/{moId}/vFlashConfigInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostConfigVFlashCacheRequestType<'a> {
    spec: &'a HostVFlashManagerVFlashCacheConfigSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostConfigureVFlashResourceRequestType<'a> {
    spec: &'a HostVFlashManagerVFlashResourceConfigSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ConfigureVFlashResourceExRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "devicePath")]
    device_path: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostGetVFlashModuleDefaultConfigRequestType<'a> {
    #[serde(rename = "vFlashModule")]
    v_flash_module: &'a str,
}
