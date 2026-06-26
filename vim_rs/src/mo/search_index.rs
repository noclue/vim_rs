use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The SearchIndex service allows a client to efficiently query the
/// inventory for a specific managed entity by attributes such as UUID, IP address, DNS
/// name, or datastore path.
/// 
/// Such searches typically return a VirtualMachine or a
/// HostSystem. While searching, only objects for which the user has sufficient
/// privileges are considered. The findByInventoryPath and findChild operations only
/// search on entities for which the user has view privileges; all other SearchIndex
/// find operations only search virtual machines and hosts for which the user has
/// read privileges. If the user does not have sufficient privileges for an object that
/// matches the search criteria, that object is not returned.
#[derive(Clone)]
pub struct SearchIndex {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl SearchIndex {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Finds all virtual machines or hosts by DNS name.
    /// 
    /// The DNS name for a virtual
    /// machine is the one returned from VMware tools, *GuestInfo.hostName*.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### datacenter
    /// If specified, restricts the query to entities in
    /// a particular datacenter. If not specified, the entire inventory is
    /// searched.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### dns_name
    /// The fully qualified domain name to find.
    ///
    /// ### vm_search
    /// If true, search for virtual machines, otherwise search for
    /// hosts.
    ///
    /// ## Returns:
    ///
    /// The list of all virtual machines or hosts that are found. If no managed
    /// entities are found, an empty list is returned. If there are multiple
    /// matches, all matching entities are returned.
    /// 
    /// Refers instances of *ManagedEntity*.
    pub async fn find_all_by_dns_name(&self, datacenter: Option<&crate::types::structs::ManagedObjectReference>, dns_name: &str, vm_search: bool) -> Result<Vec<crate::types::structs::ManagedObjectReference>> {
        let input = FindAllByDnsNameRequestType {datacenter, dns_name, vm_search, };
        let bytes = self.client.invoke("", "SearchIndex", &self.mo_id, "FindAllByDnsName", Some(&input)).await?;
        let result: Vec<crate::types::structs::ManagedObjectReference> = crate::core::client::unmarshal_array(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Finds all virtual machines or hosts by IP address, where the IP address is
    /// in dot-decimal notation.
    /// 
    /// For example, 10.17.12.12. The IP address for a virtual
    /// machine is the one returned from VMware tools, *GuestInfo.ipAddress*.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### datacenter
    /// If specified, restricts the query to entities in
    /// a particular datacenter. If not specified, the entire inventory is
    /// searched.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### ip
    /// The dot-decimal notation formatted IP address to find.
    ///
    /// ### vm_search
    /// If true, search for virtual machines, otherwise search for
    /// hosts.
    ///
    /// ## Returns:
    ///
    /// The list of all virtual machines or hosts that are found. If no
    /// managed entities are found, an empty list is returned. If there
    /// are multiple matches, all matching entities are returned.
    /// 
    /// Refers instances of *ManagedEntity*.
    pub async fn find_all_by_ip(&self, datacenter: Option<&crate::types::structs::ManagedObjectReference>, ip: &str, vm_search: bool) -> Result<Vec<crate::types::structs::ManagedObjectReference>> {
        let input = FindAllByIpRequestType {datacenter, ip, vm_search, };
        let bytes = self.client.invoke("", "SearchIndex", &self.mo_id, "FindAllByIp", Some(&input)).await?;
        let result: Vec<crate::types::structs::ManagedObjectReference> = crate::core::client::unmarshal_array(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Finds all virtual machines or hosts by UUID.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### datacenter
    /// If specified, restricts the query to entities in
    /// a particular datacenter. If not specified, the entire inventory is
    /// searched.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### uuid
    /// The UUID to find. If vmSearch is true, the UUID can be either BIOS
    /// or instance UUID.
    ///
    /// ### vm_search
    /// If true, search for virtual machines, otherwise search for
    /// hosts.
    ///
    /// ### instance_uuid
    /// Should only be set when vmSearch is true. If specified, search
    /// for virtual machines whose instance UUID matches the given uuid.
    /// Otherwise, search for virtual machines whose BIOS UUID matches the given
    /// uuid.
    ///
    /// ## Returns:
    ///
    /// The list of all virtual machines or hosts that are matching with
    /// the given UUID. If no managed entities are found, an empty list
    /// is returned. If there are multiple matches, all matching entities
    /// are returned.
    /// 
    /// Refers instances of *ManagedEntity*.
    pub async fn find_all_by_uuid(&self, datacenter: Option<&crate::types::structs::ManagedObjectReference>, uuid: &str, vm_search: bool, instance_uuid: Option<bool>) -> Result<Vec<crate::types::structs::ManagedObjectReference>> {
        let input = FindAllByUuidRequestType {datacenter, uuid, vm_search, instance_uuid, };
        let bytes = self.client.invoke("", "SearchIndex", &self.mo_id, "FindAllByUuid", Some(&input)).await?;
        let result: Vec<crate::types::structs::ManagedObjectReference> = crate::core::client::unmarshal_array(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Finds a virtual machine by its location on a datastore.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### datacenter
    /// Specifies the datacenter to which the datastore path belongs.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### path
    /// A datastore path to the .vmx file for the virtual machine.
    ///
    /// ## Returns:
    ///
    /// The virtual machine that is found. If no virtual machine is found, null
    /// is returned. Only a single entity is returned, even if there are multiple
    /// matches.
    /// 
    /// Refers instance of *VirtualMachine*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidDatastore***: if a datastore has not been specified in the path or if
    /// the specified datastore does not exist on the specified datacenter.
    pub async fn find_by_datastore_path(&self, datacenter: &crate::types::structs::ManagedObjectReference, path: &str) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let input = FindByDatastorePathRequestType {datacenter, path, };
        let bytes_opt = self.client.invoke_optional("", "SearchIndex", &self.mo_id, "FindByDatastorePath", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Finds a virtual machine or host by DNS name.
    /// 
    /// The DNS name for a virtual
    /// machine is the one returned from VMware tools, *GuestInfo.hostName*.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### datacenter
    /// If specified, restricts the query to entities in
    /// a particular datacenter. If not specified, the entire inventory is
    /// searched.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### dns_name
    /// The fully qualified domain name to find.
    ///
    /// ### vm_search
    /// if true, search for virtual machines, otherwise search for
    /// hosts.
    ///
    /// ## Returns:
    ///
    /// The virtual machine or host managed entity that is found. If no managed
    /// entities are found, null is returned. Only a single entity is returned, even if
    /// there are multiple matches.
    /// 
    /// Refers instance of *ManagedEntity*.
    pub async fn find_by_dns_name(&self, datacenter: Option<&crate::types::structs::ManagedObjectReference>, dns_name: &str, vm_search: bool) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let input = FindByDnsNameRequestType {datacenter, dns_name, vm_search, };
        let bytes_opt = self.client.invoke_optional("", "SearchIndex", &self.mo_id, "FindByDnsName", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Finds a managed entity based on its location in the inventory.
    /// 
    /// The path is
    /// separated by slashes ('/'). For example, a path should be of the form
    /// "My Folder/My Datacenter/vm/Discovered VM/VM1". A leading slash or trailing
    /// slash is ignored. Thus, the following paths all represents the same object:
    /// "a/b", "/a/b", "a/b/", and '/a/b/'. Slashes in names must be represented using
    /// %2f, following the standard URL syntax. Any object in the inventory can be
    /// retrieved using this method, including resource pools and hosts.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### inventory_path
    /// The path to the entity.
    ///
    /// ## Returns:
    ///
    /// The managed entity that is found. If no match is found, null is
    /// returned.
    /// 
    /// Refers instance of *ManagedEntity*.
    pub async fn find_by_inventory_path(&self, inventory_path: &str) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let input = FindByInventoryPathRequestType {inventory_path, };
        let bytes_opt = self.client.invoke_optional("", "SearchIndex", &self.mo_id, "FindByInventoryPath", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Finds a virtual machine or host by IP address, where the IP address is in
    /// dot-decimal notation.
    /// 
    /// For example, 10.17.12.12. The IP address for a virtual
    /// machine is the one returned from VMware tools, *GuestInfo.ipAddress*.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### datacenter
    /// If specified, restricts the query to entities in
    /// a particular datacenter. If not specified, the entire inventory is
    /// searched.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### ip
    /// The dot-decimal notation formatted IP address to find.
    ///
    /// ### vm_search
    /// if true, search for virtual machines, otherwise search for
    /// hosts.
    ///
    /// ## Returns:
    ///
    /// The virtual machine or host managed entity that is found. If no managed
    /// entities are found, null is returned. Only a single entity is returned,
    /// even if there are multiple matches. If called directly on an ESX server
    /// with vmSearch set to false, returns the host managed entity if the address
    /// matches any of the Console OS IP addresses.
    /// 
    /// Refers instance of *ManagedEntity*.
    pub async fn find_by_ip(&self, datacenter: Option<&crate::types::structs::ManagedObjectReference>, ip: &str, vm_search: bool) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let input = FindByIpRequestType {datacenter, ip, vm_search, };
        let bytes_opt = self.client.invoke_optional("", "SearchIndex", &self.mo_id, "FindByIp", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Finds a virtual machine or host by BIOS or instance UUID.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### datacenter
    /// If specified, restricts the query to entities in
    /// a particular datacenter. If not specified, the entire inventory is
    /// searched.
    /// 
    /// Refers instance of *Datacenter*.
    ///
    /// ### uuid
    /// The UUID to find. If vmSearch is true, the uuid can be either BIOS
    /// or instance UUID.
    ///
    /// ### vm_search
    /// If true, search for virtual machines, otherwise search for
    /// hosts.
    ///
    /// ### instance_uuid
    /// Should only be set when vmSearch is true. If specified, search
    /// for virtual machines whose instance UUID matches the given uuid.
    /// Otherwise, search for virtual machines whose BIOS UUID matches the given
    /// uuid.
    ///
    /// ## Returns:
    ///
    /// The virtual machine or host managed entity that is found. If no managed
    /// entities are found, null is returned. Only a single entity is
    /// returned, even if there are multiple matches.
    /// 
    /// Refers instance of *ManagedEntity*.
    pub async fn find_by_uuid(&self, datacenter: Option<&crate::types::structs::ManagedObjectReference>, uuid: &str, vm_search: bool, instance_uuid: Option<bool>) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let input = FindByUuidRequestType {datacenter, uuid, vm_search, instance_uuid, };
        let bytes_opt = self.client.invoke_optional("", "SearchIndex", &self.mo_id, "FindByUuid", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Finds a particular child based on a managed entity
    /// name.
    /// 
    /// This only searches the immediate children of a managed entity.
    /// For a *Datacenter*, the host and vm folders are
    /// considered children. For a *ComputeResource*,
    /// the hosts and root *ResourcePool* are considered
    /// children.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// A reference to a managed entity.
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ### name
    /// The name of the child object.
    ///
    /// ## Returns:
    ///
    /// The managed entity that is found, or null if no match is found.
    /// 
    /// Refers instance of *ManagedEntity*.
    pub async fn find_child(&self, entity: &crate::types::structs::ManagedObjectReference, name: &str) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let input = FindChildRequestType {entity, name, };
        let bytes_opt = self.client.invoke_optional("", "SearchIndex", &self.mo_id, "FindChild", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// API for efficient query/search over the managed objects (resource
    /// model) data.
    /// 
    /// It provides means to filter and retrieve projections (subsets)
    /// onto the matched resources' properties.
    /// Executes the provided *SearchIndexQuerySpec* instance, i.e., retrieves a
    /// snapshot of the resource model data as specified in the query.
    /// 
    /// ***Since:*** vSphere API Release 9.1.0.0
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### query_spec
    /// The query that defines the
    /// managed objects data to retrieve.
    ///
    /// ## Returns:
    ///
    /// Result set with data retrieved for the provided query.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if *SearchIndexQuerySpec* is not specified correctly
    pub async fn query(&self, query_spec: &crate::types::structs::SearchIndexQuerySpec) -> Result<crate::types::structs::SearchIndexResultSet> {
        let input = QueryRequestType {query_spec, };
        let bytes = self.client.invoke("", "SearchIndex", &self.mo_id, "Query", Some(&input)).await?;
        let result: crate::types::structs::SearchIndexResultSet = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// API to fetch the next page during pagination.
    /// 
    /// The marker for the *SearchIndexIterationSpec* is provided
    /// by the *SearchIndexResultSet* from the initial query request.
    /// 
    /// ***Since:*** vSphere API Release 9.1.0.0
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### iteration_spec
    /// Marker of the place to continiue iteration from.
    /// It recalculates the result again to return the rest of result.
    ///
    /// ## Returns:
    ///
    /// Result set with data retrieved for the provided query.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if *SearchIndexIterationSpec* is not specified correctly
    pub async fn query_next(&self, iteration_spec: &crate::types::structs::SearchIndexIterationSpec) -> Result<crate::types::structs::SearchIndexResultSet> {
        let input = QueryNextRequestType {iteration_spec, };
        let bytes = self.client.invoke("", "SearchIndex", &self.mo_id, "QueryNext", Some(&input)).await?;
        let result: crate::types::structs::SearchIndexResultSet = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct FindAllByDnsNameRequestType<'a> {
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    dns_name: &'a str,
    vm_search: bool,
}

impl<'a> miniserde::Serialize for FindAllByDnsNameRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(FindAllByDnsNameRequestTypeSer { data: self, seq: 0 }))
    }
}

struct FindAllByDnsNameRequestTypeSer<'b, 'a> {
    data: &'b FindAllByDnsNameRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for FindAllByDnsNameRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"FindAllByDnsNameRequestType")),
                1 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("dnsName"), &self.data.dns_name as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("vmSearch"), &self.data.vm_search as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct FindAllByIpRequestType<'a> {
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    ip: &'a str,
    vm_search: bool,
}

impl<'a> miniserde::Serialize for FindAllByIpRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(FindAllByIpRequestTypeSer { data: self, seq: 0 }))
    }
}

struct FindAllByIpRequestTypeSer<'b, 'a> {
    data: &'b FindAllByIpRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for FindAllByIpRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"FindAllByIpRequestType")),
                1 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("ip"), &self.data.ip as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("vmSearch"), &self.data.vm_search as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct FindAllByUuidRequestType<'a> {
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    uuid: &'a str,
    vm_search: bool,
    instance_uuid: Option<bool>,
}

impl<'a> miniserde::Serialize for FindAllByUuidRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(FindAllByUuidRequestTypeSer { data: self, seq: 0 }))
    }
}

struct FindAllByUuidRequestTypeSer<'b, 'a> {
    data: &'b FindAllByUuidRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for FindAllByUuidRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"FindAllByUuidRequestType")),
                1 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("uuid"), &self.data.uuid as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("vmSearch"), &self.data.vm_search as &dyn miniserde::Serialize)),
                4 => {
                    let Some(ref val) = self.data.instance_uuid else { continue; };
                    return Some((std::borrow::Cow::Borrowed("instanceUuid"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct FindByDatastorePathRequestType<'a> {
    datacenter: &'a crate::types::structs::ManagedObjectReference,
    path: &'a str,
}

impl<'a> miniserde::Serialize for FindByDatastorePathRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(FindByDatastorePathRequestTypeSer { data: self, seq: 0 }))
    }
}

struct FindByDatastorePathRequestTypeSer<'b, 'a> {
    data: &'b FindByDatastorePathRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for FindByDatastorePathRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"FindByDatastorePathRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("datacenter"), &self.data.datacenter as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("path"), &self.data.path as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct FindByDnsNameRequestType<'a> {
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    dns_name: &'a str,
    vm_search: bool,
}

impl<'a> miniserde::Serialize for FindByDnsNameRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(FindByDnsNameRequestTypeSer { data: self, seq: 0 }))
    }
}

struct FindByDnsNameRequestTypeSer<'b, 'a> {
    data: &'b FindByDnsNameRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for FindByDnsNameRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"FindByDnsNameRequestType")),
                1 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("dnsName"), &self.data.dns_name as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("vmSearch"), &self.data.vm_search as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct FindByInventoryPathRequestType<'a> {
    inventory_path: &'a str,
}

impl<'a> miniserde::Serialize for FindByInventoryPathRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(FindByInventoryPathRequestTypeSer { data: self, seq: 0 }))
    }
}

struct FindByInventoryPathRequestTypeSer<'b, 'a> {
    data: &'b FindByInventoryPathRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for FindByInventoryPathRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"FindByInventoryPathRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("inventoryPath"), &self.data.inventory_path as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct FindByIpRequestType<'a> {
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    ip: &'a str,
    vm_search: bool,
}

impl<'a> miniserde::Serialize for FindByIpRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(FindByIpRequestTypeSer { data: self, seq: 0 }))
    }
}

struct FindByIpRequestTypeSer<'b, 'a> {
    data: &'b FindByIpRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for FindByIpRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"FindByIpRequestType")),
                1 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("ip"), &self.data.ip as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("vmSearch"), &self.data.vm_search as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct FindByUuidRequestType<'a> {
    datacenter: Option<&'a crate::types::structs::ManagedObjectReference>,
    uuid: &'a str,
    vm_search: bool,
    instance_uuid: Option<bool>,
}

impl<'a> miniserde::Serialize for FindByUuidRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(FindByUuidRequestTypeSer { data: self, seq: 0 }))
    }
}

struct FindByUuidRequestTypeSer<'b, 'a> {
    data: &'b FindByUuidRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for FindByUuidRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"FindByUuidRequestType")),
                1 => {
                    let Some(ref val) = self.data.datacenter else { continue; };
                    return Some((std::borrow::Cow::Borrowed("datacenter"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("uuid"), &self.data.uuid as &dyn miniserde::Serialize)),
                3 => return Some((std::borrow::Cow::Borrowed("vmSearch"), &self.data.vm_search as &dyn miniserde::Serialize)),
                4 => {
                    let Some(ref val) = self.data.instance_uuid else { continue; };
                    return Some((std::borrow::Cow::Borrowed("instanceUuid"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct FindChildRequestType<'a> {
    entity: &'a crate::types::structs::ManagedObjectReference,
    name: &'a str,
}

impl<'a> miniserde::Serialize for FindChildRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(FindChildRequestTypeSer { data: self, seq: 0 }))
    }
}

struct FindChildRequestTypeSer<'b, 'a> {
    data: &'b FindChildRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for FindChildRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"FindChildRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("entity"), &self.data.entity as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryRequestType<'a> {
    query_spec: &'a crate::types::structs::SearchIndexQuerySpec,
}

impl<'a> miniserde::Serialize for QueryRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryRequestTypeSer<'b, 'a> {
    data: &'b QueryRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("querySpec"), &self.data.query_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryNextRequestType<'a> {
    iteration_spec: &'a crate::types::structs::SearchIndexIterationSpec,
}

impl<'a> miniserde::Serialize for QueryNextRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryNextRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryNextRequestTypeSer<'b, 'a> {
    data: &'b QueryNextRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryNextRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryNextRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("iterationSpec"), &self.data.iteration_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
