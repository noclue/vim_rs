use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object type provides a way to manage and manipulate files and
/// folders on datastores.
/// 
/// The source and the destination names are in the form of
/// a URL or a datastore path.
/// 
/// A URL has the form
/// > _scheme_://_authority_/folder/_path_?dcPath=_dcPath_&amp;dsName=_dsName_
/// 
/// where
/// - _scheme_ is <code>http</code> or <code>https</code>.
/// - _authority_ specifies the hostname or IP address of the VirtualCenter or
///   ESX server and optionally the port.
/// - _dcPath_ is the inventory path to the Datacenter containing the
///   Datastore.
/// - _dsName_ is the name of the Datastore.
/// - _path_ is a slash-delimited path from the root of the datastore.
///   
///   A datastore path has the form
///   > \[_datastore_\] _path_
///   
///   where
/// - _datastore_ is the datastore name.
/// - _path_ is a slash-delimited path from the root of the datastore.
/// 
/// An example datastore path is "\[storage\] path/to/file.extension".
/// A listing of all the files, disks and folders on
/// a datastore can be obtained from the datastore browser.
/// 
/// See also *HostDatastoreBrowser*.
#[derive(Clone)]
pub struct FileManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl FileManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Change the owner for a file.
    /// 
    /// This method is currently not supported.
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// -
    ///
    /// ### datacenter
    /// ***Required privileges:*** System.View
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### owner
    /// -
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn change_owner(&self, name: &str, datacenter: Option<&crate::types::structs::ManagedObjectReference>, owner: &str) -> Result<()> {
        let input = ChangeOwnerRequestType {name, datacenter, owner, };
        let path = format!("/FileManager/{moId}/ChangeOwner", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Copies the source file or folder to the destination.
    /// 
    /// If the destination file does not exist, it is created.
    /// If the destination file exists, the force parameter determines whether
    /// to overwrite it with the source or not.
    /// Folders can be copied recursively. In this case, the
    /// destination, if it exists, must be a folder, else one will be created. Existing
    /// files on the destination that conflict with source files can be overwritten using
    /// the force parameter. Files and disks are always copied in binary format during
    /// recursive copy.
    /// 
    /// If source (or destination) name is specified as a URL, then the
    /// corresponding datacenter parameter may be omitted.
    /// 
    /// If any intermediate level folder specified by the source and destination
    /// does not exist, a *FileNotFound* fault is thrown.
    /// 
    /// If a file of a virtual machine is overwritten on the destination datastore
    /// as a result of the force parameter, it may corrupt that virtual machine.
    /// 
    /// If the source is an extent of a virtual disk, this operation treats the extent
    /// as a file.
    /// 
    /// If source and destination resolve to the same file system location,
    /// the call has no effect.
    /// 
    /// It is important to note that this operation will provide transactional guarantees
    /// only for a file. No guarantees are provided when copying a folder.
    /// If the intent is to clone a virtual machine registered in the inventory,
    /// with transactional guarantees, please refer to *VirtualMachine.CloneVM_Task*.
    /// 
    /// Datastore.FileManagement privilege is required on both source and
    /// destination datastores.
    ///
    /// ## Parameters:
    ///
    /// ### source_name
    /// The name of the source, either a URL or a
    /// datastore path referring to the file or folder to be copied.
    ///
    /// ### source_datacenter
    /// If <code>sourceName</code> is a datastore path, the
    /// datacenter for that datastore path.
    /// Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter,
    /// <code>sourceName</code> must be a URL.
    /// 
    /// ***Required privileges:*** System.View
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### destination_name
    /// The name of the destination, either a
    /// URL or a datastore path referring to the destination file
    /// or folder.
    ///
    /// ### destination_datacenter
    /// If <code>destinationName</code> is a datastore
    /// path, the datacenter for that datastore path.
    /// Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter, it is assumed that
    /// the destination path belongs to the source datacenter.
    /// 
    /// ***Required privileges:*** System.View
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### force
    /// If true, overwrite any identically named file
    /// at the destination. If not specified, it is assumed to be false.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: if the operation cannot be performed on the source
    /// or destination datastores. Typically, a specific subclass of this exception is
    /// thrown.
    /// 
    /// ***FileNotFound***: if the file or folder specified by sourceName is not
    /// found, or, any intermediate level folder specified by the destinationName is not
    /// found.
    /// 
    /// ***CannotAccessFile***: if the source cannot be accessed because of
    /// insufficient permissions.
    /// 
    /// ***FileLocked***: if the source file or folder is currently locked or in use.
    /// 
    /// ***FileAlreadyExists***: if a file with the given name already
    /// exists at the destination, and the force flag is false.
    /// 
    /// ***NoDiskSpace***: if there is not enough space available at the destination
    /// datastore.
    /// 
    /// ***FileFault***: if there is a generic file error
    pub async fn copy_datastore_file_task(&self, source_name: &str, source_datacenter: Option<&crate::types::structs::ManagedObjectReference>, destination_name: &str, destination_datacenter: Option<&crate::types::structs::ManagedObjectReference>, force: Option<bool>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CopyDatastoreFileRequestType {source_name, source_datacenter, destination_name, destination_datacenter, force, };
        let path = format!("/FileManager/{moId}/CopyDatastoreFile_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deletes the specified file or folder from the datastore.
    /// 
    /// If a file of a virtual machine is deleted, it may corrupt that
    /// virtual machine. Folder deletes are always recursive.
    /// The datacenter parameter may be omitted if a URL is used to name
    /// the file or folder.
    /// 
    /// If the source is an extent of a virtual disk, this operation treats the extent
    /// as a file.
    /// 
    /// It is important to note that this operation will provide transactional guarantees
    /// only for a file. No guarantees are provided when deleting folders.
    /// If the intent is to delete a virtual machine registered in the inventory,
    /// please refer to *ManagedEntity.Destroy_Task*.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore.
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of the file or folder, either a URL or a datastore path
    /// referring to the file or folder to be deleted.
    ///
    /// ### datacenter
    /// If <code>name</code> is a datastore path, the datacenter for
    /// that datastore path. Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter,
    /// <code>name</code> must be a URL.
    /// 
    /// ***Required privileges:*** System.View
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: if the operation cannot be performed on the
    /// datastore. Typically, a specific subclass of this exception is thrown.
    /// 
    /// ***FileNotFound***: if the file or folder specified by name
    /// is not found.
    /// 
    /// ***CannotDeleteFile***: if the delete operation on the file or folder
    /// fails.
    /// 
    /// ***FileLocked***: if the source file or folder is currently locked or
    /// in use.
    /// 
    /// ***FileFault***: if there is a generic file error
    pub async fn delete_datastore_file_task(&self, name: &str, datacenter: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = DeleteDatastoreFileRequestType {name, datacenter, };
        let path = format!("/FileManager/{moId}/DeleteDatastoreFile_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Create a folder using the specified name.
    /// 
    /// If the parent
    /// or intermediate level folders do not exist, and the parameter
    /// createParentDirectories is false, a *FileNotFound* fault
    /// is thrown.
    /// If the intermediate level folders do not exist, and the parameter
    /// createParentDirectories is true, all the non-existent folders
    /// are created.
    /// 
    /// Requires Datastore.FileManagement privilege on the datastore.
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The name of the folder, either a URL or a datastore path
    /// referring to the folder to be created.
    ///
    /// ### datacenter
    /// If <code>name</code> is a datastore path, the datacenter for
    /// that datastore path. Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter,
    /// <code>name</code> must be a URL.
    /// 
    /// ***Required privileges:*** System.View
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### create_parent_directories
    /// If true, any non-existent intermediate level
    /// folders will be created. If not specified,
    /// it is assumed to be false.
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: if the operation cannot be performed on the
    /// datastore. Typically, a specific subclass of this exception is thrown.
    /// 
    /// ***CannotCreateFile***: if the create operation on the folder fails.
    /// 
    /// ***FileAlreadyExists***: if a file or folder with the given name already
    /// exists at the destination.
    /// 
    /// ***FileNotFound***: if the createParentDirectories is false and a intermediate
    /// level folder specified by name is not found.
    /// 
    /// ***FileFault***: if there is a generic file error
    pub async fn make_directory(&self, name: &str, datacenter: Option<&crate::types::structs::ManagedObjectReference>, create_parent_directories: Option<bool>) -> Result<()> {
        let input = MakeDirectoryRequestType {name, datacenter, create_parent_directories, };
        let path = format!("/FileManager/{moId}/MakeDirectory", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Moves the source file or folder to the destination.
    /// 
    /// If the destination file does not exist, it is created.
    /// If the destination file exists, the force parameter determines whether
    /// to overwrite it with the source or not.
    /// If the source path is a folder, then the destination path must not exist; the
    /// destination cannot be overwritten even with a force flag in that case. Folder
    /// moves are recursive, treating all files and disks to move as binary.
    /// 
    /// If source (or destination) name is specified as a URL, then the
    /// corresponding datacenter parameter may be omitted.
    /// 
    /// If any intermediate level folder specified by the source and destination
    /// does not exist, a *FileNotFound* fault is thrown.
    /// 
    /// If a file of a virtual machine is moved, it may corrupt that virtual machine.
    /// If a file of a virtual machine is overwritten on the destination datastore
    /// as a result of the force parameter, it may corrupt that virtual machine.
    /// 
    /// If the source is an extent of a virtual disk, this operation treats the extent
    /// as a file.
    /// 
    /// If source and destination resolve to the same file system location,
    /// the call has no effect.
    /// 
    /// It is important to note that this operation will provide transactional guarantees
    /// only for a file. No guarantees are provided for folder moves. If the intent is
    /// to move a virtual machine registered in the inventory, with transactional
    /// guarantees, please refer to *VirtualMachine.RelocateVM_Task*.
    /// If the intent is to rename a virtual machine registered in the inventory, please
    /// refer to *ManagedEntity.Rename_Task*.
    /// 
    /// Datastore.FileManagement privilege is required on both source and
    /// destination datastores.
    ///
    /// ## Parameters:
    ///
    /// ### source_name
    /// The name of the source, either a URL or a datastore path
    /// referring to the file or folder to be moved.
    ///
    /// ### source_datacenter
    /// If <code>sourceName</code> is a datastore path, the
    /// datacenter for that datastore path.
    /// Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter,
    /// <code>sourceName</code> must be a URL.
    /// 
    /// ***Required privileges:*** System.View
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### destination_name
    /// The name of the destination, either a URL or a
    /// datastore path referring to the destination file or folder.
    ///
    /// ### destination_datacenter
    /// If <code>destinationName</code> is a datastore
    /// path, the datacenter for that datastore path.
    /// Not needed when invoked directly on ESX.
    /// If not specified on a call to VirtualCenter, it is assumed that
    /// the destination path belongs to the source datacenter.
    /// 
    /// ***Required privileges:*** System.View
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### force
    /// If true, overwrite any identically named file
    /// at the destination. If not specified, it is assumed to be false.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to monitor the
    /// operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: if the operation cannot be performed on the source
    /// or destination datastores. Typically, a specific subclass of this exception is
    /// thrown.
    /// 
    /// ***FileNotFound***: if the file or folder specified by sourceName is not
    /// found, or, any intermediate level folder specified by the destinationName is not
    /// found.
    /// 
    /// ***CannotAccessFile***: if the source file or folder cannot be moved because of
    /// insufficient permissions.
    /// 
    /// ***FileLocked***: if the source file or folder is currently locked or in use.
    /// 
    /// ***FileAlreadyExists***: if a file with the given name already
    /// exists at the destination, and the force flag is false. For folders, if the
    /// destination exists, this fault is thrown regardless.
    /// 
    /// ***NoDiskSpace***: if there is not enough space available on the destination
    /// datastore.
    /// 
    /// ***FileFault***: if there is a generic file error
    pub async fn move_datastore_file_task(&self, source_name: &str, source_datacenter: Option<&crate::types::structs::ManagedObjectReference>, destination_name: &str, destination_datacenter: Option<&crate::types::structs::ManagedObjectReference>, force: Option<bool>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = MoveDatastoreFileRequestType {source_name, source_datacenter, destination_name, destination_datacenter, force, };
        let path = format!("/FileManager/{moId}/MoveDatastoreFile_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Fetches as much information as possible for the file path passed in.
    /// 
    /// The main purpose of the API is to show caller any lock information
    /// that can be queried from the host. The API gathers various information
    /// depending on which file-system (VMFS/NFS/VSAN) the file is located on.
    /// 
    /// ***Since:*** vSphere API Release 8.0.2.0
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### path
    /// Full file path to look up lock information on.
    /// For example specific VM file like:
    /// /vmfs/volumes/datastore1/vm/vm-flat.vmdk
    ///
    /// ### host
    /// Host id is required if API is invoked on vCenter Server.
    /// It is optional if invoked on host directly. Esx does not
    /// require this parameter.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: If the operation cannot be performed on the
    /// datastore. Typically, a specific subclass of this exception
    /// is thrown.
    /// 
    /// ***FileFault***: If there is a generic file error.
    /// 
    /// ***InvalidArgument***: If invoked with no host param on vCenter or if
    /// invoked with invalid path. Expected VM file path
    /// would be: /vmfs/volumes/datastore1/vm/vm-flat.vmdk
    pub async fn query_file_lock_info(&self, path: &str, host: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::FileLockInfoResult> {
        let input = QueryFileLockInfoRequestType {path, host, };
        let path = format!("/FileManager/{moId}/QueryFileLockInfo", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::FileLockInfoResult = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct ChangeOwnerRequestType<'a> {
    name: &'a str,
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    owner: &'a str,
}

impl<'a> miniserde::Serialize for ChangeOwnerRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ChangeOwnerRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ChangeOwnerRequestTypeSer<'b, 'a> {
    data: &'b ChangeOwnerRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ChangeOwnerRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ChangeOwnerRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                3 => return Some((std::borrow::Cow::Borrowed("owner"), &self.data.owner as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct CopyDatastoreFileRequestType<'a> {
    source_name: &'a str,
    source_datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    destination_name: &'a str,
    destination_datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    force: Option<bool>,
}

impl<'a> miniserde::Serialize for CopyDatastoreFileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CopyDatastoreFileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CopyDatastoreFileRequestTypeSer<'b, 'a> {
    data: &'b CopyDatastoreFileRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CopyDatastoreFileRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CopyDatastoreFileRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("sourceName"), &self.data.source_name as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.source_datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("sourceDatacenter"), val as &dyn miniserde::Serialize));
                }
                3 => return Some((std::borrow::Cow::Borrowed("destinationName"), &self.data.destination_name as &dyn miniserde::Serialize)),
                4 => {
                    let Some(ref val) = self.data.destination_datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("destinationDatacenter"), val as &dyn miniserde::Serialize));
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
struct DeleteDatastoreFileRequestType<'a> {
    name: &'a str,
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for DeleteDatastoreFileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DeleteDatastoreFileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DeleteDatastoreFileRequestTypeSer<'b, 'a> {
    data: &'b DeleteDatastoreFileRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DeleteDatastoreFileRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DeleteDatastoreFileRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct MakeDirectoryRequestType<'a> {
    name: &'a str,
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    create_parent_directories: Option<bool>,
}

impl<'a> miniserde::Serialize for MakeDirectoryRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MakeDirectoryRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MakeDirectoryRequestTypeSer<'b, 'a> {
    data: &'b MakeDirectoryRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for MakeDirectoryRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MakeDirectoryRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.create_parent_directories else { continue; };
                    return Some((std::borrow::Cow::Borrowed("createParentDirectories"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct MoveDatastoreFileRequestType<'a> {
    source_name: &'a str,
    source_datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    destination_name: &'a str,
    destination_datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    force: Option<bool>,
}

impl<'a> miniserde::Serialize for MoveDatastoreFileRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MoveDatastoreFileRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MoveDatastoreFileRequestTypeSer<'b, 'a> {
    data: &'b MoveDatastoreFileRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for MoveDatastoreFileRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MoveDatastoreFileRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("sourceName"), &self.data.source_name as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.source_datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("sourceDatacenter"), val as &dyn miniserde::Serialize));
                }
                3 => return Some((std::borrow::Cow::Borrowed("destinationName"), &self.data.destination_name as &dyn miniserde::Serialize)),
                4 => {
                    let Some(ref val) = self.data.destination_datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("destinationDatacenter"), val as &dyn miniserde::Serialize));
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
struct QueryFileLockInfoRequestType<'a> {
    path: &'a str,
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for QueryFileLockInfoRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryFileLockInfoRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryFileLockInfoRequestTypeSer<'b, 'a> {
    data: &'b QueryFileLockInfoRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryFileLockInfoRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryFileLockInfoRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("path"), &self.data.path as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
