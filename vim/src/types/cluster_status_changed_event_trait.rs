use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This event records when a cluster's overall status changed.
pub trait ClusterStatusChangedEventTrait : super::cluster_event_trait::ClusterEventTrait {
    /// The old (*status*).
    fn get_old_status(&self) -> &str;
    /// The new (*status*).
    fn get_new_status(&self) -> &str;
}
impl<'s> serde::Serialize for dyn ClusterStatusChangedEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ClusterStatusChangedEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ClusterStatusChangedEventVisitor)
            }
        }

struct ClusterStatusChangedEventVisitor;

impl<'de> de::Visitor<'de> for ClusterStatusChangedEventVisitor {
    type Value = Box<dyn ClusterStatusChangedEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ClusterStatusChangedEventTrait JSON object with a _typeName field")
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

impl ClusterStatusChangedEventTrait for ClusterStatusChangedEvent {
    fn get_old_status(&self) -> &str { &self.old_status }
    fn get_new_status(&self) -> &str { &self.new_status }
}
impl ClusterStatusChangedEventTrait for HostStatusChangedEvent {
    fn get_old_status(&self) -> &str { &self.old_status }
    fn get_new_status(&self) -> &str { &self.new_status }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ClusterStatusChangedEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterStatusChangedEvent => Some(from.as_any_ref().downcast_ref::<ClusterStatusChangedEvent>()?),
            StructType::HostStatusChangedEvent => Some(from.as_any_ref().downcast_ref::<HostStatusChangedEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterStatusChangedEvent => Ok(from.as_any_box().downcast::<ClusterStatusChangedEvent>()?),
            StructType::HostStatusChangedEvent => Ok(from.as_any_box().downcast::<HostStatusChangedEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
