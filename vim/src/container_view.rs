use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::ManagedObjectReference;
/// The *ContainerView* managed object provides a means of monitoring the contents of
/// a single container and, optionally, other containers.
/// 
/// You can use a *ContainerView* with a *PropertyCollector* method
/// to retrieve data or receive notification of changes. For information about using views
/// with the *PropertyCollector*, see the description of *ViewManager*.
/// 
/// When you invoke the *ViewManager.CreateContainerView* method, you specify
/// a managed object instance that provides the starting point for object selection.
/// You can use the following managed objects as the basis of a container view:
/// - *Folder*
/// - *Datacenter*
/// - *ComputeResource*
/// - *ResourcePool*
/// - *HostSystem*
///   
/// Once you have created the view, the *ManagedObjectView.view* list
/// always represents the current configuration of the virtual environment and reflects
/// any subsequent changes that occur.
pub struct ContainerView {
    client: Arc<VimClient>,
    mo_id: String,
}
impl ContainerView {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Destroy this view.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn destroy_view(&self) -> Result<()> {
        let path = format!("/ContainerView/{moId}/DestroyView", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// The Folder, Datacenter, ComputeResource, ResourcePool, or HostSystem instance
    /// that provides the objects that the view presents.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *ManagedEntity*.
    pub async fn container(&self) -> Result<ManagedObjectReference> {
        let path = format!("/ContainerView/{moId}/container", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Whether to include only the immediate children of the container instance,
    /// or to include additional objects by following the paths beyond the
    /// immediate children.
    /// 
    /// For information about recursive behavior, see the description of
    /// *ViewManager.CreateContainerView*.
    pub async fn recursive(&self) -> Result<bool> {
        let path = format!("/ContainerView/{moId}/recursive", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// An optional list of types to be applied to the set of objects in the view.
    /// 
    /// The list of types indicates objects that are included in the view.
    /// If empty, all types are included.
    pub async fn r#type(&self) -> Result<Option<Vec<String>>> {
        let path = format!("/ContainerView/{moId}/type", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// The list of references to objects mapped by this view.
    pub async fn view(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/ContainerView/{moId}/view", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
