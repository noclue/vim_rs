use std::sync::Arc;
use crate::core::client::{VimClient, Result};
#[derive(Clone)]
pub struct HealthUpdateManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HealthUpdateManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
        let bytes = self.client.invoke("", "HealthUpdateManager", &self.mo_id, "AddFilter", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn add_filter_entities(&self, filter_id: &str, entities: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<()> {
        let input = AddFilterEntitiesRequestType {filter_id, entities, };
        self.client.invoke_void("", "HealthUpdateManager", &self.mo_id, "AddFilterEntities", Some(&input)).await
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
    pub async fn add_monitored_entities(&self, provider_id: &str, entities: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<()> {
        let input = AddMonitoredEntitiesRequestType {provider_id, entities, };
        self.client.invoke_void("", "HealthUpdateManager", &self.mo_id, "AddMonitoredEntities", Some(&input)).await
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
    pub async fn has_monitored_entity(&self, provider_id: &str, entity: &crate::types::structs::ManagedObjectReference) -> Result<bool> {
        let input = HasMonitoredEntityRequestType {provider_id, entity, };
        let bytes = self.client.invoke("", "HealthUpdateManager", &self.mo_id, "HasMonitoredEntity", Some(&input)).await?;
        let result: bool = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        let bytes = self.client.invoke("", "HealthUpdateManager", &self.mo_id, "HasProvider", Some(&input)).await?;
        let result: bool = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn post_health_updates(&self, provider_id: &str, updates: Option<&[crate::types::structs::HealthUpdate]>) -> Result<()> {
        let input = PostHealthUpdatesRequestType {provider_id, updates, };
        self.client.invoke_void("", "HealthUpdateManager", &self.mo_id, "PostHealthUpdates", Some(&input)).await
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
    pub async fn query_filter_entities(&self, filter_id: &str) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let input = QueryFilterEntitiesRequestType {filter_id, };
        let bytes_opt = self.client.invoke_optional("", "HealthUpdateManager", &self.mo_id, "QueryFilterEntities", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
        let bytes_opt = self.client.invoke_optional("", "HealthUpdateManager", &self.mo_id, "QueryFilterInfoIds", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
        let bytes_opt = self.client.invoke_optional("", "HealthUpdateManager", &self.mo_id, "QueryFilterList", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
        let bytes = self.client.invoke("", "HealthUpdateManager", &self.mo_id, "QueryFilterName", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn query_health_update_infos(&self, provider_id: &str) -> Result<Option<Vec<crate::types::structs::HealthUpdateInfo>>> {
        let input = QueryHealthUpdateInfosRequestType {provider_id, };
        let bytes_opt = self.client.invoke_optional("", "HealthUpdateManager", &self.mo_id, "QueryHealthUpdateInfos", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn query_health_updates(&self, provider_id: &str) -> Result<Option<Vec<crate::types::structs::HealthUpdate>>> {
        let input = QueryHealthUpdatesRequestType {provider_id, };
        let bytes_opt = self.client.invoke_optional("", "HealthUpdateManager", &self.mo_id, "QueryHealthUpdates", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn query_monitored_entities(&self, provider_id: &str) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let input = QueryMonitoredEntitiesRequestType {provider_id, };
        let bytes_opt = self.client.invoke_optional("", "HealthUpdateManager", &self.mo_id, "QueryMonitoredEntities", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// The providers.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// The list of identifiers of registered providers.
    pub async fn query_provider_list(&self) -> Result<Option<Vec<String>>> {
        let bytes_opt = self.client.invoke_optional("", "HealthUpdateManager", &self.mo_id, "QueryProviderList", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
        let bytes = self.client.invoke("", "HealthUpdateManager", &self.mo_id, "QueryProviderName", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn query_unmonitored_hosts(&self, provider_id: &str, cluster: &crate::types::structs::ManagedObjectReference) -> Result<Option<Vec<crate::types::structs::ManagedObjectReference>>> {
        let input = QueryUnmonitoredHostsRequestType {provider_id, cluster, };
        let bytes_opt = self.client.invoke_optional("", "HealthUpdateManager", &self.mo_id, "QueryUnmonitoredHosts", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn register_health_update_provider(&self, name: &str, health_update_info: Option<&[crate::types::structs::HealthUpdateInfo]>) -> Result<String> {
        let input = RegisterHealthUpdateProviderRequestType {name, health_update_info, };
        let bytes = self.client.invoke("", "HealthUpdateManager", &self.mo_id, "RegisterHealthUpdateProvider", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
        self.client.invoke_void("", "HealthUpdateManager", &self.mo_id, "RemoveFilter", Some(&input)).await
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
    pub async fn remove_filter_entities(&self, filter_id: &str, entities: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<()> {
        let input = RemoveFilterEntitiesRequestType {filter_id, entities, };
        self.client.invoke_void("", "HealthUpdateManager", &self.mo_id, "RemoveFilterEntities", Some(&input)).await
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
    pub async fn remove_monitored_entities(&self, provider_id: &str, entities: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<()> {
        let input = RemoveMonitoredEntitiesRequestType {provider_id, entities, };
        self.client.invoke_void("", "HealthUpdateManager", &self.mo_id, "RemoveMonitoredEntities", Some(&input)).await
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
        self.client.invoke_void("", "HealthUpdateManager", &self.mo_id, "UnregisterHealthUpdateProvider", Some(&input)).await
    }
}
struct AddFilterRequestType<'a> {
    provider_id: &'a str,
    filter_name: &'a str,
    info_ids: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for AddFilterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AddFilterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AddFilterRequestTypeSer<'b, 'a> {
    data: &'b AddFilterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AddFilterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AddFilterRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("providerId"), &self.data.provider_id as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("filterName"), &self.data.filter_name as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.info_ids else { continue; };
                    return Some((std::borrow::Cow::Borrowed("infoIds"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct AddFilterEntitiesRequestType<'a> {
    filter_id: &'a str,
    entities: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for AddFilterEntitiesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AddFilterEntitiesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AddFilterEntitiesRequestTypeSer<'b, 'a> {
    data: &'b AddFilterEntitiesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AddFilterEntitiesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AddFilterEntitiesRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("filterId"), &self.data.filter_id as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.entities else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entities"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct AddMonitoredEntitiesRequestType<'a> {
    provider_id: &'a str,
    entities: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for AddMonitoredEntitiesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AddMonitoredEntitiesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AddMonitoredEntitiesRequestTypeSer<'b, 'a> {
    data: &'b AddMonitoredEntitiesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AddMonitoredEntitiesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AddMonitoredEntitiesRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("providerId"), &self.data.provider_id as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.entities else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entities"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct HasMonitoredEntityRequestType<'a> {
    provider_id: &'a str,
    entity: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for HasMonitoredEntityRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HasMonitoredEntityRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HasMonitoredEntityRequestTypeSer<'b, 'a> {
    data: &'b HasMonitoredEntityRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HasMonitoredEntityRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HasMonitoredEntityRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("providerId"), &self.data.provider_id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct HasProviderRequestType<'a> {
    id: &'a str,
}

impl<'a> miniserde::Serialize for HasProviderRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(HasProviderRequestTypeSer { data: self, seq: 0 }))
    }
}

struct HasProviderRequestTypeSer<'b, 'a> {
    data: &'b HasProviderRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for HasProviderRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"HasProviderRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct PostHealthUpdatesRequestType<'a> {
    provider_id: &'a str,
    updates: Option<&'a [crate::types::structs::HealthUpdate]>,
}

impl<'a> miniserde::Serialize for PostHealthUpdatesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PostHealthUpdatesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PostHealthUpdatesRequestTypeSer<'b, 'a> {
    data: &'b PostHealthUpdatesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for PostHealthUpdatesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"PostHealthUpdatesRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("providerId"), &self.data.provider_id as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.updates else { continue; };
                    return Some((std::borrow::Cow::Borrowed("updates"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryFilterEntitiesRequestType<'a> {
    filter_id: &'a str,
}

impl<'a> miniserde::Serialize for QueryFilterEntitiesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryFilterEntitiesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryFilterEntitiesRequestTypeSer<'b, 'a> {
    data: &'b QueryFilterEntitiesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryFilterEntitiesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryFilterEntitiesRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("filterId"), &self.data.filter_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryFilterInfoIdsRequestType<'a> {
    filter_id: &'a str,
}

impl<'a> miniserde::Serialize for QueryFilterInfoIdsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryFilterInfoIdsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryFilterInfoIdsRequestTypeSer<'b, 'a> {
    data: &'b QueryFilterInfoIdsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryFilterInfoIdsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryFilterInfoIdsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("filterId"), &self.data.filter_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryFilterListRequestType<'a> {
    provider_id: &'a str,
}

impl<'a> miniserde::Serialize for QueryFilterListRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryFilterListRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryFilterListRequestTypeSer<'b, 'a> {
    data: &'b QueryFilterListRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryFilterListRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryFilterListRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("providerId"), &self.data.provider_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryFilterNameRequestType<'a> {
    filter_id: &'a str,
}

impl<'a> miniserde::Serialize for QueryFilterNameRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryFilterNameRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryFilterNameRequestTypeSer<'b, 'a> {
    data: &'b QueryFilterNameRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryFilterNameRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryFilterNameRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("filterId"), &self.data.filter_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryHealthUpdateInfosRequestType<'a> {
    provider_id: &'a str,
}

impl<'a> miniserde::Serialize for QueryHealthUpdateInfosRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryHealthUpdateInfosRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryHealthUpdateInfosRequestTypeSer<'b, 'a> {
    data: &'b QueryHealthUpdateInfosRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryHealthUpdateInfosRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryHealthUpdateInfosRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("providerId"), &self.data.provider_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryHealthUpdatesRequestType<'a> {
    provider_id: &'a str,
}

impl<'a> miniserde::Serialize for QueryHealthUpdatesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryHealthUpdatesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryHealthUpdatesRequestTypeSer<'b, 'a> {
    data: &'b QueryHealthUpdatesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryHealthUpdatesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryHealthUpdatesRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("providerId"), &self.data.provider_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryMonitoredEntitiesRequestType<'a> {
    provider_id: &'a str,
}

impl<'a> miniserde::Serialize for QueryMonitoredEntitiesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryMonitoredEntitiesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryMonitoredEntitiesRequestTypeSer<'b, 'a> {
    data: &'b QueryMonitoredEntitiesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryMonitoredEntitiesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryMonitoredEntitiesRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("providerId"), &self.data.provider_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryProviderNameRequestType<'a> {
    id: &'a str,
}

impl<'a> miniserde::Serialize for QueryProviderNameRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryProviderNameRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryProviderNameRequestTypeSer<'b, 'a> {
    data: &'b QueryProviderNameRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryProviderNameRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryProviderNameRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryUnmonitoredHostsRequestType<'a> {
    provider_id: &'a str,
    cluster: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for QueryUnmonitoredHostsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryUnmonitoredHostsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryUnmonitoredHostsRequestTypeSer<'b, 'a> {
    data: &'b QueryUnmonitoredHostsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryUnmonitoredHostsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryUnmonitoredHostsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("providerId"), &self.data.provider_id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RegisterHealthUpdateProviderRequestType<'a> {
    name: &'a str,
    health_update_info: Option<&'a [crate::types::structs::HealthUpdateInfo]>,
}

impl<'a> miniserde::Serialize for RegisterHealthUpdateProviderRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RegisterHealthUpdateProviderRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RegisterHealthUpdateProviderRequestTypeSer<'b, 'a> {
    data: &'b RegisterHealthUpdateProviderRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RegisterHealthUpdateProviderRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RegisterHealthUpdateProviderRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.health_update_info else { continue; };
                    return Some((std::borrow::Cow::Borrowed("healthUpdateInfo"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RemoveFilterRequestType<'a> {
    filter_id: &'a str,
}

impl<'a> miniserde::Serialize for RemoveFilterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveFilterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveFilterRequestTypeSer<'b, 'a> {
    data: &'b RemoveFilterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveFilterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveFilterRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("filterId"), &self.data.filter_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RemoveFilterEntitiesRequestType<'a> {
    filter_id: &'a str,
    entities: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for RemoveFilterEntitiesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveFilterEntitiesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveFilterEntitiesRequestTypeSer<'b, 'a> {
    data: &'b RemoveFilterEntitiesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveFilterEntitiesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveFilterEntitiesRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("filterId"), &self.data.filter_id as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.entities else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entities"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RemoveMonitoredEntitiesRequestType<'a> {
    provider_id: &'a str,
    entities: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for RemoveMonitoredEntitiesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveMonitoredEntitiesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveMonitoredEntitiesRequestTypeSer<'b, 'a> {
    data: &'b RemoveMonitoredEntitiesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveMonitoredEntitiesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveMonitoredEntitiesRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("providerId"), &self.data.provider_id as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.entities else { continue; };
                    return Some((std::borrow::Cow::Borrowed("entities"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct UnregisterHealthUpdateProviderRequestType<'a> {
    provider_id: &'a str,
}

impl<'a> miniserde::Serialize for UnregisterHealthUpdateProviderRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UnregisterHealthUpdateProviderRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UnregisterHealthUpdateProviderRequestTypeSer<'b, 'a> {
    data: &'b UnregisterHealthUpdateProviderRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UnregisterHealthUpdateProviderRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UnregisterHealthUpdateProviderRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("providerId"), &self.data.provider_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
