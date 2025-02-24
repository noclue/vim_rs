use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This event records a virtual machine migration.
pub trait VmMigratedEventTrait : super::vm_event_trait::VmEventTrait {
    /// The source host.
    /// 
    /// (Because this is after a successful migration,
    /// the destination host is recorded in the inherited "host" property.)
    fn get_source_host(&self) -> &HostEventArgument;
    /// The source datacenter
    fn get_source_datacenter(&self) -> &Option<DatacenterEventArgument>;
    /// The source primary datastore
    fn get_source_datastore(&self) -> &Option<DatastoreEventArgument>;
}
impl<'s> serde::Serialize for dyn VmMigratedEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VmMigratedEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VmMigratedEventVisitor)
            }
        }

struct VmMigratedEventVisitor;

impl<'de> de::Visitor<'de> for VmMigratedEventVisitor {
    type Value = Box<dyn VmMigratedEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VmMigratedEventTrait JSON object with a _typeName field")
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

impl VmMigratedEventTrait for VmMigratedEvent {
    fn get_source_host(&self) -> &HostEventArgument { &self.source_host }
    fn get_source_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.source_datacenter }
    fn get_source_datastore(&self) -> &Option<DatastoreEventArgument> { &self.source_datastore }
}
impl VmMigratedEventTrait for DrsVmMigratedEvent {
    fn get_source_host(&self) -> &HostEventArgument { &self.source_host }
    fn get_source_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.source_datacenter }
    fn get_source_datastore(&self) -> &Option<DatastoreEventArgument> { &self.source_datastore }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VmMigratedEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmMigratedEvent => Some(from.as_any_ref().downcast_ref::<VmMigratedEvent>()?),
            StructType::DrsVmMigratedEvent => Some(from.as_any_ref().downcast_ref::<DrsVmMigratedEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmMigratedEvent => Ok(from.as_any_box().downcast::<VmMigratedEvent>()?),
            StructType::DrsVmMigratedEvent => Ok(from.as_any_box().downcast::<DrsVmMigratedEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
