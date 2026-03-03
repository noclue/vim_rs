use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// VsanMassCollector contains a collection of APIs to query vSAN management API's
/// and values of managed object properties.
/// 
/// The Managed Entity can be accessed
/// through MOID of vsan-mass-collector through vSAN service at vCenter server
/// side.
#[derive(Clone)]
pub struct VsanMassCollector {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanMassCollector {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Retrieve properties for an object type with specs.
    /// 
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### mass_collector_specs
    /// specify a list of objects, properties, constraints
    /// for retrieve.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_retrieve_properties(&self, mass_collector_specs: &[crate::types::structs::VsanMassCollectorSpec]) -> Result<Option<Vec<crate::types::structs::ObjectContent>>> {
        let input = VsanRetrievePropertiesRequestType {mass_collector_specs, };
        let path = format!("/vsan/VsanMassCollector/{moId}/VsanRetrieveProperties", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::ObjectContent>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
}
struct VsanRetrievePropertiesRequestType<'a> {
    mass_collector_specs: &'a [crate::types::structs::VsanMassCollectorSpec],
}

impl<'a> miniserde::Serialize for VsanRetrievePropertiesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanRetrievePropertiesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanRetrievePropertiesRequestTypeSer<'b, 'a> {
    data: &'b VsanRetrievePropertiesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanRetrievePropertiesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanRetrievePropertiesRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("massCollectorSpecs"), &self.data.mass_collector_specs as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
