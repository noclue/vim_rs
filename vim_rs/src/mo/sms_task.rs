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
        let path = format!("/sms/SmsTask/{moId}/QuerySmsTaskInfo", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::SmsTaskInfo = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Get the result of the task.
    /// 
    /// ***Required privileges:*** StorageViews.View
    pub async fn query_sms_task_result(&self) -> Result<Option<crate::types::vim_any::VimAny>> {
        let path = format!("/sms/SmsTask/{moId}/QuerySmsTaskResult", moId = &self.mo_id);
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
