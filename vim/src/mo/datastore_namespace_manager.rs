use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::DatastoreNamespaceManagerDirectoryInfo;
use crate::types::structs::ManagedObjectReference;
/// The DatastoreNamespaceManager managed object exposes APIs for
/// manipulating top-level directories of datastores which do not
/// support the traditional top-level directory creation.
/// 
/// See also *DatastoreCapability.topLevelDirectoryCreateSupported*.
pub struct DatastoreNamespaceManager {
    client: Arc<Client>,
    mo_id: String,
}
impl DatastoreNamespaceManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
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
    /// Namesapce URL of the form
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
    pub async fn convert_namespace_path_to_uuid_path(&self, datacenter: Option<&ManagedObjectReference>, namespace_url: &str) -> Result<String> {
        let input = ConvertNamespacePathToUuidPathRequestType {datacenter, namespace_url, };
        let path = format!("/DatastoreNamespaceManager/{moId}/ConvertNamespacePathToUuidPath", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn create_directory(&self, datastore: &ManagedObjectReference, display_name: Option<&str>, policy: Option<&str>, size: Option<i64>) -> Result<String> {
        let input = CreateDirectoryRequestType {datastore, display_name, policy, size, };
        let path = format!("/DatastoreNamespaceManager/{moId}/CreateDirectory", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn delete_directory(&self, datacenter: Option<&ManagedObjectReference>, datastore_path: &str) -> Result<()> {
        let input = DeleteDirectoryRequestType {datacenter, datastore_path, };
        let path = format!("/DatastoreNamespaceManager/{moId}/DeleteDirectory", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn increase_directory_size(&self, datacenter: Option<&ManagedObjectReference>, stable_name: &str, size: i64) -> Result<()> {
        let input = IncreaseDirectorySizeRequestType {datacenter, stable_name, size, };
        let path = format!("/DatastoreNamespaceManager/{moId}/IncreaseDirectorySize", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn query_directory_info(&self, datacenter: Option<&ManagedObjectReference>, stable_name: &str) -> Result<DatastoreNamespaceManagerDirectoryInfo> {
        let input = QueryDirectoryInfoRequestType {datacenter, stable_name, };
        let path = format!("/DatastoreNamespaceManager/{moId}/QueryDirectoryInfo", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ConvertNamespacePathToUuidPathRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
    #[serde(rename = "namespaceUrl")]
    namespace_url: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateDirectoryRequestType<'a> {
    datastore: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "displayName")]
    display_name: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    policy: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    size: Option<i64>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DeleteDirectoryRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
    #[serde(rename = "datastorePath")]
    datastore_path: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct IncreaseDirectorySizeRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
    #[serde(rename = "stableName")]
    stable_name: &'a str,
    size: i64,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryDirectoryInfoRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
    #[serde(rename = "stableName")]
    stable_name: &'a str,
}
