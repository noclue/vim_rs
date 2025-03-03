use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::CustomFieldDef;
use crate::types::structs::HostHyperThreadScheduleInfo;
/// This managed object provides an interface
/// through which you can gather and configure the host CPU scheduler
/// policies that affect the performance of running virtual machines.
/// 
/// **Note**: This managed object is useful only on platforms where
/// resource management controls are available to optimize the running
/// of virtual machines.
pub struct HostCpuSchedulerSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl HostCpuSchedulerSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Don't treat hyperthreads as schedulable resources the next time the CPU
    /// scheduler starts.
    /// 
    /// If successful, this operation will change the
    /// configured setting.
    /// 
    /// ***Required privileges:*** Host.Config.HyperThreading
    pub async fn disable_hyper_threading(&self) -> Result<()> {
        let path = format!("/HostCpuSchedulerSystem/{moId}/DisableHyperThreading", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Treat hyperthreads as schedulable resources the next time the CPU
    /// scheduler starts.
    /// 
    /// If successful, this operation will set the
    /// *config*
    /// property to "true".
    /// 
    /// ***Required privileges:*** Host.Config.HyperThreading
    pub async fn enable_hyper_threading(&self) -> Result<()> {
        let path = format!("/HostCpuSchedulerSystem/{moId}/EnableHyperThreading", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Assigns a value to a custom field.
    /// 
    /// The setCustomValue method requires
    /// whichever updatePrivilege is defined as one of the
    /// *CustomFieldDef.fieldInstancePrivileges*
    /// for the CustomFieldDef whose value is being changed.
    ///
    /// ## Parameters:
    ///
    /// ### key
    /// The name of the field whose value is to be updated.
    ///
    /// ### value
    /// Value to be assigned to the custom field.
    pub async fn set_custom_value(&self, key: &str, value: &str) -> Result<()> {
        let input = SetCustomValueRequestType {key, value, };
        let path = format!("/HostCpuSchedulerSystem/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/HostCpuSchedulerSystem/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// The hyperthread configuration for the CpuSchedulerSystem.
    /// 
    /// The
    /// existence of this data object type indicates if the CPU scheduler
    /// is capable of scheduling hyperthreads as resources.
    pub async fn hyperthread_info(&self) -> Result<Option<HostHyperThreadScheduleInfo>> {
        let path = format!("/HostCpuSchedulerSystem/{moId}/hyperthreadInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// List of custom field values.
    /// 
    /// Each value uses a key to associate
    /// an instance of a *CustomFieldStringValue* with
    /// a custom field definition.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>> {
        let path = format!("/HostCpuSchedulerSystem/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(rename = "setCustomValueRequestType", tag = "_typeName")]
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}
