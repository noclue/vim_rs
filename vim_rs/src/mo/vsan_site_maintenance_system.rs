use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object type provides the vSAN fault domain maintenance operations.
/// 
/// Fault
/// domain maintenance is a vSAN specific operation that allows all hosts within a fault domain
/// to enter maintenance mode while ensuring object consistency. This managed
/// object provides the functionalities to run precheck, enter maintenance mode
/// and exit maintenance mode.
/// It can be accessed through MOID of 'vsan-cluster-site-maintenance-system',
/// through vSAN service on vCenter at cluster level.
#[derive(Clone)]
pub struct VsanSiteMaintenanceSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanSiteMaintenanceSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Put all hosts in a fault domain into maintenance mode.
    /// 
    /// This operation puts the hosts on this fault domain into the maintenance mode, and
    /// guarantees the consistencies for all objects within this fault domain. A precheck
    /// will be performed as part of this fault domain maintenance workflow. Upon success,
    /// all VMs in this fault domain will be powered off or migrated to the other fault domain.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### fault_domain_name
    /// The fault domain name to enter maintenance mode.
    ///
    /// ### cluster
    /// The cluster containing the fault domain to enter maintenance mode.
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// the task tracking the enter fault domain maintenance workflow.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If any runtime error is hit.
    pub async fn vsan_enter_site_maintenance_mode(&self, fault_domain_name: &str, cluster: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanEnterSiteMaintenanceModeRequestType {fault_domain_name, cluster, };
        let bytes = self.client.invoke("vsan", "VsanSiteMaintenanceSystem", &self.mo_id, "VsanEnterSiteMaintenanceMode", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Exit the fault domain maintenance mode.
    /// 
    /// This operation exits the maintenance mode for all hosts in the specified
    /// fault domain. The objects will start the data resync during the host exit maintenance
    /// workflow. When the exit maintenance mode operation is successful, users
    /// can enable the intended workloads.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### fault_domain_name
    /// The fault domain name to exit maintenance mode.
    ///
    /// ### cluster
    /// The cluster containing the fault domain to exit maintenance mode.
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// the task tracking the exit fault domain maintenance mode workflow.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If any runtime error is hit.
    pub async fn vsan_exit_site_maintenance_mode(&self, fault_domain_name: &str, cluster: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanExitSiteMaintenanceModeRequestType {fault_domain_name, cluster, };
        let bytes = self.client.invoke("vsan", "VsanSiteMaintenanceSystem", &self.mo_id, "VsanExitSiteMaintenanceMode", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Retrieves the result of the latest fault domain maintenance check.
    /// 
    /// If a precheck task is running, its status will be returned.
    /// Otherwise the status of the last fault domain maintenance precheck
    /// will be returned.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The specified cluster to query.
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### fault_domain_name
    /// The fault domain to query.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***ManagedObjectNotFound***: If the cluster cannot be found.
    pub async fn vsan_get_site_maintenance_precheck_status(&self, cluster: &crate::types::structs::ManagedObjectReference, fault_domain_name: &str) -> Result<crate::types::structs::VsanSiteMaintenancePrecheckStatus> {
        let input = VsanGetSiteMaintenancePrecheckStatusRequestType {cluster, fault_domain_name, };
        let bytes = self.client.invoke("vsan", "VsanSiteMaintenanceSystem", &self.mo_id, "VsanGetSiteMaintenancePrecheckStatus", Some(&input)).await?;
        let result: crate::types::structs::VsanSiteMaintenancePrecheckStatus = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Initiates a precheck to determine if the target fault domain can enter maintenance mode.
    /// 
    /// The precheck performs a health check and a "what-if" query to assess the impact
    /// on cluster resources (objects, VMs) once the proposed maintenance action is executed.
    /// Only one precheck should run in a vSAN cluster at a time.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The cluster on which to run the precheck.
    /// 
    /// ***Required privileges:*** Host.Config.Maintenance
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### spec
    /// The specification of the queried fault domain maintenance action.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if run directly on an ESX Server host.
    /// 
    /// ***ManagedObjectNotFound***: If the cluster cannot be found.
    pub async fn vsan_perform_site_maintenance_precheck(&self, cluster: &crate::types::structs::ManagedObjectReference, spec: &crate::types::structs::VsanSiteMaintenanceSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanPerformSiteMaintenancePrecheckRequestType {cluster, spec, };
        let bytes = self.client.invoke("vsan", "VsanSiteMaintenanceSystem", &self.mo_id, "VsanPerformSiteMaintenancePrecheck", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Query the maintenance state of all fault domains in the specified cluster.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster to query.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// A list of SiteMaintenanceInfo objects, each representing the
    /// maintenance state of a fault domain in the target cluster. If the target
    /// cluster is not a stretched cluster, an empty list is returned.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: If any unexpected runtime fault occurs.
    pub async fn vsan_query_cluster_site_maintenance_state(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<crate::types::structs::VsanSiteMaintenanceInfo>>> {
        let input = VsanQueryClusterSiteMaintenanceStateRequestType {cluster, };
        let bytes_opt = self.client.invoke_optional("vsan", "VsanSiteMaintenanceSystem", &self.mo_id, "VsanQueryClusterSiteMaintenanceState", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
}
struct VsanEnterSiteMaintenanceModeRequestType<'a> {
    fault_domain_name: &'a str,
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for VsanEnterSiteMaintenanceModeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanEnterSiteMaintenanceModeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanEnterSiteMaintenanceModeRequestTypeSer<'b, 'a> {
    data: &'b VsanEnterSiteMaintenanceModeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanEnterSiteMaintenanceModeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanEnterSiteMaintenanceModeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("faultDomainName"), &self.data.fault_domain_name as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanExitSiteMaintenanceModeRequestType<'a> {
    fault_domain_name: &'a str,
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for VsanExitSiteMaintenanceModeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanExitSiteMaintenanceModeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanExitSiteMaintenanceModeRequestTypeSer<'b, 'a> {
    data: &'b VsanExitSiteMaintenanceModeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanExitSiteMaintenanceModeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanExitSiteMaintenanceModeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("faultDomainName"), &self.data.fault_domain_name as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanGetSiteMaintenancePrecheckStatusRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    fault_domain_name: &'a str,
}

impl<'a> miniserde::Serialize for VsanGetSiteMaintenancePrecheckStatusRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanGetSiteMaintenancePrecheckStatusRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanGetSiteMaintenancePrecheckStatusRequestTypeSer<'b, 'a> {
    data: &'b VsanGetSiteMaintenancePrecheckStatusRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanGetSiteMaintenancePrecheckStatusRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanGetSiteMaintenancePrecheckStatusRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("faultDomainName"), &self.data.fault_domain_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanPerformSiteMaintenancePrecheckRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    spec: &'a crate::types::structs::VsanSiteMaintenanceSpec,
}

impl<'a> miniserde::Serialize for VsanPerformSiteMaintenancePrecheckRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanPerformSiteMaintenancePrecheckRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanPerformSiteMaintenancePrecheckRequestTypeSer<'b, 'a> {
    data: &'b VsanPerformSiteMaintenancePrecheckRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanPerformSiteMaintenancePrecheckRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanPerformSiteMaintenancePrecheckRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanQueryClusterSiteMaintenanceStateRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for VsanQueryClusterSiteMaintenanceStateRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryClusterSiteMaintenanceStateRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryClusterSiteMaintenanceStateRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryClusterSiteMaintenanceStateRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanQueryClusterSiteMaintenanceStateRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryClusterSiteMaintenanceStateRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
