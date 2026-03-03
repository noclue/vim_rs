use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The *InventoryView* managed object provides a means of browsing the inventory and tracking
/// changes to open folders.
/// 
/// This managed object is particularly useful for UI clients that
/// display a tree-based navigation panel of the inventory.
/// 
/// *InventoryView* maintains the *ManagedObjectView.view* list
/// of managed object references to inventory objects. When you create an inventory view
/// (*ViewManager.CreateInventoryView*), the server initializes the view's object
/// list with a single folder - the root folder.
/// 
/// *InventoryView* provides methods to open and close folders in the inventory. Use these
/// methods to add and subtract objects from the *ManagedObjectView.view* list.
/// Use the *InventoryView* together with the *PropertyCollector*
/// to manage the data resulting from *InventoryView.OpenInventoryViewFolder*
/// and *InventoryView.CloseInventoryViewFolder* methods. By using the *PropertyCollector*,
/// you have access to the modifications to the view, rather than processing the entire view list.
/// 
/// For example, you might use the following sequence of operations with
/// an *InventoryView* and the *PropertyCollector*:
/// 1. Create an *InventoryView*.
/// 2. Create a filter specification for the *PropertyCollector*.
///    - Use the *InventoryView* as the starting object in the
///      *ObjectSpec* for the filter.
///    - Use a set of *TraversalSpec*
///      data objects to identify paths in possible inventory configurations.
///    - Use the *PropertySpec*
///      to identify object properties for retrieval.
/// 3. Use either the *PropertyCollector.CheckForUpdates* or
///    *PropertyCollector.WaitForUpdates* method to obtain
///    *InventoryView* modifications. Both methods return
///    an *UpdateSet* object that describes
///    the changes returned by the *PropertyCollector*.
/// 4. Call the *InventoryView.OpenInventoryViewFolder* or *method*.
#[derive(Clone)]
pub struct InventoryView {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl InventoryView {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Notify the server that folder(s) have been closed, and changes for all
    /// its contained objects should no longer be sent.
    /// 
    /// The associated child
    /// objects are removed from the view. The containers themselves
    /// will still be retained as open objects until their parent is closed.
    /// 
    /// May partially succeed if some entities could not be resolved. The operation
    /// will still succeed for all entities that could be resolved, and the
    /// list of those that failed is returned as the result.
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// An array of managed object references. Each array entry is a
    /// reference to an entity to collapse.
    /// 
    /// ***Required privileges:*** System.View
    /// 
    /// Refers instances of *ManagedEntity*.
    ///
    /// ## Returns:
    ///
    /// A list containing any entities in the argument could not be resolved.
    /// 
    /// Refers instances of *ManagedEntity*.
    pub async fn close_inventory_view_folder(&self, entity: &[crate::types::structs::ManagedObjectReference]) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let input = CloseInventoryViewFolderRequestType {entity, };
        let path = format!("/InventoryView/{moId}/CloseInventoryViewFolder", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::ManagedObjectReference>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Destroy this view.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn destroy_view(&self) -> Result<()> {
        let path = format!("/InventoryView/{moId}/DestroyView", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Adds the child objects of a given managed entity to the view.
    /// 
    /// If a *Datacenter* is returned as a child, the implicit virtual machine folder and
    /// host folder objects are also returned. If a *ComputeResource* is returned,
    /// the implicit root *ResourcePool* and *HostSystem* objects are also returned.
    /// 
    /// May partially succeed if some entities could not be resolved. The operation
    /// will still succeed for all entities which could be resolved, and the
    /// list of those which failed is returned as the result.
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// An array of managed object references. Each array entry is a reference
    /// to an entity to expand. Expands each entity in the
    /// order given. If an entity is not in the current view,
    /// expands the view as needed.
    /// 
    /// ***Required privileges:*** System.View
    /// 
    /// Refers instances of *ManagedEntity*.
    ///
    /// ## Returns:
    ///
    /// A list containing any entities in the argument could not be resolved.
    /// 
    /// Refers instances of *ManagedEntity*.
    pub async fn open_inventory_view_folder(&self, entity: &[crate::types::structs::ManagedObjectReference]) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let input = OpenInventoryViewFolderRequestType {entity, };
        let path = format!("/InventoryView/{moId}/OpenInventoryViewFolder", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::ManagedObjectReference>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// The list of references to objects mapped by this view.
    pub async fn view(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let path = format!("/InventoryView/{moId}/view", moId = &self.mo_id);
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
struct CloseInventoryViewFolderRequestType<'a> {
    entity: &'a [crate::types::structs::ManagedObjectReference],
}

impl<'a> miniserde::Serialize for CloseInventoryViewFolderRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CloseInventoryViewFolderRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CloseInventoryViewFolderRequestTypeSer<'b, 'a> {
    data: &'b CloseInventoryViewFolderRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CloseInventoryViewFolderRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CloseInventoryViewFolderRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct OpenInventoryViewFolderRequestType<'a> {
    entity: &'a [crate::types::structs::ManagedObjectReference],
}

impl<'a> miniserde::Serialize for OpenInventoryViewFolderRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(OpenInventoryViewFolderRequestTypeSer { data: self, seq: 0 }))
    }
}

struct OpenInventoryViewFolderRequestTypeSer<'b, 'a> {
    data: &'b OpenInventoryViewFolderRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for OpenInventoryViewFolderRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"OpenInventoryViewFolderRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
