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
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
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
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::HostCacheConfigurationInfo>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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

impl miniserde::ser::Map for ConfigureHostCacheRequestTypeSer<'_, '_> {
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
