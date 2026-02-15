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
        let path = format!("/ServiceManager/{moId}/QueryServiceList", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::ServiceManagerServiceInfo>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// The full list of services available in this directory.
    /// 
    /// ***Required privileges:*** Global.ServiceManagers
    pub async fn service(&self) -> Result<Option<Vec<crate::types::structs::ServiceManagerServiceInfo>>> {
        let path = format!("/ServiceManager/{moId}/service", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::ServiceManagerServiceInfo>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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

impl miniserde::ser::Map for QueryServiceListRequestTypeSer<'_, '_> {
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
