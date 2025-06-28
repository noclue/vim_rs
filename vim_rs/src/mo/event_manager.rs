use std::sync::Arc;
use crate::core::client::{Client, Result};
/// This managed object type provides properties and methods for
/// event management support.
/// 
/// Event objects are used to record significant state changes of
/// managed entities.
#[derive(Clone)]
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
    /// Returns events based on the specified *EventFilterSpec* and
    /// *EventManagerEventViewSpec* parameters.
    /// 
    /// Returns an empty array when there are
    /// no events matching the filtering parameters.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### filter
    /// Defines a filtering criteria for the event query.
    ///
    /// ### event_view_spec
    /// Defines the view parameters for the event query.
    /// 
    /// To retrieve the newest events in the system, use 2147480000
    /// (or the value of "config.vpxd.event.maxEventId" advanced vCenter
    /// configuration option) as *EventManagerViewByStartId.startEventId* and
    /// set *EventManagerViewByStartId.isForward* to false. To retrieve subsequent
    /// pages of results with older events, use the smallest event ID
    /// from the previous response as the *EventManagerViewByStartId.startEventId*
    /// and set*EventManagerViewByStartId.isForward* to false in the next query.
    /// To retrieve subsequent pages of results with newer events, use
    /// the biggest event ID from the previous page as the
    /// *EventManagerViewByStartId.startEventId* and set
    /// *EventManagerViewByStartId.isForward* to true in the next query.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    ///
    /// ## Returns:
    ///
    /// The events matching the filters.
    pub async fn query_events(&self, filter: &crate::types::structs::EventFilterSpec, event_view_spec: Option<&dyn crate::types::traits::EventManagerEventViewSpecTrait>) -> Result<Option<Vec<crate::types::structs::Event>>> {
        let input = QueryEventsRequestType {filter, event_view_spec, };
        let path = format!("/EventManager/{moId}/QueryEvents", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
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
    pub async fn create_collector_for_events(&self, filter: &crate::types::structs::EventFilterSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateCollectorForEventsRequestType {filter, };
        let path = format!("/EventManager/{moId}/CreateCollectorForEvents", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn log_user_event(&self, entity: &crate::types::structs::ManagedObjectReference, msg: &str) -> Result<()> {
        let input = LogUserEventRequestType {entity, msg, };
        let path = format!("/EventManager/{moId}/LogUserEvent", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
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
    pub async fn post_event(&self, event_to_post: &crate::types::structs::Event, task_info: Option<&crate::types::structs::TaskInfo>) -> Result<()> {
        let input = PostEventRequestType {event_to_post, task_info, };
        let path = format!("/EventManager/{moId}/PostEvent", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Retrieves the argument meta-data for a given Event type
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### event_type_id
    /// -
    pub async fn retrieve_argument_description(&self, event_type_id: &str) -> Result<Option<Vec<crate::types::structs::EventArgDesc>>> {
        let input = RetrieveArgumentDescriptionRequestType {event_type_id, };
        let path = format!("/EventManager/{moId}/RetrieveArgumentDescription", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Static descriptive strings used in events.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn description(&self) -> Result<crate::types::structs::EventDescription> {
        let path = format!("/EventManager/{moId}/description", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// The latest event that happened on the VirtualCenter server.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn latest_event(&self) -> Result<Option<crate::types::structs::Event>> {
        let path = format!("/EventManager/{moId}/latestEvent", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// For each client, the maximum number of event collectors that can exist
    /// simultaneously.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn max_collector(&self) -> Result<i32> {
        let path = format!("/EventManager/{moId}/maxCollector", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryEventsRequestType<'a> {
    filter: &'a crate::types::structs::EventFilterSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eventViewSpec")]
    event_view_spec: Option<&'a dyn crate::types::traits::EventManagerEventViewSpecTrait>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateCollectorForEventsRequestType<'a> {
    filter: &'a crate::types::structs::EventFilterSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct LogUserEventRequestType<'a> {
    entity: &'a crate::types::structs::ManagedObjectReference,
    msg: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PostEventRequestType<'a> {
    #[serde(rename = "eventToPost")]
    event_to_post: &'a crate::types::structs::Event,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "taskInfo")]
    task_info: Option<&'a crate::types::structs::TaskInfo>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveArgumentDescriptionRequestType<'a> {
    #[serde(rename = "eventTypeId")]
    event_type_id: &'a str,
}
