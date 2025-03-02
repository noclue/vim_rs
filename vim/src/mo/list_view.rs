use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::ManagedObjectReference;
/// The *ListView* managed object provides access to updates on a specific set of objects.
/// 
/// You can use a *ListView* with a *PropertyCollector* method
/// to retrieve data or receive notification of changes. For information about using views
/// with the *PropertyCollector*, see the description of *ViewManager*.
/// 
/// When you invoke the *ViewManager.CreateListView* method, you specify
/// a list of objects. The *ManagedObjectView.view* list
/// always represents the current configuration of the virtual environment
/// and reflects any subsequent changes that occur.
pub struct ListView {
    client: Arc<Client>,
    mo_id: String,
}
impl ListView {
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
        let path = format!("/ListView/{moId}/DestroyView", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Modify the list by giving a delta of entities to add and
    /// entities to remove.
    /// 
    /// May partially succeed if some objects could not be resolved. The operation
    /// will still succeed for all objects which could be resolved, and the
    /// list of those which failed is returned as the result.
    ///
    /// ## Parameters:
    ///
    /// ### add
    /// Optional list of objects to add to the view.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ### remove
    /// Optional list of objects to remove from the view.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// A list containing any objects in 'add' that could not be resolved.
    pub async fn modify_list_view(&self, add: Option<&[ManagedObjectReference]>, remove: Option<&[ManagedObjectReference]>) -> Result<Option<Vec<ManagedObjectReference>>> {
        let input = ModifyListViewRequestType {add, remove, };
        let path = format!("/ListView/{moId}/ModifyListView", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Replaces the list with an entirely new set of objects.
    /// 
    /// If
    /// the entire set is changing, this is less data to send than a delta.
    /// 
    /// May partially succeed if some objects could not be resolved. The operation
    /// will still succeed for all objects which could be resolved, and the
    /// list of those which failed is as the result.
    ///
    /// ## Parameters:
    ///
    /// ### obj
    /// The new list of objects.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// A list containing any objects in 'obj' that could not be resolved.
    pub async fn reset_list_view(&self, obj: Option<&[ManagedObjectReference]>) -> Result<Option<Vec<ManagedObjectReference>>> {
        let input = ResetListViewRequestType {obj, };
        let path = format!("/ListView/{moId}/ResetListView", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Replaces the list with the set of objects in a given view.
    ///
    /// ## Parameters:
    ///
    /// ### view
    /// The view to copy objects from.
    /// 
    /// Refers instance of *View*.
    pub async fn reset_list_view_from_view(&self, view: &ManagedObjectReference) -> Result<()> {
        let input = ResetListViewFromViewRequestType {view, };
        let path = format!("/ListView/{moId}/ResetListViewFromView", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// The list of references to objects mapped by this view.
    pub async fn view(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/ListView/{moId}/view", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ModifyListViewRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    add: Option<&'a [ManagedObjectReference]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    remove: Option<&'a [ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ResetListViewRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    obj: Option<&'a [ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ResetListViewFromViewRequestType<'a> {
    view: &'a ManagedObjectReference,
}
