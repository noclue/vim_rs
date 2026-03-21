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
        self.client.invoke_void("vslm", "VslmTask", &self.mo_id, "VslmCancelTask", None).await
    }
    /// Get detailed information about this task.
    ///
    /// ## Returns:
    ///
    /// TaskInfo
    pub async fn vslm_query_info(&self) -> Result<crate::types::structs::VslmTaskInfo> {
        let bytes = self.client.invoke("vslm", "VslmTask", &self.mo_id, "VslmQueryInfo", None).await?;
        let result: crate::types::structs::VslmTaskInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Get the result of the task.
    pub async fn vslm_query_task_result(&self) -> Result<Option<crate::types::vim_any::VimAny>> {
        let bytes_opt = self.client.invoke_optional("vslm", "VslmTask", &self.mo_id, "VslmQueryTaskResult", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
}
