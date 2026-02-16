use std::sync::Arc;
use crate::core::client::{VimClient, Result};
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
#[derive(Clone)]
pub struct ListView {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl ListView {
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
    pub async fn modify_list_view(&self, add: Option<&[crate::types::structs::ManagedObjectReference]>, remove: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let input = ModifyListViewRequestType {add, remove, };
        let path = format!("/ListView/{moId}/ModifyListView", moId = &self.mo_id);
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
    pub async fn reset_list_view(&self, obj: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let input = ResetListViewRequestType {obj, };
        let path = format!("/ListView/{moId}/ResetListView", moId = &self.mo_id);
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
    /// Replaces the list with the set of objects in a given view.
    ///
    /// ## Parameters:
    ///
    /// ### view
    /// The view to copy objects from.
    /// 
    /// Refers instance of *View*.
    pub async fn reset_list_view_from_view(&self, view: &crate::types::structs::ManagedObjectReference) -> Result<()> {
        let input = ResetListViewFromViewRequestType {view, };
        let path = format!("/ListView/{moId}/ResetListViewFromView", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// The list of references to objects mapped by this view.
    pub async fn view(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let path = format!("/ListView/{moId}/view", moId = &self.mo_id);
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
struct ModifyListViewRequestType<'a> {
    add: Option<&'a [crate::types::structs::ManagedObjectReference]>,
    remove: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for ModifyListViewRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ModifyListViewRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ModifyListViewRequestTypeSer<'b, 'a> {
    data: &'b ModifyListViewRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ModifyListViewRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ModifyListViewRequestType")),
                1 => {
                    let Some(ref val) = self.data.add else { continue; };
                    return Some((std::borrow::Cow::Borrowed("add"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.remove else { continue; };
                    return Some((std::borrow::Cow::Borrowed("remove"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ResetListViewRequestType<'a> {
    obj: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for ResetListViewRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ResetListViewRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ResetListViewRequestTypeSer<'b, 'a> {
    data: &'b ResetListViewRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ResetListViewRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ResetListViewRequestType")),
                1 => {
                    let Some(ref val) = self.data.obj else { continue; };
                    return Some((std::borrow::Cow::Borrowed("obj"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ResetListViewFromViewRequestType<'a> {
    view: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for ResetListViewFromViewRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ResetListViewFromViewRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ResetListViewFromViewRequestTypeSer<'b, 'a> {
    data: &'b ResetListViewFromViewRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ResetListViewFromViewRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ResetListViewFromViewRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("view"), &self.data.view as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
