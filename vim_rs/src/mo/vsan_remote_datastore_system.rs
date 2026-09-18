use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The managed object provides support for vSAN remote datastore management
/// operations.
/// 
/// It can be accessed with MOID of 'vsan-remote-datastore-system',
/// through vSAN service at vCenter server side.
#[derive(Clone)]
pub struct VsanRemoteDatastoreSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanRemoteDatastoreSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
        let bytes = self.client.invoke("vsan", "VsanRemoteDatastoreSystem", &self.mo_id, "VsanCreateDatastoreSource", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "VsanRemoteDatastoreSystem", &self.mo_id, "VsanDestroyDatastoreSource", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "VsanRemoteDatastoreSystem", &self.mo_id, "MountPrecheck", Some(&input)).await?;
        let result: Box<dyn crate::types::traits::VsanMountPrecheckResultTrait> = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "VsanRemoteDatastoreSystem", &self.mo_id, "VsanPrecheckDatastoreSource", Some(&input)).await?;
        let result: crate::types::structs::VsanDatastoreSourcePrecheckResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes_opt = self.client.invoke_optional("vsan", "VsanRemoteDatastoreSystem", &self.mo_id, "VsanQueryDatastoreSource", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
        let bytes_opt = self.client.invoke_optional("vsan", "VsanRemoteDatastoreSystem", &self.mo_id, "VsanQueryHciMeshDatastores", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    /// ### server_cluster_info
    /// Server cluster information of the remote VC
    /// vSAN datastore to be checked for mount.
    ///
    /// ## Returns:
    ///
    /// Pre-check results of a client cluster mounting cross VC vSAN datastore.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: vSAN related faults.
    pub async fn remote_vc_mount_precheck(&self, cluster: &crate::types::structs::ManagedObjectReference, xvc_datastore: &crate::types::structs::VsanXvcDatastoreInfo, server_cluster_info: Option<&crate::types::structs::VcRemoteVsanServerClusterInfo>) -> Result<Box<dyn crate::types::traits::VsanMountPrecheckResultTrait>> {
        let input = RemoteVcMountPrecheckRequestType {cluster, xvc_datastore, server_cluster_info, };
        let bytes = self.client.invoke("vsan", "VsanRemoteDatastoreSystem", &self.mo_id, "RemoteVcMountPrecheck", Some(&input)).await?;
        let result: Box<dyn crate::types::traits::VsanMountPrecheckResultTrait> = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("vsan", "VsanRemoteDatastoreSystem", &self.mo_id, "VsanUpdateDatastoreSource", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct VsanCreateDatastoreSourceRequestType<'a> {
    datastore_source: &'a crate::types::structs::VsanHciMeshDatastoreSource,
}

impl<'a> miniserde::Serialize for VsanCreateDatastoreSourceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanCreateDatastoreSourceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanCreateDatastoreSourceRequestTypeSer<'b, 'a> {
    data: &'b VsanCreateDatastoreSourceRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanCreateDatastoreSourceRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanCreateDatastoreSourceRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastoreSource"), &self.data.datastore_source as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanDestroyDatastoreSourceRequestType<'a> {
    datastore_source: &'a crate::types::structs::VsanHciMeshDatastoreSource,
}

impl<'a> miniserde::Serialize for VsanDestroyDatastoreSourceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanDestroyDatastoreSourceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanDestroyDatastoreSourceRequestTypeSer<'b, 'a> {
    data: &'b VsanDestroyDatastoreSourceRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanDestroyDatastoreSourceRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanDestroyDatastoreSourceRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastoreSource"), &self.data.datastore_source as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct MountPrecheckRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    datastore: &'a crate::types::structs::ManagedObjectReference,
    server_cluster_info: Option<&'a crate::types::structs::VcRemoteVsanServerClusterInfo>,
}

impl<'a> miniserde::Serialize for MountPrecheckRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MountPrecheckRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MountPrecheckRequestTypeSer<'b, 'a> {
    data: &'b MountPrecheckRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for MountPrecheckRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MountPrecheckRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.server_cluster_info else { continue; };
                    return Some((std::borrow::Cow::Borrowed("serverClusterInfo"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanPrecheckDatastoreSourceRequestType<'a> {
    datastore_source: &'a crate::types::structs::VsanHciMeshDatastoreSource,
    operation: Option<&'a str>,
}

impl<'a> miniserde::Serialize for VsanPrecheckDatastoreSourceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanPrecheckDatastoreSourceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanPrecheckDatastoreSourceRequestTypeSer<'b, 'a> {
    data: &'b VsanPrecheckDatastoreSourceRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanPrecheckDatastoreSourceRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanPrecheckDatastoreSourceRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("datastoreSource"), &self.data.datastore_source as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.operation else { continue; };
                    return Some((std::borrow::Cow::Borrowed("operation"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanQueryDatastoreSourceRequestType<'a> {
    vc_hosts: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for VsanQueryDatastoreSourceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryDatastoreSourceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryDatastoreSourceRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryDatastoreSourceRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanQueryDatastoreSourceRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryDatastoreSourceRequestType")),
                1 => {
                    let Some(ref val) = self.data.vc_hosts else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vcHosts"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanQueryHciMeshDatastoresRequestType<'a> {
    query_specs: Option<&'a [crate::types::structs::VsanXvcQuerySpec]>,
    extra_vc_infos: Option<&'a [Box<dyn crate::types::traits::VsanRemoteVcInfoTrait>]>,
}

impl<'a> miniserde::Serialize for VsanQueryHciMeshDatastoresRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryHciMeshDatastoresRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryHciMeshDatastoresRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryHciMeshDatastoresRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanQueryHciMeshDatastoresRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryHciMeshDatastoresRequestType")),
                1 => {
                    let Some(ref val) = self.data.query_specs else { continue; };
                    return Some((std::borrow::Cow::Borrowed("querySpecs"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.extra_vc_infos else { continue; };
                    return Some((std::borrow::Cow::Borrowed("extraVcInfos"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RemoteVcMountPrecheckRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    xvc_datastore: &'a crate::types::structs::VsanXvcDatastoreInfo,
    server_cluster_info: Option<&'a crate::types::structs::VcRemoteVsanServerClusterInfo>,
}

impl<'a> miniserde::Serialize for RemoteVcMountPrecheckRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoteVcMountPrecheckRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoteVcMountPrecheckRequestTypeSer<'b, 'a> {
    data: &'b RemoteVcMountPrecheckRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoteVcMountPrecheckRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoteVcMountPrecheckRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("xvcDatastore"), &self.data.xvc_datastore as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.server_cluster_info else { continue; };
                    return Some((std::borrow::Cow::Borrowed("serverClusterInfo"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanUpdateDatastoreSourceRequestType<'a> {
    datastore_source: &'a crate::types::structs::VsanHciMeshDatastoreSource,
}

impl<'a> miniserde::Serialize for VsanUpdateDatastoreSourceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanUpdateDatastoreSourceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanUpdateDatastoreSourceRequestTypeSer<'b, 'a> {
    data: &'b VsanUpdateDatastoreSourceRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanUpdateDatastoreSourceRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanUpdateDatastoreSourceRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastoreSource"), &self.data.datastore_source as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
