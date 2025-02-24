use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::KernelModuleInfo;
/// The KernelModuleSystem managed object controls the configuration
/// of kernel modules on the host.
pub struct HostKernelModuleSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl HostKernelModuleSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
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
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Query the set of modules on the host.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    pub async fn query_modules(&self) -> Result<Option<Vec<KernelModuleInfo>>> {
        let path = format!("/HostKernelModuleSystem/{moId}/QueryModules", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
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
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
