use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base class for faults that can be thrown while invoking iSCSI management operations.
pub trait IscsiFaultTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn IscsiFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn IscsiFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(IscsiFaultVisitor)
            }
        }

struct IscsiFaultVisitor;

impl<'de> de::Visitor<'de> for IscsiFaultVisitor {
    type Value = Box<dyn IscsiFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid IscsiFaultTrait JSON object with a _typeName field")
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

impl IscsiFaultTrait for IscsiFault {
}
impl IscsiFaultTrait for IscsiFaultInvalidVnic {
}
impl IscsiFaultTrait for IscsiFaultPnicInUse {
}
impl IscsiFaultTrait for IscsiFaultVnicAlreadyBound {
}
impl IscsiFaultTrait for IscsiFaultVnicHasActivePaths {
}
impl IscsiFaultTrait for IscsiFaultVnicHasMultipleUplinks {
}
impl IscsiFaultTrait for IscsiFaultVnicHasNoUplinks {
}
impl IscsiFaultTrait for IscsiFaultVnicHasWrongUplink {
}
impl IscsiFaultTrait for IscsiFaultVnicInUse {
}
impl IscsiFaultTrait for IscsiFaultVnicIsLastPath {
}
impl IscsiFaultTrait for IscsiFaultVnicNotBound {
}
impl IscsiFaultTrait for IscsiFaultVnicNotFound {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn IscsiFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::IscsiFault => Some(from.as_any_ref().downcast_ref::<IscsiFault>()?),
            StructType::IscsiFaultInvalidVnic => Some(from.as_any_ref().downcast_ref::<IscsiFaultInvalidVnic>()?),
            StructType::IscsiFaultPnicInUse => Some(from.as_any_ref().downcast_ref::<IscsiFaultPnicInUse>()?),
            StructType::IscsiFaultVnicAlreadyBound => Some(from.as_any_ref().downcast_ref::<IscsiFaultVnicAlreadyBound>()?),
            StructType::IscsiFaultVnicHasActivePaths => Some(from.as_any_ref().downcast_ref::<IscsiFaultVnicHasActivePaths>()?),
            StructType::IscsiFaultVnicHasMultipleUplinks => Some(from.as_any_ref().downcast_ref::<IscsiFaultVnicHasMultipleUplinks>()?),
            StructType::IscsiFaultVnicHasNoUplinks => Some(from.as_any_ref().downcast_ref::<IscsiFaultVnicHasNoUplinks>()?),
            StructType::IscsiFaultVnicHasWrongUplink => Some(from.as_any_ref().downcast_ref::<IscsiFaultVnicHasWrongUplink>()?),
            StructType::IscsiFaultVnicInUse => Some(from.as_any_ref().downcast_ref::<IscsiFaultVnicInUse>()?),
            StructType::IscsiFaultVnicIsLastPath => Some(from.as_any_ref().downcast_ref::<IscsiFaultVnicIsLastPath>()?),
            StructType::IscsiFaultVnicNotBound => Some(from.as_any_ref().downcast_ref::<IscsiFaultVnicNotBound>()?),
            StructType::IscsiFaultVnicNotFound => Some(from.as_any_ref().downcast_ref::<IscsiFaultVnicNotFound>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::IscsiFault => Ok(from.as_any_box().downcast::<IscsiFault>()?),
            StructType::IscsiFaultInvalidVnic => Ok(from.as_any_box().downcast::<IscsiFaultInvalidVnic>()?),
            StructType::IscsiFaultPnicInUse => Ok(from.as_any_box().downcast::<IscsiFaultPnicInUse>()?),
            StructType::IscsiFaultVnicAlreadyBound => Ok(from.as_any_box().downcast::<IscsiFaultVnicAlreadyBound>()?),
            StructType::IscsiFaultVnicHasActivePaths => Ok(from.as_any_box().downcast::<IscsiFaultVnicHasActivePaths>()?),
            StructType::IscsiFaultVnicHasMultipleUplinks => Ok(from.as_any_box().downcast::<IscsiFaultVnicHasMultipleUplinks>()?),
            StructType::IscsiFaultVnicHasNoUplinks => Ok(from.as_any_box().downcast::<IscsiFaultVnicHasNoUplinks>()?),
            StructType::IscsiFaultVnicHasWrongUplink => Ok(from.as_any_box().downcast::<IscsiFaultVnicHasWrongUplink>()?),
            StructType::IscsiFaultVnicInUse => Ok(from.as_any_box().downcast::<IscsiFaultVnicInUse>()?),
            StructType::IscsiFaultVnicIsLastPath => Ok(from.as_any_box().downcast::<IscsiFaultVnicIsLastPath>()?),
            StructType::IscsiFaultVnicNotBound => Ok(from.as_any_box().downcast::<IscsiFaultVnicNotBound>()?),
            StructType::IscsiFaultVnicNotFound => Ok(from.as_any_box().downcast::<IscsiFaultVnicNotFound>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
