use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::CustomFieldDef;
use crate::types::CustomFieldValueTrait;
use crate::types::ScheduledTaskInfo;
use crate::types::ScheduledTaskSpecTrait;
/// The scheduled task object.
pub struct ScheduledTask {
    client: Arc<VimClient>,
    mo_id: String,
}
impl ScheduledTask {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Reconfigures the scheduled task properties.
    /// 
    /// ***Required privileges:*** ScheduledTask.Edit
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// The new specification for the scheduled task.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the scheduled task is running.
    /// 
    /// ***InvalidName***: if the scheduled task name is empty or too long.
    /// 
    /// ***DuplicateName***: if a scheduled task with the name already exists.
    /// 
    /// ***InvalidArgument***: if the specification is invalid.
    pub async fn reconfigure_scheduled_task(&self, spec: &dyn ScheduledTaskSpecTrait) -> Result<()> {
        let input = ReconfigureScheduledTaskRequestType {spec, };
        let path = format!("/ScheduledTask/{moId}/ReconfigureScheduledTask", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Removes the scheduled task.
    /// 
    /// ***Required privileges:*** ScheduledTask.Delete
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the scheduled task is running.
    pub async fn remove_scheduled_task(&self) -> Result<()> {
        let path = format!("/ScheduledTask/{moId}/RemoveScheduledTask", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Runs the scheduled task immediately.
    /// 
    /// The schedule for future runs remains in effect.
    /// 
    /// ***Required privileges:*** ScheduledTask.Run
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the scheduled task is running already.
    pub async fn run_scheduled_task(&self) -> Result<()> {
        let path = format!("/ScheduledTask/{moId}/RunScheduledTask", moId = &self.mo_id);
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
        let path = format!("/ScheduledTask/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/ScheduledTask/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Information about the current scheduled task.
    pub async fn info(&self) -> Result<ScheduledTaskInfo> {
        let path = format!("/ScheduledTask/{moId}/info", moId = &self.mo_id);
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
        let path = format!("/ScheduledTask/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReconfigureScheduledTaskRequestType<'a> {
    spec: &'a dyn ScheduledTaskSpecTrait,
}
#[derive(serde::Serialize)]
#[serde(rename = "setCustomValueRequestType", tag = "_typeName")]
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}
