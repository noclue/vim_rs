use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::TaskDescription;
use crate::types::structs::TaskFilterSpec;
use crate::types::structs::TaskInfo;
/// The TaskManager managed object provides an interface for creating and managing
/// *Task* managed objects.
/// 
/// Many operations are non-blocking,
/// returning a *Task* managed object that can be monitored by a
/// client application. *Task* managed objects may also be
/// accessed through the TaskManager.
pub struct TaskManager {
    client: Arc<Client>,
    mo_id: String,
}
impl TaskManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
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
    pub async fn create_collector_for_tasks(&self, filter: &TaskFilterSpec) -> Result<ManagedObjectReference> {
        let input = CreateCollectorForTasksRequestType {filter, };
        let path = format!("/TaskManager/{moId}/CreateCollectorForTasks", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn create_task(&self, obj: &ManagedObjectReference, task_type_id: &str, initiated_by: Option<&str>, cancelable: bool, parent_task_key: Option<&str>, activation_id: Option<&str>) -> Result<TaskInfo> {
        let input = CreateTaskRequestType {obj, task_type_id, initiated_by, cancelable, parent_task_key, activation_id, };
        let path = format!("/TaskManager/{moId}/CreateTask", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Locale-specific, static strings that describe *Task*
    /// information to users.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn description(&self) -> Result<TaskDescription> {
        let path = format!("/TaskManager/{moId}/description", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// Maximum number of *TaskHistoryCollector*
    /// data objects that can exist concurrently, per client.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn max_collector(&self) -> Result<i32> {
        let path = format!("/TaskManager/{moId}/maxCollector", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
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
    pub async fn recent_task(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/TaskManager/{moId}/recentTask", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateCollectorForTasksRequestType<'a> {
    filter: &'a TaskFilterSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateTaskRequestType<'a> {
    obj: &'a ManagedObjectReference,
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
