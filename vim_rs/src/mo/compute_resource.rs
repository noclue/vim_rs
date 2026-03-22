use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Represents a set of physical compute resources for a set of virtual machines.
/// 
/// The base type *ComputeResource*, when instantiated by calling
/// *Folder.AddStandaloneHost_Task*, represents a single host. The subclass
/// *ClusterComputeResource* represents a cluster of hosts and adds distributed management
/// features such as availability and resource scheduling.
/// 
/// A *ComputeResource* always has a root *ResourcePool* associated with it.
/// Certain types of clusters such as those with VMware DRS enabled and standalone hosts
/// (ESX Server 3) support the creation of *ResourcePool* hierarchies.
#[derive(Clone)]
pub struct ComputeResource {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl ComputeResource {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    /// ***Required privileges:*** Host.Inventory.RemoveHostFromCluster
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
    pub async fn destroy_task(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let bytes = self.client.invoke("", "ComputeResource", &self.mo_id, "Destroy_Task", None).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Disable network boot support for this compute resource.
    /// 
    /// This configuration can be modified only when the compute resource is
    /// empty i.e. there are no hosts in it.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation progress and result.
    /// 
    /// Refers instance of *Task*.
    pub async fn disable_network_boot_task(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let bytes = self.client.invoke("", "ComputeResource", &self.mo_id, "DisableNetworkBoot_Task", None).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Enable network boot in the specified mode for this compute resource.
    /// 
    /// Supported values are enumerated in
    /// *ComputeResourceNetworkBootMode_enum*. This configuration can be
    /// modified only when the compute resource is empty i.e. there are no hosts
    /// in it or during compute resource creation. In addition transition to some
    /// network boot mode(s) may be restricted depending on the current state of
    /// the compute resource.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    ///
    /// ## Parameters:
    ///
    /// ### network_boot_mode
    /// -
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation progress and result.
    /// 
    /// Refers instance of *Task*.
    pub async fn enable_network_boot_task(&self, network_boot_mode: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = EnableNetworkBootRequestType {network_boot_mode, };
        let bytes = self.client.invoke("", "ComputeResource", &self.mo_id, "EnableNetworkBoot_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Change the compute resource configuration.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// A set of configuration changes to apply to the compute resource.
    /// The specification can be a complete set of changes or a partial
    /// set of changes, applied incrementally. When invoking
    /// reconfigureEx on a cluster, this argument may be a
    /// *ClusterConfigSpecEx* object.
    ///
    /// ### modify
    /// Flag to specify whether the specification ("spec") should
    /// be applied incrementally. If "modify" is false and the
    /// operation succeeds, then the configuration of the cluster
    /// matches the specification exactly; in this case any unset
    /// portions of the specification will result in unset or
    /// default portions of the configuration.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation.
    /// 
    /// Refers instance of *Task*.
    pub async fn reconfigure_compute_resource_task(&self, spec: &dyn crate::types::traits::ComputeResourceConfigSpecTrait, modify: bool) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ReconfigureComputeResourceRequestType {spec, modify, };
        let bytes = self.client.invoke("", "ComputeResource", &self.mo_id, "ReconfigureComputeResource_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        self.client.invoke_void("", "ComputeResource", &self.mo_id, "Reload", None).await
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
    pub async fn rename_task(&self, new_name: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RenameRequestType {new_name, };
        let bytes = self.client.invoke("", "ComputeResource", &self.mo_id, "Rename_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
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
        self.client.invoke_void("", "ComputeResource", &self.mo_id, "setCustomValue", Some(&input)).await
    }
    /// Whether alarm actions are enabled for this entity.
    /// 
    /// True if enabled; false otherwise.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn alarm_actions_enabled(&self) -> Result<Option<bool>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "alarmActionsEnabled").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<crate::types::structs::CustomFieldDef>>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "availableField").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Current configuration issues that have been detected for this entity.
    /// 
    /// Typically,
    /// these issues have already been logged as events. The entity stores these
    /// events as long as they are still current. The
    /// *configStatus* property provides an overall status
    /// based on these events.
    pub async fn config_issue(&self) -> Result<Option<Vec<crate::types::structs::Event>>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "configIssue").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Flag indicating whether or not desired configuration
    /// management platform is enabled on the compute resource.
    /// 
    /// This property can be set only at the time of creation or through the
    /// *ComputeResource.EnableConfigurationManagement* method.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.0
    /// 
    /// ***Required privileges:*** System.View
    pub async fn config_manager_enabled(&self) -> Result<Option<bool>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "configManagerEnabled").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
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
    pub async fn config_status(&self) -> Result<crate::types::enums::ManagedEntityStatusEnum> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "configStatus").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property configStatus was empty".to_string()))?;
        let result: crate::types::enums::ManagedEntityStatusEnum = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// Configuration of the compute resource; applies to both standalone hosts
    /// and clusters.
    /// 
    /// For a cluster this property will return a
    /// *ClusterConfigInfoEx* object.
    pub async fn configuration_ex(&self) -> Result<Box<dyn crate::types::traits::ComputeResourceConfigInfoTrait>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "configurationEx").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property configurationEx was empty".to_string()))?;
        let result: Box<dyn crate::types::traits::ComputeResourceConfigInfoTrait> = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// Custom field values.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn custom_value(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "customValue").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// The datastore property is the subset of datastore objects in the datacenter
    /// available in this ComputeResource.
    /// 
    /// This property is computed as the aggregate set of datastores available from all
    /// the hosts that are part of this compute resource.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Datastore*.
    pub async fn datastore(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "datastore").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
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
    pub async fn declared_alarm_state(&self) -> Result<Option<Vec<crate::types::structs::AlarmState>>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "declaredAlarmState").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
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
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "disabledMethod").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Access rights the current session has to this entity.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn effective_role(&self) -> Result<Option<Vec<i32>>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "effectiveRole").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// The environment browser object that identifies the environments that are supported
    /// on this compute resource.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instance of *EnvironmentBrowser*.
    pub async fn environment_browser(&self) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "environmentBrowser").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// List of hosts that are part of this compute resource.
    /// 
    /// If the compute resource is a
    /// standalone type, then this list contains just one element.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *HostSystem*.
    pub async fn host(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "host").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Flag indicating whether or not the lifecycle of the compute resource is
    /// managed.
    /// 
    /// Once it is enabled, it cannot be disabled.
    /// This property can be set only at the time of creation or through the
    /// *ComputeResource.EnableLifecycleManagement* method.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn lifecycle_managed(&self) -> Result<Option<bool>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "lifecycleManaged").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
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
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "name").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property name was empty".to_string()))?;
        let result: String = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// The subset of network objects available in the datacenter that is available in
    /// this ComputeResource.
    /// 
    /// This property is computed as the aggregate set of networks available from all the
    /// hosts that are part of this compute resource.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Network*.
    pub async fn network(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "network").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Managed property indicating whether and what kind of netwoork boot mode
    /// is configured for this compute resource.
    /// 
    /// Supported values are enumerated
    /// in *ComputeResourceNetworkBootMode_enum*.
    /// This property can be configured via the
    /// *ComputeResource.EnableNetworkBoot_Task* method or during compute
    /// resource creation through the
    /// *ComputeResource.networkBootMode* property.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    /// 
    /// ***Required privileges:*** System.View
    pub async fn network_boot_mode(&self) -> Result<Option<String>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "networkBootMode").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
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
    pub async fn overall_status(&self) -> Result<crate::types::enums::ManagedEntityStatusEnum> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "overallStatus").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property overallStatus was empty".to_string()))?;
        let result: crate::types::enums::ManagedEntityStatusEnum = crate::core::client::extract_property(pv)?;
        Ok(result)
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
    pub async fn parent(&self) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "parent").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// List of permissions defined for this entity.
    pub async fn permission(&self) -> Result<Option<Vec<crate::types::structs::Permission>>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "permission").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
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
    pub async fn recent_task(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "recentTask").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Reference to root resource pool.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instance of *ResourcePool*.
    pub async fn resource_pool(&self) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "resourcePool").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Basic runtime information about a compute resource.
    /// 
    /// This information is used on
    /// summary screens and in list views.
    pub async fn summary(&self) -> Result<Box<dyn crate::types::traits::ComputeResourceSummaryTrait>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "summary").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property summary was empty".to_string()))?;
        let result: Box<dyn crate::types::traits::ComputeResourceSummaryTrait> = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// The set of tags associated with this managed entity.
    /// 
    /// Experimental. Subject to change.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn tag(&self) -> Result<Option<Vec<crate::types::structs::Tag>>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "tag").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
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
    pub async fn triggered_alarm_state(&self) -> Result<Option<Vec<crate::types::structs::AlarmState>>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "triggeredAlarmState").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// List of custom field values.
    /// 
    /// Each value uses a key to associate
    /// an instance of a *CustomFieldStringValue* with
    /// a custom field definition.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>> {
        let pv_opt = self.client.fetch_property_raw("", "ComputeResource", &self.mo_id, "value").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
}
struct EnableNetworkBootRequestType<'a> {
    network_boot_mode: &'a str,
}

impl<'a> miniserde::Serialize for EnableNetworkBootRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(EnableNetworkBootRequestTypeSer { data: self, seq: 0 }))
    }
}

struct EnableNetworkBootRequestTypeSer<'b, 'a> {
    data: &'b EnableNetworkBootRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for EnableNetworkBootRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"EnableNetworkBootRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("networkBootMode"), &self.data.network_boot_mode as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ReconfigureComputeResourceRequestType<'a> {
    spec: &'a dyn crate::types::traits::ComputeResourceConfigSpecTrait,
    modify: bool,
}

impl<'a> miniserde::Serialize for ReconfigureComputeResourceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReconfigureComputeResourceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReconfigureComputeResourceRequestTypeSer<'b, 'a> {
    data: &'b ReconfigureComputeResourceRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ReconfigureComputeResourceRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReconfigureComputeResourceRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("modify"), &self.data.modify as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RenameRequestType<'a> {
    new_name: &'a str,
}

impl<'a> miniserde::Serialize for RenameRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RenameRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RenameRequestTypeSer<'b, 'a> {
    data: &'b RenameRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RenameRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RenameRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("newName"), &self.data.new_name as &dyn miniserde::Serialize)),
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
