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
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::ObjectContent>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanRetrievePropertiesRequestType<'a> {
    #[serde(rename = "massCollectorSpecs")]
    mass_collector_specs: &'a [crate::types::structs::VsanMassCollectorSpec],
}
