use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::ClusterEvcManagerEvcState;
use crate::types::structs::CustomFieldDef;
use crate::types::structs::CustomFieldValueTrait;
use crate::types::structs::HostConnectSpec;
use crate::types::structs::ManagedObjectReference;
/// Controls Enhanced vMotion Compatibility mode for a particular cluster given
/// by *ClusterEVCManager.managedCluster*.
/// 
/// VMware Enhanced vMotion Compatibility (EVC) facilitates vMotion between
/// different CPU generations through the use of Intel Flex Migration and AMD-V
/// Extended Migration technologies. When enabled for a cluster, EVC ensures
/// that all VMs are vMotion-compatible with the CPUs of all hosts in the
/// cluster.
/// 
/// See also *EVCMode*.
pub struct ClusterEvcManager {
    client: Arc<Client>,
    mo_id: String,
}
impl ClusterEvcManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Test the validity of adding a host into the managed cluster.
    /// 
    /// Note that
    /// this method only tests EVC admission control; host-add may fail for
    /// other reasons.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### cnx_spec
    /// The spec that will be used to add the host.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidLogin***: if authentication with the host fails.
    /// 
    /// ***HostConnectFault***: if an error occurred when attempting to connect
    /// to the host. Typically, a more specific subclass is thrown.
    pub async fn check_add_host_evc_task(&self, cnx_spec: &HostConnectSpec) -> Result<ManagedObjectReference> {
        let input = CheckAddHostEvcRequestType {cnx_spec, };
        let path = format!("/ClusterEVCManager/{moId}/CheckAddHostEvc_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Test the validity of configuring an EVC mode on the managed cluster.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### evc_mode_key
    /// A key referencing the desired EVC mode.
    ///
    /// ### evc_graphics_mode_key
    /// A key referencing the desired EVC Graphics
    /// mode *Capability.supportedEVCGraphicsMode*.
    /// 
    /// ***Since:*** vSphere API Release 7.0.1.0
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the operation.
    /// 
    /// Refers instance of *Task*.
    pub async fn check_configure_evc_mode_task(&self, evc_mode_key: &str, evc_graphics_mode_key: Option<&str>) -> Result<ManagedObjectReference> {
        let input = CheckConfigureEvcModeRequestType {evc_mode_key, evc_graphics_mode_key, };
        let path = format!("/ClusterEVCManager/{moId}/CheckConfigureEvcMode_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Set the EVC mode.
    /// 
    /// If EVC is currently disabled, then this will enable
    /// EVC. The parameter must specify a key to one of the EVC modes listed in
    /// the *ClusterEVCManagerEVCState.supportedEVCMode* array
    /// property. If there are no modes listed there,
    /// then EVC may not currently be enabled; reference the other properties
    /// in *ClusterEVCManagerEVCState* to determine what conditions are blocking EVC.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    ///
    /// ## Parameters:
    ///
    /// ### evc_mode_key
    /// A key referencing the desired EVC mode.
    ///
    /// ### evc_graphics_mode_key
    /// A key referencing the desired EVC Graphics
    /// mode *Capability.supportedEVCGraphicsMode*.
    /// 
    /// ***Since:*** vSphere API Release 7.0.1.0
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***EVCConfigFault***: if configuring EVC failed. Typically, a more
    /// specific subclass is thrown.
    pub async fn configure_evc_mode_task(&self, evc_mode_key: &str, evc_graphics_mode_key: Option<&str>) -> Result<ManagedObjectReference> {
        let input = ConfigureEvcModeRequestType {evc_mode_key, evc_graphics_mode_key, };
        let path = format!("/ClusterEVCManager/{moId}/ConfigureEvcMode_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Disable EVC.
    /// 
    /// EVC may be disabled at any time.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    pub async fn disable_evc_mode_task(&self) -> Result<ManagedObjectReference> {
        let path = format!("/ClusterEVCManager/{moId}/DisableEvcMode_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
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
        let path = format!("/ClusterEVCManager/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/ClusterEVCManager/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// EVC-related state of the managed cluster.
    pub async fn evc_state(&self) -> Result<ClusterEvcManagerEvcState> {
        let path = format!("/ClusterEVCManager/{moId}/evcState", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Cluster associated with this manager object.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *ClusterComputeResource*.
    pub async fn managed_cluster(&self) -> Result<ManagedObjectReference> {
        let path = format!("/ClusterEVCManager/{moId}/managedCluster", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// List of custom field values.
    /// 
    /// Each value uses a key to associate
    /// an instance of a *CustomFieldStringValue* with
    /// a custom field definition.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn CustomFieldValueTrait>>>> {
        let path = format!("/ClusterEVCManager/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CheckAddHostEvcRequestType<'a> {
    #[serde(rename = "cnxSpec")]
    cnx_spec: &'a HostConnectSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CheckConfigureEvcModeRequestType<'a> {
    #[serde(rename = "evcModeKey")]
    evc_mode_key: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "evcGraphicsModeKey")]
    evc_graphics_mode_key: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ConfigureEvcModeRequestType<'a> {
    #[serde(rename = "evcModeKey")]
    evc_mode_key: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "evcGraphicsModeKey")]
    evc_graphics_mode_key: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(rename = "setCustomValueRequestType", tag = "_typeName")]
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}
