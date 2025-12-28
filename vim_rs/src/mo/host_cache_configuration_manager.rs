use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Deprecated as of vSphere API 9.0 with no direct replacement. Still,
/// you may consider using Memory Tiering APIs
/// *NVMe*.
/// 
/// Solid state drive Cache Configuration Manager.
/// 
/// This is a managed object which provides access to ESX performance tuning
/// features using solid state drive based cache.
#[derive(Clone)]
pub struct HostCacheConfigurationManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostCacheConfigurationManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Configure host cache/swap performance enhancement.
    /// 
    /// ***Required privileges:*** Host.Config.AdvancedConfig
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// Specification for solid state drive cache configuration.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the operation.
    /// 
    /// Refers instance of *Task*.
    pub async fn configure_host_cache_task(&self, spec: &crate::types::structs::HostCacheConfigurationSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ConfigureHostCacheRequestType {spec, };
        let path = format!("/HostCacheConfigurationManager/{moId}/ConfigureHostCache_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::ManagedObjectReference = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// The swap performance configuration for the ESX host.
    /// 
    /// This includes
    /// configuration information for each datastore enabled for this purpose.
    /// 
    /// ***Required privileges:*** Host.Config.AdvancedConfig
    pub async fn cache_configuration_info(&self) -> Result<Option<Vec<crate::types::structs::HostCacheConfigurationInfo>>> {
        let path = format!("/HostCacheConfigurationManager/{moId}/cacheConfigurationInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::HostCacheConfigurationInfo>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ConfigureHostCacheRequestType<'a> {
    spec: &'a crate::types::structs::HostCacheConfigurationSpec,
}
