use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object type provides the file service related configuration
/// and query APIs.
/// 
/// It can be accessed through MOID of 'vsan-cluster-file-service-system',
/// through vSAN service on vCenter at cluster level, or accessed through MOID
/// of 'vsan-file-service-system' on ESXi host for the detailed operation.
#[derive(Clone)]
pub struct VsanFileServiceSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanFileServiceSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Creates a file service domain in the vSAN cluster.
    /// 
    /// A vSAN file service domain is designed with the following properties:
    /// - A file service domain has a FQDN domain name (e.g., fs.mycompany.com)
    ///   that client can mount.
    /// - It can join an Active Directory domain and use Kerberos for
    ///   authentication and user ID mapping, or use AUTH\_SYS to trust user ID
    ///   sent from clients.
    /// - One more more file shares can be created in a file service domain
    ///   and all these file shares have the same security/network properties
    ///   of this file service domain. These file shares can be accessed from a
    ///   certain network (e.g., VM Network).
    ///   
    /// In current version, only one file service domain can be created per cluster,
    /// which can be initiated when the file service is enabled.
    ///
    /// ## Parameters:
    ///
    /// ### domain_config
    /// Domain configuration information.
    ///
    /// ### cluster
    /// The target cluster. Ignored when called on ESXi hosts.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// A task tracking the domain creation progress.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: Exception when vSAN file service is not enabled in
    /// this cluster.
    /// 
    /// ***InvalidArgument***: Exception for invalid input arguments, for example,
    /// if the domain name already configured in the cluster.
    pub async fn vsan_cluster_create_fs_domain(&self, domain_config: &crate::types::structs::VsanFileServiceDomainConfig, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanClusterCreateFsDomainRequestType {domain_config, cluster, };
        let path = format!("/vsan/VsanFileServiceSystem/{moId}/VsanClusterCreateFsDomain", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Creates a file share in this vSAN cluster.
    ///
    /// ## Parameters:
    ///
    /// ### config
    /// The file share configuration, as specified in
    /// . Note that the backing vSAN object for the
    /// file share will be by default 1TB if the file share is thin
    /// provisioned. Otherwise the backing vSAN object is set to 255GB if
    /// quota is not set.
    ///
    /// ### cluster
    /// The target cluster. Ignored when called on ESXi hosts.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// A task tracking the share creation progress. The UUID of the file
    /// share will be set to the task result field if the task succeeds.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: Exception when vSAN file service is not enabled in
    /// this cluster.
    /// 
    /// ***InvalidArgument***: Exception for invalid input arguments, for example,
    /// if the domain for this new file share does not exist
    /// in this cluster.
    pub async fn vsan_create_file_share(&self, config: &crate::types::structs::VsanFileShareConfig, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanCreateFileShareRequestType {config, cluster, };
        let path = format!("/vsan/VsanFileServiceSystem/{moId}/VsanCreateFileShare", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Create a snapshot for a file share in this vSAN cluster.
    ///
    /// ## Parameters:
    ///
    /// ### config
    /// The snapshot configuration.
    ///
    /// ### cluster
    /// The target cluster. Ignored when called on ESXi hosts.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// A task tracking the snapshot creation progress.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: Exception for invalid states, for example, if vSAN
    /// file service is not enabled in this cluster.
    /// 
    /// ***InvalidArgument***: Exception for invalid input arguments, for example,
    /// if the file share to create a snapshot does not exist
    /// in this cluster.
    pub async fn vsan_cluster_create_share_snapshot(&self, config: &crate::types::structs::VsanFileShareSnapshotConfig, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanClusterCreateShareSnapshotRequestType {config, cluster, };
        let path = format!("/vsan/VsanFileServiceSystem/{moId}/VsanClusterCreateShareSnapshot", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Downloads a file service OVF file of the specified version from VMware
    /// website and install it to the OVF repository in vCenter.
    /// 
    /// ***Required privileges:*** Global.VCServer
    ///
    /// ## Parameters:
    ///
    /// ### download_url
    /// URL to download the vSAN file service OVF, only HTTPS URL
    /// is supported.
    ///
    /// ## Returns:
    ///
    /// Task for tracking the OVF installation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: Exception if the OVF of the specified version does
    /// not exist as specified in the source URL.
    pub async fn vsan_download_file_service_ovf(&self, download_url: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanDownloadFileServiceOvfRequestType {download_url, };
        let path = format!("/vsan/VsanFileServiceSystem/{moId}/VsanDownloadFileServiceOvf", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Find a compatible vSAN File Service OVF download URL for the target cluster.
    /// 
    /// This API needs internet access to fetch the latest OVF download URL for the
    /// specified cluster.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster to download OVF files.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// The OVF download URL. Return an empty string if no proper OVF
    /// download URL could be found.
    pub async fn vsan_find_ovf_download_url(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<String> {
        let input = VsanFindOvfDownloadUrlRequestType {cluster, };
        let path = format!("/vsan/VsanFileServiceSystem/{moId}/VsanFindOvfDownloadUrl", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: String = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Perform a preflight check on a cluster for enabling vSAN file service
    /// and/or for the new file service domain configuration.
    /// 
    /// The preflight check items includes:
    /// - Basic check
    ///   - Network partition check
    ///   - vSAN datastore presence to each host
    ///   - The versions of the ESXi hosts match in this cluster
    ///   - NTP configuration check in ESXi hosts if vSAN file service
    ///     version is 8.0 or newer
    ///   - Domain configuration format check
    ///   - Running OVF information check
    ///   - If a DVS portgroup is passed as network,
    ///     it checks if DVS version is older than 6.6
    /// - Advanced check
    ///   - Checks covered in 'basic' scope
    ///   - File server domain configuration validation in live environment
    ///     when vSAN file service has been enabled
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster to perform preflight check.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### domain_config
    /// The domain configuration to be validated.
    /// If not specified, the validation for the
    /// domain will be skipped.
    ///
    /// ### network
    /// The network which will be used for fs containers
    /// 
    /// Refers instance of *Network*.
    ///
    /// ### scope
    /// The scope that preflight check will cover. Valid inputs are
    /// listed in the *VsanFileServicePreflightCheckScope_enum*
    /// field. Defaults to "basic".
    ///
    /// ### domain_uuid
    /// The file service domain UUID. It is required when
    /// the advanced preflight check is performed on an
    /// existing domain. If a new file service domain is
    /// to be created, leave it empty.
    ///
    /// ## Returns:
    ///
    /// Preflight check result.
    pub async fn vsan_perform_file_service_enable_preflight_check(&self, cluster: &crate::types::structs::ManagedObjectReference, domain_config: Option<&crate::types::structs::VsanFileServiceDomainConfig>, network: Option<&crate::types::structs::ManagedObjectReference>, scope: Option<&str>, domain_uuid: Option<&str>) -> Result<crate::types::structs::VsanFileServicePreflightCheckResult> {
        let input = VsanPerformFileServiceEnablePreflightCheckRequestType {cluster, domain_config, network, scope, domain_uuid, };
        let path = format!("/vsan/VsanFileServiceSystem/{moId}/VsanPerformFileServiceEnablePreflightCheck", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::VsanFileServicePreflightCheckResult = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Lists all file service domains in the vSAN cluster.
    /// 
    /// To list all the file service domains, leave both the uuids and names
    /// field empty. Otherwise the return result will be a set of file service
    /// domains matches the domain UUID or domain name provided in the parameters.
    /// If both fields are provided, only those file service domain with both
    /// domain name and domain UUID matched will be returned.
    ///
    /// ## Parameters:
    ///
    /// ### query_spec
    /// The specifications of the file service domains to be
    /// queried.
    ///
    /// ### cluster
    /// The target cluster. Ignored when called on ESXi hosts.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// List of all the domain information on this host.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: Exception when the file service is not enabled in this
    /// cluster.
    pub async fn vsan_cluster_query_fs_domains(&self, query_spec: Option<&crate::types::structs::VsanFileServiceDomainQuerySpec>, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<crate::types::structs::VsanFileServiceDomain>>> {
        let input = VsanClusterQueryFsDomainsRequestType {query_spec, cluster, };
        let path = format!("/vsan/VsanFileServiceSystem/{moId}/VsanClusterQueryFsDomains", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::VsanFileServiceDomain>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Lists all available vSAN File Service OVF in this vCenter.
    /// 
    /// ***Required privileges:*** Global.VCServer
    ///
    /// ## Returns:
    ///
    /// List of the file services OVFs available in this vCenter.
    pub async fn vsan_query_file_service_ovfs(&self) -> Result<Option<Vec<crate::types::structs::VsanFileServiceOvfSpec>>> {
        let path = format!("/vsan/VsanFileServiceSystem/{moId}/VsanQueryFileServiceOvfs", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::VsanFileServiceOvfSpec>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// List all the snapshots that match the query spec.
    ///
    /// ## Parameters:
    ///
    /// ### query_spec
    /// The specification of the snapshots to be queried.
    ///
    /// ### cluster
    /// The target cluster. Ignored when called on ESXi hosts.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// Result of the snapshot query.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: Exception for invalid states, for example, if vSAN
    /// file service is not enabled in this cluster.
    /// 
    /// ***InvalidArgument***: Exception for invalid input arguments, for example,
    /// if the share does not exist in this cluster.
    pub async fn vsan_cluster_query_share_snapshots(&self, query_spec: &crate::types::structs::VsanFileShareSnapshotQuerySpec, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<crate::types::structs::VsanFileShareSnapshotQueryResult>> {
        let input = VsanClusterQueryShareSnapshotsRequestType {query_spec, cluster, };
        let path = format!("/vsan/VsanFileServiceSystem/{moId}/VsanClusterQueryShareSnapshots", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::VsanFileShareSnapshotQueryResult>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Lists all file shares in the domain.
    /// 
    /// To list all the file shares in the domain, leave both the shareUuids and
    /// shareNames field empty. Otherwise the return result will be a set of file
    /// shares matches the share UUID or share name provided in the parameters. If
    /// both fields are provided, only those file shares with both share name and
    /// share UUID matched will be returned.
    /// Note that if none of file shares matches the querySpec, the return value is
    /// None.
    ///
    /// ## Parameters:
    ///
    /// ### query_spec
    /// The specifications of the file shares to be queried.
    ///
    /// ### cluster
    /// The target cluster. Ignored when called on ESXi hosts.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// Result of the file share query.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: Exception when vSAN file service is not enabled in
    /// this cluster.
    /// 
    /// ***InvalidArgument***: Exception for invalid input arguments, for example,
    /// if the domain does not exist in this cluster.
    pub async fn vsan_cluster_query_file_shares(&self, query_spec: &crate::types::structs::VsanFileShareQuerySpec, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<crate::types::structs::FileShareQueryResult>> {
        let input = VsanClusterQueryFileSharesRequestType {query_spec, cluster, };
        let path = format!("/vsan/VsanFileServiceSystem/{moId}/VsanClusterQueryFileShares", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::FileShareQueryResult>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// This API is to rebalance file service in cluster.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster to do rebalance.
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// the task tracking the rebalance progress.
    /// 
    /// Refers instance of *Task*.
    pub async fn vsan_rebalance_file_service(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanRebalanceFileServiceRequestType {cluster, };
        let path = format!("/vsan/VsanFileServiceSystem/{moId}/VsanRebalanceFileService", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Updates a file service domain in the vSAN cluster.
    ///
    /// ## Parameters:
    ///
    /// ### domain_uuid
    /// The UUID of the domain to be reconfigured.
    ///
    /// ### domain_config
    /// New configuration of the domain. Only set the fields that
    /// require reconfiguration, and leave others unset.
    ///
    /// ### cluster
    /// The target cluster. Ignored when called on ESXi hosts.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### delete_domain_config_fields
    /// The domain config fields to be deleted. For
    /// example to remove directoryServerConfig
    /// from the domainConfig provide
    /// \["directoryServerConfig"\] here and keep the
    /// same unset in provided domainConfig. Do note
    /// removing directoryServerConfig is only
    /// allowed when there are no active shares.
    /// Here is the list of currently supported field:
    /// - directoryServerConfig since 7.0U1
    /// - directoryServerConfig.preferredADServers
    ///   since 8.0U1 
    ///   
    /// Providing any other value here will cause
    /// InvalidArgumentError fault.
    ///
    /// ## Returns:
    ///
    /// A task tracking the domain reconfiguration progress.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: Exception when vSAN file service is not enabled in
    /// this cluster.
    /// 
    /// ***InvalidArgument***: Exception for invalid input arguments, for example,
    /// if the domain does not exist in this cluster.
    pub async fn vsan_cluster_reconfigure_fs_domain(&self, domain_uuid: &str, domain_config: &crate::types::structs::VsanFileServiceDomainConfig, cluster: Option<&crate::types::structs::ManagedObjectReference>, delete_domain_config_fields: Option<&[String]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanClusterReconfigureFsDomainRequestType {domain_uuid, domain_config, cluster, delete_domain_config_fields, };
        let path = format!("/vsan/VsanFileServiceSystem/{moId}/VsanClusterReconfigureFsDomain", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Reconfigures a file share in this vSAN cluster.
    /// 
    /// All the configurations of a vSAN file share can be reconfigured through this
    /// API, except the file service domain.
    ///
    /// ## Parameters:
    ///
    /// ### share_uuid
    /// The UUID of the file share to be reconfigured.
    ///
    /// ### config
    /// The file services configuration. Only set the fields that
    /// require reconfiguration, and leave the others unset.
    /// This API will update or create the labels specified in the
    /// config. Labels to be deleted should be specified in the
    /// 'deleteLabelKeys' parameter. Other labels will remain intact.
    ///
    /// ### cluster
    /// The target cluster. Ignored when called on ESXi hosts.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### delete_label_keys
    /// The keys of share labels to be deleted. If a specified
    /// label key does not exist in the file share, the deletion of
    /// this label will be ignored.
    ///
    /// ### force
    /// The force flag is to force the reconfiguration of a vSphere
    /// managed file share, for example, the file share managed by Cloud
    /// Native Storage (CNS) service.
    ///
    /// ## Returns:
    ///
    /// A task tracking the share reconfiguring progress.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: Exception when vSAN file service is not enabled in
    /// this cluster.
    /// 
    /// ***InvalidArgument***: Exception for invalid input arguments, for example,
    /// if the file share does not exist in this cluster.
    pub async fn vsan_reconfigure_file_share(&self, share_uuid: &str, config: &crate::types::structs::VsanFileShareConfig, cluster: Option<&crate::types::structs::ManagedObjectReference>, delete_label_keys: Option<&[String]>, force: Option<bool>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanReconfigureFileShareRequestType {share_uuid, config, cluster, delete_label_keys, force, };
        let path = format!("/vsan/VsanFileServiceSystem/{moId}/VsanReconfigureFileShare", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Removes a file service domain in the vSAN cluster.
    /// 
    /// A file service domain is not allowed to be removed if it still has any
    /// file shares.
    ///
    /// ## Parameters:
    ///
    /// ### domain_uuid
    /// The unique domain uuid that is configured in the directory
    /// service, for example, Active Directory (AD) from Microsoft.
    ///
    /// ### cluster
    /// The target cluster. Ignored when called on ESXi hosts.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// A task tracking the domain remove progress.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: Exception when vSAN file service is not enabled in
    /// this cluster, or it still has file shares.
    /// 
    /// ***InvalidArgument***: Exception for invalid input arguments, for example,
    /// if the domain does not exist in this cluster.
    pub async fn vsan_cluster_remove_fs_domain(&self, domain_uuid: &str, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanClusterRemoveFsDomainRequestType {domain_uuid, cluster, };
        let path = format!("/vsan/VsanFileServiceSystem/{moId}/VsanClusterRemoveFsDomain", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Removes a file share in the domain.
    ///
    /// ## Parameters:
    ///
    /// ### share_uuid
    /// The UUID of the file share to be removed.
    ///
    /// ### cluster
    /// The target cluster. Ignored when called on ESXi hosts.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### force
    /// The force flag is to force the removal of a vSphere managed
    /// file share, for example, the file share managed by Cloud
    /// Native Storage (CNS) service.
    ///
    /// ## Returns:
    ///
    /// A task tracking the file share remove progress.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: Exception when vSAN file service is not enabled in
    /// this cluster.
    /// 
    /// ***InvalidArgument***: Exception for invalid input arguments, for example,
    /// if the file share does not exist in this cluster.
    pub async fn vsan_cluster_remove_share(&self, share_uuid: &str, cluster: Option<&crate::types::structs::ManagedObjectReference>, force: Option<bool>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanClusterRemoveShareRequestType {share_uuid, cluster, force, };
        let path = format!("/vsan/VsanFileServiceSystem/{moId}/VsanClusterRemoveShare", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Remove a snapshot of a file share in this vSAN cluster.
    /// 
    /// Note: due to the limitation, it's not allowed to remove the latest snapshot
    /// for file service in 70U2 and an InvalidState exception will be thrown for
    /// this case.
    ///
    /// ## Parameters:
    ///
    /// ### share_uuid
    /// UUID of the file share to delete the snapshots.
    ///
    /// ### snapshot_name
    /// Name of the snapshot to be deleted.
    ///
    /// ### cluster
    /// The target cluster. Ignored when called on ESXi hosts.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// A task tracking the snapshot remove progress.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: Exception for invalid states, for example, if vSAN
    /// file service is not enabled in this cluster.
    /// 
    /// ***InvalidArgument***: Exception for invalid input arguments, for example,
    /// if the snapshot does not exist in this cluster.
    pub async fn vsan_cluster_remove_share_snapshot(&self, share_uuid: &str, snapshot_name: &str, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanClusterRemoveShareSnapshotRequestType {share_uuid, snapshot_name, cluster, };
        let path = format!("/vsan/VsanFileServiceSystem/{moId}/VsanClusterRemoveShareSnapshot", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Upgrade FSVM to latest ovf that is compatible with cluster's host version.
    /// 
    /// A compatible OVF is expected be uploaded before calling this API.
    /// A preflight check will be conducted before the upgrade on the following API.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The target cluster in which FSVM deployed.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// None
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: Exception if the OVF of the compatible version for the
    /// hosts cannot be found on the corresponding vCenter
    /// server, or file service is not running in a valid state.
    pub async fn vsan_upgrade_fsvm(&self, cluster: &crate::types::structs::ManagedObjectReference) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanUpgradeFsvmRequestType {cluster, };
        let path = format!("/vsan/VsanFileServiceSystem/{moId}/VsanUpgradeFsvm", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct VsanClusterCreateFsDomainRequestType<'a> {
    domain_config: &'a crate::types::structs::VsanFileServiceDomainConfig,
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for VsanClusterCreateFsDomainRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanClusterCreateFsDomainRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanClusterCreateFsDomainRequestTypeSer<'b, 'a> {
    data: &'b VsanClusterCreateFsDomainRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanClusterCreateFsDomainRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanClusterCreateFsDomainRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("domainConfig"), &self.data.domain_config as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanCreateFileShareRequestType<'a> {
    config: &'a crate::types::structs::VsanFileShareConfig,
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for VsanCreateFileShareRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanCreateFileShareRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanCreateFileShareRequestTypeSer<'b, 'a> {
    data: &'b VsanCreateFileShareRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanCreateFileShareRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanCreateFileShareRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("config"), &self.data.config as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanClusterCreateShareSnapshotRequestType<'a> {
    config: &'a crate::types::structs::VsanFileShareSnapshotConfig,
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for VsanClusterCreateShareSnapshotRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanClusterCreateShareSnapshotRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanClusterCreateShareSnapshotRequestTypeSer<'b, 'a> {
    data: &'b VsanClusterCreateShareSnapshotRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanClusterCreateShareSnapshotRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanClusterCreateShareSnapshotRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("config"), &self.data.config as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanDownloadFileServiceOvfRequestType<'a> {
    download_url: &'a str,
}

impl<'a> miniserde::Serialize for VsanDownloadFileServiceOvfRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanDownloadFileServiceOvfRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanDownloadFileServiceOvfRequestTypeSer<'b, 'a> {
    data: &'b VsanDownloadFileServiceOvfRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanDownloadFileServiceOvfRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanDownloadFileServiceOvfRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("downloadUrl"), &self.data.download_url as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanFindOvfDownloadUrlRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for VsanFindOvfDownloadUrlRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanFindOvfDownloadUrlRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanFindOvfDownloadUrlRequestTypeSer<'b, 'a> {
    data: &'b VsanFindOvfDownloadUrlRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanFindOvfDownloadUrlRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanFindOvfDownloadUrlRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanPerformFileServiceEnablePreflightCheckRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    domain_config: Option<&'a crate::types::structs::VsanFileServiceDomainConfig>,
    network: Option<&'a crate::types::structs::ManagedObjectReference>,
    scope: Option<&'a str>,
    domain_uuid: Option<&'a str>,
}

impl<'a> miniserde::Serialize for VsanPerformFileServiceEnablePreflightCheckRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanPerformFileServiceEnablePreflightCheckRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanPerformFileServiceEnablePreflightCheckRequestTypeSer<'b, 'a> {
    data: &'b VsanPerformFileServiceEnablePreflightCheckRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanPerformFileServiceEnablePreflightCheckRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanPerformFileServiceEnablePreflightCheckRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.domain_config else { continue; };
                    return Some((std::borrow::Cow::Borrowed("domainConfig"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.network else { continue; };
                    return Some((std::borrow::Cow::Borrowed("network"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.scope else { continue; };
                    return Some((std::borrow::Cow::Borrowed("scope"), val as &dyn miniserde::Serialize));
                }
                5 => {
                    let Some(ref val) = self.data.domain_uuid else { continue; };
                    return Some((std::borrow::Cow::Borrowed("domainUuid"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanClusterQueryFsDomainsRequestType<'a> {
    query_spec: Option<&'a crate::types::structs::VsanFileServiceDomainQuerySpec>,
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for VsanClusterQueryFsDomainsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanClusterQueryFsDomainsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanClusterQueryFsDomainsRequestTypeSer<'b, 'a> {
    data: &'b VsanClusterQueryFsDomainsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanClusterQueryFsDomainsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanClusterQueryFsDomainsRequestType")),
                1 => {
                    let Some(ref val) = self.data.query_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("querySpec"), val as &dyn miniserde::Serialize));
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
struct VsanClusterQueryShareSnapshotsRequestType<'a> {
    query_spec: &'a crate::types::structs::VsanFileShareSnapshotQuerySpec,
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for VsanClusterQueryShareSnapshotsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanClusterQueryShareSnapshotsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanClusterQueryShareSnapshotsRequestTypeSer<'b, 'a> {
    data: &'b VsanClusterQueryShareSnapshotsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanClusterQueryShareSnapshotsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanClusterQueryShareSnapshotsRequestType")),
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
struct VsanClusterQueryFileSharesRequestType<'a> {
    query_spec: &'a crate::types::structs::VsanFileShareQuerySpec,
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for VsanClusterQueryFileSharesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanClusterQueryFileSharesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanClusterQueryFileSharesRequestTypeSer<'b, 'a> {
    data: &'b VsanClusterQueryFileSharesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanClusterQueryFileSharesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanClusterQueryFileSharesRequestType")),
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
struct VsanRebalanceFileServiceRequestType<'a> {
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for VsanRebalanceFileServiceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanRebalanceFileServiceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanRebalanceFileServiceRequestTypeSer<'b, 'a> {
    data: &'b VsanRebalanceFileServiceRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanRebalanceFileServiceRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanRebalanceFileServiceRequestType")),
                1 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanClusterReconfigureFsDomainRequestType<'a> {
    domain_uuid: &'a str,
    domain_config: &'a crate::types::structs::VsanFileServiceDomainConfig,
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    delete_domain_config_fields: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for VsanClusterReconfigureFsDomainRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanClusterReconfigureFsDomainRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanClusterReconfigureFsDomainRequestTypeSer<'b, 'a> {
    data: &'b VsanClusterReconfigureFsDomainRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanClusterReconfigureFsDomainRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanClusterReconfigureFsDomainRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("domainUuid"), &self.data.domain_uuid as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("domainConfig"), &self.data.domain_config as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.delete_domain_config_fields else { continue; };
                    return Some((std::borrow::Cow::Borrowed("deleteDomainConfigFields"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanReconfigureFileShareRequestType<'a> {
    share_uuid: &'a str,
    config: &'a crate::types::structs::VsanFileShareConfig,
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    delete_label_keys: Option<&'a [String]>,
    force: Option<bool>,
}

impl<'a> miniserde::Serialize for VsanReconfigureFileShareRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanReconfigureFileShareRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanReconfigureFileShareRequestTypeSer<'b, 'a> {
    data: &'b VsanReconfigureFileShareRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanReconfigureFileShareRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanReconfigureFileShareRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("shareUuid"), &self.data.share_uuid as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("config"), &self.data.config as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.delete_label_keys else { continue; };
                    return Some((std::borrow::Cow::Borrowed("deleteLabelKeys"), val as &dyn miniserde::Serialize));
                }
                5 => {
                    let Some(ref val) = self.data.force else { continue; };
                    return Some((std::borrow::Cow::Borrowed("force"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanClusterRemoveFsDomainRequestType<'a> {
    domain_uuid: &'a str,
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for VsanClusterRemoveFsDomainRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanClusterRemoveFsDomainRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanClusterRemoveFsDomainRequestTypeSer<'b, 'a> {
    data: &'b VsanClusterRemoveFsDomainRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanClusterRemoveFsDomainRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanClusterRemoveFsDomainRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("domainUuid"), &self.data.domain_uuid as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanClusterRemoveShareRequestType<'a> {
    share_uuid: &'a str,
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    force: Option<bool>,
}

impl<'a> miniserde::Serialize for VsanClusterRemoveShareRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanClusterRemoveShareRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanClusterRemoveShareRequestTypeSer<'b, 'a> {
    data: &'b VsanClusterRemoveShareRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanClusterRemoveShareRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanClusterRemoveShareRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("shareUuid"), &self.data.share_uuid as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.force else { continue; };
                    return Some((std::borrow::Cow::Borrowed("force"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanClusterRemoveShareSnapshotRequestType<'a> {
    share_uuid: &'a str,
    snapshot_name: &'a str,
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for VsanClusterRemoveShareSnapshotRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanClusterRemoveShareSnapshotRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanClusterRemoveShareSnapshotRequestTypeSer<'b, 'a> {
    data: &'b VsanClusterRemoveShareSnapshotRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanClusterRemoveShareSnapshotRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanClusterRemoveShareSnapshotRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("shareUuid"), &self.data.share_uuid as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("snapshotName"), &self.data.snapshot_name as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanUpgradeFsvmRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for VsanUpgradeFsvmRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanUpgradeFsvmRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanUpgradeFsvmRequestTypeSer<'b, 'a> {
    data: &'b VsanUpgradeFsvmRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanUpgradeFsvmRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanUpgradeFsvmRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
