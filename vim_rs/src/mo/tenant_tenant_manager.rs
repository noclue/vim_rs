use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// A singleton managed object for managing vCenter tenants.
#[derive(Clone)]
pub struct TenantTenantManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl TenantTenantManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn mark_service_provider_entities(&self, entity: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<()> {
        let input = MarkServiceProviderEntitiesRequestType {entity, };
        self.client.invoke_void("", "TenantTenantManager", &self.mo_id, "MarkServiceProviderEntities", Some(&input)).await
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
    pub async fn retrieve_service_provider_entities(&self) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let bytes_opt = self.client.invoke_optional("", "TenantTenantManager", &self.mo_id, "RetrieveServiceProviderEntities", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn unmark_service_provider_entities(&self, entity: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<()> {
        let input = UnmarkServiceProviderEntitiesRequestType {entity, };
        self.client.invoke_void("", "TenantTenantManager", &self.mo_id, "UnmarkServiceProviderEntities", Some(&input)).await
    }
}
struct MarkServiceProviderEntitiesRequestType<'a> {
    entity: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for MarkServiceProviderEntitiesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(MarkServiceProviderEntitiesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct MarkServiceProviderEntitiesRequestTypeSer<'b, 'a> {
    data: &'b MarkServiceProviderEntitiesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for MarkServiceProviderEntitiesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"MarkServiceProviderEntitiesRequestType")),
                1 => {
                    let Some(ref val) = self.data.entity else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entity"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct UnmarkServiceProviderEntitiesRequestType<'a> {
    entity: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for UnmarkServiceProviderEntitiesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UnmarkServiceProviderEntitiesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UnmarkServiceProviderEntitiesRequestTypeSer<'b, 'a> {
    data: &'b UnmarkServiceProviderEntitiesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UnmarkServiceProviderEntitiesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UnmarkServiceProviderEntitiesRequestType")),
                1 => {
                    let Some(ref val) = self.data.entity else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entity"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
