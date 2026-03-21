use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The *HostServiceSystem* managed object describes the configuration
/// of host services.
/// 
/// This managed object operates in conjunction
/// with the *HostFirewallSystem*
/// managed object.
#[derive(Clone)]
pub struct HostServiceSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostServiceSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Refresh the service information and settings to pick up any changes
    /// made directly on the host.
    /// 
    /// ***Required privileges:*** Host.Config.NetService
    pub async fn refresh_services(&self) -> Result<()> {
        self.client.invoke_void("", "HostServiceSystem", &self.mo_id, "RefreshServices", None).await
    }
    /// Restarts the service.
    /// 
    /// ***Required privileges:*** Host.Config.NetService
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// Service identifier
    /// (*HostServiceSystem.serviceInfo*.*HostServiceInfo.service*.*HostService.key*).
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the service ID is unknown.
    /// 
    /// ***InvalidState***: if the service is not running.
    /// Only hosts with ESX/ESXi 4.1 or earlier software use this fault.
    /// Hosts running later versions of ESXi do not throw a fault in this case.
    /// 
    /// ***HostConfigFault***: for all other failures.
    pub async fn restart_service(&self, id: &str) -> Result<()> {
        let input = RestartServiceRequestType {id, };
        self.client.invoke_void("", "HostServiceSystem", &self.mo_id, "RestartService", Some(&input)).await
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
        self.client.invoke_void("", "HostServiceSystem", &self.mo_id, "setCustomValue", Some(&input)).await
    }
    /// Starts the service.
    /// 
    /// ***Required privileges:*** Host.Config.NetService
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// Service identifier
    /// (*HostServiceSystem.serviceInfo*.*HostServiceInfo.service*.*HostService.key*).
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the service ID is unknown.
    /// 
    /// ***InvalidState***: if the service is already running.
    /// Only hosts with ESX/ESXi 4.1 or earlier software use this fault.
    /// Hosts running later versions of ESXi do not throw a fault in this case.
    /// 
    /// ***HostConfigFault***: for all other failures.
    pub async fn start_service(&self, id: &str) -> Result<()> {
        let input = StartServiceRequestType {id, };
        self.client.invoke_void("", "HostServiceSystem", &self.mo_id, "StartService", Some(&input)).await
    }
    /// Stops the service.
    /// 
    /// ***Required privileges:*** Host.Config.NetService
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// Service identifier
    /// (*HostServiceSystem.serviceInfo*.*HostServiceInfo.service*.*HostService.key*).
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the service ID is unknown.
    /// 
    /// ***InvalidState***: if the service is not running.
    /// Only hosts with ESX/ESXi 4.1 or earlier software use this fault.
    /// Hosts running later versions of ESXi do not throw a fault in this case.
    /// 
    /// ***HostConfigFault***: for all other failures.
    pub async fn stop_service(&self, id: &str) -> Result<()> {
        let input = StopServiceRequestType {id, };
        self.client.invoke_void("", "HostServiceSystem", &self.mo_id, "StopService", Some(&input)).await
    }
    /// Uninstalls the service.
    /// 
    /// If the service is running, it is
    /// stopped before being uninstalled.
    /// 
    /// ***Required privileges:*** Host.Config.NetService
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// Service identifier
    /// (*HostServiceSystem.serviceInfo*.*HostServiceInfo.service*.*HostService.key*).
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the service ID is unknown.
    /// 
    /// ***InvalidArgument***: if the service is a required service.
    /// 
    /// ***NotSupported***: if the service doesn't support uninstallation.
    /// 
    /// ***HostConfigFault***: for all other failures.
    pub async fn uninstall_service(&self, id: &str) -> Result<()> {
        let input = UninstallServiceRequestType {id, };
        self.client.invoke_void("", "HostServiceSystem", &self.mo_id, "UninstallService", Some(&input)).await
    }
    /// Updates the activation policy of the service.
    /// 
    /// ***Required privileges:*** Host.Config.NetService
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// Service identifier
    /// (*HostServiceSystem.serviceInfo*.*HostServiceInfo.service*.*HostService.key*).
    ///
    /// ### policy
    /// Specifies the condition for service activation.
    /// Use one of the *HostServicePolicy_enum* values.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the service ID is unknown.
    /// 
    /// ***InvalidArgument***: if the service ID represents a required service,
    /// or if the specified policy is undefined.
    /// 
    /// ***HostConfigFault***: for all other failures.
    pub async fn update_service_policy(&self, id: &str, policy: &str) -> Result<()> {
        let input = UpdateServicePolicyRequestType {id, policy, };
        self.client.invoke_void("", "HostServiceSystem", &self.mo_id, "UpdateServicePolicy", Some(&input)).await
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<crate::types::structs::CustomFieldDef>>> {
        let bytes_opt = self.client.fetch_property_raw("", "HostServiceSystem", &self.mo_id, "availableField").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Service configuration.
    pub async fn service_info(&self) -> Result<crate::types::structs::HostServiceInfo> {
        let bytes_opt = self.client.fetch_property_raw("", "HostServiceSystem", &self.mo_id, "serviceInfo").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property serviceInfo was empty".to_string()))?;
        let result: crate::types::structs::HostServiceInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
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
        let bytes_opt = self.client.fetch_property_raw("", "HostServiceSystem", &self.mo_id, "value").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
}
struct RestartServiceRequestType<'a> {
    id: &'a str,
}

impl<'a> miniserde::Serialize for RestartServiceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RestartServiceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RestartServiceRequestTypeSer<'b, 'a> {
    data: &'b RestartServiceRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RestartServiceRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RestartServiceRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            _ => return None,
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

impl<'b, 'a> miniserde::ser::Map for SetCustomValueRequestTypeSer<'b, 'a> {
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
struct StartServiceRequestType<'a> {
    id: &'a str,
}

impl<'a> miniserde::Serialize for StartServiceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(StartServiceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct StartServiceRequestTypeSer<'b, 'a> {
    data: &'b StartServiceRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for StartServiceRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"StartServiceRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct StopServiceRequestType<'a> {
    id: &'a str,
}

impl<'a> miniserde::Serialize for StopServiceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(StopServiceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct StopServiceRequestTypeSer<'b, 'a> {
    data: &'b StopServiceRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for StopServiceRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"StopServiceRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UninstallServiceRequestType<'a> {
    id: &'a str,
}

impl<'a> miniserde::Serialize for UninstallServiceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UninstallServiceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UninstallServiceRequestTypeSer<'b, 'a> {
    data: &'b UninstallServiceRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UninstallServiceRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UninstallServiceRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateServicePolicyRequestType<'a> {
    id: &'a str,
    policy: &'a str,
}

impl<'a> miniserde::Serialize for UpdateServicePolicyRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateServicePolicyRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateServicePolicyRequestTypeSer<'b, 'a> {
    data: &'b UpdateServicePolicyRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateServicePolicyRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateServicePolicyRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("policy"), &self.data.policy as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
