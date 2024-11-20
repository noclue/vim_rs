use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::CustomFieldValueTrait;
use crate::types::CustomFieldDef;
use crate::types::Permission;
use crate::types::Tag;
use crate::types::AlarmState;
use crate::types::EventTrait;
use crate::types::StoragePlacementResult;
use crate::types::DatastoreSummary;
use crate::types::DatastoreCapability;
use crate::types::ManagedObjectReference;
use crate::types::DatastoreInfoTrait;
use crate::types::StorageIormInfo;
use crate::types::DatastoreMountPathDatastorePair;
use crate::types::DatastoreHostMount;
use crate::types::DatastoreVVolContainerFailoverPair;
use crate::types::ManagedEntityStatusEnum;
/// Represents a storage location for virtual machine files.
/// 
/// A storage location can be a
/// VMFS volume, a directory on Network Attached Storage, or a local file system path.
/// 
/// A datastore is platform-independent and host-independent. Therefore, datastores do
/// not change when the virtual machines they contain are moved between hosts. The scope
/// of a datastore is a datacenter; the datastore is uniquely named within the
/// datacenter.
/// 
/// Any reference to a virtual machine or file accessed by any host within the
/// datacenter must use a datastore path. A datastore path has the form
/// "\[&lt;datastore&gt;\] &lt;path&gt;", where &lt;datastore&gt; is the datastore name,
/// and &lt;path&gt; is a slash-delimited path from the root of the datastore. An
/// example datastore path is "\[storage\] path/to/config.vmx".
/// 
/// You may use the following characters in a path, but not in a datastore name:
/// slash (/), backslash (\\), and percent (%).
/// 
/// All references to files in the VIM API are implicitly done using datastore paths.
/// 
/// When a client creates a virtual machine, it may specify the name of
/// the datastore, omitting the path; the system, meaning VirtualCenter or the host,
/// automatically assigns filenames and creates directories on the given datastore. For
/// example, specifying My\_Datastore as a location for a virtual machine called MyVm
/// results in a datastore location of My\_Datastore\\MyVm\\MyVm.vmx.
/// 
/// Datastores are configured per host. As part of host configuration, a HostSystem can
/// be configured to mount a set of network drives. Multiple hosts
/// may be configured to point to the same storage location. There exists only one
/// Datastore object per Datacenter, for each such shared location. Each Datastore
/// object keeps a reference to the set of hosts that have mounted the datastore. A
/// Datastore object can be removed only if no hosts currently have the datastore
/// mounted.
/// 
/// Thus, managing datastores is done both at the host level and the datacenter level.
/// Each host is configured explicitly with the set of datastores it can access. At the
/// datacenter, a view of the datastores across the datacenter is shown.
pub struct Datastore {
    client: Arc<VimClient>,
    mo_id: String,
}
impl Datastore {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Destroys this object, deleting its contents and removing it from its parent
    /// folder (if any).
    /// 
    /// NOTE: The appropriate privilege must be held on the parent of the destroyed
    /// entity as well as the entity itself.
    /// This method can throw one of several exceptions. The exact set of exceptions
    /// depends on the kind of entity that is being removed. See comments for
    /// each entity for more information on destroy behavior.
    /// 
    /// ***Required privileges:*** System.Read
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
    /// Failure
    pub async fn destroy_task(&self) -> Result<ManagedObjectReference> {
        let path = format!("/Datastore/{moId}/Destroy_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of VI API 2.5 do not use this method. This method throws
    /// *ResourceInUse*. Datastores are automatically
    /// removed when no longer in use, so this method is unnecessary.
    /// 
    /// Removes a datastore.
    /// 
    /// A datastore can be removed only if it is not currently used
    /// by any host or virtual machine.
    /// 
    /// ***Required privileges:*** Datastore.Delete
    ///
    /// ## Errors:
    ///
    /// ***ResourceInUse***: if one or more hosts or virtual machines are configured
    /// to use the datastore.
    pub async fn destroy_datastore(&self) -> Result<()> {
        let path = format!("/Datastore/{moId}/DestroyDatastore", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Puts the datastore in maintenance mode.
    /// 
    /// While this task is running and when the
    /// datastore is in maintenance mode, no virtual machines can be powered on and no
    /// provisioning operations can be performed on the datastore. Once the call
    /// completes, it is safe to remove datastore without disrupting any virtual machines.
    /// 
    /// The task completes once there are no virtual machines on the datastore
    /// and no provisioning operations in progress on the datastore. The operation does not
    /// directly initiate any operations to evacuate or power-down powered-on virtual machines.
    /// However, if the datastore is part of a storage pod with VMware Storage DRS enabled,
    /// Storage DRS provides migration recommendations to evacuate the virtual
    /// machines. If Storage DRS is in fully-automatic mode, these are automatically scheduled.
    /// The task is cancellable.
    /// This method returns a *StoragePlacementResult* object, which includes
    /// a *Task* object with which to monitor the operation, and a list of recommendations
    /// and faults generated by Storage DRS when it tries to evacuate the virtual machines
    /// on the datastore. The recommendations and faults fields are set only if the datastore is
    /// a part of a storage pod with Storage DRS enabled.
    /// 
    /// ***Required privileges:*** Datastore.Config
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the datastore is already in maintenance mode.
    /// 
    /// ***RequestCanceled***: if the operation is canceled.
    pub async fn datastore_enter_maintenance_mode(&self) -> Result<StoragePlacementResult> {
        let path = format!("/Datastore/{moId}/DatastoreEnterMaintenanceMode", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Takes the datastore out of maintenance mode.
    /// 
    /// The task is cancellable.
    /// 
    /// ***Required privileges:*** Datastore.Config
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
    /// ***InvalidState***: if the datastore is not in maintenance mode.
    pub async fn datastore_exit_maintenance_mode_task(&self) -> Result<ManagedObjectReference> {
        let path = format!("/Datastore/{moId}/DatastoreExitMaintenanceMode_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Check whether clustered VMDK feature is enabled on this datastore.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// true if clustered VMDK feature is enabled. false otherwise.
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: if the given datastore is not valid.
    pub async fn is_clustered_vmdk_enabled(&self) -> Result<bool> {
        let path = format!("/Datastore/{moId}/IsClusteredVmdkEnabled", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Explicitly refreshes free-space and capacity values in *Datastore.summary*
    /// and *Datastore.info*.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the datastore or its underlying volume is not found.
    /// 
    /// ***HostConfigFault***: if unable to get the current system information
    /// for the datastore.
    pub async fn refresh_datastore(&self) -> Result<()> {
        let path = format!("/Datastore/{moId}/RefreshDatastore", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Refreshes all storage related information including free-space, capacity,
    /// and detailed usage of virtual machines.
    /// 
    /// Updated values are available
    /// in *Datastore.summary* and *Datastore.info*.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn refresh_datastore_storage_info(&self) -> Result<()> {
        let path = format!("/Datastore/{moId}/RefreshDatastoreStorageInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Reload the entity state.
    /// 
    /// Clients only need to call this method
    /// if they changed some external state that affects the service
    /// without using the Web service interface to perform the change.
    /// For example, hand-editing a virtual machine configuration file
    /// affects the configuration of the associated virtual machine but
    /// the service managing the virtual machine might not monitor the
    /// file for changes. In this case, after such an edit, a client
    /// would call "reload" on the associated virtual machine to ensure
    /// the service and its clients have current data for the
    /// virtual machine.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn reload(&self) -> Result<()> {
        let path = format!("/Datastore/{moId}/Reload", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Renames this managed entity.
    /// 
    /// Any % (percent) character used in this name parameter
    /// must be escaped, unless it is used to start an escape
    /// sequence. Clients may also escape any other characters in
    /// this name parameter.
    /// 
    /// See also *ManagedEntity.name*.
    /// 
    /// ***Required privileges:*** Datastore.Rename
    ///
    /// ## Parameters:
    ///
    /// ### new_name
    /// -
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
    /// ***DuplicateName***: If another object in the same folder has the target name.
    /// 
    /// ***InvalidName***: If the new name is not a valid entity name.
    pub async fn rename_task(&self, new_name: &str) -> Result<ManagedObjectReference> {
        let input = RenameRequestType {new_name, };
        let path = format!("/Datastore/{moId}/Rename_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of vSphere API 4.0, use *ManagedEntity.Rename_Task*.
    /// 
    /// Renames a datastore.
    /// 
    /// ***Required privileges:*** Datastore.Rename
    ///
    /// ## Parameters:
    ///
    /// ### new_name
    /// The new name to assign to the datastore.
    ///
    /// ## Errors:
    ///
    /// ***DuplicateName***: if another datastore in this datacenter already
    /// has the same name.
    /// 
    /// ***InvalidName***: if the name is not a valid datastore name.
    pub async fn rename_datastore(&self, new_name: &str) -> Result<()> {
        let input = RenameDatastoreRequestType {new_name, };
        let path = format!("/Datastore/{moId}/RenameDatastore", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
        let path = format!("/Datastore/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Scan a VVol storage container to update file paths and objectID pointers
    /// embedded in virtual machine files on a given storage container.
    /// 
    /// ***Required privileges:*** Datastore.UpdateVirtualMachineFiles
    ///
    /// ## Parameters:
    ///
    /// ### failover_pair
    /// Mapping of source to target storage container
    /// as well as source to target VVol IDs.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation. The *TaskInfo.result* property in the
    /// *Task* contains a
    /// *VVolVmConfigFileUpdateResult* object, which
    /// provides a list of successfully updated virtual machine
    /// config files and a list of virtual machine config files
    /// that failed to update, for all virtual machine config files
    /// failed over onto the VVol storage container from the source
    /// containers in the failover pair.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if all hosts attached to this datastore do not
    /// support updating VVol Virtual Machine Files.
    /// 
    /// ***PlatformConfigFault***: if any error related to platform
    /// occurs during the operation.
    /// 
    /// ***TaskInProgress***: if the datastore is busy, for example, while
    /// another task is updating the datastore.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed due to
    /// some error with the datastore.
    pub async fn update_v_vol_virtual_machine_files_task(&self, failover_pair: Option<&[DatastoreVVolContainerFailoverPair]>) -> Result<ManagedObjectReference> {
        let input = UpdateVVolVirtualMachineFilesRequestType {failover_pair, };
        let path = format!("/Datastore/{moId}/UpdateVVolVirtualMachineFiles_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Update file paths embedded in virtual machine files on the datastore.
    /// 
    /// This can be called after the file system corresponding to the datastore
    /// has been resignatured or remounted. Any MountPathDatastorePairs
    /// where the new path is the same as the original file path will be ignored.
    /// 
    /// This method is only supported by vCenter server. Also, this method requires
    /// that the datastore is *accessible* from
    /// at least one host (ESX version 4.1 or above) in vCenter server.
    /// 
    /// While this operation is in progress, it is important that users do not
    /// initiate any operations that might read or write to any files on the
    /// datastore, such as registering a virtual machine with files residing
    /// on the datastore, or performing any virtual disk operations on any files
    /// in the datastore. These operations can potentially cause spurious file
    /// update failures, while at the same time they can prevent virtual machine
    /// files from being updated correctly.
    /// 
    /// If users intend to update multiple datastores using this method, it is
    /// strongly advised that the users do not initiate any operations that
    /// might read or write to files on any of the datastores, until all of
    /// them have been updated. The files of a single virtual machine can
    /// reside on multiple datastores, and thus all the involved datastores
    /// should be updated, before the virtual machine is considered updated
    /// completely.
    /// 
    /// ***Required privileges:*** Datastore.UpdateVirtualMachineFiles
    ///
    /// ## Parameters:
    ///
    /// ### mount_path_datastore_mapping
    /// Old mount path to datastore mapping.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation. The *TaskInfo.result* property in the
    /// *Task* contains an *UpdateVirtualMachineFilesResult*
    /// object, upon success, which is a list of virtual machines files the
    /// server has attempted to update but failed to update. When there are too
    /// many failed files, only a subset of failed files will be returned.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if old mount path is mapped to more than one datastores,
    /// or if any of the datastore being mapped can not be found.
    /// 
    /// ***NotSupported***: if all hosts attached to this datastore do not support
    /// updating virtual machine files.
    /// 
    /// ***ResourceInUse***: if there exists a registered virtual machine in the volume.
    /// 
    /// ***PlatformConfigFault***: if any error related to platform
    /// occurs during the operation.
    /// 
    /// ***TaskInProgress***: if the datastore is busy, for example, while another task
    /// is updating the datastore after volume resignaturing
    /// or remounting.
    /// 
    /// ***InvalidDatastore***: if the operation cannot be performed due to some error
    /// with the datastore; typically a specific subclass
    /// of the fault is reported.
    pub async fn update_virtual_machine_files_task(&self, mount_path_datastore_mapping: &[DatastoreMountPathDatastorePair]) -> Result<ManagedObjectReference> {
        let input = UpdateVirtualMachineFilesRequestType {mount_path_datastore_mapping, };
        let path = format!("/Datastore/{moId}/UpdateVirtualMachineFiles_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Whether alarm actions are enabled for this entity.
    /// 
    /// True if enabled; false otherwise.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn alarm_actions_enabled(&self) -> Result<Option<bool>> {
        let path = format!("/Datastore/{moId}/alarmActionsEnabled", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/Datastore/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// DatastoreBrowser used to browse this datastore.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *HostDatastoreBrowser*.
    pub async fn browser(&self) -> Result<ManagedObjectReference> {
        let path = format!("/Datastore/{moId}/browser", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Capabilities of this datastore.
    pub async fn capability(&self) -> Result<DatastoreCapability> {
        let path = format!("/Datastore/{moId}/capability", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Current configuration issues that have been detected for this entity.
    /// 
    /// Typically,
    /// these issues have already been logged as events. The entity stores these
    /// events as long as they are still current. The
    /// *configStatus* property provides an overall status
    /// based on these events.
    pub async fn config_issue(&self) -> Result<Option<Vec<Box<dyn EventTrait>>>> {
        let path = format!("/Datastore/{moId}/configIssue", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// The configStatus indicates whether or not the system has detected a configuration
    /// issue involving this entity.
    /// 
    /// For example, it might have detected a
    /// duplicate IP address or MAC address, or a host in a cluster
    /// might be out of compliance. The meanings of the configStatus values are:
    /// - red: A problem has been detected involving the entity.
    /// - yellow: A problem is about to occur or a transient condition
    ///   has occurred (For example, reconfigure fail-over policy).
    /// - green: No configuration issues have been detected.
    /// - gray: The configuration status of the entity is not being monitored.
    ///   
    /// A green status indicates only that a problem has not been detected;
    /// it is not a guarantee that the entity is problem-free.
    /// 
    /// The *configIssue* property contains a list of the
    /// problems that have been detected.
    /// In releases after vSphere API 5.0, vSphere Servers might not
    /// generate property collector update notifications for this property.
    /// To obtain the latest value of the property, you can use
    /// PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
    /// If you use the PropertyCollector.WaitForUpdatesEx method, specify
    /// an empty string for the version parameter. Any other version value will not
    /// produce any property values as no updates are generated.
    pub async fn config_status(&self) -> Result<ManagedEntityStatusEnum> {
        let path = format!("/Datastore/{moId}/configStatus", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Custom field values.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn custom_value(&self) -> Result<Option<Vec<Box<dyn CustomFieldValueTrait>>>> {
        let path = format!("/Datastore/{moId}/customValue", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// A set of alarm states for alarms that apply to this managed entity.
    /// 
    /// The set includes alarms defined on this entity
    /// and alarms inherited from the parent entity,
    /// or from any ancestors in the inventory hierarchy.
    /// 
    /// Alarms are inherited if they can be triggered by this entity or its descendants.
    /// This set does not include alarms that are defined on descendants of this entity.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn declared_alarm_state(&self) -> Result<Option<Vec<AlarmState>>> {
        let path = format!("/Datastore/{moId}/declaredAlarmState", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// List of operations that are disabled, given the current runtime
    /// state of the entity.
    /// 
    /// For example, a power-on operation always fails if a
    /// virtual machine is already powered on. This list can be used by clients to
    /// enable or disable operations in a graphical user interface.
    /// 
    /// Note: This list is determined by the current runtime state of an entity,
    /// not by its permissions.
    /// 
    /// This list may include the following operations for a HostSystem:
    /// - *HostSystem.EnterMaintenanceMode_Task*
    /// - *HostSystem.ExitMaintenanceMode_Task*
    /// - *HostSystem.RebootHost_Task*
    /// - *HostSystem.ShutdownHost_Task*
    /// - *HostSystem.ReconnectHost_Task*
    /// - *HostSystem.DisconnectHost_Task*
    ///   
    /// This list may include the following operations for a VirtualMachine:
    /// - *VirtualMachine.AnswerVM*
    /// - *ManagedEntity.Rename_Task*
    /// - *VirtualMachine.CloneVM_Task*
    /// - *VirtualMachine.PowerOffVM_Task*
    /// - *VirtualMachine.PowerOnVM_Task*
    /// - *VirtualMachine.SuspendVM_Task*
    /// - *VirtualMachine.ResetVM_Task*
    /// - *VirtualMachine.ReconfigVM_Task*
    /// - *VirtualMachine.RelocateVM_Task*
    /// - *VirtualMachine.MigrateVM_Task*
    /// - *VirtualMachine.CustomizeVM_Task*
    /// - *VirtualMachine.ShutdownGuest*
    /// - *VirtualMachine.StandbyGuest*
    /// - *VirtualMachine.RebootGuest*
    /// - *VirtualMachine.CreateSnapshot_Task*
    /// - *VirtualMachine.RemoveAllSnapshots_Task*
    /// - *VirtualMachine.RevertToCurrentSnapshot_Task*
    /// - *VirtualMachine.MarkAsTemplate*
    /// - *VirtualMachine.MarkAsVirtualMachine*
    /// - *VirtualMachine.ResetGuestInformation*
    /// - *VirtualMachine.MountToolsInstaller*
    /// - *VirtualMachine.UnmountToolsInstaller*
    /// - *ManagedEntity.Destroy_Task*
    /// - *VirtualMachine.UpgradeVM_Task*
    /// - *VirtualMachine.ExportVm*
    ///   
    /// This list may include the following operations for a ResourcePool:
    /// - *ResourcePool.ImportVApp*
    /// - *ResourcePool.CreateChildVM_Task*
    /// - *ResourcePool.UpdateConfig*
    /// - *Folder.CreateVM_Task*
    /// - *ManagedEntity.Destroy_Task*
    /// - *ManagedEntity.Rename_Task*
    ///   
    /// This list may include the following operations for a VirtualApp:
    /// - *ManagedEntity.Destroy_Task*
    /// - *VirtualApp.CloneVApp_Task*
    /// - *VirtualApp.unregisterVApp_Task*
    /// - *VirtualApp.ExportVApp*
    /// - *VirtualApp.PowerOnVApp_Task*
    /// - *VirtualApp.PowerOffVApp_Task*
    /// - *VirtualApp.UpdateVAppConfig*
    ///   
    /// In releases after vSphere API 5.0, vSphere Servers might not
    /// generate property collector update notifications for this property.
    /// To obtain the latest value of the property, you can use
    /// PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
    /// If you use the PropertyCollector.WaitForUpdatesEx method, specify
    /// an empty string for the version parameter. Any other version value will not
    /// produce any property values as no updates are generated.
    pub async fn disabled_method(&self) -> Result<Option<Vec<String>>> {
        let path = format!("/Datastore/{moId}/disabledMethod", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Access rights the current session has to this entity.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn effective_role(&self) -> Result<Option<Vec<i32>>> {
        let path = format!("/Datastore/{moId}/effectiveRole", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Hosts attached to this datastore.
    pub async fn host(&self) -> Result<Option<Vec<DatastoreHostMount>>> {
        let path = format!("/Datastore/{moId}/host", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Specific information about the datastore.
    pub async fn info(&self) -> Result<Box<dyn DatastoreInfoTrait>> {
        let path = format!("/Datastore/{moId}/info", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Configuration of storage I/O resource management for the datastore.
    /// 
    /// Currently we only support storage I/O resource management on VMFS volumes
    /// of a datastore.
    /// 
    /// This configuration may not be available if the datastore is not accessible
    /// from any host, or if the datastore does not have VMFS volume.
    /// The configuration can be modified using the method
    /// *StorageResourceManager.ConfigureDatastoreIORM_Task*
    pub async fn iorm_configuration(&self) -> Result<StorageIormInfo> {
        let path = format!("/Datastore/{moId}/iormConfiguration", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Name of this entity, unique relative to its parent.
    /// 
    /// Any / (slash), \\ (backslash), character used in this
    /// name element will be escaped. Similarly, any % (percent) character used in
    /// this name element will be escaped, unless it is used to start an escape
    /// sequence. A slash is escaped as %2F or %2f. A backslash is escaped as %5C or
    /// %5c, and a percent is escaped as %25.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn name(&self) -> Result<String> {
        let path = format!("/Datastore/{moId}/name", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// General health of this managed entity.
    /// 
    /// The overall status of the managed entity is computed as the worst status
    /// among its alarms and the configuration issues detected on the entity.
    /// The status is reported as one of the following values:
    /// - red: The entity has alarms or configuration issues with a red status.
    /// - yellow: The entity does not have alarms or configuration issues with a
    ///   red status, and has at least one with a yellow status.
    /// - green: The entity does not have alarms or configuration issues with a
    ///   red or yellow status, and has at least one with a green status.
    /// - gray: All of the entity's alarms have a gray status and the
    ///   configuration status of the entity is not being monitored.
    ///   
    /// In releases after vSphere API 5.0, vSphere Servers might not
    /// generate property collector update notifications for this property.
    /// To obtain the latest value of the property, you can use
    /// PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
    /// If you use the PropertyCollector.WaitForUpdatesEx method, specify
    /// an empty string for the version parameter. Any other version value will not
    /// produce any property values as no updates are generated.
    pub async fn overall_status(&self) -> Result<ManagedEntityStatusEnum> {
        let path = format!("/Datastore/{moId}/overallStatus", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Parent of this entity.
    /// 
    /// This value is null for the root object and for
    /// *VirtualMachine* objects that are part of
    /// a *VirtualApp*.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instance of *ManagedEntity*.
    pub async fn parent(&self) -> Result<ManagedObjectReference> {
        let path = format!("/Datastore/{moId}/parent", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// List of permissions defined for this entity.
    pub async fn permission(&self) -> Result<Option<Vec<Permission>>> {
        let path = format!("/Datastore/{moId}/permission", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// The set of recent tasks operating on this managed entity.
    /// 
    /// This is a subset
    /// of *TaskManager.recentTask* belong to this entity. A task in this
    /// list could be in one of the four states: pending, running, success or error.
    /// 
    /// This property can be used to deduce intermediate power states for
    /// a virtual machine entity. For example, if the current powerState is "poweredOn"
    /// and there is a running task performing the "suspend" operation, then the virtual
    /// machine's intermediate state might be described as "suspending."
    /// 
    /// Most tasks (such as power operations) obtain exclusive access to the virtual
    /// machine, so it is unusual for this list to contain more than one running task.
    /// One exception, however, is the task of cloning a virtual machine.
    /// In releases after vSphere API 5.0, vSphere Servers might not
    /// generate property collector update notifications for this property.
    /// To obtain the latest value of the property, you can use
    /// PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
    /// If you use the PropertyCollector.WaitForUpdatesEx method, specify
    /// an empty string for the version parameter. Any other version value will not
    /// produce any property values as no updates are generated.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Task*.
    pub async fn recent_task(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/Datastore/{moId}/recentTask", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Global properties of the datastore.
    pub async fn summary(&self) -> Result<DatastoreSummary> {
        let path = format!("/Datastore/{moId}/summary", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// The set of tags associated with this managed entity.
    /// 
    /// Experimental. Subject to change.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn tag(&self) -> Result<Option<Vec<Tag>>> {
        let path = format!("/Datastore/{moId}/tag", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// A set of alarm states for alarms triggered by this entity
    /// or by its descendants.
    /// 
    /// Triggered alarms are propagated up the inventory hierarchy
    /// so that a user can readily tell when a descendant has triggered an alarm.
    /// In releases after vSphere API 5.0, vSphere Servers might not
    /// generate property collector update notifications for this property.
    /// To obtain the latest value of the property, you can use
    /// PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
    /// If you use the PropertyCollector.WaitForUpdatesEx method, specify
    /// an empty string for the version parameter. Any other version value will not
    /// produce any property values as no updates are generated.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn triggered_alarm_state(&self) -> Result<Option<Vec<AlarmState>>> {
        let path = format!("/Datastore/{moId}/triggeredAlarmState", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// List of custom field values.
    /// 
    /// Each value uses a key to associate
    /// an instance of a *CustomFieldStringValue* with
    /// a custom field definition.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn CustomFieldValueTrait>>>> {
        let path = format!("/Datastore/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Virtual machines stored on this datastore.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *VirtualMachine*.
    pub async fn vm(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/Datastore/{moId}/vm", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RenameRequestType<'a> {
    #[serde(rename = "newName")]
    new_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RenameDatastoreRequestType<'a> {
    #[serde(rename = "newName")]
    new_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "setCustomValueRequestType", tag = "_typeName")]
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateVVolVirtualMachineFilesRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "failoverPair")]
    failover_pair: Option<&'a [DatastoreVVolContainerFailoverPair]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateVirtualMachineFilesRequestType<'a> {
    #[serde(rename = "mountPathDatastoreMapping")]
    mount_path_datastore_mapping: &'a [DatastoreMountPathDatastorePair],
}
