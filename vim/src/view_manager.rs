use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::ManagedObjectReference;
/// The *ViewManager* managed object provides methods to create *ContainerView*,
/// *InventoryView*, and *ListView* managed objects.
/// 
/// The *ViewManager* also maintains a list of managed object references
/// to the views that you have created. Use the *ViewManager.viewList*
/// property to access the views.
/// 
/// A *View* is a mechanism that supports selection of objects on the server
/// and subsequently, access to those objects. Views can simplify the task of
/// retrieving data from the server. When you use a view, you can use a single
/// invocation of a *PropertyCollector* method
/// to retrieve data or receive notification of changes instead of multiple invocations
/// involving multiple filter specifications. A view exists until you destroy it
/// or until the end of the session.
/// 
/// The *ViewManager* supports the following views:
/// - A *ContainerView* is based on *Folder*,
///   *Datacenter*, *ComputeResource*,
///   *ResourcePool*, or *HostSystem* managed objects.
///   Use a container view to monitor the container contents and optionally,
///   its descendants.
/// - A *ListView* managed object is based on an arbitrary but
///   specific set of objects. When you create a list view, you provide
///   a list of objects to populate the view
///   (*ViewManager.CreateListView*),
///   or you provide an existing view from which the new view is created
///   (*ViewManager.CreateListViewFromView*).
/// - An *InventoryView* managed object is based on the entire inventory.
///   Use an inventory view as a general mechanism to monitor the inventory
///   or portions of the inventory.
///   
/// For example, you might use the following sequence of operations to get the
/// names of all the virtual machines on a server:
/// 1. Create a *ContainerView* for the root folder in the server inventory.
///    For the *ContainerView*, use the *ContainerView.type* property
///    to include only virtual machines.
/// 2. Create a filter specification for the *PropertyCollector*.
///    - Use the *ContainerView* as the starting object in the
///      *ObjectSpec* for the filter.
///    - Use the *TraversalSpec*
///      to select all objects in the view list (all the virtual machines).
///    - Use the *PropertySpec*
///      to retrieve the name property from each virtual machine.
/// 3. Invoke the *PropertyCollector*
///    *PropertyCollector.RetrieveProperties* method.
pub struct ViewManager {
    client: Arc<VimClient>,
    mo_id: String,
}
impl ViewManager {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Create a *ContainerView* managed object for this session.
    /// 
    /// The method returns
    /// a reference to a *ContainerView* object that has a list of managed object references.
    /// The list references objects in the container and may include references to objects from
    /// additional containers. You can configure the resulting list of objects by specifying
    /// a type list and recursion. Once you have created the view, the object list always
    /// represents the current configuration of the virtual environment and reflects any
    /// subsequent changes that occur.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### container
    /// A reference to an instance of a *Folder*,
    /// *Datacenter*, *ComputeResource*,
    /// *ResourcePool*, or *HostSystem* object.
    /// 
    /// ***Required privileges:*** System.View
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ### r#type
    /// An optional list of managed entity types. The server
    /// associates only objects of the specified type(s) with the view.
    /// If you specify an empty array, the server uses all types.
    ///
    /// ### recursive
    /// Whether to include only the immediate children of the
    /// container instance, or to include additional objects by
    /// following paths beyond the immediate children.
    /// 
    /// When recursive is false, the list of objects contains
    /// only immediate children.
    /// When recursive is true, the server populates the list
    /// by following references beyond the immediate children
    /// (using a child's references, and then references in the
    /// resulting objects, and so on).
    /// 
    /// Depending on the container type, the server will use the following
    /// properties of the container instance to obtain objects for the
    /// view's object list:
    /// - *Folder* object - *Folder.childEntity*
    ///   property.
    ///   If recursive is false, the container list includes the reference
    ///   to the child entity in the folder instance.
    ///   If recursive is true, the server will follow the child
    ///   folder path(s) to collect additional childEntity references.
    /// - *ResourcePool* object - *ResourcePool.vm*
    ///   and *ResourcePool.resourcePool* properties.
    ///   If recursive is false, the object list will contain references
    ///   to the virtual machines associated with this resource pool,
    ///   and references to virtual machines associated with the
    ///   immediate child resource pools. If recursive is true,
    ///   the server will follow all child resource pool paths
    ///   extending from the immediate children (and their children,
    ///   and so on) to collect additional references to virtual machines.
    /// - *ComputeResource* object - *ComputeResource.host*
    ///   and *ComputeResource.resourcePool* properties.
    ///   If recursive is false, the object list will contain references
    ///   to the host systems associated with this compute resource,
    ///   references to virtual machines associated with the
    ///   host systems, and references to virtual machines associated
    ///   with the immediate child resource pools.
    ///   If recursive is true, the server will follow the child
    ///   resource pool paths (and their child resource pool paths,
    ///   and so on) to collect additional references to virtual machines.
    /// - *Datacenter* object - *Datacenter.vmFolder*,
    ///   *Datacenter.hostFolder*,
    ///   *Datacenter.datastoreFolder*, and
    ///   *Datacenter.networkFolder* properties.
    ///   If recursive is set to false, the server uses the
    ///   immediate child folders for the virtual machines,
    ///   hosts, datastores, and networks associated with this
    ///   datacenter. If recursive is set to true, the server
    ///   will follow the folder paths to collect references
    ///   to additional objects.
    /// - *HostSystem* object - *HostSystem.vm*
    ///   property.
    ///   The view object list contains references to the virtual machines
    ///   associated with this host system. The value of recursive does not
    ///   affect this behavior.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *ContainerView*.
    pub async fn create_container_view(&self, container: &ManagedObjectReference, r#type: Option<&[String]>, recursive: bool) -> Result<ManagedObjectReference> {
        let input = CreateContainerViewRequestType {container, r#type, recursive, };
        let path = format!("/ViewManager/{moId}/CreateContainerView", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Create a new *InventoryView* managed object for this session.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instance of *InventoryView*.
    pub async fn create_inventory_view(&self) -> Result<ManagedObjectReference> {
        let path = format!("/ViewManager/{moId}/CreateInventoryView", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Create a *ListView* object for this session.
    /// 
    /// The method returns
    /// a session object that has a list of managed object references. The list
    /// of references corresponds to the input object list.
    /// You can modify the resulting list after you have created the object.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### obj
    /// The initial list of objects in the view.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instance of *ListView*.
    pub async fn create_list_view(&self, obj: Option<&[ManagedObjectReference]>) -> Result<ManagedObjectReference> {
        let input = CreateListViewRequestType {obj, };
        let path = format!("/ViewManager/{moId}/CreateListView", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Create a *ListView* object for this session.
    /// 
    /// This method uses an existing
    /// view to construct the object list for the new view.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### view
    /// The view that will provide the object list for the
    /// new ListView object.
    /// 
    /// Refers instance of *View*.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *ListView*.
    pub async fn create_list_view_from_view(&self, view: &ManagedObjectReference) -> Result<ManagedObjectReference> {
        let input = CreateListViewFromViewRequestType {view, };
        let path = format!("/ViewManager/{moId}/CreateListViewFromView", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// An array of view references.
    /// 
    /// Each array entry is a managed object reference
    /// to a view created by this ViewManager.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *View*.
    pub async fn view_list(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/ViewManager/{moId}/viewList", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateContainerViewRequestType<'a> {
    container: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    r#type: Option<&'a [String]>,
    recursive: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateListViewRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    obj: Option<&'a [ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateListViewFromViewRequestType<'a> {
    view: &'a ManagedObjectReference,
}
