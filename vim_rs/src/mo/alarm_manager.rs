use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The alarm manager is a singleton object for managing alarms
/// within a service instance.
#[derive(Clone)]
pub struct AlarmManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl AlarmManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Acknowledge the alarm on a managed entity.
    /// 
    /// The actions associated
    /// with the alarm will not fire until the alarm's next distinct
    /// occurrence; that is, until after the alarm has entered the green
    /// or gray states at least once. Calling this method on an acknowledged
    /// or non-triggered alarm.
    ///
    /// ## Parameters:
    ///
    /// ### alarm
    /// The Alarm to acknowledge.
    /// 
    /// ***Required privileges:*** Alarm.Acknowledge
    /// 
    /// Refers instance of *Alarm*.
    ///
    /// ### entity
    /// The ManagedEntity for which to acknowledge the Alarm.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ManagedEntity*.
    pub async fn acknowledge_alarm(&self, alarm: &crate::types::structs::ManagedObjectReference, entity: &crate::types::structs::ManagedObjectReference) -> Result<()> {
        let input = AcknowledgeAlarmRequestType {alarm, entity, };
        self.client.invoke_void("", "AlarmManager", &self.mo_id, "AcknowledgeAlarm", Some(&input)).await
    }
    /// Resets all triggered alarms to green.
    /// 
    /// Should be used when mass alarm reset is needed.
    /// 
    /// ***Required privileges:*** Alarm.SetStatus
    ///
    /// ## Parameters:
    ///
    /// ### filter
    /// -
    pub async fn clear_triggered_alarms(&self, filter: &crate::types::structs::AlarmFilterSpec) -> Result<()> {
        let input = ClearTriggeredAlarmsRequestType {filter, };
        self.client.invoke_void("", "AlarmManager", &self.mo_id, "ClearTriggeredAlarms", Some(&input)).await
    }
    /// Creates an alarm.
    /// 
    /// In addition to the Alarm.Create privilege, may also require the
    /// Global.ScriptAction if a RunScriptAction action is specified in
    /// the AlarmSpec.
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// The entity with which the alarm is associated.
    /// 
    /// ***Required privileges:*** Alarm.Create
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ### spec
    /// The specification for the new alarm.
    ///
    /// ## Returns:
    ///
    /// A reference to the Alarm object created by the
    /// operation.
    /// 
    /// Refers instance of *Alarm*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidName***: if the alarm name is empty or too long.
    /// 
    /// ***DuplicateName***: if an alarm with the name already exists.
    /// 
    /// ***InvalidArgument***: if the specification is invalid.
    pub async fn create_alarm(&self, entity: &crate::types::structs::ManagedObjectReference, spec: &dyn crate::types::traits::AlarmSpecTrait) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateAlarmRequestType {entity, spec, };
        let bytes = self.client.invoke("", "AlarmManager", &self.mo_id, "CreateAlarm", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Disables alarm for a specific entity.
    ///
    /// ## Parameters:
    ///
    /// ### alarm
    /// The Alarm being disabled.
    /// 
    /// ***Required privileges:*** Alarm.ToggleEnableOnEntity
    /// 
    /// Refers instance of *Alarm*.
    ///
    /// ### entity
    /// The ManagedEntity on which to disable the alarm.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ManagedEntity*.
    pub async fn disable_alarm(&self, alarm: &crate::types::structs::ManagedObjectReference, entity: &crate::types::structs::ManagedObjectReference) -> Result<()> {
        let input = DisableAlarmRequestType {alarm, entity, };
        self.client.invoke_void("", "AlarmManager", &self.mo_id, "DisableAlarm", Some(&input)).await
    }
    /// Enables alarm for a specific entity.
    ///
    /// ## Parameters:
    ///
    /// ### alarm
    /// The Alarm being enabled.
    /// 
    /// ***Required privileges:*** Alarm.ToggleEnableOnEntity
    /// 
    /// Refers instance of *Alarm*.
    ///
    /// ### entity
    /// The ManagedEntity on which to enable the alarm.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ManagedEntity*.
    pub async fn enable_alarm(&self, alarm: &crate::types::structs::ManagedObjectReference, entity: &crate::types::structs::ManagedObjectReference) -> Result<()> {
        let input = EnableAlarmRequestType {alarm, entity, };
        self.client.invoke_void("", "AlarmManager", &self.mo_id, "EnableAlarm", Some(&input)).await
    }
    /// Available alarms defined on the entity.
    /// 
    /// These alarms do not include any inherited alarms; that is,
    /// alarms associated with parent entities.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// The entity. If not set, alarms are returned for
    /// all visible entities.
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ## Returns:
    ///
    /// A reference to the Alarm objects returned by the
    /// operation.
    /// 
    /// Refers instances of *Alarm*.
    pub async fn get_alarm(&self, entity: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let input = GetAlarmRequestType {entity, };
        let bytes_opt = self.client.invoke_optional("", "AlarmManager", &self.mo_id, "GetAlarm", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Returns true if alarm actions are enabled on the specified managed entity.
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// The managed entity to look up.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ManagedEntity*.
    pub async fn are_alarm_actions_enabled(&self, entity: &crate::types::structs::ManagedObjectReference) -> Result<bool> {
        let input = AreAlarmActionsEnabledRequestType {entity, };
        let bytes = self.client.invoke("", "AlarmManager", &self.mo_id, "AreAlarmActionsEnabled", Some(&input)).await?;
        let result: bool = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// The state of instantiated alarms on the entity.
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// The entity.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ## Returns:
    ///
    /// The state of instantiated alarms.
    pub async fn get_alarm_state(&self, entity: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<crate::types::structs::AlarmState>>> {
        let input = GetAlarmStateRequestType {entity, };
        let bytes_opt = self.client.invoke_optional("", "AlarmManager", &self.mo_id, "GetAlarmState", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Enables or disables alarms on the specified managed entity.
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// The managed entity on which to set a schedule.
    /// 
    /// ***Required privileges:*** Alarm.DisableActions
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ### enabled
    /// true, if alarms are enabled during the schedule.
    pub async fn enable_alarm_actions(&self, entity: &crate::types::structs::ManagedObjectReference, enabled: bool) -> Result<()> {
        let input = EnableAlarmActionsRequestType {entity, enabled, };
        self.client.invoke_void("", "AlarmManager", &self.mo_id, "EnableAlarmActions", Some(&input)).await
    }
    /// The default setting for each alarm expression, used to populate the
    /// initial client wizard screen.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn default_expression(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::AlarmExpressionTrait>>>> {
        let bytes_opt = self.client.fetch_property_raw("", "AlarmManager", &self.mo_id, "defaultExpression").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// The static descriptive strings used in alarms.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn description(&self) -> Result<crate::types::structs::AlarmDescription> {
        let bytes_opt = self.client.fetch_property_raw("", "AlarmManager", &self.mo_id, "description").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property description was empty".to_string()))?;
        let result: crate::types::structs::AlarmDescription = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct AcknowledgeAlarmRequestType<'a> {
    alarm: &'a crate::types::structs::ManagedObjectReference,
    entity: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for AcknowledgeAlarmRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AcknowledgeAlarmRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AcknowledgeAlarmRequestTypeSer<'b, 'a> {
    data: &'b AcknowledgeAlarmRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AcknowledgeAlarmRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AcknowledgeAlarmRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("alarm"), &self.data.alarm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ClearTriggeredAlarmsRequestType<'a> {
    filter: &'a crate::types::structs::AlarmFilterSpec,
}

impl<'a> miniserde::Serialize for ClearTriggeredAlarmsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ClearTriggeredAlarmsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ClearTriggeredAlarmsRequestTypeSer<'b, 'a> {
    data: &'b ClearTriggeredAlarmsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ClearTriggeredAlarmsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ClearTriggeredAlarmsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("filter"), &self.data.filter as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateAlarmRequestType<'a> {
    entity: &'a crate::types::structs::ManagedObjectReference,
    spec: &'a dyn crate::types::traits::AlarmSpecTrait,
}

impl<'a> miniserde::Serialize for CreateAlarmRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateAlarmRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateAlarmRequestTypeSer<'b, 'a> {
    data: &'b CreateAlarmRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateAlarmRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateAlarmRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DisableAlarmRequestType<'a> {
    alarm: &'a crate::types::structs::ManagedObjectReference,
    entity: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for DisableAlarmRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DisableAlarmRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DisableAlarmRequestTypeSer<'b, 'a> {
    data: &'b DisableAlarmRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DisableAlarmRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DisableAlarmRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("alarm"), &self.data.alarm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct EnableAlarmRequestType<'a> {
    alarm: &'a crate::types::structs::ManagedObjectReference,
    entity: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for EnableAlarmRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(EnableAlarmRequestTypeSer { data: self, seq: 0 }))
    }
}

struct EnableAlarmRequestTypeSer<'b, 'a> {
    data: &'b EnableAlarmRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for EnableAlarmRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"EnableAlarmRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("alarm"), &self.data.alarm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct GetAlarmRequestType<'a> {
    entity: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for GetAlarmRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(GetAlarmRequestTypeSer { data: self, seq: 0 }))
    }
}

struct GetAlarmRequestTypeSer<'b, 'a> {
    data: &'b GetAlarmRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for GetAlarmRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"GetAlarmRequestType")),
                1 => {
                    let Some(ref val) = self.data.entity else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entity"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct AreAlarmActionsEnabledRequestType<'a> {
    entity: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for AreAlarmActionsEnabledRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AreAlarmActionsEnabledRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AreAlarmActionsEnabledRequestTypeSer<'b, 'a> {
    data: &'b AreAlarmActionsEnabledRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AreAlarmActionsEnabledRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AreAlarmActionsEnabledRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct GetAlarmStateRequestType<'a> {
    entity: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for GetAlarmStateRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(GetAlarmStateRequestTypeSer { data: self, seq: 0 }))
    }
}

struct GetAlarmStateRequestTypeSer<'b, 'a> {
    data: &'b GetAlarmStateRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for GetAlarmStateRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"GetAlarmStateRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct EnableAlarmActionsRequestType<'a> {
    entity: &'a crate::types::structs::ManagedObjectReference,
    enabled: bool,
}

impl<'a> miniserde::Serialize for EnableAlarmActionsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(EnableAlarmActionsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct EnableAlarmActionsRequestTypeSer<'b, 'a> {
    data: &'b EnableAlarmActionsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for EnableAlarmActionsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"EnableAlarmActionsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("enabled"), &self.data.enabled as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
