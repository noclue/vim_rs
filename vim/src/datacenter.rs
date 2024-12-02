use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::AlarmState;
use crate::types::CustomFieldDef;
use crate::types::CustomFieldValueTrait;
use crate::types::DatacenterBasicConnectInfo;
use crate::types::DatacenterConfigInfo;
use crate::types::DatacenterConfigSpec;
use crate::types::EventTrait;
use crate::types::HostConnectInfo;
use crate::types::HostConnectSpec;
use crate::types::ManagedEntityStatusEnum;
use crate::types::ManagedObjectReference;
use crate::types::OptionValueTrait;
use crate::types::Permission;
use crate::types::Tag;
use crate::types::VirtualMachineConfigOptionDescriptor;
/// The *Datacenter* managed object provides the interface to the common container
/// object for hosts, virtual machines, networks, and datastores.
/// 
/// These entities
/// must be under a distinct datacenter in the inventory, and datacenters may not
/// be nested under other datacenters.
/// 
/// Every *Datacenter* has the following set of dedicated folders. These folders are empty
/// until you create entities for the Datacenter.
/// - A folder for *VirtualMachine*, template, and
///   *VirtualApp* objects.
/// - A folder for a *ComputeResource* hierarchy.
/// - A folder for *Network*, *DistributedVirtualSwitch*,
///   and *DistributedVirtualPortgroup* objects.
/// - A folder for *Datastore* objects.
///   
/// For a visual representation of the organization of objects in a vCenter
/// hierarchy, see the description of the *ServiceInstance* object.
pub struct Datacenter {
    client: Arc<VimClient>,
    mo_id: String,
}
impl Datacenter {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// This interface returns a list of information about the specified hosts
    /// without adding them to the vCenter inventory.
    /// 
    /// It does so by calling
    /// *Datacenter.QueryConnectionInfoViaSpec*.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### host_specs
    /// Information about the set of hosts to query.
    pub async fn batch_query_connect_info(&self, host_specs: Option<&[HostConnectSpec]>) -> Result<Option<Vec<DatacenterBasicConnectInfo>>> {
        let input = BatchQueryConnectInfoRequestType {host_specs, };
        let path = format!("/Datacenter/{moId}/BatchQueryConnectInfo", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
    /// ***Required privileges:*** Datacenter.Delete
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
        let path = format!("/Datacenter/{moId}/Destroy_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Powers on multiple virtual machines in a data center.
    /// 
    /// If the virtual
    /// machines are suspended, this method resumes execution from the suspend
    /// point.
    /// The virtual machines can belong to different clusters in the data center.
    /// 
    /// If any virtual machine in the list is manually managed by DRS, or DRS
    /// has to migrate any manually managed virtual machine or power on any manually
    /// managed host in order to power on these virtual machines, a DRS recommendation
    /// will be generated, and the users need to manually apply the recommendation
    /// for actually powering on these virtual machines.
    /// Otherwise, all the virtual machine will be automatically powered on. The
    /// virtual machines on stand alone hosts or DRS disabled will be powered-on
    /// on the current host. The DRS automatically managed virtual machines will
    /// be powered-on on the recommended hosts.
    /// 
    /// When powering on a virtual machine in a cluster, the system
    /// might do an implicit relocation of the virtual machine to
    /// another host.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// The virtual machines to power on.
    /// 
    /// ***Required privileges:*** VirtualMachine.Interact.PowerOn
    /// 
    /// Refers instances of *VirtualMachine*.
    ///
    /// ### option
    /// An array of *OptionValue* options
    /// for this power-on session. The names and values of the
    /// options are defined in
    /// *ClusterPowerOnVmOption_enum*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation, and also a *ClusterPowerOnVmResult* object.
    /// 
    /// Refers instance of *Task*.
    pub async fn power_on_multi_vm_task(&self, vm: &[ManagedObjectReference], option: Option<&[Box<dyn OptionValueTrait>]>) -> Result<ManagedObjectReference> {
        let input = PowerOnMultiVmRequestType {vm, option, };
        let path = format!("/Datacenter/{moId}/PowerOnMultiVM_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// The list of possible choices for
    /// *DatacenterConfigSpec.defaultHardwareVersionKey*.
    /// 
    /// Descriptors returned by the vCenter implementation do not have
    /// *VirtualMachineConfigOptionDescriptor.host* field populated.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn query_datacenter_config_option_descriptor(&self) -> Result<Option<Vec<VirtualMachineConfigOptionDescriptor>>> {
        let path = format!("/Datacenter/{moId}/queryDatacenterConfigOptionDescriptor", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// This method provides a way of getting basic information about a host without
    /// adding it to a datacenter.
    /// 
    /// Connection wizards typically use this method to show
    /// information about a host so a user can confirm a set of changes before applying
    /// them.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### hostname
    /// The target of the query.
    ///
    /// ### port
    /// The port number of the target host. For ESX 2.x this is the authd port
    /// (902 by default). For ESX 3.x and above and for VMware Server hosts
    /// this is the https port (443 by default). You can specify -1 to have the
    /// vCenter Server try the default ports.
    ///
    /// ### username
    /// The name of the user.
    ///
    /// ### password
    /// The password of the user.
    ///
    /// ### ssl_thumbprint
    /// The expected SSL thumbprint of the host's certificate.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if called directly on a host.
    /// 
    /// ***NoHost***: if unable to contact the host.
    /// 
    /// ***InvalidLogin***: if unable to authenticate with the host.
    /// 
    /// ***NotSupportedHost***: if the software version on the host is not supported.
    /// 
    /// ***AlreadyConnected***: if the host is already being managed by this server.
    /// 
    /// ***HostConnectFault***: if an error occurred when querying about a host.
    /// Typically, a more specific subclass, such as AlreadyBeingManaged,
    /// is thrown.
    /// 
    /// ***SSLDisabledFault***: if the host has SSL access disabled
    /// 
    /// ***SSLVerifyFault***: if the host certificate could not be authenticated
    /// 
    /// ***InvalidArgument***: if both arguments `sslThumbprint`
    /// and `sslCertificate` are set, or if only the `sslThumbprint`
    /// argument is set, but the SHA-1 hashing algorithm is currently disabled
    /// for computing certificate thumbprints.
    pub async fn query_connection_info(&self, hostname: &str, port: i32, username: &str, password: &str, ssl_thumbprint: Option<&str>) -> Result<HostConnectInfo> {
        let input = QueryConnectionInfoRequestType {hostname, port, username, password, ssl_thumbprint, };
        let path = format!("/Datacenter/{moId}/QueryConnectionInfo", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// This method provides a way of getting basic information about a host
    /// without adding it to a datacenter.
    /// 
    /// This method is similar to
    /// *Datacenter.QueryConnectionInfo*, but it takes a *HostConnectSpec*
    /// as argument, instead of list of parameters.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// The connection spec for the host to be queried. It must contain
    /// values for all parameters required by *Datacenter.QueryConnectionInfo*
    /// See *Datacenter.QueryConnectionInfo* or a list of thrown expections.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the spec argument does not provide values for
    /// all needed connection parameters.
    pub async fn query_connection_info_via_spec(&self, spec: &HostConnectSpec) -> Result<HostConnectInfo> {
        let input = QueryConnectionInfoViaSpecRequestType {spec, };
        let path = format!("/Datacenter/{moId}/QueryConnectionInfoViaSpec", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Change the datacenter configuration.
    /// 
    /// ***Required privileges:*** Datacenter.Reconfigure
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// A set of configuration changes to apply to the datacenter.
    /// The specification can be a complete set of changes or a partial
    /// set of changes, applied incrementally.
    ///
    /// ### modify
    /// Flag to specify whether the specification ("spec") should
    /// be applied incrementally. If "modify" is false and the
    /// operation succeeds, then the configuration of the datacenter
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
    pub async fn reconfigure_datacenter_task(&self, spec: &DatacenterConfigSpec, modify: bool) -> Result<ManagedObjectReference> {
        let input = ReconfigureDatacenterRequestType {spec, modify, };
        let path = format!("/Datacenter/{moId}/ReconfigureDatacenter_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
        let path = format!("/Datacenter/{moId}/Reload", moId = &self.mo_id);
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
    /// ***Required privileges:*** Datacenter.Rename
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
        let path = format!("/Datacenter/{moId}/Rename_Task", moId = &self.mo_id);
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
        let path = format!("/Datacenter/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Whether alarm actions are enabled for this entity.
    /// 
    /// True if enabled; false otherwise.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn alarm_actions_enabled(&self) -> Result<Option<bool>> {
        let path = format!("/Datacenter/{moId}/alarmActionsEnabled", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/Datacenter/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Current configuration issues that have been detected for this entity.
    /// 
    /// Typically,
    /// these issues have already been logged as events. The entity stores these
    /// events as long as they are still current. The
    /// *configStatus* property provides an overall status
    /// based on these events.
    pub async fn config_issue(&self) -> Result<Option<Vec<Box<dyn EventTrait>>>> {
        let path = format!("/Datacenter/{moId}/configIssue", moId = &self.mo_id);
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
        let path = format!("/Datacenter/{moId}/configStatus", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Configuration of the datacenter.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn configuration(&self) -> Result<DatacenterConfigInfo> {
        let path = format!("/Datacenter/{moId}/configuration", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Custom field values.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn custom_value(&self) -> Result<Option<Vec<Box<dyn CustomFieldValueTrait>>>> {
        let path = format!("/Datacenter/{moId}/customValue", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// A collection of references to the datastore objects
    /// available in this datacenter.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Datastore*.
    pub async fn datastore(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/Datacenter/{moId}/datastore", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// A reference to the folder hierarchy that contains
    /// the datastores for this datacenter.
    /// 
    /// This folder is guaranteed to exist.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Folder*.
    pub async fn datastore_folder(&self) -> Result<ManagedObjectReference> {
        let path = format!("/Datacenter/{moId}/datastoreFolder", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
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
        let path = format!("/Datacenter/{moId}/declaredAlarmState", moId = &self.mo_id);
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
        let path = format!("/Datacenter/{moId}/disabledMethod", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Access rights the current session has to this entity.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn effective_role(&self) -> Result<Option<Vec<i32>>> {
        let path = format!("/Datacenter/{moId}/effectiveRole", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// A reference to the folder hierarchy that contains
    /// the compute resources, including hosts and clusters, for this datacenter.
    /// 
    /// This folder is guaranteed to exist.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Folder*.
    pub async fn host_folder(&self) -> Result<ManagedObjectReference> {
        let path = format!("/Datacenter/{moId}/hostFolder", moId = &self.mo_id);
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
        let path = format!("/Datacenter/{moId}/name", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// A collection of references to the network objects
    /// available in this datacenter.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Network*.
    pub async fn network(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/Datacenter/{moId}/network", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// A reference to the folder hierarchy that contains the network entities
    /// for this datacenter.
    /// 
    /// The folder can include *Network*,
    /// *DistributedVirtualSwitch*, and
    /// *DistributedVirtualPortgroup* objects.
    /// 
    /// This folder is guaranteed to exist.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Folder*.
    pub async fn network_folder(&self) -> Result<ManagedObjectReference> {
        let path = format!("/Datacenter/{moId}/networkFolder", moId = &self.mo_id);
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
        let path = format!("/Datacenter/{moId}/overallStatus", moId = &self.mo_id);
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
        let path = format!("/Datacenter/{moId}/parent", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// List of permissions defined for this entity.
    pub async fn permission(&self) -> Result<Option<Vec<Permission>>> {
        let path = format!("/Datacenter/{moId}/permission", moId = &self.mo_id);
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
        let path = format!("/Datacenter/{moId}/recentTask", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// The set of tags associated with this managed entity.
    /// 
    /// Experimental. Subject to change.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn tag(&self) -> Result<Option<Vec<Tag>>> {
        let path = format!("/Datacenter/{moId}/tag", moId = &self.mo_id);
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
        let path = format!("/Datacenter/{moId}/triggeredAlarmState", moId = &self.mo_id);
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
        let path = format!("/Datacenter/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// A reference to the folder hierarchy that contains *VirtualMachine*
    /// virtual machine templates (identified by the *VirtualMachineConfigInfo.template*
    /// property, and *VirtualApp* objects for this datacenter.
    /// 
    /// Note that a VirtualApp that is a child of a *ResourcePool*
    /// may also be visible in this folder. VirtualApp objects can be nested,
    /// but only the parent VirtualApp can be visible in the folder.
    /// 
    /// This folder is guaranteed to exist.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Folder*.
    pub async fn vm_folder(&self) -> Result<ManagedObjectReference> {
        let path = format!("/Datacenter/{moId}/vmFolder", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct BatchQueryConnectInfoRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hostSpecs")]
    host_specs: Option<&'a [HostConnectSpec]>,
}
#[derive(serde::Serialize)]
#[serde(rename = "PowerOnMultiVMRequestType", tag = "_typeName")]
struct PowerOnMultiVmRequestType<'a> {
    vm: &'a [ManagedObjectReference],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    option: Option<&'a [Box<dyn OptionValueTrait>]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryConnectionInfoRequestType<'a> {
    hostname: &'a str,
    port: i32,
    username: &'a str,
    password: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sslThumbprint")]
    ssl_thumbprint: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryConnectionInfoViaSpecRequestType<'a> {
    spec: &'a HostConnectSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReconfigureDatacenterRequestType<'a> {
    spec: &'a DatacenterConfigSpec,
    modify: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RenameRequestType<'a> {
    #[serde(rename = "newName")]
    new_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "setCustomValueRequestType", tag = "_typeName")]
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}
