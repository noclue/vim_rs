use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// *PortGroupProfile* is the base class for the different port group
/// subprofile objects.
pub trait PortGroupProfileTrait : super::apply_profile_trait::ApplyProfileTrait {
    /// Linkable identifier.
    fn get_key(&self) -> &str;
    /// Name of the portgroup.
    fn get_name(&self) -> &str;
    /// VLAN identifier for the port group.
    fn get_vlan(&self) -> &VlanProfile;
    /// Virtual switch to which the port group is connected.
    fn get_vswitch(&self) -> &VirtualSwitchSelectionProfile;
    /// The network policy/policies applicable on the port group.
    fn get_network_policy(&self) -> &NetworkPolicyProfile;
}
impl<'s> serde::Serialize for dyn PortGroupProfileTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn PortGroupProfileTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(PortGroupProfileVisitor)
            }
        }

struct PortGroupProfileVisitor;

impl<'de> de::Visitor<'de> for PortGroupProfileVisitor {
    type Value = Box<dyn PortGroupProfileTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid PortGroupProfileTrait JSON object with a _typeName field")
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

impl PortGroupProfileTrait for PortGroupProfile {
    fn get_key(&self) -> &str { &self.key }
    fn get_name(&self) -> &str { &self.name }
    fn get_vlan(&self) -> &VlanProfile { &self.vlan }
    fn get_vswitch(&self) -> &VirtualSwitchSelectionProfile { &self.vswitch }
    fn get_network_policy(&self) -> &NetworkPolicyProfile { &self.network_policy }
}
impl PortGroupProfileTrait for HostPortGroupProfile {
    fn get_key(&self) -> &str { &self.key }
    fn get_name(&self) -> &str { &self.name }
    fn get_vlan(&self) -> &VlanProfile { &self.vlan }
    fn get_vswitch(&self) -> &VirtualSwitchSelectionProfile { &self.vswitch }
    fn get_network_policy(&self) -> &NetworkPolicyProfile { &self.network_policy }
}
impl PortGroupProfileTrait for ServiceConsolePortGroupProfile {
    fn get_key(&self) -> &str { &self.key }
    fn get_name(&self) -> &str { &self.name }
    fn get_vlan(&self) -> &VlanProfile { &self.vlan }
    fn get_vswitch(&self) -> &VirtualSwitchSelectionProfile { &self.vswitch }
    fn get_network_policy(&self) -> &NetworkPolicyProfile { &self.network_policy }
}
impl PortGroupProfileTrait for VmPortGroupProfile {
    fn get_key(&self) -> &str { &self.key }
    fn get_name(&self) -> &str { &self.name }
    fn get_vlan(&self) -> &VlanProfile { &self.vlan }
    fn get_vswitch(&self) -> &VirtualSwitchSelectionProfile { &self.vswitch }
    fn get_network_policy(&self) -> &NetworkPolicyProfile { &self.network_policy }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn PortGroupProfileTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::PortGroupProfile => Some(from.as_any_ref().downcast_ref::<PortGroupProfile>()?),
            StructType::HostPortGroupProfile => Some(from.as_any_ref().downcast_ref::<HostPortGroupProfile>()?),
            StructType::ServiceConsolePortGroupProfile => Some(from.as_any_ref().downcast_ref::<ServiceConsolePortGroupProfile>()?),
            StructType::VmPortGroupProfile => Some(from.as_any_ref().downcast_ref::<VmPortGroupProfile>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::PortGroupProfile => Ok(from.as_any_box().downcast::<PortGroupProfile>()?),
            StructType::HostPortGroupProfile => Ok(from.as_any_box().downcast::<HostPortGroupProfile>()?),
            StructType::ServiceConsolePortGroupProfile => Ok(from.as_any_box().downcast::<ServiceConsolePortGroupProfile>()?),
            StructType::VmPortGroupProfile => Ok(from.as_any_box().downcast::<VmPortGroupProfile>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
