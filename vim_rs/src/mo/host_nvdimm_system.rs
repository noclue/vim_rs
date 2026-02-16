use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Deprecated as of vSphere 9.0 APIs with no replacement.
/// 
/// Managed object responsible for reading and configuring
/// Non-Volatile DIMMs.
#[derive(Clone)]
pub struct HostNvdimmSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostNvdimmSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Deprecated as of vSphere 6.7u1, use createPMemNamespace
    /// Create nvd namespace from information passed in NamespaceCreationSpec.
    /// 
    /// A new block or persistent namespace can be created on the NVDIMM(s)
    /// when the system is in maintenance mode.
    /// 
    /// If all the parameters passed
    /// are valid and system is in maintenance mode, then a DSM (Device
    /// Specific Method) call is made to create the namespace. DSM calls are
    /// blockable and slow operations and hence the use of task.
    /// 
    /// If a new namespace is created, its UUID is returned.
    /// 
    /// ***Required privileges:*** Host.Config.Nvdimm
    ///
    /// ## Parameters:
    ///
    /// ### create_spec
    /// Parameters to create the required namespace.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object which is used to
    /// monitor this operation. The task result
    /// (*Task.info*.*TaskInfo.result*) contains a
    /// *NvdimmGuid* object that has the UUID of the
    /// newly created namespace.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if an argument to create namespace is invalid.
    /// 
    /// ***NotSupported***: if no NVDIMMs are found, namespace type is
    /// not supported or if operation does not have
    /// DSM method.
    /// 
    /// ***InvalidHostState***: if operation is not allowed as system is not in
    /// maintenance mode.
    /// 
    /// ***AlreadyExists***: if the namespace of type already exists.
    /// 
    /// ***SystemError***: for other system errors along with localized
    /// reason for failure.
    /// 
    /// ***HostConfigFault***: for any other failure.
    pub async fn create_nvdimm_namespace_task(&self, create_spec: &crate::types::structs::NvdimmNamespaceCreateSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateNvdimmNamespaceRequestType {create_spec, };
        let path = format!("/HostNvdimmSystem/{moId}/CreateNvdimmNamespace_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Create persistent memory mode nvd namespace from information passed
    /// in PMemNamespaceCreationSpec.
    /// 
    /// A new persistent namespace can be created on the NVDIMM(s)
    /// when the system is in maintenance mode. If all the parameters passed
    /// are valid and system is in maintenance mode, then a DSM (Device
    /// Specific Method) call is made to create the namespace. DSM calls are
    /// blockable and slow operations and hence the use of task.
    /// 
    /// If a new namespace is created, its UUID is returned.
    /// 
    /// ***Required privileges:*** Host.Config.Nvdimm
    ///
    /// ## Parameters:
    ///
    /// ### create_spec
    /// Parameters to create the required namespace.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object which is used to
    /// monitor this operation. The task result
    /// (*Task.info*.*TaskInfo.result*) contains a
    /// *NvdimmGuid* object that has the UUID of the
    /// newly created namespace.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if an argument to create namespace is invalid.
    /// 
    /// ***NotSupported***: if no NVDIMMs are found or if operation does
    /// not have the supporting DSM method.
    /// 
    /// ***InvalidHostState***: if operation is not allowed as system is not in
    /// maintenance mode.
    /// 
    /// ***AlreadyExists***: if the namespace of type already exists.
    /// 
    /// ***SystemError***: for other system errors along with localized
    /// reason for failure.
    /// 
    /// ***HostConfigFault***: for any other failure.
    pub async fn create_nvdimm_p_mem_namespace_task(&self, create_spec: &crate::types::structs::NvdimmPMemNamespaceCreateSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateNvdimmPMemNamespaceRequestType {create_spec, };
        let path = format!("/HostNvdimmSystem/{moId}/CreateNvdimmPMemNamespace_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Delete all block mode namespaces in the system.
    /// 
    /// Existing block namespace(s) can be deleted from all NVDIMMs, if the
    /// system is in maintenance mode. If paramters passed are valid and
    /// the system is in maintenance mode, then DSM calls are made to
    /// delete these namespaces. DSM calls are blockable, slow operations
    /// and hence the use of task.
    /// 
    /// If a particular block namespace is to be deleted, use
    /// *HostNvdimmSystem.DeleteNvdimmNamespace_Task* by passing it the UUID
    /// of the block namespace.
    /// 
    /// ***Required privileges:*** Host.Config.Nvdimm
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object which is used to
    /// monitor this operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if UUID of namespace to be created is invalid.
    /// 
    /// ***NotFound***: if the namespace to be deleted is not found.
    /// 
    /// ***NotSupported***: if no NVDIMMs are found and if operation does
    /// not have DSM method.
    /// 
    /// ***InvalidHostState***: if operation is not allowed as system is not
    /// in maintenance mode.
    /// 
    /// ***SystemError***: for any other system error along with localized
    /// reason for failure.
    /// 
    /// ***HostConfigFault***: for any other failure.
    pub async fn delete_nvdimm_block_namespaces_task(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let path = format!("/HostNvdimmSystem/{moId}/DeleteNvdimmBlockNamespaces_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Delete nvd namespace whose uuid matches passed parameter.
    /// 
    /// An existing namespace of type block or persistent mode can be deleted
    /// from NVDIMM(s), if the system is in maintenance mode. If paramters
    /// passed are valid and the system is in maintenance mode, then a DSM call
    /// is made to delete this namespace. DSM calls are blockable, slow
    /// operations and hence the use of task.
    /// 
    /// ***Required privileges:*** Host.Config.Nvdimm
    ///
    /// ## Parameters:
    ///
    /// ### delete_spec
    /// Details of namespace to be deleted.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object which is used to
    /// monitor this operation.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if UUID of namespace to be created is invalid.
    /// 
    /// ***NotFound***: if the namespace to be deleted is not found.
    /// 
    /// ***NotSupported***: if no NVDIMMs are found or if operation does
    /// not have DSM method.
    /// 
    /// ***InvalidHostState***: if operation is not allowed as system is not in
    /// maintenance mode.
    /// 
    /// ***SystemError***: for any other system error along with
    /// localized reason for failure.
    /// 
    /// ***HostConfigFault***: for any other failure.
    pub async fn delete_nvdimm_namespace_task(&self, delete_spec: &crate::types::structs::NvdimmNamespaceDeleteSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = DeleteNvdimmNamespaceRequestType {delete_spec, };
        let path = format!("/HostNvdimmSystem/{moId}/DeleteNvdimmNamespace_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Host NVDIMM information.
    /// 
    /// \- Summary of all dimms on the host.
    /// \- Array of all DIMMs on the host.
    /// \- Array of DIMM information and health for all dimms on the host.
    /// \- Array of interleave set for all sets on the host.
    /// \- Array of interleave set information for all sets on the host.
    /// \- Array of namespace IDs for all dimms on the host.
    /// \- Array of namespace details of all dimms on the host.
    /// 
    /// ***Required privileges:*** Host.Config.Nvdimm
    ///
    /// ## Returns:
    ///
    /// Return set of all NVDIMM related information.
    pub async fn nvdimm_system_info(&self) -> Result<crate::types::structs::NvdimmSystemInfo> {
        let path = format!("/HostNvdimmSystem/{moId}/nvdimmSystemInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::NvdimmSystemInfo = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct CreateNvdimmNamespaceRequestType<'a> {
    create_spec: &'a crate::types::structs::NvdimmNamespaceCreateSpec,
}

impl<'a> miniserde::Serialize for CreateNvdimmNamespaceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateNvdimmNamespaceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateNvdimmNamespaceRequestTypeSer<'b, 'a> {
    data: &'b CreateNvdimmNamespaceRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateNvdimmNamespaceRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateNvdimmNamespaceRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("createSpec"), &self.data.create_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateNvdimmPMemNamespaceRequestType<'a> {
    create_spec: &'a crate::types::structs::NvdimmPMemNamespaceCreateSpec,
}

impl<'a> miniserde::Serialize for CreateNvdimmPMemNamespaceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateNvdimmPMemNamespaceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateNvdimmPMemNamespaceRequestTypeSer<'b, 'a> {
    data: &'b CreateNvdimmPMemNamespaceRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateNvdimmPMemNamespaceRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateNvdimmPMemNamespaceRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("createSpec"), &self.data.create_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DeleteNvdimmNamespaceRequestType<'a> {
    delete_spec: &'a crate::types::structs::NvdimmNamespaceDeleteSpec,
}

impl<'a> miniserde::Serialize for DeleteNvdimmNamespaceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DeleteNvdimmNamespaceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DeleteNvdimmNamespaceRequestTypeSer<'b, 'a> {
    data: &'b DeleteNvdimmNamespaceRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DeleteNvdimmNamespaceRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DeleteNvdimmNamespaceRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("deleteSpec"), &self.data.delete_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
