use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object type is used for managing external site-related
/// capabilities which are advertised by vCenter.
#[derive(Clone)]
pub struct SiteInfoManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl SiteInfoManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Returns the *SiteInfo* object associated with this vCenter.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn get_site_info(&self) -> Result<crate::types::structs::SiteInfo> {
        let path = format!("/SiteInfoManager/{moId}/GetSiteInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::SiteInfo = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
}
