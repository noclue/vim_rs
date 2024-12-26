use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::Extension;
use crate::types::structs::ExtensionManagerIpAllocationUsage;
use crate::types::structs::ManagedObjectReference;
/// This managed object type provides directory and basic management
/// services for all registered extensions.
/// 
/// Clients use the ExtensionManager, available in
/// *ServiceInstance*,
/// to access extension objects.
/// 
/// While several authentication methods are available for extension
/// servers to use (see *SessionManager*), only one
/// authentication method is valid for an extension at any given
/// time.
pub struct ExtensionManager {
    client: Arc<Client>,
    mo_id: String,
}
impl ExtensionManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Returns extension with the given key, if any.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### extension_key
    /// Key to search for.
    ///
    /// ## Returns:
    ///
    /// Extension that matches given key, if any.
    pub async fn find_extension(&self, extension_key: &str) -> Result<Extension> {
        let input = FindExtensionRequestType {extension_key, };
        let path = format!("/ExtensionManager/{moId}/FindExtension", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of VI 4.0, use trusted certificates and
    /// *SessionManager.LoginExtensionBySubjectName* or
    /// *ExtensionManager.SetExtensionCertificate* and
    /// *SessionManager.LoginExtensionByCertificate*.
    /// 
    /// Returns VirtualCenter Server public key.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Public key of VirtualCenter Server, encoded
    /// in PEM (privacy-enhanced mail) format.
    pub async fn get_public_key(&self) -> Result<String> {
        let path = format!("/ExtensionManager/{moId}/GetPublicKey", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Query statistics about IP allocation usage, either system wide or for
    /// specified extensions.
    /// 
    /// Refer to *IpPoolManager* for details.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### extension_keys
    /// List of extensions whose IP allocation is being queried.
    /// If no extension keys are specified then allocation data
    /// for all registered extensions are returned.
    ///
    /// ## Returns:
    ///
    /// List of IP allocation usage.
    pub async fn query_extension_ip_allocation_usage(&self, extension_keys: Option<&[String]>) -> Result<Option<Vec<ExtensionManagerIpAllocationUsage>>> {
        let input = QueryExtensionIpAllocationUsageRequestType {extension_keys, };
        let path = format!("/ExtensionManager/{moId}/QueryExtensionIpAllocationUsage", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Find entities managed by an extension.
    /// 
    /// These can be either virtual machines
    /// or vApps.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### extension_key
    /// Key of the extension to find managed entities for.
    ///
    /// ## Returns:
    ///
    /// List of entities managed by the extension.
    /// 
    /// Refers instances of *ManagedEntity*.
    pub async fn query_managed_by(&self, extension_key: &str) -> Result<Option<Vec<ManagedObjectReference>>> {
        let input = QueryManagedByRequestType {extension_key, };
        let path = format!("/ExtensionManager/{moId}/QueryManagedBy", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Registers extension.
    /// 
    /// ***Required privileges:*** Extension.Register
    ///
    /// ## Parameters:
    ///
    /// ### extension
    /// Extension description to register.
    pub async fn register_extension(&self, extension: &Extension) -> Result<()> {
        let input = RegisterExtensionRequestType {extension, };
        let path = format!("/ExtensionManager/{moId}/RegisterExtension", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Update the stored authentication certificate for a specified extension.
    /// 
    /// Updates the registration of the specified extension with the
    /// thumbprint of the X.509 client certificate provided over SSL handshake,
    /// or by the &quot;certificatePem&quot;argument. The thumbprint
    /// will be used to authenticate the extension during invocations of
    /// *SessionManager.LoginExtensionByCertificate*.
    /// 
    /// NOTE: No verification is performed on the received certificate, such as
    /// expiry or revocation.
    /// 
    /// This method will unset any public key or subject name
    /// associated with the extension.
    /// 
    /// ***Required privileges:*** Extension.Update
    ///
    /// ## Parameters:
    ///
    /// ### extension_key
    /// Key of extension to update.
    ///
    /// ### certificate_pem
    /// PEM encoded certificate. If not specified, the
    /// certificate passed over SSL handshake is used.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the certificate described by
    /// &quot;certificatePem&quot; is not in PEM
    /// format, or could not be decoded to an X.509 certificate.
    /// 
    /// ***NoClientCertificate***: if certificatePem is not specified, and
    /// no certificate was passed over SSL handshake.
    /// 
    /// ***NotFound***: if an extension specified by &quot;extensionKey&quot;
    /// is not registered.
    pub async fn set_extension_certificate(&self, extension_key: &str, certificate_pem: Option<&str>) -> Result<()> {
        let input = SetExtensionCertificateRequestType {extension_key, certificate_pem, };
        let path = format!("/ExtensionManager/{moId}/SetExtensionCertificate", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Deprecated as of VI 4.0, use trusted certificates and
    /// *SessionManager.LoginExtensionBySubjectName* or
    /// *ExtensionManager.SetExtensionCertificate* and
    /// *SessionManager.LoginExtensionByCertificate*.
    /// 
    /// Sets extension's public key.
    /// 
    /// This method will unset any subject name or
    /// certificate associated with the extension.
    /// 
    /// ***Required privileges:*** Extension.Update
    ///
    /// ## Parameters:
    ///
    /// ### extension_key
    /// Key of extension to update.
    ///
    /// ### public_key
    /// Public key of extension, encoded
    /// in PEM (privacy-enhanced mail) format.
    pub async fn set_public_key(&self, extension_key: &str, public_key: &str) -> Result<()> {
        let input = SetPublicKeyRequestType {extension_key, public_key, };
        let path = format!("/ExtensionManager/{moId}/SetPublicKey", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Update the stored authentication service account for the specified extension.
    /// 
    /// Updates the registration of the specified extension with the
    /// service account passed in the &quot;serviceAccount&quot;parameter.
    /// The service account will be used to authenticate the extension
    /// by invoking vCenter Server Login APIs and the user session can be
    /// used to update an Extension's data.
    /// 
    /// This method will unset the previous service account name
    /// associated with the extension.
    /// 
    /// The account name must be passed in with same case as the created
    /// account name.
    /// The account name must be suffixed with the vCenter machine id.
    /// The account name passed in must be qualified by the SSO domain
    /// for the vCenter server using the same format as userName for
    /// *SessionManager.Login*
    /// 
    /// NOTE: Account lifetime is managed by the extension owning the account.
    /// 
    /// ***Since:*** vSphere API Release 8.0.2.0
    /// 
    /// ***Required privileges:*** Extension.Update
    ///
    /// ## Parameters:
    ///
    /// ### extension_key
    /// Key of extension to update.
    ///
    /// ### service_account
    /// account name qualified with SSO domain.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the account name described by
    /// &quot;serviceAccount&quot; is not present.
    /// 
    /// ***NotFound***: if an extension specified by &quot;extensionKey&quot;
    /// is not registered or the service account is not found.
    pub async fn set_service_account(&self, extension_key: &str, service_account: &str) -> Result<()> {
        let input = SetServiceAccountRequestType {extension_key, service_account, };
        let path = format!("/ExtensionManager/{moId}/SetServiceAccount", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Unregisters the specified extension if it exists.
    /// 
    /// ***Required privileges:*** Extension.Unregister
    ///
    /// ## Parameters:
    ///
    /// ### extension_key
    /// Unique name of extension to unregister.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the specified extension
    /// is not registered.
    pub async fn unregister_extension(&self, extension_key: &str) -> Result<()> {
        let input = UnregisterExtensionRequestType {extension_key, };
        let path = format!("/ExtensionManager/{moId}/UnregisterExtension", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// If the key specified in the extension exists,
    /// the existing record is updated.
    /// 
    /// If the &quot;subjectName&quot; property of the Extension
    /// object has a value, and it is different from the existing
    /// value, this method will unset any public key or
    /// certificate associated with the extension.
    /// 
    /// ***Required privileges:*** Extension.Update
    ///
    /// ## Parameters:
    ///
    /// ### extension
    /// Updated extension description.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the specified extension key is not registered.
    /// 
    /// ***InvalidArgument***: if the Extension description is incomplete or invalid, or
    /// if the extension is an OVF extension and its section types overlap with other
    /// registered OVF extensions.
    pub async fn update_extension(&self, extension: &Extension) -> Result<()> {
        let input = UpdateExtensionRequestType {extension, };
        let path = format!("/ExtensionManager/{moId}/UpdateExtension", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// The list of currently registered extensions.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn extension_list(&self) -> Result<Option<Vec<Extension>>> {
        let path = format!("/ExtensionManager/{moId}/extensionList", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct FindExtensionRequestType<'a> {
    #[serde(rename = "extensionKey")]
    extension_key: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryExtensionIpAllocationUsageRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "extensionKeys")]
    extension_keys: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryManagedByRequestType<'a> {
    #[serde(rename = "extensionKey")]
    extension_key: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RegisterExtensionRequestType<'a> {
    extension: &'a Extension,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetExtensionCertificateRequestType<'a> {
    #[serde(rename = "extensionKey")]
    extension_key: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "certificatePem")]
    certificate_pem: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetPublicKeyRequestType<'a> {
    #[serde(rename = "extensionKey")]
    extension_key: &'a str,
    #[serde(rename = "publicKey")]
    public_key: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetServiceAccountRequestType<'a> {
    #[serde(rename = "extensionKey")]
    extension_key: &'a str,
    #[serde(rename = "serviceAccount")]
    service_account: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UnregisterExtensionRequestType<'a> {
    #[serde(rename = "extensionKey")]
    extension_key: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateExtensionRequestType<'a> {
    extension: &'a Extension,
}
