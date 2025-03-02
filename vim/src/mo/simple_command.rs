use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::ServiceManagerServiceInfo;
/// A managed object that wraps the execution of a single arbitrary
/// command.
/// 
/// The specific command executed is assumed to be known from
/// the service name by the client invoking this command. This object
/// presents a generic interface for such services.
pub struct SimpleCommand {
    client: Arc<Client>,
    mo_id: String,
}
impl SimpleCommand {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
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
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// The encoding type used in the result.
    pub async fn encoding_type(&self) -> Result<crate::types::enums::SimpleCommandEncodingEnum> {
        let path = format!("/SimpleCommand/{moId}/encodingType", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// A description of the service.
    pub async fn entity(&self) -> Result<ServiceManagerServiceInfo> {
        let path = format!("/SimpleCommand/{moId}/entity", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ExecuteSimpleCommandRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    arguments: Option<&'a [String]>,
}
