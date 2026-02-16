use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The Snapshot managed object type specifies the interface to individual snapshots
/// of a virtual machine.
/// 
/// Although these are managed objects, they are subordinate to
/// their virtual machine.
#[derive(Clone)]
pub struct VirtualMachineSnapshot {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VirtualMachineSnapshot {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn export_snapshot(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let path = format!("/VirtualMachineSnapshot/{moId}/ExportSnapshot", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn remove_snapshot_task(&self, remove_children: bool, consolidate: Option<bool>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RemoveSnapshotRequestType {remove_children, consolidate, };
        let path = format!("/VirtualMachineSnapshot/{moId}/RemoveSnapshot_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
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
    pub async fn revert_to_snapshot_task(&self, host: Option<&crate::types::structs::ManagedObjectReference>, suppress_power_on: Option<bool>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RevertToSnapshotRequestType {host, suppress_power_on, };
        let path = format!("/VirtualMachineSnapshot/{moId}/RevertToSnapshot_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
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
        let path = format!("/VirtualMachineSnapshot/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<crate::types::structs::CustomFieldDef>>> {
        let path = format!("/VirtualMachineSnapshot/{moId}/availableField", moId = &self.mo_id);
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
    /// All snapshots for which this snapshot is the parent.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *VirtualMachineSnapshot*.
    pub async fn child_snapshot(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let path = format!("/VirtualMachineSnapshot/{moId}/childSnapshot", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::ManagedObjectReference>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Information about the configuration of this virtual machine when this snapshot was
    /// taken.
    /// 
    /// The datastore paths for the virtual machine disks point to the head of the disk
    /// chain that represents the disk at this given snapshot. The fileInfo.fileLayout
    /// field is not set.
    pub async fn config(&self) -> Result<crate::types::structs::VirtualMachineConfigInfo> {
        let path = format!("/VirtualMachineSnapshot/{moId}/config", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::VirtualMachineConfigInfo = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
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
        let path = format!("/VirtualMachineSnapshot/{moId}/value", moId = &self.mo_id);
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
    /// The virtual machine for which the snapshot was taken.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *VirtualMachine*.
    pub async fn vm(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let path = format!("/VirtualMachineSnapshot/{moId}/vm", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct RemoveSnapshotRequestType {
    remove_children: bool,
    consolidate: Option<bool>,
}

impl miniserde::Serialize for RemoveSnapshotRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveSnapshotRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveSnapshotRequestTypeSer<'b> {
    data: &'b RemoveSnapshotRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for RemoveSnapshotRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveSnapshotRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("removeChildren"), &self.data.remove_children as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.consolidate else { continue; };
                    return Some((std::borrow::Cow::Borrowed("consolidate"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RenameSnapshotRequestType<'a> {
    name: Option<&'a str>,
    description: Option<&'a str>,
}

impl<'a> miniserde::Serialize for RenameSnapshotRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RenameSnapshotRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RenameSnapshotRequestTypeSer<'b, 'a> {
    data: &'b RenameSnapshotRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RenameSnapshotRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RenameSnapshotRequestType")),
                1 => {
                    let Some(ref val) = self.data.name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("name"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.description else { continue; };
                    return Some((std::borrow::Cow::Borrowed("description"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RevertToSnapshotRequestType<'a> {
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
    suppress_power_on: Option<bool>,
}

impl<'a> miniserde::Serialize for RevertToSnapshotRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RevertToSnapshotRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RevertToSnapshotRequestTypeSer<'b, 'a> {
    data: &'b RevertToSnapshotRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RevertToSnapshotRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RevertToSnapshotRequestType")),
                1 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.suppress_power_on else { continue; };
                    return Some((std::borrow::Cow::Borrowed("suppressPowerOn"), val as &dyn miniserde::Serialize));
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
