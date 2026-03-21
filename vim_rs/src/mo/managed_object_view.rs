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
        self.client.invoke_void("", "ManagedObjectView", &self.mo_id, "DestroyView", None).await
    }
    /// The list of references to objects mapped by this view.
    pub async fn view(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let bytes_opt = self.client.fetch_property_raw("", "ManagedObjectView", &self.mo_id, "view").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
}
