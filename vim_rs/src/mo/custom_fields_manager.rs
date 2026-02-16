use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The CustomFieldsManager object is used to add and remove custom fields
/// to managed entities.
/// 
/// The custom fields values set on managed entities are available through the
/// *ManagedEntity.customValue* property and through the summary objects
/// for *VirtualMachine* and *HostSystem*. They are not available
/// directly through this managed object.
/// 
/// This functionality is only available through VirtualCenter.
#[derive(Clone)]
pub struct CustomFieldsManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl CustomFieldsManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Creates a new custom field.
    /// 
    /// If the moType is specified, the
    /// field will only be available for that type of managed object.
    /// 
    /// ***Required privileges:*** Global.ManageCustomFields
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of the field.
    ///
    /// ### mo_type
    /// The managed object type to which this field
    /// will apply
    ///
    /// ### field_def_policy
    /// Privilege policy to apply to FieldDef being
    /// created
    ///
    /// ### field_policy
    /// Privilege policy to apply to instances of field
    ///
    /// ## Errors:
    ///
    /// ***DuplicateName***: if a custom field with the name already exists.
    /// 
    /// ***InvalidPrivilege***: if a specified privilege is not defined.
    pub async fn add_custom_field_def(&self, name: &str, mo_type: Option<&str>, field_def_policy: Option<&crate::types::structs::PrivilegePolicyDef>, field_policy: Option<&crate::types::structs::PrivilegePolicyDef>) -> Result<crate::types::structs::CustomFieldDef> {
        let input = AddCustomFieldDefRequestType {name, mo_type, field_def_policy, field_policy, };
        let path = format!("/CustomFieldsManager/{moId}/AddCustomFieldDef", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::CustomFieldDef = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Removes a custom field.
    /// 
    /// This also removes all values assigned
    /// to this custom field.
    /// 
    /// ***Required privileges:*** Global.ManageCustomFields
    ///
    /// ## Parameters:
    ///
    /// ### key
    /// The unique key for the field definition.
    pub async fn remove_custom_field_def(&self, key: i32) -> Result<()> {
        let input = RemoveCustomFieldDefRequestType {key, };
        let path = format!("/CustomFieldsManager/{moId}/RemoveCustomFieldDef", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Renames a custom field.
    /// 
    /// ***Required privileges:*** Global.ManageCustomFields
    ///
    /// ## Parameters:
    ///
    /// ### key
    /// The unique key for the field definition.
    ///
    /// ### name
    /// The new name for the field.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if no custom field with that key exists.
    /// 
    /// ***DuplicateName***: if a custom field with the name already exists.
    pub async fn rename_custom_field_def(&self, key: i32, name: &str) -> Result<()> {
        let input = RenameCustomFieldDefRequestType {key, name, };
        let path = format!("/CustomFieldsManager/{moId}/RenameCustomFieldDef", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Assigns a value to a custom field on an entity.
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// ***Required privileges:*** Global.SetCustomField
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ### key
    /// -
    ///
    /// ### value
    /// -
    pub async fn set_field(&self, entity: &crate::types::structs::ManagedObjectReference, key: i32, value: &str) -> Result<()> {
        let input = SetFieldRequestType {entity, key, value, };
        let path = format!("/CustomFieldsManager/{moId}/SetField", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// List of custom fields defined on this server.
    /// 
    /// The fields are
    /// sorted by name.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn field(&self) -> Result<Option<Vec<crate::types::structs::CustomFieldDef>>> {
        let path = format!("/CustomFieldsManager/{moId}/field", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::CustomFieldDef>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
}
struct AddCustomFieldDefRequestType<'a> {
    name: &'a str,
    mo_type: Option<&'a str>,
    field_def_policy: Option<&'a crate::types::structs::PrivilegePolicyDef>,
    field_policy: Option<&'a crate::types::structs::PrivilegePolicyDef>,
}

impl<'a> miniserde::Serialize for AddCustomFieldDefRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AddCustomFieldDefRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AddCustomFieldDefRequestTypeSer<'b, 'a> {
    data: &'b AddCustomFieldDefRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AddCustomFieldDefRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AddCustomFieldDefRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.mo_type else { continue; };
                    return Some((std::borrow::Cow::Borrowed("moType"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.field_def_policy else { continue; };
                    return Some((std::borrow::Cow::Borrowed("fieldDefPolicy"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.field_policy else { continue; };
                    return Some((std::borrow::Cow::Borrowed("fieldPolicy"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RemoveCustomFieldDefRequestType {
    key: i32,
}

impl miniserde::Serialize for RemoveCustomFieldDefRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveCustomFieldDefRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveCustomFieldDefRequestTypeSer<'b> {
    data: &'b RemoveCustomFieldDefRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for RemoveCustomFieldDefRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveCustomFieldDefRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("key"), &self.data.key as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RenameCustomFieldDefRequestType<'a> {
    key: i32,
    name: &'a str,
}

impl<'a> miniserde::Serialize for RenameCustomFieldDefRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RenameCustomFieldDefRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RenameCustomFieldDefRequestTypeSer<'b, 'a> {
    data: &'b RenameCustomFieldDefRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RenameCustomFieldDefRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RenameCustomFieldDefRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("key"), &self.data.key as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct SetFieldRequestType<'a> {
    entity: &'a crate::types::structs::ManagedObjectReference,
    key: i32,
    value: &'a str,
}

impl<'a> miniserde::Serialize for SetFieldRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetFieldRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetFieldRequestTypeSer<'b, 'a> {
    data: &'b SetFieldRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for SetFieldRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SetFieldRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("key"), &self.data.key as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("value"), &self.data.value as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
