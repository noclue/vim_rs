use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object type defines an alarm that is triggered and
/// an action that occurs due to the triggered alarm when certain conditions
/// are met on a specific *ManagedEntity* object.
#[derive(Clone)]
pub struct Alarm {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl Alarm {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn reconfigure_alarm(&self, spec: &dyn crate::types::traits::AlarmSpecTrait) -> Result<()> {
        let input = ReconfigureAlarmRequestType {spec, };
        self.client.invoke_void("", "Alarm", &self.mo_id, "ReconfigureAlarm", Some(&input)).await
    }
    /// Removes the alarm.
    /// 
    /// ***Required privileges:*** Alarm.Delete
    pub async fn remove_alarm(&self) -> Result<()> {
        self.client.invoke_void("", "Alarm", &self.mo_id, "RemoveAlarm", None).await
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
        self.client.invoke_void("", "Alarm", &self.mo_id, "setCustomValue", Some(&input)).await
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<crate::types::structs::CustomFieldDef>>> {
        let bytes_opt = self.client.fetch_property_raw("", "Alarm", &self.mo_id, "availableField").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Information about this alarm.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn info(&self) -> Result<crate::types::structs::AlarmInfo> {
        let bytes_opt = self.client.fetch_property_raw("", "Alarm", &self.mo_id, "info").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property info was empty".to_string()))?;
        let result: crate::types::structs::AlarmInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
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
        let bytes_opt = self.client.fetch_property_raw("", "Alarm", &self.mo_id, "value").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
}
struct ReconfigureAlarmRequestType<'a> {
    spec: &'a dyn crate::types::traits::AlarmSpecTrait,
}

impl<'a> miniserde::Serialize for ReconfigureAlarmRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReconfigureAlarmRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReconfigureAlarmRequestTypeSer<'b, 'a> {
    data: &'b ReconfigureAlarmRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ReconfigureAlarmRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReconfigureAlarmRequestType")),
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
