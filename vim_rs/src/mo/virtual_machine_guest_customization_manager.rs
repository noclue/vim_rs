use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// GuestCustomizationManager is a singleton managed object that provides APIs
/// for guest customization of a running VM.
#[derive(Clone)]
pub struct VirtualMachineGuestCustomizationManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VirtualMachineGuestCustomizationManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Abort any running guest customization process in the guest and remove
    /// the guest customization lock in the guest as well.
    /// 
    /// As a result of the
    /// operation, the guest configuration may be left in an undefined state,
    /// which is however fine because guest customization is idempotent.
    /// A later successful guest customization can set the guest configuration
    /// to a valid state.
    /// The virtual machine must be in the powered-on state and the VMware Tools
    /// must be running.
    /// The VM is typically a cloned VM after the InstantClone operation. See
    /// *VirtualMachine.InstantClone_Task*.
    /// 
    /// ***Required privileges:*** VirtualMachine.Provisioning.Customize
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// The Virtual Machine managed object reference.
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data. See
    /// *GuestAuthentication*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***InvalidPowerState***: if the VM is not powered on.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, if the VMware
    /// Tools is not running.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***GuestPermissionDenied***: if the provided guest authentication
    /// is not sufficient to perform the guest customization.
    /// 
    /// ***CustomizationFault***: if a customization error occurs.
    pub async fn abort_customization_task(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = AbortCustomizationRequestType {vm, auth, };
        let path = format!("/VirtualMachineGuestCustomizationManager/{moId}/AbortCustomization_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Customize a running virtual machine.
    /// 
    /// The virtual machine must be in the powered-on state and the VMware Tools
    /// must be running.
    /// The VM is typically a cloned VM after the InstantClone operation. See
    /// *VirtualMachine.InstantClone_Task*.
    /// 
    /// ***Required privileges:*** VirtualMachine.Provisioning.Customize
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// The Virtual Machine managed object reference.
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data. See
    /// *GuestAuthentication*.
    ///
    /// ### spec
    /// Is a *CustomizationSpec*.
    /// It specifies the virtual machine's configuration.
    ///
    /// ### config_params
    /// addtional key/value pair list to support
    /// third party customization.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***InvalidPowerState***: if the VM is not powered on.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, if the VMware
    /// Tools is not running.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***GuestPermissionDenied***: if the provided guest authentication
    /// is not sufficient to perform the guest customization.
    /// 
    /// ***CustomizationFault***: if a customization error occurs.
    pub async fn customize_guest_task(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait, spec: &crate::types::structs::CustomizationSpec, config_params: Option<&[Box<dyn crate::types::traits::OptionValueTrait>]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CustomizeGuestRequestType {vm, auth, spec, config_params, };
        let path = format!("/VirtualMachineGuestCustomizationManager/{moId}/CustomizeGuest_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Start the network service in the guest, e.g.
    /// 
    /// acquire IPs from DHCP.
    /// The virtual machine must be in the powered-on state and the VMware Tools
    /// must be running.
    /// The VM is typically a cloned VM after the InstantClone operation. See
    /// *VirtualMachine.InstantClone_Task*.
    /// 
    /// ***Required privileges:*** VirtualMachine.Provisioning.Customize
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// The Virtual Machine managed object reference.
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### auth
    /// The guest authentication data. See
    /// *GuestAuthentication*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***InvalidPowerState***: if the VM is not powered on.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, if the VMware
    /// Tools is not running.
    /// 
    /// ***InvalidGuestLogin***: if the the guest authentication information
    /// was not accepted.
    /// 
    /// ***GuestPermissionDenied***: if the provided guest authentication
    /// is not sufficient to perform the guest customization.
    /// 
    /// ***CustomizationFault***: if a customization error occurs.
    pub async fn start_guest_network_task(&self, vm: &crate::types::structs::ManagedObjectReference, auth: &dyn crate::types::traits::GuestAuthenticationTrait) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = StartGuestNetworkRequestType {vm, auth, };
        let path = format!("/VirtualMachineGuestCustomizationManager/{moId}/StartGuestNetwork_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct AbortCustomizationRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
}

impl<'a> miniserde::Serialize for AbortCustomizationRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AbortCustomizationRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AbortCustomizationRequestTypeSer<'b, 'a> {
    data: &'b AbortCustomizationRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AbortCustomizationRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AbortCustomizationRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CustomizeGuestRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
    spec: &'a crate::types::structs::CustomizationSpec,
    config_params: Option<&'a [Box<dyn crate::types::traits::OptionValueTrait>]>,
}

impl<'a> miniserde::Serialize for CustomizeGuestRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CustomizeGuestRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CustomizeGuestRequestTypeSer<'b, 'a> {
    data: &'b CustomizeGuestRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CustomizeGuestRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CustomizeGuestRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
                4 => {
                    let Some(ref val) = self.data.config_params else { continue; };
                    return Some((std::borrow::Cow::Borrowed("configParams"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct StartGuestNetworkRequestType<'a> {
    vm: &'a crate::types::structs::ManagedObjectReference,
    auth: &'a dyn crate::types::traits::GuestAuthenticationTrait,
}

impl<'a> miniserde::Serialize for StartGuestNetworkRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(StartGuestNetworkRequestTypeSer { data: self, seq: 0 }))
    }
}

struct StartGuestNetworkRequestTypeSer<'b, 'a> {
    data: &'b StartGuestNetworkRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for StartGuestNetworkRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"StartGuestNetworkRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vm"), &self.data.vm as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("auth"), &self.data.auth as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
