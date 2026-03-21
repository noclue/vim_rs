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
        let bytes_opt = self.client.invoke_optional("sms", "SmsStorageManager", &self.mo_id, "QueryArray", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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
        let bytes_opt = self.client.invoke_optional("sms", "SmsStorageManager", &self.mo_id, "QueryArrayAssociatedWithLun", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
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
        let bytes_opt = self.client.invoke_optional("sms", "SmsStorageManager", &self.mo_id, "QueryAssociatedBackingStoragePool", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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
        let bytes = self.client.invoke("sms", "SmsStorageManager", &self.mo_id, "QueryDatastoreBackingPoolMapping", Some(&input)).await?;
        let result: Vec<crate::types::structs::DatastoreBackingPoolMapping> = crate::core::client::unmarshal_array(self.client.transport(), &bytes)?;
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
        let bytes_opt = self.client.invoke_optional("sms", "SmsStorageManager", &self.mo_id, "QueryDatastoreCapability", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
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
        let bytes = self.client.invoke("sms", "SmsStorageManager", &self.mo_id, "QueryDrsMigrationCapabilityForPerformance", Some(&input)).await?;
        let result: bool = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
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
        let bytes = self.client.invoke("sms", "SmsStorageManager", &self.mo_id, "QueryDrsMigrationCapabilityForPerformanceEx", Some(&input)).await?;
        let result: crate::types::structs::DrsMigrationCapabilityResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
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
        let bytes_opt = self.client.invoke_optional("sms", "SmsStorageManager", &self.mo_id, "QueryFaultDomain", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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
        let bytes_opt = self.client.invoke_optional("sms", "SmsStorageManager", &self.mo_id, "QueryFileSystemAssociatedWithArray", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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
        let bytes_opt = self.client.invoke_optional("sms", "SmsStorageManager", &self.mo_id, "QueryHostAssociatedWithLun", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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
        let bytes_opt = self.client.invoke_optional("sms", "SmsStorageManager", &self.mo_id, "QueryLunAssociatedWithArray", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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
        let bytes_opt = self.client.invoke_optional("sms", "SmsStorageManager", &self.mo_id, "QueryLunAssociatedWithPort", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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
        let bytes_opt = self.client.invoke_optional("sms", "SmsStorageManager", &self.mo_id, "QueryNfsDatastoreAssociatedWithFileSystem", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
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
        let bytes_opt = self.client.invoke_optional("sms", "SmsStorageManager", &self.mo_id, "QueryPortAssociatedWithArray", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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
        let bytes_opt = self.client.invoke_optional("sms", "SmsStorageManager", &self.mo_id, "QueryPortAssociatedWithLun", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
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
        let bytes_opt = self.client.invoke_optional("sms", "SmsStorageManager", &self.mo_id, "QueryPortAssociatedWithProcessor", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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
        let bytes_opt = self.client.invoke_optional("sms", "SmsStorageManager", &self.mo_id, "QueryProcessorAssociatedWithArray", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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
        let bytes_opt = self.client.invoke_optional("sms", "SmsStorageManager", &self.mo_id, "QueryProvider", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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
        let bytes_opt = self.client.invoke_optional("sms", "SmsStorageManager", &self.mo_id, "QueryReplicationGroupInfo", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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
        let bytes_opt = self.client.invoke_optional("sms", "SmsStorageManager", &self.mo_id, "QueryStorageContainer", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
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
        let bytes_opt = self.client.invoke_optional("sms", "SmsStorageManager", &self.mo_id, "QueryVmfsDatastoreAssociatedWithLun", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
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
        let bytes = self.client.invoke("sms", "SmsStorageManager", &self.mo_id, "SmsRefreshCACertificatesAndCRLs_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
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
        let bytes = self.client.invoke("sms", "SmsStorageManager", &self.mo_id, "RegisterProvider_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
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
        let bytes = self.client.invoke("sms", "SmsStorageManager", &self.mo_id, "UnregisterProvider_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
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
        let bytes = self.client.invoke("sms", "SmsStorageManager", &self.mo_id, "UpgradeVASAProvider_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct QueryArrayRequestType<'a> {
    provider_id: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for QueryArrayRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryArrayRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryArrayRequestTypeSer<'b, 'a> {
    data: &'b QueryArrayRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryArrayRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryArrayRequestType")),
                1 => {
                    let Some(ref val) = self.data.provider_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("providerId"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryArrayAssociatedWithLunRequestType<'a> {
    canonical_name: &'a str,
}

impl<'a> miniserde::Serialize for QueryArrayAssociatedWithLunRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryArrayAssociatedWithLunRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryArrayAssociatedWithLunRequestTypeSer<'b, 'a> {
    data: &'b QueryArrayAssociatedWithLunRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryArrayAssociatedWithLunRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryArrayAssociatedWithLunRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("canonicalName"), &self.data.canonical_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryAssociatedBackingStoragePoolRequestType<'a> {
    entity_id: Option<&'a str>,
    entity_type: Option<&'a str>,
}

impl<'a> miniserde::Serialize for QueryAssociatedBackingStoragePoolRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryAssociatedBackingStoragePoolRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryAssociatedBackingStoragePoolRequestTypeSer<'b, 'a> {
    data: &'b QueryAssociatedBackingStoragePoolRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryAssociatedBackingStoragePoolRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryAssociatedBackingStoragePoolRequestType")),
                1 => {
                    let Some(ref val) = self.data.entity_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entityId"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.entity_type else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entityType"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryDatastoreBackingPoolMappingRequestType<'a> {
    datastore: &'a [crate::types::structs::ManagedObjectReference],
}

impl<'a> miniserde::Serialize for QueryDatastoreBackingPoolMappingRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryDatastoreBackingPoolMappingRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryDatastoreBackingPoolMappingRequestTypeSer<'b, 'a> {
    data: &'b QueryDatastoreBackingPoolMappingRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryDatastoreBackingPoolMappingRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryDatastoreBackingPoolMappingRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryDatastoreCapabilityRequestType<'a> {
    datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for QueryDatastoreCapabilityRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryDatastoreCapabilityRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryDatastoreCapabilityRequestTypeSer<'b, 'a> {
    data: &'b QueryDatastoreCapabilityRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryDatastoreCapabilityRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryDatastoreCapabilityRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryDrsMigrationCapabilityForPerformanceRequestType<'a> {
    src_datastore: &'a crate::types::structs::ManagedObjectReference,
    dst_datastore: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for QueryDrsMigrationCapabilityForPerformanceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryDrsMigrationCapabilityForPerformanceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryDrsMigrationCapabilityForPerformanceRequestTypeSer<'b, 'a> {
    data: &'b QueryDrsMigrationCapabilityForPerformanceRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryDrsMigrationCapabilityForPerformanceRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryDrsMigrationCapabilityForPerformanceRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("srcDatastore"), &self.data.src_datastore as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("dstDatastore"), &self.data.dst_datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryDrsMigrationCapabilityForPerformanceExRequestType<'a> {
    datastore: &'a [crate::types::structs::ManagedObjectReference],
}

impl<'a> miniserde::Serialize for QueryDrsMigrationCapabilityForPerformanceExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryDrsMigrationCapabilityForPerformanceExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryDrsMigrationCapabilityForPerformanceExRequestTypeSer<'b, 'a> {
    data: &'b QueryDrsMigrationCapabilityForPerformanceExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryDrsMigrationCapabilityForPerformanceExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryDrsMigrationCapabilityForPerformanceExRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datastore"), &self.data.datastore as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryFaultDomainRequestType<'a> {
    filter: Option<&'a crate::types::structs::FaultDomainFilter>,
}

impl<'a> miniserde::Serialize for QueryFaultDomainRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryFaultDomainRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryFaultDomainRequestTypeSer<'b, 'a> {
    data: &'b QueryFaultDomainRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryFaultDomainRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryFaultDomainRequestType")),
                1 => {
                    let Some(ref val) = self.data.filter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("filter"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryFileSystemAssociatedWithArrayRequestType<'a> {
    array_id: &'a str,
}

impl<'a> miniserde::Serialize for QueryFileSystemAssociatedWithArrayRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryFileSystemAssociatedWithArrayRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryFileSystemAssociatedWithArrayRequestTypeSer<'b, 'a> {
    data: &'b QueryFileSystemAssociatedWithArrayRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryFileSystemAssociatedWithArrayRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryFileSystemAssociatedWithArrayRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("arrayId"), &self.data.array_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryHostAssociatedWithLunRequestType<'a> {
    scsi_3_id: &'a str,
    array_id: &'a str,
}

impl<'a> miniserde::Serialize for QueryHostAssociatedWithLunRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryHostAssociatedWithLunRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryHostAssociatedWithLunRequestTypeSer<'b, 'a> {
    data: &'b QueryHostAssociatedWithLunRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryHostAssociatedWithLunRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryHostAssociatedWithLunRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("scsi3Id"), &self.data.scsi_3_id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("arrayId"), &self.data.array_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryLunAssociatedWithArrayRequestType<'a> {
    array_id: &'a str,
}

impl<'a> miniserde::Serialize for QueryLunAssociatedWithArrayRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryLunAssociatedWithArrayRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryLunAssociatedWithArrayRequestTypeSer<'b, 'a> {
    data: &'b QueryLunAssociatedWithArrayRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryLunAssociatedWithArrayRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryLunAssociatedWithArrayRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("arrayId"), &self.data.array_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryLunAssociatedWithPortRequestType<'a> {
    port_id: &'a str,
    array_id: &'a str,
}

impl<'a> miniserde::Serialize for QueryLunAssociatedWithPortRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryLunAssociatedWithPortRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryLunAssociatedWithPortRequestTypeSer<'b, 'a> {
    data: &'b QueryLunAssociatedWithPortRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryLunAssociatedWithPortRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryLunAssociatedWithPortRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("portId"), &self.data.port_id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("arrayId"), &self.data.array_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryNfsDatastoreAssociatedWithFileSystemRequestType<'a> {
    file_system_id: &'a str,
    array_id: &'a str,
}

impl<'a> miniserde::Serialize for QueryNfsDatastoreAssociatedWithFileSystemRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryNfsDatastoreAssociatedWithFileSystemRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryNfsDatastoreAssociatedWithFileSystemRequestTypeSer<'b, 'a> {
    data: &'b QueryNfsDatastoreAssociatedWithFileSystemRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryNfsDatastoreAssociatedWithFileSystemRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryNfsDatastoreAssociatedWithFileSystemRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("fileSystemId"), &self.data.file_system_id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("arrayId"), &self.data.array_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryPortAssociatedWithArrayRequestType<'a> {
    array_id: &'a str,
}

impl<'a> miniserde::Serialize for QueryPortAssociatedWithArrayRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryPortAssociatedWithArrayRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryPortAssociatedWithArrayRequestTypeSer<'b, 'a> {
    data: &'b QueryPortAssociatedWithArrayRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryPortAssociatedWithArrayRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryPortAssociatedWithArrayRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("arrayId"), &self.data.array_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryPortAssociatedWithLunRequestType<'a> {
    scsi_3_id: &'a str,
    array_id: &'a str,
}

impl<'a> miniserde::Serialize for QueryPortAssociatedWithLunRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryPortAssociatedWithLunRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryPortAssociatedWithLunRequestTypeSer<'b, 'a> {
    data: &'b QueryPortAssociatedWithLunRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryPortAssociatedWithLunRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryPortAssociatedWithLunRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("scsi3Id"), &self.data.scsi_3_id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("arrayId"), &self.data.array_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryPortAssociatedWithProcessorRequestType<'a> {
    processor_id: &'a str,
    array_id: &'a str,
}

impl<'a> miniserde::Serialize for QueryPortAssociatedWithProcessorRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryPortAssociatedWithProcessorRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryPortAssociatedWithProcessorRequestTypeSer<'b, 'a> {
    data: &'b QueryPortAssociatedWithProcessorRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryPortAssociatedWithProcessorRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryPortAssociatedWithProcessorRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("processorId"), &self.data.processor_id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("arrayId"), &self.data.array_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryProcessorAssociatedWithArrayRequestType<'a> {
    array_id: &'a str,
}

impl<'a> miniserde::Serialize for QueryProcessorAssociatedWithArrayRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryProcessorAssociatedWithArrayRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryProcessorAssociatedWithArrayRequestTypeSer<'b, 'a> {
    data: &'b QueryProcessorAssociatedWithArrayRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryProcessorAssociatedWithArrayRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryProcessorAssociatedWithArrayRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("arrayId"), &self.data.array_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryReplicationGroupInfoRequestType<'a> {
    rg_filter: &'a crate::types::structs::ReplicationGroupFilter,
}

impl<'a> miniserde::Serialize for QueryReplicationGroupInfoRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryReplicationGroupInfoRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryReplicationGroupInfoRequestTypeSer<'b, 'a> {
    data: &'b QueryReplicationGroupInfoRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryReplicationGroupInfoRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryReplicationGroupInfoRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("rgFilter"), &self.data.rg_filter as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryStorageContainerRequestType<'a> {
    container_spec: Option<&'a crate::types::structs::StorageContainerSpec>,
}

impl<'a> miniserde::Serialize for QueryStorageContainerRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryStorageContainerRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryStorageContainerRequestTypeSer<'b, 'a> {
    data: &'b QueryStorageContainerRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryStorageContainerRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryStorageContainerRequestType")),
                1 => {
                    let Some(ref val) = self.data.container_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("containerSpec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryVmfsDatastoreAssociatedWithLunRequestType<'a> {
    scsi_3_id: &'a str,
    array_id: &'a str,
}

impl<'a> miniserde::Serialize for QueryVmfsDatastoreAssociatedWithLunRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryVmfsDatastoreAssociatedWithLunRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryVmfsDatastoreAssociatedWithLunRequestTypeSer<'b, 'a> {
    data: &'b QueryVmfsDatastoreAssociatedWithLunRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryVmfsDatastoreAssociatedWithLunRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryVmfsDatastoreAssociatedWithLunRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("scsi3Id"), &self.data.scsi_3_id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("arrayId"), &self.data.array_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct SmsRefreshCaCertificatesAndCrLsRequestType<'a> {
    provider_id: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for SmsRefreshCaCertificatesAndCrLsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SmsRefreshCaCertificatesAndCrLsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SmsRefreshCaCertificatesAndCrLsRequestTypeSer<'b, 'a> {
    data: &'b SmsRefreshCaCertificatesAndCrLsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for SmsRefreshCaCertificatesAndCrLsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SmsRefreshCACertificatesAndCRLsRequestType")),
                1 => {
                    let Some(ref val) = self.data.provider_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("providerId"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RegisterProviderRequestType<'a> {
    provider_spec: &'a dyn crate::types::traits::SmsProviderSpecTrait,
}

impl<'a> miniserde::Serialize for RegisterProviderRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RegisterProviderRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RegisterProviderRequestTypeSer<'b, 'a> {
    data: &'b RegisterProviderRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RegisterProviderRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RegisterProviderRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("providerSpec"), &self.data.provider_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UnregisterProviderRequestType<'a> {
    provider_id: &'a str,
}

impl<'a> miniserde::Serialize for UnregisterProviderRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UnregisterProviderRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UnregisterProviderRequestTypeSer<'b, 'a> {
    data: &'b UnregisterProviderRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UnregisterProviderRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UnregisterProviderRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("providerId"), &self.data.provider_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpgradeVasaProviderRequestType<'a> {
    upgrade_spec: &'a crate::types::structs::VasaProviderUpgradeSpec,
}

impl<'a> miniserde::Serialize for UpgradeVasaProviderRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpgradeVasaProviderRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpgradeVasaProviderRequestTypeSer<'b, 'a> {
    data: &'b UpgradeVasaProviderRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpgradeVasaProviderRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpgradeVASAProviderRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("upgradeSpec"), &self.data.upgrade_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
