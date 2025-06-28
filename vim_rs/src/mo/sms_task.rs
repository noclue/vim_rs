use std::sync::Arc;
use crate::core::client::{Client, Result};
/// A task is used to monitor long running operations.
#[derive(Clone)]
pub struct SmsTask {
    client: Arc<Client>,
    mo_id: String,
}
impl SmsTask {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
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
        let path = format!("/sms/SmsTask/{moId}/QuerySmsTaskInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute(req).await
    }
    /// Get the result of the task.
    /// 
    /// ***Required privileges:*** StorageViews.View
    pub async fn query_sms_task_result(&self) -> Result<Option<crate::types::vim_any::VimAny>> {
        let path = format!("/sms/SmsTask/{moId}/QuerySmsTaskResult", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_option(req).await
    }
}
