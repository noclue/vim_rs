use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object type provides properties and methods for
/// event management support.
/// 
/// Event objects are used to record significant state changes of
/// managed entities.
#[derive(Clone)]
pub struct EventManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl EventManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
        let bytes_opt = self.client.invoke_optional("", "EventManager", &self.mo_id, "QueryEvents", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
        let bytes = self.client.invoke("", "EventManager", &self.mo_id, "CreateCollectorForEvents", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        self.client.invoke_void("", "EventManager", &self.mo_id, "LogUserEvent", Some(&input)).await
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
        self.client.invoke_void("", "EventManager", &self.mo_id, "PostEvent", Some(&input)).await
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
        let bytes_opt = self.client.invoke_optional("", "EventManager", &self.mo_id, "RetrieveArgumentDescription", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Static descriptive strings used in events.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn description(&self) -> Result<crate::types::structs::EventDescription> {
        let pv_opt = self.client.fetch_property_raw("", "EventManager", &self.mo_id, "description").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property description was empty".to_string()))?;
        let result: crate::types::structs::EventDescription = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// The latest event that happened on the VirtualCenter server.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn latest_event(&self) -> Result<Option<crate::types::structs::Event>> {
        let pv_opt = self.client.fetch_property_raw("", "EventManager", &self.mo_id, "latestEvent").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// For each client, the maximum number of event collectors that can exist
    /// simultaneously.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn max_collector(&self) -> Result<i32> {
        let pv_opt = self.client.fetch_property_raw("", "EventManager", &self.mo_id, "maxCollector").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property maxCollector was empty".to_string()))?;
        let result: i32 = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
}
struct QueryEventsRequestType<'a> {
    filter: &'a crate::types::structs::EventFilterSpec,
    event_view_spec: Option<&'a dyn crate::types::traits::EventManagerEventViewSpecTrait>,
}

impl<'a> miniserde::Serialize for QueryEventsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryEventsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryEventsRequestTypeSer<'b, 'a> {
    data: &'b QueryEventsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryEventsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryEventsRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("filter"), &self.data.filter as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.event_view_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("eventViewSpec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CreateCollectorForEventsRequestType<'a> {
    filter: &'a crate::types::structs::EventFilterSpec,
}

impl<'a> miniserde::Serialize for CreateCollectorForEventsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateCollectorForEventsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateCollectorForEventsRequestTypeSer<'b, 'a> {
    data: &'b CreateCollectorForEventsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateCollectorForEventsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateCollectorForEventsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("filter"), &self.data.filter as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct LogUserEventRequestType<'a> {
    entity: &'a crate::types::structs::ManagedObjectReference,
    msg: &'a str,
}

impl<'a> miniserde::Serialize for LogUserEventRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(LogUserEventRequestTypeSer { data: self, seq: 0 }))
    }
}

struct LogUserEventRequestTypeSer<'b, 'a> {
    data: &'b LogUserEventRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for LogUserEventRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"LogUserEventRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("msg"), &self.data.msg as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct PostEventRequestType<'a> {
    event_to_post: &'a crate::types::structs::Event,
    task_info: Option<&'a crate::types::structs::TaskInfo>,
}

impl<'a> miniserde::Serialize for PostEventRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PostEventRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PostEventRequestTypeSer<'b, 'a> {
    data: &'b PostEventRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for PostEventRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PostEventRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("eventToPost"), &self.data.event_to_post as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.task_info else { continue; };
                    return Some((std::borrow::Cow::Borrowed("taskInfo"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RetrieveArgumentDescriptionRequestType<'a> {
    event_type_id: &'a str,
}

impl<'a> miniserde::Serialize for RetrieveArgumentDescriptionRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveArgumentDescriptionRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveArgumentDescriptionRequestTypeSer<'b, 'a> {
    data: &'b RetrieveArgumentDescriptionRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RetrieveArgumentDescriptionRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveArgumentDescriptionRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("eventTypeId"), &self.data.event_type_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
