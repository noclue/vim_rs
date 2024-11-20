use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::ManagedObjectReference;
use crate::types::HealthUpdateInfo;
use crate::types::HealthUpdate;
pub struct HealthUpdateManager {
    client: Arc<VimClient>,
    mo_id: String,
}
impl HealthUpdateManager {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Adds health update filters.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    ///
    /// ## Parameters:
    ///
    /// ### provider_id
    /// The provider identifier.
    ///
    /// ### filter_name
    /// The filter name.
    ///
    /// ### info_ids
    /// The list of HealthUpdateInfo IDs that should be
    /// filtered.
    ///
    /// ## Returns:
    ///
    /// The filter identifier.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If no provider with this id is registered.
    /// 
    /// ***InvalidArgument***: - If filter name exceeds the maximum length
    /// limit of 56 characters.
    /// \- If a filter with this name already exists for
    /// this provider.
    /// \- If infoIds list contains a HealthUpdateInfo id
    /// which is not associated with the specified
    /// provider.
    /// \- If there are duplicate HealthUpdateInfo ids
    /// in the infoIds list.
    pub async fn add_filter(&self, provider_id: &str, filter_name: &str, info_ids: Option<&[String]>) -> Result<String> {
        let input = AddFilterRequestType {provider_id, filter_name, info_ids, };
        let path = format!("/HealthUpdateManager/{moId}/AddFilter", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Add entities on which this filter is configured.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    ///
    /// ## Parameters:
    ///
    /// ### filter_id
    /// The filter id.
    ///
    /// ### entities
    /// The list of additional managed entities. Only
    /// entities of type HostSystem or
    /// ClusterComputeResource are valid.
    /// 
    /// Refers instances of *ManagedEntity*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If no filter with this id is registered.
    /// 
    /// ***InvalidArgument***: - If any of the entities is already associated
    /// with the specified filter.
    /// \- If there are duplicate entities in the given
    /// entities list.
    /// \- If the entities list contains an entity of
    /// type other than HostSystem and
    /// ClusterComputeResource.
    pub async fn add_filter_entities(&self, filter_id: &str, entities: Option<&[ManagedObjectReference]>) -> Result<()> {
        let input = AddFilterEntitiesRequestType {filter_id, entities, };
        let path = format!("/HealthUpdateManager/{moId}/AddFilterEntities", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// The provider monitors additional managed entities.
    /// 
    /// A particular
    /// managed entity can be monitored by multiple providers.
    /// 
    /// ***Required privileges:*** HealthUpdateProvider.Update
    ///
    /// ## Parameters:
    ///
    /// ### provider_id
    /// The provider id.
    ///
    /// ### entities
    /// The entities that are newly monitored by this
    /// provider.
    /// 
    /// Refers instances of *ManagedEntity*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If no provider with this id is registered.
    /// 
    /// ***NotSupported***: If the http session user does not match the user
    /// who registered the provider, or if the http
    /// session cannot be retrieved.
    /// 
    /// ***InvalidArgument***: - If any of the entities is not of type
    /// HostSystem.
    /// \- If there are duplicate entities in the given
    /// entities list.
    /// \- If any of the entities is already monitored by
    /// the specified provider.
    pub async fn add_monitored_entities(&self, provider_id: &str, entities: Option<&[ManagedObjectReference]>) -> Result<()> {
        let input = AddMonitoredEntitiesRequestType {provider_id, entities, };
        let path = format!("/HealthUpdateManager/{moId}/AddMonitoredEntities", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Check if the managed entity is monitored by the provider.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### provider_id
    /// The provider id.
    ///
    /// ### entity
    /// An entity of type HostSystem.
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ## Returns:
    ///
    /// True iff the entity is monitored by this
    /// provider.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If no provider with this id is registered.
    /// 
    /// ***InvalidArgument***: If the specified entity is not of type
    /// HostSystem.
    pub async fn has_monitored_entity(&self, provider_id: &str, entity: &ManagedObjectReference) -> Result<bool> {
        let input = HasMonitoredEntityRequestType {provider_id, entity, };
        let path = format!("/HealthUpdateManager/{moId}/HasMonitoredEntity", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Verifies if the given provider is registered.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// The provider id.
    ///
    /// ## Returns:
    ///
    /// True iff the provider is registered.
    pub async fn has_provider(&self, id: &str) -> Result<bool> {
        let input = HasProviderRequestType {id, };
        let path = format!("/HealthUpdateManager/{moId}/HasProvider", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Report a change in health status.
    /// 
    /// ***Required privileges:*** HealthUpdateProvider.Update
    ///
    /// ## Parameters:
    ///
    /// ### provider_id
    /// The provider id.
    ///
    /// ### updates
    /// The changes in health states.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If no provider with this id is registered.
    /// 
    /// ***NotSupported***: If the http session user does not match the user
    /// who registered the provider, or if the http
    /// session cannot be retrieved.
    /// 
    /// ***InvalidArgument***: - If an unknown HealthUpdate id is given.
    /// \- If updates list contains a HealthUpdate for a
    /// host which is not monitored by the specified
    /// provider.
    /// \- If updates list contains multiple
    /// HealthUpdates with the same id.
    /// \- If an existing HealthUpdate id is used in the
    /// given updates.
    /// \- If there is a HealthUpdate with green status
    /// and non-empty remediation.
    /// \- If there is a HealthUpdate with gray status.
    pub async fn post_health_updates(&self, provider_id: &str, updates: Option<&[HealthUpdate]>) -> Result<()> {
        let input = PostHealthUpdatesRequestType {provider_id, updates, };
        let path = format!("/HealthUpdateManager/{moId}/PostHealthUpdates", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Returns the list of entities on which this filter is configured.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### filter_id
    /// The filter id.
    ///
    /// ## Returns:
    ///
    /// The list of managed entities.
    /// 
    /// Refers instances of *ManagedEntity*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If no filter with this id is registered.
    pub async fn query_filter_entities(&self, filter_id: &str) -> Result<Option<Vec<ManagedObjectReference>>> {
        let input = QueryFilterEntitiesRequestType {filter_id, };
        let path = format!("/HealthUpdateManager/{moId}/QueryFilterEntities", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Returns the list of HealthUpdateInfos configured for this filter.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### filter_id
    /// The filter id.
    ///
    /// ## Returns:
    ///
    /// The list of identifiers of the configured
    /// HealthUpdateInfos.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If no filter with this id is registered.
    pub async fn query_filter_info_ids(&self, filter_id: &str) -> Result<Option<Vec<String>>> {
        let input = QueryFilterInfoIdsRequestType {filter_id, };
        let path = format!("/HealthUpdateManager/{moId}/QueryFilterInfoIds", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Returns the list of filters.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### provider_id
    /// The provider id.
    ///
    /// ## Returns:
    ///
    /// The list of filters identifiers.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If no provider with this id is registered.
    pub async fn query_filter_list(&self, provider_id: &str) -> Result<Option<Vec<String>>> {
        let input = QueryFilterListRequestType {provider_id, };
        let path = format!("/HealthUpdateManager/{moId}/QueryFilterList", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Returns the filter name.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### filter_id
    /// The filter id.
    ///
    /// ## Returns:
    ///
    /// The name of the filter.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If no filter with this id is registered.
    pub async fn query_filter_name(&self, filter_id: &str) -> Result<String> {
        let input = QueryFilterNameRequestType {filter_id, };
        let path = format!("/HealthUpdateManager/{moId}/QueryFilterName", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Returns the list of HealthUpdateInfo configured for the given provider.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### provider_id
    /// The provider id.
    ///
    /// ## Returns:
    ///
    /// The list of configured HealthUpdateInfo.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If no provider with this id is registered.
    pub async fn query_health_update_infos(&self, provider_id: &str) -> Result<Option<Vec<HealthUpdateInfo>>> {
        let input = QueryHealthUpdateInfosRequestType {provider_id, };
        let path = format!("/HealthUpdateManager/{moId}/QueryHealthUpdateInfos", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Returns the list of health updates reported by the given provider.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### provider_id
    /// The provider id.
    ///
    /// ## Returns:
    ///
    /// The list of current health updates by this provider.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If no provider with this id is registered.
    pub async fn query_health_updates(&self, provider_id: &str) -> Result<Option<Vec<HealthUpdate>>> {
        let input = QueryHealthUpdatesRequestType {provider_id, };
        let path = format!("/HealthUpdateManager/{moId}/QueryHealthUpdates", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Returns the list of managed entities monitored by the given provider.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### provider_id
    /// The provider id.
    ///
    /// ## Returns:
    ///
    /// The list of monitored managed entities.
    /// 
    /// Refers instances of *ManagedEntity*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If no provider with this id is registered.
    pub async fn query_monitored_entities(&self, provider_id: &str) -> Result<Option<Vec<ManagedObjectReference>>> {
        let input = QueryMonitoredEntitiesRequestType {provider_id, };
        let path = format!("/HealthUpdateManager/{moId}/QueryMonitoredEntities", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// The providers.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// The list of identifiers of registered providers.
    pub async fn query_provider_list(&self) -> Result<Option<Vec<String>>> {
        let path = format!("/HealthUpdateManager/{moId}/QueryProviderList", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Query the name of the provider.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// -
    ///
    /// ## Returns:
    ///
    /// The name of the provider.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If no provider with this id is registered.
    pub async fn query_provider_name(&self, id: &str) -> Result<String> {
        let input = QueryProviderNameRequestType {id, };
        let path = format!("/HealthUpdateManager/{moId}/QueryProviderName", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// The set of hosts that are in the cluster, but not monitored by
    /// the provider.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### provider_id
    /// The provider id.
    ///
    /// ### cluster
    /// The cluster.
    /// 
    /// Refers instance of *ClusterComputeResource*.
    ///
    /// ## Returns:
    ///
    /// The hosts in the cluster that are not monitored by
    /// the provider.
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If no provider with this id is registered.
    pub async fn query_unmonitored_hosts(&self, provider_id: &str, cluster: &ManagedObjectReference) -> Result<Option<Vec<ManagedObjectReference>>> {
        let input = QueryUnmonitoredHostsRequestType {provider_id, cluster, };
        let path = format!("/HealthUpdateManager/{moId}/QueryUnmonitoredHosts", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Registers provider.
    /// 
    /// ***Required privileges:*** HealthUpdateProvider.Register
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// The provider name. Should follow Java package
    /// naming convention to minimize name clashes with
    /// currently registered providers.
    /// For example, "com.vmware.HealthUpdateProvider".
    ///
    /// ### health_update_info
    /// The list of healthUpdateInfo that can be
    /// reported in healthUpdates.
    ///
    /// ## Returns:
    ///
    /// The identifier for the registered provider.
    pub async fn register_health_update_provider(&self, name: &str, health_update_info: Option<&[HealthUpdateInfo]>) -> Result<String> {
        let input = RegisterHealthUpdateProviderRequestType {name, health_update_info, };
        let path = format!("/HealthUpdateManager/{moId}/RegisterHealthUpdateProvider", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Removes the specified filter.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    ///
    /// ## Parameters:
    ///
    /// ### filter_id
    /// The filter id.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If no filter with this id is registered.
    pub async fn remove_filter(&self, filter_id: &str) -> Result<()> {
        let input = RemoveFilterRequestType {filter_id, };
        let path = format!("/HealthUpdateManager/{moId}/RemoveFilter", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Remove entities on which this filter is configured.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    ///
    /// ## Parameters:
    ///
    /// ### filter_id
    /// The filter id.
    ///
    /// ### entities
    /// The list of removed managed entities.
    /// 
    /// Refers instances of *ManagedEntity*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If no filter with this id is registered.
    /// 
    /// ***InvalidArgument***: - If there are duplicate managed entities in the
    /// given entities list.
    /// \- If there is a managed entity of type other
    /// than HostSystem and ClusterComputeResource.
    /// \- If the entities list contains an entity which
    /// is not associated with the specified filter.
    pub async fn remove_filter_entities(&self, filter_id: &str, entities: Option<&[ManagedObjectReference]>) -> Result<()> {
        let input = RemoveFilterEntitiesRequestType {filter_id, entities, };
        let path = format!("/HealthUpdateManager/{moId}/RemoveFilterEntities", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// The provider monitors fewer managed entities.
    /// 
    /// ***Required privileges:*** HealthUpdateProvider.Update
    ///
    /// ## Parameters:
    ///
    /// ### provider_id
    /// The provider id.
    ///
    /// ### entities
    /// The entities that are no longer monitored by
    /// this provider.
    /// 
    /// Refers instances of *ManagedEntity*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If no provider with this id is registered.
    /// 
    /// ***InvalidState***: If any of the entities is a part of an
    /// InfraUpdateHa cluster that has the provider
    /// enabled.
    /// 
    /// ***NotSupported***: If the http session user does not match the user
    /// who registered the provider, or if the http
    /// session cannot be retrieved.
    /// 
    /// ***InvalidArgument***: - If any of the specified entities is not of
    /// type HostSystem.
    /// \- If there are duplicate entities in the given
    /// entities list.
    /// \- If any of the entities is already not being
    /// monitored by the specified provider.
    pub async fn remove_monitored_entities(&self, provider_id: &str, entities: Option<&[ManagedObjectReference]>) -> Result<()> {
        let input = RemoveMonitoredEntitiesRequestType {provider_id, entities, };
        let path = format!("/HealthUpdateManager/{moId}/RemoveMonitoredEntities", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Unregisters the specified provider, if it exists.
    /// 
    /// A VirtualCenter
    /// Server restart implicitly unregisters all providers.
    /// 
    /// ***Required privileges:*** HealthUpdateProvider.Unregister
    ///
    /// ## Parameters:
    ///
    /// ### provider_id
    /// The provider id.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If the specified provider is not registered.
    /// 
    /// ***InvalidState***: If the specified provider is still used in an
    /// InfraUpdateHa cluster.
    pub async fn unregister_health_update_provider(&self, provider_id: &str) -> Result<()> {
        let input = UnregisterHealthUpdateProviderRequestType {provider_id, };
        let path = format!("/HealthUpdateManager/{moId}/UnregisterHealthUpdateProvider", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddFilterRequestType<'a> {
    #[serde(rename = "providerId")]
    provider_id: &'a str,
    #[serde(rename = "filterName")]
    filter_name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "infoIds")]
    info_ids: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddFilterEntitiesRequestType<'a> {
    #[serde(rename = "filterId")]
    filter_id: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entities: Option<&'a [ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddMonitoredEntitiesRequestType<'a> {
    #[serde(rename = "providerId")]
    provider_id: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entities: Option<&'a [ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HasMonitoredEntityRequestType<'a> {
    #[serde(rename = "providerId")]
    provider_id: &'a str,
    entity: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HasProviderRequestType<'a> {
    id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct PostHealthUpdatesRequestType<'a> {
    #[serde(rename = "providerId")]
    provider_id: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    updates: Option<&'a [HealthUpdate]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryFilterEntitiesRequestType<'a> {
    #[serde(rename = "filterId")]
    filter_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryFilterInfoIdsRequestType<'a> {
    #[serde(rename = "filterId")]
    filter_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryFilterListRequestType<'a> {
    #[serde(rename = "providerId")]
    provider_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryFilterNameRequestType<'a> {
    #[serde(rename = "filterId")]
    filter_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryHealthUpdateInfosRequestType<'a> {
    #[serde(rename = "providerId")]
    provider_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryHealthUpdatesRequestType<'a> {
    #[serde(rename = "providerId")]
    provider_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryMonitoredEntitiesRequestType<'a> {
    #[serde(rename = "providerId")]
    provider_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryProviderNameRequestType<'a> {
    id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryUnmonitoredHostsRequestType<'a> {
    #[serde(rename = "providerId")]
    provider_id: &'a str,
    cluster: &'a ManagedObjectReference,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RegisterHealthUpdateProviderRequestType<'a> {
    name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "healthUpdateInfo")]
    health_update_info: Option<&'a [HealthUpdateInfo]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveFilterRequestType<'a> {
    #[serde(rename = "filterId")]
    filter_id: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveFilterEntitiesRequestType<'a> {
    #[serde(rename = "filterId")]
    filter_id: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entities: Option<&'a [ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveMonitoredEntitiesRequestType<'a> {
    #[serde(rename = "providerId")]
    provider_id: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entities: Option<&'a [ManagedObjectReference]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UnregisterHealthUpdateProviderRequestType<'a> {
    #[serde(rename = "providerId")]
    provider_id: &'a str,
}
