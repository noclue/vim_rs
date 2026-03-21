use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// CertificateManager provides an interface for managing the SSL
/// certificates used by the ESX server.
#[derive(Clone)]
pub struct CertificateManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl CertificateManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn cert_mgr_refresh_ca_certificates_and_cr_ls_task(&self, host: &[crate::types::structs::ManagedObjectReference]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CertMgrRefreshCaCertificatesAndCrLsRequestType {host, };
        let bytes = self.client.invoke("", "CertificateManager", &self.mo_id, "CertMgrRefreshCACertificatesAndCRLs_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn cert_mgr_refresh_certificates_task(&self, host: &[crate::types::structs::ManagedObjectReference]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CertMgrRefreshCertificatesRequestType {host, };
        let bytes = self.client.invoke("", "CertificateManager", &self.mo_id, "CertMgrRefreshCertificates_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn cert_mgr_revoke_certificates_task(&self, host: &[crate::types::structs::ManagedObjectReference]) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CertMgrRevokeCertificatesRequestType {host, };
        let bytes = self.client.invoke("", "CertificateManager", &self.mo_id, "CertMgrRevokeCertificates_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct CertMgrRefreshCaCertificatesAndCrLsRequestType<'a> {
    host: &'a [crate::types::structs::ManagedObjectReference],
}

impl<'a> miniserde::Serialize for CertMgrRefreshCaCertificatesAndCrLsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CertMgrRefreshCaCertificatesAndCrLsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CertMgrRefreshCaCertificatesAndCrLsRequestTypeSer<'b, 'a> {
    data: &'b CertMgrRefreshCaCertificatesAndCrLsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CertMgrRefreshCaCertificatesAndCrLsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CertMgrRefreshCACertificatesAndCRLsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("host"), &self.data.host as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CertMgrRefreshCertificatesRequestType<'a> {
    host: &'a [crate::types::structs::ManagedObjectReference],
}

impl<'a> miniserde::Serialize for CertMgrRefreshCertificatesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CertMgrRefreshCertificatesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CertMgrRefreshCertificatesRequestTypeSer<'b, 'a> {
    data: &'b CertMgrRefreshCertificatesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CertMgrRefreshCertificatesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CertMgrRefreshCertificatesRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("host"), &self.data.host as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CertMgrRevokeCertificatesRequestType<'a> {
    host: &'a [crate::types::structs::ManagedObjectReference],
}

impl<'a> miniserde::Serialize for CertMgrRevokeCertificatesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CertMgrRevokeCertificatesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CertMgrRevokeCertificatesRequestTypeSer<'b, 'a> {
    data: &'b CertMgrRevokeCertificatesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CertMgrRevokeCertificatesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CertMgrRevokeCertificatesRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("host"), &self.data.host as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
