use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Timedout exception is thrown when a server abandons an operation that
/// is taking longer than expected.
pub trait TimedoutTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn TimedoutTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn TimedoutTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(TimedoutVisitor)
            }
        }

struct TimedoutVisitor;

impl<'de> de::Visitor<'de> for TimedoutVisitor {
    type Value = Box<dyn TimedoutTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid TimedoutTrait JSON object with a _typeName field")
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

impl TimedoutTrait for Timedout {
}
impl TimedoutTrait for PowerOnFtSecondaryTimedout {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn TimedoutTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::Timedout => Some(from.as_any_ref().downcast_ref::<Timedout>()?),
            StructType::PowerOnFtSecondaryTimedout => Some(from.as_any_ref().downcast_ref::<PowerOnFtSecondaryTimedout>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::Timedout => Ok(from.as_any_box().downcast::<Timedout>()?),
            StructType::PowerOnFtSecondaryTimedout => Ok(from.as_any_box().downcast::<PowerOnFtSecondaryTimedout>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
