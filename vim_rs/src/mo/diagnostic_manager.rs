use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Provides an interface for obtaining diagnostic information on a host
/// (e.g.
/// 
/// generating and retrieving support logs on the host, reading audit
/// records).
/// For VirtualCenter, this includes the log files for the server daemon.
/// For an ESX Server host, this includes detailed log files for the VMkernel.
#[derive(Clone)]
pub struct DiagnosticManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl DiagnosticManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
        self.client.invoke_void("", "DiagnosticManager", &self.mo_id, "EmitSyslogMark", Some(&input)).await
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
    pub async fn fetch_audit_records(&self, token: Option<&str>) -> Result<crate::types::structs::DiagnosticManagerAuditRecordResult> {
        let input = FetchAuditRecordsRequestType {token, };
        let bytes = self.client.invoke("", "DiagnosticManager", &self.mo_id, "FetchAuditRecords", Some(&input)).await?;
        let result: crate::types::structs::DiagnosticManagerAuditRecordResult = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn browse_diagnostic_log(&self, host: Option<&crate::types::structs::ManagedObjectReference>, key: &str, start: Option<i32>, lines: Option<i32>) -> Result<crate::types::structs::DiagnosticManagerLogHeader> {
        let input = BrowseDiagnosticLogRequestType {host, key, start, lines, };
        let bytes = self.client.invoke("", "DiagnosticManager", &self.mo_id, "BrowseDiagnosticLog", Some(&input)).await?;
        let result: crate::types::structs::DiagnosticManagerLogHeader = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn generate_log_bundles_task(&self, include_default: bool, host: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = GenerateLogBundlesRequestType {include_default, host, };
        let bytes = self.client.invoke("", "DiagnosticManager", &self.mo_id, "GenerateLogBundles_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn query_descriptions(&self, host: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<crate::types::structs::DiagnosticManagerLogDescriptor>>> {
        let input = QueryDescriptionsRequestType {host, };
        let bytes_opt = self.client.invoke_optional("", "DiagnosticManager", &self.mo_id, "QueryDescriptions", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
}
struct EmitSyslogMarkRequestType<'a> {
    message: &'a str,
}

impl<'a> miniserde::Serialize for EmitSyslogMarkRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(EmitSyslogMarkRequestTypeSer { data: self, seq: 0 }))
    }
}

struct EmitSyslogMarkRequestTypeSer<'b, 'a> {
    data: &'b EmitSyslogMarkRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for EmitSyslogMarkRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"EmitSyslogMarkRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("message"), &self.data.message as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct FetchAuditRecordsRequestType<'a> {
    token: Option<&'a str>,
}

impl<'a> miniserde::Serialize for FetchAuditRecordsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(FetchAuditRecordsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct FetchAuditRecordsRequestTypeSer<'b, 'a> {
    data: &'b FetchAuditRecordsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for FetchAuditRecordsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"FetchAuditRecordsRequestType")),
                1 => {
                    let Some(ref val) = self.data.token else { continue; };
                    return Some((std::borrow::Cow::Borrowed("token"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct BrowseDiagnosticLogRequestType<'a> {
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
    key: &'a str,
    start: Option<i32>,
    lines: Option<i32>,
}

impl<'a> miniserde::Serialize for BrowseDiagnosticLogRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(BrowseDiagnosticLogRequestTypeSer { data: self, seq: 0 }))
    }
}

struct BrowseDiagnosticLogRequestTypeSer<'b, 'a> {
    data: &'b BrowseDiagnosticLogRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for BrowseDiagnosticLogRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"BrowseDiagnosticLogRequestType")),
                1 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("key"), &self.data.key as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.start else { continue; };
                    return Some((std::borrow::Cow::Borrowed("start"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.lines else { continue; };
                    return Some((std::borrow::Cow::Borrowed("lines"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct GenerateLogBundlesRequestType<'a> {
    include_default: bool,
    host: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for GenerateLogBundlesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(GenerateLogBundlesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct GenerateLogBundlesRequestTypeSer<'b, 'a> {
    data: &'b GenerateLogBundlesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for GenerateLogBundlesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"GenerateLogBundlesRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("includeDefault"), &self.data.include_default as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryDescriptionsRequestType<'a> {
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for QueryDescriptionsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryDescriptionsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryDescriptionsRequestTypeSer<'b, 'a> {
    data: &'b QueryDescriptionsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryDescriptionsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryDescriptionsRequestType")),
                1 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
