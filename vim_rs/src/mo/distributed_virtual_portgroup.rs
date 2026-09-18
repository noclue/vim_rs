use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The *DistributedVirtualPortgroup* managed object
/// defines how hosts and virtual machines connect to a network.
/// 
/// A distributed virtual portgroup specifies *DistributedVirtualPort*
/// configuration options for the ports on a *DistributedVirtualSwitch*.
/// A portgroup also represents a *Network* entity in the datacenter.
/// - To configure host access by portgroup, set the portgroup in the host virtual NIC specification
///   (*HostVirtualNicSpec*.*HostVirtualNicSpec.portgroup*).
/// - To configure virtual machine access by portgroup, set the portgroup
///   in the virtual Ethernet card backing
///   (*VirtualEthernetCard*.*VirtualDevice.backing*.*VirtualEthernetCardDistributedVirtualPortBackingInfo.port*.*DistributedVirtualSwitchPortConnection.portgroupKey*).
///   
/// When you use a portgroup for network access, the Server will create a port according
/// to *DistributedVirtualPortgroup.config*.*DVPortgroupConfigInfo.type*.
#[derive(Clone)]
pub struct DistributedVirtualPortgroup {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl DistributedVirtualPortgroup {
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
    /// ***Required privileges:*** DVPortgroup.Delete
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
        let bytes = self.client.invoke("", "DistributedVirtualPortgroup", &self.mo_id, "Destroy_Task", None).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Deprecated as of VI API 2.5 do not use this method. This method throws
    /// *ResourceInUse*. Networks are automatically
    /// removed when no longer in use, so this method is unnecessary.
    /// 
    /// Removes a network.
    /// 
    /// A network can be removed only if it is not used by any host or
    /// virtual machine.
    /// 
    /// ***Required privileges:*** Network.Delete
    ///
    /// ## Errors:
    ///
    /// ***ResourceInUse***: if one or more hosts or virtual machines are configured
    /// to use the network.
    pub async fn destroy_network(&self) -> Result<()> {
        self.client.invoke_void("", "DistributedVirtualPortgroup", &self.mo_id, "DestroyNetwork", None).await
    }
    /// Reconfigures one or more distributed virtual portgroups.
    /// 
    /// You can use this method to set portgroup properties or
    /// to reset the portgroup to a previous state.
    /// 
    /// **Reconfiguring a Standard Distributed Virtual Portgroup**
    /// 
    /// To reconfigure a *DistributedVirtualPortgroup*,
    /// use a *DVPortgroupConfigSpec*
    /// to set the portgroup properties.
    /// 
    /// **Reconfiguring a Portgroup Associated With a VMware Distributed Virtual Switch**
    /// 
    /// If you use a *VmwareDistributedVirtualSwitch*,
    /// you can perform the following portgroup reconfiguration:
    /// - Use a *DVPortgroupConfigSpec*
    ///   to set the portgroup properties.
    /// - Use the *DVPortgroupConfigSpec*
    ///   returned by *DistributedVirtualPortgroup.DVPortgroupRollback_Task*
    ///   to reset the portgroup to a previous state.
    ///   
    /// The following privileges are required to reconfigure a portgroup.
    /// - DVPortgroup.PolicyOp if you are changing the policy of the portgroup.
    /// - DVPortgroup.ScopeOp if you are changing the scope of the portgroup.
    /// - DVPortgroup.Modify for anything else.
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// Configuration data for the portgroup.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***DvsFault***: if spec is not valid.
    /// 
    /// ***NotSupported***: if the spec includes settings for any VDS feature
    /// that is not supported on this switch.
    /// 
    /// ***DvsNotAuthorized***: if login-session's extension key does not match
    /// the switch's configured
    /// *DVSConfigInfo.extensionKey*.
    pub async fn reconfigure_dv_portgroup_task(&self, spec: &crate::types::structs::DvPortgroupConfigSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ReconfigureDvPortgroupRequestType {spec, };
        let bytes = self.client.invoke("", "DistributedVirtualPortgroup", &self.mo_id, "ReconfigureDVPortgroup_Task", Some(&input)).await?;
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
        self.client.invoke_void("", "DistributedVirtualPortgroup", &self.mo_id, "Reload", None).await
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
    /// ***Required privileges:*** DVPortgroup.Modify
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
        let bytes = self.client.invoke("", "DistributedVirtualPortgroup", &self.mo_id, "Rename_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// This method determines if the portgroup configuration has changed.
    /// 
    /// If it has changed, the method returns a
    /// *DVPortgroupConfigSpec*.
    /// Use the *DistributedVirtualPortgroup.ReconfigureDVPortgroup_Task* method
    /// to apply the rollback configuration to the portgroup. You can use the
    /// rollback method only on a portgroup that is associated with a
    /// *VmwareDistributedVirtualSwitch*.
    /// - If you specify the <code>entityBackup</code> parameter, the returned
    ///   configuration specification represents the exported portgroup configuration.
    ///   If the <code>entityBackup</code> matches the current portgroup
    ///   configuration, the method does not return a configuration specification.
    /// - If <code>entityBackup</code> is not specified, the returned configuration
    ///   specification represents a previous state of the portgroup, if available.
    ///   When you use a VMware distributed virtual switch, each time you reconfigure
    ///   the switch, the Server saves the switch configuration before applying the updates.
    ///   If the vCenter Server is restarted, the saved configuration is not preserved
    ///   and the method does not return a configuration specification.
    ///   
    /// To use the rollback method, you must have the DVPortgroup.Read privilege.
    ///
    /// ## Parameters:
    ///
    /// ### entity_backup
    /// The backup of Distributed Virtual PortGroup entity.
    ///
    /// ## Returns:
    ///
    /// Returns a *Task* object with which to monitor the operation.
    /// If the distributed virtual portgroup configuration has changed, the
    /// *Task*.*Task.info*.*TaskInfo.result*
    /// property contains the *DVPortgroupConfigSpec* object.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***RollbackFailure***: if there is no configuration specified in entityBackup and
    /// the previous configuration does not exist either
    /// 
    /// ***DvsFault***: if operation fails.
    pub async fn dv_portgroup_rollback_task(&self, entity_backup: Option<&crate::types::structs::EntityBackupConfig>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = DvPortgroupRollbackRequestType {entity_backup, };
        let bytes = self.client.invoke("", "DistributedVirtualPortgroup", &self.mo_id, "DVPortgroupRollback_Task", Some(&input)).await?;
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
        self.client.invoke_void("", "DistributedVirtualPortgroup", &self.mo_id, "setCustomValue", Some(&input)).await
    }
    /// Whether alarm actions are enabled for this entity.
    /// 
    /// True if enabled; false otherwise.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn alarm_actions_enabled(&self) -> Result<Option<bool>> {
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "alarmActionsEnabled").await?;
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
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "availableField").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Configuration of the portgroup.
    pub async fn config(&self) -> Result<crate::types::structs::DvPortgroupConfigInfo> {
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "config").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property config was empty".to_string()))?;
        let result: crate::types::structs::DvPortgroupConfigInfo = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// Current configuration issues that have been detected for this entity.
    /// 
    /// Typically,
    /// these issues have already been logged as events. The entity stores these
    /// events as long as they are still current. The
    /// *configStatus* property provides an overall status
    /// based on these events.
    pub async fn config_issue(&self) -> Result<Option<Vec<crate::types::structs::Event>>> {
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "configIssue").await?;
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
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "configStatus").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property configStatus was empty".to_string()))?;
        let result: crate::types::enums::ManagedEntityStatusEnum = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// Custom field values.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn custom_value(&self) -> Result<Option<Vec<Box<dyn crate::types::traits::CustomFieldValueTrait>>>> {
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "customValue").await?;
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
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "declaredAlarmState").await?;
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
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "disabledMethod").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Access rights the current session has to this entity.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn effective_role(&self) -> Result<Option<Vec<i32>>> {
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "effectiveRole").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Hosts attached to this network.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *HostSystem*.
    pub async fn host(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "host").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Generated UUID of the portgroup.
    pub async fn key(&self) -> Result<String> {
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "key").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property key was empty".to_string()))?;
        let result: String = crate::core::client::extract_property(pv)?;
        Ok(result)
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
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "name").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property name was empty".to_string()))?;
        let result: String = crate::core::client::extract_property(pv)?;
        Ok(result)
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
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "overallStatus").await?;
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
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "parent").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// List of the permissions explicitly defined for this entity.
    pub async fn permission(&self) -> Result<Option<Vec<crate::types::structs::Permission>>> {
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "permission").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Port keys for the set of ports in the portgroup.
    pub async fn port_keys(&self) -> Result<Option<Vec<String>>> {
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "portKeys").await?;
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
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "recentTask").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Properties of a network.
    pub async fn summary(&self) -> Result<Box<dyn crate::types::traits::NetworkSummaryTrait>> {
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "summary").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property summary was empty".to_string()))?;
        let result: Box<dyn crate::types::traits::NetworkSummaryTrait> = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// The set of tags associated with this managed entity.
    /// 
    /// Experimental. Subject to change.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn tag(&self) -> Result<Option<Vec<crate::types::structs::Tag>>> {
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "tag").await?;
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
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "triggeredAlarmState").await?;
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
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "value").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
    /// Virtual machines using this network.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *VirtualMachine*.
    pub async fn vm(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let pv_opt = self.client.fetch_property_raw("", "DistributedVirtualPortgroup", &self.mo_id, "vm").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
}
struct ReconfigureDvPortgroupRequestType<'a> {
    spec: &'a crate::types::structs::DvPortgroupConfigSpec,
}

impl<'a> miniserde::Serialize for ReconfigureDvPortgroupRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReconfigureDvPortgroupRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReconfigureDvPortgroupRequestTypeSer<'b, 'a> {
    data: &'b ReconfigureDvPortgroupRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ReconfigureDvPortgroupRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReconfigureDVPortgroupRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
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
struct DvPortgroupRollbackRequestType<'a> {
    entity_backup: Option<&'a crate::types::structs::EntityBackupConfig>,
}

impl<'a> miniserde::Serialize for DvPortgroupRollbackRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DvPortgroupRollbackRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DvPortgroupRollbackRequestTypeSer<'b, 'a> {
    data: &'b DvPortgroupRollbackRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DvPortgroupRollbackRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DVPortgroupRollbackRequestType")),
                1 => {
                    let Some(ref val) = self.data.entity_backup else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entityBackup"), val as &dyn miniserde::Serialize));
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
