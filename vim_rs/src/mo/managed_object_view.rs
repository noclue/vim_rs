use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// *ManagedObjectView* is the base class for view objects that provide access
/// to a set of *ManagedEntity* objects.
/// 
/// *ManagedObjectView* defines
/// a view list; the list contains references to objects in the view.
/// To create a view use the *ViewManager* methods.
#[derive(Clone)]
pub struct ManagedObjectView {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl ManagedObjectView {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Destroy this view.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn destroy_view(&self) -> Result<()> {
        let path = format!("/ManagedObjectView/{moId}/DestroyView", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// The list of references to objects mapped by this view.
    pub async fn view(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let path = format!("/ManagedObjectView/{moId}/view", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::ManagedObjectReference>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
}
