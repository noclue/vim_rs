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
        let bytes = self.client.invoke("", "SiteInfoManager", &self.mo_id, "GetSiteInfo", None).await?;
        let result: crate::types::structs::SiteInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
