use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The *PropertyFilter* managed object type defines a filter
/// that controls the properties for which a *PropertyCollector* detects
/// incremental changes.
/// 
/// Filters are subordinate objects; they are part of the *PropertyCollector* and do not have independent lifetimes. A Filter
/// is automatically destroyed when the session on which it was created is
/// closed or the *PropertyCollector* on which it was created is
/// destroyed.
#[derive(Clone)]
pub struct PropertyFilter {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl PropertyFilter {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Destroys this filter.
    /// 
    /// This operation can be called explicitly, or it can take place
    /// implicitly when the session that created the filter is closed.
    pub async fn destroy_property_filter(&self) -> Result<()> {
        let path = format!("/PropertyFilter/{moId}/DestroyPropertyFilter", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Flag to indicate if a change to a nested property reports only the
    /// nested change or the entire specified property value.
    /// 
    /// If the value is
    /// true, a change reports only the nested property. If the value is
    /// false, a change reports the enclosing property named in the filter.
    pub async fn partial_updates(&self) -> Result<bool> {
        let path = format!("/PropertyFilter/{moId}/partialUpdates", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: bool = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Specifications for this filter.
    pub async fn spec(&self) -> Result<crate::types::structs::PropertyFilterSpec> {
        let path = format!("/PropertyFilter/{moId}/spec", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::PropertyFilterSpec = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
