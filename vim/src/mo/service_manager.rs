use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::ServiceManagerServiceInfo;
/// The ServiceManager managed object is a singleton object that is used to present
/// services that are optional and not necessarily formally defined.
/// 
/// This directory makes available a list of such services and provides an easy way
/// to locate them. The service being represented can take arbitrary form here and
/// is thus represented by a generic ManagedObject. The expectation is that the
/// client side is knowledgeable of the instance type of the specific service it
/// is interested in using.
pub struct ServiceManager {
    client: Arc<Client>,
    mo_id: String,
}
impl ServiceManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// A query interface that returns a list of services that match certain criteria.
    /// 
    /// Besides a basic service name entry, an arbitrary list of matching locations
    /// can also be specified. The location array is assumed to be a list of AND expressions,
    /// ie, all locations must match for an entry to be considered a match.
    /// Regular expressions are not allowed in the query service.
    /// 
    /// ***Required privileges:*** Global.ServiceManagers
    ///
    /// ## Parameters:
    ///
    /// ### service_name
    /// The name of the service to be located.
    ///
    /// ### location
    /// The list of location information that needs to match for a service to be
    /// considered a match.
    pub async fn query_service_list(&self, service_name: Option<&str>, location: Option<&[String]>) -> Result<Option<Vec<ServiceManagerServiceInfo>>> {
        let input = QueryServiceListRequestType {service_name, location, };
        let path = format!("/ServiceManager/{moId}/QueryServiceList", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// The full list of services available in this directory.
    /// 
    /// ***Required privileges:*** Global.ServiceManagers
    pub async fn service(&self) -> Result<Option<Vec<ServiceManagerServiceInfo>>> {
        let path = format!("/ServiceManager/{moId}/service", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryServiceListRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serviceName")]
    service_name: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    location: Option<&'a [String]>,
}
