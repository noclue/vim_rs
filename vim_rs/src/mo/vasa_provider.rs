use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// VASA(vStorage APIs for Storage Awareness) provider
/// definition.
#[derive(Clone)]
pub struct VasaProvider {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VasaProvider {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Failover the specified device groups.
    /// 
    /// This function will always be called
    /// at the replication target location.
    /// 
    /// ***Required privileges:*** StorageViews.ConfigureService
    ///
    /// ## Parameters:
    ///
    /// ### failover_param
    /// Settings for the failover.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if failoverParam is null or contains invalid data.
    /// 
    /// ***NotImplemented***: if the provider does not implement this function.
    /// 
    /// ***ProviderUnavailable***: if the provider is temporarily unavailable.
    /// 
    /// ***ProviderOutOfResource***: if it is not possible to perform the operation
    /// due to lack of resources.
    /// 
    /// ***InactiveProvider***: if the provider is inactive for the specified entity.
    /// 
    /// ***TooMany***: Thrown if the Provider is unable to handle the given set of
    /// replication groups in one call. The client needs to call this method based
    /// on the maxBatchSize specified in the TooMany fault. If the maxBatchSize is
    /// not specified, the client is expected to call the function for each group
    /// individually (i.e. maxBatchSize = 1).
    /// 
    /// ***ProviderBusy***: if the provider is busy and cannot process the request.
    /// 
    /// ***SmsReplicationFault***: if an error is encountered while processing the request.
    pub async fn failover_replication_group_task(&self, failover_param: &dyn crate::types::traits::FailoverParamTrait) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = FailoverReplicationGroupRequestType {failover_param, };
        let path = format!("/sms/VasaProvider/{moId}/FailoverReplicationGroup_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Prepare to fail over the specified replication groups.
    /// 
    /// This function is always
    /// called at the replication source location.
    /// 
    /// ***Required privileges:*** StorageViews.ConfigureService
    ///
    /// ## Parameters:
    ///
    /// ### group_id
    /// List of replication group IDs.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if groupId is null or empty.
    /// 
    /// ***NotImplemented***: if the provider does not implement this function.
    /// 
    /// ***ProviderUnavailable***: if the provider is temporarily unavailable.
    /// 
    /// ***ProviderOutOfResource***: if it is not possible to perform the operation
    /// due to lack of resources.
    /// 
    /// ***TooMany***: Thrown if the Provider is unable to handle the given set of
    /// replication groups in one call. The client needs to call this method based
    /// on the maxBatchSize specified in the TooMany fault. If the maxBatchSize is
    /// not specified, the client is expected to call the function for each group
    /// individually (i.e. maxBatchSize = 1).
    /// 
    /// ***InactiveProvider***: if the provider is inactive for the specified entity.
    /// 
    /// ***ProviderBusy***: if the provider is busy and cannot process the request.
    /// 
    /// ***SmsReplicationFault***: if an error is encountered while processing the request.
    pub async fn prepare_failover_replication_group_task(&self, group_id: Option<&[crate::types::structs::ReplicationGroupId]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = PrepareFailoverReplicationGroupRequestType {group_id, };
        let path = format!("/sms/VasaProvider/{moId}/PrepareFailoverReplicationGroup_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Promotes the replication groups currently *INTEST*
    /// to *FAILEDOVER*.
    /// 
    /// This
    /// function must be called at the replication target location.
    /// 
    /// ***Required privileges:*** StorageViews.ConfigureService
    ///
    /// ## Parameters:
    ///
    /// ### promote_param
    /// Specifies an array of replication group IDs whose
    /// in-test devices (*INTEST*) need to be
    /// promoted to failover *FAILEDOVER* state.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if promoteParam is null or contains invalid data.
    /// 
    /// ***NotImplemented***: if the provider does not implement this function.
    /// 
    /// ***ProviderUnavailable***: if the provider is temporarily unavailable.
    /// 
    /// ***ProviderOutOfResource***: if it is not possible to perform the operation
    /// due to lack of resources.
    /// 
    /// ***InactiveProvider***: if the provider is inactive for the specified entity.
    /// 
    /// ***TooMany***: Thrown if the Provider is unable to handle the given set of
    /// replication groups in one call. The client needs to call this method based
    /// on the maxBatchSize specified in the TooMany fault. If the maxBatchSize is
    /// not specified, the client is expected to call the function for each group
    /// individually (i.e. maxBatchSize = 1).
    /// 
    /// ***ProviderBusy***: if the provider is busy and cannot process the request.
    /// 
    /// ***SmsReplicationFault***: if an error is encountered while processing the request.
    pub async fn promote_replication_group_task(&self, promote_param: &crate::types::structs::PromoteParam) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = PromoteReplicationGroupRequestType {promote_param, };
        let path = format!("/sms/VasaProvider/{moId}/PromoteReplicationGroup_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Query for the currently active alarms known to this VASA provider.
    /// 
    /// Provider is expected to return Red and Yellow types of alarms only.
    /// No Green alarms should be included in the result for this API.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### alarm_filter
    /// Filter criteria for the alarm state.
    ///
    /// ## Returns:
    ///
    /// *AlarmResult* containing all (or requested) active alarm objects owned
    /// by the VASA provider.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if invalid input is provided.
    /// 
    /// ***NotImplemented***: if the provider does not implement this function.
    /// 
    /// ***NotFound***: if the specified entity does not exist.
    /// 
    /// ***ProviderBusy***: if the provider is busy and cannot process the request.
    /// 
    /// ***InactiveProvider***: if the provider is inactive for the specified entity.
    /// 
    /// ***ProviderUnavailable***: if the provider is temporarily unavailable.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while processing the
    /// query request.
    pub async fn query_active_alarm(&self, alarm_filter: Option<&crate::types::structs::AlarmFilter>) -> Result<Option<crate::types::structs::AlarmResult>> {
        let input = QueryActiveAlarmRequestType {alarm_filter, };
        let path = format!("/sms/VasaProvider/{moId}/QueryActiveAlarm", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::AlarmResult>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Query for the point-in-time replicas available at the target location.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### group_id
    /// List of replication group IDs.
    ///
    /// ### query_param
    /// Search criteria specification for all the groups.
    ///
    /// ## Returns:
    ///
    /// An array of GroupOperationResult elements.
    /// 
    /// Each of these elements is either *GroupErrorResult* or
    /// *QueryPointInTimeReplicaSuccessResult* or
    /// *QueryPointInTimeReplicaSummaryResult* for CDP capable replicators.
    /// 
    /// The fault in the result entry can be set to:
    /// - *NotFound* if the replication group identifier is not present.
    /// - *DuplicateEntry* if the replication group identifier is duplicate.
    /// - *TooMany* if the number of entries is too large to be returned in one call.
    /// - *QueryExecutionFault* for any other error.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if groupId is null or empty, or queryParam is invalid.
    /// 
    /// ***NotImplemented***: if the provider does not implement this function.
    /// 
    /// ***ProviderUnavailable***: if the provider is temporarily unavailable.
    /// 
    /// ***InactiveProvider***: if the provider is inactive for the specified
    /// replication groups.
    /// 
    /// ***ProviderBusy***: if the provider is busy and cannot process the request.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while processing
    /// the query request.
    pub async fn query_point_in_time_replica(&self, group_id: Option<&[crate::types::structs::ReplicationGroupId]>, query_param: Option<&crate::types::structs::QueryPointInTimeReplicaParam>) -> Result<Option<Vec<Box<dyn crate::types::traits::GroupOperationResultTrait>>>> {
        let input = QueryPointInTimeReplicaRequestType {group_id, query_param, };
        let path = format!("/sms/VasaProvider/{moId}/QueryPointInTimeReplica", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<Box<dyn crate::types::traits::GroupOperationResultTrait>>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Get provider information.
    /// 
    /// ***Required privileges:*** StorageViews.View
    pub async fn query_provider_info(&self) -> Result<Box<dyn crate::types::traits::SmsProviderInfoTrait>> {
        let path = format!("/sms/VasaProvider/{moId}/QueryProviderInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: Box<dyn crate::types::traits::SmsProviderInfoTrait> = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Query for the replication group details.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### group_id
    /// List of replication group IDs.
    ///
    /// ## Returns:
    ///
    /// An array of *GroupOperationResult* elements.
    /// 
    /// If the input array is null or empty, VASA Provider needs to return all
    /// available replication groups. Depending on the number of replication
    /// groups to be returned, VASA Provider can return either a list of
    /// *GroupOperationResult* or a list of
    /// *QueryReplicationGroupSuccessResult*. However, VASA
    /// Provider should not return a hybrid result.
    /// 
    /// If the input array is not empty, VASA Provider needs to return an array
    /// of results, one for each entry in the input. Each entry in the returned
    /// array is either a *QueryReplicationGroupSuccessResult*
    /// (for success), or a *GroupErrorResult* (for failure).
    /// The length of the result arrays must be the same as the input.
    /// 
    /// The fault in the result entry can be set to:
    /// - *NotFound* if the replication group identifier is not present.
    /// - *DuplicateEntry* if the replication group identifier is duplicate.
    /// - *TooMany* if the number of entries is too large to be returned in one call.
    /// - *QueryExecutionFault* for any other error.
    ///
    /// ## Errors:
    ///
    /// ***NotImplemented***: if the provider does not implement this function.
    /// 
    /// ***ProviderUnavailable***: if the provider is temporarily unavailable.
    /// 
    /// ***InactiveProvider***: if the provider is inactive for the specified
    /// replication groups.
    /// 
    /// ***ProviderBusy***: if the provider is busy and cannot process the request.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while processing
    /// the query request.
    pub async fn query_replication_group(&self, group_id: Option<&[crate::types::structs::ReplicationGroupId]>) -> Result<Option<Vec<Box<dyn crate::types::traits::GroupOperationResultTrait>>>> {
        let input = QueryReplicationGroupRequestType {group_id, };
        let path = format!("/sms/VasaProvider/{moId}/QueryReplicationGroup", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<Box<dyn crate::types::traits::GroupOperationResultTrait>>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Query for the replication peer fault domains.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### fault_domain_id
    /// An optional list of source fault domain ID.
    ///
    /// ## Returns:
    ///
    /// An array of *QueryReplicationPeerResult*.
    /// 
    /// If the input array is null or empty, VASA provider needs to return
    /// result for all available source FaultDomain(s). If the input array is
    /// not empty, VASA Provider needs to return one entry in result for each
    /// entry in the input. The length of the input and result arrays must be
    /// same in that case.
    /// 
    /// The fault in the result entry can be set to:
    /// - *NotFound* if the fault domain identifier is not present.
    /// - *DuplicateEntry* if the fault domain identifier is duplicate.
    /// - *TooMany* if the number of entries is too large to be returned in one call.
    /// - *QueryExecutionFault* for any other error.
    ///
    /// ## Errors:
    ///
    /// ***NotImplemented***: if the provider does not implement this function.
    /// 
    /// ***ProviderUnavailable***: if the provider is temporarily unavailable.
    /// 
    /// ***InactiveProvider***: if the provider is inactive for the specified
    /// fault domains.
    /// 
    /// ***ProviderBusy***: if the provider is busy and cannot process the request.
    /// 
    /// ***QueryExecutionFault***: if an error is encountered while processing
    /// the query request.
    pub async fn query_replication_peer(&self, fault_domain_id: Option<&[Box<dyn crate::types::traits::FaultDomainIdTrait>]>) -> Result<Option<Vec<crate::types::structs::QueryReplicationPeerResult>>> {
        let input = QueryReplicationPeerRequestType {fault_domain_id, };
        let path = format!("/sms/VasaProvider/{moId}/QueryReplicationPeer", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::QueryReplicationPeerResult>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Reconnect to the provider.
    /// 
    /// This API will be used to reconnect to a provider that
    /// is in "disconnected" state. If reconnecting fails due to InvalidCertificate exception,
    /// that means the current provider certificate is expired or corrupted. Then user has to
    /// recover the provider following these steps:
    /// 1\. Unregister the provider using *SmsStorageManager.UnregisterProvider_Task*
    /// 2\. Provision a new self signed certificate for the provider
    /// 3\. Register the provider using *SmsStorageManager.RegisterProvider_Task*
    /// If the provider is not in "disconnected" state, this operation will be a no-op.
    /// Note: This API works only for providers that support VASA 2.0 and onwards.
    /// 
    /// ***Required privileges:*** StorageViews.ConfigureService
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidCertificate***: if the provider certificate is invalid
    /// 
    /// ***ProviderConnectionFailed***: if an error is encountered while reconnecting to
    /// the provider.
    pub async fn vasa_provider_reconnect_task(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let path = format!("/sms/VasaProvider/{moId}/VasaProviderReconnect_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Refresh a CA signed certificate for the provider.
    /// 
    /// This API will be used when provider
    /// certificate is about to expire, but still within soft or hard limit window. If the
    /// provider is in "disconnected" state, this operation will be a no-op.
    /// Note: This API works only for providers that support VASA 2.0 and onwards.
    /// 
    /// ***Required privileges:*** StorageViews.ConfigureService
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***CertificateRefreshFailed***: if an error is encountered while refreshing
    /// CA signed certificate for the provider.
    pub async fn vasa_provider_refresh_certificate_task(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let path = format!("/sms/VasaProvider/{moId}/VasaProviderRefreshCertificate_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Initiate replication in the reverse way, making the currently
    /// *FAILEDOVER* devices as sources.
    /// 
    /// ***Required privileges:*** StorageViews.ConfigureService
    ///
    /// ## Parameters:
    ///
    /// ### group_id
    /// Array of replication groups (currently in
    /// *FAILEDOVER* state) that need to be reversed.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if groupId is null or empty.
    /// 
    /// ***NotImplemented***: if the provider does not implement this function.
    /// 
    /// ***ProviderUnavailable***: if the provider is temporarily unavailable.
    /// 
    /// ***ProviderOutOfResource***: if it is not possible to perform the operation
    /// due to lack of resources.
    /// 
    /// ***InactiveProvider***: if the provider is inactive for the specified entity.
    /// 
    /// ***TooMany***: Thrown if the Provider is unable to handle the given set of
    /// replication groups in one call. The client needs to call this method based
    /// on the maxBatchSize specified in the TooMany fault. If the maxBatchSize is
    /// not specified, the client is expected to call the function for each group
    /// individually (i.e. maxBatchSize = 1).
    /// 
    /// ***ProviderBusy***: if the provider is busy and cannot process the request.
    /// 
    /// ***SmsReplicationFault***: if an error is encountered while processing the request.
    pub async fn reverse_replicate_group_task(&self, group_id: Option<&[crate::types::structs::ReplicationGroupId]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ReverseReplicateGroupRequestType {group_id, };
        let path = format!("/sms/VasaProvider/{moId}/ReverseReplicateGroup_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Revoke CA signed certificate of the provider.
    /// 
    /// This API will unregister the
    /// provider automatically.
    /// Note: This API works only for providers that support VASA 2.0 and onwards.
    /// 
    /// ***Required privileges:*** StorageViews.ConfigureService
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***CertificateRevocationFailed***: if an error is encountered while revoking CA signed
    /// certificate of the provider.
    pub async fn vasa_provider_revoke_certificate_task(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let path = format!("/sms/VasaProvider/{moId}/VasaProviderRevokeCertificate_Task", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Issue a sync for the given Storage Array.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Parameters:
    ///
    /// ### array_id
    /// -
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if invalid input is provided.
    /// 
    /// ***ProviderSyncFailed***: if an error is encountered while
    /// executing sync operation for the
    /// provider.
    pub async fn vasa_provider_sync_task(&self, array_id: Option<&str>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VasaProviderSyncRequestType {array_id, };
        let path = format!("/sms/VasaProvider/{moId}/VasaProviderSync_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Synchronize the data between source and replica for the specified
    /// replication group.
    /// 
    /// This function will always be called at the replication
    /// target location.
    /// 
    /// ***Required privileges:*** StorageViews.ConfigureService
    ///
    /// ## Parameters:
    ///
    /// ### group_id
    /// List of replication group IDs.
    ///
    /// ### pit_name
    /// Localized name for the point-in-time snapshot created.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if groupId is null or empty, or pitName is null.
    /// 
    /// ***NotImplemented***: if the provider does not implement this function.
    /// 
    /// ***ProviderUnavailable***: if the provider is temporarily unavailable.
    /// 
    /// ***ProviderOutOfResource***: if it is not possible to perform the operation
    /// due to lack of resources.
    /// 
    /// ***InactiveProvider***: if the provider is inactive for the specified entity.
    /// 
    /// ***ProviderBusy***: if the provider is busy and cannot process the request.
    /// 
    /// ***SmsReplicationFault***: if an error is encountered while processing the request.
    /// 
    /// ***TooMany***: Thrown if the Provider is unable to handle the given set of
    /// replication groups in one call. The client needs to call this method based
    /// on the maxBatchSize specified in the TooMany fault. If the maxBatchSize is
    /// not specified, the client is expected to call the function for each group
    /// individually (i.e. maxBatchSize = 1).
    pub async fn sync_replication_group_task(&self, group_id: Option<&[crate::types::structs::ReplicationGroupId]>, pit_name: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = SyncReplicationGroupRequestType {group_id, pit_name, };
        let path = format!("/sms/VasaProvider/{moId}/SyncReplicationGroup_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Start a test failover for the specified replication groups.
    /// 
    /// This
    /// function will always be called at the replication target location.
    /// 
    /// ***Required privileges:*** StorageViews.ConfigureService
    ///
    /// ## Parameters:
    ///
    /// ### test_failover_param
    /// Settings for the failover.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if testFailoverParam is null or contains invalid data.
    /// 
    /// ***NotImplemented***: if the provider does not implement this function.
    /// 
    /// ***ProviderUnavailable***: if the provider is temporarily unavailable.
    /// 
    /// ***ProviderOutOfResource***: if it is not possible to perform the operation
    /// due to lack of resources.
    /// 
    /// ***InactiveProvider***: if the provider is inactive for the specified entity.
    /// 
    /// ***TooMany***: Thrown if the Provider is unable to handle the given set of
    /// replication groups in one call. The client needs to call this method based
    /// on the maxBatchSize specified in the TooMany fault. If the maxBatchSize is
    /// not specified, the client is expected to call the function for each group
    /// individually (i.e. maxBatchSize = 1).
    /// 
    /// ***ProviderBusy***: if the provider is busy and cannot process the request.
    /// 
    /// ***SmsReplicationFault***: if an error is encountered while processing the request.
    pub async fn test_failover_replication_group_start_task(&self, test_failover_param: &crate::types::structs::TestFailoverParam) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = TestFailoverReplicationGroupStartRequestType {test_failover_param, };
        let path = format!("/sms/VasaProvider/{moId}/TestFailoverReplicationGroupStart_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Stop the ongoing test failover.
    /// 
    /// This function will always be called at
    /// the replication target location.
    /// 
    /// ***Required privileges:*** StorageViews.ConfigureService
    ///
    /// ## Parameters:
    ///
    /// ### group_id
    /// Array of replication groups that need to stop test.
    ///
    /// ### force
    /// \- if true, VP should force-unbind all Virtual Volumes
    /// and move the RG from INTEST to TARGET state. If false, VP will report all the
    /// Virtual Volumes which need to be cleaned up before a failover operation
    /// can be triggered. The default value will be false.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if groupId is null or empty.
    /// 
    /// ***NotImplemented***: if the provider does not implement this function.
    /// 
    /// ***ProviderUnavailable***: if the provider is temporarily unavailable.
    /// 
    /// ***ProviderOutOfResource***: if it is not possible to perform the operation
    /// due to lack of resources.
    /// 
    /// ***InactiveProvider***: if the provider is inactive for the specified
    /// replication groups.
    /// 
    /// ***TooMany***: Thrown if the Provider is unable to handle the given set of
    /// replication groups in one call. The client needs to call this method based
    /// on the maxBatchSize specified in the TooMany fault. If the maxBatchSize is
    /// not specified, the client is expected to call the function for each group
    /// individually (i.e. maxBatchSize = 1).
    /// 
    /// ***ProviderBusy***: if the provider is busy and cannot process the request.
    /// 
    /// ***SmsReplicationFault***: if an error is encountered while processing the request.
    /// 
    /// ***NotSupportedByProvider***: if the provider does not support force operation.
    pub async fn test_failover_replication_group_stop_task(&self, group_id: Option<&[crate::types::structs::ReplicationGroupId]>, force: bool) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = TestFailoverReplicationGroupStopRequestType {group_id, force, };
        let path = format!("/sms/VasaProvider/{moId}/TestFailoverReplicationGroupStop_Task", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct FailoverReplicationGroupRequestType<'a> {
    failover_param: &'a dyn crate::types::traits::FailoverParamTrait,
}

impl<'a> miniserde::Serialize for FailoverReplicationGroupRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(FailoverReplicationGroupRequestTypeSer { data: self, seq: 0 }))
    }
}

struct FailoverReplicationGroupRequestTypeSer<'b, 'a> {
    data: &'b FailoverReplicationGroupRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for FailoverReplicationGroupRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"FailoverReplicationGroupRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("failoverParam"), &self.data.failover_param as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct PrepareFailoverReplicationGroupRequestType<'a> {
    group_id: Option<&'a [crate::types::structs::ReplicationGroupId]>,
}

impl<'a> miniserde::Serialize for PrepareFailoverReplicationGroupRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PrepareFailoverReplicationGroupRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PrepareFailoverReplicationGroupRequestTypeSer<'b, 'a> {
    data: &'b PrepareFailoverReplicationGroupRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for PrepareFailoverReplicationGroupRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PrepareFailoverReplicationGroupRequestType")),
                1 => {
                    let Some(ref val) = self.data.group_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("groupId"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct PromoteReplicationGroupRequestType<'a> {
    promote_param: &'a crate::types::structs::PromoteParam,
}

impl<'a> miniserde::Serialize for PromoteReplicationGroupRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PromoteReplicationGroupRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PromoteReplicationGroupRequestTypeSer<'b, 'a> {
    data: &'b PromoteReplicationGroupRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for PromoteReplicationGroupRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PromoteReplicationGroupRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("promoteParam"), &self.data.promote_param as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryActiveAlarmRequestType<'a> {
    alarm_filter: Option<&'a crate::types::structs::AlarmFilter>,
}

impl<'a> miniserde::Serialize for QueryActiveAlarmRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryActiveAlarmRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryActiveAlarmRequestTypeSer<'b, 'a> {
    data: &'b QueryActiveAlarmRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryActiveAlarmRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryActiveAlarmRequestType")),
                1 => {
                    let Some(ref val) = self.data.alarm_filter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("alarmFilter"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryPointInTimeReplicaRequestType<'a> {
    group_id: Option<&'a [crate::types::structs::ReplicationGroupId]>,
    query_param: Option<&'a crate::types::structs::QueryPointInTimeReplicaParam>,
}

impl<'a> miniserde::Serialize for QueryPointInTimeReplicaRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryPointInTimeReplicaRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryPointInTimeReplicaRequestTypeSer<'b, 'a> {
    data: &'b QueryPointInTimeReplicaRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryPointInTimeReplicaRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryPointInTimeReplicaRequestType")),
                1 => {
                    let Some(ref val) = self.data.group_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("groupId"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.query_param else { continue; };
                    return Some((std::borrow::Cow::Borrowed("queryParam"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryReplicationGroupRequestType<'a> {
    group_id: Option<&'a [crate::types::structs::ReplicationGroupId]>,
}

impl<'a> miniserde::Serialize for QueryReplicationGroupRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryReplicationGroupRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryReplicationGroupRequestTypeSer<'b, 'a> {
    data: &'b QueryReplicationGroupRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryReplicationGroupRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryReplicationGroupRequestType")),
                1 => {
                    let Some(ref val) = self.data.group_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("groupId"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryReplicationPeerRequestType<'a> {
    fault_domain_id: Option<&'a [Box<dyn crate::types::traits::FaultDomainIdTrait>]>,
}

impl<'a> miniserde::Serialize for QueryReplicationPeerRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryReplicationPeerRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryReplicationPeerRequestTypeSer<'b, 'a> {
    data: &'b QueryReplicationPeerRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryReplicationPeerRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryReplicationPeerRequestType")),
                1 => {
                    let Some(ref val) = self.data.fault_domain_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("faultDomainId"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ReverseReplicateGroupRequestType<'a> {
    group_id: Option<&'a [crate::types::structs::ReplicationGroupId]>,
}

impl<'a> miniserde::Serialize for ReverseReplicateGroupRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReverseReplicateGroupRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReverseReplicateGroupRequestTypeSer<'b, 'a> {
    data: &'b ReverseReplicateGroupRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ReverseReplicateGroupRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReverseReplicateGroupRequestType")),
                1 => {
                    let Some(ref val) = self.data.group_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("groupId"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VasaProviderSyncRequestType<'a> {
    array_id: Option<&'a str>,
}

impl<'a> miniserde::Serialize for VasaProviderSyncRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VasaProviderSyncRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VasaProviderSyncRequestTypeSer<'b, 'a> {
    data: &'b VasaProviderSyncRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VasaProviderSyncRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VasaProviderSyncRequestType")),
                1 => {
                    let Some(ref val) = self.data.array_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("arrayId"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct SyncReplicationGroupRequestType<'a> {
    group_id: Option<&'a [crate::types::structs::ReplicationGroupId]>,
    pit_name: &'a str,
}

impl<'a> miniserde::Serialize for SyncReplicationGroupRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SyncReplicationGroupRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SyncReplicationGroupRequestTypeSer<'b, 'a> {
    data: &'b SyncReplicationGroupRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for SyncReplicationGroupRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SyncReplicationGroupRequestType")),
                1 => {
                    let Some(ref val) = self.data.group_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("groupId"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("pitName"), &self.data.pit_name as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct TestFailoverReplicationGroupStartRequestType<'a> {
    test_failover_param: &'a crate::types::structs::TestFailoverParam,
}

impl<'a> miniserde::Serialize for TestFailoverReplicationGroupStartRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(TestFailoverReplicationGroupStartRequestTypeSer { data: self, seq: 0 }))
    }
}

struct TestFailoverReplicationGroupStartRequestTypeSer<'b, 'a> {
    data: &'b TestFailoverReplicationGroupStartRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for TestFailoverReplicationGroupStartRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"TestFailoverReplicationGroupStartRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("testFailoverParam"), &self.data.test_failover_param as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct TestFailoverReplicationGroupStopRequestType<'a> {
    group_id: Option<&'a [crate::types::structs::ReplicationGroupId]>,
    force: bool,
}

impl<'a> miniserde::Serialize for TestFailoverReplicationGroupStopRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(TestFailoverReplicationGroupStopRequestTypeSer { data: self, seq: 0 }))
    }
}

struct TestFailoverReplicationGroupStopRequestTypeSer<'b, 'a> {
    data: &'b TestFailoverReplicationGroupStopRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for TestFailoverReplicationGroupStopRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"TestFailoverReplicationGroupStopRequestType")),
                1 => {
                    let Some(ref val) = self.data.group_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("groupId"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("force"), &self.data.force as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
