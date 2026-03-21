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
        let bytes_opt = self.client.invoke_optional("vsan", "VsanMassCollector", &self.mo_id, "VsanRetrieveProperties", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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
