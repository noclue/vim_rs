use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base class for Vlan Specifiation for ports.
pub trait VmwareDistributedVirtualSwitchVlanSpecTrait : super::inheritable_policy_trait::InheritablePolicyTrait {
}
impl<'s> serde::Serialize for dyn VmwareDistributedVirtualSwitchVlanSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VmwareDistributedVirtualSwitchVlanSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VmwareDistributedVirtualSwitchVlanSpecVisitor)
            }
        }

struct VmwareDistributedVirtualSwitchVlanSpecVisitor;

impl<'de> de::Visitor<'de> for VmwareDistributedVirtualSwitchVlanSpecVisitor {
    type Value = Box<dyn VmwareDistributedVirtualSwitchVlanSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VmwareDistributedVirtualSwitchVlanSpecTrait JSON object with a _typeName field")
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

impl VmwareDistributedVirtualSwitchVlanSpecTrait for VmwareDistributedVirtualSwitchVlanSpec {
}
impl VmwareDistributedVirtualSwitchVlanSpecTrait for VmwareDistributedVirtualSwitchPvlanSpec {
}
impl VmwareDistributedVirtualSwitchVlanSpecTrait for VmwareDistributedVirtualSwitchTrunkVlanSpec {
}
impl VmwareDistributedVirtualSwitchVlanSpecTrait for VmwareDistributedVirtualSwitchVlanIdSpec {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VmwareDistributedVirtualSwitchVlanSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmwareDistributedVirtualSwitchVlanSpec => Some(from.as_any_ref().downcast_ref::<VmwareDistributedVirtualSwitchVlanSpec>()?),
            StructType::VmwareDistributedVirtualSwitchPvlanSpec => Some(from.as_any_ref().downcast_ref::<VmwareDistributedVirtualSwitchPvlanSpec>()?),
            StructType::VmwareDistributedVirtualSwitchTrunkVlanSpec => Some(from.as_any_ref().downcast_ref::<VmwareDistributedVirtualSwitchTrunkVlanSpec>()?),
            StructType::VmwareDistributedVirtualSwitchVlanIdSpec => Some(from.as_any_ref().downcast_ref::<VmwareDistributedVirtualSwitchVlanIdSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmwareDistributedVirtualSwitchVlanSpec => Ok(from.as_any_box().downcast::<VmwareDistributedVirtualSwitchVlanSpec>()?),
            StructType::VmwareDistributedVirtualSwitchPvlanSpec => Ok(from.as_any_box().downcast::<VmwareDistributedVirtualSwitchPvlanSpec>()?),
            StructType::VmwareDistributedVirtualSwitchTrunkVlanSpec => Ok(from.as_any_box().downcast::<VmwareDistributedVirtualSwitchTrunkVlanSpec>()?),
            StructType::VmwareDistributedVirtualSwitchVlanIdSpec => Ok(from.as_any_box().downcast::<VmwareDistributedVirtualSwitchVlanIdSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
