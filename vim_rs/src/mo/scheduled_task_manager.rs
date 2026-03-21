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
        let bytes = self.client.invoke("", "ScheduledTaskManager", &self.mo_id, "CreateScheduledTask", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
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
        let bytes = self.client.invoke("", "ScheduledTaskManager", &self.mo_id, "CreateObjectScheduledTask", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
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
        let bytes_opt = self.client.invoke_optional("", "ScheduledTaskManager", &self.mo_id, "RetrieveEntityScheduledTask", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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
        let bytes_opt = self.client.invoke_optional("", "ScheduledTaskManager", &self.mo_id, "RetrieveObjectScheduledTask", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Static descriptive strings used in scheduled tasks.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn description(&self) -> Result<crate::types::structs::ScheduledTaskDescription> {
        let bytes_opt = self.client.fetch_property_raw("", "ScheduledTaskManager", &self.mo_id, "description").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property description was empty".to_string()))?;
        let result: crate::types::structs::ScheduledTaskDescription = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
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
        let bytes_opt = self.client.fetch_property_raw("", "ScheduledTaskManager", &self.mo_id, "scheduledTask").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
}
struct CreateScheduledTaskRequestType<'a> {
    entity: &'a crate::types::structs::ManagedObjectReference,
    spec: &'a dyn crate::types::traits::ScheduledTaskSpecTrait,
}

impl<'a> miniserde::Serialize for CreateScheduledTaskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateScheduledTaskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateScheduledTaskRequestTypeSer<'b, 'a> {
    data: &'b CreateScheduledTaskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateScheduledTaskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateScheduledTaskRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateObjectScheduledTaskRequestType<'a> {
    obj: &'a crate::types::structs::ManagedObjectReference,
    spec: &'a dyn crate::types::traits::ScheduledTaskSpecTrait,
}

impl<'a> miniserde::Serialize for CreateObjectScheduledTaskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateObjectScheduledTaskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateObjectScheduledTaskRequestTypeSer<'b, 'a> {
    data: &'b CreateObjectScheduledTaskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateObjectScheduledTaskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateObjectScheduledTaskRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("obj"), &self.data.obj as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RetrieveEntityScheduledTaskRequestType<'a> {
    entity: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for RetrieveEntityScheduledTaskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveEntityScheduledTaskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveEntityScheduledTaskRequestTypeSer<'b, 'a> {
    data: &'b RetrieveEntityScheduledTaskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RetrieveEntityScheduledTaskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveEntityScheduledTaskRequestType")),
                1 => {
                    let Some(ref val) = self.data.entity else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entity"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RetrieveObjectScheduledTaskRequestType<'a> {
    obj: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for RetrieveObjectScheduledTaskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveObjectScheduledTaskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveObjectScheduledTaskRequestTypeSer<'b, 'a> {
    data: &'b RetrieveObjectScheduledTaskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RetrieveObjectScheduledTaskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveObjectScheduledTaskRequestType")),
                1 => {
                    let Some(ref val) = self.data.obj else { continue; };
                    return Some((std::borrow::Cow::Borrowed("obj"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
