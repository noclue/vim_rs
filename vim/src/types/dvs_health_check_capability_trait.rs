use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Health check capabilities of health check supported by the
/// vSphere Distributed Switch
pub trait DvsHealthCheckCapabilityTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn DvsHealthCheckCapabilityTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DvsHealthCheckCapabilityTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DvsHealthCheckCapabilityVisitor)
            }
        }

struct DvsHealthCheckCapabilityVisitor;

impl<'de> de::Visitor<'de> for DvsHealthCheckCapabilityVisitor {
    type Value = Box<dyn DvsHealthCheckCapabilityTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DvsHealthCheckCapabilityTrait JSON object with a _typeName field")
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

impl DvsHealthCheckCapabilityTrait for DvsHealthCheckCapability {
}
impl DvsHealthCheckCapabilityTrait for VMwareDvsHealthCheckCapability {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DvsHealthCheckCapabilityTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsHealthCheckCapability => Some(from.as_any_ref().downcast_ref::<DvsHealthCheckCapability>()?),
            StructType::VMwareDvsHealthCheckCapability => Some(from.as_any_ref().downcast_ref::<VMwareDvsHealthCheckCapability>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsHealthCheckCapability => Ok(from.as_any_box().downcast::<DvsHealthCheckCapability>()?),
            StructType::VMwareDvsHealthCheckCapability => Ok(from.as_any_box().downcast::<VMwareDvsHealthCheckCapability>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
