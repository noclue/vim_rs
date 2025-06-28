use std::sync::Arc;
use crate::core::client::{Client, Result};
/// This managed object provides the diagnostics service that operates at cluster
/// level.
/// 
/// It runs the periodical diagnostics on the vSAN related statistics, as
/// well as the on-demand diagnostics operation, e.g., the diagnostics of the
/// IOs for a certain virtual machine.
/// The ManagedEntity can be accessed with MOID of
/// 'vsan-cluster-diagnostics-system' through vSAN service at at vCenter side.
#[derive(Clone)]
pub struct VsanDiagnosticsSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl VsanDiagnosticsSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Schedule a set of IO trip analyzer recurrences.
    /// 
    /// Currently only 1 recurrence supported for each vSAN cluster.
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The cluster on which the IO trip analyzer scheduler is
    /// configured.
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### recurrences
    /// The scheduler recurrences configurations.
    ///
    /// ## Returns:
    ///
    /// The list of recurrences that are configured successfully. If the
    /// name of the recurrence is not given, it will fill the name with
    /// automatically generated name and return it in the list.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the recurrence(s) setting(s) are not valid.
    /// 
    /// ***AlreadyExists***: if the recurrence(s) name(s) already exists.
    /// 
    /// ***VsanFault***: if internal vSAN error hit.
    pub async fn create_io_trip_analyzer_recurrences(&self, cluster: &crate::types::structs::ManagedObjectReference, recurrences: &[crate::types::structs::VsanIoTripAnalyzerRecurrence]) -> Result<Vec<crate::types::structs::VsanIoTripAnalyzerRecurrence>> {
        let input = CreateIoTripAnalyzerRecurrencesRequestType {cluster, recurrences, };
        let path = format!("/vsan/VsanDiagnosticsSystem/{moId}/CreateIOTripAnalyzerRecurrences", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Edit the IO trip analyzer recurrences with the given configuration.
    /// 
    /// The name of the recurrence is used to find the recurrence to be updated. It
    /// must be provided when editing.
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The cluster on which the IO trip analyzer scheduler is
    /// configured.
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### recurrences
    /// The updated recurrence(s) configuration.
    ///
    /// ## Returns:
    ///
    /// The list of recurrence(s) that are edited successfully.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the recurrence(s) setting(s) are not valid.
    /// 
    /// ***NotFound***: if the recurrence(s) name(s) not found.
    /// 
    /// ***VsanFault***: if internal vSAN error hit.
    pub async fn edit_io_trip_analyzer_recurrences(&self, cluster: &crate::types::structs::ManagedObjectReference, recurrences: &[crate::types::structs::VsanIoTripAnalyzerRecurrence]) -> Result<Vec<crate::types::structs::VsanIoTripAnalyzerRecurrence>> {
        let input = EditIoTripAnalyzerRecurrencesRequestType {cluster, recurrences, };
        let path = format!("/vsan/VsanDiagnosticsSystem/{moId}/EditIOTripAnalyzerRecurrences", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Get the IO trip analyzer scheduler configurations for the given cluster.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The cluster on which the IO trip analyzer scheduler is
    /// configured.
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ## Returns:
    ///
    /// The IO trip analyzer scheduler configuration.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: if internal vSAN error hit.
    pub async fn get_io_trip_analyzer_scheduler_config(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<Option<crate::types::structs::VsanIoTripAnalyzerConfig>> {
        let input = GetIoTripAnalyzerSchedulerConfigRequestType {cluster, };
        let path = format!("/vsan/VsanDiagnosticsSystem/{moId}/GetIOTripAnalyzerSchedulerConfig", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Get the threshold.
    /// 
    /// If "entityType" is not set, all threshold settings will be returned;
    /// If "entityType" is set but "metric" is not set, threshold settings for the
    /// specified "entityType" will be returned; If threshold settings doesn't exist
    /// for "entityType", empty result (\[\]) will be returned;
    /// If both "entityType" and "metric" are set, threshold settings for the
    /// specified "entityType" and "metric" will be returned; If threshold settings
    /// doesn't exist for either "entityType" or "metric", empty result (\[\]) will be
    /// returned.
    /// If "metric" is set but "entityType" is not set, "metric" will be ignored and
    /// all threshold settings will be returned.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The cluster where the threshold is to get from
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### entity_type
    /// Get threshold only for the specified entity type.
    ///
    /// ### metric
    /// Get threshold only for the specified metric, "entityType" must
    /// also be set if "metric" is set.
    ///
    /// ## Returns:
    ///
    /// vim.cluster.VsanDiagnosticsThreshold\[\]
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_get_thresholds(&self, cluster: &crate::types::structs::ManagedObjectReference, entity_type: Option<&str>, metric: Option<&str>) -> Result<Option<Vec<crate::types::structs::VsanDiagnosticsThreshold>>> {
        let input = VsanGetThresholdsRequestType {cluster, entity_type, metric, };
        let path = format!("/vsan/VsanDiagnosticsSystem/{moId}/VsanGetThresholds", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Query the completed diagnostics instances by the given query spec.
    ///
    /// ## Parameters:
    ///
    /// ### query_spec
    /// The spec for instance query.
    ///
    /// ### cluster
    /// The cluster where the diagnostics is performed, ignored when
    /// calling against ESXi hosts.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// The list of the completed diagnostics instances.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the query spec is not valid.
    /// 
    /// ***VsanFault***: if the caller doesn't have the required privilege or if
    /// the vSAN performance service is disabled.
    pub async fn query_io_diagnostics_instances(&self, query_spec: &crate::types::structs::VsanIoDiagnosticsInstanceQuerySpec, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<crate::types::structs::VsanIoDiagnosticsInstance>>> {
        let input = QueryIoDiagnosticsInstancesRequestType {query_spec, cluster, };
        let path = format!("/vsan/VsanDiagnosticsSystem/{moId}/QueryIODiagnosticsInstances", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Query the IO diagnostics stats according to the given diagnostics instance
    /// name.
    ///
    /// ## Parameters:
    ///
    /// ### instance_name
    /// The completed diagnostics instance name.
    ///
    /// ### cluster
    /// The cluster where the diagnostics is performed.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// The diagnostics stats.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the given diagnostics instance name represent a
    /// running instance.
    /// 
    /// ***VsanFault***: if the caller doesn't have the required privilege or if
    /// the vSAN performance service is disabled.
    pub async fn query_io_diagnostics_stats(&self, instance_name: &str, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<crate::types::structs::VsanIoDiagnosticsTargetStats>>> {
        let input = QueryIoDiagnosticsStatsRequestType {instance_name, cluster, };
        let path = format!("/vsan/VsanDiagnosticsSystem/{moId}/QueryIODiagnosticsStats", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Get the latest network events that triggered network alarms.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The cluster where the network diagnostics is performed.
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### host
    /// Get only for the specified host.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// vim.cluster.VsanNetworkDiagnostics\[\]
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_network_diagnostics(&self, cluster: &crate::types::structs::ManagedObjectReference, host: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<crate::types::structs::VsanNetworkDiagnostics>>> {
        let input = VsanQueryNetworkDiagnosticsRequestType {cluster, host, };
        let path = format!("/vsan/VsanDiagnosticsSystem/{moId}/VsanQueryNetworkDiagnostics", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Remove the IO trip analyzer recurrence(s) with the given name(s).
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The cluster on which the IO trip analyzer scheduler is
    /// configured.
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### names
    /// The name(s) of the IO trip analyzer recurrence(s) to be
    /// removed.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the recurrence(s) name(s) not found.
    /// 
    /// ***VsanFault***: if internal vSAN error hit.
    pub async fn remove_io_trip_analyzer_recurrences(&self, cluster: &crate::types::structs::ManagedObjectReference, names: &[String]) -> Result<()> {
        let input = RemoveIoTripAnalyzerRecurrencesRequestType {cluster, names, };
        let path = format!("/vsan/VsanDiagnosticsSystem/{moId}/RemoveIOTripAnalyzerRecurrences", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Set the threshold.
    /// 
    /// The "entityType" and "metric" of the new threshold must be in current
    /// supported list (see *VsanDiagnosticsThreshold.entityType*),
    /// or fault "vmodl.fault.InvalidArgument" will be raised.
    /// If "yellow" or "red" value is "0" or not set in the new threshold, the
    /// existing "yellow" or "red" value of current threshold setting will keep
    /// unchanged.
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The cluster where the threshold is to set to
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### thresholds
    /// The new threshold value to set
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_set_thresholds(&self, cluster: &crate::types::structs::ManagedObjectReference, thresholds: Option<&[crate::types::structs::VsanDiagnosticsThreshold]>) -> Result<()> {
        let input = VsanSetThresholdsRequestType {cluster, thresholds, };
        let path = format!("/vsan/VsanDiagnosticsSystem/{moId}/VsanSetThresholds", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Set the policy of the vSAN namespace object that hold the vSAN trace files.
    /// 
    /// The cluster parameter is ignored if called on ESXi.
    /// Profile can be:
    /// - VirtualMachineDefinedProfileSpec where profileId is an empty string and
    ///   instead the profileData is set for extensionKey 'com.vmware.vim.sps'. In this
    ///   case the objectData field can be either the vSAN expression format, or a SPBM
    ///   XML string.
    ///   
    ///   
    /// If no profile is supplied, a default profile with hostFailuresToTolerate
    /// setting will be used.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// vSAN cluster. Ignored if called against host.
    /// 
    /// ***Required privileges:*** Global.Settings
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### trace_object_uuid
    /// -
    ///
    /// ### profile
    /// See above description for all possible options.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if no ESXi host could be contacted to perform the operation
    /// when called against vCenter.
    /// 
    /// ***VsanFault***: if the caller doesn't have the required privilege
    pub async fn vsan_set_trace_object_policy(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, trace_object_uuid: &str, profile: Option<&dyn crate::types::traits::VirtualMachineProfileSpecTrait>) -> Result<bool> {
        let input = VsanSetTraceObjectPolicyRequestType {cluster, trace_object_uuid, profile, };
        let path = format!("/vsan/VsanDiagnosticsSystem/{moId}/VsanSetTraceObjectPolicy", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Start IO diagnostics task against the given targets running on vSAN datastore.
    /// 
    /// It's not supported to run multiple diagnostics tasks at the same time. You
    /// need either cancel the running task or wait until it completes before
    /// starting a new diagnostics task.
    ///
    /// ## Parameters:
    ///
    /// ### targets
    /// The targets to run the diagnostics.
    ///
    /// ### cluster
    /// The cluster where the targets belong to, ignored when calling
    /// against ESXi hosts.
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### duration
    /// The duration time in seconds to run diagnostics. The valid range
    /// is \[300, 3600\], default is 300.
    ///
    /// ## Returns:
    ///
    /// The task that runs the diagnostics.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the targets are not valid.
    /// 
    /// ***NotSupported***: if there's already a running diagnostics task.
    /// 
    /// ***VsanFault***: if the caller doesn't have the required privilege or
    /// if the pre-check tests failed.
    pub async fn start_io_diagnostics_task(&self, targets: &[crate::types::structs::VsanIoDiagnosticsTarget], cluster: Option<&crate::types::structs::ManagedObjectReference>, duration: Option<i64>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = StartIoDiagnosticsTaskRequestType {targets, cluster, duration, };
        let path = format!("/vsan/VsanDiagnosticsSystem/{moId}/StartIODiagnosticsTask", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(rename = "CreateIOTripAnalyzerRecurrencesRequestType", tag = "_typeName")]
struct CreateIoTripAnalyzerRecurrencesRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    recurrences: &'a [crate::types::structs::VsanIoTripAnalyzerRecurrence],
}
#[derive(serde::Serialize)]
#[serde(rename = "EditIOTripAnalyzerRecurrencesRequestType", tag = "_typeName")]
struct EditIoTripAnalyzerRecurrencesRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    recurrences: &'a [crate::types::structs::VsanIoTripAnalyzerRecurrence],
}
#[derive(serde::Serialize)]
#[serde(rename = "GetIOTripAnalyzerSchedulerConfigRequestType", tag = "_typeName")]
struct GetIoTripAnalyzerSchedulerConfigRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanGetThresholdsRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "entityType")]
    entity_type: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    metric: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(rename = "QueryIODiagnosticsInstancesRequestType", tag = "_typeName")]
struct QueryIoDiagnosticsInstancesRequestType<'a> {
    #[serde(rename = "querySpec")]
    query_spec: &'a crate::types::structs::VsanIoDiagnosticsInstanceQuerySpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(rename = "QueryIODiagnosticsStatsRequestType", tag = "_typeName")]
struct QueryIoDiagnosticsStatsRequestType<'a> {
    #[serde(rename = "instanceName")]
    instance_name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryNetworkDiagnosticsRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(rename = "RemoveIOTripAnalyzerRecurrencesRequestType", tag = "_typeName")]
struct RemoveIoTripAnalyzerRecurrencesRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    names: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanSetThresholdsRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    thresholds: Option<&'a [crate::types::structs::VsanDiagnosticsThreshold]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanSetTraceObjectPolicyRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    #[serde(rename = "traceObjectUuid")]
    trace_object_uuid: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile: Option<&'a dyn crate::types::traits::VirtualMachineProfileSpecTrait>,
}
#[derive(serde::Serialize)]
#[serde(rename = "StartIODiagnosticsTaskRequestType", tag = "_typeName")]
struct StartIoDiagnosticsTaskRequestType<'a> {
    targets: &'a [crate::types::structs::VsanIoDiagnosticsTarget],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    duration: Option<i64>,
}
