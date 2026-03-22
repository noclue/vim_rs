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
        self.client.invoke_void("", "ContainerView", &self.mo_id, "DestroyView", None).await
    }
    /// The Folder, Datacenter, ComputeResource, ResourcePool, or HostSystem instance
    /// that provides the objects that the view presents.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *ManagedEntity*.
    pub async fn container(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let pv_opt = self.client.fetch_property_raw("", "ContainerView", &self.mo_id, "container").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property container was empty".to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// Whether to include only the immediate children of the container instance,
    /// or to include additional objects by following the paths beyond the
    /// immediate children.
    /// 
    /// For information about recursive behavior, see the description of
    /// *ViewManager.CreateContainerView*.
    pub async fn recursive(&self) -> Result<bool> {
        let pv_opt = self.client.fetch_property_raw("", "ContainerView", &self.mo_id, "recursive").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property recursive was empty".to_string()))?;
        let result: bool = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// An optional list of types to be applied to the set of objects in the view.
    /// 
    /// The list of types indicates objects that are included in the view.
    /// If empty, all types are included.
    pub async fn r#type(&self) -> Result<Option<Vec<String>>> {
        let pv_opt = self.client.fetch_property_raw("", "ContainerView", &self.mo_id, "type").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// The list of references to objects mapped by this view.
    pub async fn view(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let pv_opt = self.client.fetch_property_raw("", "ContainerView", &self.mo_id, "view").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
}
