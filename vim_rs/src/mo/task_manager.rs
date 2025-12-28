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
        let path = format!("/TaskManager/{moId}/ReadNextTasksByViewSpec", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::TaskInfo>>(bytes.as_ref())?)),
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
        let path = format!("/TaskManager/{moId}/CreateCollectorForTasks", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::ManagedObjectReference = serde_json::from_slice(bytes.as_ref())?;
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
        let path = format!("/TaskManager/{moId}/CreateCollectorWithInfoFilterForTasks", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::ManagedObjectReference = serde_json::from_slice(bytes.as_ref())?;
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
        let path = format!("/TaskManager/{moId}/CreateTask", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::TaskInfo = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Locale-specific, static strings that describe *Task*
    /// information to users.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn description(&self) -> Result<crate::types::structs::TaskDescription> {
        let path = format!("/TaskManager/{moId}/description", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::TaskDescription = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Maximum number of *TaskHistoryCollector*
    /// data objects that can exist concurrently, per client.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn max_collector(&self) -> Result<i32> {
        let path = format!("/TaskManager/{moId}/maxCollector", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: i32 = serde_json::from_slice(bytes.as_ref())?;
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
        let path = format!("/TaskManager/{moId}/recentTask", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::ManagedObjectReference>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReadNextTasksByViewSpecRequestType<'a> {
    #[serde(rename = "viewSpec")]
    view_spec: &'a dyn crate::types::traits::TaskManagerTaskViewSpecTrait,
    #[serde(rename = "filterSpec")]
    filter_spec: &'a crate::types::structs::TaskFilterSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "infoFilterSpec")]
    info_filter_spec: Option<&'a crate::types::structs::TaskInfoFilterSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateCollectorForTasksRequestType<'a> {
    filter: &'a crate::types::structs::TaskFilterSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateCollectorWithInfoFilterForTasksRequestType<'a> {
    filter: &'a crate::types::structs::TaskFilterSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "infoFilter")]
    info_filter: Option<&'a crate::types::structs::TaskInfoFilterSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateTaskRequestType<'a> {
    obj: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "taskTypeId")]
    task_type_id: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "initiatedBy")]
    initiated_by: Option<&'a str>,
    cancelable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parentTaskKey")]
    parent_task_key: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "activationId")]
    activation_id: Option<&'a str>,
}
