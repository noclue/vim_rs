use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Deprecated as of vSphere 9.0. Please refer to vLCM APIs.
/// 
/// Base class for the <code>Agent</code>, <code>Agency</code> and
/// <code>EsxAgentManager</code> classes.
#[derive(Clone)]
pub struct EamObject {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl EamObject {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Current issues that have been detected for this entity.
    /// 
    /// Each issue can be remediated
    /// by invoking *EamObject.Resolve* or *EamObject.ResolveAll*.
    /// 
    /// Requires view privileges.
    ///
    /// ## Parameters:
    ///
    /// ### issue_key
    /// An optional array of issue keys. If not set, all issues for this
    /// entity are returned.
    ///
    /// ## Returns:
    ///
    /// A possibly empty array of issues that match the input <code>issueKey</code> array. Note
    /// that the returned array can be smaller than <code>issueKey</code> if one or more
    /// issue keys refers to issues that this entity does not have.
    pub async fn query_issue(&self, issue_key: Option<&[i32]>) -> Result<Option<Vec<Box<dyn crate::types::traits::IssueTrait>>>> {
        let input = QueryIssueRequestType {issue_key, };
        let path = format!("/eam/EamObject/{moId}/QueryIssue", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<Box<dyn crate::types::traits::IssueTrait>>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Resolves the issues specified in the input.
    /// 
    /// If an issue is remediable, ESX
    /// Agent Manager
    /// tries to resolve the misconfiguration that caused the issue. If it is not
    /// remediable, the offending issue is removed and ESX Agent Manager assumes that the issue has been
    /// resolved.
    /// 
    /// Requires modify privileges.
    /// 
    /// See also *Issue*.
    ///
    /// ## Parameters:
    ///
    /// ### issue_key
    /// A non-empty array of issue keys.
    ///
    /// ## Returns:
    ///
    /// A possibly empty array of issue keys for the issues that were not found on the
    /// entity. This can happen if <code>resolve</code> is called with issue keys that were
    /// resolved just prior to calling <code>resolve</code> or if an issue is currenly not resolvable.
    pub async fn resolve(&self, issue_key: &[i32]) -> Result<Option<Vec<i32>>> {
        let input = ResolveRequestType {issue_key, };
        let path = format!("/eam/EamObject/{moId}/Resolve", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<i32>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Resolve all outstanding issues.
    /// 
    /// The method calls *EamObject.Resolve*
    /// with all issues the <code>EsxAgentManager</code>, <code>Agency</code>, or
    /// <code>Agent</code> have encountered. It is the equivalent of calling the following methods:
    /// - <code>agent.resolve(getIssueKeys(agent.getRuntime().getIssue()));</code>
    ///   for <code>Agent</code> objects
    /// - <code>agency.resolve(getIssueKeys(agency.getRuntime().getIssue()));</code>
    ///   for <code>Agency</code> objects
    /// - <code>esxAgentManager.resolve(getIssueKeys(esxAgentManager.getIssue()));</code>
    ///   for the <code>EsxAgentManager</code> object.
    ///   
    /// Requires modify privileges.
    /// 
    /// See also *Issue*.
    pub async fn resolve_all(&self) -> Result<()> {
        let path = format!("/eam/EamObject/{moId}/ResolveAll", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
}
struct QueryIssueRequestType<'a> {
    issue_key: Option<&'a [i32]>,
}

impl<'a> miniserde::Serialize for QueryIssueRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryIssueRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryIssueRequestTypeSer<'b, 'a> {
    data: &'b QueryIssueRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryIssueRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryIssueRequestType")),
                1 => {
                    let Some(ref val) = self.data.issue_key else { continue; };
                    return Some((std::borrow::Cow::Borrowed("issueKey"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ResolveRequestType<'a> {
    issue_key: &'a [i32],
}

impl<'a> miniserde::Serialize for ResolveRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ResolveRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ResolveRequestTypeSer<'b, 'a> {
    data: &'b ResolveRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ResolveRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ResolveRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("issueKey"), &self.data.issue_key as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
