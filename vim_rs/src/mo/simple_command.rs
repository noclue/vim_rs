use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// A managed object that wraps the execution of a single arbitrary
/// command.
/// 
/// The specific command executed is assumed to be known from
/// the service name by the client invoking this command. This object
/// presents a generic interface for such services.
#[derive(Clone)]
pub struct SimpleCommand {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl SimpleCommand {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// The single function execution point for this simple command.
    /// 
    /// The actual effects of
    /// this command depend upon the service handler registered for this instance.
    /// 
    /// ***Required privileges:*** Global.ServiceManagers
    ///
    /// ## Parameters:
    ///
    /// ### arguments
    /// An arbitrary collection of arguments.
    pub async fn execute_simple_command(&self, arguments: Option<&[String]>) -> Result<String> {
        let input = ExecuteSimpleCommandRequestType {arguments, };
        let path = format!("/SimpleCommand/{moId}/ExecuteSimpleCommand", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: String = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// The encoding type used in the result.
    pub async fn encoding_type(&self) -> Result<crate::types::enums::SimpleCommandEncodingEnum> {
        let path = format!("/SimpleCommand/{moId}/encodingType", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::enums::SimpleCommandEncodingEnum = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// A description of the service.
    pub async fn entity(&self) -> Result<crate::types::structs::ServiceManagerServiceInfo> {
        let path = format!("/SimpleCommand/{moId}/entity", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ServiceManagerServiceInfo = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct ExecuteSimpleCommandRequestType<'a> {
    arguments: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for ExecuteSimpleCommandRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ExecuteSimpleCommandRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ExecuteSimpleCommandRequestTypeSer<'b, 'a> {
    data: &'b ExecuteSimpleCommandRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ExecuteSimpleCommandRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ExecuteSimpleCommandRequestType")),
                1 => {
                    let Some(ref val) = self.data.arguments else { continue; };
                    return Some((std::borrow::Cow::Borrowed("arguments"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
