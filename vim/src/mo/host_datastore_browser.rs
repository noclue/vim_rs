use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::FileQueryTrait;
use crate::types::structs::HostDatastoreBrowserSearchSpec;
use crate::types::structs::ManagedObjectReference;
/// The DatastoreBrowser managed object type provides access to the contents of one or
/// more datastores.
/// 
/// The items in a datastore are files that contain configuration,
/// virtual disk, and the other data associated with a virtual machine.
/// 
/// Although datastores may often be implemented using a traditional file system, a full
/// interface to a file system is not provided here. Instead, specialized access for
/// virtual machine files is provided. A datastore implementation may completely hide the
/// file directory structure.
/// 
/// The intent is to provide functionality analogous to a file chooser in a user
/// interface.
/// 
/// Files on datastores do not have independent permissions through this API. Instead,
/// the permissions for all the files on a datastore derive from the datastore object
/// itself. It is not possible to change individual file permissions as the user browsing
/// the datastore may not necessarily be a recognized user from the point of view of the
/// host changing the permission. This occurs if the user browsing the datastore is doing
/// so through the VirtualCenter management server.
/// 
/// The DatastoreBrowser provides many ways to customize a search for files. A search can
/// be customized by specifying the types of files to be searched, search criteria
/// specific to a file type, and the amount of detail about each file. The most basic
/// queries only use file details and are efficient with limited side effects. For these
/// queries, file metadata details can be optionally retrieved, but the files themselves
/// are opened and their contents examined. As a result, these files are not necessarily
/// validated.
/// 
/// More complicated queries can be formed by specifying the specific types of files to
/// be searched, the parameters to filter for each type, and the desired level of detail
/// about each file. This method of searching for files is convenient because it allows
/// additional data about the virtual machine component to be retrieved. In addition,
/// certain validation checks can be performed on matched files as an inherent part of
/// the details collection process. However, gathering extra details or the use of type
/// specific filters can sometimes only be implemented by examining the contents of a
/// file. As a result, the use of these conveniences comes with the cost of additional
/// latency in the request and possible side effects on the system as a whole.
/// 
/// The DatastoreBrowser is referenced from various objects, including from
/// *Datastore*, *ComputeResource*, *HostSystem* and
/// *VirtualMachine*. Depending on which object is used, there are different
/// requirements for the accessibility of the browsed datastore from the host (or hosts)
/// associated with the object:
/// - When referenced from the target *Datastore*, it needs to be
///   accessible from at least one host on which the datastore is mounted.
///   See *DatastoreSummary.accessible*.
/// - When referenced from a *ComputeResource*, the target datastore
///   needs to be accessible from at least one host in the ComputeResource.
///   See *HostMountInfo.accessible*.
/// - When referenced from a *HostSystem*, the target datastore needs
///   to be accessible from that host. See *HostMountInfo.accessible*.
/// - When referenced from a *VirtualMachine*, the target datastore
///   needs to be accessible from the host on which the virtual machine is
///   registered. See *HostMountInfo.accessible*.
///   
/// See also *FileInfo*.
pub struct HostDatastoreBrowser {
    client: Arc<Client>,
    mo_id: String,
}
impl HostDatastoreBrowser {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Deprecated as of VI API 2.5, use *FileManager.DeleteDatastoreFile_Task*.
    /// 
    /// Deletes the specified files from the datastore.
    /// 
    /// If a valid virtual disk file is
    /// specified, then all the components of the virtual disk are deleted.
    /// 
    /// ***Required privileges:*** Datastore.DeleteFile
    ///
    /// ## Parameters:
    ///
    /// ### datastore_path
    /// -
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: if the operation cannot be performed on the target
    /// datastores. Typically, a specific subclass of this exception is thrown.
    /// 
    /// ***FileNotFound***: if the file or folder specified by datastorePath is not
    /// found.
    /// 
    /// ***CannotDeleteFile***: if the delete operation on the file fails.
    /// 
    /// ***InvalidArgument***: if fileInfo is not a valid FileInfo type.
    pub async fn delete_file(&self, datastore_path: &str) -> Result<()> {
        let input = DeleteFileRequestType {datastore_path, };
        let path = format!("/HostDatastoreBrowser/{moId}/DeleteFile", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Returns the information for the files that match the given search criteria as a
    /// SearchResults object.
    /// 
    /// Searches only the folder specified by the datastore path.
    /// The Datastore.Browse privilege must be held on the datastore identified
    /// by the datastore path.
    ///
    /// ## Parameters:
    ///
    /// ### datastore_path
    /// -
    ///
    /// ### search_spec
    /// -
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation. The *info.result* property in the
    /// *Task* contains the
    /// *HostDatastoreBrowserSearchResults* upon success.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: if the operation cannot be performed on the target
    /// datastores. The server can throw InvalidDatastorePath to indicate a malformed
    /// datastorePath, or InaccessibleDatastore to indicate inaccessibility of the
    /// datastore.
    /// 
    /// ***InvalidArgument***: if the SearchSpec contains duplicate file types.
    /// 
    /// ***FileNotFound***: if the file or folder specified by datastorePath is not
    /// found.
    pub async fn search_datastore_task(&self, datastore_path: &str, search_spec: Option<&HostDatastoreBrowserSearchSpec>) -> Result<ManagedObjectReference> {
        let input = SearchDatastoreRequestType {datastore_path, search_spec, };
        let path = format!("/HostDatastoreBrowser/{moId}/SearchDatastore_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Returns the information for the files that match the given search criteria as a
    /// SearchResults\[\] object.
    /// 
    /// Searches the folder specified by the datastore path and
    /// all subfolders. The Datastore.Browse privilege must be held on the
    /// datastore identified by the datastore path.
    ///
    /// ## Parameters:
    ///
    /// ### datastore_path
    /// -
    ///
    /// ### search_spec
    /// -
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation. The *info.result* property in the
    /// *Task* contains the
    /// *HostDatastoreBrowserSearchResults* upon success.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: if the operation cannot be performed on the target
    /// datastores. Typically, a specific subclass of this exception is thrown.
    /// 
    /// ***InvalidArgument***: if the SearchSpec contains duplicate file types.
    /// 
    /// ***FileNotFound***: if the file or folder specified by datastorePath is not
    /// found.
    pub async fn search_datastore_sub_folders_task(&self, datastore_path: &str, search_spec: Option<&HostDatastoreBrowserSearchSpec>) -> Result<ManagedObjectReference> {
        let input = SearchDatastoreSubFoldersRequestType {datastore_path, search_spec, };
        let path = format!("/HostDatastoreBrowser/{moId}/SearchDatastoreSubFolders_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Set of datastores that can be searched on this DatastoreBrowser.
    /// 
    /// The list of datastores available to browse on this DatastoreBrowser is contextual
    /// information that depends on the object being browsed. If the host is being
    /// browsed, the host's datastores are used. If the Datacenter is being browsed, the
    /// Datacenter's list of datastores is used.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *Datastore*.
    pub async fn datastore(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/HostDatastoreBrowser/{moId}/datastore", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// The list of supported file types.
    /// 
    /// The supported file types are represented as
    /// items in this list. For each supported file type, there is an object in the list
    /// whose dynamic type is one of the types derived from the
    /// *FileQuery* data object
    /// type. In general, the properties in this query type are not set.
    /// 
    /// Use the Query of the desired file type in the SearchSpec.query to indicate the
    /// desired file types.
    /// 
    /// This property is used by clients to determine what kinds of file types are
    /// supported. Clients should consult this list to avoid querying for types of virtual
    /// machine components that are not supported.
    pub async fn supported_type(&self) -> Result<Option<Vec<Box<dyn FileQueryTrait>>>> {
        let path = format!("/HostDatastoreBrowser/{moId}/supportedType", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DeleteFileRequestType<'a> {
    #[serde(rename = "datastorePath")]
    datastore_path: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SearchDatastoreRequestType<'a> {
    #[serde(rename = "datastorePath")]
    datastore_path: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "searchSpec")]
    search_spec: Option<&'a HostDatastoreBrowserSearchSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SearchDatastoreSubFoldersRequestType<'a> {
    #[serde(rename = "datastorePath")]
    datastore_path: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "searchSpec")]
    search_spec: Option<&'a HostDatastoreBrowserSearchSpec>,
}
