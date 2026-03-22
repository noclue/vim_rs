use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Singleton Managed Object used to manage cryptographic keys.
#[derive(Clone)]
pub struct CryptoManagerKmip {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl CryptoManagerKmip {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Check whether an active KMS exists in cluster.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// \[in\] KMIP cluster.
    /// Will use default cluster if omitted.
    ///
    /// ## Returns:
    ///
    /// true if active KMS exists in cluster, false otherwise.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: in case the cluster is not found.
    pub async fn is_kms_cluster_active(&self, cluster: Option<&crate::types::structs::KeyProviderId>) -> Result<bool> {
        let input = IsKmsClusterActiveRequestType {cluster, };
        let bytes = self.client.invoke("", "CryptoManagerKmip", &self.mo_id, "IsKmsClusterActive", Some(&input)).await?;
        let result: bool = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Add an existing key.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeys
    ///
    /// ## Parameters:
    ///
    /// ### key
    /// \[in\] The cryptographic key to add.
    ///
    /// ## Errors:
    ///
    /// ***AlreadyExists***: in case the key is already in the key cache
    /// 
    /// ***InvalidState***: in case the host is not Crypto Safe
    /// 
    /// ***InvalidArgument***: in case the keyID is duplicated or key properties
    /// are incorrect.
    pub async fn add_key(&self, key: &crate::types::structs::CryptoKeyPlain) -> Result<()> {
        let input = AddKeyRequestType {key, };
        self.client.invoke_void("", "CryptoManagerKmip", &self.mo_id, "AddKey", Some(&input)).await
    }
    /// Add multiple existing keys.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeys
    ///
    /// ## Parameters:
    ///
    /// ### keys
    /// \[in\] List of cryptographic keys to add.
    ///
    /// ## Returns:
    ///
    /// the result for each key operation.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: in case the host is not Crypto Safe
    pub async fn add_keys(&self, keys: Option<&[crate::types::structs::CryptoKeyPlain]>) -> Result<Option<Vec<crate::types::structs::CryptoKeyResult>>> {
        let input = AddKeysRequestType {keys, };
        let bytes_opt = self.client.invoke_optional("", "CryptoManagerKmip", &self.mo_id, "AddKeys", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Generate a certificate signing request with its private key.
    /// 
    /// This generates a CSR request as well as its private key. The private key
    /// will not be returned to caller for security protection. If this method
    /// is called again, the CSR and private key generated in the new invocation
    /// will overwrite the old ones.
    /// After the CSR is signed by KMS into a certificate, it should be updated
    /// by calling *CryptoManagerKmip.UpdateKmsSignedCsrClientCert*. The generated CSR
    /// can be later retrieved by calling *CryptoManagerKmip.RetrieveClientCsr*.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// \[in\] KMIP cluster.
    ///
    /// ### request
    /// \[in\] Certificate sign request.
    /// 
    /// ***Since:*** vSphere API Release 8.0.1.0
    ///
    /// ## Returns:
    ///
    /// A newly generated CSR.
    pub async fn generate_client_csr(&self, cluster: &crate::types::structs::KeyProviderId, request: Option<&crate::types::structs::CryptoManagerKmipCertSignRequest>) -> Result<String> {
        let input = GenerateClientCsrRequestType {cluster, request, };
        let bytes = self.client.invoke("", "CryptoManagerKmip", &self.mo_id, "GenerateClientCsr", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Generate new encryption key.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeys
    ///
    /// ## Parameters:
    ///
    /// ### key_provider
    /// \[in\] Which provider will generate the key.
    /// If omitted, will use the default key provider.
    ///
    /// ### spec
    /// \[in\] The spec that contains custom attributes key/value pairs.
    /// 
    /// ***Since:*** vSphere API Release 8.0.1.0
    ///
    /// ### key_spec
    /// \[in\] The keySpec that contains key generation options.
    /// If unset, key will be generated with default settings
    /// of the key provider.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    ///
    /// ## Returns:
    ///
    /// the generated key.
    pub async fn generate_key(&self, key_provider: Option<&crate::types::structs::KeyProviderId>, spec: Option<&crate::types::structs::CryptoManagerKmipCustomAttributeSpec>, key_spec: Option<&crate::types::structs::CryptoManagerKmipGenerateKeySpec>) -> Result<crate::types::structs::CryptoKeyResult> {
        let input = GenerateKeyRequestType {key_provider, spec, key_spec, };
        let bytes = self.client.invoke("", "CryptoManagerKmip", &self.mo_id, "GenerateKey", Some(&input)).await?;
        let result: crate::types::structs::CryptoKeyResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Generate a self-signed client certificate with its private key.
    /// 
    /// This generates a self signed certificate as well as its private key.
    /// The private key will not be returned to caller for security protection.
    /// If this method is called again, the certificate and private key
    /// generated in the new invocation will overwrite the old ones.
    /// The generated certificate will not replace current working certificate
    /// until *CryptoManagerKmip.UpdateSelfSignedClientCert* is called. The generated self
    /// signed certificate can be later retrieved by calling
    /// *CryptoManagerKmip.RetrieveSelfSignedClientCert*.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// \[in\] KMIP cluster.
    ///
    /// ### request
    /// \[in\] Certificate sign request.
    /// 
    /// ***Since:*** vSphere API Release 8.0.1.0
    ///
    /// ## Returns:
    ///
    /// A new self-signed client certificate.
    pub async fn generate_self_signed_client_cert(&self, cluster: &crate::types::structs::KeyProviderId, request: Option<&crate::types::structs::CryptoManagerKmipCertSignRequest>) -> Result<String> {
        let input = GenerateSelfSignedClientCertRequestType {cluster, request, };
        let bytes = self.client.invoke("", "CryptoManagerKmip", &self.mo_id, "GenerateSelfSignedClientCert", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Get the default KMS cluster of the specified managed entity.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// \[in\] The entity where the default KMS cluster to get.
    /// If omitted, then return global default KMS cluster.
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ### defaults_to_parent
    /// \[in\] (Optional, default = false) If set to true,
    /// then get the default kms cluster follow the
    /// entity hierarchy. That means if the entity has
    /// no default kms cluster, then try to get from
    /// its parent.
    ///
    /// ## Returns:
    ///
    /// The default kms cluster of the entity, if any.
    pub async fn get_default_kms_cluster(&self, entity: Option<&crate::types::structs::ManagedObjectReference>, defaults_to_parent: Option<bool>) -> Result<Option<crate::types::structs::KeyProviderId>> {
        let input = GetDefaultKmsClusterRequestType {entity, defaults_to_parent, };
        let bytes_opt = self.client.invoke_optional("", "CryptoManagerKmip", &self.mo_id, "GetDefaultKmsCluster", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// List keys.
    /// 
    /// \* When executed against the host, lists all the keys
    /// added to the host's key cache by *CryptoManager.AddKey*/*CryptoManager.AddKeys*.
    /// \* When executed against the VC, lists all the keys used by
    /// the correctly registered VMs, and the host key.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeys
    ///
    /// ## Parameters:
    ///
    /// ### limit
    /// \[in\] maximum keys to return.
    ///
    /// ## Returns:
    ///
    /// List of known keys.
    pub async fn list_keys(&self, limit: Option<i32>) -> Result<Option<Vec<crate::types::structs::CryptoKeyId>>> {
        let input = ListKeysRequestType {limit, };
        let bytes_opt = self.client.invoke_optional("", "CryptoManagerKmip", &self.mo_id, "ListKeys", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// List the registered KMIP servers.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### limit
    /// \[in\] maximum clusters to return.
    ///
    /// ## Returns:
    ///
    /// List of known KMIP servers grouped in clusters.
    pub async fn list_kmip_servers(&self, limit: Option<i32>) -> Result<Option<Vec<crate::types::structs::KmipClusterInfo>>> {
        let input = ListKmipServersRequestType {limit, };
        let bytes_opt = self.client.invoke_optional("", "CryptoManagerKmip", &self.mo_id, "ListKmipServers", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// List the KMS clusters information.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### include_kms_servers
    /// \[in\] Whether to list KMS servers information
    /// in the cluster.
    /// By default will not include the KMS servers
    /// information.
    ///
    /// ### management_type_filter
    /// \[in\] The KMS cluster management type filter.
    /// Bit map values:
    /// 0x01 - Return VC managed Key Providers
    /// registered in the CryptoManager.
    /// 0x02 - Return Trusted Key Providers
    /// registered in the CryptoManager.
    /// 0x04 - Return Trusted Key Providers which are
    /// not registered with the CryptoManager.
    /// 0x08 - Return Native Key Providers.
    /// others - reserved, will be ignored
    /// If omitted or -1, then all kinds of Key Providers
    /// will be returned.
    ///
    /// ### status_filter
    /// \[in\] The Key Provider status filter.
    /// Bit map values:
    /// 0x01 - Return active Key Providers.
    /// 0x02 - Return inactive Key Providers.
    /// others - reserved, will be ignored
    /// If omitted or -1, then all status of Key Providers
    /// will be returned.
    ///
    /// ## Returns:
    ///
    /// List of Key Providers.
    pub async fn list_kms_clusters(&self, include_kms_servers: Option<bool>, management_type_filter: Option<i32>, status_filter: Option<i32>) -> Result<Option<Vec<crate::types::structs::KmipClusterInfo>>> {
        let input = ListKmsClustersRequestType {include_kms_servers, management_type_filter, status_filter, };
        let bytes_opt = self.client.invoke_optional("", "CryptoManagerKmip", &self.mo_id, "ListKmsClusters", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Set the default KMIP cluster.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### cluster_id
    /// \[in\] KMIP cluster ID to become default.
    pub async fn mark_default(&self, cluster_id: &crate::types::structs::KeyProviderId) -> Result<()> {
        let input = MarkDefaultRequestType {cluster_id, };
        self.client.invoke_void("", "CryptoManagerKmip", &self.mo_id, "MarkDefault", Some(&input)).await
    }
    /// Check CryptoKey status, such as if VC can access the key, if the key is
    /// used by some VMs or as host key.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeys
    ///
    /// ## Parameters:
    ///
    /// ### key_ids
    /// \[in\] The Crypto Key Ids to query.
    ///
    /// ### check_key_bit_map
    /// \[in\] The key state to check. Supported value:
    /// 0x01. check if key data is available to VC.
    /// 0x02. check the VMs which use that key.
    /// 0x04. check the hosts using this key as host key.
    /// 0x08. Check 3rd party program which use that key.
    /// Other bits - reserved and will be igonred.
    ///
    /// ## Returns:
    ///
    /// The structure combined with key status. If bit in parameter
    /// is not set when invoke, the returned data in related
    /// CryptoKeyStatus will be unknown.
    pub async fn query_crypto_key_status(&self, key_ids: Option<&[crate::types::structs::CryptoKeyId]>, check_key_bit_map: i32) -> Result<Option<Vec<crate::types::structs::CryptoManagerKmipCryptoKeyStatus>>> {
        let input = QueryCryptoKeyStatusRequestType {key_ids, check_key_bit_map, };
        let bytes_opt = self.client.invoke_optional("", "CryptoManagerKmip", &self.mo_id, "QueryCryptoKeyStatus", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Register a KMIP server.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### server
    /// \[in\] KMIP server connection information.
    /// When register a new KMIP server to the key provider,
    /// the *KmipServerSpec.defaultKeyType* and
    /// *KmipServerSpecWrappingKeyIdKeySpec* must match
    /// existing servers.
    pub async fn register_kmip_server(&self, server: &crate::types::structs::KmipServerSpec) -> Result<()> {
        let input = RegisterKmipServerRequestType {server, };
        self.client.invoke_void("", "CryptoManagerKmip", &self.mo_id, "RegisterKmipServer", Some(&input)).await
    }
    /// Register the specified KMS cluster to the CryptoManager.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### cluster_id
    /// \[in\] KMS cluster ID to register.
    ///
    /// ### management_type
    /// \[in\] Key provider management type
    /// See *KmipClusterInfoKmsManagementType_enum*
    /// for valid values.
    /// By default trustAuthority.
    pub async fn register_kms_cluster(&self, cluster_id: &crate::types::structs::KeyProviderId, management_type: Option<&str>) -> Result<()> {
        let input = RegisterKmsClusterRequestType {cluster_id, management_type, };
        self.client.invoke_void("", "CryptoManagerKmip", &self.mo_id, "RegisterKmsCluster", Some(&input)).await
    }
    /// Remove a key (only the UUID is needed to remove).
    /// 
    /// If "force" is set, removal will happen even if the key is in use.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeys
    ///
    /// ## Parameters:
    ///
    /// ### key
    /// \[in\] The key to remove.
    ///
    /// ### force
    /// \[in\] Remove the key even if in use or not existent.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: in case the keyID is not found
    /// and "force" is false.
    /// 
    /// ***ResourceInUse***: if the key is used to encrypt any object
    /// and "force" is false.
    pub async fn remove_key(&self, key: &crate::types::structs::CryptoKeyId, force: bool) -> Result<()> {
        let input = RemoveKeyRequestType {key, force, };
        self.client.invoke_void("", "CryptoManagerKmip", &self.mo_id, "RemoveKey", Some(&input)).await
    }
    /// Remove multiple keys (only the UUID is needed to remove).
    /// 
    /// If "force" is set, removal will happen even if they are in use.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeys
    ///
    /// ## Parameters:
    ///
    /// ### keys
    /// \[in\] List of keys to remove.
    ///
    /// ### force
    /// \[in\] Remove the key even if in use. Always successful.
    pub async fn remove_keys(&self, keys: Option<&[crate::types::structs::CryptoKeyId]>, force: bool) -> Result<Option<Vec<crate::types::structs::CryptoKeyResult>>> {
        let input = RemoveKeysRequestType {keys, force, };
        let bytes_opt = self.client.invoke_optional("", "CryptoManagerKmip", &self.mo_id, "RemoveKeys", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Remove a KMIP server, even if in use.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### cluster_id
    /// \[in\] KMIP cluster ID.
    ///
    /// ### server_name
    /// \[in\] KMIP server name.
    pub async fn remove_kmip_server(&self, cluster_id: &crate::types::structs::KeyProviderId, server_name: &str) -> Result<()> {
        let input = RemoveKmipServerRequestType {cluster_id, server_name, };
        self.client.invoke_void("", "CryptoManagerKmip", &self.mo_id, "RemoveKmipServer", Some(&input)).await
    }
    /// Get the client certificate of the KMIP cluster.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// \[in\] KMIP cluster.
    ///
    /// ## Returns:
    ///
    /// The client certificate.
    pub async fn retrieve_client_cert(&self, cluster: &crate::types::structs::KeyProviderId) -> Result<String> {
        let input = RetrieveClientCertRequestType {cluster, };
        let bytes = self.client.invoke("", "CryptoManagerKmip", &self.mo_id, "RetrieveClientCert", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Get the generated client certificate signing request.
    /// 
    /// If *CryptoManagerKmip.GenerateClientCsr* is called previously, this will return
    /// the generated certificate signing request; otherwise return empty string.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// \[in\] KMIP cluster.
    ///
    /// ## Returns:
    ///
    /// The CSR generated previously, if any.
    pub async fn retrieve_client_csr(&self, cluster: &crate::types::structs::KeyProviderId) -> Result<String> {
        let input = RetrieveClientCsrRequestType {cluster, };
        let bytes = self.client.invoke("", "CryptoManagerKmip", &self.mo_id, "RetrieveClientCsr", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Get the server certficate.
    /// 
    /// In the case of error, an empty
    /// certificate string is returned.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### key_provider
    /// \[in\] KMIP cluster in which the server is placed
    /// or will be created.
    ///
    /// ### server
    /// \[in\] KMIP server.
    ///
    /// ## Returns:
    ///
    /// Information about the server certificate.
    pub async fn retrieve_kmip_server_cert(&self, key_provider: &crate::types::structs::KeyProviderId, server: &crate::types::structs::KmipServerInfo) -> Result<crate::types::structs::CryptoManagerKmipServerCertInfo> {
        let input = RetrieveKmipServerCertRequestType {key_provider, server, };
        let bytes = self.client.invoke("", "CryptoManagerKmip", &self.mo_id, "RetrieveKmipServerCert", Some(&input)).await?;
        let result: crate::types::structs::CryptoManagerKmipServerCertInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Get the status of the KMIP servers.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### clusters
    /// \[in\] KMIP clusters and their servers.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    pub async fn retrieve_kmip_servers_status_task(&self, clusters: Option<&[crate::types::structs::KmipClusterInfo]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = RetrieveKmipServersStatusRequestType {clusters, };
        let bytes = self.client.invoke("", "CryptoManagerKmip", &self.mo_id, "RetrieveKmipServersStatus_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Get the generated self signed client certificate.
    /// 
    /// If *CryptoManagerKmip.GenerateSelfSignedClientCert* is called previously, this
    /// will return the generated certificate; otherwise return empty string.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// \[in\] KMIP cluster.
    ///
    /// ## Returns:
    ///
    /// The self signed certificate generated previously, if any.
    pub async fn retrieve_self_signed_client_cert(&self, cluster: &crate::types::structs::KeyProviderId) -> Result<String> {
        let input = RetrieveSelfSignedClientCertRequestType {cluster, };
        let bytes = self.client.invoke("", "CryptoManagerKmip", &self.mo_id, "RetrieveSelfSignedClientCert", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Set the default KMS cluster for the specified managed entity.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// \[in\] The managed entity where the default KMS cluster to be
    /// set. Currently the valid managed entity could be
    /// cluster or host folder.
    /// If omitted, then will set global default KMS cluster.
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ### cluster_id
    /// \[in\] KMS cluster ID to become default.
    /// If omitted, then will clear the default KMS cluster
    /// setting.
    pub async fn set_default_kms_cluster(&self, entity: Option<&crate::types::structs::ManagedObjectReference>, cluster_id: Option<&crate::types::structs::KeyProviderId>) -> Result<()> {
        let input = SetDefaultKmsClusterRequestType {entity, cluster_id, };
        self.client.invoke_void("", "CryptoManagerKmip", &self.mo_id, "SetDefaultKmsCluster", Some(&input)).await
    }
    /// Set crypto key's custom attributes.
    /// 
    /// ***Since:*** vSphere API Release 8.0.1.0
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeys
    ///
    /// ## Parameters:
    ///
    /// ### key_id
    /// \[in\] The crypto Key Id.
    ///
    /// ### spec
    /// \[in\] The spec that contains custom attributes key/value pairs.
    ///
    /// ## Returns:
    ///
    /// The structure combined with status and fail reason.
    pub async fn set_key_custom_attributes(&self, key_id: &crate::types::structs::CryptoKeyId, spec: &crate::types::structs::CryptoManagerKmipCustomAttributeSpec) -> Result<crate::types::structs::CryptoKeyResult> {
        let input = SetKeyCustomAttributesRequestType {key_id, spec, };
        let bytes = self.client.invoke("", "CryptoManagerKmip", &self.mo_id, "SetKeyCustomAttributes", Some(&input)).await?;
        let result: crate::types::structs::CryptoKeyResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Unregister the specified KMS cluster from the CryptoManager.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### cluster_id
    /// \[in\] KMS cluster ID to unregister.
    pub async fn unregister_kms_cluster(&self, cluster_id: &crate::types::structs::KeyProviderId) -> Result<()> {
        let input = UnregisterKmsClusterRequestType {cluster_id, };
        self.client.invoke_void("", "CryptoManagerKmip", &self.mo_id, "UnregisterKmsCluster", Some(&input)).await
    }
    /// Update a KMIP server.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### server
    /// \[in\] KMIP server connection information.
    /// When update a KMIP server settings, changes to
    /// *KmipServerSpec.defaultKeyType* and
    /// *KmipServerSpecWrappingKeyIdKeySpec*
    /// will apply to all servers.
    pub async fn update_kmip_server(&self, server: &crate::types::structs::KmipServerSpec) -> Result<()> {
        let input = UpdateKmipServerRequestType {server, };
        self.client.invoke_void("", "CryptoManagerKmip", &self.mo_id, "UpdateKmipServer", Some(&input)).await
    }
    /// Set KMS server signed certificate as KMIP client certificate for the KMS
    /// cluster.
    /// 
    /// This method should be called to update the certificate signed by KMS
    /// server from a CSR that is generated by calling *CryptoManagerKmip.GenerateClientCsr*.
    /// If *CryptoManagerKmip.GenerateClientCsr* is called more than once, the CSR that is
    /// generated last time should be used; otherwise the certificate will be
    /// rejected because the private key from last time won't match the public
    /// key in the certificate.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// \[in\] KMIP cluster.
    ///
    /// ### certificate
    /// \[in\] Client certificate.
    pub async fn update_kms_signed_csr_client_cert(&self, cluster: &crate::types::structs::KeyProviderId, certificate: &str) -> Result<()> {
        let input = UpdateKmsSignedCsrClientCertRequestType {cluster, certificate, };
        self.client.invoke_void("", "CryptoManagerKmip", &self.mo_id, "UpdateKmsSignedCsrClientCert", Some(&input)).await
    }
    /// Set a self-signed certificate as KMIP client certificate for the KMS
    /// cluster.
    /// 
    /// This method should be called to update the certificate which is generated
    /// by calling *CryptoManagerKmip.GenerateSelfSignedClientCert*. If
    /// *CryptoManagerKmip.GenerateSelfSignedClientCert* is called more than once, the self
    /// signed certificate that is generated last time should be used; otherwise
    /// the certificate will be rejected because the private key from last time
    /// won't match the public key in the certificate.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// \[in\] KMIP cluster.
    ///
    /// ### certificate
    /// \[in\] Client certificate.
    pub async fn update_self_signed_client_cert(&self, cluster: &crate::types::structs::KeyProviderId, certificate: &str) -> Result<()> {
        let input = UpdateSelfSignedClientCertRequestType {cluster, certificate, };
        self.client.invoke_void("", "CryptoManagerKmip", &self.mo_id, "UpdateSelfSignedClientCert", Some(&input)).await
    }
    /// Set a client certificate with private key for the KMIP cluster.
    /// 
    /// The certificate and private key can be assigned by a KMS server and the
    /// certificate might be already trusted by the KMS server.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// \[in\] KMIP cluster.
    ///
    /// ### certificate
    /// \[in\] Client certificate.
    ///
    /// ### private_key
    /// \[in\] Private key.
    pub async fn upload_client_cert(&self, cluster: &crate::types::structs::KeyProviderId, certificate: &str, private_key: &str) -> Result<()> {
        let input = UploadClientCertRequestType {cluster, certificate, private_key, };
        self.client.invoke_void("", "CryptoManagerKmip", &self.mo_id, "UploadClientCert", Some(&input)).await
    }
    /// Upload a server certficate.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// \[in\] KMIP cluster.
    ///
    /// ### certificate
    /// \[in\] Server certificate in PEM encoding.
    pub async fn upload_kmip_server_cert(&self, cluster: &crate::types::structs::KeyProviderId, certificate: &str) -> Result<()> {
        let input = UploadKmipServerCertRequestType {cluster, certificate, };
        self.client.invoke_void("", "CryptoManagerKmip", &self.mo_id, "UploadKmipServerCert", Some(&input)).await
    }
    /// Indicate if the encryption feature is enabled.
    pub async fn enabled(&self) -> Result<bool> {
        let pv_opt = self.client.fetch_property_raw("", "CryptoManagerKmip", &self.mo_id, "enabled").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property enabled was empty".to_string()))?;
        let result: bool = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// A list of registered KMIP servers, grouped by clusters.
    pub async fn kmip_servers(&self) -> Result<Option<Vec<crate::types::structs::KmipClusterInfo>>> {
        let pv_opt = self.client.fetch_property_raw("", "CryptoManagerKmip", &self.mo_id, "kmipServers").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
}
struct IsKmsClusterActiveRequestType<'a> {
    cluster: Option<&'a crate::types::structs::KeyProviderId>,
}

impl<'a> miniserde::Serialize for IsKmsClusterActiveRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(IsKmsClusterActiveRequestTypeSer { data: self, seq: 0 }))
    }
}

struct IsKmsClusterActiveRequestTypeSer<'b, 'a> {
    data: &'b IsKmsClusterActiveRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for IsKmsClusterActiveRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"IsKmsClusterActiveRequestType")),
                1 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct AddKeyRequestType<'a> {
    key: &'a crate::types::structs::CryptoKeyPlain,
}

impl<'a> miniserde::Serialize for AddKeyRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AddKeyRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AddKeyRequestTypeSer<'b, 'a> {
    data: &'b AddKeyRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AddKeyRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AddKeyRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("key"), &self.data.key as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct AddKeysRequestType<'a> {
    keys: Option<&'a [crate::types::structs::CryptoKeyPlain]>,
}

impl<'a> miniserde::Serialize for AddKeysRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AddKeysRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AddKeysRequestTypeSer<'b, 'a> {
    data: &'b AddKeysRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AddKeysRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AddKeysRequestType")),
                1 => {
                    let Some(ref val) = self.data.keys else { continue; };
                    return Some((std::borrow::Cow::Borrowed("keys"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct GenerateClientCsrRequestType<'a> {
    cluster: &'a crate::types::structs::KeyProviderId,
    request: Option<&'a crate::types::structs::CryptoManagerKmipCertSignRequest>,
}

impl<'a> miniserde::Serialize for GenerateClientCsrRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(GenerateClientCsrRequestTypeSer { data: self, seq: 0 }))
    }
}

struct GenerateClientCsrRequestTypeSer<'b, 'a> {
    data: &'b GenerateClientCsrRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for GenerateClientCsrRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"GenerateClientCsrRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.request else { continue; };
                    return Some((std::borrow::Cow::Borrowed("request"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct GenerateKeyRequestType<'a> {
    key_provider: Option<&'a crate::types::structs::KeyProviderId>,
    spec: Option<&'a crate::types::structs::CryptoManagerKmipCustomAttributeSpec>,
    key_spec: Option<&'a crate::types::structs::CryptoManagerKmipGenerateKeySpec>,
}

impl<'a> miniserde::Serialize for GenerateKeyRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(GenerateKeyRequestTypeSer { data: self, seq: 0 }))
    }
}

struct GenerateKeyRequestTypeSer<'b, 'a> {
    data: &'b GenerateKeyRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for GenerateKeyRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"GenerateKeyRequestType")),
                1 => {
                    let Some(ref val) = self.data.key_provider else { continue; };
                    return Some((std::borrow::Cow::Borrowed("keyProvider"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("spec"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.key_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("keySpec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct GenerateSelfSignedClientCertRequestType<'a> {
    cluster: &'a crate::types::structs::KeyProviderId,
    request: Option<&'a crate::types::structs::CryptoManagerKmipCertSignRequest>,
}

impl<'a> miniserde::Serialize for GenerateSelfSignedClientCertRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(GenerateSelfSignedClientCertRequestTypeSer { data: self, seq: 0 }))
    }
}

struct GenerateSelfSignedClientCertRequestTypeSer<'b, 'a> {
    data: &'b GenerateSelfSignedClientCertRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for GenerateSelfSignedClientCertRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"GenerateSelfSignedClientCertRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.request else { continue; };
                    return Some((std::borrow::Cow::Borrowed("request"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct GetDefaultKmsClusterRequestType<'a> {
    entity: Option<&'a crate::types::structs::ManagedObjectReference>,
    defaults_to_parent: Option<bool>,
}

impl<'a> miniserde::Serialize for GetDefaultKmsClusterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(GetDefaultKmsClusterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct GetDefaultKmsClusterRequestTypeSer<'b, 'a> {
    data: &'b GetDefaultKmsClusterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for GetDefaultKmsClusterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"GetDefaultKmsClusterRequestType")),
                1 => {
                    let Some(ref val) = self.data.entity else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entity"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.defaults_to_parent else { continue; };
                    return Some((std::borrow::Cow::Borrowed("defaultsToParent"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ListKeysRequestType {
    limit: Option<i32>,
}

impl miniserde::Serialize for ListKeysRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ListKeysRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ListKeysRequestTypeSer<'b> {
    data: &'b ListKeysRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for ListKeysRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ListKeysRequestType")),
                1 => {
                    let Some(ref val) = self.data.limit else { continue; };
                    return Some((std::borrow::Cow::Borrowed("limit"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ListKmipServersRequestType {
    limit: Option<i32>,
}

impl miniserde::Serialize for ListKmipServersRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ListKmipServersRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ListKmipServersRequestTypeSer<'b> {
    data: &'b ListKmipServersRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for ListKmipServersRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ListKmipServersRequestType")),
                1 => {
                    let Some(ref val) = self.data.limit else { continue; };
                    return Some((std::borrow::Cow::Borrowed("limit"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ListKmsClustersRequestType {
    include_kms_servers: Option<bool>,
    management_type_filter: Option<i32>,
    status_filter: Option<i32>,
}

impl miniserde::Serialize for ListKmsClustersRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ListKmsClustersRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ListKmsClustersRequestTypeSer<'b> {
    data: &'b ListKmsClustersRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for ListKmsClustersRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ListKmsClustersRequestType")),
                1 => {
                    let Some(ref val) = self.data.include_kms_servers else { continue; };
                    return Some((std::borrow::Cow::Borrowed("includeKmsServers"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.management_type_filter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("managementTypeFilter"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.status_filter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("statusFilter"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct MarkDefaultRequestType<'a> {
    cluster_id: &'a crate::types::structs::KeyProviderId,
}

impl<'a> miniserde::Serialize for MarkDefaultRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MarkDefaultRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MarkDefaultRequestTypeSer<'b, 'a> {
    data: &'b MarkDefaultRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for MarkDefaultRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MarkDefaultRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("clusterId"), &self.data.cluster_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryCryptoKeyStatusRequestType<'a> {
    key_ids: Option<&'a [crate::types::structs::CryptoKeyId]>,
    check_key_bit_map: i32,
}

impl<'a> miniserde::Serialize for QueryCryptoKeyStatusRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryCryptoKeyStatusRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryCryptoKeyStatusRequestTypeSer<'b, 'a> {
    data: &'b QueryCryptoKeyStatusRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryCryptoKeyStatusRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryCryptoKeyStatusRequestType")),
                1 => {
                    let Some(ref val) = self.data.key_ids else { continue; };
                    return Some((std::borrow::Cow::Borrowed("keyIds"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("checkKeyBitMap"), &self.data.check_key_bit_map as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct RegisterKmipServerRequestType<'a> {
    server: &'a crate::types::structs::KmipServerSpec,
}

impl<'a> miniserde::Serialize for RegisterKmipServerRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RegisterKmipServerRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RegisterKmipServerRequestTypeSer<'b, 'a> {
    data: &'b RegisterKmipServerRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RegisterKmipServerRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RegisterKmipServerRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("server"), &self.data.server as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RegisterKmsClusterRequestType<'a> {
    cluster_id: &'a crate::types::structs::KeyProviderId,
    management_type: Option<&'a str>,
}

impl<'a> miniserde::Serialize for RegisterKmsClusterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RegisterKmsClusterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RegisterKmsClusterRequestTypeSer<'b, 'a> {
    data: &'b RegisterKmsClusterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RegisterKmsClusterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RegisterKmsClusterRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("clusterId"), &self.data.cluster_id as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.management_type else { continue; };
                    return Some((std::borrow::Cow::Borrowed("managementType"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RemoveKeyRequestType<'a> {
    key: &'a crate::types::structs::CryptoKeyId,
    force: bool,
}

impl<'a> miniserde::Serialize for RemoveKeyRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveKeyRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveKeyRequestTypeSer<'b, 'a> {
    data: &'b RemoveKeyRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveKeyRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveKeyRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("key"), &self.data.key as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("force"), &self.data.force as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RemoveKeysRequestType<'a> {
    keys: Option<&'a [crate::types::structs::CryptoKeyId]>,
    force: bool,
}

impl<'a> miniserde::Serialize for RemoveKeysRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveKeysRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveKeysRequestTypeSer<'b, 'a> {
    data: &'b RemoveKeysRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveKeysRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveKeysRequestType")),
                1 => {
                    let Some(ref val) = self.data.keys else { continue; };
                    return Some((std::borrow::Cow::Borrowed("keys"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("force"), &self.data.force as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct RemoveKmipServerRequestType<'a> {
    cluster_id: &'a crate::types::structs::KeyProviderId,
    server_name: &'a str,
}

impl<'a> miniserde::Serialize for RemoveKmipServerRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveKmipServerRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveKmipServerRequestTypeSer<'b, 'a> {
    data: &'b RemoveKmipServerRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveKmipServerRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveKmipServerRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("clusterId"), &self.data.cluster_id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("serverName"), &self.data.server_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RetrieveClientCertRequestType<'a> {
    cluster: &'a crate::types::structs::KeyProviderId,
}

impl<'a> miniserde::Serialize for RetrieveClientCertRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveClientCertRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveClientCertRequestTypeSer<'b, 'a> {
    data: &'b RetrieveClientCertRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RetrieveClientCertRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveClientCertRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RetrieveClientCsrRequestType<'a> {
    cluster: &'a crate::types::structs::KeyProviderId,
}

impl<'a> miniserde::Serialize for RetrieveClientCsrRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveClientCsrRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveClientCsrRequestTypeSer<'b, 'a> {
    data: &'b RetrieveClientCsrRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RetrieveClientCsrRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveClientCsrRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RetrieveKmipServerCertRequestType<'a> {
    key_provider: &'a crate::types::structs::KeyProviderId,
    server: &'a crate::types::structs::KmipServerInfo,
}

impl<'a> miniserde::Serialize for RetrieveKmipServerCertRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveKmipServerCertRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveKmipServerCertRequestTypeSer<'b, 'a> {
    data: &'b RetrieveKmipServerCertRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RetrieveKmipServerCertRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveKmipServerCertRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("keyProvider"), &self.data.key_provider as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("server"), &self.data.server as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RetrieveKmipServersStatusRequestType<'a> {
    clusters: Option<&'a [crate::types::structs::KmipClusterInfo]>,
}

impl<'a> miniserde::Serialize for RetrieveKmipServersStatusRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveKmipServersStatusRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveKmipServersStatusRequestTypeSer<'b, 'a> {
    data: &'b RetrieveKmipServersStatusRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RetrieveKmipServersStatusRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveKmipServersStatusRequestType")),
                1 => {
                    let Some(ref val) = self.data.clusters else { continue; };
                    return Some((std::borrow::Cow::Borrowed("clusters"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RetrieveSelfSignedClientCertRequestType<'a> {
    cluster: &'a crate::types::structs::KeyProviderId,
}

impl<'a> miniserde::Serialize for RetrieveSelfSignedClientCertRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RetrieveSelfSignedClientCertRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RetrieveSelfSignedClientCertRequestTypeSer<'b, 'a> {
    data: &'b RetrieveSelfSignedClientCertRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RetrieveSelfSignedClientCertRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RetrieveSelfSignedClientCertRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct SetDefaultKmsClusterRequestType<'a> {
    entity: Option<&'a crate::types::structs::ManagedObjectReference>,
    cluster_id: Option<&'a crate::types::structs::KeyProviderId>,
}

impl<'a> miniserde::Serialize for SetDefaultKmsClusterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetDefaultKmsClusterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetDefaultKmsClusterRequestTypeSer<'b, 'a> {
    data: &'b SetDefaultKmsClusterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for SetDefaultKmsClusterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SetDefaultKmsClusterRequestType")),
                1 => {
                    let Some(ref val) = self.data.entity else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entity"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.cluster_id else { continue; };
                    return Some((std::borrow::Cow::Borrowed("clusterId"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct SetKeyCustomAttributesRequestType<'a> {
    key_id: &'a crate::types::structs::CryptoKeyId,
    spec: &'a crate::types::structs::CryptoManagerKmipCustomAttributeSpec,
}

impl<'a> miniserde::Serialize for SetKeyCustomAttributesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetKeyCustomAttributesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetKeyCustomAttributesRequestTypeSer<'b, 'a> {
    data: &'b SetKeyCustomAttributesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for SetKeyCustomAttributesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SetKeyCustomAttributesRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("keyId"), &self.data.key_id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UnregisterKmsClusterRequestType<'a> {
    cluster_id: &'a crate::types::structs::KeyProviderId,
}

impl<'a> miniserde::Serialize for UnregisterKmsClusterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UnregisterKmsClusterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UnregisterKmsClusterRequestTypeSer<'b, 'a> {
    data: &'b UnregisterKmsClusterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UnregisterKmsClusterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UnregisterKmsClusterRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("clusterId"), &self.data.cluster_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateKmipServerRequestType<'a> {
    server: &'a crate::types::structs::KmipServerSpec,
}

impl<'a> miniserde::Serialize for UpdateKmipServerRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateKmipServerRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateKmipServerRequestTypeSer<'b, 'a> {
    data: &'b UpdateKmipServerRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateKmipServerRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateKmipServerRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("server"), &self.data.server as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateKmsSignedCsrClientCertRequestType<'a> {
    cluster: &'a crate::types::structs::KeyProviderId,
    certificate: &'a str,
}

impl<'a> miniserde::Serialize for UpdateKmsSignedCsrClientCertRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateKmsSignedCsrClientCertRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateKmsSignedCsrClientCertRequestTypeSer<'b, 'a> {
    data: &'b UpdateKmsSignedCsrClientCertRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateKmsSignedCsrClientCertRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateKmsSignedCsrClientCertRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("certificate"), &self.data.certificate as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateSelfSignedClientCertRequestType<'a> {
    cluster: &'a crate::types::structs::KeyProviderId,
    certificate: &'a str,
}

impl<'a> miniserde::Serialize for UpdateSelfSignedClientCertRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateSelfSignedClientCertRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateSelfSignedClientCertRequestTypeSer<'b, 'a> {
    data: &'b UpdateSelfSignedClientCertRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateSelfSignedClientCertRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateSelfSignedClientCertRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("certificate"), &self.data.certificate as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UploadClientCertRequestType<'a> {
    cluster: &'a crate::types::structs::KeyProviderId,
    certificate: &'a str,
    private_key: &'a str,
}

impl<'a> miniserde::Serialize for UploadClientCertRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UploadClientCertRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UploadClientCertRequestTypeSer<'b, 'a> {
    data: &'b UploadClientCertRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UploadClientCertRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UploadClientCertRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("certificate"), &self.data.certificate as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("privateKey"), &self.data.private_key as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UploadKmipServerCertRequestType<'a> {
    cluster: &'a crate::types::structs::KeyProviderId,
    certificate: &'a str,
}

impl<'a> miniserde::Serialize for UploadKmipServerCertRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UploadKmipServerCertRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UploadKmipServerCertRequestTypeSer<'b, 'a> {
    data: &'b UploadKmipServerCertRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UploadKmipServerCertRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UploadKmipServerCertRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("certificate"), &self.data.certificate as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
