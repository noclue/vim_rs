use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::CustomizationSpec;
use crate::types::structs::ManagedObjectReference;
/// GuestCustomizationManager is a singleton managed object that provides APIs
/// for guest customization of a running VM.
pub struct VirtualMachineGuestCustomizationManager {
    client: Arc<Client>,
    mo_id: String,
}
impl VirtualMachineGuestCustomizationManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
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
    pub async fn abort_customization_task(&self, vm: &ManagedObjectReference, auth: &dyn crate::types::guest_authentication_trait::GuestAuthenticationTrait) -> Result<ManagedObjectReference> {
        let input = AbortCustomizationRequestType {vm, auth, };
        let path = format!("/VirtualMachineGuestCustomizationManager/{moId}/AbortCustomization_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn customize_guest_task(&self, vm: &ManagedObjectReference, auth: &dyn crate::types::guest_authentication_trait::GuestAuthenticationTrait, spec: &CustomizationSpec, config_params: Option<&[Box<dyn crate::types::option_value_trait::OptionValueTrait>]>) -> Result<ManagedObjectReference> {
        let input = CustomizeGuestRequestType {vm, auth, spec, config_params, };
        let path = format!("/VirtualMachineGuestCustomizationManager/{moId}/CustomizeGuest_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn start_guest_network_task(&self, vm: &ManagedObjectReference, auth: &dyn crate::types::guest_authentication_trait::GuestAuthenticationTrait) -> Result<ManagedObjectReference> {
        let input = StartGuestNetworkRequestType {vm, auth, };
        let path = format!("/VirtualMachineGuestCustomizationManager/{moId}/StartGuestNetwork_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AbortCustomizationRequestType<'a> {
    vm: &'a ManagedObjectReference,
    auth: &'a dyn crate::types::guest_authentication_trait::GuestAuthenticationTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CustomizeGuestRequestType<'a> {
    vm: &'a ManagedObjectReference,
    auth: &'a dyn crate::types::guest_authentication_trait::GuestAuthenticationTrait,
    spec: &'a CustomizationSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "configParams")]
    config_params: Option<&'a [Box<dyn crate::types::option_value_trait::OptionValueTrait>]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct StartGuestNetworkRequestType<'a> {
    vm: &'a ManagedObjectReference,
    auth: &'a dyn crate::types::guest_authentication_trait::GuestAuthenticationTrait,
}
