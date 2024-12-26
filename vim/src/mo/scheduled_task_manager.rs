use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::ScheduledTaskDescription;
use crate::types::structs::ScheduledTaskSpecTrait;
/// Object manager for scheduled tasks.
pub struct ScheduledTaskManager {
    client: Arc<Client>,
    mo_id: String,
}
impl ScheduledTaskManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Creates a scheduled task.
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// The managed entity (or entities) for which the
    /// scheduled task triggers an action. You can
    /// schedule tasks on any managed entity. If the
    /// scheduled task is associated with a leaf node in
    /// the inventory tree, it applies only to a single
    /// entity (virtual machine or host). If the
    /// task is associated with a folder, a datacenter, a
    /// compute resource, or a resource pool, it applies to
    /// the virtual machine or host descendants of the entity.
    /// 
    /// ***Required privileges:*** ScheduledTask.Create
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ### spec
    /// The specification for the new scheduled task.
    ///
    /// ## Returns:
    ///
    /// The scheduled task created by the operation.
    /// 
    /// Refers instance of *ScheduledTask*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidName***: if the scheduled task name is empty or too long.
    /// 
    /// ***DuplicateName***: if a scheduled task with the name already exists.
    /// 
    /// ***InvalidArgument***: if the specification is invalid.
    pub async fn create_scheduled_task(&self, entity: &ManagedObjectReference, spec: &dyn ScheduledTaskSpecTrait) -> Result<ManagedObjectReference> {
        let input = CreateScheduledTaskRequestType {entity, spec, };
        let path = format!("/ScheduledTaskManager/{moId}/CreateScheduledTask", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Creates a scheduled task.
    ///
    /// ## Parameters:
    ///
    /// ### obj
    /// The managed object for which the
    /// scheduled task triggers an action. You can
    /// schedule tasks on any managed object.
    /// 
    /// ***Required privileges:*** ScheduledTask.Create
    ///
    /// ### spec
    /// The specification for the new scheduled task.
    ///
    /// ## Returns:
    ///
    /// The scheduled task created by the operation.
    /// 
    /// Refers instance of *ScheduledTask*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidName***: if the scheduled task name is empty or too long.
    /// 
    /// ***DuplicateName***: if a scheduled task with the name already exists.
    /// 
    /// ***InvalidArgument***: if the specification is invalid.
    pub async fn create_object_scheduled_task(&self, obj: &ManagedObjectReference, spec: &dyn ScheduledTaskSpecTrait) -> Result<ManagedObjectReference> {
        let input = CreateObjectScheduledTaskRequestType {obj, spec, };
        let path = format!("/ScheduledTaskManager/{moId}/CreateObjectScheduledTask", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Available scheduled tasks defined on the entity.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// The entity. If null, all scheduled tasks are returned
    /// for visible entities.
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ## Returns:
    ///
    /// The scheduled tasks.
    /// 
    /// Refers instances of *ScheduledTask*.
    pub async fn retrieve_entity_scheduled_task(&self, entity: Option<&ManagedObjectReference>) -> Result<Option<Vec<ManagedObjectReference>>> {
        let input = RetrieveEntityScheduledTaskRequestType {entity, };
        let path = format!("/ScheduledTaskManager/{moId}/RetrieveEntityScheduledTask", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Available scheduled tasks defined on the object.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### obj
    /// The object. If not specified, all scheduled tasks are returned
    /// for visible entities and visible ManagedObjects.
    ///
    /// ## Returns:
    ///
    /// The scheduled tasks.
    /// 
    /// Refers instances of *ScheduledTask*.
    pub async fn retrieve_object_scheduled_task(&self, obj: Option<&ManagedObjectReference>) -> Result<Option<Vec<ManagedObjectReference>>> {
        let input = RetrieveObjectScheduledTaskRequestType {obj, };
        let path = format!("/ScheduledTaskManager/{moId}/RetrieveObjectScheduledTask", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Static descriptive strings used in scheduled tasks.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn description(&self) -> Result<ScheduledTaskDescription> {
        let path = format!("/ScheduledTaskManager/{moId}/description", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// All available scheduled tasks.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *ScheduledTask*.
    pub async fn scheduled_task(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/ScheduledTaskManager/{moId}/scheduledTask", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateScheduledTaskRequestType<'a> {
    entity: &'a ManagedObjectReference,
    spec: &'a dyn ScheduledTaskSpecTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateObjectScheduledTaskRequestType<'a> {
    obj: &'a ManagedObjectReference,
    spec: &'a dyn ScheduledTaskSpecTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveEntityScheduledTaskRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entity: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveObjectScheduledTaskRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    obj: Option<&'a ManagedObjectReference>,
}
