use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::OptionDef;
use crate::types::structs::OptionValueTrait;
/// This managed object type is used for managing key/value pair
/// options.
/// - You can define options on the fly only if the option is supported
///   by the concrete implementation, in a logical tree using a dot notation
///   for keys. For example, "Ethernet.Connection" describes the Connection
///   option as child of the Ethernet option.
/// - Options can be updated even if not visible in supportedOption or
///   settings or the queryMethod returned values only if supported by the
///   concrete implementation.
/// - Attempt to add random Options that are not supported by the concrete
///   implementation may result in unexpected side-effects.
/// - You can use the queryMethod to retrieve a single property or
///   a subset of properties based on the dot notation path.
pub struct OptionManager {
    client: Arc<Client>,
    mo_id: String,
}
impl OptionManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Returns a specific node or nodes in the option hierarchy.
    /// 
    /// This method might require any of the following privileges depending
    /// on where the property fits in the inventory tree.
    /// - System.View on the root folder, if this is used to read settings
    ///   in the &quot;client&quot; subtree.
    /// - System.Read on the root folder, if this is used to read all settings
    ///   or any settings beside those in the &quot;client&quot; subtree.
    /// - System.Read on the host, if this is used to read the advanced
    ///   options for a host configuration.
    ///   
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// -
    ///
    /// ## Returns:
    ///
    /// The option with the given name. If the name ends with a
    /// dot, all options for that subtree are returned.
    ///
    /// ## Errors:
    ///
    /// ***InvalidName***: if no option or subtree exists with the
    /// given name.
    pub async fn query_options(&self, name: Option<&str>) -> Result<Option<Vec<Box<dyn OptionValueTrait>>>> {
        let input = QueryOptionsRequestType {name, };
        let path = format!("/OptionManager/{moId}/QueryOptions", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Updates one or more options.
    /// 
    /// The options are changed one by one, and the operation is not atomic.
    /// This means that on failure some of the options may not be changed.
    /// 
    /// A nested option setting can be named using a dot notation; for example,
    /// system.cacheSize.
    /// 
    /// This method might require any of the following privileges depending
    /// on where the property fits in the inventory tree.
    /// - Global.Settings on the root folder, if this is used to modify the
    ///   settings in the service node.
    /// - Host.Config.AdvancedConfig on the host, if this is used to set the
    ///   advanced options in the host configuration.
    ///
    /// ## Parameters:
    ///
    /// ### changed_value
    /// -
    ///
    /// ## Errors:
    ///
    /// ***InvalidName***: if one or more OptionValue objects refers to a
    /// non-existent option.
    /// 
    /// ***InvalidArgument***: if one or more OptionValue contains an
    /// invalid value.
    pub async fn update_options(&self, changed_value: &[Box<dyn OptionValueTrait>]) -> Result<()> {
        let input = UpdateOptionsRequestType {changed_value, };
        let path = format!("/OptionManager/{moId}/UpdateOptions", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// A list of the current settings for the key/value pair options.
    pub async fn setting(&self) -> Result<Option<Vec<Box<dyn OptionValueTrait>>>> {
        let path = format!("/OptionManager/{moId}/setting", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// A list of supported key/value pair options including their
    /// type information.
    pub async fn supported_option(&self) -> Result<Option<Vec<OptionDef>>> {
        let path = format!("/OptionManager/{moId}/supportedOption", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryOptionsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateOptionsRequestType<'a> {
    #[serde(rename = "changedValue")]
    changed_value: &'a [Box<dyn OptionValueTrait>],
}
