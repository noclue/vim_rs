use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Thrown when the client is not allowed access to the property or method.
pub trait SecurityErrorTrait : super::runtime_fault_trait::RuntimeFaultTrait {
}
impl<'s> serde::Serialize for dyn SecurityErrorTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn SecurityErrorTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(SecurityErrorVisitor)
            }
        }

struct SecurityErrorVisitor;

impl<'de> de::Visitor<'de> for SecurityErrorVisitor {
    type Value = Box<dyn SecurityErrorTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid SecurityErrorTrait JSON object with a _typeName field")
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

impl SecurityErrorTrait for SecurityError {
}
impl SecurityErrorTrait for NoPermission {
}
impl SecurityErrorTrait for NotAuthenticated {
}
impl SecurityErrorTrait for RestrictedVersion {
}
impl SecurityErrorTrait for SolutionUserRequired {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn SecurityErrorTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::SecurityError => Some(from.as_any_ref().downcast_ref::<SecurityError>()?),
            StructType::NoPermission => Some(from.as_any_ref().downcast_ref::<NoPermission>()?),
            StructType::NotAuthenticated => Some(from.as_any_ref().downcast_ref::<NotAuthenticated>()?),
            StructType::RestrictedVersion => Some(from.as_any_ref().downcast_ref::<RestrictedVersion>()?),
            StructType::SolutionUserRequired => Some(from.as_any_ref().downcast_ref::<SolutionUserRequired>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::SecurityError => Ok(from.as_any_box().downcast::<SecurityError>()?),
            StructType::NoPermission => Ok(from.as_any_box().downcast::<NoPermission>()?),
            StructType::NotAuthenticated => Ok(from.as_any_box().downcast::<NotAuthenticated>()?),
            StructType::RestrictedVersion => Ok(from.as_any_box().downcast::<RestrictedVersion>()?),
            StructType::SolutionUserRequired => Ok(from.as_any_box().downcast::<SolutionUserRequired>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
