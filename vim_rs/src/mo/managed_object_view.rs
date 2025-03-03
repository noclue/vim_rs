use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::ManagedObjectReference;
/// *ManagedObjectView* is the base class for view objects that provide access
/// to a set of *ManagedEntity* objects.
/// 
/// *ManagedObjectView* defines
/// a view list; the list contains references to objects in the view.
/// To create a view use the *ViewManager* methods.
pub struct ManagedObjectView {
    client: Arc<Client>,
    mo_id: String,
}
impl ManagedObjectView {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
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
    pub async fn view(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/ManagedObjectView/{moId}/view", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
}
