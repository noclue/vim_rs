use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::CryptoKeyId;
use crate::types::CryptoKeyPlain;
use crate::types::CryptoKeyResult;
use crate::types::CryptoManagerKmipCertSignRequest;
use crate::types::CryptoManagerKmipCryptoKeyStatus;
use crate::types::CryptoManagerKmipCustomAttributeSpec;
use crate::types::CryptoManagerKmipServerCertInfo;
use crate::types::KeyProviderId;
use crate::types::KmipClusterInfo;
use crate::types::KmipServerInfo;
use crate::types::KmipServerSpec;
use crate::types::ManagedObjectReference;
/// Singleton Managed Object used to manage cryptographic keys.
pub struct CryptoManagerKmip {
    client: Arc<VimClient>,
    mo_id: String,
}
impl CryptoManagerKmip {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
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
    pub async fn is_kms_cluster_active(&self, cluster: Option<&KeyProviderId>) -> Result<bool> {
        let input = IsKmsClusterActiveRequestType {cluster, };
        let path = format!("/CryptoManagerKmip/{moId}/IsKmsClusterActive", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn add_key(&self, key: &CryptoKeyPlain) -> Result<()> {
        let input = AddKeyRequestType {key, };
        let path = format!("/CryptoManagerKmip/{moId}/AddKey", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn add_keys(&self, keys: Option<&[CryptoKeyPlain]>) -> Result<Option<Vec<CryptoKeyResult>>> {
        let input = AddKeysRequestType {keys, };
        let path = format!("/CryptoManagerKmip/{moId}/AddKeys", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn generate_client_csr(&self, cluster: &KeyProviderId, request: Option<&CryptoManagerKmipCertSignRequest>) -> Result<String> {
        let input = GenerateClientCsrRequestType {cluster, request, };
        let path = format!("/CryptoManagerKmip/{moId}/GenerateClientCsr", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    /// ## Returns:
    ///
    /// the generated key.
    pub async fn generate_key(&self, key_provider: Option<&KeyProviderId>, spec: Option<&CryptoManagerKmipCustomAttributeSpec>) -> Result<CryptoKeyResult> {
        let input = GenerateKeyRequestType {key_provider, spec, };
        let path = format!("/CryptoManagerKmip/{moId}/GenerateKey", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn generate_self_signed_client_cert(&self, cluster: &KeyProviderId, request: Option<&CryptoManagerKmipCertSignRequest>) -> Result<String> {
        let input = GenerateSelfSignedClientCertRequestType {cluster, request, };
        let path = format!("/CryptoManagerKmip/{moId}/GenerateSelfSignedClientCert", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn get_default_kms_cluster(&self, entity: Option<&ManagedObjectReference>, defaults_to_parent: Option<bool>) -> Result<KeyProviderId> {
        let input = GetDefaultKmsClusterRequestType {entity, defaults_to_parent, };
        let path = format!("/CryptoManagerKmip/{moId}/GetDefaultKmsCluster", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn list_keys(&self, limit: Option<i32>) -> Result<Option<Vec<CryptoKeyId>>> {
        let input = ListKeysRequestType {limit, };
        let path = format!("/CryptoManagerKmip/{moId}/ListKeys", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn list_kmip_servers(&self, limit: Option<i32>) -> Result<Option<Vec<KmipClusterInfo>>> {
        let input = ListKmipServersRequestType {limit, };
        let path = format!("/CryptoManagerKmip/{moId}/ListKmipServers", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn list_kms_clusters(&self, include_kms_servers: Option<bool>, management_type_filter: Option<i32>, status_filter: Option<i32>) -> Result<Option<Vec<KmipClusterInfo>>> {
        let input = ListKmsClustersRequestType {include_kms_servers, management_type_filter, status_filter, };
        let path = format!("/CryptoManagerKmip/{moId}/ListKmsClusters", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Set the default KMIP cluster.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### cluster_id
    /// \[in\] KMIP cluster ID to become default.
    pub async fn mark_default(&self, cluster_id: &KeyProviderId) -> Result<()> {
        let input = MarkDefaultRequestType {cluster_id, };
        let path = format!("/CryptoManagerKmip/{moId}/MarkDefault", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn query_crypto_key_status(&self, key_ids: Option<&[CryptoKeyId]>, check_key_bit_map: i32) -> Result<Option<Vec<CryptoManagerKmipCryptoKeyStatus>>> {
        let input = QueryCryptoKeyStatusRequestType {key_ids, check_key_bit_map, };
        let path = format!("/CryptoManagerKmip/{moId}/QueryCryptoKeyStatus", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Register a KMIP server.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### server
    /// \[in\] KMIP server connection information.
    pub async fn register_kmip_server(&self, server: &KmipServerSpec) -> Result<()> {
        let input = RegisterKmipServerRequestType {server, };
        let path = format!("/CryptoManagerKmip/{moId}/RegisterKmipServer", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn register_kms_cluster(&self, cluster_id: &KeyProviderId, management_type: Option<&str>) -> Result<()> {
        let input = RegisterKmsClusterRequestType {cluster_id, management_type, };
        let path = format!("/CryptoManagerKmip/{moId}/RegisterKmsCluster", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn remove_key(&self, key: &CryptoKeyId, force: bool) -> Result<()> {
        let input = RemoveKeyRequestType {key, force, };
        let path = format!("/CryptoManagerKmip/{moId}/RemoveKey", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn remove_keys(&self, keys: Option<&[CryptoKeyId]>, force: bool) -> Result<Option<Vec<CryptoKeyResult>>> {
        let input = RemoveKeysRequestType {keys, force, };
        let path = format!("/CryptoManagerKmip/{moId}/RemoveKeys", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn remove_kmip_server(&self, cluster_id: &KeyProviderId, server_name: &str) -> Result<()> {
        let input = RemoveKmipServerRequestType {cluster_id, server_name, };
        let path = format!("/CryptoManagerKmip/{moId}/RemoveKmipServer", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn retrieve_client_cert(&self, cluster: &KeyProviderId) -> Result<String> {
        let input = RetrieveClientCertRequestType {cluster, };
        let path = format!("/CryptoManagerKmip/{moId}/RetrieveClientCert", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn retrieve_client_csr(&self, cluster: &KeyProviderId) -> Result<String> {
        let input = RetrieveClientCsrRequestType {cluster, };
        let path = format!("/CryptoManagerKmip/{moId}/RetrieveClientCsr", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn retrieve_kmip_server_cert(&self, key_provider: &KeyProviderId, server: &KmipServerInfo) -> Result<CryptoManagerKmipServerCertInfo> {
        let input = RetrieveKmipServerCertRequestType {key_provider, server, };
        let path = format!("/CryptoManagerKmip/{moId}/RetrieveKmipServerCert", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn retrieve_kmip_servers_status_task(&self, clusters: Option<&[KmipClusterInfo]>) -> Result<ManagedObjectReference> {
        let input = RetrieveKmipServersStatusRequestType {clusters, };
        let path = format!("/CryptoManagerKmip/{moId}/RetrieveKmipServersStatus_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn retrieve_self_signed_client_cert(&self, cluster: &KeyProviderId) -> Result<String> {
        let input = RetrieveSelfSignedClientCertRequestType {cluster, };
        let path = format!("/CryptoManagerKmip/{moId}/RetrieveSelfSignedClientCert", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn set_default_kms_cluster(&self, entity: Option<&ManagedObjectReference>, cluster_id: Option<&KeyProviderId>) -> Result<()> {
        let input = SetDefaultKmsClusterRequestType {entity, cluster_id, };
        let path = format!("/CryptoManagerKmip/{moId}/SetDefaultKmsCluster", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn set_key_custom_attributes(&self, key_id: &CryptoKeyId, spec: &CryptoManagerKmipCustomAttributeSpec) -> Result<CryptoKeyResult> {
        let input = SetKeyCustomAttributesRequestType {key_id, spec, };
        let path = format!("/CryptoManagerKmip/{moId}/SetKeyCustomAttributes", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Unregister the specified KMS cluster from the CryptoManager.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### cluster_id
    /// \[in\] KMS cluster ID to unregister.
    pub async fn unregister_kms_cluster(&self, cluster_id: &KeyProviderId) -> Result<()> {
        let input = UnregisterKmsClusterRequestType {cluster_id, };
        let path = format!("/CryptoManagerKmip/{moId}/UnregisterKmsCluster", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Update a KMIP server.
    /// 
    /// ***Required privileges:*** Cryptographer.ManageKeyServers
    ///
    /// ## Parameters:
    ///
    /// ### server
    /// \[in\] KMIP server connection information.
    pub async fn update_kmip_server(&self, server: &KmipServerSpec) -> Result<()> {
        let input = UpdateKmipServerRequestType {server, };
        let path = format!("/CryptoManagerKmip/{moId}/UpdateKmipServer", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn update_kms_signed_csr_client_cert(&self, cluster: &KeyProviderId, certificate: &str) -> Result<()> {
        let input = UpdateKmsSignedCsrClientCertRequestType {cluster, certificate, };
        let path = format!("/CryptoManagerKmip/{moId}/UpdateKmsSignedCsrClientCert", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn update_self_signed_client_cert(&self, cluster: &KeyProviderId, certificate: &str) -> Result<()> {
        let input = UpdateSelfSignedClientCertRequestType {cluster, certificate, };
        let path = format!("/CryptoManagerKmip/{moId}/UpdateSelfSignedClientCert", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn upload_client_cert(&self, cluster: &KeyProviderId, certificate: &str, private_key: &str) -> Result<()> {
        let input = UploadClientCertRequestType {cluster, certificate, private_key, };
        let path = format!("/CryptoManagerKmip/{moId}/UploadClientCert", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn upload_kmip_server_cert(&self, cluster: &KeyProviderId, certificate: &str) -> Result<()> {
        let input = UploadKmipServerCertRequestType {cluster, certificate, };
        let path = format!("/CryptoManagerKmip/{moId}/UploadKmipServerCert", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Indicate if the encryption feature is enabled.
    pub async fn enabled(&self) -> Result<bool> {
        let path = format!("/CryptoManagerKmip/{moId}/enabled", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// A list of registered KMIP servers, grouped by clusters.
    pub async fn kmip_servers(&self) -> Result<Option<Vec<KmipClusterInfo>>> {
        let path = format!("/CryptoManagerKmip/{moId}/kmipServers", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct IsKmsClusterActiveRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a KeyProviderId>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddKeyRequestType<'a> {
    key: &'a CryptoKeyPlain,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddKeysRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    keys: Option<&'a [CryptoKeyPlain]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct GenerateClientCsrRequestType<'a> {
    cluster: &'a KeyProviderId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    request: Option<&'a CryptoManagerKmipCertSignRequest>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct GenerateKeyRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "keyProvider")]
    key_provider: Option<&'a KeyProviderId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    spec: Option<&'a CryptoManagerKmipCustomAttributeSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct GenerateSelfSignedClientCertRequestType<'a> {
    cluster: &'a KeyProviderId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    request: Option<&'a CryptoManagerKmipCertSignRequest>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct GetDefaultKmsClusterRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entity: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "defaultsToParent")]
    defaults_to_parent: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ListKeysRequestType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ListKmipServersRequestType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ListKmsClustersRequestType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "includeKmsServers")]
    include_kms_servers: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "managementTypeFilter")]
    management_type_filter: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "statusFilter")]
    status_filter: Option<i32>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct MarkDefaultRequestType<'a> {
    #[serde(rename = "clusterId")]
    cluster_id: &'a KeyProviderId,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryCryptoKeyStatusRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "keyIds")]
    key_ids: Option<&'a [CryptoKeyId]>,
    #[serde(rename = "checkKeyBitMap")]
    check_key_bit_map: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RegisterKmipServerRequestType<'a> {
    server: &'a KmipServerSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RegisterKmsClusterRequestType<'a> {
    #[serde(rename = "clusterId")]
    cluster_id: &'a KeyProviderId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "managementType")]
    management_type: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveKeyRequestType<'a> {
    key: &'a CryptoKeyId,
    force: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveKeysRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    keys: Option<&'a [CryptoKeyId]>,
    force: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveKmipServerRequestType<'a> {
    #[serde(rename = "clusterId")]
    cluster_id: &'a KeyProviderId,
    #[serde(rename = "serverName")]
    server_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveClientCertRequestType<'a> {
    cluster: &'a KeyProviderId,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveClientCsrRequestType<'a> {
    cluster: &'a KeyProviderId,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveKmipServerCertRequestType<'a> {
    #[serde(rename = "keyProvider")]
    key_provider: &'a KeyProviderId,
    server: &'a KmipServerInfo,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveKmipServersStatusRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    clusters: Option<&'a [KmipClusterInfo]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveSelfSignedClientCertRequestType<'a> {
    cluster: &'a KeyProviderId,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetDefaultKmsClusterRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entity: Option<&'a ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clusterId")]
    cluster_id: Option<&'a KeyProviderId>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetKeyCustomAttributesRequestType<'a> {
    #[serde(rename = "keyId")]
    key_id: &'a CryptoKeyId,
    spec: &'a CryptoManagerKmipCustomAttributeSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UnregisterKmsClusterRequestType<'a> {
    #[serde(rename = "clusterId")]
    cluster_id: &'a KeyProviderId,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateKmipServerRequestType<'a> {
    server: &'a KmipServerSpec,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateKmsSignedCsrClientCertRequestType<'a> {
    cluster: &'a KeyProviderId,
    certificate: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateSelfSignedClientCertRequestType<'a> {
    cluster: &'a KeyProviderId,
    certificate: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UploadClientCertRequestType<'a> {
    cluster: &'a KeyProviderId,
    certificate: &'a str,
    #[serde(rename = "privateKey")]
    private_key: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UploadKmipServerCertRequestType<'a> {
    cluster: &'a KeyProviderId,
    certificate: &'a str,
}
