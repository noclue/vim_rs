use std::sync::Arc;
use crate::core::client::{VimClient, Result};
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
#[derive(Clone)]
pub struct ClusterEvcManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl ClusterEvcManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn check_add_host_evc_task(&self, cnx_spec: &crate::types::structs::HostConnectSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CheckAddHostEvcRequestType {cnx_spec, };
        let path = format!("/ClusterEVCManager/{moId}/CheckAddHostEvc_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn check_configure_evc_mode_task(&self, evc_mode_key: &str, evc_graphics_mode_key: Option<&str>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CheckConfigureEvcModeRequestType {evc_mode_key, evc_graphics_mode_key, };
        let path = format!("/ClusterEVCManager/{moId}/CheckConfigureEvcMode_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn configure_evc_mode_task(&self, evc_mode_key: &str, evc_graphics_mode_key: Option<&str>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ConfigureEvcModeRequestType {evc_mode_key, evc_graphics_mode_key, };
        let path = format!("/ClusterEVCManager/{moId}/ConfigureEvcMode_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn disable_evc_mode_task(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let path = format!("/ClusterEVCManager/{moId}/DisableEvcMode_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<crate::types::structs::CustomFieldDef>>> {
        let path = format!("/ClusterEVCManager/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::CustomFieldDef>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// EVC-related state of the managed cluster.
    pub async fn evc_state(&self) -> Result<crate::types::structs::ClusterEvcManagerEvcState> {
        let path = format!("/ClusterEVCManager/{moId}/evcState", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ClusterEvcManagerEvcState = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Cluster associated with this manager object.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *ClusterComputeResource*.
    pub async fn managed_cluster(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let path = format!("/ClusterEVCManager/{moId}/managedCluster", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// List of custom field values.
    /// 
    /// Each value uses a key to associate
    /// an instance of a *CustomFieldStringValue* with
    /// a custom field definition.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>> {
        let path = format!("/ClusterEVCManager/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
}
struct CheckAddHostEvcRequestType<'a> {
    cnx_spec: &'a crate::types::structs::HostConnectSpec,
}

impl<'a> miniserde::Serialize for CheckAddHostEvcRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CheckAddHostEvcRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CheckAddHostEvcRequestTypeSer<'b, 'a> {
    data: &'b CheckAddHostEvcRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for CheckAddHostEvcRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CheckAddHostEvcRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cnxSpec"), &self.data.cnx_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CheckConfigureEvcModeRequestType<'a> {
    evc_mode_key: &'a str,
    evc_graphics_mode_key: Option<&'a str>,
}

impl<'a> miniserde::Serialize for CheckConfigureEvcModeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CheckConfigureEvcModeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CheckConfigureEvcModeRequestTypeSer<'b, 'a> {
    data: &'b CheckConfigureEvcModeRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for CheckConfigureEvcModeRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CheckConfigureEvcModeRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("evcModeKey"), &self.data.evc_mode_key as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.evc_graphics_mode_key else { continue; };
                    return Some((std::borrow::Cow::Borrowed("evcGraphicsModeKey"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ConfigureEvcModeRequestType<'a> {
    evc_mode_key: &'a str,
    evc_graphics_mode_key: Option<&'a str>,
}

impl<'a> miniserde::Serialize for ConfigureEvcModeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ConfigureEvcModeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ConfigureEvcModeRequestTypeSer<'b, 'a> {
    data: &'b ConfigureEvcModeRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for ConfigureEvcModeRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ConfigureEvcModeRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("evcModeKey"), &self.data.evc_mode_key as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.evc_graphics_mode_key else { continue; };
                    return Some((std::borrow::Cow::Borrowed("evcGraphicsModeKey"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}

impl<'a> miniserde::Serialize for SetCustomValueRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetCustomValueRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetCustomValueRequestTypeSer<'b, 'a> {
    data: &'b SetCustomValueRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for SetCustomValueRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"setCustomValueRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("key"), &self.data.key as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("value"), &self.data.value as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
