use std::sync::Arc;
use crate::core::client::{VimClient, Result};
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
#[derive(Clone)]
pub struct ContainerView {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl ContainerView {
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
        let path = format!("/ContainerView/{moId}/DestroyView", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// The Folder, Datacenter, ComputeResource, ResourcePool, or HostSystem instance
    /// that provides the objects that the view presents.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *ManagedEntity*.
    pub async fn container(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let path = format!("/ContainerView/{moId}/container", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::ManagedObjectReference = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
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
        let bytes = self.client.execute_bytes(req).await?;
        let result: bool = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// An optional list of types to be applied to the set of objects in the view.
    /// 
    /// The list of types indicates objects that are included in the view.
    /// If empty, all types are included.
    pub async fn r#type(&self) -> Result<Option<Vec<String>>> {
        let path = format!("/ContainerView/{moId}/type", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<String>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// The list of references to objects mapped by this view.
    pub async fn view(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let path = format!("/ContainerView/{moId}/view", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::ManagedObjectReference>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
}
