use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The KernelModuleSystem managed object controls the configuration
/// of kernel modules on the host.
#[derive(Clone)]
pub struct HostKernelModuleSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostKernelModuleSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Query the options configured to be passed to the kernel module when loaded.
    /// 
    /// Note that this is not necessarily the option string currently in use by
    /// the kernel module.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// Module name.
    ///
    /// ## Returns:
    ///
    /// Option string to be passed to the kernel module at load time.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the kernel module does not exist on the host.
    pub async fn query_configured_module_option_string(&self, name: &str) -> Result<String> {
        let input = QueryConfiguredModuleOptionStringRequestType {name, };
        let bytes = self.client.invoke("", "HostKernelModuleSystem", &self.mo_id, "QueryConfiguredModuleOptionString", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Query the set of modules on the host.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    pub async fn query_modules(&self) -> Result<Option<Vec<crate::types::structs::KernelModuleInfo>>> {
        let bytes_opt = self.client.invoke_optional("", "HostKernelModuleSystem", &self.mo_id, "QueryModules", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Specifies the options to be passed to the kernel module when loaded.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// Module name.
    ///
    /// ### options
    /// Option string to be passed to the kernel module at
    /// load time.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the kernel module does not exist on the host.
    pub async fn update_module_option_string(&self, name: &str, options: &str) -> Result<()> {
        let input = UpdateModuleOptionStringRequestType {name, options, };
        self.client.invoke_void("", "HostKernelModuleSystem", &self.mo_id, "UpdateModuleOptionString", Some(&input)).await
    }
}
struct QueryConfiguredModuleOptionStringRequestType<'a> {
    name: &'a str,
}

impl<'a> miniserde::Serialize for QueryConfiguredModuleOptionStringRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryConfiguredModuleOptionStringRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryConfiguredModuleOptionStringRequestTypeSer<'b, 'a> {
    data: &'b QueryConfiguredModuleOptionStringRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryConfiguredModuleOptionStringRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryConfiguredModuleOptionStringRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateModuleOptionStringRequestType<'a> {
    name: &'a str,
    options: &'a str,
}

impl<'a> miniserde::Serialize for UpdateModuleOptionStringRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateModuleOptionStringRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateModuleOptionStringRequestTypeSer<'b, 'a> {
    data: &'b UpdateModuleOptionStringRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateModuleOptionStringRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateModuleOptionStringRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("options"), &self.data.options as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
