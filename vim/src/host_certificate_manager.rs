use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::HostCertificateManagerCertificateInfo;
use crate::types::HostCertificateManagerCertificateSpec;
/// CertificateManager provides an interface for managing the SSL
/// certificates used by the server.
pub struct HostCertificateManager {
    client: Arc<VimClient>,
    mo_id: String,
}
impl HostCertificateManager {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Requests the server to generate a certificate-signing
    /// request (CSR) for itself.
    /// 
    /// The CSR is then typically
    /// provided to a Certificate Authority to sign and issue
    /// the SSL certificate for the server. Use *HostCertificateManager.InstallServerCertificate*
    /// to install this certificate.
    /// 
    /// ***Required privileges:*** Certificate.Manage
    ///
    /// ## Parameters:
    ///
    /// ### use_ip_address_as_common_name
    /// if true, use host's
    /// management IP address as CN in the CSR;
    /// otherwise use host's FQDN.
    ///
    /// ### spec
    /// is used to generate CSR for selected
    /// certificate kind.
    /// 
    /// ***Since:*** vSphere API Release 8.0.1.0
    ///
    /// ## Returns:
    ///
    /// CSR in PEM format
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if there's a problem generating the CSR.
    pub async fn generate_certificate_signing_request(&self, use_ip_address_as_common_name: bool, spec: Option<&HostCertificateManagerCertificateSpec>) -> Result<String> {
        let input = GenerateCertificateSigningRequestRequestType {use_ip_address_as_common_name, spec, };
        let path = format!("/HostCertificateManager/{moId}/GenerateCertificateSigningRequest", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Requests the server to generate a certificate-signing
    /// request (CSR) for itself.
    /// 
    /// Alternative version similar to *HostCertificateManager.GenerateCertificateSigningRequest*
    /// but takes a Distinguished Name (DN) as a parameter.
    /// 
    /// ***Required privileges:*** Certificate.Manage
    ///
    /// ## Parameters:
    ///
    /// ### distinguished_name
    /// DN to be used as subject in CSR.
    ///
    /// ### spec
    /// is used to generate CSR for selected certificate kind
    /// 
    /// ***Since:*** vSphere API Release 8.0.1.0
    ///
    /// ## Returns:
    ///
    /// CSR in PEM format
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if there's a problem generating the CSR.
    pub async fn generate_certificate_signing_request_by_dn(&self, distinguished_name: &str, spec: Option<&HostCertificateManagerCertificateSpec>) -> Result<String> {
        let input = GenerateCertificateSigningRequestByDnRequestType {distinguished_name, spec, };
        let path = format!("/HostCertificateManager/{moId}/GenerateCertificateSigningRequestByDn", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Installs a given SSL certificate on the server.
    /// 
    /// ***Required privileges:*** Certificate.Manage
    ///
    /// ## Parameters:
    ///
    /// ### cert
    /// SSL certificate in PEM format
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if there's a problem with the input certificate.
    pub async fn install_server_certificate(&self, cert: &str) -> Result<()> {
        let input = InstallServerCertificateRequestType {cert, };
        let path = format!("/HostCertificateManager/{moId}/InstallServerCertificate", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Fetches the SSL CRLs of Certificate Authorities that are trusted.
    /// 
    /// ***Required privileges:*** Certificate.Manage
    ///
    /// ## Returns:
    ///
    /// SSL CRLs of trusted CAs in PEM format
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if there's a problem with the certificate store.
    pub async fn list_ca_certificate_revocation_lists(&self) -> Result<Option<Vec<String>>> {
        let path = format!("/HostCertificateManager/{moId}/ListCACertificateRevocationLists", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Fetches the SSL certificates of Certificate Authorities
    /// that are trusted.
    /// 
    /// ***Required privileges:*** Certificate.Manage
    ///
    /// ## Returns:
    ///
    /// SSL certificates of trusted CAs in PEM format
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if there's a problem with the certificate store.
    pub async fn list_ca_certificates(&self) -> Result<Option<Vec<String>>> {
        let path = format!("/HostCertificateManager/{moId}/ListCACertificates", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Replaces the trusted Certificate Authority (CA)
    /// certificates and Certification Revocation List (CRL)
    /// used by the server with the provided values.
    /// 
    /// These determine whether the server can verify
    /// the identity of an external entity.
    /// 
    /// ***Required privileges:*** Certificate.Manage
    ///
    /// ## Parameters:
    ///
    /// ### ca_cert
    /// List of SSL certificates, in PEM format,
    /// of all CAs that should be trusted
    ///
    /// ### ca_crl
    /// List of SSL CRLs, in PEM format,
    /// issued by trusted CAs from the above list
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if there's a problem with the input certificates
    /// or CRLs.
    pub async fn replace_ca_certificates_and_cr_ls(&self, ca_cert: &[String], ca_crl: Option<&[String]>) -> Result<()> {
        let input = ReplaceCaCertificatesAndCrLsRequestType {ca_cert, ca_crl, };
        let path = format!("/HostCertificateManager/{moId}/ReplaceCACertificatesAndCRLs", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// the CertificateInfos of all known Certificates on
    /// the host
    /// 
    /// ***Since:*** vSphere API Release 8.0.1.0
    /// 
    /// ***Required privileges:*** Certificate.Manage
    pub async fn retrieve_certificate_info_list(&self) -> Result<Option<Vec<HostCertificateManagerCertificateInfo>>> {
        let path = format!("/HostCertificateManager/{moId}/RetrieveCertificateInfoList", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// the CertificateInfo of the Host Certificate.
    /// 
    /// ***Required privileges:*** Certificate.Manage
    pub async fn certificate_info(&self) -> Result<HostCertificateManagerCertificateInfo> {
        let path = format!("/HostCertificateManager/{moId}/certificateInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct GenerateCertificateSigningRequestRequestType<'a> {
    #[serde(rename = "useIpAddressAsCommonName")]
    use_ip_address_as_common_name: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    spec: Option<&'a HostCertificateManagerCertificateSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct GenerateCertificateSigningRequestByDnRequestType<'a> {
    #[serde(rename = "distinguishedName")]
    distinguished_name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    spec: Option<&'a HostCertificateManagerCertificateSpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct InstallServerCertificateRequestType<'a> {
    cert: &'a str,
}
#[derive(serde::Serialize)]
#[serde(rename = "ReplaceCACertificatesAndCRLsRequestType", tag = "_typeName")]
struct ReplaceCaCertificatesAndCrLsRequestType<'a> {
    #[serde(rename = "caCert")]
    ca_cert: &'a [String],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "caCrl")]
    ca_crl: Option<&'a [String]>,
}
