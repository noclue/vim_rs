use std::sync::Arc;
use crate::core::client::{Client, Result};
/// The managed object provides support for vSAN remote datastore management
/// operations.
/// 
/// It can be accessed with MOID of 'vsan-remote-datastore-system',
/// through vSAN service at vCenter server side.
#[derive(Clone)]
pub struct VsanRemoteDatastoreSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl VsanRemoteDatastoreSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Create a new Datastore Source.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### datastore_source
    /// The information of the Datastore Source to be
    /// created. If the vCenter is an ELM linked vCenter, only
    /// *VsanRemoteVcInfo.vcHost*
    /// needs to be specified. For a standalone vCenter, the
    /// *VsanRemoteVcInfoStandalone.user* and
    /// *VsanRemoteVcInfoStandalone.password*
    /// needs to be specified for an user credential with
    /// the privileges to create the service account.
    ///
    /// ## Returns:
    ///
    /// The task for creating Datastore Source operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***AlreadyExists***: if the Datastore Source being created has existed.
    /// 
    /// ***VsanSslVerifyCertFault***: SSL verification fault for remote vCenter
    /// such as certificate not verified.
    /// 
    /// ***VsanFault***: Other vSAN related faults.
    pub async fn vsan_create_datastore_source(&self, datastore_source: &crate::types::structs::VsanHciMeshDatastoreSource) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanCreateDatastoreSourceRequestType {datastore_source, };
        let path = format!("/vsan/VsanRemoteDatastoreSystem/{moId}/VsanCreateDatastoreSource", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Destroy an existing Datastore Source configuration.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### datastore_source
    /// The Datastore Source to be destroyed. If the vCenter
    /// is an ELM linked vCenter, only
    /// *VsanRemoteVcInfo.vcHost*
    /// needs to be specified. For a standalone vCenter, the
    /// *VsanRemoteVcInfoStandalone.user* and
    /// *VsanRemoteVcInfoStandalone.password*
    /// needs to be specified for an user credential with
    /// the privileges to delete the service account.
    ///
    /// ## Returns:
    ///
    /// The task for destroying Datastore Source operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: The specified Datastore Source is not found.
    /// 
    /// ***ResourceInUse***: The specified Datastore Source still has datastores
    /// being mounted.
    /// 
    /// ***VsanFault***: Other vSAN related faults.
    pub async fn vsan_destroy_datastore_source(&self, datastore_source: &crate::types::structs::VsanHciMeshDatastoreSource) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanDestroyDatastoreSourceRequestType {datastore_source, };
        let path = format!("/vsan/VsanRemoteDatastoreSystem/{moId}/VsanDestroyDatastoreSource", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Checks mount compatibility of a vSAN datastore with given vSAN cluster.
    /// 
    /// Get mount pre-check results of a client cluster and remote
    /// vSAN datastore. Different types of checks are needed, see
    /// *VsanMountPrecheckResult* for details.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Client cluster that triggers the mount precheck request.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### datastore
    /// The server vSAN datastore to be checked for mount.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ### server_cluster_info
    /// The server cluster information of the vSAN
    /// datastore to be checked for mount. It's used
    /// for stretched cluster and remote
    /// data-in-transit configuration check.
    ///
    /// ## Returns:
    ///
    /// Pre-check results of a client cluster mounting server vSAN datastore.
    pub async fn mount_precheck(&self, cluster: &crate::types::structs::ManagedObjectReference, datastore: &crate::types::structs::ManagedObjectReference, server_cluster_info: Option<&crate::types::structs::VcRemoteVsanServerClusterInfo>) -> Result<Box<dyn crate::types::traits::VsanMountPrecheckResultTrait>> {
        let input = MountPrecheckRequestType {cluster, datastore, server_cluster_info, };
        let path = format!("/vsan/VsanRemoteDatastoreSystem/{moId}/MountPrecheck", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Run prechecks for a Datastore Source.
    /// 
    /// This can be used before creating,
    /// updating, destroying a Datastore Source or other places needing to verify
    /// a Datastore Source.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### datastore_source
    /// The information of the Datastore Source to be
    /// prechecked.
    ///
    /// ### operation
    /// The hint of the operation which the precheck is performed
    /// against, see
    /// *PrecheckDatastoreSourceOperation_enum*.
    ///
    /// ## Returns:
    ///
    /// Pre-check results of the specified Datastore Source.
    ///
    /// ## Errors:
    ///
    /// ***VsanSslVerifyCertFault***: SSL verification fault for remote vCenter
    /// such as certificate not verified.
    /// 
    /// ***VsanFault***: Other vSAN related faults.
    pub async fn vsan_precheck_datastore_source(&self, datastore_source: &crate::types::structs::VsanHciMeshDatastoreSource, operation: Option<&str>) -> Result<crate::types::structs::VsanDatastoreSourcePrecheckResult> {
        let input = VsanPrecheckDatastoreSourceRequestType {datastore_source, operation, };
        let path = format!("/vsan/VsanRemoteDatastoreSystem/{moId}/VsanPrecheckDatastoreSource", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Query Datastore Source information for specified remote vCenters.
    /// 
    /// If no vCenter is specified, all Datastore Sources configured are returned.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### vc_hosts
    /// The names (e.g. FQDNs) of the remote vCenters to be queried.
    /// Only those names configured through
    /// *VsanRemoteDatastoreSystem.VsanCreateDatastoreSource*
    /// are valid to be used.
    ///
    /// ## Returns:
    ///
    /// The Datastore Sources information queried.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: vSAN related faults.
    pub async fn vsan_query_datastore_source(&self, vc_hosts: Option<&[String]>) -> Result<Option<Vec<crate::types::structs::VsanHciMeshDatastoreSource>>> {
        let input = VsanQueryDatastoreSourceRequestType {vc_hosts, };
        let path = format!("/vsan/VsanRemoteDatastoreSystem/{moId}/VsanQueryDatastoreSource", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// This method takes a list of VsanXvcQuerySpec, then returns a list of
    /// VsanXvcQueryResultSet.
    /// 
    /// Each element of returned VsanXvcQueryResultSet
    /// list maps to the query spec in the VsanXvcQuerySpec list by index.
    /// Caller can provide a proper VsanXvcQuerySpec, and specify the required
    /// properties to be returned. The full list of the properties for each kind
    /// of returned object is described in VsanXvcQueryResultSet.
    /// Optionally, user can provide a list of remote VCs' information
    /// *VsanRemoteVcInfo* to specify the remote VC to query.
    /// The supported objects are listed in
    /// *VsanXvcQuerySpec.objectModel*.
    /// Each item is one datastore's info mapping to the specified properties
    /// *VsanXvcQuerySpec.properties* in spec.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### query_specs
    /// The spec information to specify what to be queried.
    ///
    /// ### extra_vc_infos
    /// The remote vCenter information for communication if
    /// caller needs to explicitly specify a remote vCenter
    /// as a provider vCenter which is not configured as a
    /// Datastore Source configuration. The user credential
    /// needs to be provided in the remote vCenter
    /// information, e.g., use
    /// *VsanRemoteVcInfoStandalone*.
    ///
    /// ## Returns:
    ///
    /// The query result information per the query specs.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: vSAN related faults.
    pub async fn vsan_query_hci_mesh_datastores(&self, query_specs: Option<&[crate::types::structs::VsanXvcQuerySpec]>, extra_vc_infos: Option<&[Box<dyn crate::types::traits::VsanRemoteVcInfoTrait>]>) -> Result<Option<Vec<crate::types::structs::VsanXvcQueryResultSet>>> {
        let input = VsanQueryHciMeshDatastoresRequestType {query_specs, extra_vc_infos, };
        let path = format!("/vsan/VsanRemoteDatastoreSystem/{moId}/VsanQueryHciMeshDatastores", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Checks mount compatibility of a cross VC vSAN datastore with given vSAN
    /// cluster.
    /// 
    /// Get mount pre-check results of a client cluster and cross VC remote
    /// vSAN datastore. Different types of checks are needed, see
    /// *VsanMountPrecheckResult* for details.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Client cluster that trigger the mount precheck request.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ### xvc_datastore
    /// -
    ///
    /// ## Returns:
    ///
    /// Pre-check results of a client cluster mounting cross VC vSAN datastore.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: vSAN related faults.
    pub async fn remote_vc_mount_precheck(&self, cluster: &crate::types::structs::ManagedObjectReference, xvc_datastore: &crate::types::structs::VsanXvcDatastoreInfo) -> Result<Box<dyn crate::types::traits::VsanMountPrecheckResultTrait>> {
        let input = RemoteVcMountPrecheckRequestType {cluster, xvc_datastore, };
        let path = format!("/vsan/VsanRemoteDatastoreSystem/{moId}/RemoteVcMountPrecheck", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Update the configuration of an existing Datastore Source.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### datastore_source
    /// The information of the Datastore Source to be
    /// updated. The updated Datastore Source is
    /// identified by the vcHost specified in the
    /// remote vCenter info of this param.
    /// If the original or updated vCenter is a standalone
    /// vCenter, the
    /// *VsanRemoteVcInfoStandalone.user* and
    /// *VsanRemoteVcInfoStandalone.password*
    /// needs to be specified for an user credential with
    /// the privilege to update the service account.
    /// If both original and updated vCenters are ELM
    /// vCenters only *VsanRemoteVcInfo.vcHost*
    /// needs to be specified.
    ///
    /// ## Returns:
    ///
    /// The task for updating Datastore Source operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: The specified Datastore Source is not found.
    /// 
    /// ***VsanFault***: Other vSAN related faults.
    pub async fn vsan_update_datastore_source(&self, datastore_source: &crate::types::structs::VsanHciMeshDatastoreSource) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanUpdateDatastoreSourceRequestType {datastore_source, };
        let path = format!("/vsan/VsanRemoteDatastoreSystem/{moId}/VsanUpdateDatastoreSource", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanCreateDatastoreSourceRequestType<'a> {
    #[serde(rename = "datastoreSource")]
    datastore_source: &'a crate::types::structs::VsanHciMeshDatastoreSource,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanDestroyDatastoreSourceRequestType<'a> {
    #[serde(rename = "datastoreSource")]
    datastore_source: &'a crate::types::structs::VsanHciMeshDatastoreSource,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct MountPrecheckRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serverClusterInfo")]
    server_cluster_info: Option<&'a crate::types::structs::VcRemoteVsanServerClusterInfo>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanPrecheckDatastoreSourceRequestType<'a> {
    #[serde(rename = "datastoreSource")]
    datastore_source: &'a crate::types::structs::VsanHciMeshDatastoreSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    operation: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryDatastoreSourceRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vcHosts")]
    vc_hosts: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanQueryHciMeshDatastoresRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "querySpecs")]
    query_specs: Option<&'a [crate::types::structs::VsanXvcQuerySpec]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "extraVcInfos")]
    extra_vc_infos: Option<&'a [Box<dyn crate::types::traits::VsanRemoteVcInfoTrait>]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoteVcMountPrecheckRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "xvcDatastore")]
    xvc_datastore: &'a crate::types::structs::VsanXvcDatastoreInfo,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanUpdateDatastoreSourceRequestType<'a> {
    #[serde(rename = "datastoreSource")]
    datastore_source: &'a crate::types::structs::VsanHciMeshDatastoreSource,
}
