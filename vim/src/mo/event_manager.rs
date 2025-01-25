use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::EventArgDesc;
use crate::types::structs::EventDescription;
use crate::types::structs::EventFilterSpec;
use crate::types::structs::EventTrait;
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::TaskInfo;
/// This managed object type provides properties and methods for
/// event management support.
/// 
/// Event objects are used to record significant state changes of
/// managed entities.
pub struct EventManager {
    client: Arc<Client>,
    mo_id: String,
}
impl EventManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Returns the events in specified filter.
    /// 
    /// Returns empty array when there are not any events qualified.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### filter
    /// The events qualified.
    ///
    /// ## Returns:
    ///
    /// The events matching the filter.
    pub async fn query_events(&self, filter: &EventFilterSpec) -> Result<Option<Vec<Box<dyn EventTrait>>>> {
        let input = QueryEventsRequestType {filter, };
        let path = format!("/EventManager/{moId}/QueryEvents", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Creates an event history collector, which is a specialized history collector
    /// that provides Event objects.
    /// 
    /// Event collectors do not persist beyond the current client session.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### filter
    /// The event query filter.
    ///
    /// ## Returns:
    ///
    /// The event collector based on the filter.
    /// 
    /// Refers instance of *EventHistoryCollector*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the filter is null or if any of its fields
    /// is invalid, such as an invalid reference to a
    /// managed object, alarm, or scheduled task, or an
    /// invalid event type or event chain id, etc.
    /// 
    /// ***InvalidState***: if there are more than the maximum number of
    /// event collectors.
    pub async fn create_collector_for_events(&self, filter: &EventFilterSpec) -> Result<ManagedObjectReference> {
        let input = CreateCollectorForEventsRequestType {filter, };
        let path = format!("/EventManager/{moId}/CreateCollectorForEvents", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Logs a user defined event against a particular managed entity.
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// The entity against which the event is logged. The entity must be
    /// the root folder, a DataCenter, a VirtualMachine, a HostSystem,
    /// or a ComputeResource.
    /// 
    /// ***Required privileges:*** Global.LogEvent
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ### msg
    /// The message to be logged.
    pub async fn log_user_event(&self, entity: &ManagedObjectReference, msg: &str) -> Result<()> {
        let input = LogUserEventRequestType {entity, msg, };
        let path = format!("/EventManager/{moId}/LogUserEvent", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Posts the specified event, optionally associating it with
    /// a task.
    /// 
    /// The event being posted should have the following info in it:
    /// - The *ManagedEntity* on which the event is being posted should
    ///   be set in the appropriate *EntityEventArgument* field of the base
    ///   *Event* class. It is OK to not set any entity, in which case the
    ///   event is treated as an event about the system.
    /// - Some Event fields (*Event.key*, *Event.chainId*,
    ///   *Event.createdTime*) are mandatory because of the nature of
    ///   the structure, but any caller-supplied values will be overwritten by
    ///   the system.
    ///   
    /// If the event being posted is to be associated with an existing
    /// *Task*, the appropriate *TaskInfo* needs to be passed in.
    /// This task can either be one returned from a vSphere API operation or
    /// an extension task created by calling *TaskManager.CreateTask*.
    /// 
    /// ***Required privileges:*** Global.LogEvent
    ///
    /// ## Parameters:
    ///
    /// ### event_to_post
    /// Fully-specified event to post
    ///
    /// ### task_info
    /// optional task associated with the event
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if
    /// - an invalid reference to a managed object is passed in to one of the
    ///   *EntityEventArgument* fields
    /// - an invalid severity value is passed in an *EventEx*.
    ///   
    /// ***InvalidEvent***: no longer thrown by this API
    pub async fn post_event(&self, event_to_post: &dyn EventTrait, task_info: Option<&TaskInfo>) -> Result<()> {
        let input = PostEventRequestType {event_to_post, task_info, };
        let path = format!("/EventManager/{moId}/PostEvent", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Retrieves the argument meta-data for a given Event type
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### event_type_id
    /// -
    pub async fn retrieve_argument_description(&self, event_type_id: &str) -> Result<Option<Vec<EventArgDesc>>> {
        let input = RetrieveArgumentDescriptionRequestType {event_type_id, };
        let path = format!("/EventManager/{moId}/RetrieveArgumentDescription", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Static descriptive strings used in events.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn description(&self) -> Result<EventDescription> {
        let path = format!("/EventManager/{moId}/description", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// The latest event that happened on the VirtualCenter server.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn latest_event(&self) -> Result<Option<Box<dyn EventTrait>>> {
        let path = format!("/EventManager/{moId}/latestEvent", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// For each client, the maximum number of event collectors that can exist
    /// simultaneously.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn max_collector(&self) -> Result<i32> {
        let path = format!("/EventManager/{moId}/maxCollector", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryEventsRequestType<'a> {
    filter: &'a EventFilterSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateCollectorForEventsRequestType<'a> {
    filter: &'a EventFilterSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct LogUserEventRequestType<'a> {
    entity: &'a ManagedObjectReference,
    msg: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PostEventRequestType<'a> {
    #[serde(rename = "eventToPost")]
    event_to_post: &'a dyn EventTrait,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "taskInfo")]
    task_info: Option<&'a TaskInfo>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveArgumentDescriptionRequestType<'a> {
    #[serde(rename = "eventTypeId")]
    event_type_id: &'a str,
}
