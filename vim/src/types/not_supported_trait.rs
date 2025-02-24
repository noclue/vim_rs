use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Thrown if the method is not supported on the
/// server.
/// 
/// Not all methods are supported on all servers
/// (for example, an ESX Server host supports less functionality than
/// a VirtualCenter server). A feature might also be disabled
/// due to missing liceneses.
pub trait NotSupportedTrait : super::runtime_fault_trait::RuntimeFaultTrait {
}
impl<'s> serde::Serialize for dyn NotSupportedTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn NotSupportedTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(NotSupportedVisitor)
            }
        }

struct NotSupportedVisitor;

impl<'de> de::Visitor<'de> for NotSupportedVisitor {
    type Value = Box<dyn NotSupportedTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid NotSupportedTrait JSON object with a _typeName field")
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

impl NotSupportedTrait for NotSupported {
}
impl NotSupportedTrait for HostAccessRestrictedToManagementServer {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn NotSupportedTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::NotSupported => Some(from.as_any_ref().downcast_ref::<NotSupported>()?),
            StructType::HostAccessRestrictedToManagementServer => Some(from.as_any_ref().downcast_ref::<HostAccessRestrictedToManagementServer>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::NotSupported => Ok(from.as_any_box().downcast::<NotSupported>()?),
            StructType::HostAccessRestrictedToManagementServer => Ok(from.as_any_box().downcast::<HostAccessRestrictedToManagementServer>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
