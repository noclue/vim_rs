use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::DiagnosticManagerAuditRecordResult;
use crate::types::structs::DiagnosticManagerLogDescriptor;
use crate::types::structs::DiagnosticManagerLogHeader;
use crate::types::structs::ManagedObjectReference;
/// Provides an interface for obtaining diagnostic information on a host
/// (e.g.
/// 
/// generating and retrieving support logs on the host, reading audit
/// records).
/// For VirtualCenter, this includes the log files for the server daemon.
/// For an ESX Server host, this includes detailed log files for the VMkernel.
pub struct DiagnosticManager {
    client: Arc<Client>,
    mo_id: String,
}
impl DiagnosticManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Issue a "mark" to syslog and the audit trail.
    /// 
    /// The specified message string will be written to syslog and the audit
    /// trail. The "mark" audit record will contain the message string in
    /// its comment parameter.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.2
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    ///
    /// ## Parameters:
    ///
    /// ### message
    /// The string to be used.
    pub async fn emit_syslog_mark(&self, message: &str) -> Result<()> {
        let input = EmitSyslogMarkRequestType {message, };
        let path = format!("/DiagnosticManager/{moId}/EmitSyslogMark", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Retrieve audit records from their storage on the specified host.
    /// 
    /// Audit records are stored on the host in a (large) FIFO. The FIFO is
    /// continuously being written to due to system activities. It is the
    /// responsibility of the caller to issue reads fast enough to keep ahead
    /// of the write traffic.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    ///
    /// ## Parameters:
    ///
    /// ### token
    /// The token to be used for the operation. The first call must
    /// be made without a token. All subsequent calls use the token
    /// returned in AuditRecordStatus.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: The reader has failed to keep up with the write
    /// data rate. Data has been lost. It is up to the
    /// caller to decide how to react to this. One
    /// possibility is to "start again from the beginning"
    /// with a call with no token.
    /// 
    /// ***SystemError***: One more more errors (on the host) have occurred.
    /// One or more error strings are available to detail
    /// the issues.
    pub async fn fetch_audit_records(&self, token: Option<&str>) -> Result<DiagnosticManagerAuditRecordResult> {
        let input = FetchAuditRecordsRequestType {token, };
        let path = format!("/DiagnosticManager/{moId}/FetchAuditRecords", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Returns part of a log file.
    /// 
    /// Log entries are always returned chronologically, typically with the
    /// newest event last.
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Specifies the host. If not specified, then it defaults
    /// to the default server. For example, if called on
    /// VirtualCenter, then the value defaults to VirtualCenter
    /// logs.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### key
    /// A string key specifying the key for the log file to
    /// browse. Keys can be obtained using the queryDescriptions
    /// method.
    ///
    /// ### start
    /// The line number for the first entry to be returned. If the
    /// parameter is not specified, then the operation returns
    /// with lines starting from the top of the log.
    ///
    /// ### lines
    /// The number of lines to return. If not specified, then
    /// all lines are returned from the start value to the end of
    /// the file.
    ///
    /// ## Returns:
    ///
    /// A LogHeader that includes the log lines. Sometimes fewer log
    /// lines are returned than were requested. For example, fewer lines
    /// are returned than expected if the client requests lines that do
    /// not exist or if the server limits the number of lines that it
    /// returns. If zero lines are returned, then the end of the log
    /// file may have been reached.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if the key refers to a nonexistent log file or
    /// the log file is not of type "plain".
    /// 
    /// ***CannotAccessFile***: if the key refers to a file that cannot be
    /// accessed at the present time.
    pub async fn browse_diagnostic_log(&self, host: Option<&ManagedObjectReference>, key: &str, start: Option<i32>, lines: Option<i32>) -> Result<DiagnosticManagerLogHeader> {
        let input = BrowseDiagnosticLogRequestType {host, key, start, lines, };
        let path = format!("/DiagnosticManager/{moId}/BrowseDiagnosticLog", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated since version 5.0 M/N it is recommended to use the CGI
    /// interface for the host bundles, use the address instead:
    /// `https://<<ESX_name>>/cgi-bin/vm-support.cgi`
    /// for the VC bundles, use
    /// `https://<<VC_name>>/appliance/support-bundle`
    /// 
    /// The caller can download the bundles using an HTTP GET operation
    /// for each returned URL. Bundles are usually available for at least 24
    /// hours, but the caller should not assume that the returned URLs are
    /// valid indefinitely. Servers often automatically delete generated
    /// diagnostic bundles after some given period of time.
    /// 
    /// Instructs the server to generate diagnostic bundles.
    /// 
    /// A diagnostic bundle includes log files and other configuration
    /// information that can be used to investigate potential server issues.
    /// Virtual machine and guest operation system state is excluded from
    /// diagnostic bundles.
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    ///
    /// ## Parameters:
    ///
    /// ### include_default
    /// Specifies if the bundle should include the
    /// default server. If called on a VirtualCenter
    /// server, then this means the VirtualCenter
    /// diagnostic files. If called directly on a host,
    /// then includeDefault must be set to true.
    ///
    /// ### host
    /// Lists hosts that are included. This is only used
    /// when called on VirtualCenter. If called directly
    /// on a host, then this parameter must be empty.
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the operation. Upon success, the
    /// *info.result* property in the
    /// *Task* contains a list of
    /// *DiagnosticManagerBundleInfo* objects for each
    /// diagnostic bundle that has been generated.
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***LogBundlingFailed***: if generation of support bundle failed.
    /// 
    /// ***TaskInProgress***: if there is a pending request to generate a
    /// support bundle.
    pub async fn generate_log_bundles_task(&self, include_default: bool, host: Option<&[ManagedObjectReference]>) -> Result<ManagedObjectReference> {
        let input = GenerateLogBundlesRequestType {include_default, host, };
        let path = format!("/DiagnosticManager/{moId}/GenerateLogBundles_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Returns a list of diagnostic files for a given system.
    /// 
    /// ***Required privileges:*** Global.Diagnostics
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Specifies the host. If not specified, then it defaults
    /// to the server itself. For example, if called on
    /// VirtualCenter, then the value defaults to VirtualCenter
    /// logs. When called on an ESX server host, the host should
    /// not be specified.
    /// 
    /// Refers instance of *HostSystem*.
    pub async fn query_descriptions(&self, host: Option<&ManagedObjectReference>) -> Result<Option<Vec<DiagnosticManagerLogDescriptor>>> {
        let input = QueryDescriptionsRequestType {host, };
        let path = format!("/DiagnosticManager/{moId}/QueryDescriptions", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct EmitSyslogMarkRequestType<'a> {
    message: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct FetchAuditRecordsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    token: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct BrowseDiagnosticLogRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
    key: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    start: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    lines: Option<i32>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct GenerateLogBundlesRequestType<'a> {
    #[serde(rename = "includeDefault")]
    include_default: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a [ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryDescriptionsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
}
