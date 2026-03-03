use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// A task is used to monitor long running operations.
#[derive(Clone)]
pub struct VslmTask {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VslmTask {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Cancel a running or queued task.
    /// 
    /// A task may only be canceled if it is
    /// cancelable. Multiple cancel requests will be treated as a single
    /// cancellation request. Canceling a completed task will throw an
    /// InvalidState exception.
    /// 
    /// If a task is canceled, its runtime state will be set to
    /// *error* and the *VslmTaskInfo.cancelled* flag will
    /// be set to true.
    /// 
    /// A cancel operation is asynchronous. The operation may return before the
    /// task is canceled.
    ///
    /// ## Errors:
    ///
    /// ***InvalidState***: - if the task is already canceled or completed.
    pub async fn vslm_cancel_task(&self) -> Result<()> {
        let path = format!("/vslm/VslmTask/{moId}/VslmCancelTask", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Get detailed information about this task.
    ///
    /// ## Returns:
    ///
    /// TaskInfo
    pub async fn vslm_query_info(&self) -> Result<crate::types::structs::VslmTaskInfo> {
        let path = format!("/vslm/VslmTask/{moId}/VslmQueryInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::VslmTaskInfo = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Get the result of the task.
    pub async fn vslm_query_task_result(&self) -> Result<Option<crate::types::vim_any::VimAny>> {
        let path = format!("/vslm/VslmTask/{moId}/VslmQueryTaskResult", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::vim_any::VimAny>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
}
