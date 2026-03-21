use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object type provides interfaces to perform resource check
/// and query for various operations, e.g.
/// 
/// host enter maintenance mode,
/// disk upgrade.
/// It can be accessed through MOID of 'vsan-cluster-resource-check-system',
/// via vSAN service on vCenter at cluster level, or accessed through MOID
/// of 'vsan-resource-check-system' on ESXi host.
#[derive(Clone)]
pub struct VsanResourceCheckSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanResourceCheckSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// ***Required privileges:*** System.Read
    pub async fn vsan_host_cancel_resource_check(&self) -> Result<bool> {
        let bytes = self.client.invoke("vsan", "VsanResourceCheckSystem", &self.mo_id, "VsanHostCancelResourceCheck", None).await?;
        let result: bool = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Retrieve the status of the latest resource check.
    /// 
    /// If a resource check task is running, its status will be returned.
    /// Otherwise the status of the last resource check will be returned.
    ///
    /// ## Parameters:
    ///
    /// ### resource_check_spec
    /// The specification of the resource check to be queried.
    /// If it is not specified, *VsanResourceCheckStatus*
    /// will still be returned with *VsanResourceCheckStatus.task*
    /// and/or *VsanResourceCheckStatus.parentTask* information
    /// if such task is running in the specified cluster. However,
    /// *VsanResourceCheckStatus.result* will not be populated in this case.
    ///
    /// ### cluster
    /// The cluster to fetch the resource check status.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// The status of the specified resource check. The resource check may
    /// have not started yet, or be currently running, or have completed
    /// already.
    /// If a resource check has completed successfully, detailed resource
    /// check result will also be included.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***InvalidArgument***: Exception when the given operation is not
    /// supported, some parameter is missing or incorrect
    /// in the spec. For example, for host enter
    /// maintenance mode operation, host vSAN UUID or
    /// maintenanceSpec is not provided, or data evacuation
    /// mode is not supported.
    /// 
    /// ***VsanFault***: If any error happens when retrieving
    /// the resource check status.
    pub async fn vsan_get_resource_check_status(&self, resource_check_spec: Option<&crate::types::structs::VsanResourceCheckSpec>, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::VsanResourceCheckStatus> {
        let input = VsanGetResourceCheckStatusRequestType {resource_check_spec, cluster, };
        let bytes = self.client.invoke("vsan", "VsanResourceCheckSystem", &self.mo_id, "VsanGetResourceCheckStatus", Some(&input)).await?;
        let result: crate::types::structs::VsanResourceCheckStatus = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Perform a resource check for given spec.
    /// 
    /// Given the operation type and parameter(s) for the operation, the resource
    /// check will run a point-in-time simulation based on the cluster state at
    /// the time of the resource check request. It will check whether current vSAN
    /// cluster has enough resource or any issue to perform the operation.
    /// For example, given the host enter maintenance mode operation, the host
    /// vSAN UUID and data evacuation mode, it will check whether there is enough
    /// fault domains, disk capacity, etc. or any inaccessible object which can
    /// prevent the host from entering maintenance mode.
    /// Only one resource check should be run in a vSAN cluster at a time.
    ///
    /// ## Parameters:
    ///
    /// ### resource_check_spec
    /// The specification of the resource check.
    ///
    /// ### cluster
    /// The cluster to run the resource check.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// A task to monitor resource check progress. Results can be obtained
    /// with the API *VsanResourceCheckSystem.VsanGetResourceCheckStatus*
    /// after the resource check finishes.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***InvalidArgument***: Exception when the given operation is not
    /// supported, some parameter is missing or incorrect
    /// in the spec. For example, for host enter
    /// maintenance mode operation, host vSAN UUID or
    /// maintenanceSpec is not provided, or data evacuation
    /// mode is not supported.
    /// 
    /// ***InvalidState***: Exception when it is not qualified to run the resource
    /// check. For example, for host enter maintenance mode
    /// operation, if the given host is already in maintenance
    /// mode.
    /// 
    /// ***VsanFault***: If any error happens during the resource check.
    /// 
    /// ***NotFound***: Exception when the given entity in the spec cannot be
    /// found.
    /// 
    /// ***NotSupported***: Exception if the resource check is not supported
    /// on the given entity. For example, it's not supported
    /// to run disk data evacuation resource check on an
    /// unmounted disk/disk-group.
    pub async fn vsan_perform_resource_check(&self, resource_check_spec: &crate::types::structs::VsanResourceCheckSpec, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanPerformResourceCheckRequestType {resource_check_spec, cluster, };
        let bytes = self.client.invoke("vsan", "VsanResourceCheckSystem", &self.mo_id, "VsanPerformResourceCheck", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### resource_check_spec
    /// -
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_host_perform_resource_check(&self, resource_check_spec: &crate::types::structs::VsanResourceCheckSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanHostPerformResourceCheckRequestType {resource_check_spec, };
        let bytes = self.client.invoke("vsan", "VsanResourceCheckSystem", &self.mo_id, "VsanHostPerformResourceCheck", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct VsanGetResourceCheckStatusRequestType<'a> {
    resource_check_spec: Option<&'a crate::types::structs::VsanResourceCheckSpec>,
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for VsanGetResourceCheckStatusRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanGetResourceCheckStatusRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanGetResourceCheckStatusRequestTypeSer<'b, 'a> {
    data: &'b VsanGetResourceCheckStatusRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanGetResourceCheckStatusRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanGetResourceCheckStatusRequestType")),
                1 => {
                    let Some(ref val) = self.data.resource_check_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("resourceCheckSpec"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanPerformResourceCheckRequestType<'a> {
    resource_check_spec: &'a crate::types::structs::VsanResourceCheckSpec,
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for VsanPerformResourceCheckRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanPerformResourceCheckRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanPerformResourceCheckRequestTypeSer<'b, 'a> {
    data: &'b VsanPerformResourceCheckRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanPerformResourceCheckRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanPerformResourceCheckRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("resourceCheckSpec"), &self.data.resource_check_spec as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanHostPerformResourceCheckRequestType<'a> {
    resource_check_spec: &'a crate::types::structs::VsanResourceCheckSpec,
}

impl<'a> miniserde::Serialize for VsanHostPerformResourceCheckRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanHostPerformResourceCheckRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanHostPerformResourceCheckRequestTypeSer<'b, 'a> {
    data: &'b VsanHostPerformResourceCheckRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanHostPerformResourceCheckRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanHostPerformResourceCheckRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("resourceCheckSpec"), &self.data.resource_check_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
