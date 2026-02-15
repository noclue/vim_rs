use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// A task is used to monitor and potentially cancel long
/// running operations.
#[derive(Clone)]
pub struct Task {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl Task {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn set_task_description(&self, description: &crate::types::structs::LocalizableMessage) -> Result<()> {
        let input = SetTaskDescriptionRequestType {description, };
        let path = format!("/Task/{moId}/SetTaskDescription", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
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
        let req = self.client.post_json(&path, &input);
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
        let req = self.client.post_json(&path, &input);
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
    pub async fn set_task_state(&self, state: crate::types::enums::TaskInfoStateEnum, result: Option<crate::types::vim_any::VimAny>, fault: Option<&crate::types::structs::MethodFault>) -> Result<()> {
        let input = SetTaskStateRequestType {state, result, fault, };
        let path = format!("/Task/{moId}/SetTaskState", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<crate::types::structs::CustomFieldDef>>> {
        let path = format!("/Task/{moId}/availableField", moId = &self.mo_id);
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
    /// Detailed information about this task.
    pub async fn info(&self) -> Result<crate::types::structs::TaskInfo> {
        let path = format!("/Task/{moId}/info", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::TaskInfo = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
}
struct SetTaskDescriptionRequestType<'a> {
    description: &'a crate::types::structs::LocalizableMessage,
}

impl<'a> miniserde::Serialize for SetTaskDescriptionRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetTaskDescriptionRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetTaskDescriptionRequestTypeSer<'b, 'a> {
    data: &'b SetTaskDescriptionRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for SetTaskDescriptionRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SetTaskDescriptionRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("description"), &self.data.description as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateProgressRequestType {
    percent_done: i32,
}

impl miniserde::Serialize for UpdateProgressRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateProgressRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateProgressRequestTypeSer<'b> {
    data: &'b UpdateProgressRequestType,
    seq: usize,
}

impl miniserde::ser::Map for UpdateProgressRequestTypeSer<'_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateProgressRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("percentDone"), &self.data.percent_done as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}

impl<'a> miniserde::Serialize for SetCustomValueRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetCustomValueRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetCustomValueRequestTypeSer<'b, 'a> {
    data: &'b SetCustomValueRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for SetCustomValueRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"setCustomValueRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("key"), &self.data.key as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("value"), &self.data.value as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct SetTaskStateRequestType<'a> {
    state: crate::types::enums::TaskInfoStateEnum,
    result: Option<crate::types::vim_any::VimAny>,
    fault: Option<&'a crate::types::structs::MethodFault>,
}

impl<'a> miniserde::Serialize for SetTaskStateRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetTaskStateRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetTaskStateRequestTypeSer<'b, 'a> {
    data: &'b SetTaskStateRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for SetTaskStateRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SetTaskStateRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("state"), &self.data.state as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.result else { continue; };
                    return Some((std::borrow::Cow::Borrowed("result"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.fault else { continue; };
                    return Some((std::borrow::Cow::Borrowed("fault"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
