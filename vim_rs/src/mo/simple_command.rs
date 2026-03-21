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
        let bytes = self.client.invoke("", "SimpleCommand", &self.mo_id, "ExecuteSimpleCommand", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// The encoding type used in the result.
    pub async fn encoding_type(&self) -> Result<crate::types::enums::SimpleCommandEncodingEnum> {
        let bytes_opt = self.client.fetch_property_raw("", "SimpleCommand", &self.mo_id, "encodingType").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property encodingType was empty".to_string()))?;
        let result: crate::types::enums::SimpleCommandEncodingEnum = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// A description of the service.
    pub async fn entity(&self) -> Result<crate::types::structs::ServiceManagerServiceInfo> {
        let bytes_opt = self.client.fetch_property_raw("", "SimpleCommand", &self.mo_id, "entity").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property entity was empty".to_string()))?;
        let result: crate::types::structs::ServiceManagerServiceInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
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
