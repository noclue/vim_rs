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
        self.client.invoke_void("", "PropertyFilter", &self.mo_id, "DestroyPropertyFilter", None).await
    }
    /// Flag to indicate if a change to a nested property reports only the
    /// nested change or the entire specified property value.
    /// 
    /// If the value is
    /// true, a change reports only the nested property. If the value is
    /// false, a change reports the enclosing property named in the filter.
    pub async fn partial_updates(&self) -> Result<bool> {
        let bytes_opt = self.client.fetch_property_raw("", "PropertyFilter", &self.mo_id, "partialUpdates").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property partialUpdates was empty".to_string()))?;
        let result: bool = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Specifications for this filter.
    pub async fn spec(&self) -> Result<crate::types::structs::PropertyFilterSpec> {
        let bytes_opt = self.client.fetch_property_raw("", "PropertyFilter", &self.mo_id, "spec").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property spec was empty".to_string()))?;
        let result: crate::types::structs::PropertyFilterSpec = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
