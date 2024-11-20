use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::EnvironmentBrowserConfigOptionQuerySpec;
use crate::types::ManagedObjectReference;
use crate::types::ConfigTarget;
use crate::types::HostCapability;
use crate::types::VirtualMachineConfigOption;
use crate::types::VirtualMachineConfigOptionDescriptor;
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
pub struct EnvironmentBrowser {
    client: Arc<VimClient>,
    mo_id: String,
}
impl EnvironmentBrowser {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
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
    pub async fn query_config_option(&self, key: Option<&str>, host: Option<&ManagedObjectReference>) -> Result<VirtualMachineConfigOption> {
        let input = QueryConfigOptionRequestType {key, host, };
        let path = format!("/EnvironmentBrowser/{moId}/QueryConfigOption", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// The list of ConfigOption keys available on this entity.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn query_config_option_descriptor(&self) -> Result<Option<Vec<VirtualMachineConfigOptionDescriptor>>> {
        let path = format!("/EnvironmentBrowser/{moId}/QueryConfigOptionDescriptor", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
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
    /// vmx version of the virutal machine and the
    /// *guestOSDescriptor* list contains
    /// only the guestId of the virutal machine.
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
    pub async fn query_config_option_ex(&self, spec: Option<&EnvironmentBrowserConfigOptionQuerySpec>) -> Result<VirtualMachineConfigOption> {
        let input = QueryConfigOptionExRequestType {spec, };
        let path = format!("/EnvironmentBrowser/{moId}/QueryConfigOptionEx", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn query_config_target(&self, host: Option<&ManagedObjectReference>) -> Result<ConfigTarget> {
        let input = QueryConfigTargetRequestType {host, };
        let path = format!("/EnvironmentBrowser/{moId}/QueryConfigTarget", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn query_target_capabilities(&self, host: Option<&ManagedObjectReference>) -> Result<HostCapability> {
        let input = QueryTargetCapabilitiesRequestType {host, };
        let path = format!("/EnvironmentBrowser/{moId}/QueryTargetCapabilities", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// DatastoreBrowser to browse datastores that are available on this entity.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instance of *HostDatastoreBrowser*.
    pub async fn datastore_browser(&self) -> Result<ManagedObjectReference> {
        let path = format!("/EnvironmentBrowser/{moId}/datastoreBrowser", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryConfigOptionRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    key: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryConfigOptionExRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    spec: Option<&'a EnvironmentBrowserConfigOptionQuerySpec>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryConfigTargetRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryTargetCapabilitiesRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<&'a ManagedObjectReference>,
}
