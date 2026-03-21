use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The *HostActiveDirectoryAuthentication* managed object
/// indicates domain membership status and provides methods
/// for adding a host to and removing a host from a domain.
#[derive(Clone)]
pub struct HostActiveDirectoryAuthentication {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostActiveDirectoryAuthentication {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Deprecated as of vSphere API 8.0U3, and there is no replacement for it.
    /// 
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
        self.client.invoke_void("", "HostActiveDirectoryAuthentication", &self.mo_id, "DisableSmartCardAuthentication", None).await
    }
    /// Deprecated as of vSphere API 8.0U3, and there is no replacement for it.
    /// 
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
        self.client.invoke_void("", "HostActiveDirectoryAuthentication", &self.mo_id, "EnableSmartCardAuthentication", None).await
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
    pub async fn import_certificate_for_cam_task(&self, cert_path: &str, cam_server: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ImportCertificateForCamRequestType {cert_path, cam_server, };
        let bytes = self.client.invoke("", "HostActiveDirectoryAuthentication", &self.mo_id, "ImportCertificateForCAM_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Deprecated as of vSphere API 8.0U3, and there is no replacement for it.
    /// 
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
        self.client.invoke_void("", "HostActiveDirectoryAuthentication", &self.mo_id, "InstallSmartCardTrustAnchor", Some(&input)).await
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
    pub async fn join_domain_task(&self, domain_name: &str, user_name: &str, password: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = JoinDomainRequestType {domain_name, user_name, password, };
        let bytes = self.client.invoke("", "HostActiveDirectoryAuthentication", &self.mo_id, "JoinDomain_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn join_domain_with_cam_task(&self, domain_name: &str, cam_server: &str) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = JoinDomainWithCamRequestType {domain_name, cam_server, };
        let bytes = self.client.invoke("", "HostActiveDirectoryAuthentication", &self.mo_id, "JoinDomainWithCAM_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn leave_current_domain_task(&self, force: bool) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = LeaveCurrentDomainRequestType {force, };
        let bytes = self.client.invoke("", "HostActiveDirectoryAuthentication", &self.mo_id, "LeaveCurrentDomain_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Deprecated as of vSphere API 8.0U3, and there is no replacement for it.
    /// 
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
        let bytes_opt = self.client.invoke_optional("", "HostActiveDirectoryAuthentication", &self.mo_id, "ListSmartCardTrustAnchors", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
        self.client.invoke_void("", "HostActiveDirectoryAuthentication", &self.mo_id, "RemoveSmartCardTrustAnchor", Some(&input)).await
    }
    /// Deprecated as of vSphere API 8.0U3, and there is no replacement for it.
    /// 
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
        self.client.invoke_void("", "HostActiveDirectoryAuthentication", &self.mo_id, "RemoveSmartCardTrustAnchorByFingerprint", Some(&input)).await
    }
    /// Remove a smart card trust anchor certificate from the system
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    /// 
    /// ***Required privileges:*** Host.Config.AuthenticationStore
    ///
    /// ## Parameters:
    ///
    /// ### certificate
    /// PEM encoded certificate to remove
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if the host configuration prevents the
    /// certificate from being removed.
    pub async fn remove_smart_card_trust_anchor_certificate(&self, certificate: &str) -> Result<()> {
        let input = RemoveSmartCardTrustAnchorCertificateRequestType {certificate, };
        self.client.invoke_void("", "HostActiveDirectoryAuthentication", &self.mo_id, "RemoveSmartCardTrustAnchorCertificate", Some(&input)).await
    }
    /// Deprecated as of vSphere API 8.0U3, and there is no replacement for it.
    /// 
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
        self.client.invoke_void("", "HostActiveDirectoryAuthentication", &self.mo_id, "ReplaceSmartCardTrustAnchors", Some(&input)).await
    }
    /// Information about the authentication store.
    pub async fn info(&self) -> Result<Box<dyn crate::types::traits::HostAuthenticationStoreInfoTrait>> {
        let bytes_opt = self.client.fetch_property_raw("", "HostActiveDirectoryAuthentication", &self.mo_id, "info").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property info was empty".to_string()))?;
        let result: Box<dyn crate::types::traits::HostAuthenticationStoreInfoTrait> = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct ImportCertificateForCamRequestType<'a> {
    cert_path: &'a str,
    cam_server: &'a str,
}

impl<'a> miniserde::Serialize for ImportCertificateForCamRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ImportCertificateForCamRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ImportCertificateForCamRequestTypeSer<'b, 'a> {
    data: &'b ImportCertificateForCamRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ImportCertificateForCamRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ImportCertificateForCAMRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("certPath"), &self.data.cert_path as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("camServer"), &self.data.cam_server as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct InstallSmartCardTrustAnchorRequestType<'a> {
    cert: &'a str,
}

impl<'a> miniserde::Serialize for InstallSmartCardTrustAnchorRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(InstallSmartCardTrustAnchorRequestTypeSer { data: self, seq: 0 }))
    }
}

struct InstallSmartCardTrustAnchorRequestTypeSer<'b, 'a> {
    data: &'b InstallSmartCardTrustAnchorRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for InstallSmartCardTrustAnchorRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"InstallSmartCardTrustAnchorRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cert"), &self.data.cert as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct JoinDomainRequestType<'a> {
    domain_name: &'a str,
    user_name: &'a str,
    password: &'a str,
}

impl<'a> miniserde::Serialize for JoinDomainRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(JoinDomainRequestTypeSer { data: self, seq: 0 }))
    }
}

struct JoinDomainRequestTypeSer<'b, 'a> {
    data: &'b JoinDomainRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for JoinDomainRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"JoinDomainRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("domainName"), &self.data.domain_name as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("userName"), &self.data.user_name as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("password"), &self.data.password as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct JoinDomainWithCamRequestType<'a> {
    domain_name: &'a str,
    cam_server: &'a str,
}

impl<'a> miniserde::Serialize for JoinDomainWithCamRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(JoinDomainWithCamRequestTypeSer { data: self, seq: 0 }))
    }
}

struct JoinDomainWithCamRequestTypeSer<'b, 'a> {
    data: &'b JoinDomainWithCamRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for JoinDomainWithCamRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"JoinDomainWithCAMRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("domainName"), &self.data.domain_name as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("camServer"), &self.data.cam_server as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct LeaveCurrentDomainRequestType {
    force: bool,
}

impl miniserde::Serialize for LeaveCurrentDomainRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(LeaveCurrentDomainRequestTypeSer { data: self, seq: 0 }))
    }
}

