use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::HostDiagnosticPartition;
use crate::types::structs::HostDiagnosticPartitionCreateDescription;
use crate::types::structs::HostDiagnosticPartitionCreateOption;
use crate::types::structs::HostDiagnosticPartitionCreateSpec;
use crate::types::structs::HostScsiDiskPartition;
/// The DiagnosticSystem managed object is used to configure the diagnostic
/// mechanisms specific to the host.
/// 
/// The DiagnosticSystem interface supports
/// the following concepts:
/// - Notion of an active diagnostic partition that is selected from
///   a set of available partitions.
/// - Ability to create a diagnostic partition that gets added to the
///   list of available partitions and could be made active.
pub struct HostDiagnosticSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl HostDiagnosticSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Creates a diagnostic partition according to the provided create
    /// specification.
    /// 
    /// On success, this method will create the partition
    /// and make the partition the active diagnostic partition if specified.
    /// On failure, the diagnostic partition may exist but may not be active
    /// if the partition was supposed to be made active.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if the host is not an ESX Server.
    /// 
    /// ***NotFound***: if the specified disk cannot be found.
    /// 
    /// ***InvalidArgument***: if an invalid storage type is specified or the
    /// specified disk is unable to accommodate a new diagnostic
    /// partition.
    /// 
    /// ***HostConfigFault***: on some internal failure while trying to
    /// create the diagnostic partition or to activate the diagnostic
    /// partition.
    pub async fn create_diagnostic_partition(&self, spec: &HostDiagnosticPartitionCreateSpec) -> Result<()> {
        let input = CreateDiagnosticPartitionRequestType {spec, };
        let path = format!("/HostDiagnosticSystem/{moId}/CreateDiagnosticPartition", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Retrieves a list of available diagnostic partitions.
    /// 
    /// The server will
    /// provide the list in order of preference. In general, local diagnostic
    /// partitions are better than shared diagnostic partitions because of
    /// the impossibility of multiple servers sharing the same partition. The
    /// most preferred diagnostic partition will be first in the array.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if the host is not an ESX Server.
    /// 
    /// ***HostConfigFault***: on some internal failure while setting the
    /// active partition.
    pub async fn query_available_partition(&self) -> Result<Option<Vec<HostDiagnosticPartition>>> {
        let path = format!("/HostDiagnosticSystem/{moId}/QueryAvailablePartition", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// For a disk, query for the diagnostic partition creation description.
    /// 
    /// The description details how the diagnostic partition will be created
    /// on the disk and provides a creation specification that is needed to
    /// invoke the create operation.
    /// 
    /// See also *HostScsiDisk*, *ScsiLun.uuid*.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### disk_uuid
    /// -
    ///
    /// ### diagnostic_type
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if the host is not an ESX Server.
    /// 
    /// ***NotFound***: if the specified disk cannot be found.
    /// 
    /// ***InvalidArgument***: if an invalid storage type is specified or the
    /// specified disk is unable to accommodate a new diagnostic
    /// partition.
    /// 
    /// ***HostConfigFault***: on some internal failure while trying to
    /// query information about the disk.
    pub async fn query_partition_create_desc(&self, disk_uuid: &str, diagnostic_type: &str) -> Result<HostDiagnosticPartitionCreateDescription> {
        let input = QueryPartitionCreateDescRequestType {disk_uuid, diagnostic_type, };
        let path = format!("/HostDiagnosticSystem/{moId}/QueryPartitionCreateDesc", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Retrieves a list of disks that can be used to contain a diagnostic
    /// partition.
    /// 
    /// This list will contain disks that have sufficient space
    /// to contain a diagnostic partition of the specific type.
    /// 
    /// The choices will be returned in the order that is most preferable
    /// as determined by the system.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### storage_type
    /// -
    ///
    /// ### diagnostic_type
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if the host is not an ESX Server.
    /// 
    /// ***InvalidArgument***: if an invalid storage type is specified.
    /// 
    /// ***HostConfigFault***: on some internal failure while querying the
    /// create options.
    pub async fn query_partition_create_options(&self, storage_type: &str, diagnostic_type: &str) -> Result<Option<Vec<HostDiagnosticPartitionCreateOption>>> {
        let input = QueryPartitionCreateOptionsRequestType {storage_type, diagnostic_type, };
        let path = format!("/HostDiagnosticSystem/{moId}/QueryPartitionCreateOptions", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Changes the active diagnostic partition to a different partition.
    /// 
    /// Setting a NULL partition will result in unsetting the diagnostic
    /// partition.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### partition
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: if the host is not an ESX Server.
    /// 
    /// ***NotFound***: if the diagnostic partition does not exist.
    /// 
    /// ***InvalidArgument***: if the partition is not a diagnostic partition.
    /// 
    /// ***HostConfigFault***: on some internal failure while selecting the
    /// active partition.
    pub async fn select_active_partition(&self, partition: Option<&HostScsiDiskPartition>) -> Result<()> {
        let input = SelectActivePartitionRequestType {partition, };
        let path = format!("/HostDiagnosticSystem/{moId}/SelectActivePartition", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// The currently active diagnostic partition.
    pub async fn active_partition(&self) -> Result<HostDiagnosticPartition> {
        let path = format!("/HostDiagnosticSystem/{moId}/activePartition", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateDiagnosticPartitionRequestType<'a> {
    spec: &'a HostDiagnosticPartitionCreateSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryPartitionCreateDescRequestType<'a> {
    #[serde(rename = "diskUuid")]
    disk_uuid: &'a str,
    #[serde(rename = "diagnosticType")]
    diagnostic_type: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryPartitionCreateOptionsRequestType<'a> {
    #[serde(rename = "storageType")]
    storage_type: &'a str,
    #[serde(rename = "diagnosticType")]
    diagnostic_type: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SelectActivePartitionRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    partition: Option<&'a HostScsiDiskPartition>,
}
