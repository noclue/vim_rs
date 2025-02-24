use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::ManagedObjectReference;
/// The *HostActiveDirectoryAuthentication* managed object
/// indicates domain membership status and provides methods
/// for adding a host to and removing a host from a domain.
pub struct HostActiveDirectoryAuthentication {
    client: Arc<Client>,
    mo_id: String,
}
impl HostActiveDirectoryAuthentication {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Disables console authentication using a local smart card and reader.
    /// 
    /// ***Required privileges:*** Host.Config.AuthenticationStore
    ///
    /// ## Errors:
    ///
    /// ***ActiveDirectoryFault***: if the active directory client could not
    /// be reconfigured.
    /// 
    /// ***HostConfigFault***: if the host configuration prevents smart card
    /// authentication from being disabled.
    pub async fn disable_smart_card_authentication(&self) -> Result<()> {
        let path = format!("/HostActiveDirectoryAuthentication/{moId}/DisableSmartCardAuthentication", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Enables console authentication using a local smart card and reader.
    /// 
    /// To take effect this feature requires an active domain membership to a
    /// domain with users configured to authenticate using smart cards.
    /// 
    /// ***Required privileges:*** Host.Config.AuthenticationStore
    ///
    /// ## Errors:
    ///
    /// ***ActiveDirectoryFault***: if the active directory client could not
    /// be reconfigured.
    /// 
    /// ***HostConfigFault***: if the host configuration prevents smart card
    /// authentication from being enabled.
    pub async fn enable_smart_card_authentication(&self) -> Result<()> {
        let path = format!("/HostActiveDirectoryAuthentication/{moId}/EnableSmartCardAuthentication", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Import the CAM server's certificate to the local store of vmwauth.
    /// 
    /// The certificate should have already been uploaded to ESXi file system.
    /// 
    /// ***Required privileges:*** Host.Config.AuthenticationStore
    ///
    /// ## Parameters:
    ///
    /// ### cert_path
    /// full path of the certificate on ESXi
    ///
    /// ### cam_server
    /// IP of server providing the CAM service.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***FileNotFound***: if the certificate file does not exist
    /// 
    /// ***InvalidCAMServer***: if camServer is not a valid IP address
    /// 
    /// ***ActiveDirectoryFault***: for any problem that is not handled with a more specific fault.
    pub async fn import_certificate_for_cam_task(&self, cert_path: &str, cam_server: &str) -> Result<ManagedObjectReference> {
        let input = ImportCertificateForCamRequestType {cert_path, cam_server, };
        let path = format!("/HostActiveDirectoryAuthentication/{moId}/ImportCertificateForCAM_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Install a trust anchor certificate for smart card authentication.
    /// 
    /// ***Required privileges:*** Host.Config.AuthenticationStore
    ///
    /// ## Parameters:
    ///
    /// ### cert
    /// SSL certificate in PEM format
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if the host configuration prevents the
    /// certificate from being installed.
    pub async fn install_smart_card_trust_anchor(&self, cert: &str) -> Result<()> {
        let input = InstallSmartCardTrustAnchorRequestType {cert, };
        let path = format!("/HostActiveDirectoryAuthentication/{moId}/InstallSmartCardTrustAnchor", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Adds the host to an Active Directory domain.
    /// 
    /// If the *HostAuthenticationStoreInfo*.*HostAuthenticationStoreInfo.enabled*
    /// property is <code>True</code> (accessed through the <code>info</code> property),
    /// the host has joined a domain.
    /// The vSphere API will throw the <code>InvalidState</code> fault if you try
    /// to add a host to a domain when the host has already joined a domain.
    /// 
    /// ***Required privileges:*** Host.Config.AuthenticationStore
    ///
    /// ## Parameters:
    ///
    /// ### domain_name
    /// Name of the domain to be joined.
    ///
    /// ### user_name
    /// Name for an Active Directory account
    /// that has the authority to add hosts to the domain.
    ///
    /// ### password
    /// Password for the <code>userName</code> account.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the host has already joined a domain.
    /// 
    /// ***BlockedByFirewall***: if ports needed by the join operation are
    /// blocked by the firewall.
    /// 
    /// ***HostConfigFault***: if the host configuration prevents the join operation
    /// from succeeding.
    /// 
    /// ***InvalidLogin***: if <code>userName</code> and <code>password</code>
    /// are not valid user credentials.
    /// 
    /// ***DomainNotFound***: if the domain controller for <code>domainName</code>
    /// cannot be reached.
    /// 
    /// ***NoPermissionOnAD***: if <code>userName</code> has no right to add hosts to the domain.
    /// 
    /// ***InvalidHostName***: if the domain part of the host's FQDN doesn't match
    /// the domain being joined.
    /// 
    /// ***ClockSkew***: if the clocks of the host and the domain controller
    /// differ by more than the allowed amount of time.
    /// 
    /// ***ActiveDirectoryFault***: for any problem that is not handled with a more specific fault.
    /// 
    /// ***TaskInProgress***: if the *HostActiveDirectoryAuthentication* object is busy.
    pub async fn join_domain_task(&self, domain_name: &str, user_name: &str, password: &str) -> Result<ManagedObjectReference> {
        let input = JoinDomainRequestType {domain_name, user_name, password, };
        let path = format!("/HostActiveDirectoryAuthentication/{moId}/JoinDomain_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Adds the host to an Active Directory domain through CAM service.
    /// 
    /// If the *HostAuthenticationStoreInfo*.*HostAuthenticationStoreInfo.enabled*
    /// property is <code>True</code> (accessed through the <code>info</code> property),
    /// the host has joined a domain.
    /// The vSphere API will throw the <code>InvalidState</code> fault if you try
    /// to add a host to a domain when the host has already joined a domain.
    /// 
    /// ***Required privileges:*** Host.Config.AuthenticationStore
    ///
    /// ## Parameters:
    ///
    /// ### domain_name
    /// Name of the domain to be joined.
    ///
    /// ### cam_server
    /// Name of server providing the CAM service.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the host has already joined a domain.
    /// 
    /// ***BlockedByFirewall***: if ports needed by the join operation are
    /// blocked by the firewall.
    /// 
    /// ***HostConfigFault***: if the host configuration prevents the join operation
    /// from succeeding.
    /// 
    /// ***DomainNotFound***: if the domain controller for <code>domainName</code>
    /// cannot be reached.
    /// 
    /// ***InvalidHostName***: if the domain part of the host's FQDN doesn't match
    /// the domain being joined.
    /// 
    /// ***ClockSkew***: if the clocks of the host and the domain controller
    /// differ by more than the allowed amount of time.
    /// 
    /// ***InvalidCAMServer***: if camServer is not a valid IP address, or
    /// if camServer is not accessible.
    /// 
    /// ***InvalidCAMCertificate***: if the certificate of the given CAM server
    /// cannot be verified.
    /// 
    /// ***CAMServerRefusedConnection***: if the specified CAM server is not
    /// reachable, or
    /// if the server denied access.
    /// 
    /// ***ActiveDirectoryFault***: for any problem that is not handled with a more specific fault.
    /// 
    /// ***TaskInProgress***: if the *HostActiveDirectoryAuthentication* object is busy.
    pub async fn join_domain_with_cam_task(&self, domain_name: &str, cam_server: &str) -> Result<ManagedObjectReference> {
        let input = JoinDomainWithCamRequestType {domain_name, cam_server, };
        let path = format!("/HostActiveDirectoryAuthentication/{moId}/JoinDomainWithCAM_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Removes the host from the Active Directory domain to which it belongs.
    /// 
    /// ***Required privileges:*** Host.Config.AuthenticationStore
    ///
    /// ## Parameters:
    ///
    /// ### force
    /// If <code>True</code>, any existing permissions on managed entities for
    /// Active Directory users will be deleted. If <code>False</code> and such
    /// permissions exist, the operation will fail.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: if the host is not in a domain or there are active
    /// permissions for Active Directory users.
    /// 
    /// ***NonADUserRequired***: only non Active Directory users can initiate
    /// the leave domain operation.
    /// 
    /// ***AuthMinimumAdminPermission***: if this change would leave the system with
    /// no Administrator permission on the root node.
    /// 
    /// ***ActiveDirectoryFault***: for any problem that is not handled with a specific fault.
    /// 
    /// ***TaskInProgress***: if the ActiveDirectoryAuthentication object is busy.
    pub async fn leave_current_domain_task(&self, force: bool) -> Result<ManagedObjectReference> {
        let input = LeaveCurrentDomainRequestType {force, };
        let path = format!("/HostActiveDirectoryAuthentication/{moId}/LeaveCurrentDomain_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Lists installed trust anchor certificates for smart card authentication.
    /// 
    /// ***Required privileges:*** Host.Config.AuthenticationStore
    ///
    /// ## Returns:
    ///
    /// SSL certificates of trusted CAs in PEM format.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if the host configuration prevents the
    /// certificates from being listed.
    pub async fn list_smart_card_trust_anchors(&self) -> Result<Option<Vec<String>>> {
        let path = format!("/HostActiveDirectoryAuthentication/{moId}/ListSmartCardTrustAnchors", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Deprecated please remove by fingerprint/digest instead.
    /// 
    /// Remove a smart card trust anchor certificate from the system.
    /// 
    /// ***Required privileges:*** Host.Config.AuthenticationStore
    ///
    /// ## Parameters:
    ///
    /// ### issuer
    /// Certificate issuer
    ///
    /// ### serial
    /// Certificate serial number (decimal integer)
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if the host configuration prevents the
    /// certificate from being removed.
    pub async fn remove_smart_card_trust_anchor(&self, issuer: &str, serial: &str) -> Result<()> {
        let input = RemoveSmartCardTrustAnchorRequestType {issuer, serial, };
        let path = format!("/HostActiveDirectoryAuthentication/{moId}/RemoveSmartCardTrustAnchor", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Remove a smart card trust anchor certificate from the system by
    /// fingerprint.
    /// 
    /// ***Required privileges:*** Host.Config.AuthenticationStore
    ///
    /// ## Parameters:
    ///
    /// ### fingerprint
    /// Certificate fingerprint
    ///
    /// ### digest
    /// Digest function used to compute fingerprint. One of
    /// *HostActiveDirectoryAuthenticationCertificateDigest_enum*.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if the host configuration prevents the
    /// certificate from being removed.
    pub async fn remove_smart_card_trust_anchor_by_fingerprint(&self, fingerprint: &str, digest: &str) -> Result<()> {
        let input = RemoveSmartCardTrustAnchorByFingerprintRequestType {fingerprint, digest, };
        let path = format!("/HostActiveDirectoryAuthentication/{moId}/RemoveSmartCardTrustAnchorByFingerprint", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Replace the trust anchor certificates for smart card authentication.
    /// 
    /// ***Required privileges:*** Host.Config.AuthenticationStore
    ///
    /// ## Parameters:
    ///
    /// ### certs
    /// List of trusted CA certificates in PEM format. If empty
    /// then all existing trust anchors are removed.
    pub async fn replace_smart_card_trust_anchors(&self, certs: Option<&[String]>) -> Result<()> {
        let input = ReplaceSmartCardTrustAnchorsRequestType {certs, };
        let path = format!("/HostActiveDirectoryAuthentication/{moId}/ReplaceSmartCardTrustAnchors", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Information about the authentication store.
    pub async fn info(&self) -> Result<Box<dyn crate::types::host_authentication_store_info_trait::HostAuthenticationStoreInfoTrait>> {
        let path = format!("/HostActiveDirectoryAuthentication/{moId}/info", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(rename = "ImportCertificateForCAMRequestType", tag = "_typeName")]
struct ImportCertificateForCamRequestType<'a> {
    #[serde(rename = "certPath")]
    cert_path: &'a str,
    #[serde(rename = "camServer")]
    cam_server: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct InstallSmartCardTrustAnchorRequestType<'a> {
    cert: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct JoinDomainRequestType<'a> {
    #[serde(rename = "domainName")]
    domain_name: &'a str,
    #[serde(rename = "userName")]
    user_name: &'a str,
    password: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "JoinDomainWithCAMRequestType", tag = "_typeName")]
struct JoinDomainWithCamRequestType<'a> {
    #[serde(rename = "domainName")]
    domain_name: &'a str,
    #[serde(rename = "camServer")]
    cam_server: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct LeaveCurrentDomainRequestType {
    force: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveSmartCardTrustAnchorRequestType<'a> {
    issuer: &'a str,
    serial: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveSmartCardTrustAnchorByFingerprintRequestType<'a> {
    fingerprint: &'a str,
    digest: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReplaceSmartCardTrustAnchorsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    certs: Option<&'a [String]>,
}
