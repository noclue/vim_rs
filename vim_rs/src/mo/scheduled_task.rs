use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The scheduled task object.
#[derive(Clone)]
pub struct ScheduledTask {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl ScheduledTask {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn reconfigure_scheduled_task(&self, spec: &dyn crate::types::traits::ScheduledTaskSpecTrait) -> Result<()> {
        let input = ReconfigureScheduledTaskRequestType {spec, };
        self.client.invoke_void("", "ScheduledTask", &self.mo_id, "ReconfigureScheduledTask", Some(&input)).await
    }
    /// Removes the scheduled task.
    /// 
    /// ***Required privileges:*** ScheduledTask.Delete
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the scheduled task is running.
    pub async fn remove_scheduled_task(&self) -> Result<()> {
        self.client.invoke_void("", "ScheduledTask", &self.mo_id, "RemoveScheduledTask", None).await
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
        self.client.invoke_void("", "ScheduledTask", &self.mo_id, "RunScheduledTask", None).await
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
        self.client.invoke_void("", "ScheduledTask", &self.mo_id, "setCustomValue", Some(&input)).await
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<crate::types::structs::CustomFieldDef>>> {
        let pv_opt = self.client.fetch_property_raw("", "ScheduledTask", &self.mo_id, "availableField").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Information about the current scheduled task.
    pub async fn info(&self) -> Result<crate::types::structs::ScheduledTaskInfo> {
        let pv_opt = self.client.fetch_property_raw("", "ScheduledTask", &self.mo_id, "info").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property info was empty".to_string()))?;
        let result: crate::types::structs::ScheduledTaskInfo = crate::core::client::extract_property(pv)?;
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
        let pv_opt = self.client.fetch_property_raw("", "ScheduledTask", &self.mo_id, "value").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
}
struct ReconfigureScheduledTaskRequestType<'a> {
    spec: &'a dyn crate::types::traits::ScheduledTaskSpecTrait,
}

impl<'a> miniserde::Serialize for ReconfigureScheduledTaskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReconfigureScheduledTaskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReconfigureScheduledTaskRequestTypeSer<'b, 'a> {
    data: &'b ReconfigureScheduledTaskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ReconfigureScheduledTaskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReconfigureScheduledTaskRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
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

impl<'b, 'a> miniserde::ser::Map for SetCustomValueRequestTypeSer<'b, 'a> {
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
