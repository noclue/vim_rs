use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::ManagedObjectReference;
use crate::types::NvdimmNamespaceCreateSpec;
use crate::types::NvdimmNamespaceDeleteSpec;
use crate::types::NvdimmPMemNamespaceCreateSpec;
use crate::types::NvdimmSystemInfo;
/// Managed object responsible for reading and configuring
/// Non-Volatile DIMMs.
pub struct HostNvdimmSystem {
    client: Arc<VimClient>,
    mo_id: String,
}
impl HostNvdimmSystem {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
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
    pub async fn create_nvdimm_namespace_task(&self, create_spec: &NvdimmNamespaceCreateSpec) -> Result<ManagedObjectReference> {
        let input = CreateNvdimmNamespaceRequestType {create_spec, };
        let path = format!("/HostNvdimmSystem/{moId}/CreateNvdimmNamespace_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn create_nvdimm_p_mem_namespace_task(&self, create_spec: &NvdimmPMemNamespaceCreateSpec) -> Result<ManagedObjectReference> {
        let input = CreateNvdimmPMemNamespaceRequestType {create_spec, };
        let path = format!("/HostNvdimmSystem/{moId}/CreateNvdimmPMemNamespace_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn delete_nvdimm_block_namespaces_task(&self) -> Result<ManagedObjectReference> {
        let path = format!("/HostNvdimmSystem/{moId}/DeleteNvdimmBlockNamespaces_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
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
    pub async fn delete_nvdimm_namespace_task(&self, delete_spec: &NvdimmNamespaceDeleteSpec) -> Result<ManagedObjectReference> {
        let input = DeleteNvdimmNamespaceRequestType {delete_spec, };
        let path = format!("/HostNvdimmSystem/{moId}/DeleteNvdimmNamespace_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn nvdimm_system_info(&self) -> Result<NvdimmSystemInfo> {
        let path = format!("/HostNvdimmSystem/{moId}/nvdimmSystemInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateNvdimmNamespaceRequestType<'a> {
    #[serde(rename = "createSpec")]
    create_spec: &'a NvdimmNamespaceCreateSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateNvdimmPMemNamespaceRequestType<'a> {
    #[serde(rename = "createSpec")]
    create_spec: &'a NvdimmPMemNamespaceCreateSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DeleteNvdimmNamespaceRequestType<'a> {
    #[serde(rename = "deleteSpec")]
    delete_spec: &'a NvdimmNamespaceDeleteSpec,
}
