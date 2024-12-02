use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::ManagedObjectReference;
use crate::types::OvfCreateDescriptorParams;
use crate::types::OvfCreateDescriptorResult;
use crate::types::OvfCreateImportSpecParams;
use crate::types::OvfCreateImportSpecResult;
use crate::types::OvfOptionInfo;
use crate::types::OvfParseDescriptorParams;
use crate::types::OvfParseDescriptorResult;
use crate::types::OvfValidateHostParams;
use crate::types::OvfValidateHostResult;
/// Service interface to parse and generate OVF descriptors.
/// 
/// The purpose of this interface is to make it easier for callers to import VMs and
/// vApps from OVF packages and to export VI packages to OVF. In the following
/// description, the term "client" is used to mean any caller of the interface.
/// 
/// This interface only converts between OVF and VI types. To actually import and export
/// entities, use *ResourcePool.importVApp*,
/// *VirtualMachine.exportVm* and
/// *VirtualApp.exportVApp*.
/// 
/// **Import**
/// 
/// For the import scenario, the typical sequence of events is as follows:
/// 
/// The client calls parseDescriptor to obtain information about the OVF descriptor. This
/// typically includes information (such as a list of networks) that must be mapped to VI
/// infrastructure entities.
/// 
/// The OVF descriptor is validated against the OVF Specification, and any errors or
/// warnings are returned as part of the ParseResult. For example, the parser might
/// encounter a section marked required that it does not understand, or the XML descriptor
/// might be malformed.
/// 
/// The client decides on network mappings, datastore, properties etc. It then calls
/// createImportSpec to obtain the parameters needed to call
/// *ResourcePool.importVApp*.
/// 
/// If any warnings are present, the client will review these and decide whether to
/// proceed or not. If errors are present, the ImportSpec will be missing, so
/// the client is forced to give up or fix the problems and then try again.
/// 
/// The client now calls *ResourcePool.importVApp*, passing the ImportSpec as a parameter. This will create
/// the virtual machines and *VirtualApp* objects in VI and return locations
/// to which the files of the entity can be uploaded. It also returns a lease that
/// controls the duration of the lock taken on the newly created inventory objects. When
/// all files have been uploaded, the client must release this lease.
/// 
/// **Export**
/// 
/// Creating the OVF descriptor is the last part of exporting an entity to OVF. At this
/// point, the client has already downloaded all files for the entity, optionally
/// compressing and/or chunking them (however, the client may do a "dry run" of creating
/// the descriptor before downloading the files. See *OvfManager.createDescriptor*).
/// 
/// In addition to the entity reference itself, information about the choices made on
/// these files is passed to createDescriptor as a list of OvfFile instances.
/// 
/// The client must inspect and act upon warnings and errors as previously described.
/// 
/// No matter if the export succeeds or fails, the client is responsible for releasing the
/// shared state lock taken on the entity (by *VirtualMaching.exportVm* or *VirtualApp.exportVApp*) during the export.
/// 
/// **Error handling**
/// 
/// All result types contain warning and error lists. Warnings do not cause processing to
/// fail, but the caller (typically, the user of a GUI client) may choose to reject the
/// result based on the warnings issued.
/// 
/// Errors cause processing to abort by definition.
pub struct OvfManager {
    client: Arc<VimClient>,
    mo_id: String,
}
impl OvfManager {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Create an OVF descriptor for the specified ManagedEntity, which may be a
    /// *VirtualMachine* or a *VirtualApp*.
    /// 
    /// To create the complete OVF descriptor, the client must already have downloaded the
    /// files that are part of the entity, because information about these files
    /// (compression, chunking, filename etc.) is part of the descriptor.
    /// 
    /// However, these downloads can be quite time-consuming, so if the descriptor for some
    /// reason cannot be generated, the client will want to know this before downloading
    /// the files.
    /// 
    /// For this reason, the client may do an initial "dry run" with the ovfFiles
    /// parameter unset. Default filenames will then be used in the descriptor, and the
    /// client can examine any warnings and/or errors before downloading the files.
    /// 
    /// After the final call to this method, client must release the lock on the entity
    /// given to it by *VirtualMachine.exportVm* or *VirtualApp.exportVApp*.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### obj
    /// The entity to export. Supported types are *VirtualMachine*
    /// and *VirtualApp*.
    /// 
    /// ***Required privileges:*** VApp.Export
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ### cdp
    /// Parameters to the method, bundled in an instance of
    /// CreateDescriptorParams.
    ///
    /// ## Returns:
    ///
    /// An instance of CreateDescriptorResult
    ///
    /// ## Errors:
    ///
    /// ***TaskInProgress***: if a required managed entity is busy.
    /// 
    /// ***VmConfigFault***: if a configuration issue prevents the operation from
    /// succeeding. Typically, a more specific subclass is thrown.
    /// 
    /// ***ConcurrentAccess***: if a concurrency issue prevents the operation from
    /// succeeding.
    /// 
    /// ***FileFault***: if there is a generic file error
    /// 
    /// ***InvalidState***: if the operation failed due to the current state of the system.
    pub async fn create_descriptor(&self, obj: &ManagedObjectReference, cdp: &OvfCreateDescriptorParams) -> Result<OvfCreateDescriptorResult> {
        let input = CreateDescriptorRequestType {obj, cdp, };
        let path = format!("/OvfManager/{moId}/CreateDescriptor", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Validate the OVF descriptor against the hardware supported by the
    /// host system.
    /// 
    /// If the validation succeeds, return a result containing:
    /// - An *ImportSpec* to use when importing the entity.
    /// - A list of items to upload (for example disk backing files, ISO images etc.)
    ///   
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### ovf_descriptor
    /// The OVF descriptor of the entity.
    ///
    /// ### resource_pool
    /// The resource pool to import the entity to. May be a
    /// vApp.
    /// 
    /// ***Required privileges:*** VApp.Import
    /// 
    /// Refers instance of *ResourcePool*.
    ///
    /// ### datastore
    /// The datastore on which to create the inventory objects
    /// of the entity, for example "storage1". The privilege
    /// Datastore.AllocateSpace is required on the datastore.
    /// 
    /// ***Required privileges:*** Datastore.AllocateSpace
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ### cisp
    /// Additional parameters to the method, bundled in an instance of
    /// CreateImportSpecParams.
    ///
    /// ## Errors:
    ///
    /// ***TaskInProgress***: if a required managed entity is busy.
    /// 
    /// ***VmConfigFault***: if a configuration issue prevents the operation from
    /// succeeding. Typically, a more specific subclass is thrown.
    /// 
    /// ***ConcurrentAccess***: if a concurrency issue prevents the operation from
    /// succeeding.
    /// 
    /// ***FileFault***: if there is a generic file error
    /// 
    /// ***InvalidState***: if the operation failed due to the current state of the system.
    pub async fn create_import_spec(&self, ovf_descriptor: &str, resource_pool: &ManagedObjectReference, datastore: &ManagedObjectReference, cisp: &OvfCreateImportSpecParams) -> Result<OvfCreateImportSpecResult> {
        let input = CreateImportSpecRequestType {ovf_descriptor, resource_pool, datastore, cisp, };
        let path = format!("/OvfManager/{moId}/CreateImportSpec", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Parse the OVF descriptor and return as much information about it as possible
    /// without knowing the host on which it will be imported.
    /// 
    /// Typically, this method is called once without a deploymentOption parameter to
    /// obtain the values for the default deployment option. Part of the result is the list
    /// of possible deployment options. To obtain the values for a particular deployment
    /// option, call this method again, specifying that option.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### ovf_descriptor
    /// The OVF descriptor to examine.
    ///
    /// ### pdp
    /// Additional parameters for parseDescriptor, wrapped in an instance of
    /// ParseDescriptorParams.
    ///
    /// ## Returns:
    ///
    /// The information about the descriptor
    ///
    /// ## Errors:
    ///
    /// ***TaskInProgress***: if a required managed entity is busy.
    /// 
    /// ***VmConfigFault***: if a configuration issue prevents the operation from
    /// succeeding. Typically, a more specific subclass is thrown.
    /// 
    /// ***ConcurrentAccess***: if a concurrency issue prevents the operation from
    /// succeeding.
    /// 
    /// ***FileFault***: if there is a generic file error
    /// 
    /// ***InvalidState***: if the operation failed due to the current state of the system.
    pub async fn parse_descriptor(&self, ovf_descriptor: &str, pdp: &OvfParseDescriptorParams) -> Result<OvfParseDescriptorResult> {
        let input = ParseDescriptorRequestType {ovf_descriptor, pdp, };
        let path = format!("/OvfManager/{moId}/ParseDescriptor", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Validate that the given OVF can be imported on the host.
    /// 
    /// More specifically, this means whether or not the host supports the virtual hardware
    /// required by the OVF descriptor.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### ovf_descriptor
    /// The OVF descriptor to examine.
    ///
    /// ### host
    /// The host to validate against.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### vhp
    /// Additional parameters for validateHost, wrapped in a ValidateHostParams
    /// instance.
    ///
    /// ## Returns:
    ///
    /// A ValidateResult instance containing any warnings and/or errors from the
    /// validation.
    ///
    /// ## Errors:
    ///
    /// ***TaskInProgress***: if a required managed entity is busy.
    /// 
    /// ***ConcurrentAccess***: if a concurrency issue prevents the operation from
    /// succeeding.
    /// 
    /// ***FileFault***: if there is a generic file error
    /// 
    /// ***InvalidState***: if the operation failed due to the current state of the system.
    pub async fn validate_host(&self, ovf_descriptor: &str, host: &ManagedObjectReference, vhp: &OvfValidateHostParams) -> Result<OvfValidateHostResult> {
        let input = ValidateHostRequestType {ovf_descriptor, host, vhp, };
        let path = format!("/OvfManager/{moId}/ValidateHost", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Returns an array of *OvfOptionInfo* object that specifies what options the server
    /// support for exporting an OVF descriptor.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// An instance of *OvfOptionInfo*
    pub async fn ovf_export_option(&self) -> Result<Option<Vec<OvfOptionInfo>>> {
        let path = format!("/OvfManager/{moId}/ovfExportOption", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Returns an array of *OvfOptionInfo* object that specifies what options the server
    /// support for modifing/relaxing the OVF import process.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// An instance of *OvfOptionInfo*
    pub async fn ovf_import_option(&self) -> Result<Option<Vec<OvfOptionInfo>>> {
        let path = format!("/OvfManager/{moId}/ovfImportOption", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateDescriptorRequestType<'a> {
    obj: &'a ManagedObjectReference,
    cdp: &'a OvfCreateDescriptorParams,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateImportSpecRequestType<'a> {
    #[serde(rename = "ovfDescriptor")]
    ovf_descriptor: &'a str,
    #[serde(rename = "resourcePool")]
    resource_pool: &'a ManagedObjectReference,
    datastore: &'a ManagedObjectReference,
    cisp: &'a OvfCreateImportSpecParams,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ParseDescriptorRequestType<'a> {
    #[serde(rename = "ovfDescriptor")]
    ovf_descriptor: &'a str,
    pdp: &'a OvfParseDescriptorParams,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ValidateHostRequestType<'a> {
    #[serde(rename = "ovfDescriptor")]
    ovf_descriptor: &'a str,
    host: &'a ManagedObjectReference,
    vhp: &'a OvfValidateHostParams,
}
