use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::ManagedObjectReference;
/// CertificateManager provides an interface for managing the SSL
/// certificates used by the ESX server.
pub struct CertificateManager {
    client: Arc<Client>,
    mo_id: String,
}
impl CertificateManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Re-fetches certificates of trusted CAs and the Certificate Revocation
    /// Lists (CRL) from the appropriate authoritative source and pushes them to
    /// the hosts.
    /// 
    /// ***Required privileges:*** Certificate.Manage
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// the hosts on which the certificates need to be refreshed
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    pub async fn cert_mgr_refresh_ca_certificates_and_cr_ls_task(&self, host: &[ManagedObjectReference]) -> Result<ManagedObjectReference> {
        let input = CertMgrRefreshCaCertificatesAndCrLsRequestType {host, };
        let path = format!("/CertificateManager/{moId}/CertMgrRefreshCACertificatesAndCRLs_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Gets CSRs from the hosts and then gets these certificates signed by the
    /// VMware Certificate Service and pushes them down to the hosts.
    /// 
    /// ***Required privileges:*** Certificate.Manage
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// the hosts on which the certificates need to be refreshed
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    pub async fn cert_mgr_refresh_certificates_task(&self, host: &[ManagedObjectReference]) -> Result<ManagedObjectReference> {
        let input = CertMgrRefreshCertificatesRequestType {host, };
        let path = format!("/CertificateManager/{moId}/CertMgrRefreshCertificates_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Revokes the certificates of some hosts.
    /// 
    /// ***Required privileges:*** Certificate.Manage
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// the hosts on which the certificates need to be revoked
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    pub async fn cert_mgr_revoke_certificates_task(&self, host: &[ManagedObjectReference]) -> Result<ManagedObjectReference> {
        let input = CertMgrRevokeCertificatesRequestType {host, };
        let path = format!("/CertificateManager/{moId}/CertMgrRevokeCertificates_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(rename = "CertMgrRefreshCACertificatesAndCRLsRequestType", tag = "_typeName")]
struct CertMgrRefreshCaCertificatesAndCrLsRequestType<'a> {
    host: &'a [ManagedObjectReference],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CertMgrRefreshCertificatesRequestType<'a> {
    host: &'a [ManagedObjectReference],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CertMgrRevokeCertificatesRequestType<'a> {
    host: &'a [ManagedObjectReference],
}
