use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::AlarmState;
use crate::types::structs::AnswerFileStatusResult;
use crate::types::structs::ApplyHostProfileConfigurationResult;
use crate::types::structs::ApplyHostProfileConfigurationSpec;
use crate::types::structs::ComplianceResult;
use crate::types::structs::CryptoKeyId;
use crate::types::structs::CryptoKeyPlain;
use crate::types::structs::CustomFieldDef;
use crate::types::structs::HostCapability;
use crate::types::structs::HostConfigInfo;
use crate::types::structs::HostConfigManager;
use crate::types::structs::HostConnectInfo;
use crate::types::structs::HostConnectSpec;
use crate::types::structs::HostFlagInfo;
use crate::types::structs::HostHardwareInfo;
use crate::types::structs::HostIpmiInfo;
use crate::types::structs::HostLicensableResourceInfo;
use crate::types::structs::HostListSummary;
use crate::types::structs::HostMaintenanceSpec;
use crate::types::structs::HostRuntimeInfo;
use crate::types::structs::HostServiceTicket;
use crate::types::structs::HostSystemComplianceCheckState;
use crate::types::structs::HostSystemReconnectSpec;
use crate::types::structs::HostSystemRemediationState;
use crate::types::structs::HostSystemResourceInfo;
use crate::types::structs::HostSystemSwapConfiguration;
use crate::types::structs::HostTpmAttestationReport;
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::Permission;
use crate::types::structs::Tag;
use crate::types::structs::VirtualMachineConfigInfo;
/// The HostSystem managed object type provides access to a virtualization
/// host platform.
/// 
/// Invoking destroy on a HostSystem of standalone type throws a NotSupported fault.
/// A standalone HostSystem can be destroyed only by invoking destroy on its parent
/// ComputeResource.
/// Invoking destroy on a failover host throws a
/// *DisallowedOperationOnFailoverHost* fault. See
/// *ClusterFailoverHostAdmissionControlPolicy*.
pub struct HostSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl HostSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Creates and returns a credential used to establish a remote
    /// connection to a Web Based Management (CIM) interface.
    /// 
    /// Valid only
    /// when ESXi wbem authentication mode is set to password.
    /// The ticket provides the port for the service and sslThumbprint should
    /// be used by client to validate ssl connection. This ticket is valid for 2
    /// minutes then will expire and is non-renewable.
    /// 
    /// ***Required privileges:*** Host.Cim.CimInteraction
    pub async fn acquire_cim_services_ticket(&self) -> Result<HostServiceTicket> {
        let path = format!("/HostSystem/{moId}/AcquireCimServicesTicket", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Sets/changes the key to be used for coredump encryption
    /// and puts the host in *safe* state.
    /// 
    /// This function will make the host crypto safe and unlock all encrypted
    /// VMs on the host. When the encryption on the host is enabled for the
    /// first time after adding it to vCenter Server, this method will start
    /// sending asynchronously all the encryption keys for VMs on the host and
    /// cluster to unlock encrypted VMs.
    /// This API behaves differently on the ESXi host vs. the vCenter server.
    /// Before vSphere 7.0, it is not supported on host, and invoking directly
    /// on a host will throw NotSupported fault. Since vSphere 7.0, calling the
    /// API on host will make the host crypto safe, but the parameter should not
    /// be blank and should only be a key id from a trusted key provider.
    /// 
    /// ***Required privileges:*** Cryptographer.RegisterHost
    ///
    /// ## Parameters:
    ///
    /// ### key_id
    /// The key to be used for coredump encryption. If unset, uses
    /// existing host or cluster key or new key is generated from
    /// the default KMIP server.
    pub async fn configure_crypto_key(&self, key_id: Option<&CryptoKeyId>) -> Result<()> {
        let input = ConfigureCryptoKeyRequestType {key_id, };
        let path = format!("/HostSystem/{moId}/ConfigureCryptoKey", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn destroy_task(&self) -> Result<ManagedObjectReference> {
        let path = format!("/HostSystem/{moId}/Destroy_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Disconnects from a host and instructs the server to stop sending heartbeats.
    /// 
    /// ***Required privileges:*** Host.Config.Connection
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation.
    /// 
    /// Refers instance of *Task*.
    pub async fn disconnect_host_task(&self) -> Result<ManagedObjectReference> {
        let path = format!("/HostSystem/{moId}/DisconnectHost_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Sets/changes the key to be used for coredump encryption
    /// and puts the host in *safe* state
    /// Note: *HostSystem.PrepareCrypto* must be called first
    /// 
    /// ***Required privileges:*** Cryptographer.RegisterHost
    ///
    /// ## Parameters:
    ///
    /// ### key_plain
    /// The key to be used for coredump encryption
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the host is in
    /// *incapable* state
    pub async fn enable_crypto(&self, key_plain: &CryptoKeyPlain) -> Result<()> {
        let input = EnableCryptoRequestType {key_plain, };
        let path = format!("/HostSystem/{moId}/EnableCrypto", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Deprecated as of vSphere API 6.0, use
    /// *HostAccessManager.ChangeLockdownMode*.
    /// 
    /// Modifies the permissions on the host, so that it will only be accessible
    /// through local console or an authorized centralized management application.
    /// 
    /// Any user defined permissions found on the host are lost.
    /// 
    /// Access via a VI client connected to the host is blocked.
    /// Access though other services running on the host is also blocked.
    /// 
    /// If the operation is successful, *HostConfigInfo.adminDisabled*
    /// will be set to true. This API is not supported on the host, If invoked
    /// directly on a host, a NotSupported fault will be thrown.
    /// 
    /// See also *AuthorizationManager*for more information on permissions..
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    ///
    /// ## Errors:
    ///
    /// ***AdminDisabled***: If the host's Administrator permission has been
    /// disabled.
    /// 
    /// ***DisableAdminNotSupported***: If invoked directly on the host or the
    /// host doesn't support this operation.
    pub async fn enter_lockdown_mode(&self) -> Result<()> {
        let path = format!("/HostSystem/{moId}/EnterLockdownMode", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Puts the host in maintenance mode.
    /// 
    /// While this task is running and when the host is
    /// in maintenance mode, no virtual machines can be powered on and no provisioning
    /// operations can be performed on the host. Once the call completes, it is safe to
    /// turn off a host without disrupting any virtual machines.
    /// 
    /// The task completes once there are no powered-on virtual machines on the host and
    /// no provisioning operations in progress on the host. The operation does not
    /// directly initiate any operations to evacuate or power-down powered-on virtual machines.
    /// However, if the host is part of a cluster with VMware DRS enabled, DRS provides
    /// migration recommendations to evacuate the powered-on virtual machines. If DRS is in
    /// fully-automatic mode, these are automatically scheduled.
    /// 
    /// If the host is part of a cluster and the task is issued through VirtualCenter with
    /// evacuatePoweredOffVms set to true, the task will not succeed unless all the
    /// powered-off virtual machines are reregistered to other hosts. If VMware DRS is
    /// enabled, vCenter Server will automatically evacuate powered-off virtual machines.
    /// 
    /// If this API is called directly on the ESXi host, then the user is responsible
    /// for powering off, suspending or evacuating all powered-on virtual machines.
    /// The task is cancellable.
    /// 
    /// ***Required privileges:*** Host.Config.Maintenance
    ///
    /// ## Parameters:
    ///
    /// ### timeout
    /// The task completes when the host successfully enters maintenance
    /// mode or the timeout expires, and in the latter case the task
    /// contains a Timeout fault. If the timeout is less than or equal to
    /// zero, there is no timeout. The timeout is specified in seconds.
    ///
    /// ### evacuate_powered_off_vms
    /// This is a parameter only supported by VirtualCenter.
    /// If set to true, for a DRS disabled cluster, the task will not
    /// succeed unless all powered-off virtual machines have been manually
    /// reregistered; for a DRS enabled cluster, VirtualCenter will
    /// automatically reregister powered-off virtual machines and a
    /// powered-off virtual machine may remain at the host only for two
    /// reasons: (a) no compatible host found for reregistration, (b) DRS
    /// is disabled for the virtual machine. If set to false, powered-off
    /// virtual machines do not need to be moved.
    ///
    /// ### maintenance_spec
    /// Any additional actions to be taken by the host upon
    /// entering maintenance mode. If omitted, default actions will
    /// be taken as documented in the *HostMaintenanceSpec*.
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
    /// ***InvalidState***: if the host is already in maintenance mode.
    /// 
    /// ***Timedout***: if the operation timed out.
    /// 
    /// ***RequestCanceled***: if the operation is canceled.
    pub async fn enter_maintenance_mode_task(&self, timeout: i32, evacuate_powered_off_vms: Option<bool>, maintenance_spec: Option<&HostMaintenanceSpec>) -> Result<ManagedObjectReference> {
        let input = EnterMaintenanceModeRequestType {timeout, evacuate_powered_off_vms, maintenance_spec, };
        let path = format!("/HostSystem/{moId}/EnterMaintenanceMode_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Puts the host in standby mode, a mode in which the host is in a
    /// standby state from which it can be powered up remotely.
    /// 
    /// While
    /// this task is running, no virtual machines can be powered on and
    /// no provisioning operations can be performed on the host.
    /// 
    /// The task completes only if there are no powered-on virtual
    /// machines on the host, no provisioning operations in progress on
    /// the host, and the host stopped responding. The operation does
    /// not directly initiate any operations to evacuate or power-down
    /// powered-on virtual machines. However, if a dynamic recommendation
    /// generation module is running, if possible, it will provide, and
    /// depending on the automation level, it will execute migrations
    /// of powered-on virtual machine. Furthermore, VMware power
    /// management module may evacute and put a host in standby mode to
    /// save power.
    /// If the host is part of a cluster and the task is issued through VirtualCenter with
    /// evacuatePoweredOffVms set to true, the task will not succeed unless all the
    /// powered-off virtual machines are reregistered to other hosts. If VMware DRS is
    /// enabled, vCenter Server will automatically evacuate powered-off virtual machines.
    /// 
    /// The task is cancellable.
    /// 
    /// This command is not supported on all hosts. Check the host capability
    /// *HostCapability.standbySupported*.
    /// 
    /// ***Required privileges:*** Host.Config.Maintenance
    ///
    /// ## Parameters:
    ///
    /// ### timeout_sec
    /// The task completes when the host successfully
    /// enters standby mode and stops sending heartbeat signals.
    /// If heartbeats are still coming after timeoutSecs seconds,
    /// the host is declared timedout, and the task is assumed
    /// failed.
    ///
    /// ### evacuate_powered_off_vms
    /// This is a parameter used only by VirtualCenter. If
    /// set to true, for a DRS disabled cluster, the task will not
    /// succeed unless all powered-off virtual machines have been manually
    /// reregistered; for a DRS enabled cluster, VirtualCenter will
    /// automatically reregister powered-off virtual machines and a
    /// powered-off virtual machine may remain at the host only for two
    /// reasons: (a) no compatible host found for reregistration, (b) DRS
    /// is disabled for the virtual machine.
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
    /// ***HostPowerOpFailed***: if the standby operation fails.
    /// 
    /// ***InvalidState***: if the host is already in standby mode, or disconnected.
    /// 
    /// ***NotSupported***: if the host does not support standby mode.
    /// 
    /// ***Timedout***: if the host did not enter standby mode in the given time
    /// 
    /// ***RequestCanceled***: if the operation is canceled.
    pub async fn power_down_host_to_stand_by_task(&self, timeout_sec: i32, evacuate_powered_off_vms: Option<bool>) -> Result<ManagedObjectReference> {
        let input = PowerDownHostToStandByRequestType {timeout_sec, evacuate_powered_off_vms, };
        let path = format!("/HostSystem/{moId}/PowerDownHostToStandBy_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of vSphere API 6.0, use
    /// *HostAccessManager.ChangeLockdownMode*.
    /// 
    /// Restores Administrator permission for the local administrative account
    /// for the host that was removed by prior call to *HostSystem.EnterLockdownMode*.
    /// 
    /// If the operation is successful,
    /// *HostConfigInfo.adminDisabled* will be set to false. This API
    /// is not supported on the host. If invoked directly on a host, a
    /// NotSupported fault will be thrown.
    /// 
    /// See also *AuthorizationManager*for more information on permissions..
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    ///
    /// ## Errors:
    ///
    /// ***DisableAdminNotSupported***: If invoked directly on the host or the
    /// host doesn't support this operation.
    /// 
    /// ***AdminNotDisabled***: If the host's Administrator permission
    /// is not disabled.
    pub async fn exit_lockdown_mode(&self) -> Result<()> {
        let path = format!("/HostSystem/{moId}/ExitLockdownMode", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Takes the host out of maintenance mode.
    /// 
    /// This blocks if any concurrent
    /// running maintenance-only host configurations operations are being performed.
    /// For example, if VMFS volumes are being upgraded.
    /// 
    /// The task is cancellable.
    /// 
    /// ***Required privileges:*** Host.Config.Maintenance
    ///
    /// ## Parameters:
    ///
    /// ### timeout
    /// Number of seconds to wait for the exit maintenance mode to
    /// succeed. If the timeout is less than or equal to zero, there
    /// is no timeout.
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
    /// ***InvalidState***: if the host is not in maintenance mode.
    pub async fn exit_maintenance_mode_task(&self, timeout: i32) -> Result<ManagedObjectReference> {
        let input = ExitMaintenanceModeRequestType {timeout, };
        let path = format!("/HostSystem/{moId}/ExitMaintenanceMode_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Takes the host out of standby mode.
    /// 
    /// If the command is
    /// successful, the host wakes up and starts sending
    /// heartbeats. This method may be called automatically by a
    /// dynamic recommendation generation module to add capacity to a
    /// cluster, if the host is not in maintenance mode.
    /// 
    /// Note that, depending on the implementation of the wakeup
    /// method, the client may never receive an indicator of success in
    /// the returned task. In some cases, it is not even possible to
    /// ensure that the wakeup request has made it to the host.
    /// 
    /// The task is cancellable.
    /// 
    /// ***Required privileges:*** Host.Config.Maintenance
    ///
    /// ## Parameters:
    ///
    /// ### timeout_sec
    /// The task completes when the host successfully
    /// exits standby state and sends a heartbeat signal. If nothing is
    /// received from the host for timeoutSec seconds, the host is
    /// declared timedout, and the task is assumed failed.
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
    /// ***HostPowerOpFailed***: if the standby operation fails.
    /// 
    /// ***InvalidState***: if the host is in a state from which it
    /// cannot be woken up (e.g., disconnected, poweredOff)
    /// 
    /// ***NotSupported***: if the host does not support standby mode.
    /// 
    /// ***Timedout***: if the host did not exit standby mode in the given time
    /// 
    /// ***RequestCanceled***: if the operation is canceled.
    pub async fn power_up_host_from_stand_by_task(&self, timeout_sec: i32) -> Result<ManagedObjectReference> {
        let input = PowerUpHostFromStandByRequestType {timeout_sec, };
        let path = format!("/HostSystem/{moId}/PowerUpHostFromStandBy_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Prepare the host for receiving sensitive information
    /// and puts the host in *prepared* mode
    /// Note: Must be invoked before *HostSystem.EnableCrypto*
    /// 
    /// ***Required privileges:*** Cryptographer.RegisterHost
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the host is not in
    /// *incapable* state
    pub async fn prepare_crypto(&self) -> Result<()> {
        let path = format!("/HostSystem/{moId}/PrepareCrypto", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Connection-oriented information about a host.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn query_host_connection_info(&self) -> Result<HostConnectInfo> {
        let path = format!("/HostSystem/{moId}/QueryHostConnectionInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of VI API 2.5, use *HostSystem.QueryMemoryOverheadEx*.
    /// 
    /// Determines the amount of memory overhead necessary to power on a virtual
    /// machine with the specified characteristics.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### memory_size
    /// The amount of virtual system RAM, in bytes. For an existing
    /// virtual machine, this value can be found (in megabytes) as the memoryMB
    /// property of the *VirtualHardware*.
    ///
    /// ### video_ram_size
    /// The amount of virtual video RAM, in bytes. For an existing
    /// virtual machine on a host that supports advertising this property, this
    /// value can be found (in kilobytes) as the videoRamSizeInKB property of the
    /// *VirtualMachineVideoCard*. If this parameter is left unset, the
    /// default video RAM size for virtual machines on this host is assumed.
    ///
    /// ### num_vcpus
    /// The number of virtual CPUs. For an existing virtual machine, this
    /// value can be found as the numCPU property of the
    /// *VirtualHardware*.
    ///
    /// ## Returns:
    ///
    /// The amount of overhead memory required to power on such a virtual machine,
    /// in bytes.
    pub async fn query_memory_overhead(&self, memory_size: i64, video_ram_size: Option<i32>, num_vcpus: i32) -> Result<i64> {
        let input = QueryMemoryOverheadRequestType {memory_size, video_ram_size, num_vcpus, };
        let path = format!("/HostSystem/{moId}/QueryMemoryOverhead", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of VI API 6.0, use
    /// *VirtualMachineConfigInfo.initialOverhead*.
    /// 
    /// Determines the amount of memory overhead necessary to power on a virtual
    /// machine with the specified characteristics.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### vm_config_info
    /// The configuration of the virtual machine.
    ///
    /// ## Returns:
    ///
    /// The amount of overhead memory required to power on such a virtual machine,
    /// in bytes.
    pub async fn query_memory_overhead_ex(&self, vm_config_info: &VirtualMachineConfigInfo) -> Result<i64> {
        let input = QueryMemoryOverheadExRequestType {vm_config_info, };
        let path = format!("/HostSystem/{moId}/QueryMemoryOverheadEx", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Query the path to VMware Tools repository configured on the host.
    /// 
    /// The host should be powered on.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// The absolute path currently set for the VMware Tools
    /// repository on the host.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if the configuration could not be read.
    pub async fn query_product_locker_location(&self) -> Result<String> {
        let path = format!("/HostSystem/{moId}/QueryProductLockerLocation", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Basic information about TPM attestation state of the host.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn query_tpm_attestation_report(&self) -> Result<Option<HostTpmAttestationReport>> {
        let path = format!("/HostSystem/{moId}/QueryTpmAttestationReport", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Reboots a host.
    /// 
    /// If the command is successful, then the host has been rebooted. If
    /// connected directly to the host, the client never receives an indicator of success
    /// in the returned task but simply loses connection to the host, upon success.
    /// 
    /// This command is not supported on all hosts. Check the host capability
    /// *vim.host.Capability.rebootSupported*.
    /// If QuickBoot is enabled on the host, additional setup steps are performed.
    /// 
    /// ***Required privileges:*** Host.Config.Maintenance
    ///
    /// ## Parameters:
    ///
    /// ### force
    /// Flag to specify whether or not the host should be rebooted
    /// regardless of whether it is in maintenance mode. If true, the host
    /// is rebooted, even if there are virtual machines running or other
    /// operations in progress.
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
    /// ***InvalidState***: if "force" is false and the host is not in maintenance mode.
    /// 
    /// ***NotSupported***: if the host does not support the reboot operation.
    pub async fn reboot_host_task(&self, force: bool) -> Result<ManagedObjectReference> {
        let input = RebootHostRequestType {force, };
        let path = format!("/HostSystem/{moId}/RebootHost_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Reconfigures the host for vSphere HA.
    /// 
    /// If the host is part of a HA cluster, this operation reconfigures the host for HA.
    /// For example, this operation may be used if a host is added to a HA enabled cluster
    /// and the automatic HA configuration system task fails. Automatic HA configuration
    /// may fail for a variety of reasons. For example, the host is configured
    /// incorrectly.
    /// 
    /// ***Required privileges:*** Host.Config.Connection
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
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***DasConfigFault***: if there is a problem reconfiguring the host for HA.
    pub async fn reconfigure_host_for_das_task(&self) -> Result<ManagedObjectReference> {
        let path = format!("/HostSystem/{moId}/ReconfigureHostForDAS_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Reconnects to a host.
    /// 
    /// This process reinstalls agents and reconfigures the host, if
    /// it has gotten out of date with VirtualCenter. The reconnection process goes
    /// through many of the same steps as addHost: ensuring the correct set of licenses
    /// for the number of CPUs on the host, ensuring the correct set of agents is
    /// installed, and ensuring that networks and datastores are discovered and registered
    /// with VirtualCenter.
    /// 
    /// The client can change the IP address and port of the host when doing a reconnect
    /// operation. This can be useful if the client wants to preserve existing metadata,
    /// even though the host is changing its IP address. For example, clients could
    /// preserve existing statistics, alarms, and privileges.
    /// 
    /// This method can also be used to change the SSL thumbprint of a connected host
    /// without disconnecting it.
    /// 
    /// Any changes made to the resource hierarchy on the host when the host
    /// was disconnected are overriden by VirtualCenter settings on
    /// reconnect.
    /// 
    /// This method is only supported through VirtualCenter.
    /// 
    /// ***Required privileges:*** Host.Config.Connection
    ///
    /// ## Parameters:
    ///
    /// ### cnx_spec
    /// Includes the parameters to use, including user name and password,
    /// when reconnecting to the host. If this parameter is not specified,
    /// the default connection parameters is used.
    ///
    /// ### reconnect_spec
    /// Includes connection parameters specific to
    /// reconnect. This will mainly be used to indicate how to
    /// handle divergence between the host settings and vCenter Server
    /// settings when the host was disconnected.
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
    /// ***NotSupported***: if no host can be added to this group. This is the case if
    /// the ComputeResource is a standalone type.
    /// 
    /// ***InvalidLogin***: if the method fails to authenticate with the host.
    /// 
    /// ***AlreadyBeingManaged***: if host is already being managed by another
    /// VirtualCenter server
    /// 
    /// ***NotEnoughLicenses***: if there are not enough licenses to add this host.
    /// 
    /// ***NoHost***: if the method is unable to contact the server.
    /// 
    /// ***NotSupportedHost***: if the host is running a software version that is not
    /// supported.
    /// 
    /// ***InvalidState***: if the host is not disconnected.
    /// 
    /// ***InvalidName***: if the host name is invalid.
    /// 
    /// ***HostConnectFault***: if an error occurred when attempting to reconnect
    /// to a host. Typically, a more specific subclass, such as
    /// AlreadyBeingManaged, is thrown.
    /// 
    /// ***SSLVerifyFault***: if the host certificate could not be authenticated.
    pub async fn reconnect_host_task(&self, cnx_spec: Option<&HostConnectSpec>, reconnect_spec: Option<&HostSystemReconnectSpec>) -> Result<ManagedObjectReference> {
        let input = ReconnectHostRequestType {cnx_spec, reconnect_spec, };
        let path = format!("/HostSystem/{moId}/ReconnectHost_Task", moId = &self.mo_id);
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
        let path = format!("/HostSystem/{moId}/Reload", moId = &self.mo_id);
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
        let path = format!("/HostSystem/{moId}/Rename_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Return the amount of free EPC memory on the host in bytes.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn retrieve_free_epc_memory(&self) -> Result<i64> {
        let path = format!("/HostSystem/{moId}/RetrieveFreeEpcMemory", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Return the hardware uptime of the host in seconds.
    /// 
    /// The harware uptime of a host is not affected by NTP and changes to its
    /// wall clock time and can be used by clients to provide a common time
    /// reference for all hosts.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn retrieve_hardware_uptime(&self) -> Result<i64> {
        let path = format!("/HostSystem/{moId}/RetrieveHardwareUptime", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
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
        let path = format!("/HostSystem/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Shuts down a host.
    /// 
    /// If the command is successful, then the host has been shut down.
    /// Thus, the client never receives an indicator of success in the returned task if
    /// connected directly to the host.
    /// 
    /// This command is not supported on all hosts. Check the host capability
    /// *HostCapability.shutdownSupported*.
    /// 
    /// ***Required privileges:*** Host.Config.Maintenance
    ///
    /// ## Parameters:
    ///
    /// ### force
    /// Flag to specify whether or not the host should be shut down
    /// regardless of whether it is in maintenance mode.
    /// If true, the host is shut down, even if there are
    /// virtual machines running or other operations in progress.
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
    /// ***InvalidState***: if "force" is false and the host is not in
    /// maintenance mode.
    /// 
    /// ***NotSupported***: if the host does not support shutdown.
    pub async fn shutdown_host_task(&self, force: bool) -> Result<ManagedObjectReference> {
        let input = ShutdownHostRequestType {force, };
        let path = format!("/HostSystem/{moId}/ShutdownHost_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Update flags that are part of the *HostFlagInfo* object.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    ///
    /// ## Parameters:
    ///
    /// ### flag_info
    /// -
    pub async fn update_flags(&self, flag_info: &HostFlagInfo) -> Result<()> {
        let input = UpdateFlagsRequestType {flag_info, };
        let path = format!("/HostSystem/{moId}/UpdateFlags", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Update fields that are part of the *HostIpmiInfo* object.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    ///
    /// ## Parameters:
    ///
    /// ### ipmi_info
    /// -
    ///
    /// ## Errors:
    ///
    /// ***InvalidIpmiLoginInfo***: if the supplied user ID and/or password is invalid.
    /// 
    /// ***InvalidIpmiMacAddress***: if the supplied MAC address is invalid.
    pub async fn update_ipmi(&self, ipmi_info: &HostIpmiInfo) -> Result<()> {
        let input = UpdateIpmiRequestType {ipmi_info, };
        let path = format!("/HostSystem/{moId}/UpdateIpmi", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Change and reconfigure the VMware Tools repository on the host.
    /// 
    /// If the new path is the same as the path already configured on
    /// the host, no changes will be made to the host.
    /// The host should be powered on.
    /// 
    /// This task is not cancellable and cannot be reverted once started.
    /// 
    /// ***Required privileges:*** Host.Config.ProductLocker
    ///
    /// ## Parameters:
    ///
    /// ### path
    /// The absolute path for the VMware Tools repository
    /// on the host. It should have "/vmfs/volumes/" prefix and
    /// it should be a valid existing path, or it could be
    /// empty to restore to default value.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the operation. The *info.result*
    /// property in the *Task* contains the stable vmfs path
    /// of the VMware Tools repository upon success. A stable vmfs
    /// path is of the form:
    /// /vmfs/volumes/\[datastore-uuid\]/\[path/inside/datastore\]
    /// or
    /// empty to indicate restoring to default value.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the path does not have "/vmfs/volumes/"
    /// prefix and is not empty.
    /// 
    /// ***FileNotFound***: if the path does not exist.
    /// 
    /// ***TaskInProgress***: if there is another task configuring the
    /// VMware Tools repository on the host.
    /// 
    /// ***HostConfigFault***: if the configuration could not be written.
    pub async fn update_product_locker_location_task(&self, path: &str) -> Result<ManagedObjectReference> {
        let input = UpdateProductLockerLocationRequestType {path, };
        let path = format!("/HostSystem/{moId}/UpdateProductLockerLocation_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of Vsphere API 6.0. Please, contact VMware Support to get
    /// instructions on how to configure system ESX resource pools.
    /// 
    /// Update the configuration of the system resource hierarchy.
    /// 
    /// ***Required privileges:*** Host.Config.Resources
    ///
    /// ## Parameters:
    ///
    /// ### resource_info
    /// -
    pub async fn update_system_resources(&self, resource_info: &HostSystemResourceInfo) -> Result<()> {
        let input = UpdateSystemResourcesRequestType {resource_info, };
        let path = format!("/HostSystem/{moId}/UpdateSystemResources", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Update the System Swap Configuration.
    /// 
    /// See also *HostSystemSwapConfiguration*.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    ///
    /// ## Parameters:
    ///
    /// ### sys_swap_config
    /// Contains a list of system swap options that
    /// configure the system swap functionality.
    pub async fn update_system_swap_configuration(&self, sys_swap_config: &HostSystemSwapConfiguration) -> Result<()> {
        let input = UpdateSystemSwapConfigurationRequestType {sys_swap_config, };
        let path = format!("/HostSystem/{moId}/UpdateSystemSwapConfiguration", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Whether alarm actions are enabled for this entity.
    /// 
    /// True if enabled; false otherwise.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn alarm_actions_enabled(&self) -> Result<Option<bool>> {
        let path = format!("/HostSystem/{moId}/alarmActionsEnabled", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Host answer file validation result.
    pub async fn answer_file_validation_result(&self) -> Result<Option<AnswerFileStatusResult>> {
        let path = format!("/HostSystem/{moId}/answerFileValidationResult", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Host answer file validation state.
    pub async fn answer_file_validation_state(&self) -> Result<Option<AnswerFileStatusResult>> {
        let path = format!("/HostSystem/{moId}/answerFileValidationState", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/HostSystem/{moId}/availableField", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Host capabilities.
    /// 
    /// This might not be available for a
    /// disconnected host.
    pub async fn capability(&self) -> Result<Option<HostCapability>> {
        let path = format!("/HostSystem/{moId}/capability", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// The host profile compliance check result.
    pub async fn compliance_check_result(&self) -> Result<Option<ComplianceResult>> {
        let path = format!("/HostSystem/{moId}/complianceCheckResult", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// The host profile compliance check state.
    pub async fn compliance_check_state(&self) -> Result<Option<HostSystemComplianceCheckState>> {
        let path = format!("/HostSystem/{moId}/complianceCheckState", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Host configuration information.
    /// 
    /// This might not be available for a disconnected
    /// host.
    pub async fn config(&self) -> Result<Option<HostConfigInfo>> {
        let path = format!("/HostSystem/{moId}/config", moId = &self.mo_id);
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
    pub async fn config_issue(&self) -> Result<Option<Vec<Box<dyn crate::types::event_trait::EventTrait>>>> {
        let path = format!("/HostSystem/{moId}/configIssue", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Host configuration systems.
    /// 
    /// In releases after vSphere API 5.0, vSphere Servers might not
    /// generate property collector update notifications for this property.
    /// To obtain the latest value of the property, you can use
    /// PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
    /// If you use the PropertyCollector.WaitForUpdatesEx method, specify
    /// an empty string for the version parameter. Any other version value will not
    /// produce any property values as no updates are generated.
    pub async fn config_manager(&self) -> Result<HostConfigManager> {
        let path = format!("/HostSystem/{moId}/configManager", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
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
        let path = format!("/HostSystem/{moId}/configStatus", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Custom field values.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn custom_value(&self) -> Result<Option<Vec<Box<dyn crate::types::custom_field_value_trait::CustomFieldValueTrait>>>> {
        let path = format!("/HostSystem/{moId}/customValue", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// A collection of references to the subset of datastore objects in the datacenter
    /// that are available in this HostSystem.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Datastore*.
    pub async fn datastore(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/HostSystem/{moId}/datastore", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// DatastoreBrowser to browse datastores for this host.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instance of *HostDatastoreBrowser*.
    pub async fn datastore_browser(&self) -> Result<ManagedObjectReference> {
        let path = format!("/HostSystem/{moId}/datastoreBrowser", moId = &self.mo_id);
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
        let path = format!("/HostSystem/{moId}/declaredAlarmState", moId = &self.mo_id);
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
        let path = format!("/HostSystem/{moId}/disabledMethod", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Access rights the current session has to this entity.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn effective_role(&self) -> Result<Option<Vec<i32>>> {
        let path = format!("/HostSystem/{moId}/effectiveRole", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Hardware configuration of the host.
    /// 
    /// This might not be available for a
    /// disconnected host.
    pub async fn hardware(&self) -> Result<Option<HostHardwareInfo>> {
        let path = format!("/HostSystem/{moId}/hardware", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Information about all licensable resources, currently present on this host.
    /// 
    /// This information is used mostly by the modules, manipulating information
    /// in the *LicenseManager*. Developers of such modules
    /// should use this property instead of *hardware*.
    /// 
    /// NOTE:
    /// The values in this property may not be accurate for pre-5.0 hosts when returned by vCenter 5.0
    pub async fn licensable_resource(&self) -> Result<HostLicensableResourceInfo> {
        let path = format!("/HostSystem/{moId}/licensableResource", moId = &self.mo_id);
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
        let path = format!("/HostSystem/{moId}/name", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// A collection of references to the subset of network objects in the datacenter that
    /// are available in this HostSystem.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Network*.
    pub async fn network(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/HostSystem/{moId}/network", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
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
        let path = format!("/HostSystem/{moId}/overallStatus", moId = &self.mo_id);
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
    pub async fn parent(&self) -> Result<Option<ManagedObjectReference>> {
        let path = format!("/HostSystem/{moId}/parent", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// List of permissions defined for this entity.
    pub async fn permission(&self) -> Result<Option<Vec<Permission>>> {
        let path = format!("/HostSystem/{moId}/permission", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// The host profile precheck-remediation result.
    pub async fn precheck_remediation_result(&self) -> Result<Option<ApplyHostProfileConfigurationSpec>> {
        let path = format!("/HostSystem/{moId}/precheckRemediationResult", moId = &self.mo_id);
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
        let path = format!("/HostSystem/{moId}/recentTask", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// The host profile remediation result.
    pub async fn remediation_result(&self) -> Result<Option<ApplyHostProfileConfigurationResult>> {
        let path = format!("/HostSystem/{moId}/remediationResult", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// The host profile remediation state.
    pub async fn remediation_state(&self) -> Result<Option<HostSystemRemediationState>> {
        let path = format!("/HostSystem/{moId}/remediationState", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Runtime state information about the host such as connection state.
    pub async fn runtime(&self) -> Result<HostRuntimeInfo> {
        let path = format!("/HostSystem/{moId}/runtime", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Basic information about the host, including connection state.
    pub async fn summary(&self) -> Result<HostListSummary> {
        let path = format!("/HostSystem/{moId}/summary", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Reference for the system resource hierarchy, used for configuring the set of
    /// resources reserved to the system and unavailable to virtual machines.
    pub async fn system_resources(&self) -> Result<Option<HostSystemResourceInfo>> {
        let path = format!("/HostSystem/{moId}/systemResources", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// The set of tags associated with this managed entity.
    /// 
    /// Experimental. Subject to change.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn tag(&self) -> Result<Option<Vec<Tag>>> {
        let path = format!("/HostSystem/{moId}/tag", moId = &self.mo_id);
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
        let path = format!("/HostSystem/{moId}/triggeredAlarmState", moId = &self.mo_id);
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
    pub async fn value(&self) -> Result<Option<Vec<Box<dyn crate::types::custom_field_value_trait::CustomFieldValueTrait>>>> {
        let path = format!("/HostSystem/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// List of virtual machines associated with this host.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *VirtualMachine*.
    pub async fn vm(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/HostSystem/{moId}/vm", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ConfigureCryptoKeyRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "keyId")]
    key_id: Option<&'a CryptoKeyId>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct EnableCryptoRequestType<'a> {
    #[serde(rename = "keyPlain")]
    key_plain: &'a CryptoKeyPlain,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct EnterMaintenanceModeRequestType<'a> {
    timeout: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "evacuatePoweredOffVms")]
    evacuate_powered_off_vms: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maintenanceSpec")]
    maintenance_spec: Option<&'a HostMaintenanceSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PowerDownHostToStandByRequestType {
    #[serde(rename = "timeoutSec")]
    timeout_sec: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "evacuatePoweredOffVms")]
    evacuate_powered_off_vms: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ExitMaintenanceModeRequestType {
    timeout: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PowerUpHostFromStandByRequestType {
    #[serde(rename = "timeoutSec")]
    timeout_sec: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryMemoryOverheadRequestType {
    #[serde(rename = "memorySize")]
    memory_size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "videoRamSize")]
    video_ram_size: Option<i32>,
    #[serde(rename = "numVcpus")]
    num_vcpus: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryMemoryOverheadExRequestType<'a> {
    #[serde(rename = "vmConfigInfo")]
    vm_config_info: &'a VirtualMachineConfigInfo,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RebootHostRequestType {
    force: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReconnectHostRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cnxSpec")]
    cnx_spec: Option<&'a HostConnectSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "reconnectSpec")]
    reconnect_spec: Option<&'a HostSystemReconnectSpec>,
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
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ShutdownHostRequestType {
    force: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateFlagsRequestType<'a> {
    #[serde(rename = "flagInfo")]
    flag_info: &'a HostFlagInfo,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateIpmiRequestType<'a> {
    #[serde(rename = "ipmiInfo")]
    ipmi_info: &'a HostIpmiInfo,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateProductLockerLocationRequestType<'a> {
    path: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateSystemResourcesRequestType<'a> {
    #[serde(rename = "resourceInfo")]
    resource_info: &'a HostSystemResourceInfo,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateSystemSwapConfigurationRequestType<'a> {
    #[serde(rename = "sysSwapConfig")]
    sys_swap_config: &'a HostSystemSwapConfiguration,
}
