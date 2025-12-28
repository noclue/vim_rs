use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The KernelModuleSystem managed object controls the configuration
/// of kernel modules on the host.
#[derive(Clone)]
pub struct HostKernelModuleSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostKernelModuleSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Query the options configured to be passed to the kernel module when loaded.
    /// 
    /// Note that this is not necessarily the option string currently in use by
    /// the kernel module.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// Module name.
    ///
    /// ## Returns:
    ///
    /// Option string to be passed to the kernel module at load time.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the kernel module does not exist on the host.
    pub async fn query_configured_module_option_string(&self, name: &str) -> Result<String> {
        let input = QueryConfiguredModuleOptionStringRequestType {name, };
        let path = format!("/HostKernelModuleSystem/{moId}/QueryConfiguredModuleOptionString", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: String = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Query the set of modules on the host.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    pub async fn query_modules(&self) -> Result<Option<Vec<crate::types::structs::KernelModuleInfo>>> {
        let path = format!("/HostKernelModuleSystem/{moId}/QueryModules", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::KernelModuleInfo>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Specifies the options to be passed to the kernel module when loaded.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// Module name.
    ///
    /// ### options
    /// Option string to be passed to the kernel module at
    /// load time.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the kernel module does not exist on the host.
    pub async fn update_module_option_string(&self, name: &str, options: &str) -> Result<()> {
        let input = UpdateModuleOptionStringRequestType {name, options, };
        let path = format!("/HostKernelModuleSystem/{moId}/UpdateModuleOptionString", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryConfiguredModuleOptionStringRequestType<'a> {
    name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateModuleOptionStringRequestType<'a> {
    name: &'a str,
    options: &'a str,
}
