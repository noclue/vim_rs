use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::ClusterIoFilterInfo;
use crate::types::structs::IoFilterQueryIssueResult;
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::VirtualDiskId;
/// Interface to manage IO Filters installed on the ESXi hosts and
/// IO Filter configurations on virtual disks.
/// 
/// IO Filters are customized
/// filters provided by third parties to process I/Os to virtual disks.
/// They can be used to provide data services such as flash caching and
/// replication.
/// This interface is only supported on vCenter server.
pub struct IoFilterManager {
    client: Arc<Client>,
    mo_id: String,
}
impl IoFilterManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Install an IO Filter on a compute resource.
    /// 
    /// IO Filters can only be installed on a cluster.
    ///
    /// ## Parameters:
    ///
    /// ### vib_url
    /// The URL that points to the IO Filter VIB package.
    ///
    /// ### comp_res
    /// The compute resource to install the IO Filter on.
    /// "compRes" must be a cluster.
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation. The task is set to success if the filter is installed on
    /// all the hosts in the compute resource successfully. If the task fails, first
    /// check *TaskInfo.error* to see the error. If the error indicates that
    /// installation has failed on the hosts, use *IoFilterManager.QueryIoFilterIssues*
    /// to get the detailed errors occurred during installation on each host.
    /// 
    /// The dynamic privilege check ensures that the user must have
    /// Host.Config.Patch privilege for all the hosts in the compute resource.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if "compRes" is a standalone host.
    /// 
    /// ***AlreadyExists***: if another VIB with the same name and vendor has
    /// been installed.
    pub async fn install_io_filter_task(&self, vib_url: &str, comp_res: &ManagedObjectReference) -> Result<ManagedObjectReference> {
        let input = InstallIoFilterRequestType {vib_url, comp_res, };
        let path = format!("/IoFilterManager/{moId}/InstallIoFilter_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Return the list of virtual disks that use an IO Filter installed on
    /// a compute resource.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### filter_id
    /// ID of the filter.
    ///
    /// ### comp_res
    /// The compute resource that the filter has been installed on.
    /// "compRes" must be a cluster.
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ## Returns:
    ///
    /// An array of *VirtualDiskId* objects that use
    /// the given IO Filter installed on the compute resource.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the filter specified by "filterId" is
    /// not installed on the cluster.
    pub async fn query_disks_using_filter(&self, filter_id: &str, comp_res: &ManagedObjectReference) -> Result<Vec<VirtualDiskId>> {
        let input = QueryDisksUsingFilterRequestType {filter_id, comp_res, };
        let path = format!("/IoFilterManager/{moId}/QueryDisksUsingFilter", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Return the information for the IO Filters that are installed on the cluster.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### comp_res
    /// The compute resource.
    /// "compRes" must be a cluster.
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ## Returns:
    ///
    /// An array of *ClusterIoFilterInfo* objects
    /// that contain the information for the IO Filters that are installed
    /// on the compute resource.
    pub async fn query_io_filter_info(&self, comp_res: &ManagedObjectReference) -> Result<Option<Vec<ClusterIoFilterInfo>>> {
        let input = QueryIoFilterInfoRequestType {comp_res, };
        let path = format!("/IoFilterManager/{moId}/QueryIoFilterInfo", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Return the issues that occurred during the last installation/uninstallation/upgrade
    /// operation of an IO Filter on a compute resource.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### filter_id
    /// The filter.
    ///
    /// ### comp_res
    /// The compute resource.
    /// "compRes" must be a cluster.
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ## Returns:
    ///
    /// A *IoFilterQueryIssueResult* object.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the filter specified by "filterId" is
    /// not installed on the cluster.
    pub async fn query_io_filter_issues(&self, filter_id: &str, comp_res: &ManagedObjectReference) -> Result<IoFilterQueryIssueResult> {
        let input = QueryIoFilterIssuesRequestType {filter_id, comp_res, };
        let path = format!("/IoFilterManager/{moId}/QueryIoFilterIssues", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Resolve the errors occurred during an installation/uninstallation/upgrade
    /// operation of an IO Filter on a cluster.
    /// 
    /// Depending on the nature of the installation failure, vCenter will take the
    /// appropriate actions to resolve it. For example, retry or resume
    /// installation.
    ///
    /// ## Parameters:
    ///
    /// ### filter_id
    /// ID of the filter.
    ///
    /// ### cluster
    /// The cluster.
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation. The task is set to success if all the errors related to the
    /// filter are resolved on the cluster. If the task fails, first check
    /// *TaskInfo.error* to see the error. If the error indicates that
    /// issues persist on the cluster, use *IoFilterManager.QueryIoFilterIssues*
    /// to get the detailed errors on the hosts in the cluster.
    /// 
    /// The dynamic privilege check will ensure that the appropriate privileges
    /// must be acquired for all the hosts in the cluster based on the remediation
    /// actions. For example, Host.Config.Maintenance privilege and Host.Config.Patch
    /// privileges must be required for upgrading a VIB.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the filter specified by "filterId" is
    /// not installed on the cluster.
    pub async fn resolve_installation_errors_on_cluster_task(&self, filter_id: &str, cluster: &ManagedObjectReference) -> Result<ManagedObjectReference> {
        let input = ResolveInstallationErrorsOnClusterRequestType {filter_id, cluster, };
        let path = format!("/IoFilterManager/{moId}/ResolveInstallationErrorsOnCluster_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Resolve the errors occurred during an installation/uninstallation/upgrade
    /// operation of an IO Filter on a host.
    /// 
    /// Depending on the nature of the installation failure, vCenter will take the
    /// appropriate actions to resolve it. For example, retry or resume
    /// installation.
    ///
    /// ## Parameters:
    ///
    /// ### filter_id
    /// ID of the filter.
    ///
    /// ### host
    /// The host.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation. The task is set to success if all the errors related to the
    /// filter are resolved on the host. If the task fails, first check
    /// *TaskInfo.error* to see the error. If the error indicates that
    /// issues persist on the host, use *IoFilterManager.QueryIoFilterIssues*
    /// to get the detailed errors on the host.
    /// 
    /// The dynamic privilege check will ensure that the appropriate privileges
    /// are acquired based on the remediation actions. For example,
    /// Host.Config.Maintenance and Host.Config.Patch privilege must required for
    /// upgrading a VIB.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the filter specified by "filterId" is
    /// not installed on the cluster.
    pub async fn resolve_installation_errors_on_host_task(&self, filter_id: &str, host: &ManagedObjectReference) -> Result<ManagedObjectReference> {
        let input = ResolveInstallationErrorsOnHostRequestType {filter_id, host, };
        let path = format!("/IoFilterManager/{moId}/ResolveInstallationErrorsOnHost_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Uninstall an IO Filter from a compute resource.
    ///
    /// ## Parameters:
    ///
    /// ### filter_id
    /// ID of the filter.
    ///
    /// ### comp_res
    /// The compute resource to uninstall the IO Filter from.
    /// "compRes" must be a cluster.
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation. The task is set to success if the filter is uninstalled from
    /// all the hosts in the compute resource successfully. If the task fails, first
    /// check *TaskInfo.error* to see the error. If the error indicates that
    /// uninstallation has failed on the hosts, use *IoFilterManager.QueryIoFilterIssues*
    /// to get the detailed errors occurred during uninstallation on each host.
    /// 
    /// The dynamic privilege check ensures that the user must have
    /// Host.Config.Maintenance and Host.Config.Patch privilege for
    /// all the hosts in the compute resource.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if "compRes" is a standalone host.
    /// 
    /// ***NotFound***: if the filter is not installed on the cluster.
    /// 
    /// ***FilterInUse***: if the filter to be uninstalled is being used by a
    /// virtual disk.
    /// 
    /// ***InvalidState***: if "compRes" is a cluster and DRS is disabled
    /// on the cluster.
    pub async fn uninstall_io_filter_task(&self, filter_id: &str, comp_res: &ManagedObjectReference) -> Result<ManagedObjectReference> {
        let input = UninstallIoFilterRequestType {filter_id, comp_res, };
        let path = format!("/IoFilterManager/{moId}/UninstallIoFilter_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Upgrade an IO Filter on a compute resource.
    ///
    /// ## Parameters:
    ///
    /// ### filter_id
    /// The filter to be upgraded.
    ///
    /// ### comp_res
    /// The compute resource that the filter is installed on.
    /// "compRes" must be a cluster.
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### vib_url
    /// The URL that points to the new IO Filter VIB package.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation. The task is set to success if all the hosts in the compute
    /// resource are upgraded successfully. If the task fails, first check
    /// *TaskInfo.error* to see the error. If the error indicates that
    /// upgrade has failed on the hosts, use *IoFilterManager.QueryIoFilterIssues*
    /// to get the detailed errors occurred during upgrade on each host.
    /// 
    /// The dynamic privilege check ensures that the user must have
    /// Host.Config.Maintenance and Host.Config.Patch privileges for
    /// all the hosts in the compute resource.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if "compRes" is a standalone host; or if the VIB
    /// package pointed by "vibUrl" is not an upgrade of the
    /// IO Filter specified by "filterId".
    /// 
    /// ***NotFound***: if the filter specified by "filterId" is
    /// not installed on the cluster.
    /// 
    /// ***InvalidState***: if "compRes" is a cluster and DRS is disabled
    /// on the cluster.
    pub async fn upgrade_io_filter_task(&self, filter_id: &str, comp_res: &ManagedObjectReference, vib_url: &str) -> Result<ManagedObjectReference> {
        let input = UpgradeIoFilterRequestType {filter_id, comp_res, vib_url, };
        let path = format!("/IoFilterManager/{moId}/UpgradeIoFilter_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct InstallIoFilterRequestType<'a> {
    #[serde(rename = "vibUrl")]
    vib_url: &'a str,
    #[serde(rename = "compRes")]
    comp_res: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryDisksUsingFilterRequestType<'a> {
    #[serde(rename = "filterId")]
    filter_id: &'a str,
    #[serde(rename = "compRes")]
    comp_res: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryIoFilterInfoRequestType<'a> {
    #[serde(rename = "compRes")]
    comp_res: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryIoFilterIssuesRequestType<'a> {
    #[serde(rename = "filterId")]
    filter_id: &'a str,
    #[serde(rename = "compRes")]
    comp_res: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ResolveInstallationErrorsOnClusterRequestType<'a> {
    #[serde(rename = "filterId")]
    filter_id: &'a str,
    cluster: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ResolveInstallationErrorsOnHostRequestType<'a> {
    #[serde(rename = "filterId")]
    filter_id: &'a str,
    host: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UninstallIoFilterRequestType<'a> {
    #[serde(rename = "filterId")]
    filter_id: &'a str,
    #[serde(rename = "compRes")]
    comp_res: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpgradeIoFilterRequestType<'a> {
    #[serde(rename = "filterId")]
    filter_id: &'a str,
    #[serde(rename = "compRes")]
    comp_res: &'a ManagedObjectReference,
    #[serde(rename = "vibUrl")]
    vib_url: &'a str,
}
