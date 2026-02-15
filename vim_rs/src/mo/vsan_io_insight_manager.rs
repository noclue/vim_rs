use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object is designed to provide service interfaces to manage
/// ioinsight(s) for VMs virtual disks I/O performance metrics monitor and
/// collection on the target hosts.
/// 
/// It can be accessed through MOID of 'vsan-cluster-ioinsight-manager' via vSAN
/// service on vCenter, or accessed through MOID of 'vsan-ioinsight-manager' on
/// ESXi host.
#[derive(Clone)]
pub struct VsanIoInsightManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanIoInsightManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Delete information of a completed ioinsight instance specified by
    /// the passed-in runName.
    ///
    /// ## Parameters:
    ///
    /// ### run_name
    /// Run name of the ioinsight instance.
    ///
    /// ### cluster
    /// The target cluster where the ioinsight instance belongs to.
    /// This parameter is ignored while the API is called against
    /// host.
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If any argument passed to the function is not
    /// specified correctly.
    /// 
    /// ***NotFound***: If the specified ioinsight instance is not found.
    /// 
    /// ***NotSupported***: If the API is not supported by the called host.
    /// 
    /// ***VsanFault***: If any other unexpected fault is encountered.
    pub async fn delete_io_insight_instance(&self, run_name: &str, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<()> {
        let input = DeleteIoInsightInstanceRequestType {run_name, cluster, };
        let path = format!("/vsan/VsanIoInsightManager/{moId}/DeleteIoInsightInstance", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Retrieve all ioinsight instances
    /// *VsanIoInsightInstance* according to the passed in
    /// parameters.
    ///
    /// ## Parameters:
    ///
    /// ### query_spec
    /// Describe specifications for the query operation.
    ///
    /// ### cluster
    /// The target cluster where the ioinsight instances belong to.
    /// This parameter is ignored while the API is called against
    /// host.
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If any argument passed to the function is not
    /// specified correctly.
    /// 
    /// ***NotSupported***: If the API is not supported by the called host.
    /// 
    /// ***VsanFault***: If any other unexpected fault is encountered.
    /// 
    /// ***NotFound***: If the stats primary node is not found in target cluster.
    pub async fn query_io_insight_instances(&self, query_spec: &crate::types::structs::VsanIoInsightInstanceQuerySpec, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<crate::types::structs::VsanIoInsightInstance>>> {
        let input = QueryIoInsightInstancesRequestType {query_spec, cluster, };
        let path = format!("/vsan/VsanIoInsightManager/{moId}/QueryIoInsightInstances", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::VsanIoInsightInstance>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Rename an ioinsight instance which is completed already
    /// (see *VsanIoInsightInstanceState_enum*).
    ///
    /// ## Parameters:
    ///
    /// ### old_run_name
    /// The current run name of an completed ioinsight instance.
    ///
    /// ### new_run_name
    /// New run name for the completed ioinsight instance.
    ///
    /// ### cluster
    /// The target cluster where the ioinsight instance belongs to.
    /// This parameter is ignored while the API is called against
    /// host.
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If any argument passed to the function is not
    /// specified correctly.
    /// 
    /// ***NotFound***: If the specified ioinsight instance is not found.
    /// 
    /// ***NotSupported***: If the API is not supported by the called host.
    /// 
    /// ***VsanFault***: If any other unexpected fault is encountered.
    pub async fn rename_io_insight_instance(&self, old_run_name: &str, new_run_name: &str, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<()> {
        let input = RenameIoInsightInstanceRequestType {old_run_name, new_run_name, cluster, };
        let path = format!("/vsan/VsanIoInsightManager/{moId}/RenameIoInsightInstance", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Start ioinsight tool(s) running on the whole vSAN cluster or on the
    /// specified ESXi host(s) for VMs virtual disks I/O performance metrics
    /// monitor.
    /// 
    /// Given the parameters passed-in, ioinsight tool(s) will be started
    /// on the ESXi host(s) to monitor and collect VMDKs I/O performance metrics of
    /// the specified VMs.
    /// Once the specified run duration has expired, ioinsight will stop running
    /// automatically. The collected performance metrics will be persisted in vSAN
    /// datastore.
    /// This API returns a task which is running background and performing actually
    /// ioinsight start operation on each host. Please wait for the task to be
    /// completed, and retrieve the final result - one or more
    /// *VsanHostIoInsightInfo* from the corresponding task
    /// information.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The cluster to which the ESXi host(s) belong.
    /// This parameter is ignored while the API is called against
    /// host.
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### run_name
    /// Caller can specify a meaningful name for ioinsight one time
    /// execution, and use that name for ioinsight metrics query in
    /// the future.
    /// This parameter can be ignored while the API is called against
    /// host.
    ///
    /// ### duration_sec
    /// Duration in seconds for ioinsight execution.
    /// Once duration has expired ioinsight will stop on it's
    /// own. The valid range of duration is between from 60 second
    /// to 86400 seconds (24 hours).
    /// If this parameter is not provided, the API will try to
    /// append targetVMs to the ioinsight(s) running on
    /// targetHosts as new monitor targets.
    ///
    /// ### target_hosts
    /// One or multiple ESXi hosts on which ioinsight is
    /// installed and going to be started.
    /// If this parameter is not provided, all hosts of the
    /// cluster will be treated as targets, and this case is only
    /// supported while being invoked against vCenter.
    /// If the API is called against host, this parameter is must
    /// to have and should be exactly same as the target host.
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ### target_v_ms
    /// One or multiple target VMs will be monitored by ioinsight.
    /// If this parameter is not provided, all VMs on the host
    /// will be treated as targets.
    /// 
    /// Refers instances of *VirtualMachine*.
    ///
    /// ## Returns:
    ///
    /// Task which is performing actually ioinsight start operation in the
    /// background.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If any argument passed to the API is not specified
    /// correctly.
    /// 
    /// ***NotFound***: if no ESXi host could be contacted to perform the operation
    /// when this method is called against vCenter, or ioinsight
    /// cannot be found on host.
    /// 
    /// ***VsanFault***: If any other unexpected failure happened during starting
    /// ioinsight.
    pub async fn start_io_insight(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, run_name: Option<&str>, duration_sec: Option<i64>, target_hosts: Option<&[crate::types::structs::ManagedObjectReference]>, target_v_ms: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = StartIoInsightRequestType {cluster, run_name, duration_sec, target_hosts, target_v_ms, };
        let path = format!("/vsan/VsanIoInsightManager/{moId}/StartIoInsight", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Stop ioinsight tool(s) running on the ESXi host(s) by given parameters
    /// passed-in.
    /// 
    /// The collected VMDKs I/O performance metrics will be persisted in vSAN
    /// datastore for further query by:
    /// *VsanPerformanceManager.VsanPerfQueryPerf*.
    /// This API returns a task which is running background and performing actually
    /// ioinsight stop operation on each host. Please wait for the task to be
    /// completed, and retrieve the final result - one or more
    /// *VsanHostIoInsightInfo* from the corresponding task
    /// information.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The cluster to which the target ESXi host(s) belong. This
    /// parameter is ignored while the API is called against host.
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### run_name
    /// The name of running ioinsight to be stopped. This parameter
    /// can be ingnored while being called against host.
    ///
    /// ### hosts_io_insight_infos
    /// One or multiple host ioinsight(s) information, which
    /// indicate the target ioinsight(s) to be stopped or the
    /// VMs to be unmonitored.
    /// If *VsanIoInsightInfo.monitoredVMs*
    /// are specified by the parameter, the API will try to
    /// unmonitor the VMs from running ioinsight(s), otherwise
    /// it will entirely stop the specified ioinsight(s) if no
    /// valid runName is provided.
    /// If neither this parameter nor runName is provided,
    /// the API will try to stop ioinsight(s) on each host in
    /// the cluster, and this case is only supported while
    /// being invoked against vCenter. If the API is called
    /// against host, this parameter is must to have and
    /// *VsanHostIoInsightInfo.host* should be
    /// exactly same as the target host.
    ///
    /// ## Returns:
    ///
    /// Task which is performing actually ioinsight stop operation in the
    /// background on each host.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If any argument passed to the function is not
    /// specified correctly.
    /// 
    /// ***NotFound***: If no ESXi host could be contacted to perform the operation
    /// when this method is called against vCenter.
    /// 
    /// ***VsanFault***: If the caller doesn't have the required privilege, or the
    /// cluster has no hosts.
    pub async fn stop_io_insight(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, run_name: Option<&str>, hosts_io_insight_infos: Option<&[crate::types::structs::VsanHostIoInsightInfo]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = StopIoInsightRequestType {cluster, run_name, hosts_io_insight_infos, };
        let path = format!("/vsan/VsanIoInsightManager/{moId}/StopIoInsight", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct DeleteIoInsightInstanceRequestType<'a> {
    run_name: &'a str,
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for DeleteIoInsightInstanceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DeleteIoInsightInstanceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DeleteIoInsightInstanceRequestTypeSer<'b, 'a> {
    data: &'b DeleteIoInsightInstanceRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for DeleteIoInsightInstanceRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DeleteIoInsightInstanceRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("runName"), &self.data.run_name as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryIoInsightInstancesRequestType<'a> {
    query_spec: &'a crate::types::structs::VsanIoInsightInstanceQuerySpec,
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for QueryIoInsightInstancesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryIoInsightInstancesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryIoInsightInstancesRequestTypeSer<'b, 'a> {
    data: &'b QueryIoInsightInstancesRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for QueryIoInsightInstancesRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryIoInsightInstancesRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("querySpec"), &self.data.query_spec as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RenameIoInsightInstanceRequestType<'a> {
    old_run_name: &'a str,
    new_run_name: &'a str,
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for RenameIoInsightInstanceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RenameIoInsightInstanceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RenameIoInsightInstanceRequestTypeSer<'b, 'a> {
    data: &'b RenameIoInsightInstanceRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for RenameIoInsightInstanceRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RenameIoInsightInstanceRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("oldRunName"), &self.data.old_run_name as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("newRunName"), &self.data.new_run_name as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct StartIoInsightRequestType<'a> {
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    run_name: Option<&'a str>,
    duration_sec: Option<i64>,
    target_hosts: Option<&'a [crate::types::structs::ManagedObjectReference]>,
    target_v_ms: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for StartIoInsightRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(StartIoInsightRequestTypeSer { data: self, seq: 0 }))
    }
}

struct StartIoInsightRequestTypeSer<'b, 'a> {
    data: &'b StartIoInsightRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for StartIoInsightRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"StartIoInsightRequestType")),
                1 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.run_name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("runName"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.duration_sec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("durationSec"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.target_hosts else { continue; };
                    return Some((std::borrow::Cow::Borrowed("targetHosts"), val as &dyn miniserde::Serialize));
                }
                5 => {
                    let Some(ref val) = self.data.target_v_ms else { continue; };
                    return Some((std::borrow::Cow::Borrowed("targetVMs"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct StopIoInsightRequestType<'a> {
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    run_name: Option<&'a str>,
    hosts_io_insight_infos: Option<&'a [crate::types::structs::VsanHostIoInsightInfo]>,
}

impl<'a> miniserde::Serialize for StopIoInsightRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(StopIoInsightRequestTypeSer { data: self, seq: 0 }))
    }
}

struct StopIoInsightRequestTypeSer<'b, 'a> {
    data: &'b StopIoInsightRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for StopIoInsightRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"StopIoInsightRequestType")),
                1 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.run_name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("runName"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.hosts_io_insight_infos else { continue; };
                    return Some((std::borrow::Cow::Borrowed("hostsIoInsightInfos"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
