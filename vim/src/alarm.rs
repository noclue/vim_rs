use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::AlarmInfo;
use crate::types::AlarmSpecTrait;
use crate::types::CustomFieldDef;
use crate::types::CustomFieldValueTrait;
/// This managed object type defines an alarm that is triggered and
/// an action that occurs due to the triggered alarm when certain conditions
/// are met on a specific *ManagedEntity* object.
pub struct Alarm {
    client: Arc<VimClient>,
    mo_id: String,
}
impl Alarm {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Reconfigures the alarm properties.
    /// 
    /// This operation requires access
    /// privileges on the entity with which the alarm is associated.
    /// 
    /// In addition to the Alarm.Edit privilege, may also require the
    /// Global.ScriptAction if a RunScriptAction action is specified in
    /// the AlarmSpec.
    /// 
    /// ***Required privileges:*** Alarm.Edit
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// The new specification for the alarm.
    ///
    /// ## Errors:
    ///
    /// ***InvalidName***: if the alarm name is empty or too long.
    /// 
    /// ***DuplicateName***: if an alarm with the name already exists.
    /// 
    /// ***InvalidArgument***: if the specification is invalid.
    pub async fn reconfigure_alarm(&self, spec: &dyn AlarmSpecTrait) -> Result<()> {
        let input = ReconfigureAlarmRequestType {spec, };
        let path = format!("/Alarm/{moId}/ReconfigureAlarm", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Removes the alarm.
    /// 
    /// ***Required privileges:*** Alarm.Delete
    pub async fn remove_alarm(&self) -> Result<()> {
        let path = format!("/Alarm/{moId}/RemoveAlarm", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Assigns a value to a custom field.
    /// 
    /// The setCustomValue method requires
    /// whichever updatePrivilege is defined as one of the
    /// *CustomFieldDef.fieldInstancePrivileges*
    /// for the CustomFieldDef whose value is being changed.
    ///
    /// ## Parameters:
    ///
    /// ### key
    /// The name of the field whose value is to be updated.
    ///
    /// ### value
    /// Value to be assigned to the custom field.
    pub async fn set_custom_value(&self, key: &str, value: &str) -> Result<()> {
        let input = SetCustomValueRequestType {key, value, };
        let path = format!("/Alarm/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/Alarm/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Information about this alarm.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn info(&self) -> Result<AlarmInfo> {
        let path = format!("/Alarm/{moId}/info", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// List of custom field values.
    /// 
    /// Each value uses a key to associate
    /// an instance of a *CustomFieldStringValue* with
    /// a custom field definition.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn CustomFieldValueTrait>>>> {
        let path = format!("/Alarm/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReconfigureAlarmRequestType<'a> {
    spec: &'a dyn AlarmSpecTrait,
}
#[derive(serde::Serialize)]
#[serde(rename = "setCustomValueRequestType", tag = "_typeName")]
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}
