use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object type provides access to the environment that a
/// ComputeResource presents for creating and configuring a virtual machine.
/// 
/// The environment consists of three main components:
/// - The virtual machine configuration options. Each vim.vm.ConfigOption
///   describes the execution environment for a virtual machine, the particular
///   set of virtual hardware that is supported. A
///   ComputeResource might support multiple sets. Access is provided
///   through the configOptionDescriptor property and the
///   *EnvironmentBrowser.QueryConfigOption* operation.
/// - The supported device targets. Each virtual device specified in the virtual
///   machine needs to be hooked up to a "physical" counterpart. For networks,
///   this means choosing a network name; for a virtual CD-rom this might be
///   an ISO image, etc. The environment browser provides access to the device
///   targets through the
///   *EnvironmentBrowser.QueryConfigTarget* operation.
/// - Storage locations and files. A selection of locations where the virtual machine
///   files can be stored, and the possibility to browse for existing virtual disks
///   and ISO images. The datastore browser, provided by the datastoreBrowser
///   property, provides access to the contents of one or more datastores. The
///   items in a datastore are files that contain configuration, virtual disk, and
///   the other data associated with a virtual machine.
/// - The capabilities supported by the ComputeResource to which the virtual
///   machine belongs.
#[derive(Clone)]
pub struct EnvironmentBrowser {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl EnvironmentBrowser {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Query for a specific virtual machine configuration option (the ConfigOption).
    /// 
    /// If the EnvironmentBrowser is from a ComputeResource or ClusterComputeResource,
    /// the key or host, or both arguments can be used to return the required config
    /// options. If a key is specified, then the ConfigOption corresponding to that key
    /// value is returned. If a host is specified, then the default ConfigOption for
    /// that host is returned. If key and host both are specified, the ConfigOption
    /// corresponding to the given key for that host is returned. If neither is specified,
    /// then the default ConfigOption for this environment browser is returned. Typically,
    /// the default contains the options for the most recent virtual hardware
    /// supported.
    /// 
    /// If the EnvironmentBrowser is from a VirtualMachine neither a host nor a
    /// key should be specified.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### key
    /// The key found in the VirtualMachineConfigOptionDescriptor,
    /// obtained by invoking the
    /// *EnvironmentBrowser.QueryConfigOptionDescriptor* operation.
    ///
    /// ### host
    /// The host whose ConfigOption is requested.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// Returns the ConfigOption object. If invoked on a cluster with no hosts, or
    /// if the ConfigOption with given key is not found for the given host, null
    /// is returned.
    pub async fn query_config_option(&self, key: Option<&str>, host: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<crate::types::structs::VirtualMachineConfigOption>> {
        let input = QueryConfigOptionRequestType {key, host, };
        let bytes_opt = self.client.invoke_optional("", "EnvironmentBrowser", &self.mo_id, "QueryConfigOption", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// The list of ConfigOption keys available on this entity.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn query_config_option_descriptor(&self) -> Result<Option<Vec<crate::types::structs::VirtualMachineConfigOptionDescriptor>>> {
        let bytes_opt = self.client.invoke_optional("", "EnvironmentBrowser", &self.mo_id, "QueryConfigOptionDescriptor", None).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Query for a virtual machine configuration *option*
    /// matching the key or host or both given in the
    /// *EnvironmentBrowserConfigOptionQuerySpec*.
    /// 
    /// For more details see
    /// *EnvironmentBrowser.QueryConfigOption*
    /// 
    /// If the Environment Browser belongs to a virtual machine and the spec argument
    /// is omitted, the method returns the ConfigOption object corresponding to the
    /// vmx version of the virtual machine and the
    /// *guestOSDescriptor* list contains
    /// only the guestId of the virtual machine.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// Search criteria and filters to control the result.
    /// If a *EnvironmentBrowserConfigOptionQuerySpec.key* or
    /// *EnvironmentBrowserConfigOptionQuerySpec.host* (or both)
    /// are specified, they will be used to search for a config option.
    /// If *EnvironmentBrowserConfigOptionQuerySpec.guestId*
    /// is nonempty, the *VirtualMachineConfigOption.guestOSDescriptor*
    /// array of the config option is filtered to match against the guest
    /// IDs in the spec. If there is no match, the whole list is returned.
    /// If the spec argument is omitted, the default
    /// *VirtualMachineConfigOption* for this environment browser is
    /// returned.
    ///
    /// ## Returns:
    ///
    /// Returns the *VirtualMachineConfigOption* object. If invoked on a cluster
    /// with no hosts, or if the *VirtualMachineConfigOption* with given key is
    /// not found for the given host, null is returned.
    pub async fn query_config_option_ex(&self, spec: Option<&crate::types::structs::EnvironmentBrowserConfigOptionQuerySpec>) -> Result<Option<crate::types::structs::VirtualMachineConfigOption>> {
        let input = QueryConfigOptionExRequestType {spec, };
        let bytes_opt = self.client.invoke_optional("", "EnvironmentBrowser", &self.mo_id, "QueryConfigOptionEx", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Queries for information about a specific target, a "physical" device that
    /// can be used to back virtual devices.
    /// 
    /// The ConfigTarget that is returned specifies
    /// the set of values that can be used in the device backings to connect the virtual
    /// machine to physical (or logical) host devices.
    /// 
    /// If the EnvironmentBrowser is from a ComputeResource or ClusterComputeResource,
    /// the host argument can be used to return the ConfigTarget provided by a particular
    /// host in the compute resource or cluster. If host is not specified and the
    /// EnvironmentBrowser is from a ComputeResource or ClusterComputeResource,
    /// then the union of all the devices is returned and the
    /// vim.vm.TargetInfo.configurationTag field indicates how widely the device is
    /// available across the compute resource or cluster.
    /// 
    /// If the EnvironmentBrowser is from a VirtualMachine a host should not be specified.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// If specified, the host whose default BackingInfo is requested.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// Returns the ConfigTarget object. If invoked on a cluster with no hosts,
    /// null is returned.
    pub async fn query_config_target(&self, host: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<crate::types::structs::ConfigTarget>> {
        let input = QueryConfigTargetRequestType {host, };
        let bytes_opt = self.client.invoke_optional("", "EnvironmentBrowser", &self.mo_id, "QueryConfigTarget", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Queries for information on the capabilities supported by the ComputeResource
    /// associated with the EnvironmentBrowser.
    /// 
    /// If the EnvironmentBrowser is from a ComputeResource or ClusterComputeResource,
    /// the host argument can be used to return the capabilities associated with a
    /// specific host in the compute resource or cluster. If the host argument is not
    /// specified and the EnvironmentBrowser is from a ComputeResource or
    /// ClusterComputeResource, then the intersection of the capabilities supported by
    /// all the hosts in the cluster is returned. If the EnvironmentBrowser is from
    /// a VirtualMachine, the compute resource associated with the virtual machine
    /// will be queried for its capabilities.
    /// 
    /// If the EnvironmentBrowser is from a VirtualMachine a host should not be specified.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// If specified, the host whose capabilities are requested.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// Returns the set of capabilities supported by the ComputeResource
    /// associated with the EnvironmentBrowser. If invoked on a cluster with
    /// no hosts, null is returned.
    pub async fn query_target_capabilities(&self, host: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<crate::types::structs::HostCapability>> {
        let input = QueryTargetCapabilitiesRequestType {host, };
        let bytes_opt = self.client.invoke_optional("", "EnvironmentBrowser", &self.mo_id, "QueryTargetCapabilities", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// DatastoreBrowser to browse datastores that are available on this entity.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instance of *HostDatastoreBrowser*.
    pub async fn datastore_browser(&self) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let pv_opt = self.client.fetch_property_raw("", "EnvironmentBrowser", &self.mo_id, "datastoreBrowser").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
}
struct QueryConfigOptionRequestType<'a> {
    key: Option<&'a str>,
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for QueryConfigOptionRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryConfigOptionRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryConfigOptionRequestTypeSer<'b, 'a> {
    data: &'b QueryConfigOptionRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryConfigOptionRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryConfigOptionRequestType")),
                1 => {
                    let Some(ref val) = self.data.key else { continue; };
                    return Some((std::borrow::Cow::Borrowed("key"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryConfigOptionExRequestType<'a> {
    spec: Option<&'a crate::types::structs::EnvironmentBrowserConfigOptionQuerySpec>,
}

impl<'a> miniserde::Serialize for QueryConfigOptionExRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryConfigOptionExRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryConfigOptionExRequestTypeSer<'b, 'a> {
    data: &'b QueryConfigOptionExRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryConfigOptionExRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryConfigOptionExRequestType")),
                1 => {
                    let Some(ref val) = self.data.spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("spec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryConfigTargetRequestType<'a> {
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for QueryConfigTargetRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryConfigTargetRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryConfigTargetRequestTypeSer<'b, 'a> {
    data: &'b QueryConfigTargetRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryConfigTargetRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryConfigTargetRequestType")),
                1 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryTargetCapabilitiesRequestType<'a> {
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for QueryTargetCapabilitiesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryTargetCapabilitiesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryTargetCapabilitiesRequestTypeSer<'b, 'a> {
    data: &'b QueryTargetCapabilitiesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryTargetCapabilitiesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryTargetCapabilitiesRequestType")),
                1 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
