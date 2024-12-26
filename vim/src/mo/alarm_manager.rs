use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::AlarmDescription;
use crate::types::structs::AlarmExpressionTrait;
use crate::types::structs::AlarmFilterSpec;
use crate::types::structs::AlarmSpecTrait;
use crate::types::structs::AlarmState;
use crate::types::structs::ManagedObjectReference;
/// The alarm manager is a singleton object for managing alarms
/// within a service instance.
pub struct AlarmManager {
    client: Arc<Client>,
    mo_id: String,
}
impl AlarmManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
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
    pub async fn acknowledge_alarm(&self, alarm: &ManagedObjectReference, entity: &ManagedObjectReference) -> Result<()> {
        let input = AcknowledgeAlarmRequestType {alarm, entity, };
        let path = format!("/AlarmManager/{moId}/AcknowledgeAlarm", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn clear_triggered_alarms(&self, filter: &AlarmFilterSpec) -> Result<()> {
        let input = ClearTriggeredAlarmsRequestType {filter, };
        let path = format!("/AlarmManager/{moId}/ClearTriggeredAlarms", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn create_alarm(&self, entity: &ManagedObjectReference, spec: &dyn AlarmSpecTrait) -> Result<ManagedObjectReference> {
        let input = CreateAlarmRequestType {entity, spec, };
        let path = format!("/AlarmManager/{moId}/CreateAlarm", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn disable_alarm(&self, alarm: &ManagedObjectReference, entity: &ManagedObjectReference) -> Result<()> {
        let input = DisableAlarmRequestType {alarm, entity, };
        let path = format!("/AlarmManager/{moId}/DisableAlarm", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn enable_alarm(&self, alarm: &ManagedObjectReference, entity: &ManagedObjectReference) -> Result<()> {
        let input = EnableAlarmRequestType {alarm, entity, };
        let path = format!("/AlarmManager/{moId}/EnableAlarm", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn get_alarm(&self, entity: Option<&ManagedObjectReference>) -> Result<Option<Vec<ManagedObjectReference>>> {
        let input = GetAlarmRequestType {entity, };
        let path = format!("/AlarmManager/{moId}/GetAlarm", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn are_alarm_actions_enabled(&self, entity: &ManagedObjectReference) -> Result<bool> {
        let input = AreAlarmActionsEnabledRequestType {entity, };
        let path = format!("/AlarmManager/{moId}/AreAlarmActionsEnabled", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn get_alarm_state(&self, entity: &ManagedObjectReference) -> Result<Option<Vec<AlarmState>>> {
        let input = GetAlarmStateRequestType {entity, };
        let path = format!("/AlarmManager/{moId}/GetAlarmState", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn enable_alarm_actions(&self, entity: &ManagedObjectReference, enabled: bool) -> Result<()> {
        let input = EnableAlarmActionsRequestType {entity, enabled, };
        let path = format!("/AlarmManager/{moId}/EnableAlarmActions", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// The default setting for each alarm expression, used to populate the
    /// initial client wizard screen.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn default_expression(&self) -> Result<Option<Vec<Box<dyn AlarmExpressionTrait>>>> {
        let path = format!("/AlarmManager/{moId}/defaultExpression", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// The static descriptive strings used in alarms.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn description(&self) -> Result<AlarmDescription> {
        let path = format!("/AlarmManager/{moId}/description", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AcknowledgeAlarmRequestType<'a> {
    alarm: &'a ManagedObjectReference,
    entity: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ClearTriggeredAlarmsRequestType<'a> {
    filter: &'a AlarmFilterSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateAlarmRequestType<'a> {
    entity: &'a ManagedObjectReference,
    spec: &'a dyn AlarmSpecTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DisableAlarmRequestType<'a> {
    alarm: &'a ManagedObjectReference,
    entity: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct EnableAlarmRequestType<'a> {
    alarm: &'a ManagedObjectReference,
    entity: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct GetAlarmRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entity: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AreAlarmActionsEnabledRequestType<'a> {
    entity: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct GetAlarmStateRequestType<'a> {
    entity: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct EnableAlarmActionsRequestType<'a> {
    entity: &'a ManagedObjectReference,
    enabled: bool,
}