struct LeaveCurrentDomainRequestTypeSer<'b> {
    data: &'b LeaveCurrentDomainRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for LeaveCurrentDomainRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"LeaveCurrentDomainRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("force"), &self.data.force as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RemoveSmartCardTrustAnchorRequestType<'a> {
    issuer: &'a str,
    serial: &'a str,
}

impl<'a> miniserde::Serialize for RemoveSmartCardTrustAnchorRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveSmartCardTrustAnchorRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveSmartCardTrustAnchorRequestTypeSer<'b, 'a> {
    data: &'b RemoveSmartCardTrustAnchorRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveSmartCardTrustAnchorRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveSmartCardTrustAnchorRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("issuer"), &self.data.issuer as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("serial"), &self.data.serial as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RemoveSmartCardTrustAnchorByFingerprintRequestType<'a> {
    fingerprint: &'a str,
    digest: &'a str,
}

impl<'a> miniserde::Serialize for RemoveSmartCardTrustAnchorByFingerprintRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveSmartCardTrustAnchorByFingerprintRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveSmartCardTrustAnchorByFingerprintRequestTypeSer<'b, 'a> {
    data: &'b RemoveSmartCardTrustAnchorByFingerprintRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveSmartCardTrustAnchorByFingerprintRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveSmartCardTrustAnchorByFingerprintRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("fingerprint"), &self.data.fingerprint as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("digest"), &self.data.digest as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RemoveSmartCardTrustAnchorCertificateRequestType<'a> {
    certificate: &'a str,
}

impl<'a> miniserde::Serialize for RemoveSmartCardTrustAnchorCertificateRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveSmartCardTrustAnchorCertificateRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveSmartCardTrustAnchorCertificateRequestTypeSer<'b, 'a> {
    data: &'b RemoveSmartCardTrustAnchorCertificateRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveSmartCardTrustAnchorCertificateRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveSmartCardTrustAnchorCertificateRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("certificate"), &self.data.certificate as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ReplaceSmartCardTrustAnchorsRequestType<'a> {
    certs: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for ReplaceSmartCardTrustAnchorsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReplaceSmartCardTrustAnchorsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReplaceSmartCardTrustAnchorsRequestTypeSer<'b, 'a> {
    data: &'b ReplaceSmartCardTrustAnchorsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ReplaceSmartCardTrustAnchorsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReplaceSmartCardTrustAnchorsRequestType")),
                1 => {
                    let Some(ref val) = self.data.certs else { continue; };
                    return Some((std::borrow::Cow::Borrowed("certs"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
