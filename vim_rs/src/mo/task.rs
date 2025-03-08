use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::CustomFieldDef;
use crate::types::structs::LocalizableMessage;
use crate::types::structs::MethodFault;
use crate::types::structs::TaskInfo;
use crate::types::vim_any::VimAny;
/// A task is used to monitor and potentially cancel long
/// running operations.
pub struct Task {
    client: Arc<Client>,
    mo_id: String,
}
impl Task {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Updates task description to describe the current phase of the task.
    /// 
    /// ***Required privileges:*** Task.Update
    ///
    /// ## Parameters:
    ///
    /// ### description
    /// New description for task
    pub async fn set_task_description(&self, description: &LocalizableMessage) -> Result<()> {
        let input = SetTaskDescriptionRequestType {description, };
        let path = format!("/Task/{moId}/SetTaskDescription", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Sets percentage done for this task and recalculates overall
    /// percentage done.
    /// 
    /// If a percentDone value of less than zero or
    /// greater than 100 is specified, a value of zero or 100
    /// respectively is used.
    /// 
    /// ***Required privileges:*** Task.Update
    ///
    /// ## Parameters:
    ///
    /// ### percent_done
    /// Percentage to set for this task
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: If task is not running
    /// 
    /// ***OutOfBounds***: VirtualCenter 2.x servers throw
    /// this fault if percentDone is less than 0 or greater than 100. Newer
    /// versions behave as described above, and never throw this fault.
    pub async fn update_progress(&self, percent_done: i32) -> Result<()> {
        let input = UpdateProgressRequestType {percent_done, };
        let path = format!("/Task/{moId}/UpdateProgress", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Cancels a running or queued task.
    /// 
    /// A task may only be canceled if it is
    /// cancelable. Multiple cancel requests will be treated as a single
    /// cancelation request. Canceling a completed or already canceled task
    /// will throw an InvalidState exception.
    /// 
    /// If a task is canceled, its runtime state will be set to error and its
    /// error state will be set to *RequestCanceled*.
    /// 
    /// A cancel operation is asynchronous. The operation may return before
    /// the task is canceled.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: If the task is not cancelable.
    /// 
    /// ***InvalidState***: If the task is already canceled or completed.
    pub async fn cancel_task(&self) -> Result<()> {
        let path = format!("/Task/{moId}/CancelTask", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
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
        let path = format!("/Task/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Sets task state and optionally sets results or fault,
    /// as appropriate for state
    /// 
    /// ***Required privileges:*** Task.Update
    ///
    /// ## Parameters:
    ///
    /// ### state
    /// New state for task
    ///
    /// ### result
    /// Result to set, valid only if task state is
    /// TaskInfo.State.success
    ///
    /// ### fault
    /// Fault to set, valid only if task state is
    /// *error*. The fault must be a of a fault type that
    /// directly or indirectly extends *VimFault*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: If attempting to change states after
    /// task is completed or in error, or attempting to set the
    /// result or fault incorrectly
    pub async fn set_task_state(&self, state: crate::types::enums::TaskInfoStateEnum, result: Option<VimAny>, fault: Option<&MethodFault>) -> Result<()> {
        let input = SetTaskStateRequestType {state, result, fault, };
        let path = format!("/Task/{moId}/SetTaskState", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/Task/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// Detailed information about this task.
    pub async fn info(&self) -> Result<TaskInfo> {
        let path = format!("/Task/{moId}/info", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// List of custom field values.
    /// 
    /// Each value uses a key to associate
    /// an instance of a *CustomFieldStringValue* with
    /// a custom field definition.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>> {
        let path = format!("/Task/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetTaskDescriptionRequestType<'a> {
    description: &'a LocalizableMessage,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateProgressRequestType {
    #[serde(rename = "percentDone")]
    percent_done: i32,
}
#[derive(serde::Serialize)]
#[serde(rename = "setCustomValueRequestType", tag = "_typeName")]
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetTaskStateRequestType<'a> {
    state: crate::types::enums::TaskInfoStateEnum,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    result: Option<VimAny>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    fault: Option<&'a MethodFault>,
}
