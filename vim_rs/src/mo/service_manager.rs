use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The ServiceManager managed object is a singleton object that is used to present
/// services that are optional and not necessarily formally defined.
/// 
/// This directory makes available a list of such services and provides an easy way
/// to locate them. The service being represented can take arbitrary form here and
/// is thus represented by a generic ManagedObject. The expectation is that the
/// client side is knowledgeable of the instance type of the specific service it
/// is interested in using.
#[derive(Clone)]
pub struct ServiceManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl ServiceManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn query_service_list(&self, service_name: Option<&str>, location: Option<&[String]>) -> Result<Option<Vec<crate::types::structs::ServiceManagerServiceInfo>>> {
        let input = QueryServiceListRequestType {service_name, location, };
        let bytes_opt = self.client.invoke_optional("", "ServiceManager", &self.mo_id, "QueryServiceList", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// The full list of services available in this directory.
    /// 
    /// ***Required privileges:*** Global.ServiceManagers
    pub async fn service(&self) -> Result<Option<Vec<crate::types::structs::ServiceManagerServiceInfo>>> {
        let bytes_opt = self.client.fetch_property_raw("", "ServiceManager", &self.mo_id, "service").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
}
struct QueryServiceListRequestType<'a> {
    service_name: Option<&'a str>,
    location: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for QueryServiceListRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryServiceListRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryServiceListRequestTypeSer<'b, 'a> {
    data: &'b QueryServiceListRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryServiceListRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryServiceListRequestType")),
                1 => {
                    let Some(ref val) = self.data.service_name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("serviceName"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.location else { continue; };
                    return Some((std::borrow::Cow::Borrowed("location"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
