use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::ManagedObjectReference;
/// A singleton managed object for managing vCenter tenants.
pub struct TenantTenantManager {
    client: Arc<Client>,
    mo_id: String,
}
impl TenantTenantManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Define a set of ManagedEntity objects as used for tenant management.
    /// 
    /// Those entities are a starting point of an inventory hierarchy
    /// (sub-tree) that functionally exists in the tenant's inventory but are
    /// owned by the system user of the vCenter Server.
    /// The operations which the tenant may perform on these objects depend
    /// on the permissions granted to the tenant by the SaaS provisioning
    /// layer.
    /// Permissions that the tenant may create on the parent objects of the
    /// management entities do not propagate to the hierarchies of
    /// management entities and thus have no effect on them.
    /// This operation will fail for all the entities if any of them does
    /// not exist.
    /// The method behaviour is transactional - either all entities are
    /// marked or none if an error occurs while processing them.
    /// The user calling this method should hold TenantManager.Update on the
    /// root folder and TenantManager.Update on each entity currently
    /// being marked as a service provider one. These are strict privilege
    /// requirements allowing only administrators to call the method.
    /// 
    /// ***Required privileges:*** TenantManager.Update
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// an array of management entities.
    /// 
    /// ***Required privileges:*** TenantManager.Update
    /// 
    /// Refers instances of *ManagedEntity*.
    ///
    /// ## Errors:
    ///
    /// ***ManagedObjectNotFound***: if any of the entities doesn't exist.
    /// 
    /// ***AuthMinimumAdminPermission***: if this change will leave the
    /// system with no Administrator permission on the root folder
    /// of the service provider inventory.
    pub async fn mark_service_provider_entities(&self, entity: Option<&[ManagedObjectReference]>) -> Result<()> {
        let input = MarkServiceProviderEntitiesRequestType {entity, };
        let path = format!("/TenantTenantManager/{moId}/MarkServiceProviderEntities", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Retrieves the list of tenant management entities.
    /// 
    /// ***Required privileges:*** TenantManager.Query
    ///
    /// ## Returns:
    ///
    /// the array of tenant management resources.
    /// 
    /// Refers instances of *ManagedEntity*.
    pub async fn retrieve_service_provider_entities(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/TenantTenantManager/{moId}/RetrieveServiceProviderEntities", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Resets the management type of an array of ManagedEntity objects.
    /// 
    /// This operation will fail if any of the entities does not exist.
    /// The method behaviour is transactional - either all entities are
    /// unmarked or none if an error occurs while processing them.
    /// The user calling this method should hold TenantManager.Update on the
    /// root folder and TenantManager.Update on each entity currently
    /// being unmarked as a service provider one. These are strict privilege
    /// requirements allowing only administrators to call the method.
    /// 
    /// ***Required privileges:*** TenantManager.Update
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// an array of management entities.
    /// 
    /// ***Required privileges:*** TenantManager.Update
    /// 
    /// Refers instances of *ManagedEntity*.
    ///
    /// ## Errors:
    ///
    /// ***ManagedObjectNotFound***: if any of the entities doesn't exist.
    pub async fn unmark_service_provider_entities(&self, entity: Option<&[ManagedObjectReference]>) -> Result<()> {
        let input = UnmarkServiceProviderEntitiesRequestType {entity, };
        let path = format!("/TenantTenantManager/{moId}/UnmarkServiceProviderEntities", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct MarkServiceProviderEntitiesRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entity: Option<&'a [ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UnmarkServiceProviderEntitiesRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entity: Option<&'a [ManagedObjectReference]>,
}
