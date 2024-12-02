use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::ManagedObjectReference;
use crate::types::MethodFaultTrait;
use crate::types::StorageDrsConfigSpec;
use crate::types::StorageIormConfigOption;
use crate::types::StorageIormConfigSpec;
use crate::types::StoragePerformanceSummary;
use crate::types::StoragePlacementResult;
use crate::types::StoragePlacementSpec;
/// This managed object type provides a way to configure resource usage for
/// storage resources.
pub struct StorageResourceManager {
    client: Arc<VimClient>,
    mo_id: String,
}
impl StorageResourceManager {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Changes configuration of storage I/O resource management for a given datastore.
    /// 
    /// The changes are applied to all the backing storage devices for the datastore.
    /// Currently we only support storage I/O resource management on VMFS volumes.
    /// In order to enable storage I/O resource management on a datstore, we require
    /// that all the hosts that are attached to the datastore support this feature.
    /// The privilege Datastore.ConfigIOManagement is required on the target
    /// datastore.
    /// 
    /// This method is only supported by vCenter server.
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// The datastore to be configured.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ### spec
    /// The configuration spec.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if called directly on a host or if called on a datastore
    /// that does not have VMFS Volume.
    /// 
    /// ***InvalidArgument***: if
    /// 1\. IORMConfigSpec.congestionThreshold is not within the
    /// desired range (5 to 100 milliseconds).
    /// 2\. IORMConfigSpec.congestionThresholdMode is not specified and
    /// IORMConfigSpec.congestionThreshold is specified. To set
    /// congestionThreshold, congestionThresholdMode should be set to
    /// manual
    /// 
    /// ***IORMNotSupportedHostOnDatastore***: if called on a datastore that is
    /// connected to a host that does not support storage I/O resource
    /// management.
    /// 
    /// ***InaccessibleDatastore***: if cannot access the datastore from any of the
    /// hosts.
    pub async fn configure_datastore_iorm_task(&self, datastore: &ManagedObjectReference, spec: &StorageIormConfigSpec) -> Result<ManagedObjectReference> {
        let input = ConfigureDatastoreIormRequestType {datastore, spec, };
        let path = format!("/StorageResourceManager/{moId}/ConfigureDatastoreIORM_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Query configuration options for storage I/O resource management.
    /// 
    /// ***Required privileges:*** Datastore.Config
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// \[in\] - The host VC will forward the query
    /// to. This parameter is ignored by host if this method is
    /// called on a host directly.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// configuration option object.
    pub async fn query_iorm_config_option(&self, host: &ManagedObjectReference) -> Result<StorageIormConfigOption> {
        let input = QueryIormConfigOptionRequestType {host, };
        let path = format!("/StorageResourceManager/{moId}/QueryIORMConfigOption", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Applies a recommendation from the recommendation list that is generated
    /// by SDRS initial placement invoked by RecommendDatastore method.
    /// 
    /// In the case of CreateVm and CloneVm a VirtualMachine is returned. Other
    /// workflows don't have a return value.
    /// 
    /// Requires Resource. ApplyRecommendation privilege on the storage pod.
    /// Additionally, depending on the workflow where this API is called from,
    /// it may require the privileges of invoking one of following APIs:
    /// - CreateVm *Folder.CreateVM_Task*
    /// - AddDisk *VirtualMachine.ReconfigVM_Task*
    /// - RelocateVm *VirtualMachine.RelocateVM_Task*
    /// - CloneVm *VirtualMachine.CloneVM_Task*
    ///   
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### key
    /// The key fields of the Recommendations that are applied.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    pub async fn apply_storage_drs_recommendation_task(&self, key: &[String]) -> Result<ManagedObjectReference> {
        let input = ApplyStorageDrsRecommendationRequestType {key, };
        let path = format!("/StorageResourceManager/{moId}/ApplyStorageDrsRecommendation_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Applies a recommendation from the recommendation list that is generated
    /// by SDRS load balancing activity.
    /// 
    /// Each recommendation can be applied
    /// only once.
    /// 
    /// Requires Resource.ApplyRecommendation privilege on the storage pod. And
    /// requires Resource.ColdMigrate privilege on the virtual machine(s) that
    /// are relocated. Additionally requires Resource.HotMigrate privilege if
    /// the virtual machine is powered on (for Storage VMotion). Also requires
    /// Datastore.AllocateSpace on any datastore the virtual machine or its disks
    /// are relocated to.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### pod
    /// The storage pod.
    /// 
    /// Refers instance of *StoragePod*.
    ///
    /// ### key
    /// The key field of the Recommendation.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    pub async fn apply_storage_drs_recommendation_to_pod_task(&self, pod: &ManagedObjectReference, key: &str) -> Result<ManagedObjectReference> {
        let input = ApplyStorageDrsRecommendationToPodRequestType {pod, key, };
        let path = format!("/StorageResourceManager/{moId}/ApplyStorageDrsRecommendationToPod_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Cancels a recommendation.
    /// 
    /// Currently only initial placement
    /// recommendations can be cancelled. Migration recommendations cannot.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### key
    /// The key field of the Recommendation.
    pub async fn cancel_storage_drs_recommendation(&self, key: &[String]) -> Result<()> {
        let input = CancelStorageDrsRecommendationRequestType {key, };
        let path = format!("/StorageResourceManager/{moId}/CancelStorageDrsRecommendation", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Change the storage DRS configuration for a pod *StoragePod*.
    ///
    /// ## Parameters:
    ///
    /// ### pod
    /// The storage pod.
    /// 
    /// ***Required privileges:*** StoragePod.Config
    /// 
    /// Refers instance of *StoragePod*.
    ///
    /// ### spec
    /// A set of storage Drs configuration changes to apply to the storage pod.
    /// The specification can be a complete set of changes or a partial
    /// set of changes, applied incrementally.
    ///
    /// ### modify
    /// Flag to specify whether the specification ("spec") should
    /// be applied incrementally. If "modify" is false and the
    /// operation succeeds, then the configuration of the storage pod
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
    pub async fn configure_storage_drs_for_pod_task(&self, pod: &ManagedObjectReference, spec: &StorageDrsConfigSpec, modify: bool) -> Result<ManagedObjectReference> {
        let input = ConfigureStorageDrsForPodRequestType {pod, spec, modify, };
        let path = format!("/StorageResourceManager/{moId}/ConfigureStorageDrsForPod_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Returns datastore summary performance statistics.
    /// 
    /// This is an experimental interface that is not intended for
    /// use in production code.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// Datastore for which summary statistics is requested.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Returns:
    ///
    /// Summary performance statistics for the datastore. The summary
    /// contains latency, throughput, and SIOC activity.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if input datastore cannot be found
    pub async fn query_datastore_performance_summary(&self, datastore: &ManagedObjectReference) -> Result<Option<Vec<StoragePerformanceSummary>>> {
        let input = QueryDatastorePerformanceSummaryRequestType {datastore, };
        let path = format!("/StorageResourceManager/{moId}/QueryDatastorePerformanceSummary", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// This method returns a *StoragePlacementResult* object.
    /// This API is intended to replace the following existing APIs for
    /// SDRS-enabled pods:
    /// CreateVm: StoragePlacementSpec::type == create =
    /// *Folder.CreateVM_Task*
    /// AddDisk: StoragePlacementSpec::type == reconfigure =
    /// *VirtualMachine.ReconfigVM_Task*
    /// RelocateVm: StoragePlacementSpec::type == relocate =
    /// *VirtualMachine.RelocateVM_Task*
    /// CloneVm: StoragePlacementSpec::type == clone =
    /// *VirtualMachine.CloneVM_Task*
    /// The PodSelectionSpec parameter in StoragePlacementSpec is required
    /// for all workflows. It specifies which SDRS-enabled pod the user
    /// has selected for the VM and/or for each disk.
    /// For CreateVm, RelocateVm and CloneVm, PodSelectionSpec.storagePod is
    /// the user selected SDRS pod for the VM, i.e., its system files.
    /// For all workflows, PodSelectionSpec.disk.storagePod is the
    /// user selected SDRS pod for the given disk. Note that a
    /// DiskLocator must be specified for each disk that the user
    /// requests to create, migrate or clone into an SDRS pod, even if it's
    /// the same pod as the VM or the user has manually selected a datastore
    /// within the pod. If the user has manually selected a datastore, the
    /// datastore must be specified in the workflow specific fields as
    /// described below.
    /// For CreateVm, AddDisk, the manually selected datastore
    /// must be specified in ConfigSpec.files or
    /// ConfigSpec.deviceChange.device.backing.datastore, the fields
    /// should will be unset if the user wants SDRS to recommend the datastore.
    /// For RelocateVm, the manually selected datastore must be specified in
    /// RelocateSpec.datastore or RelocateSpec.disk.datastore; the fields should
    /// be unset iff the user wants SDRS recommendations. For CloneVm, the
    /// manually selected datastore must be specified in
    /// CloneSpec.location.datastore or CloneSpec.location.disk\[\].datastore;
    /// the fields should be unset iff the user wants SDRS recommendations.
    /// The remaining expected input parameters in StoragePlacementSpec
    /// will be the same as those for the existing API as determined by
    /// StoragePlacementSpec::type. If a parameter is optional in the
    /// existing API, it will also be optional in the new API.
    /// - For CreateVm, the Folder, ConfigSpec, ResourcePool and HostSystem
    ///   parameters will be expected in StoragePlacementSpec. The disks
    ///   to be created can be determined by ConfigSpec -&gt;
    ///   VirtualDeviceSpec\[\] (deviceChange) -&gt; VirtualDevice (device) -&gt;
    ///   VirtualDisk (subclass).
    /// - For AddDisk, the VirtualMachine and ConfigSpec parameters will
    ///   be expected. The use of the ConfigSpec for determining the disks
    ///   to add will be the same as that in CreateVm.
    /// - For ExpandDisk, the VirtualMachine and ConfigSpec parameters will
    ///   be expected. The use of the ConfigSpec for determining the disks
    ///   to be expanded will be the same as the disks IDs of existing VM disks.
    /// - For RelocateVm, the VirtualMachine, RelocateSpec and
    ///   MovePriority parameters will be expected.
    /// - For CloneVm, the VirtualMachine, CloneSpec, Folder and cloneName
    ///   parameters will be expected.
    ///   
    /// SDRS takes into account constraints such as space usages,
    /// (anti-) affinity rules, datastore maintenance mode, etc. when
    /// making placement recommendations. Given that the constraints are
    /// satisfied, SDRS tries to balance space usages and I/O loads in
    /// the placement.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### storage_spec
    /// -
    pub async fn recommend_datastores(&self, storage_spec: &StoragePlacementSpec) -> Result<StoragePlacementResult> {
        let input = RecommendDatastoresRequestType {storage_spec, };
        let path = format!("/StorageResourceManager/{moId}/RecommendDatastores", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Make Storage DRS invoke again on the specified pod *StoragePod*
    /// and return a new list of recommendations.
    /// 
    /// Concurrent "refresh" requests
    /// may be combined together and trigger only one Storage DRS invocation.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### pod
    /// The storage pod.
    /// The recommendations generated is stored at
    /// *PodStorageDrsEntry.recommendation*.
    /// 
    /// Refers instance of *StoragePod*.
    pub async fn refresh_storage_drs_recommendation(&self, pod: &ManagedObjectReference) -> Result<()> {
        let input = RefreshStorageDrsRecommendationRequestType {pod, };
        let path = format!("/StorageResourceManager/{moId}/RefreshStorageDrsRecommendation", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Invoke Storage DRS on a specific pod *StoragePod*
    /// and return a new list of recommendations.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### pod
    /// The storage pod.
    /// The recommendations generated is stored at
    /// *PodStorageDrsEntry.recommendation*.
    /// 
    /// Refers instance of *StoragePod*.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn refresh_storage_drs_recommendations_for_pod_task(&self, pod: &ManagedObjectReference) -> Result<ManagedObjectReference> {
        let input = RefreshStorageDrsRecommendationsForPodRequestType {pod, };
        let path = format!("/StorageResourceManager/{moId}/RefreshStorageDrsRecommendationsForPod_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Validate the new storage DRS configuration for a pod
    /// *StoragePod*.
    /// 
    /// If validation fails, it will return with InvalidArgument fault.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### pod
    /// The storage pod.
    /// 
    /// Refers instance of *StoragePod*.
    ///
    /// ### spec
    /// A set of storage Drs configuration changes to apply to
    /// the storage pod.
    pub async fn validate_storage_pod_config(&self, pod: &ManagedObjectReference, spec: &StorageDrsConfigSpec) -> Result<Box<dyn MethodFaultTrait>> {
        let input = ValidateStoragePodConfigRequestType {pod, spec, };
        let path = format!("/StorageResourceManager/{moId}/ValidateStoragePodConfig", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(rename = "ConfigureDatastoreIORMRequestType", tag = "_typeName")]
struct ConfigureDatastoreIormRequestType<'a> {
    datastore: &'a ManagedObjectReference,
    spec: &'a StorageIormConfigSpec,
}
#[derive(serde::Serialize)]
#[serde(rename = "QueryIORMConfigOptionRequestType", tag = "_typeName")]
struct QueryIormConfigOptionRequestType<'a> {
    host: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ApplyStorageDrsRecommendationRequestType<'a> {
    key: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ApplyStorageDrsRecommendationToPodRequestType<'a> {
    pod: &'a ManagedObjectReference,
    key: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CancelStorageDrsRecommendationRequestType<'a> {
    key: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ConfigureStorageDrsForPodRequestType<'a> {
    pod: &'a ManagedObjectReference,
    spec: &'a StorageDrsConfigSpec,
    modify: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryDatastorePerformanceSummaryRequestType<'a> {
    datastore: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RecommendDatastoresRequestType<'a> {
    #[serde(rename = "storageSpec")]
    storage_spec: &'a StoragePlacementSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RefreshStorageDrsRecommendationRequestType<'a> {
    pod: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RefreshStorageDrsRecommendationsForPodRequestType<'a> {
    pod: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ValidateStoragePodConfigRequestType<'a> {
    pod: &'a ManagedObjectReference,
    spec: &'a StorageDrsConfigSpec,
}
