use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::HostCacheConfigurationInfo;
use crate::types::structs::HostCacheConfigurationSpec;
use crate::types::structs::ManagedObjectReference;
/// Solid state drive Cache Configuration Manager.
/// 
/// This is a managed object which provides access to ESX performance tuning
/// features using solid state drive based cache.
pub struct HostCacheConfigurationManager {
    client: Arc<Client>,
    mo_id: String,
}
impl HostCacheConfigurationManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
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
    pub async fn configure_host_cache_task(&self, spec: &HostCacheConfigurationSpec) -> Result<ManagedObjectReference> {
        let input = ConfigureHostCacheRequestType {spec, };
        let path = format!("/HostCacheConfigurationManager/{moId}/ConfigureHostCache_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// The swap performance configuration for the ESX host.
    /// 
    /// This includes
    /// configuration information for each datastore enabled for this purpose.
    /// 
    /// ***Required privileges:*** Host.Config.AdvancedConfig
    pub async fn cache_configuration_info(&self) -> Result<Option<Vec<HostCacheConfigurationInfo>>> {
        let path = format!("/HostCacheConfigurationManager/{moId}/cacheConfigurationInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ConfigureHostCacheRequestType<'a> {
    spec: &'a HostCacheConfigurationSpec,
}
