use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The NodeDeploymentSpec class defines location
/// specification of the nodes the VCHA Cluster along with Management
/// vCenter Server information that manages node VM.
pub trait NodeDeploymentSpecTrait : super::data_object_trait::DataObjectTrait {
    /// ESX host on which the VM is to be deployed.
    /// 
    /// For behavior when an esxHost is not specified,
    /// 
    /// See also *VirtualMachineRelocateSpec.host*.
    /// 
    /// Refers instance of *HostSystem*.
    fn get_esx_host(&self) -> &Option<ManagedObjectReference>;
    /// Datastore used for deploying the VM.
    /// 
    /// For behavior when a datastore is not specified,
    /// 
    /// See also *VirtualMachineRelocateSpec.datastore*.
    /// 
    /// Refers instance of *Datastore*.
    fn get_datastore(&self) -> &Option<ManagedObjectReference>;
    /// Name of the portgroup that is associated with the public IP address
    /// where clients connect to vCenter Server.
    /// 
    /// If a portgroup is not
    /// specified same portgroup present on source is used to deploy the VM
    /// with an assumption that portgroup is present on destination.
    /// 
    /// Refers instance of *Network*.
    fn get_public_network_port_group(&self) -> &Option<ManagedObjectReference>;
    /// Name of the portgroup that is associated with the VCHA Cluster IP
    /// address where clients connect to vCenter Server.
    /// 
    /// If a portgroup is not
    /// specified same portgroup present on source is used to deploy the VM
    /// with an assumption that portgroup is present on destination.
    /// 
    /// Refers instance of *Network*.
    fn get_cluster_network_port_group(&self) -> &Option<ManagedObjectReference>;
    /// Folder in which the VM is to be created.
    /// 
    /// Refers instance of *Folder*.
    fn get_folder(&self) -> &ManagedObjectReference;
    /// ResourcePool that will be used to deploy this node.
    /// 
    /// If the ResourcePool is not specified, the root resource pool for the
    /// host will be used.
    /// 
    /// Refers instance of *ResourcePool*.
    fn get_resource_pool(&self) -> &Option<ManagedObjectReference>;
    /// Management vCenter Server managing this VM.
    /// 
    /// If the managementVc is not specified, managementVc specified as
    /// part of SourceNodeSpec is used.
    fn get_management_vc(&self) -> &Option<ServiceLocator>;
    /// nodeName here refers to a name that will be assigned to the VM to which
    /// this node will be deployed to.
    fn get_node_name(&self) -> &str;
    /// VCHA Cluster network configuration of the node.
    /// 
    /// All cluster communication (state replication, heartbeat,
    /// cluster messages) happens over this network.
    fn get_ip_settings(&self) -> &CustomizationIpSettings;
}
impl<'s> serde::Serialize for dyn NodeDeploymentSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn NodeDeploymentSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(NodeDeploymentSpecVisitor)
            }
        }

struct NodeDeploymentSpecVisitor;

impl<'de> de::Visitor<'de> for NodeDeploymentSpecVisitor {
    type Value = Box<dyn NodeDeploymentSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid NodeDeploymentSpecTrait JSON object with a _typeName field")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let deserializer = de::value::MapAccessDeserializer::new(&mut map);
        let any: VimAny = de::Deserialize::deserialize(deserializer)?;
        match any {
            VimAny::Object(obj) => Ok(CastFrom::from_box(obj)
                .map_err(|_| de::Error::custom("Internal error converting to trait type"))?),
            VimAny::Value(value) => Err(de::Error::custom(format!(
                "expected object not wrapped value: {:?}",
                value))),
        }
    }
}

impl NodeDeploymentSpecTrait for NodeDeploymentSpec {
    fn get_esx_host(&self) -> &Option<ManagedObjectReference> { &self.esx_host }
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_public_network_port_group(&self) -> &Option<ManagedObjectReference> { &self.public_network_port_group }
    fn get_cluster_network_port_group(&self) -> &Option<ManagedObjectReference> { &self.cluster_network_port_group }
    fn get_folder(&self) -> &ManagedObjectReference { &self.folder }
    fn get_resource_pool(&self) -> &Option<ManagedObjectReference> { &self.resource_pool }
    fn get_management_vc(&self) -> &Option<ServiceLocator> { &self.management_vc }
    fn get_node_name(&self) -> &str { &self.node_name }
    fn get_ip_settings(&self) -> &CustomizationIpSettings { &self.ip_settings }
}
impl NodeDeploymentSpecTrait for PassiveNodeDeploymentSpec {
    fn get_esx_host(&self) -> &Option<ManagedObjectReference> { &self.esx_host }
    fn get_datastore(&self) -> &Option<ManagedObjectReference> { &self.datastore }
    fn get_public_network_port_group(&self) -> &Option<ManagedObjectReference> { &self.public_network_port_group }
    fn get_cluster_network_port_group(&self) -> &Option<ManagedObjectReference> { &self.cluster_network_port_group }
    fn get_folder(&self) -> &ManagedObjectReference { &self.folder }
    fn get_resource_pool(&self) -> &Option<ManagedObjectReference> { &self.resource_pool }
    fn get_management_vc(&self) -> &Option<ServiceLocator> { &self.management_vc }
    fn get_node_name(&self) -> &str { &self.node_name }
    fn get_ip_settings(&self) -> &CustomizationIpSettings { &self.ip_settings }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn NodeDeploymentSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::NodeDeploymentSpec => Some(from.as_any_ref().downcast_ref::<NodeDeploymentSpec>()?),
            StructType::PassiveNodeDeploymentSpec => Some(from.as_any_ref().downcast_ref::<PassiveNodeDeploymentSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::NodeDeploymentSpec => Ok(from.as_any_box().downcast::<NodeDeploymentSpec>()?),
            StructType::PassiveNodeDeploymentSpec => Ok(from.as_any_box().downcast::<PassiveNodeDeploymentSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
