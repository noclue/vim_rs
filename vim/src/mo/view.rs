use std::sync::Arc;
use crate::core::client::{Client, Result};
/// *View* is the base class for session-specific view objects.
/// 
/// A view is a mechanism that supports selection of objects on the server
/// and subsequently, access to those objects.
/// To create a view, use the *ViewManager* methods.
/// A view exists until you terminate it by calling the *View.DestroyView* method,
/// or until the end of the session.
/// Access to a view is limited to the session in which it is created.
/// 
/// There are three types of views:
/// - *ContainerView*
/// - *ListView*
/// - *InventoryView*
///   
/// A view maintains a *ManagedObjectView.view* list that contains
/// managed object references. You can use a view
/// with the *PropertyCollector* to retrieve data and
/// obtain notification of changes to the virtual environment.
/// For information about using views with the PropertyCollector,
/// see the description of *ViewManager*.
pub struct View {
    client: Arc<Client>,
    mo_id: String,
}
impl View {
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
        let path = format!("/View/{moId}/DestroyView", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
}
