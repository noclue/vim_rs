use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// These are datacenter events.
pub trait DatacenterEventTrait : super::event_trait::EventTrait {
}
impl<'s> serde::Serialize for dyn DatacenterEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DatacenterEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DatacenterEventVisitor)
            }
        }

struct DatacenterEventVisitor;

impl<'de> de::Visitor<'de> for DatacenterEventVisitor {
    type Value = Box<dyn DatacenterEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DatacenterEventTrait JSON object with a _typeName field")
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

impl DatacenterEventTrait for DatacenterEvent {
}
impl DatacenterEventTrait for DatacenterCreatedEvent {
}
impl DatacenterEventTrait for DatacenterRenamedEvent {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DatacenterEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DatacenterEvent => Some(from.as_any_ref().downcast_ref::<DatacenterEvent>()?),
            StructType::DatacenterCreatedEvent => Some(from.as_any_ref().downcast_ref::<DatacenterCreatedEvent>()?),
            StructType::DatacenterRenamedEvent => Some(from.as_any_ref().downcast_ref::<DatacenterRenamedEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DatacenterEvent => Ok(from.as_any_box().downcast::<DatacenterEvent>()?),
            StructType::DatacenterCreatedEvent => Ok(from.as_any_box().downcast::<DatacenterCreatedEvent>()?),
            StructType::DatacenterRenamedEvent => Ok(from.as_any_box().downcast::<DatacenterRenamedEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
