use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::ManagedObjectReference;
use crate::types::CustomFieldDef;
use crate::types::PrivilegePolicyDef;
/// The CustomFieldsManager object is used to add and remove custom fields
/// to managed entities.
/// 
/// The custom fields values set on managed entities are available through the
/// *ManagedEntity.customValue* property and through the summary objects
/// for *VirtualMachine* and *HostSystem*. They are not available
/// directly through this managed object.
/// 
/// This functionality is only available through VirtualCenter.
pub struct CustomFieldsManager {
    client: Arc<VimClient>,
    mo_id: String,
}
impl CustomFieldsManager {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
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
    pub async fn add_custom_field_def(&self, name: &str, mo_type: Option<&str>, field_def_policy: Option<&PrivilegePolicyDef>, field_policy: Option<&PrivilegePolicyDef>) -> Result<CustomFieldDef> {
        let input = AddCustomFieldDefRequestType {name, mo_type, field_def_policy, field_policy, };
        let path = format!("/CustomFieldsManager/{moId}/AddCustomFieldDef", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn set_field(&self, entity: &ManagedObjectReference, key: i32, value: &str) -> Result<()> {
        let input = SetFieldRequestType {entity, key, value, };
        let path = format!("/CustomFieldsManager/{moId}/SetField", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// List of custom fields defined on this server.
    /// 
    /// The fields are
    /// sorted by name.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/CustomFieldsManager/{moId}/field", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddCustomFieldDefRequestType<'a> {
    name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "moType")]
    mo_type: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fieldDefPolicy")]
    field_def_policy: Option<&'a PrivilegePolicyDef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fieldPolicy")]
    field_policy: Option<&'a PrivilegePolicyDef>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveCustomFieldDefRequestType {
    key: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RenameCustomFieldDefRequestType<'a> {
    key: i32,
    name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetFieldRequestType<'a> {
    entity: &'a ManagedObjectReference,
    key: i32,
    value: &'a str,
}
