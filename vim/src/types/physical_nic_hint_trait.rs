use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type describes each network of a physical
/// network adapter's network hint.
pub trait PhysicalNicHintTrait : super::data_object_trait::DataObjectTrait {
    /// The optional VLAN Id of the network.
    fn get_vlan_id(&self) -> Option<i32>;
}
impl<'s> serde::Serialize for dyn PhysicalNicHintTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn PhysicalNicHintTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(PhysicalNicHintVisitor)
            }
        }

struct PhysicalNicHintVisitor;

impl<'de> de::Visitor<'de> for PhysicalNicHintVisitor {
    type Value = Box<dyn PhysicalNicHintTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid PhysicalNicHintTrait JSON object with a _typeName field")
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

impl PhysicalNicHintTrait for PhysicalNicHint {
    fn get_vlan_id(&self) -> Option<i32> { self.vlan_id }
}
impl PhysicalNicHintTrait for PhysicalNicIpHint {
    fn get_vlan_id(&self) -> Option<i32> { self.vlan_id }
}
impl PhysicalNicHintTrait for PhysicalNicNameHint {
    fn get_vlan_id(&self) -> Option<i32> { self.vlan_id }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn PhysicalNicHintTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::PhysicalNicHint => Some(from.as_any_ref().downcast_ref::<PhysicalNicHint>()?),
            StructType::PhysicalNicIpHint => Some(from.as_any_ref().downcast_ref::<PhysicalNicIpHint>()?),
            StructType::PhysicalNicNameHint => Some(from.as_any_ref().downcast_ref::<PhysicalNicNameHint>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::PhysicalNicHint => Ok(from.as_any_box().downcast::<PhysicalNicHint>()?),
            StructType::PhysicalNicIpHint => Ok(from.as_any_box().downcast::<PhysicalNicIpHint>()?),
            StructType::PhysicalNicNameHint => Ok(from.as_any_box().downcast::<PhysicalNicNameHint>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
