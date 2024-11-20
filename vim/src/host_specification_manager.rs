use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::HostSubSpecification;
use crate::types::HostSpecification;
use crate::types::ManagedObjectReference;
/// The *HostSpecificationManager* provides the
/// functionality to update, restrieve and delete
/// *HostSpecification* and
/// *HostSubSpecification*.
pub struct HostSpecificationManager {
    client: Arc<VimClient>,
    mo_id: String,
}
impl HostSpecificationManager {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Delete the host specification of the specified host.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// The specified host for which the specification will be
    /// deleted.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Errors:
    ///
    /// ***HostSpecificationOperationFailed***: If the method fails when delete
    /// the spec.
    pub async fn delete_host_specification(&self, host: &ManagedObjectReference) -> Result<()> {
        let input = DeleteHostSpecificationRequestType {host, };
        let path = format!("/HostSpecificationManager/{moId}/DeleteHostSpecification", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Delete the host sub specification specified by the provided <code>
    /// subSpecname</code> of the specified host.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// The specified host for which the sub specification will be
    /// deleted.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### sub_spec_name
    /// The name of the host sub specification to be deleted.
    ///
    /// ## Errors:
    ///
    /// ***HostSpecificationOperationFailed***: If the method fails when delete
    /// the sub spec.
    pub async fn delete_host_sub_specification(&self, host: &ManagedObjectReference, sub_spec_name: &str) -> Result<()> {
        let input = DeleteHostSubSpecificationRequestType {host, sub_spec_name, };
        let path = format!("/HostSpecificationManager/{moId}/DeleteHostSubSpecification", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Query the hosts whose specification was updated in the specified
    /// time period.
    /// 
    /// When the <code>startChangeID</code> isn't provided, it will
    /// return all the host updated before the <code>endChangeID</code>. When the
    /// <code>endChangeID</code> isn't provided, it will return all the hosts
    /// updated after <code>startChangeID</code>. If both aren't provided, all
    /// hosts having host spec will be returned.
    /// The format of the change ID is defined at
    /// *HostSpecification.changeID*.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### start_change_id
    /// The beginning of the time period.
    ///
    /// ### end_change_id
    /// The ending of the time period.
    ///
    /// ## Returns:
    ///
    /// The queried host list.
    /// 
    /// Refers instances of *HostSystem*.
    pub async fn host_spec_get_updated_hosts(&self, start_change_id: Option<&str>, end_change_id: Option<&str>) -> Result<Option<Vec<ManagedObjectReference>>> {
        let input = HostSpecGetUpdatedHostsRequestType {start_change_id, end_change_id, };
        let path = format!("/HostSpecificationManager/{moId}/HostSpecGetUpdatedHosts", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Retrieve the host specification.
    /// 
    /// When the parameter <code>fromHost</code>
    /// is <code>true</code>, the host specification is retrieved from the host;
    /// otherwise, it is from the host specification "database" for this manager.
    /// When retrieved from host, the copy in host specification "database" will
    /// be updated. On success, it will fire a
    /// <code>HostSpecificationChangedEvent</code>.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// The specified host whose host specification will be retrieved.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### from_host
    /// Whether retrieve from the host.
    ///
    /// ## Returns:
    ///
    /// The host specification of the specified host.
    ///
    /// ## Errors:
    ///
    /// ***HostSpecificationOperationFailed***: If the method fails when retrieve
    /// from host.
    pub async fn retrieve_host_specification(&self, host: &ManagedObjectReference, from_host: bool) -> Result<HostSpecification> {
        let input = RetrieveHostSpecificationRequestType {host, from_host, };
        let path = format!("/HostSpecificationManager/{moId}/RetrieveHostSpecification", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Update the host specification with the provided copy.
    /// 
    /// If there is no host specification for the host, create the host
    /// specification for this host in the host specification "database";
    /// otherwise, update the host specification with the provided.
    /// *HostSpecification* object.
    /// On success, it will fire a <code>HostSpecificationChangedEvent</code>.
    /// 
    /// ***Required privileges:*** Profile.Edit
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// The host whose specification will be updated.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### host_spec
    /// The new host specification to be updated with.
    ///
    /// ## Errors:
    ///
    /// ***HostSpecificationOperationFailed***: If the method fails.
    pub async fn update_host_specification(&self, host: &ManagedObjectReference, host_spec: &HostSpecification) -> Result<()> {
        let input = UpdateHostSpecificationRequestType {host, host_spec, };
        let path = format!("/HostSpecificationManager/{moId}/UpdateHostSpecification", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Update the host specification with the provided host sub specification.
    /// 
    /// If there is no host specification for the host, create the host
    /// specification, which contains only the provided host sub specification,
    /// for this host; otherwise, add or update the host specification with the
    /// provided *HostSubSpecification* object.
    /// This method provides a way to incrementally build the host specification.
    /// On success, it will fire a <code>HostSpecificationChangedEvent</code>.
    /// 
    /// ***Required privileges:*** Profile.Edit
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// The host whose specification will be updated.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### host_sub_spec
    /// The data object for the new host sub specification.
    ///
    /// ## Errors:
    ///
    /// ***HostSpecificationOperationFailed***: If the method fails.
    pub async fn update_host_sub_specification(&self, host: &ManagedObjectReference, host_sub_spec: &HostSubSpecification) -> Result<()> {
        let input = UpdateHostSubSpecificationRequestType {host, host_sub_spec, };
        let path = format!("/HostSpecificationManager/{moId}/UpdateHostSubSpecification", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DeleteHostSpecificationRequestType<'a> {
    host: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DeleteHostSubSpecificationRequestType<'a> {
    host: &'a ManagedObjectReference,
    #[serde(rename = "subSpecName")]
    sub_spec_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HostSpecGetUpdatedHostsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startChangeID")]
    start_change_id: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endChangeID")]
    end_change_id: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveHostSpecificationRequestType<'a> {
    host: &'a ManagedObjectReference,
    #[serde(rename = "fromHost")]
    from_host: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateHostSpecificationRequestType<'a> {
    host: &'a ManagedObjectReference,
    #[serde(rename = "hostSpec")]
    host_spec: &'a HostSpecification,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateHostSubSpecificationRequestType<'a> {
    host: &'a ManagedObjectReference,
    #[serde(rename = "hostSubSpec")]
    host_sub_spec: &'a HostSubSpecification,
}
