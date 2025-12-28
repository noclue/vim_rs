use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The *SmsStorageManager* managed object (SMS) provides methods to retrieve
/// information about available storage topology, capabilities, and state.
/// 
/// SMS establishes and maintains connections with VASA providers. SMS retrieves
/// information about storage availability from the providers, and clients can use
/// the SMS API to perform the following operations.
/// - Identify VASA providers.
/// - Retrieve information about storage arrays.
/// - Identify vSphere inventory entities (hosts and datastores)
///   which are associated with external storage entities on the storage arrays.
#[derive(Clone)]
pub struct SmsStorageManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl SmsStorageManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Get the list of storage arrays managed by all the registered VASA providers.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### provider_id
    /// List of *SmsProviderInfo.uid* for the VASA
    /// provider objects.
    ///
    /// ## Returns:
    ///
    /// List of data objects containing information about
    /// StorageArray.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If the given providerId does not have any
    /// reference.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while
    /// processing the query request.
    pub async fn query_array(&self, provider_id: Option<&[String]>) -> Result<Option<Vec<crate::types::structs::StorageArray>>> {
        let input = QueryArrayRequestType {provider_id, };
        let path = format!("/sms/SmsStorageManager/{moId}/QueryArray", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::StorageArray>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Get the StorageArray object that is associated with the
    /// ScsiLun.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### canonical_name
    /// *ScsiLun.canonicalName*
    /// of ScsiLun
    ///
    /// ## Returns:
    ///
    /// StorageArray for the for the ScsiLun.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the specified entity does not exist.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while
    /// processing the query request.
    pub async fn query_array_associated_with_lun(&self, canonical_name: &str) -> Result<Option<crate::types::structs::StorageArray>> {
        let input = QueryArrayAssociatedWithLunRequestType {canonical_name, };
        let path = format!("/sms/SmsStorageManager/{moId}/QueryArrayAssociatedWithLun", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<crate::types::structs::StorageArray>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Query Backing Storage Pools for StorageLun or StorageFileSystem.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### entity_id
    /// Unique identifier of a StorageLun or StorageFileSystem.
    ///
    /// ### entity_type
    /// Entity type of the entity specified using entityId. This can be either
    /// StorageLun or StorageFileSystem.
    ///
    /// ## Returns:
    ///
    /// Array of BackingStoragePool*BackingStoragePool* associated with specified StorageLun or StorageFileSystem.
    /// If entityId is null then API returns all the BackingStoragePools of the specified type.
    /// If both entityId and entityType are not specified then API returns all the BackingStoragePools available.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the specified entityId does not exist.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while processing the query request.
    pub async fn query_associated_backing_storage_pool(&self, entity_id: Option<&str>, entity_type: Option<&str>) -> Result<Option<Vec<crate::types::structs::BackingStoragePool>>> {
        let input = QueryAssociatedBackingStoragePoolRequestType {entity_id, entity_type, };
        let path = format!("/sms/SmsStorageManager/{moId}/QueryAssociatedBackingStoragePool", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::BackingStoragePool>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Query BackingStoragePools for the given set of datastores.
    /// 
    /// Available information for all types of BackingStoragePools*BackingStoragePoolType_enum*
    /// for every input datastore is returned as part of the result.
    /// More than one datastore can map to same set of BackingStoragePools.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// Array containing references to *Datastore* objects.
    /// 
    /// Refers instances of *Datastore*.
    ///
    /// ## Returns:
    ///
    /// *DatastoreBackingPoolMapping*
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if any *Datastore* in the specified input array does not exist.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while processing the query request.
    pub async fn query_datastore_backing_pool_mapping(&self, datastore: &[crate::types::structs::ManagedObjectReference]) -> Result<Vec<crate::types::structs::DatastoreBackingPoolMapping>> {
        let input = QueryDatastoreBackingPoolMappingRequestType {datastore, };
        let path = format!("/sms/SmsStorageManager/{moId}/QueryDatastoreBackingPoolMapping", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: Vec<crate::types::structs::DatastoreBackingPoolMapping> = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Get the capability for the given datastore.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// reference to *Datastore*
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Returns:
    ///
    /// A data object containing information about StorageCapability.
    /// If the VMFS datastore have heterogeneous Luns (in case of VMFS extends),
    /// *StorageCapability.description* will be empty.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the specified entity does not exist.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while
    /// processing the query request.
    pub async fn query_datastore_capability(&self, datastore: &crate::types::structs::ManagedObjectReference) -> Result<Option<crate::types::structs::StorageCapability>> {
        let input = QueryDatastoreCapabilityRequestType {datastore, };
        let path = format!("/sms/SmsStorageManager/{moId}/QueryDatastoreCapability", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<crate::types::structs::StorageCapability>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Deprecated as of SMS API 3.0, use *SmsStorageManager.QueryDrsMigrationCapabilityForPerformanceEx*.
    /// 
    /// Query the provider to figure out whether Storage DRS should
    /// migrate VMDKs between the two given datastores.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### src_datastore
    /// Reference to the source *Datastore*
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ### dst_datastore
    /// Reference to the destination *Datastore*
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Returns:
    ///
    /// true if VM migration is recommended from srcDatastore
    /// to dstDatastore.
    /// false if VM migration is not recommended from
    /// srcDatastore to dstDatastore.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the specified entity does not exist.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while
    /// processing the query request.
    pub async fn query_drs_migration_capability_for_performance(&self, src_datastore: &crate::types::structs::ManagedObjectReference, dst_datastore: &crate::types::structs::ManagedObjectReference) -> Result<bool> {
        let input = QueryDrsMigrationCapabilityForPerformanceRequestType {src_datastore, dst_datastore, };
        let path = format!("/sms/SmsStorageManager/{moId}/QueryDrsMigrationCapabilityForPerformance", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: bool = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Deprecated as of SMS API 5.0.
    /// 
    /// Query available VASA providers for I/O performance based migration recommendations
    /// for all pair combinations of the given set of datastores.
    /// 
    /// Datastore pairs for which
    /// a recommendation cannot be obtained are not included in the result.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### datastore
    /// Array containing references to *Datastore* objects.
    /// 
    /// Refers instances of *Datastore*.
    ///
    /// ## Returns:
    ///
    /// *DrsMigrationCapabilityResult*
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if any *Datastore* in the specified input array does not exist.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while processing the query request.
    pub async fn query_drs_migration_capability_for_performance_ex(&self, datastore: &[crate::types::structs::ManagedObjectReference]) -> Result<crate::types::structs::DrsMigrationCapabilityResult> {
        let input = QueryDrsMigrationCapabilityForPerformanceExRequestType {datastore, };
        let path = format!("/sms/SmsStorageManager/{moId}/QueryDrsMigrationCapabilityForPerformanceEx", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::DrsMigrationCapabilityResult = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Query for fault domains based on the query spec.
    /// 
    /// If spec is null, SMS
    /// will return all the root fault domains only.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### filter
    /// spec for the query operation.
    ///
    /// ## Returns:
    ///
    /// all the fault domains based on the query spec.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if invalid input is provided.
    /// 
    /// ***NotFound***: if the specified providerId in the spec does not exist
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while processing the
    /// query request.
    pub async fn query_fault_domain(&self, filter: Option<&crate::types::structs::FaultDomainFilter>) -> Result<Option<Vec<Box<dyn crate::types::traits::FaultDomainIdTrait>>>> {
        let input = QueryFaultDomainRequestType {filter, };
        let path = format!("/sms/SmsStorageManager/{moId}/QueryFaultDomain", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<Box<dyn crate::types::traits::FaultDomainIdTrait>>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Get the StorageFileSystem data objects for the Array.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### array_id
    /// *StorageArray.uuid* for the StorageArray
    /// object.
    ///
    /// ## Returns:
    ///
    /// List of data objects containing information about
    /// StorageFileSystem.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the specified entity does not exist.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while
    /// processing the query request.
    pub async fn query_file_system_associated_with_array(&self, array_id: &str) -> Result<Option<Vec<crate::types::structs::StorageFileSystem>>> {
        let input = QueryFileSystemAssociatedWithArrayRequestType {array_id, };
        let path = format!("/sms/SmsStorageManager/{moId}/QueryFileSystemAssociatedWithArray", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::StorageFileSystem>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Get HostSystem managed entities that share the StorageLun.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### scsi_3_id
    /// *StorageLun.uuid* for the StorageLun
    /// object.
    ///
    /// ### array_id
    /// *StorageArray.uuid* for the StorageArray
    /// object.
    ///
    /// ## Returns:
    ///
    /// List of HostSystems.
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the specified entity does not exist.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while
    /// processing the query request.
    pub async fn query_host_associated_with_lun(&self, scsi_3_id: &str, array_id: &str) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let input = QueryHostAssociatedWithLunRequestType {scsi_3_id, array_id, };
        let path = format!("/sms/SmsStorageManager/{moId}/QueryHostAssociatedWithLun", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::ManagedObjectReference>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Get the list of StorageLun data objects that for the Array.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### array_id
    /// *StorageArray.uuid* for the StorageArray
    /// object.
    ///
    /// ## Returns:
    ///
    /// List of data object containing information about
    /// StorageLun.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the specified entity does not exist.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while
    /// processing the query request.
    pub async fn query_lun_associated_with_array(&self, array_id: &str) -> Result<Option<Vec<crate::types::structs::StorageLun>>> {
        let input = QueryLunAssociatedWithArrayRequestType {array_id, };
        let path = format!("/sms/SmsStorageManager/{moId}/QueryLunAssociatedWithArray", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::StorageLun>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Get the StorageLun data objects that are associated with StoragePort.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### port_id
    /// *StoragePort.uuid* for the StoragePort
    /// object.
    ///
    /// ### array_id
    /// *StorageArray.uuid* for the StorageArray
    /// object.
    ///
    /// ## Returns:
    ///
    /// List of data objects containing information about
    /// StorageLun.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the specified entity does not exist.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while
    /// processing the query request.
    pub async fn query_lun_associated_with_port(&self, port_id: &str, array_id: &str) -> Result<Option<Vec<crate::types::structs::StorageLun>>> {
        let input = QueryLunAssociatedWithPortRequestType {port_id, array_id, };
        let path = format!("/sms/SmsStorageManager/{moId}/QueryLunAssociatedWithPort", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::StorageLun>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Get NFS datastore managed entity that are associated with
    /// StorageFileSystem.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### file_system_id
    /// *StorageFileSystem.uuid* for the
    /// StorageFileSystem object
    ///
    /// ### array_id
    /// *StorageArray.uuid* for the StorageArray
    /// object.
    ///
    /// ## Returns:
    ///
    /// Nas datastore for the storage file system id.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the specified entity does not exist.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while
    /// processing the query request.
    pub async fn query_nfs_datastore_associated_with_file_system(&self, file_system_id: &str, array_id: &str) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let input = QueryNfsDatastoreAssociatedWithFileSystemRequestType {file_system_id, array_id, };
        let path = format!("/sms/SmsStorageManager/{moId}/QueryNfsDatastoreAssociatedWithFileSystem", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<crate::types::structs::ManagedObjectReference>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Get the StoragePort data objects that are associated with Array.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### array_id
    /// *StorageArray.uuid* for the StorageArray
    /// object.
    ///
    /// ## Returns:
    ///
    /// List of data objects containing information about
    /// StoragePort.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the specified entity does not exist.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while
    /// processing the query request.
    pub async fn query_port_associated_with_array(&self, array_id: &str) -> Result<Option<Vec<Box<dyn crate::types::traits::StoragePortTrait>>>> {
        let input = QueryPortAssociatedWithArrayRequestType {array_id, };
        let path = format!("/sms/SmsStorageManager/{moId}/QueryPortAssociatedWithArray", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<Box<dyn crate::types::traits::StoragePortTrait>>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Get the StoragePort data object that is associated with LUN.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### scsi_3_id
    /// *StorageLun.uuid* for the StorageLun
    /// object.
    ///
    /// ### array_id
    /// *StorageArray.uuid* for the StorageArray
    /// object.
    ///
    /// ## Returns:
    ///
    /// A data object containing information about StoragePort.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the specified entity does not exist.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while
    /// processing the query request.
    pub async fn query_port_associated_with_lun(&self, scsi_3_id: &str, array_id: &str) -> Result<Option<Box<dyn crate::types::traits::StoragePortTrait>>> {
        let input = QueryPortAssociatedWithLunRequestType {scsi_3_id, array_id, };
        let path = format!("/sms/SmsStorageManager/{moId}/QueryPortAssociatedWithLun", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Box<dyn crate::types::traits::StoragePortTrait>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Get the StoragePort data objects that are associated with Processor.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### processor_id
    /// *StorageProcessor.uuid* for the
    /// StorageProcessor object.
    ///
    /// ### array_id
    /// *StorageArray.uuid* for the StorageArray
    /// object.
    ///
    /// ## Returns:
    ///
    /// List of data objects containing information about
    /// StoragePort.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the specified entity does not exist.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while
    /// processing the query request.
    pub async fn query_port_associated_with_processor(&self, processor_id: &str, array_id: &str) -> Result<Option<Vec<Box<dyn crate::types::traits::StoragePortTrait>>>> {
        let input = QueryPortAssociatedWithProcessorRequestType {processor_id, array_id, };
        let path = format!("/sms/SmsStorageManager/{moId}/QueryPortAssociatedWithProcessor", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<Box<dyn crate::types::traits::StoragePortTrait>>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Get the StorageProcessor data objects that are associated with Array.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### array_id
    /// *StorageArray.uuid* for the StorageArray
    /// object.
    ///
    /// ## Returns:
    ///
    /// List of data objects containing information about
    /// StorageProcessor.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the specified entity does not exist.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while
    /// processing the query request.
    pub async fn query_processor_associated_with_array(&self, array_id: &str) -> Result<Option<Vec<crate::types::structs::StorageProcessor>>> {
        let input = QueryProcessorAssociatedWithArrayRequestType {array_id, };
        let path = format!("/sms/SmsStorageManager/{moId}/QueryProcessorAssociatedWithArray", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::StorageProcessor>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Get the list of Providers that are currently registered
    /// with StorageManager.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Returns:
    ///
    /// List of Providers.
    /// 
    /// Refers instances of *SmsProvider*.
    ///
    /// ## Errors:
    ///
    /// ***QueryExecutionFault***: if an error is encountered while processing the
    /// query request.
    pub async fn query_provider(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let path = format!("/sms/SmsStorageManager/{moId}/QueryProvider", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::ManagedObjectReference>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Query for replication group details based on the query filter spec.
    /// 
    /// The replication
    /// group id list in the filter spec cannot be null or empty.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### rg_filter
    /// -
    ///
    /// ## Returns:
    ///
    /// An array of *GroupOperationResult* elements.
    /// The length of the result array must be the same as the input.
    /// In the result array, each entry is either a
    /// *QueryReplicationGroupSuccessResult* (for success), or a
    /// *GroupErrorResult* (for failure).
    /// 
    /// The following fault may be set in error result entry:
    /// - *NotFound* if the replication group cannot be found.
    /// - *ProviderUnavailable* if the provider for the entity is temporarily unavailable.
    /// - *InactiveProvider* if the provider for the entity is not active.
    /// - *ProviderBusy* if the provider for the entity is busy.
    /// - *NotImplemented* if the provider does not implement this function.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if *ReplicationGroupFilter.groupId* is null or empty.
    /// 
    /// ***ServiceNotInitialized***: if SMS service is not initialized.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while processing the query request.
    pub async fn query_replication_group_info(&self, rg_filter: &crate::types::structs::ReplicationGroupFilter) -> Result<Option<Vec<Box<dyn crate::types::traits::GroupOperationResultTrait>>>> {
        let input = QueryReplicationGroupInfoRequestType {rg_filter, };
        let path = format!("/sms/SmsStorageManager/{moId}/QueryReplicationGroupInfo", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<Box<dyn crate::types::traits::GroupOperationResultTrait>>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Query storage containers that are retrieved from VASA providers.
    /// 
    /// Stretched container with
    /// UNREPORTED sync status is not included in returned result.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### container_spec
    /// *StorageContainerSpec*
    ///
    /// ## Returns:
    ///
    /// *StorageContainerResult*
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the input provided as part of *StorageContainerSpec* is not found.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while processing the query request.
    pub async fn query_storage_container(&self, container_spec: Option<&crate::types::structs::StorageContainerSpec>) -> Result<Option<crate::types::structs::StorageContainerResult>> {
        let input = QueryStorageContainerRequestType {container_spec, };
        let path = format!("/sms/SmsStorageManager/{moId}/QueryStorageContainer", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<crate::types::structs::StorageContainerResult>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Get VMFS Datastore managed entity that are associated with
    /// StorageLun.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### scsi_3_id
    /// *StorageLun.uuid* for the StorageLun object
    ///
    /// ### array_id
    /// *StorageArray.uuid* for the StorageArray
    /// object.
    ///
    /// ## Returns:
    ///
    /// Vmfs datastore for the file system id.
    /// 
    /// Refers instance of *Datastore*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the specified entity does not exist.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while
    /// processing the query request.
    pub async fn query_vmfs_datastore_associated_with_lun(&self, scsi_3_id: &str, array_id: &str) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let input = QueryVmfsDatastoreAssociatedWithLunRequestType {scsi_3_id, array_id, };
        let path = format!("/sms/SmsStorageManager/{moId}/QueryVmfsDatastoreAssociatedWithLun", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<crate::types::structs::ManagedObjectReference>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// SMS pushes the latest CA root certificates and CRLs to all registered VASA providers.
    /// 
    /// ***Required privileges:*** StorageViews.ConfigureService
    ///
    /// ## Parameters:
    ///
    /// ### provider_id
    /// *SmsProviderInfo.uid* for providers
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if there exists no provider for a *SmsProviderInfo.uid* in providerId
    /// 
    /// ***InvalidArgument***: if a *SmsProviderInfo.uid* in providerId is invalid.
    /// 
    /// ***CertificateRefreshFailed***: if an error is encountered while refreshing
    /// root certificates and CRLs for any provider.
    pub async fn sms_refresh_ca_certificates_and_cr_ls_task(&self, provider_id: Option<&[String]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = SmsRefreshCaCertificatesAndCrLsRequestType {provider_id, };
        let path = format!("/sms/SmsStorageManager/{moId}/SmsRefreshCACertificatesAndCRLs_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::ManagedObjectReference = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Register the provider and issue a sync operation on it.
    /// 
    /// ***Required privileges:*** StorageViews.ConfigureService
    ///
    /// ## Parameters:
    ///
    /// ### provider_spec
    /// *SmsProviderSpec*
    /// containing parameters needed to register the
    /// provider
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if invalid input is provided.
    /// 
    /// ***AlreadyExists***: if the provider already exists.
    /// 
    /// ***ProviderRegistrationFault***: if an error is encountered during the
    /// registration operation. For instance, *IncorrectUsernamePassword*
    /// is thrown if the login credentials are incorrect. *CertificateNotTrusted*
    /// is thrown if the provider identifies itself with an untrusted certificate.
    pub async fn register_provider_task(&self, provider_spec: &dyn crate::types::traits::SmsProviderSpecTrait) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RegisterProviderRequestType {provider_spec, };
        let path = format!("/sms/SmsStorageManager/{moId}/RegisterProvider_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::ManagedObjectReference = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Unregister the provider.
    /// 
    /// ***Required privileges:*** StorageViews.ConfigureService
    ///
    /// ## Parameters:
    ///
    /// ### provider_id
    /// *SmsProviderInfo.uid* for
    /// the provider
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if invalid input is provided.
    /// 
    /// ***NotFound***: if the specified entity does not exist.
    /// 
    /// ***ProviderUnregistrationFault***: if provider service is not available or
    /// any exception is thrown by the VASA provider
    /// during unregister provider.
    pub async fn unregister_provider_task(&self, provider_id: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = UnregisterProviderRequestType {provider_id, };
        let path = format!("/sms/SmsStorageManager/{moId}/UnregisterProvider_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::ManagedObjectReference = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Upgrade VASA Provider registered to vCenter/SMS to maximum common version supported by both
    /// VASA Provider and SMS.
    /// 
    /// ***Required privileges:*** StorageViews.ConfigureService
    ///
    /// ## Parameters:
    ///
    /// ### upgrade_spec
    /// *VASAProviderUpgradeSpec* containing parameter to upgrade the
    /// VASA Provider. If spec is for non VVOL VASA Provider, then exception is thrown.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***SmsFault***: If there is any error encountered while processing request
    pub async fn upgrade_vasa_provider_task(&self, upgrade_spec: &crate::types::structs::VasaProviderUpgradeSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = UpgradeVasaProviderRequestType {upgrade_spec, };
        let path = format!("/sms/SmsStorageManager/{moId}/UpgradeVASAProvider_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::ManagedObjectReference = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryArrayRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "providerId")]
    provider_id: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryArrayAssociatedWithLunRequestType<'a> {
    #[serde(rename = "canonicalName")]
    canonical_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryAssociatedBackingStoragePoolRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "entityId")]
    entity_id: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "entityType")]
    entity_type: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryDatastoreBackingPoolMappingRequestType<'a> {
    datastore: &'a [crate::types::structs::ManagedObjectReference],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryDatastoreCapabilityRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryDrsMigrationCapabilityForPerformanceRequestType<'a> {
    #[serde(rename = "srcDatastore")]
    src_datastore: &'a crate::types::structs::ManagedObjectReference,
    #[serde(rename = "dstDatastore")]
    dst_datastore: &'a crate::types::structs::ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryDrsMigrationCapabilityForPerformanceExRequestType<'a> {
    datastore: &'a [crate::types::structs::ManagedObjectReference],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryFaultDomainRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    filter: Option<&'a crate::types::structs::FaultDomainFilter>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryFileSystemAssociatedWithArrayRequestType<'a> {
    #[serde(rename = "arrayId")]
    array_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryHostAssociatedWithLunRequestType<'a> {
    #[serde(rename = "scsi3Id")]
    scsi_3_id: &'a str,
    #[serde(rename = "arrayId")]
    array_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryLunAssociatedWithArrayRequestType<'a> {
    #[serde(rename = "arrayId")]
    array_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryLunAssociatedWithPortRequestType<'a> {
    #[serde(rename = "portId")]
    port_id: &'a str,
    #[serde(rename = "arrayId")]
    array_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryNfsDatastoreAssociatedWithFileSystemRequestType<'a> {
    #[serde(rename = "fileSystemId")]
    file_system_id: &'a str,
    #[serde(rename = "arrayId")]
    array_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryPortAssociatedWithArrayRequestType<'a> {
    #[serde(rename = "arrayId")]
    array_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryPortAssociatedWithLunRequestType<'a> {
    #[serde(rename = "scsi3Id")]
    scsi_3_id: &'a str,
    #[serde(rename = "arrayId")]
    array_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryPortAssociatedWithProcessorRequestType<'a> {
    #[serde(rename = "processorId")]
    processor_id: &'a str,
    #[serde(rename = "arrayId")]
    array_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryProcessorAssociatedWithArrayRequestType<'a> {
    #[serde(rename = "arrayId")]
    array_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryReplicationGroupInfoRequestType<'a> {
    #[serde(rename = "rgFilter")]
    rg_filter: &'a crate::types::structs::ReplicationGroupFilter,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryStorageContainerRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "containerSpec")]
    container_spec: Option<&'a crate::types::structs::StorageContainerSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryVmfsDatastoreAssociatedWithLunRequestType<'a> {
    #[serde(rename = "scsi3Id")]
    scsi_3_id: &'a str,
    #[serde(rename = "arrayId")]
    array_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "SmsRefreshCACertificatesAndCRLsRequestType", tag = "_typeName")]
struct SmsRefreshCaCertificatesAndCrLsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "providerId")]
    provider_id: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RegisterProviderRequestType<'a> {
    #[serde(rename = "providerSpec")]
    provider_spec: &'a dyn crate::types::traits::SmsProviderSpecTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UnregisterProviderRequestType<'a> {
    #[serde(rename = "providerId")]
    provider_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "UpgradeVASAProviderRequestType", tag = "_typeName")]
struct UpgradeVasaProviderRequestType<'a> {
    #[serde(rename = "upgradeSpec")]
    upgrade_spec: &'a crate::types::structs::VasaProviderUpgradeSpec,
}
