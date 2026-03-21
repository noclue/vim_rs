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
        let bytes = self.client.invoke("", "HostCacheConfigurationManager", &self.mo_id, "ConfigureHostCache_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// The swap performance configuration for the ESX host.
    /// 
    /// This includes
    /// configuration information for each datastore enabled for this purpose.
    /// 
    /// ***Required privileges:*** Host.Config.AdvancedConfig
    pub async fn cache_configuration_info(&self) -> Result<Option<Vec<crate::types::structs::HostCacheConfigurationInfo>>> {
        let bytes_opt = self.client.fetch_property_raw("", "HostCacheConfigurationManager", &self.mo_id, "cacheConfigurationInfo").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
}
struct ConfigureHostCacheRequestType<'a> {
    spec: &'a crate::types::structs::HostCacheConfigurationSpec,
}

impl<'a> miniserde::Serialize for ConfigureHostCacheRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ConfigureHostCacheRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ConfigureHostCacheRequestTypeSer<'b, 'a> {
    data: &'b ConfigureHostCacheRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ConfigureHostCacheRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ConfigureHostCacheRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
