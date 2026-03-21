use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// A task is used to monitor long running operations.
#[derive(Clone)]
pub struct SmsTask {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl SmsTask {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Get detailed information about this task.
    /// 
    /// ***Required privileges:*** StorageViews.View
    ///
    /// ## Returns:
    ///
    /// TaskInfo
    pub async fn query_sms_task_info(&self) -> Result<crate::types::structs::SmsTaskInfo> {
        let bytes = self.client.invoke("sms", "SmsTask", &self.mo_id, "QuerySmsTaskInfo", None).await?;
        let result: crate::types::structs::SmsTaskInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Get the result of the task.
    /// 
    /// ***Required privileges:*** StorageViews.View
    pub async fn query_sms_task_result(&self) -> Result<Option<crate::types::vim_any::VimAny>> {
        let bytes_opt = self.client.invoke_optional("sms", "SmsTask", &self.mo_id, "QuerySmsTaskResult", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
}
