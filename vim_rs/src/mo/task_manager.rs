use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The TaskManager managed object provides an interface for creating and managing
/// *Task* managed objects.
/// 
/// Many operations are non-blocking,
/// returning a *Task* managed object that can be monitored by a
/// client application. *Task* managed objects may also be
/// accessed through the TaskManager.
#[derive(Clone)]
pub struct TaskManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl TaskManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Returns *TaskInfo* data objects
    /// based on the specified *TaskManagerTaskViewSpec*, *TaskFilterSpec* and
    /// *TaskInfoFilterSpec* parameters.
    /// 
    /// Returns an empty array when there
    /// are no tasks matching the filtering parameters.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### view_spec
    /// The view parameters for the tasks query.
    ///
    /// ### filter_spec
    /// The specification for the task query filter.
    ///
    /// ### info_filter_spec
    /// The specification for the task info filter.
    ///
    /// ## Returns:
    ///
    /// *TaskInfo* data objects matching the filtering
    /// parameters.
    pub async fn read_next_tasks_by_view_spec(&self, view_spec: &dyn crate::types::traits::TaskManagerTaskViewSpecTrait, filter_spec: &crate::types::structs::TaskFilterSpec, info_filter_spec: Option<&crate::types::structs::TaskInfoFilterSpec>) -> Result<Option<Vec<crate::types::structs::TaskInfo>>> {
        let input = ReadNextTasksByViewSpecRequestType {view_spec, filter_spec, info_filter_spec, };
        let bytes_opt = self.client.invoke_optional("", "TaskManager", &self.mo_id, "ReadNextTasksByViewSpec", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Creates a *TaskHistoryCollector*, a
    /// specialized *HistoryCollector* that gathers
    /// *TaskInfo* data objects.
    /// 
    /// A *TaskHistoryCollector* does not persist
    /// beyond the current client session.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### filter
    /// The specification for the task query filter.
    ///
    /// ## Returns:
    ///
    /// The task collector based on the filter.
    /// 
    /// Refers instance of *TaskHistoryCollector*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the filter is null or unknown.
    /// 
    /// ***InvalidState***: if there are more than the maximum number of
    /// task collectors.
    /// 
    /// ***NotSupported***: if called directly on a host.
    pub async fn create_collector_for_tasks(&self, filter: &crate::types::structs::TaskFilterSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateCollectorForTasksRequestType {filter, };
        let bytes = self.client.invoke("", "TaskManager", &self.mo_id, "CreateCollectorForTasks", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Creates a *TaskHistoryCollector*, a
    /// specialized *HistoryCollector* that gathers
    /// *TaskInfo* data objects.
    /// 
    /// A *TaskHistoryCollector* does not persist
    /// beyond the current client session.
    /// 
    /// ***Since:*** vSphere API Release 8.0.3.0
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### filter
    /// The specification for the task query filter.
    ///
    /// ### info_filter
    /// The specification for the task info filter.
    ///
    /// ## Returns:
    ///
    /// The task collector based on the filter.
    /// 
    /// Refers instance of *TaskHistoryCollector*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the filter is null or unknown.
    /// 
    /// ***InvalidState***: if there are more than the maximum number of
    /// task collectors.
    /// 
    /// ***NotSupported***: if called directly on a host.
    pub async fn create_collector_with_info_filter_for_tasks(&self, filter: &crate::types::structs::TaskFilterSpec, info_filter: Option<&crate::types::structs::TaskInfoFilterSpec>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateCollectorWithInfoFilterForTasksRequestType {filter, info_filter, };
        let bytes = self.client.invoke("", "TaskManager", &self.mo_id, "CreateCollectorWithInfoFilterForTasks", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Creates a new *Task*, specifying the object with which
    /// the *Task* is associated, the type of task,
    /// and whether the task is cancelable.
    /// 
    /// Use this operation in conjunction
    /// with the *ExtensionManager*.
    /// 
    /// ***Required privileges:*** Task.Create
    ///
    /// ## Parameters:
    ///
    /// ### obj
    /// ManagedObject with which Task will be associated
    ///
    /// ### task_type_id
    /// Extension registered task type identifier
    /// for type of task being created
    ///
    /// ### initiated_by
    /// The name of the user on whose behalf the
    /// Extension is creating the task
    ///
    /// ### cancelable
    /// True if the task should be cancelable,
    /// false otherwise
    ///
    /// ### parent_task_key
    /// Key of the task that is the parent of this task
    ///
    /// ### activation_id
    /// Activation Id is a client-provided token to link an
    /// API call with a task. When provided, the activationId is added to the
    /// *TaskInfo*
    ///
    /// ## Returns:
    ///
    /// *TaskInfo* data object describing the new task
    pub async fn create_task(&self, obj: &crate::types::structs::ManagedObjectReference, task_type_id: &str, initiated_by: Option<&str>, cancelable: bool, parent_task_key: Option<&str>, activation_id: Option<&str>) -> Result<crate::types::structs::TaskInfo> {
        let input = CreateTaskRequestType {obj, task_type_id, initiated_by, cancelable, parent_task_key, activation_id, };
        let bytes = self.client.invoke("", "TaskManager", &self.mo_id, "CreateTask", Some(&input)).await?;
        let result: crate::types::structs::TaskInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Locale-specific, static strings that describe *Task*
    /// information to users.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn description(&self) -> Result<crate::types::structs::TaskDescription> {
        let bytes_opt = self.client.fetch_property_raw("", "TaskManager", &self.mo_id, "description").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property description was empty".to_string()))?;
        let result: crate::types::structs::TaskDescription = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Maximum number of *TaskHistoryCollector*
    /// data objects that can exist concurrently, per client.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn max_collector(&self) -> Result<i32> {
        let bytes_opt = self.client.fetch_property_raw("", "TaskManager", &self.mo_id, "maxCollector").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property maxCollector was empty".to_string()))?;
        let result: i32 = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// A list of *Task* managed objects that completed recently,
    /// that are currently running, or that are queued to run.
    /// 
    /// The list contains only *Task* objects that the client
    /// has permission to access, which is determined by having permission to
    /// access the *Task* object's managed *entity*.
    /// 
    /// The completed *Task* objects by default include only
    /// *Task* objects that completed within the past 10 minutes.
    /// When connected to vCenter Server, there is an additional default limitation
    /// that each of the completed *Task* objects in this list is one
    /// of the last 200 completed *Task* objects.
    /// 
    /// This property should not be used for tracking *Task*
    /// completion. Generally, a *ListView* is a better way to
    /// monitor a specific set of *Task* objects.
    /// In releases after vSphere API 5.0, vSphere Servers might not
    /// generate property collector update notifications for this property.
    /// To obtain the latest value of the property, you can use
    /// PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
    /// If you use the PropertyCollector.WaitForUpdatesEx method, specify
    /// an empty string for the version parameter. Any other version value will not
    /// produce any property values as no updates are generated.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Task*.
    pub async fn recent_task(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let bytes_opt = self.client.fetch_property_raw("", "TaskManager", &self.mo_id, "recentTask").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
}
struct ReadNextTasksByViewSpecRequestType<'a> {
    view_spec: &'a dyn crate::types::traits::TaskManagerTaskViewSpecTrait,
    filter_spec: &'a crate::types::structs::TaskFilterSpec,
    info_filter_spec: Option<&'a crate::types::structs::TaskInfoFilterSpec>,
}

impl<'a> miniserde::Serialize for ReadNextTasksByViewSpecRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReadNextTasksByViewSpecRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReadNextTasksByViewSpecRequestTypeSer<'b, 'a> {
    data: &'b ReadNextTasksByViewSpecRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ReadNextTasksByViewSpecRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReadNextTasksByViewSpecRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("viewSpec"), &self.data.view_spec as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("filterSpec"), &self.data.filter_spec as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.info_filter_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("infoFilterSpec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CreateCollectorForTasksRequestType<'a> {
    filter: &'a crate::types::structs::TaskFilterSpec,
}

impl<'a> miniserde::Serialize for CreateCollectorForTasksRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateCollectorForTasksRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateCollectorForTasksRequestTypeSer<'b, 'a> {
    data: &'b CreateCollectorForTasksRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateCollectorForTasksRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateCollectorForTasksRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("filter"), &self.data.filter as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateCollectorWithInfoFilterForTasksRequestType<'a> {
    filter: &'a crate::types::structs::TaskFilterSpec,
    info_filter: Option<&'a crate::types::structs::TaskInfoFilterSpec>,
}

impl<'a> miniserde::Serialize for CreateCollectorWithInfoFilterForTasksRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateCollectorWithInfoFilterForTasksRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateCollectorWithInfoFilterForTasksRequestTypeSer<'b, 'a> {
    data: &'b CreateCollectorWithInfoFilterForTasksRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateCollectorWithInfoFilterForTasksRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateCollectorWithInfoFilterForTasksRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("filter"), &self.data.filter as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.info_filter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("infoFilter"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CreateTaskRequestType<'a> {
    obj: &'a crate::types::structs::ManagedObjectReference,
    task_type_id: &'a str,
    initiated_by: Option<&'a str>,
    cancelable: bool,
    parent_task_key: Option<&'a str>,
    activation_id: Option<&'a str>,
}

impl<'a> miniserde::Serialize for CreateTaskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateTaskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateTaskRequestTypeSer<'b, 'a> {
    data: &'b CreateTaskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateTaskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateTaskRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("obj"), &self.data.obj as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("taskTypeId"), &self.data.task_type_id as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.initiated_by else { continue; };
                    return Some((std::borrow::Cow::Borrowed("initiatedBy"), val as &dyn miniserde::Serialize));
                }
                4 => return Some((std::borrow::Cow::Borrowed("cancelable"), &self.data.cancelable as &dyn miniserde::Serialize)),
                5 => {
                    let Some(ref val) = self.data.parent_task_key else { continue; };
                    return Some((std::borrow::Cow::Borrowed("parentTaskKey"), val as &dyn miniserde::Serialize));
                }
                6 => {
                    let Some(ref val) = self.data.activation_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("activationId"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
