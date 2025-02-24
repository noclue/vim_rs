use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::AlarmState;
use crate::types::structs::ClusterActionHistory;
use crate::types::structs::ClusterComputeResourceHciConfigInfo;
use crate::types::structs::ClusterComputeResourceHciConfigSpec;
use crate::types::structs::ClusterComputeResourceHostConfigurationInput;
use crate::types::structs::ClusterComputeResourceSummary;
use crate::types::structs::ClusterConfigInfo;
use crate::types::structs::ClusterConfigSpec;
use crate::types::structs::ClusterDrsFaults;
use crate::types::structs::ClusterDrsMigration;
use crate::types::structs::ClusterDrsRecommendation;
use crate::types::structs::ClusterEnterMaintenanceResult;
use crate::types::structs::ClusterHostRecommendation;
use crate::types::structs::ClusterRecommendation;
use crate::types::structs::ClusterResourceUsageSummary;
use crate::types::structs::CustomFieldDef;
use crate::types::structs::HostConnectSpec;
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::Permission;
use crate::types::structs::PlacementResult;
use crate::types::structs::PlacementSpec;
use crate::types::structs::SddcBase;
use crate::types::structs::Tag;
/// The *ClusterComputeResource* data object aggregates the compute
/// resources of associated *HostSystem* objects into a single
/// compute resource for use by virtual machines.
/// 
/// The cluster services
/// such as HA (High Availability), DRS (Distributed Resource Scheduling),
/// and EVC (Enhanced vMotion Compatibility), enhance the utility of this
/// single compute resource.
/// 
/// Use the *Folder*.*Folder.CreateClusterEx* method
/// to create an instance of this object.
pub struct ClusterComputeResource {
    client: Arc<Client>,
    mo_id: String,
}
impl ClusterComputeResource {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Opt out of the HCI workflow.
    /// 
    /// This operation is only allowed on a cluster
    /// that was created with the HCI workflow.
    /// When the cluster is created, but still unconfigured, the
    /// *workflowState*
    /// is "in\_progress". The AbandonHciWorkflow method may be called at any time before
    /// cluster configuration begins; it is not possible to abandon the workflow
    /// during the configuration procedure.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn abandon_hci_workflow(&self) -> Result<()> {
        let path = format!("/ClusterComputeResource/{moId}/AbandonHciWorkflow", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Adds a host to the cluster.
    /// 
    /// The hostname must be either an IP address, such as
    /// 192.168.0.1, or a DNS resolvable name. DNS names may be fully qualified names,
    /// such as host1.domain1.com, or a short name such as host1, providing host1 resolves
    /// to host1.domain1.com. The system uses DNS to resolve short names to fully qualified
    /// names. If the cluster supports nested resource pools and the user specifies the
    /// optional ResourcePool argument, then the host's root resource pool becomes the
    /// specified resource pool. The stand-alone host resource hierarchy is imported into
    /// the new nested resource pool.
    /// 
    /// If the cluster does not support nested resource pools, then the stand-alone host
    /// resource hierarchy is discarded and all virtual machines on the host are put
    /// under the cluster's root resource pool.
    /// 
    /// In addition to the Host.Inventory.AddHostToCluster and
    /// Resource.AssignVMToPool privileges, it requires System.View privilege on
    /// the VM folder that the VMs of the host will be placed on.
    /// 
    /// ***Required privileges:*** Host.Inventory.AddHostToCluster
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// Specifies the parameters needed to add a single host.
    ///
    /// ### as_connected
    /// Flag to specify whether or not the host should be connected
    /// immediately after it is added. The host will not be added if
    /// a connection attempt is made and fails.
    ///
    /// ### resource_pool
    /// the resource pool for the root resource pool from the host.
    /// 
    /// ***Required privileges:*** Resource.AssignVMToPool
    /// 
    /// Refers instance of *ResourcePool*.
    ///
    /// ### license
    /// Provide a licenseKey or licenseKeyType. See *LicenseManager*
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation. The *info.result* property in the
    /// *Task* contains the newly added *HostSystem* upon
    /// success.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidLogin***: if "asConnected" is specified but authentication with the
    /// new host fails.
    /// 
    /// ***HostConnectFault***: if an error occurred when connecting to a host.
    /// Typically, a more specific subclass, such as AlreadyBeingManaged,
    /// is thrown.
    /// 
    /// ***AlreadyBeingManaged***: if the host is already being managed by a
    /// VirtualCenter server.
    /// 
    /// ***NotEnoughLicenses***: if no licenses are available to add this host.
    /// 
    /// ***NoHost***: if the host cannot be contacted.
    /// 
    /// ***NotSupportedHost***: if the host is running a software version that does
    /// not support clustering features. It may still be possible to add
    /// the host as a stand-alone host.
    /// 
    /// ***TooManyHosts***: if no additional hosts can be added to the cluster.
    /// 
    /// ***AgentInstallFailed***: if there is an error installing the VirtualCenter agent
    /// on the host.
    /// 
    /// ***AlreadyConnected***: if asConnected is true and the host is already
    /// connected to VirtualCenter.
    /// 
    /// ***SSLVerifyFault***: if the host certificate could not be authenticated
    /// 
    /// ***DuplicateName***: if another host in the same cluster has the name.
    /// 
    /// ***NoPermission***: if there are crypto keys to be sent to the host,
    /// but the user does not have Cryptographer.RegisterHost privilege
    /// on the Cluster.
    pub async fn add_host_task(&self, spec: &HostConnectSpec, as_connected: bool, resource_pool: Option<&ManagedObjectReference>, license: Option<&str>) -> Result<ManagedObjectReference> {
        let input = AddHostRequestType {spec, as_connected, resource_pool, license, };
        let path = format!("/ClusterComputeResource/{moId}/AddHost_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Applies a recommendation from the drsRecommendation or the
    /// recommendation list.
    /// 
    /// Each recommendation can be applied only
    /// once.
    /// 
    /// resource.applyRecommendation privilege is required if the recommendation
    /// is DRS migration or power management recommendations.
    ///
    /// ## Parameters:
    ///
    /// ### key
    /// The key field of the DrsRecommendation or Recommendation.
    pub async fn apply_recommendation(&self, key: &str) -> Result<()> {
        let input = ApplyRecommendationRequestType {key, };
        let path = format!("/ClusterComputeResource/{moId}/ApplyRecommendation", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Cancels a recommendation.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### key
    /// The key field of the Recommendation.
    pub async fn cancel_recommendation(&self, key: &str) -> Result<()> {
        let input = CancelRecommendationRequestType {key, };
        let path = format!("/ClusterComputeResource/{moId}/CancelRecommendation", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Configures the cluster.
    /// 
    /// This API requires Host.Inventory.EditCluster privilege on the cluster
    /// and the hosts; additional privileges might be required depending on the
    /// inputs.
    /// This operation is only allowed on a cluster that was created
    /// with the HCI workflow.
    /// Before calling this method, it is recommended that
    /// *ClusterComputeResource.ValidateHCIConfiguration* is
    /// invoked with the DvsProfile objects listed in
    /// *ClusterComputeResourceHCIConfigSpec.dvsProf* along with the hosts listed in
    /// *ClusterComputeResourceHostConfigurationInput* to validate that
    /// the desired network settings can be applied correctly.
    ///
    /// ## Parameters:
    ///
    /// ### cluster_spec
    /// Specification to configure the cluster,
    /// see *ClusterComputeResourceHCIConfigSpec*
    /// for details. The *DistributedVirtualSwitch* and
    /// *DistributedVirtualPortgroup* objects contained
    /// within the specification must be in the same datacenter as the
    /// cluster. Specify *ClusterComputeResourceHCIConfigSpec.vSanConfigSpec* only when
    /// vSan is enabled on the cluster.
    ///
    /// ### host_inputs
    /// Inputs to configure each host in the cluster,
    /// see *ClusterComputeResourceHostConfigurationInput*
    /// for details. Hosts in this list should be part of the cluster and
    /// should be in maintenance mode for them to be configured per
    /// specification. If this parameter is not specified, the API
    /// operates on all the hosts in the cluster. Hosts which were not
    /// configured due to not being in maintenance
    /// mode will be returned in *ClusterComputeResourceClusterConfigResult.failedHosts*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the operation. The *TaskInfo.result* property
    /// in the *Task* contains a *ClusterComputeResourceClusterConfigResult*
    /// object, which upon completion will contain a list of hosts which
    /// were successfully configured and a list of hosts
    /// which could not be configured.
    /// 
    /// Refers instance of *Task*.
    pub async fn configure_hci_task(&self, cluster_spec: &ClusterComputeResourceHciConfigSpec, host_inputs: Option<&[ClusterComputeResourceHostConfigurationInput]>) -> Result<ManagedObjectReference> {
        let input = ConfigureHciRequestType {cluster_spec, host_inputs, };
        let path = format!("/ClusterComputeResource/{moId}/ConfigureHCI_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    /// ***Required privileges:*** Host.Inventory.DeleteCluster
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
        let path = format!("/ClusterComputeResource/{moId}/Destroy_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// The API takes a list of hosts in the cluster as input, and
    /// returns a list of hosts in "ClusterMaintenanceResult" that the
    /// server can successfully evacuate given the existing
    /// constraints in the cluster, such as HA, FT, Vmotion
    /// compatibility, reservations, affinity rules, etc.
    /// 
    /// The client is allowed to pass all hosts in the cluster to the
    /// API, even though all of them cannot enter maintenance mode at
    /// the same time. The list returned from the API contains the
    /// largest number of hosts that the server can evacuate
    /// simultaneously. The client can then request to enter each host
    /// in the returned list into maintenance mode.
    /// The client can specify an integer "DemandCapacityRatioTarget"
    /// option in the "option" parameter. The allowed values of the
    /// option range from 40 to 200, and the default value is 100. This
    /// option controls how much resource overcommitment the server
    /// should make in consolidating the VMs onto fewer hosts. A value
    /// of 100 means the server will keep the same amount of powered-on
    /// capacity as the current VM demands. A value less than 100 means
    /// undercommitted resources. A value greater than 100 means
    /// overcommitted resources.
    /// The hosts are recommended based on the inventory at the time of
    /// the API invocation. It is not guaranteed that the actual
    /// enter-maintenance tasks on the hosts will succeed, if the
    /// inventory changes after the API returns, or if vmotions fail
    /// due to unexpected conditions. For possible exceptions thrown
    /// by the necessary relocate operations, see
    /// *VirtualMachine.MigrateVM_Task*.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// The array of hosts to put into maintenance mode.
    /// 
    /// ***Required privileges:*** Host.Config.Maintenance
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ### option
    /// An array of *OptionValue*
    /// options for this query. The specified options override the
    /// advanced options in *ClusterDrsConfigInfo*.
    ///
    /// ## Returns:
    ///
    /// A *ClusterEnterMaintenanceResult* object,
    /// which consists of an array of recommendations for hosts that
    /// can be evacuated and an array of faults for hosts that cannot
    /// be evacuated.
    pub async fn cluster_enter_maintenance_mode(&self, host: &[ManagedObjectReference], option: Option<&[Box<dyn crate::types::option_value_trait::OptionValueTrait>]>) -> Result<ClusterEnterMaintenanceResult> {
        let input = ClusterEnterMaintenanceModeRequestType {host, option, };
        let path = format!("/ClusterComputeResource/{moId}/ClusterEnterMaintenanceMode", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// A managed object that controls Enhanced vMotion Compatibility mode for
    /// this cluster.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// Refers instance of *ClusterEVCManager*.
    pub async fn evc_manager(&self) -> Result<Option<ManagedObjectReference>> {
        let path = format!("/ClusterComputeResource/{moId}/EvcManager", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Extend an existing HCI cluster.
    /// 
    /// This API requires Host.Inventory.EditCluster privilege on the cluster
    /// and the hosts, additional privileges might be required depending on the
    /// inputs.
    ///
    /// ## Parameters:
    ///
    /// ### host_inputs
    /// Inputs to configure specified set of hosts in the
    /// cluster. See
    /// *ClusterComputeResourceHostConfigurationInput*
    /// for details. Hosts in this list should be part of the cluster and
    /// should be in maintenance mode for them to be configured per
    /// specification. Hosts which were not configured due to not
    /// being in maintenance mode will be returned in
    /// *ClusterComputeResourceClusterConfigResult.failedHosts*. Specify
    /// *ClusterComputeResourceHostConfigurationInput.hostVmkNics* only if *dvsSetting*
    /// is set.
    ///
    /// ### v_san_config_spec
    /// Specification to configure vSAN on specified set of
    /// hosts. See vim.vsan.ReconfigSpec for details. This parameter
    /// should be specified only when vSan is enabled on the cluster.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the operation. The *TaskInfo.result* property
    /// in the *Task* contains a *ClusterComputeResourceClusterConfigResult*
    /// object, which upon successful completion would contain the list
    /// of hosts which couldn't be configured and a list of hosts which
    /// were successfully configured. This API can be called only after
    /// the cluster is configured using *ClusterComputeResource.ConfigureHCI_Task* and requires
    /// *ClusterComputeResourceHCIConfigInfo.workflowState* to be "done".
    /// 
    /// Refers instance of *Task*.
    pub async fn extend_hci_task(&self, host_inputs: Option<&[ClusterComputeResourceHostConfigurationInput]>, v_san_config_spec: Option<&SddcBase>) -> Result<ManagedObjectReference> {
        let input = ExtendHciRequestType {host_inputs, v_san_config_spec, };
        let path = format!("/ClusterComputeResource/{moId}/ExtendHCI_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Finds all enabled and disabled VM-VM Affinity and Anti-Affinity rules,
    /// involving the given Virtual Machine.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// The vm whose rules need to be looked up.
    /// 
    /// Refers instance of *VirtualMachine*.
    pub async fn find_rules_for_vm(&self, vm: &ManagedObjectReference) -> Result<Option<Vec<Box<dyn crate::types::cluster_rule_info_trait::ClusterRuleInfoTrait>>>> {
        let input = FindRulesForVmRequestType {vm, };
        let path = format!("/ClusterComputeResource/{moId}/FindRulesForVm", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// This API can be invoked to get the current CPU, memory and storage usage
    /// in the cluster.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// An instance of ResourceUsageSummary with following information:
    /// 1. cpuCapacityMHz: Sum of CPU capacity of all the available hosts in the
    ///    cluster in MHz.
    /// 2. cpuUsedMHz: Sum of CPU consumed in all the available hosts in the cluster
    ///    in MHz.
    /// 3. memCapacityMB: Sum of memory capacity of all the available hosts in the
    ///    cluster in MB.
    /// 4. memUsedMB: Sum of memory consumed in all the available hosts in this
    ///    cluster in MB.
    /// 5. storageCapacityMB: Total storage capacity of all the accessible datastores
    ///    in this cluster.
    /// 6. storageUsedMB: Total storage consumed in all the accessible datastores in
    ///    this cluster.
    pub async fn get_resource_usage(&self) -> Result<ClusterResourceUsageSummary> {
        let path = format!("/ClusterComputeResource/{moId}/GetResourceUsage", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Retrieve all the datastores that are either listed in
    /// *ClusterSystemVMsConfigInfo.notAllowedDatastores* or are
    /// tagged with a category from
    /// *ClusterSystemVMsConfigInfo.dsTagCategoriesToExclude*.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// a list of restricted datastores.
    /// 
    /// Refers instances of *Datastore*.
    pub async fn get_system_v_ms_restricted_datastores(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/ClusterComputeResource/{moId}/GetSystemVMsRestrictedDatastores", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Moves an existing host into a cluster.
    /// 
    /// The host must be part of the same
    /// datacenter, and if the host is part of a cluster, the host must be in maintenance
    /// mode.
    /// 
    /// If the host is a stand-alone host, the stand-alone ComputeResource is removed
    /// as part of this operation.
    /// 
    /// All virtual machines associated with the host, regardless of whether or not they
    /// are running, are moved with the host into the cluster. If there are virtual
    /// machines that should not be moved, then migrate those virtual machines off the
    /// host before initiating this operation.
    /// 
    /// If the host is a stand-alone host, the cluster supports nested resource pools,
    /// and the user specifies the optional resourcePool argument, then the stand-alone
    /// host's root resource pool becomes the specified resource pool and the stand-alone
    /// host resource hierarchy is imported into the new nested resource pool. If the
    /// cluster does not support nested resource pools or the resourcePool argument is not
    /// specified, then the stand-alone host resource hierarchy is ignored.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// The list of hosts to move into the cluster.
    /// 
    /// ***Required privileges:*** Host.Inventory.MoveHost
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### resource_pool
    /// The resource pool to match the root resource pool of
    /// stand-alone hosts. This argument has no effect if the host is part of a
    /// cluster.
    /// 
    /// Refers instance of *ResourcePool*.
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
    /// ***NotSupportedHost***: if the host is running a software version that does
    /// not support clustering.
    /// 
    /// ***TooManyHosts***: if no additional hosts can be added to the cluster.
    /// 
    /// ***InvalidArgument***: if the host is not a part of the same datacenter as
    /// the cluster or if the specified resource pool is not part of the cluster
    /// or if the source and destination clusters are the same.
    /// 
    /// ***InvalidState***: if a host is already part of a cluster and is not in
    /// maintenance mode.
    pub async fn move_host_into_task(&self, host: &ManagedObjectReference, resource_pool: Option<&ManagedObjectReference>) -> Result<ManagedObjectReference> {
        let input = MoveHostIntoRequestType {host, resource_pool, };
        let path = format!("/ClusterComputeResource/{moId}/MoveHostInto_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Moves an existing host into a cluster.
    /// 
    /// The host must be part of the same
    /// datacenter, and if the host is part of a cluster, the host must be in maintenance
    /// mode.
    /// 
    /// If the host is part of a stand-alone ComputeResource, then the stand-alone
    /// ComputeResource is removed as part of this operation.
    /// 
    /// All virtual machines associated with a host, regardless of whether or not they
    /// are running, are moved with the host into the cluster. If there are virtual
    /// machines that should not be moved, then migrate those virtual machines off the
    /// host before initiating this operation.
    /// 
    /// For stand-alone hosts, the host resource pool hierarchy is discarded in this call.
    /// To preserve a host resource pools from a stand-alone host, call moveHostInt,
    /// specifying an optional resource pool. This operation is transactional only with
    /// respect to each individual host. Hosts in the set are moved sequentially and are
    /// committed, one at a time. If a failure is detected, then the method terminates
    /// with an exception. Since hosts are moved one at a time, if this operation fails
    /// while in the process of moving multiple hosts, some hosts are left unmoved.
    /// 
    /// In addition to the privileges mentioned, the user must also hold
    /// Host.Inventory.EditCluster on the host's source ComputeResource object.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// The list of hosts to move into the cluster.
    /// 
    /// ***Required privileges:*** Host.Inventory.MoveHost
    /// 
    /// Refers instances of *HostSystem*.
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
    /// ***NotSupportedHost***: if the host is running a software version that does
    /// not support clustering features.
    /// 
    /// ***TooManyHosts***: if no additional hosts can be added to the cluster.
    /// 
    /// ***InvalidArgument***: if one of the hosts is not part of the same datacenter
    /// as the cluster.
    /// 
    /// ***InvalidState***: if a host is already part of a cluster and is not in
    /// maintenance mode.
    /// 
    /// ***DuplicateName***: if the host is already in the cluster
    /// 
    /// ***DisallowedOperationOnFailoverHost***: if the host is being moved
    /// from a cluster and was configured as a failover host in that
    /// cluster. See *ClusterFailoverHostAdmissionControlPolicy*.
    pub async fn move_into_task(&self, host: &[ManagedObjectReference]) -> Result<ManagedObjectReference> {
        let input = MoveIntoRequestType {host, };
        let path = format!("/ClusterComputeResource/{moId}/MoveInto_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// This method returns a *PlacementResult* object.
    /// 
    /// This API can be invoked to ask DRS for a set of recommendations
    /// for moving a virtual machine and its virtual disks into a cluster.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### placement_spec
    /// Specification for placing a virtual machine
    /// and its virtual disks
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if invoked on a DRS disabled cluster.
    /// 
    /// ***InvalidArgument***: in case of errors in the input "placementSpec".
    /// The API can be used for either intra-vCenter migration or
    /// cross-vCenter migration, with different requirements for the
    /// PlacementSpec.
    ///   For intra-vCenter migration, the requirements for PlacementSpec are:
    /// - PlacementSpec.vm is required.
    /// - PlacementSpec.relocateSpec can be used to optionally specify the
    ///   target host, target datastore, or target resource pool for the migration.
    /// - PlacementSpec.hosts can be used to optionally specify a list of
    ///   compatible hosts for the incoming virtual machine. If this list is empty,
    ///   all hosts in the cluster will be considered for placement.
    /// - PlacementSpec.datastores can be used to optionally specify a list of
    ///   compatible datastores for the incoming virtual machine. If this list is
    ///   empty, all datastores connected to the hosts in the cluster will be
    ///   considered for placement.
    /// - PlacementSpec.storagePods can be used to optionally specify a list of
    ///   compatible datastore clusters for the incoming virtual machine. If this
    ///   list is empty, all datastores connected to the hosts in the cluster will
    ///   be considered for placement.
    /// <!-- -->
    ///   For cross-vCenter migration, the requirements for PlacementSpec are:
    /// - PlacementSpec.configSpec is required. Within the ConfigSpec, the
    ///   following elements are required if PlacementSpec.relocateSpec.host is
    ///   empty: version, cpuAllocation, memoryAllocation, numCPUs, memoryMB;
    ///   additionally, the following elements of the ConfigSpec are required if
    ///   PlacementSpec.relocateSpec.datastore is empty: files, swapPlacement,
    ///   deviceChange.
    /// - PlacementSpec.relocateSpec can be used to optionally specify the
    ///   target host, target datastore, or target resource pool for the migration.
    /// - PlacementSpec.hosts is required, if PlacementSpec.relocateSpec.host is
    ///   empty; otherwise, the selected hosts in the PlacementResult are not
    ///   guaranteed to be compatible with the incoming virtual machine.
    /// - PlacementSpec.datastores is required, if PlacementSpec.relocateSpec.datastore
    ///   is empty; otherwise, the selected datastores in the PlacementResult are
    ///   not guaranteed to be compatible with the incoming virtual machine.
    pub async fn place_vm(&self, placement_spec: &PlacementSpec) -> Result<PlacementResult> {
        let input = PlaceVmRequestType {placement_spec, };
        let path = format!("/ClusterComputeResource/{moId}/PlaceVm", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of VI API 2.5, use *Datacenter.PowerOnMultiVM_Task*.
    /// *ClusterComputeResource.RecommendHostsForVm* cannot make any recommendations if DRS cannot
    /// find the specified host in the cluster.
    /// With *Datacenter.PowerOnMultiVM_Task*, DRS attempts to migrate virtual machines
    /// and power on hosts in standby mode, given the same conditions.
    /// 
    /// Gets a recommendation for where to power on, resume, revert
    /// from powered-off state to powered on state, or to migrate a
    /// specific virtual machine.
    /// 
    /// If no host is found, an empty list is
    /// returned.
    /// 
    /// The type of operation is implied by the state of the virtual machine. Returned
    /// hosts are intended for power-on or resume if the virtual machine is powered-off or
    /// suspended. However, if the virtual machine is powered-on, the request is assumed
    /// to be for migrating a virtual machine into a DRS enabled cluster. In that case,
    /// the ResourcePool argument should be specified and the ResourcePool and the virtual
    /// machine cannot be in the same cluster.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### vm
    /// Specifies the virtual machine for which the user is requesting a
    /// recommendations.
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ### pool
    /// Specifies the ResourcePool into which the virtual machine is to be
    /// migrated. If the virtual machine is powered-on, this argument must be
    /// specified and it is relevant only when the virtual machine is
    /// powered-on. This ResourcePool cannot be in the same cluster as the
    /// virtual machine.
    /// 
    /// Refers instance of *ResourcePool*.
    ///
    /// ## Returns:
    ///
    /// An array of HostRecommendation ordered by their rating.
    pub async fn recommend_hosts_for_vm(&self, vm: &ManagedObjectReference, pool: Option<&ManagedObjectReference>) -> Result<Option<Vec<ClusterHostRecommendation>>> {
        let input = RecommendHostsForVmRequestType {vm, pool, };
        let path = format!("/ClusterComputeResource/{moId}/RecommendHostsForVm", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Deprecated as of VI API 2.5, use *ComputeResource.ReconfigureComputeResource_Task*.
    /// 
    /// Reconfigures a cluster.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// A set of configuration changes to apply to the cluster. The
    /// specification can be a complete set of changes or a partial set of
    /// changes, applied incrementally.
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
    /// This method returns a *Task* object with which to monitor the
    /// operation.
    /// 
    /// Refers instance of *Task*.
    pub async fn reconfigure_cluster_task(&self, spec: &ClusterConfigSpec, modify: bool) -> Result<ManagedObjectReference> {
        let input = ReconfigureClusterRequestType {spec, modify, };
        let path = format!("/ClusterComputeResource/{moId}/ReconfigureCluster_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn reconfigure_compute_resource_task(&self, spec: &dyn crate::types::compute_resource_config_spec_trait::ComputeResourceConfigSpecTrait, modify: bool) -> Result<ManagedObjectReference> {
        let input = ReconfigureComputeResourceRequestType {spec, modify, };
        let path = format!("/ClusterComputeResource/{moId}/ReconfigureComputeResource_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Make DRS invoke again and return a new list of recommendations.
    /// 
    /// Concurrent "refresh" requests may be combined together and trigger only
    /// one DRS invocation.
    /// 
    /// The recommendations generated is stored at *ClusterComputeResource.recommendation*.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    pub async fn refresh_recommendation(&self) -> Result<()> {
        let path = format!("/ClusterComputeResource/{moId}/RefreshRecommendation", moId = &self.mo_id);
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
        let path = format!("/ClusterComputeResource/{moId}/Reload", moId = &self.mo_id);
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
    /// ***Required privileges:*** Host.Inventory.RenameCluster
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
        let path = format!("/ClusterComputeResource/{moId}/Rename_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Retrieve DAS advanced runtime info for this cluster.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn retrieve_das_advanced_runtime_info(&self) -> Result<Option<Box<dyn crate::types::cluster_das_advanced_runtime_info_trait::ClusterDasAdvancedRuntimeInfoTrait>>> {
        let path = format!("/ClusterComputeResource/{moId}/RetrieveDasAdvancedRuntimeInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Set the desired encryption mode and host key for the cluster.
    /// 
    /// The cryptoMode parameter can be used to set crypto mode policy for the
    /// cluster.
    /// 
    /// The desired host key of the cluster can also be specified optionally
    /// using the policy parameter.
    /// 
    /// ***Required privileges:*** Cryptographer.RegisterHost
    ///
    /// ## Parameters:
    ///
    /// ### crypto_mode
    /// The encryption mode for the cluster.
    /// See *ClusterCryptoConfigInfoCryptoMode_enum* for
    /// supported values. An empty string is treated as a valid
    /// input and will be interpreted as
    /// *onDemand*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidRequest***: if the interface is not implemented.
    /// 
    /// ***InvalidArgument***: if one of the parameters is invalid.
    pub async fn set_crypto_mode(&self, crypto_mode: &str) -> Result<()> {
        let input = SetCryptoModeRequestType {crypto_mode, };
        let path = format!("/ClusterComputeResource/{moId}/SetCryptoMode", moId = &self.mo_id);
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
        let path = format!("/ClusterComputeResource/{moId}/setCustomValue", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Stamp all rules in the cluster with ruleUuid.
    /// 
    /// If a rule has ruleUuid field set, and it has a value, leave it untouched.
    /// If rule's ruleUuid field is unset, generate a UUID and stamp the rule.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    pub async fn stamp_all_rules_with_uuid_task(&self) -> Result<ManagedObjectReference> {
        let path = format!("/ClusterComputeResource/{moId}/StampAllRulesWithUuid_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Validate HCI configuration in pre-configure and post-configure use-cases.
    /// 1. pre-configure use-case: Validates the HCI configuration to be applied on
    ///    the cluster. A successful validation in this case means the HCIConfigSpec
    ///    can be applied without errors on the cluster using
    ///    *ClusterComputeResource.ConfigureHCI_Task* or
    ///    *ClusterComputeResource.ExtendHCI_Task*  
    ///    These are the things the API validates:
    ///    1. When providing a set of physical adapters in the
    ///       *ClusterComputeResourceHCIConfigSpec.dvsProf* argument,
    ///       the API validates that all the adapters should be present on all the
    ///       hosts to be validated. The adapters should either be unmapped or mapped
    ///       to the same vSwitch across hosts. In addition to this, if the adapters
    ///       are connected to a *DistributedVirtualSwitch*, it should be
    ///       exactly the same way as specified in the
    ///       *ClusterComputeResourceHCIConfigSpec.dvsProf* or in the
    ///       *ClusterComputeResourceHCIConfigInfo.dvsSetting*.
    ///    2. The API will also validate that the ESXi versions of the hosts are
    ///       compatible with the version of the *DistributedVirtualSwitch*
    ///       being created.
    /// 2. post-configure case: Validate the cluster has been configured correctly
    ///    as per the *ClusterComputeResourceHCIConfigInfo* for the
    ///    cluster. In this case, the API should be invoked with both params omitted
    ///    as the intent is to validate all hosts in the cluster using the existing
    ///    *ClusterComputeResourceHCIConfigInfo*
    ///    
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### hci_config_spec
    /// The *ClusterComputeResourceHCIConfigSpec*
    /// to be used for validating the hosts. If not specified, the
    /// existing *ClusterComputeResourceHCIConfigInfo* of the
    /// cluster will be used.  
    /// Note:- This param must be omitted for post-configure validation.
    ///
    /// ### hosts
    /// The set of hosts to be validated. If not specified, the set
    /// of existing hosts in the cluster will be used.  
    /// Note:- This param must be omitted for post-configure validation.
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// A list of configuration errors. A non-empty list indicates
    /// validation has failed.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn validate_hci_configuration(&self, hci_config_spec: Option<&ClusterComputeResourceHciConfigSpec>, hosts: Option<&[ManagedObjectReference]>) -> Result<Option<Vec<Box<dyn crate::types::cluster_compute_resource_validation_result_base_trait::ClusterComputeResourceValidationResultBaseTrait>>>> {
        let input = ValidateHciConfigurationRequestType {hci_config_spec, hosts, };
        let path = format!("/ClusterComputeResource/{moId}/ValidateHCIConfiguration", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// The set of actions that have been performed recently.
    pub async fn action_history(&self) -> Result<Option<Vec<ClusterActionHistory>>> {
        let path = format!("/ClusterComputeResource/{moId}/actionHistory", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Whether alarm actions are enabled for this entity.
    /// 
    /// True if enabled; false otherwise.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn alarm_actions_enabled(&self) -> Result<Option<bool>> {
        let path = format!("/ClusterComputeResource/{moId}/alarmActionsEnabled", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// List of custom field definitions that are valid for the object's type.
    /// 
    /// The fields are sorted by *CustomFieldDef.name*.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn available_field(&self) -> Result<Option<Vec<CustomFieldDef>>> {
        let path = format!("/ClusterComputeResource/{moId}/availableField", moId = &self.mo_id);
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
        let path = format!("/ClusterComputeResource/{moId}/configIssue", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
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
        let path = format!("/ClusterComputeResource/{moId}/configManagerEnabled", moId = &self.mo_id);
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
    pub async fn config_status(&self) -> Result<crate::types::enums::ManagedEntityStatusEnum> {
        let path = format!("/ClusterComputeResource/{moId}/configStatus", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of VI API 2.5, use *ComputeResource.configurationEx*,
    /// which is a *ClusterConfigInfoEx* data object..
    /// 
    /// Configuration of the cluster.
    pub async fn configuration(&self) -> Result<ClusterConfigInfo> {
        let path = format!("/ClusterComputeResource/{moId}/configuration", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Configuration of the compute resource; applies to both standalone hosts
    /// and clusters.
    /// 
    /// For a cluster this property will return a
    /// *ClusterConfigInfoEx* object.
    pub async fn configuration_ex(&self) -> Result<Box<dyn crate::types::compute_resource_config_info_trait::ComputeResourceConfigInfoTrait>> {
        let path = format!("/ClusterComputeResource/{moId}/configurationEx", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Custom field values.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn custom_value(&self) -> Result<Option<Vec<Box<dyn crate::types::custom_field_value_trait::CustomFieldValueTrait>>>> {
        let path = format!("/ClusterComputeResource/{moId}/customValue", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn datastore(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/ClusterComputeResource/{moId}/datastore", moId = &self.mo_id);
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
        let path = format!("/ClusterComputeResource/{moId}/declaredAlarmState", moId = &self.mo_id);
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
        let path = format!("/ClusterComputeResource/{moId}/disabledMethod", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// A collection of the DRS faults generated in the last DRS invocation.
    /// 
    /// Each element of the collection is the set of faults generated in one
    /// recommendation.
    /// DRS faults are generated when DRS tries to make recommendations
    /// for rule enforcement, power management, etc., and indexed in a tree
    /// structure with reason for recommendations and VM to migrate (optional)
    /// as the index keys.
    /// In releases after vSphere API 5.0, vSphere Servers might not
    /// generate property collector update notifications for this property.
    /// To obtain the latest value of the property, you can use
    /// PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
    /// If you use the PropertyCollector.WaitForUpdatesEx method, specify
    /// an empty string for the version parameter. Any other version value will not
    /// produce any property values as no updates are generated.
    /// 
    /// ***Required privileges:*** System.Read
    pub async fn drs_fault(&self) -> Result<Option<Vec<ClusterDrsFaults>>> {
        let path = format!("/ClusterComputeResource/{moId}/drsFault", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Deprecated as of VI API 2.5, use
    /// *ClusterComputeResource.recommendation*.
    /// vSphere 6.5 is the last version where this property is populated.
    /// Later versions of vSphere no longer populate this property.
    /// 
    /// If DRS is enabled, this returns the set of recommended
    /// migrations from the DRS module.
    pub async fn drs_recommendation(&self) -> Result<Option<Vec<ClusterDrsRecommendation>>> {
        let path = format!("/ClusterComputeResource/{moId}/drsRecommendation", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Access rights the current session has to this entity.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn effective_role(&self) -> Result<Option<Vec<i32>>> {
        let path = format!("/ClusterComputeResource/{moId}/effectiveRole", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// The environment browser object that identifies the environments that are supported
    /// on this compute resource.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instance of *EnvironmentBrowser*.
    pub async fn environment_browser(&self) -> Result<Option<ManagedObjectReference>> {
        let path = format!("/ClusterComputeResource/{moId}/environmentBrowser", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// This is applicable to clusters which are configured using the HCI
    /// workflow and contains data related to the workflow and specification.
    pub async fn hci_config(&self) -> Result<Option<ClusterComputeResourceHciConfigInfo>> {
        let path = format!("/ClusterComputeResource/{moId}/hciConfig", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn host(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/ClusterComputeResource/{moId}/host", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
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
        let path = format!("/ClusterComputeResource/{moId}/lifecycleManaged", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// The set of migration decisions that have recently been performed.
    /// 
    /// This list is populated only when DRS is in automatic mode.
    pub async fn migration_history(&self) -> Result<Option<Vec<ClusterDrsMigration>>> {
        let path = format!("/ClusterComputeResource/{moId}/migrationHistory", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
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
        let path = format!("/ClusterComputeResource/{moId}/name", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
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
    pub async fn network(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/ClusterComputeResource/{moId}/network", moId = &self.mo_id);
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
        let path = format!("/ClusterComputeResource/{moId}/overallStatus", moId = &self.mo_id);
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
        let path = format!("/ClusterComputeResource/{moId}/parent", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// List of permissions defined for this entity.
    pub async fn permission(&self) -> Result<Option<Vec<Permission>>> {
        let path = format!("/ClusterComputeResource/{moId}/permission", moId = &self.mo_id);
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
        let path = format!("/ClusterComputeResource/{moId}/recentTask", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// List of recommended actions for the cluster.
    /// 
    /// It is
    /// possible that the current set of recommendations may be empty,
    /// either due to not having any running dynamic recommendation
    /// generation module, or since there may be no recommended actions
    /// at this time.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// An array of recommendations, with each of them having
    /// one or more actions.
    pub async fn recommendation(&self) -> Result<Option<Vec<ClusterRecommendation>>> {
        let path = format!("/ClusterComputeResource/{moId}/recommendation", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Reference to root resource pool.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instance of *ResourcePool*.
    pub async fn resource_pool(&self) -> Result<Option<ManagedObjectReference>> {
        let path = format!("/ClusterComputeResource/{moId}/resourcePool", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Basic runtime information about a compute resource.
    /// 
    /// This information is used on
    /// summary screens and in list views.
    pub async fn summary(&self) -> Result<Box<dyn crate::types::compute_resource_summary_trait::ComputeResourceSummaryTrait>> {
        let path = format!("/ClusterComputeResource/{moId}/summary", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated do not use this property.
    /// The same information could be obtained via
    /// *ComputeResource.summary*.
    /// 
    /// The cluster summary.
    /// 
    /// ***Since:*** vSphere API Release 7.0.1.1
    pub async fn summary_ex(&self) -> Result<ClusterComputeResourceSummary> {
        let path = format!("/ClusterComputeResource/{moId}/summaryEx", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// The set of tags associated with this managed entity.
    /// 
    /// Experimental. Subject to change.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn tag(&self) -> Result<Option<Vec<Tag>>> {
        let path = format!("/ClusterComputeResource/{moId}/tag", moId = &self.mo_id);
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
        let path = format!("/ClusterComputeResource/{moId}/triggeredAlarmState", moId = &self.mo_id);
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
        let path = format!("/ClusterComputeResource/{moId}/value", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddHostRequestType<'a> {
    spec: &'a HostConnectSpec,
    #[serde(rename = "asConnected")]
    as_connected: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resourcePool")]
    resource_pool: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    license: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ApplyRecommendationRequestType<'a> {
    key: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CancelRecommendationRequestType<'a> {
    key: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "ConfigureHCIRequestType", tag = "_typeName")]
struct ConfigureHciRequestType<'a> {
    #[serde(rename = "clusterSpec")]
    cluster_spec: &'a ClusterComputeResourceHciConfigSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hostInputs")]
    host_inputs: Option<&'a [ClusterComputeResourceHostConfigurationInput]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ClusterEnterMaintenanceModeRequestType<'a> {
    host: &'a [ManagedObjectReference],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    option: Option<&'a [Box<dyn crate::types::option_value_trait::OptionValueTrait>]>,
}
#[derive(serde::Serialize)]
#[serde(rename = "ExtendHCIRequestType", tag = "_typeName")]
struct ExtendHciRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hostInputs")]
    host_inputs: Option<&'a [ClusterComputeResourceHostConfigurationInput]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vSanConfigSpec")]
    v_san_config_spec: Option<&'a SddcBase>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct FindRulesForVmRequestType<'a> {
    vm: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct MoveHostIntoRequestType<'a> {
    host: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resourcePool")]
    resource_pool: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct MoveIntoRequestType<'a> {
    host: &'a [ManagedObjectReference],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PlaceVmRequestType<'a> {
    #[serde(rename = "placementSpec")]
    placement_spec: &'a PlacementSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RecommendHostsForVmRequestType<'a> {
    vm: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pool: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReconfigureClusterRequestType<'a> {
    spec: &'a ClusterConfigSpec,
    modify: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReconfigureComputeResourceRequestType<'a> {
    spec: &'a dyn crate::types::compute_resource_config_spec_trait::ComputeResourceConfigSpecTrait,
    modify: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RenameRequestType<'a> {
    #[serde(rename = "newName")]
    new_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetCryptoModeRequestType<'a> {
    #[serde(rename = "cryptoMode")]
    crypto_mode: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "setCustomValueRequestType", tag = "_typeName")]
struct SetCustomValueRequestType<'a> {
    key: &'a str,
    value: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "ValidateHCIConfigurationRequestType", tag = "_typeName")]
struct ValidateHciConfigurationRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hciConfigSpec")]
    hci_config_spec: Option<&'a ClusterComputeResourceHciConfigSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hosts: Option<&'a [ManagedObjectReference]>,
}
