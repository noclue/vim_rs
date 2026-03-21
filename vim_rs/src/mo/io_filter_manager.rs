use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Interface to manage IO Filters installed on the ESXi hosts and
/// IO Filter configurations on virtual disks.
/// 
/// IO Filters are customized
/// filters provided by third parties to process I/Os to virtual disks.
/// They can be used to provide data services such as flash caching and
/// replication.
/// This interface is only supported on vCenter server.
#[derive(Clone)]
pub struct IoFilterManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl IoFilterManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Initiates iofilter manager transition from EAM managed APIs to
    /// vLCM managed APIs on a vLCM cluster.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The cluster.
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor
    /// the operation. The operation succeeds if transition of all iofilters
    /// is successful from EAM managed APIs to vLCM APIs on the cluster.
    /// 
    /// The user must have Host.Config.Maintenance and Host.Config.Patch
    /// privilege for all the hosts on the compute resource.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if cluster is not a vLCM cluster.
    pub async fn initiate_transition_to_vlcm_task(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = InitiateTransitionToVlcmRequestType {cluster, };
        let bytes = self.client.invoke("", "IoFilterManager", &self.mo_id, "InitiateTransitionToVLCM_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    /// ### vib_ssl_trust
    /// This specifies SSL trust policy *IoFilterManagerSslTrust*
    /// for the given VIB URL. If unset, the server certificate is
    /// validated against the trusted root certificates.
    /// 
    /// ***Since:*** vSphere API Release 8.0.3.0
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
    pub async fn install_io_filter_task(&self, vib_url: &str, comp_res: &crate::types::structs::ManagedObjectReference, vib_ssl_trust: Option<&dyn crate::types::traits::IoFilterManagerSslTrustTrait>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = InstallIoFilterRequestType {vib_url, comp_res, vib_ssl_trust, };
        let bytes = self.client.invoke("", "IoFilterManager", &self.mo_id, "InstallIoFilter_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn query_disks_using_filter(&self, filter_id: &str, comp_res: &crate::types::structs::ManagedObjectReference) -> Result<Vec<crate::types::structs::VirtualDiskId>> {
        let input = QueryDisksUsingFilterRequestType {filter_id, comp_res, };
        let bytes = self.client.invoke("", "IoFilterManager", &self.mo_id, "QueryDisksUsingFilter", Some(&input)).await?;
        let result: Vec<crate::types::structs::VirtualDiskId> = crate::core::client::unmarshal_array(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn query_io_filter_info(&self, comp_res: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<crate::types::structs::ClusterIoFilterInfo>>> {
        let input = QueryIoFilterInfoRequestType {comp_res, };
        let bytes_opt = self.client.invoke_optional("", "IoFilterManager", &self.mo_id, "QueryIoFilterInfo", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn query_io_filter_issues(&self, filter_id: &str, comp_res: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::IoFilterQueryIssueResult> {
        let input = QueryIoFilterIssuesRequestType {filter_id, comp_res, };
        let bytes = self.client.invoke("", "IoFilterManager", &self.mo_id, "QueryIoFilterIssues", Some(&input)).await?;
        let result: crate::types::structs::IoFilterQueryIssueResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn resolve_installation_errors_on_cluster_task(&self, filter_id: &str, cluster: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ResolveInstallationErrorsOnClusterRequestType {filter_id, cluster, };
        let bytes = self.client.invoke("", "IoFilterManager", &self.mo_id, "ResolveInstallationErrorsOnCluster_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn resolve_installation_errors_on_host_task(&self, filter_id: &str, host: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ResolveInstallationErrorsOnHostRequestType {filter_id, host, };
        let bytes = self.client.invoke("", "IoFilterManager", &self.mo_id, "ResolveInstallationErrorsOnHost_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn uninstall_io_filter_task(&self, filter_id: &str, comp_res: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = UninstallIoFilterRequestType {filter_id, comp_res, };
        let bytes = self.client.invoke("", "IoFilterManager", &self.mo_id, "UninstallIoFilter_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    /// ### vib_ssl_trust
    /// This specifies SSL trust policy *IoFilterManagerSslTrust*
    /// for the given VIB URL. If unset, the server certificate is
    /// validated against the trusted root certificates.
    /// 
    /// ***Since:*** vSphere API Release 8.0.3.0
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
    pub async fn upgrade_io_filter_task(&self, filter_id: &str, comp_res: &crate::types::structs::ManagedObjectReference, vib_url: &str, vib_ssl_trust: Option<&dyn crate::types::traits::IoFilterManagerSslTrustTrait>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = UpgradeIoFilterRequestType {filter_id, comp_res, vib_url, vib_ssl_trust, };
        let bytes = self.client.invoke("", "IoFilterManager", &self.mo_id, "UpgradeIoFilter_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct InitiateTransitionToVlcmRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for InitiateTransitionToVlcmRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(InitiateTransitionToVlcmRequestTypeSer { data: self, seq: 0 }))
    }
}

struct InitiateTransitionToVlcmRequestTypeSer<'b, 'a> {
    data: &'b InitiateTransitionToVlcmRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for InitiateTransitionToVlcmRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"InitiateTransitionToVLCMRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct InstallIoFilterRequestType<'a> {
    vib_url: &'a str,
    comp_res: &'a crate::types::structs::ManagedObjectReference,
    vib_ssl_trust: Option<&'a dyn crate::types::traits::IoFilterManagerSslTrustTrait>,
}

impl<'a> miniserde::Serialize for InstallIoFilterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(InstallIoFilterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct InstallIoFilterRequestTypeSer<'b, 'a> {
    data: &'b InstallIoFilterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for InstallIoFilterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"InstallIoFilterRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("vibUrl"), &self.data.vib_url as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("compRes"), &self.data.comp_res as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.vib_ssl_trust else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vibSslTrust"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryDisksUsingFilterRequestType<'a> {
    filter_id: &'a str,
    comp_res: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for QueryDisksUsingFilterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryDisksUsingFilterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryDisksUsingFilterRequestTypeSer<'b, 'a> {
    data: &'b QueryDisksUsingFilterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryDisksUsingFilterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryDisksUsingFilterRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("filterId"), &self.data.filter_id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("compRes"), &self.data.comp_res as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryIoFilterInfoRequestType<'a> {
    comp_res: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for QueryIoFilterInfoRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryIoFilterInfoRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryIoFilterInfoRequestTypeSer<'b, 'a> {
    data: &'b QueryIoFilterInfoRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryIoFilterInfoRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryIoFilterInfoRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("compRes"), &self.data.comp_res as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryIoFilterIssuesRequestType<'a> {
    filter_id: &'a str,
    comp_res: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for QueryIoFilterIssuesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryIoFilterIssuesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryIoFilterIssuesRequestTypeSer<'b, 'a> {
    data: &'b QueryIoFilterIssuesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryIoFilterIssuesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryIoFilterIssuesRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("filterId"), &self.data.filter_id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("compRes"), &self.data.comp_res as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ResolveInstallationErrorsOnClusterRequestType<'a> {
    filter_id: &'a str,
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for ResolveInstallationErrorsOnClusterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ResolveInstallationErrorsOnClusterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ResolveInstallationErrorsOnClusterRequestTypeSer<'b, 'a> {
    data: &'b ResolveInstallationErrorsOnClusterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ResolveInstallationErrorsOnClusterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ResolveInstallationErrorsOnClusterRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("filterId"), &self.data.filter_id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ResolveInstallationErrorsOnHostRequestType<'a> {
    filter_id: &'a str,
    host: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for ResolveInstallationErrorsOnHostRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ResolveInstallationErrorsOnHostRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ResolveInstallationErrorsOnHostRequestTypeSer<'b, 'a> {
    data: &'b ResolveInstallationErrorsOnHostRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ResolveInstallationErrorsOnHostRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ResolveInstallationErrorsOnHostRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("filterId"), &self.data.filter_id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("host"), &self.data.host as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UninstallIoFilterRequestType<'a> {
    filter_id: &'a str,
    comp_res: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for UninstallIoFilterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UninstallIoFilterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UninstallIoFilterRequestTypeSer<'b, 'a> {
    data: &'b UninstallIoFilterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UninstallIoFilterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UninstallIoFilterRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("filterId"), &self.data.filter_id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("compRes"), &self.data.comp_res as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpgradeIoFilterRequestType<'a> {
    filter_id: &'a str,
    comp_res: &'a crate::types::structs::ManagedObjectReference,
    vib_url: &'a str,
    vib_ssl_trust: Option<&'a dyn crate::types::traits::IoFilterManagerSslTrustTrait>,
}

impl<'a> miniserde::Serialize for UpgradeIoFilterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpgradeIoFilterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpgradeIoFilterRequestTypeSer<'b, 'a> {
    data: &'b UpgradeIoFilterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpgradeIoFilterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpgradeIoFilterRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("filterId"), &self.data.filter_id as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("compRes"), &self.data.comp_res as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("vibUrl"), &self.data.vib_url as &dyn miniserde::Serialize)),
                4 => {
                    let Some(ref val) = self.data.vib_ssl_trust else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vibSslTrust"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
