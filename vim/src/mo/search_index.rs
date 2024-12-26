use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::ManagedObjectReference;
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
pub struct SearchIndex {
    client: Arc<Client>,
    mo_id: String,
}
impl SearchIndex {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
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
    pub async fn find_all_by_dns_name(&self, datacenter: Option<&ManagedObjectReference>, dns_name: &str, vm_search: bool) -> Result<Vec<ManagedObjectReference>> {
        let input = FindAllByDnsNameRequestType {datacenter, dns_name, vm_search, };
        let path = format!("/SearchIndex/{moId}/FindAllByDnsName", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn find_all_by_ip(&self, datacenter: Option<&ManagedObjectReference>, ip: &str, vm_search: bool) -> Result<Vec<ManagedObjectReference>> {
        let input = FindAllByIpRequestType {datacenter, ip, vm_search, };
        let path = format!("/SearchIndex/{moId}/FindAllByIp", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn find_all_by_uuid(&self, datacenter: Option<&ManagedObjectReference>, uuid: &str, vm_search: bool, instance_uuid: Option<bool>) -> Result<Vec<ManagedObjectReference>> {
        let input = FindAllByUuidRequestType {datacenter, uuid, vm_search, instance_uuid, };
        let path = format!("/SearchIndex/{moId}/FindAllByUuid", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn find_by_datastore_path(&self, datacenter: &ManagedObjectReference, path: &str) -> Result<ManagedObjectReference> {
        let input = FindByDatastorePathRequestType {datacenter, path, };
        let path = format!("/SearchIndex/{moId}/FindByDatastorePath", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn find_by_dns_name(&self, datacenter: Option<&ManagedObjectReference>, dns_name: &str, vm_search: bool) -> Result<ManagedObjectReference> {
        let input = FindByDnsNameRequestType {datacenter, dns_name, vm_search, };
        let path = format!("/SearchIndex/{moId}/FindByDnsName", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn find_by_inventory_path(&self, inventory_path: &str) -> Result<ManagedObjectReference> {
        let input = FindByInventoryPathRequestType {inventory_path, };
        let path = format!("/SearchIndex/{moId}/FindByInventoryPath", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn find_by_ip(&self, datacenter: Option<&ManagedObjectReference>, ip: &str, vm_search: bool) -> Result<ManagedObjectReference> {
        let input = FindByIpRequestType {datacenter, ip, vm_search, };
        let path = format!("/SearchIndex/{moId}/FindByIp", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn find_by_uuid(&self, datacenter: Option<&ManagedObjectReference>, uuid: &str, vm_search: bool, instance_uuid: Option<bool>) -> Result<ManagedObjectReference> {
        let input = FindByUuidRequestType {datacenter, uuid, vm_search, instance_uuid, };
        let path = format!("/SearchIndex/{moId}/FindByUuid", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn find_child(&self, entity: &ManagedObjectReference, name: &str) -> Result<ManagedObjectReference> {
        let input = FindChildRequestType {entity, name, };
        let path = format!("/SearchIndex/{moId}/FindChild", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct FindAllByDnsNameRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
    #[serde(rename = "dnsName")]
    dns_name: &'a str,
    #[serde(rename = "vmSearch")]
    vm_search: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct FindAllByIpRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
    ip: &'a str,
    #[serde(rename = "vmSearch")]
    vm_search: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct FindAllByUuidRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
    uuid: &'a str,
    #[serde(rename = "vmSearch")]
    vm_search: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instanceUuid")]
    instance_uuid: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct FindByDatastorePathRequestType<'a> {
    datacenter: &'a ManagedObjectReference,
    path: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct FindByDnsNameRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
    #[serde(rename = "dnsName")]
    dns_name: &'a str,
    #[serde(rename = "vmSearch")]
    vm_search: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct FindByInventoryPathRequestType<'a> {
    #[serde(rename = "inventoryPath")]
    inventory_path: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct FindByIpRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
    ip: &'a str,
    #[serde(rename = "vmSearch")]
    vm_search: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct FindByUuidRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    datacenter: Option<&'a ManagedObjectReference>,
    uuid: &'a str,
    #[serde(rename = "vmSearch")]
    vm_search: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instanceUuid")]
    instance_uuid: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct FindChildRequestType<'a> {
    entity: &'a ManagedObjectReference,
    name: &'a str,
}
