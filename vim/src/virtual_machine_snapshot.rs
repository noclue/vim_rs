use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::CustomFieldValueTrait;
use crate::types::ManagedObjectReference;
use crate::types::VirtualMachineConfigInfo;
use crate::types::CustomFieldDef;
/// The Snapshot managed object type specifies the interface to individual snapshots
/// of a virtual machine.
/// 
/// Although these are managed objects, they are subordinate to
/// their virtual machine.
pub struct VirtualMachineSnapshot {
    client: Arc<VimClient>,
    mo_id: String,
}
impl VirtualMachineSnapshot {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Obtains an export lease on this snapshot.
    /// 
    /// The export lease contains
    /// a list of URLs for the virtual disks for this snapshot, as well as
    /// a ticket giving access to the URLs.
    /// 
    /// See *HttpNfcLease* for information on how to use the lease.
    /// 
    /// ***Required privileges:*** VApp.Export
    ///
    /// ## Returns:
    ///
    /// The export lease on this *VirtualMachineSnapshot*. The
    /// export task continues running until the lease is completed by the
    /// caller.
    /// 
    /// Refers instance of *HttpNfcLease*.
    ///
    /// ## Errors:
    ///
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***InvalidState***: if the operation cannot be performed because of the
    /// virtual machine's current state. For example, if the virtual machine
    /// configuration information is not available.
    /// 
    /// ***FileFault***: if there is an error accessing the virtual machine files.
    pub async fn export_snapshot(&self) -> Result<ManagedObjectReference> {
        let path = format!("/VirtualMachineSnapshot/{moId}/ExportSnapshot", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Removes this snapshot and deletes any associated storage.
    /// 
    /// ***Required privileges:*** VirtualMachine.State.RemoveSnapshot
    ///
    /// ## Parameters:
    ///
    /// ### remove_children
    /// Flag to specify removal of the entire snapshot subtree.
    ///
    /// ### consolidate
    /// (optional) If set to true, the virtual disk associated
    /// with this snapshot will be merged with other disk if possible. Defaults to true.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***TaskInProgress***: if the virtual machine is busy.
    pub async fn remove_snapshot_task(&self, remove_children: bool, consolidate: Option<bool>) -> Result<ManagedObjectReference> {
        let input = RemoveSnapshotRequestType {remove_children, consolidate, };
        let path = format!("/VirtualMachineSnapshot/{moId}/RemoveSnapshot_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Rename this snapshot with either a new name or a new description or both.
    /// 
    /// At least one of these must be specified when calling the rename method.
    /// 
    /// ***Required privileges:*** VirtualMachine.State.RenameSnapshot
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// New name for the snapshot.
    ///
    /// ### description
    /// New description for the snapshot.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if the host product does not support snapshot rename.
    /// 
    /// ***InvalidName***: if the specified snapshot name is not valid.
    /// 
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***InvalidPowerState***: if the operation cannot be performed in the current
    /// power state of the virtual machine.
    /// 
    /// ***InvalidState***: if the operation cannot be performed in the current state
    /// of the virtual machine. For example, the virtual machine's configuration
    /// is not available.
    pub async fn rename_snapshot(&self, name: Option<&str>, description: Option<&str>) -> Result<()> {
        let input = RenameSnapshotRequestType {name, description, };
        let path = format!("/VirtualMachineSnapshot/{moId}/RenameSnapshot", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Change the execution state of the virtual machine to the state of this snapshot.
    /// 
    /// ***Required privileges:*** VirtualMachine.State.RevertToSnapshot
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// (optional) Choice of host for the virtual machine, in case this
    /// operation causes the virtual machine to power on.
    /// 
    /// If a snapshot was taken while a virtual machine was powered on, and this operation
    /// is invoked after the virtual machine was powered off, the operation causes the
    /// virtual machine to power on to reach the snapshot state. This parameter can be
    /// used to specify a choice of host where the virtual machine should power on.
    /// 
    /// If this parameter is not set and the vBalance feature is configured for automatic
    /// load balancing, a host is automatically selected. Otherwise, the virtual machine
    /// keeps its existing host affiliation.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### suppress_power_on
    /// (optional) If set to true, the virtual
    /// machine will not be powered on regardless of the power state when
    /// the snapshot was created. Default to false.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***TaskInProgress***: if the virtual machine is busy.
    /// 
    /// ***NotSupported***: if the host product does not support snapshots.
    /// 
    /// ***InsufficientResourcesFault***: if this operation would violate a resource
    /// usage policy.
    /// 
    /// ***InvalidPowerState***: if the operation cannot be performed in the current
    /// power state of the virtual machine.
    /// 
    /// ***InvalidState***: if the operation cannot be performed in the current state
    /// of the virtual machine. For example, the virtual machine's configuration
    /// is not available.
    /// 
    /// ***VmConfigFault***: if a configuration issue prevents the power-on. Typically, a
    /// more specific fault, such as UnsupportedVmxLocation, is thrown.
    /// 
    /// ***FileFault***: if there is a problem accessing the virtual machine on the
    /// filesystem.
    pub async fn revert_to_snapshot_task(&self, host: Option<&ManagedObjectReference>, suppress_power_on: Option<bool>) -> Result<ManagedObjectReference> {
        let input = RevertToSnapshotRequestType {host, suppress_power_on, };
        let path = format!("/VirtualMachineSnapshot/{moId}/RevertToSnapshot_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
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
        let path = format!("/VirtualMachineSnapshot/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/VirtualMachineSnapshot/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// All snapshots for which this snapshot is the parent.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *VirtualMachineSnapshot*.
    pub async fn child_snapshot(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/VirtualMachineSnapshot/{moId}/childSnapshot", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Information about the configuration of this virtual machine when this snapshot was
    /// taken.
    /// 
    /// The datastore paths for the virtual machine disks point to the head of the disk
    /// chain that represents the disk at this given snapshot. The fileInfo.fileLayout
    /// field is not set.
    pub async fn config(&self) -> Result<VirtualMachineConfigInfo> {
        let path = format!("/VirtualMachineSnapshot/{moId}/config", moId = &self.mo_id);
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
        let path = format!("/VirtualMachineSnapshot/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// The virtual machine for which the snapshot was taken.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *VirtualMachine*.
    pub async fn vm(&self) -> Result<ManagedObjectReference> {
        let path = format!("/VirtualMachineSnapshot/{moId}/vm", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveSnapshotRequestType {
    #[serde(rename = "removeChildren")]
    remove_children: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    consolidate: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RenameSnapshotRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RevertToSnapshotRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "suppressPowerOn")]
    suppress_power_on: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(rename = "setCustomValueRequestType", tag = "_typeName")]
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}
