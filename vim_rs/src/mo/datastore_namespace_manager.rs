use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The DatastoreNamespaceManager managed object exposes APIs for
/// manipulating top-level directories of datastores which do not
/// support the traditional top-level directory creation.
/// 
/// See also *DatastoreCapability.topLevelDirectoryCreateSupported*.
#[derive(Clone)]
pub struct DatastoreNamespaceManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl DatastoreNamespaceManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Convert the namespace path to the namespace UUID path.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### datacenter
    /// The datacenter of the namespace path. Needs to be set
    /// when making the call to VC; ignored when the call is
    /// made to ESX.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### namespace_url
    /// Namespace URL of the form
    /// > \[ds://\]/vmfs/volumes/\[_datastore-uuid_\]/\[_directory-name_\]/...
    /// >
    ///
    /// ## Returns:
    ///
    /// the URL path with namespace object UUID, of the form
    /// > \[ds://\]/vmfs/volumes/\[_datastore-uuid_\]/\[_directory-uuid_\]/...
    /// >
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: if the given datastore is not supported by
    /// the DatastoreNamespaceManager
    /// 
    /// ***InvalidDatastorePath***: if the given path is not a top-level
    /// directory
    pub async fn convert_namespace_path_to_uuid_path(&self, datacenter: Option<&crate::types::structs::ManagedObjectReference>, namespace_url: &str) -> Result<String> {
        let input = ConvertNamespacePathToUuidPathRequestType {datacenter, namespace_url, };
        let bytes = self.client.invoke("", "DatastoreNamespaceManager", &self.mo_id, "ConvertNamespacePathToUuidPath", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Creates a top-level directory on the given datastore, using the given
    /// user display name hint and opaque storage policy.
    /// 
    /// The optional given display name hint may be used by the underlying
    /// storage system for user display purposes, but it may not be relied
    /// upon for future directory references.
    /// 
    /// Clients must use the returned stable path for future directory
    /// references.
    /// 
    /// See also *DatastoreNamespaceManager.DeleteDirectory*.
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// datastore on which to create a top-level directory
    /// 
    /// ***Required privileges:*** Datastore.Config
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ### display_name
    /// display name hint for the directory to create
    ///
    /// ### policy
    /// opaque storage policy to associate with the directory
    ///
    /// ### size
    /// directory size in MB on vvol/vsan backed object storage.
    /// default directory size will be used for vsan backed
    /// object storage if not set.
    /// 
    /// ***Since:*** vSphere API Release 7.0.2.0
    ///
    /// ## Returns:
    ///
    /// a stable vmfs path which may be used for future
    /// reference of the created directory, of the form
    /// > /vmfs/volumes/\[_datastore-uuid_\]/\[_directory-uuid_\]
    /// >
    ///
    /// ## Errors:
    ///
    /// ***CannotCreateFile***: if a general system error occurred while creating
    /// directory on the datastore
    /// 
    /// ***FileAlreadyExists***: if the given directory already exists
    /// 
    /// ***InvalidDatastore***: if the given datastore is not supported by
    /// the DatastoreNamespaceManage
    pub async fn create_directory(&self, datastore: &crate::types::structs::ManagedObjectReference, display_name: Option<&str>, policy: Option<&str>, size: Option<i64>) -> Result<String> {
        let input = CreateDirectoryRequestType {datastore, display_name, policy, size, };
        let bytes = self.client.invoke("", "DatastoreNamespaceManager", &self.mo_id, "CreateDirectory", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Deletes the given top-level directory from a datastore.
    /// 
    /// The top-level directory must be a full path of the form
    /// > /vmfs/volumes/\[_datastore-uuid_\]/\[_directory-uuid_\]
    /// > 
    /// 
    /// as returned by
    /// *DatastoreNamespaceManager.CreateDirectory*.
    /// 
    /// Requires Datastore.Config privilege on the datastore.
    /// 
    /// See also *DatastoreNamespaceManager.CreateDirectory*.
    ///
    /// ## Parameters:
    ///
    /// ### datacenter
    /// The datacenter of the datastore path. Needs to be set
    /// when making the call to VC; ignored when the call is
    /// made to ESX.
    /// 
    /// ***Required privileges:*** System.View
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### datastore_path
    /// Stable vmfs path of the directory to delete.
    ///
    /// ## Errors:
    ///
    /// ***FileNotFound***: if the given directory can not be found
    /// 
    /// ***FileFault***: if a generic system error happened.
    /// 
    /// ***InvalidDatastore***: if the given datastore is not supported by
    /// the DatastoreNamespaceManager
    /// 
    /// ***InvalidDatastorePath***: if the given path is not a top-level directory
    pub async fn delete_directory(&self, datacenter: Option<&crate::types::structs::ManagedObjectReference>, datastore_path: &str) -> Result<()> {
        let input = DeleteDirectoryRequestType {datacenter, datastore_path, };
        self.client.invoke_void("", "DatastoreNamespaceManager", &self.mo_id, "DeleteDirectory", Some(&input)).await
    }
    /// Increase size of the given top-level directory to the given size on
    /// vSAN backed object storage.
    /// 
    /// The top-level directory must be a full path in the form
    /// > /vmfs/volumes/\[_datastore-uuid_\]/\[_directory-uuid_\]
    /// > 
    /// 
    /// as returned by
    /// *DatastoreNamespaceManager.CreateDirectory*.
    /// 
    /// Requires Datastore.Config privilege on the datastore.
    /// 
    /// See also *DatastoreNamespaceManager.CreateDirectory*.
    /// 
    /// ***Since:*** vSphere API Release 8.0.1.0
    ///
    /// ## Parameters:
    ///
    /// ### datacenter
    /// the datacenter of the namespace path. Needs to be set
    /// when making the call to VC; ignored when the call is
    /// made to ESX.
    /// 
    /// ***Required privileges:*** System.View
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### stable_name
    /// stable vmfs path of the top-level directory
    ///
    /// ### size
    /// the desired final size in MB of the directory, not a diff
    /// from the current size; should be more than current size
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: if a generic system error happened
    /// 
    /// ***FileNotFound***: if the given directory can not be found
    /// 
    /// ***InvalidDatastore***: if the given datastore is not supported by
    /// the DatastoreNamespaceManager
    /// 
    /// ***NotSupported***: if resize is not supported on the directory
    /// 
    /// ***InvalidArgument***: if passed size is not valid
    pub async fn increase_directory_size(&self, datacenter: Option<&crate::types::structs::ManagedObjectReference>, stable_name: &str, size: i64) -> Result<()> {
        let input = IncreaseDirectorySizeRequestType {datacenter, stable_name, size, };
        self.client.invoke_void("", "DatastoreNamespaceManager", &self.mo_id, "IncreaseDirectorySize", Some(&input)).await
    }
    /// Query directory information of the given top-level directory on vSAN
    /// backed object storage.
    /// 
    /// The top-level directory must be a full path in the form
    /// > /vmfs/volumes/\[_datastore-uuid_\]/\[_directory-uuid_\]
    /// > 
    /// 
    /// as returned by
    /// *DatastoreNamespaceManager.CreateDirectory*.
    /// 
    /// See also *DatastoreNamespaceManager.CreateDirectory*.
    /// 
    /// ***Since:*** vSphere API Release 8.0.1.0
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### datacenter
    /// the datacenter of the namespace path. Needs to be set
    /// when making the call to VC; ignored when the call is
    /// made to ESX.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### stable_name
    /// stable vmfs path of the top-level directory to query
    ///
    /// ## Returns:
    ///
    /// directory information defined by *DatastoreNamespaceManagerDirectoryInfo*
    ///
    /// ## Errors:
    ///
    /// ***FileFault***: if a generic system error happened
    /// 
    /// ***FileNotFound***: if the given directory can not be found
    /// 
    /// ***InvalidDatastore***: if the given datastore is not supported by
    /// the DatastoreNamespaceManager
    /// 
    /// ***NotSupported***: if query is not supported on the directory
    pub async fn query_directory_info(&self, datacenter: Option<&crate::types::structs::ManagedObjectReference>, stable_name: &str) -> Result<crate::types::structs::DatastoreNamespaceManagerDirectoryInfo> {
        let input = QueryDirectoryInfoRequestType {datacenter, stable_name, };
        let bytes = self.client.invoke("", "DatastoreNamespaceManager", &self.mo_id, "QueryDirectoryInfo", Some(&input)).await?;
        let result: crate::types::structs::DatastoreNamespaceManagerDirectoryInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct ConvertNamespacePathToUuidPathRequestType<'a> {
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    namespace_url: &'a str,
}

impl<'a> miniserde::Serialize for ConvertNamespacePathToUuidPathRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ConvertNamespacePathToUuidPathRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ConvertNamespacePathToUuidPathRequestTypeSer<'b, 'a> {
    data: &'b ConvertNamespacePathToUuidPathRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ConvertNamespacePathToUuidPathRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ConvertNamespacePathToUuidPathRequestType")),
                1 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("namespaceUrl"), &self.data.namespace_url as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct CreateDirectoryRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
    display_name: Option<&'a str>,
    policy: Option<&'a str>,
    size: Option<i64>,
}

impl<'a> miniserde::Serialize for CreateDirectoryRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateDirectoryRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateDirectoryRequestTypeSer<'b, 'a> {
    data: &'b CreateDirectoryRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateDirectoryRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateDirectoryRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.display_name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("displayName"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.policy else { continue; };
                    return Some((std::borrow::Cow::Borrowed("policy"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.size else { continue; };
                    return Some((std::borrow::Cow::Borrowed("size"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct DeleteDirectoryRequestType<'a> {
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    datastore_path: &'a str,
}

impl<'a> miniserde::Serialize for DeleteDirectoryRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DeleteDirectoryRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DeleteDirectoryRequestTypeSer<'b, 'a> {
    data: &'b DeleteDirectoryRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DeleteDirectoryRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DeleteDirectoryRequestType")),
                1 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("datastorePath"), &self.data.datastore_path as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct IncreaseDirectorySizeRequestType<'a> {
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    stable_name: &'a str,
    size: i64,
}

impl<'a> miniserde::Serialize for IncreaseDirectorySizeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(IncreaseDirectorySizeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct IncreaseDirectorySizeRequestTypeSer<'b, 'a> {
    data: &'b IncreaseDirectorySizeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for IncreaseDirectorySizeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"IncreaseDirectorySizeRequestType")),
                1 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("stableName"), &self.data.stable_name as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("size"), &self.data.size as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct QueryDirectoryInfoRequestType<'a> {
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    stable_name: &'a str,
}

impl<'a> miniserde::Serialize for QueryDirectoryInfoRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryDirectoryInfoRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryDirectoryInfoRequestTypeSer<'b, 'a> {
    data: &'b QueryDirectoryInfoRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryDirectoryInfoRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryDirectoryInfoRequestType")),
                1 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("stableName"), &self.data.stable_name as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
