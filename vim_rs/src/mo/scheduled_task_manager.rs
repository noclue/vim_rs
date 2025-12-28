use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Object manager for scheduled tasks.
#[derive(Clone)]
pub struct ScheduledTaskManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl ScheduledTaskManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn create_scheduled_task(&self, entity: &crate::types::structs::ManagedObjectReference, spec: &dyn crate::types::traits::ScheduledTaskSpecTrait) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateScheduledTaskRequestType {entity, spec, };
        let path = format!("/ScheduledTaskManager/{moId}/CreateScheduledTask", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::ManagedObjectReference = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
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
    pub async fn create_object_scheduled_task(&self, obj: &crate::types::structs::ManagedObjectReference, spec: &dyn crate::types::traits::ScheduledTaskSpecTrait) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateObjectScheduledTaskRequestType {obj, spec, };
        let path = format!("/ScheduledTaskManager/{moId}/CreateObjectScheduledTask", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::ManagedObjectReference = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
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
    pub async fn retrieve_entity_scheduled_task(&self, entity: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let input = RetrieveEntityScheduledTaskRequestType {entity, };
        let path = format!("/ScheduledTaskManager/{moId}/RetrieveEntityScheduledTask", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::ManagedObjectReference>>(bytes.as_ref())?)),
            None => Ok(None),
        }
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
    pub async fn retrieve_object_scheduled_task(&self, obj: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let input = RetrieveObjectScheduledTaskRequestType {obj, };
        let path = format!("/ScheduledTaskManager/{moId}/RetrieveObjectScheduledTask", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::ManagedObjectReference>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Static descriptive strings used in scheduled tasks.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn description(&self) -> Result<crate::types::structs::ScheduledTaskDescription> {
        let path = format!("/ScheduledTaskManager/{moId}/description", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::ScheduledTaskDescription = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// All available scheduled tasks.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *ScheduledTask*.
    pub async fn scheduled_task(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let path = format!("/ScheduledTaskManager/{moId}/scheduledTask", moId = &self.mo_id);
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
struct CreateScheduledTaskRequestType<'a> {
    entity: &'a crate::types::structs::ManagedObjectReference,
    spec: &'a dyn crate::types::traits::ScheduledTaskSpecTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateObjectScheduledTaskRequestType<'a> {
    obj: &'a crate::types::structs::ManagedObjectReference,
    spec: &'a dyn crate::types::traits::ScheduledTaskSpecTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveEntityScheduledTaskRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entity: Option<&'a crate::types::structs::ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveObjectScheduledTaskRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    obj: Option<&'a crate::types::structs::ManagedObjectReference>,
}
