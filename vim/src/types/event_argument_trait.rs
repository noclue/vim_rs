use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This is the base type for event argument types.
/// 
/// Event argument objects, which inherit from a common subtype,
/// are used to manage supplementary properties of different kinds
/// of event objects.
pub trait EventArgumentTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn EventArgumentTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn EventArgumentTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(EventArgumentVisitor)
            }
        }

struct EventArgumentVisitor;

impl<'de> de::Visitor<'de> for EventArgumentVisitor {
    type Value = Box<dyn EventArgumentTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid EventArgumentTrait JSON object with a _typeName field")
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

impl EventArgumentTrait for EventArgument {
}
impl EventArgumentTrait for EntityEventArgument {
}
impl EventArgumentTrait for AlarmEventArgument {
}
impl EventArgumentTrait for ComputeResourceEventArgument {
}
impl EventArgumentTrait for DatacenterEventArgument {
}
impl EventArgumentTrait for DatastoreEventArgument {
}
impl EventArgumentTrait for DvsEventArgument {
}
impl EventArgumentTrait for FolderEventArgument {
}
impl EventArgumentTrait for HostEventArgument {
}
impl EventArgumentTrait for ManagedEntityEventArgument {
}
impl EventArgumentTrait for NetworkEventArgument {
}
impl EventArgumentTrait for ResourcePoolEventArgument {
}
impl EventArgumentTrait for ScheduledTaskEventArgument {
}
impl EventArgumentTrait for VmEventArgument {
}
impl EventArgumentTrait for ProfileEventArgument {
}
impl EventArgumentTrait for RoleEventArgument {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn EventArgumentTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::EventArgument => Some(from.as_any_ref().downcast_ref::<EventArgument>()?),
            StructType::EntityEventArgument => Some(from.as_any_ref().downcast_ref::<EntityEventArgument>()?),
            StructType::AlarmEventArgument => Some(from.as_any_ref().downcast_ref::<AlarmEventArgument>()?),
            StructType::ComputeResourceEventArgument => Some(from.as_any_ref().downcast_ref::<ComputeResourceEventArgument>()?),
            StructType::DatacenterEventArgument => Some(from.as_any_ref().downcast_ref::<DatacenterEventArgument>()?),
            StructType::DatastoreEventArgument => Some(from.as_any_ref().downcast_ref::<DatastoreEventArgument>()?),
            StructType::DvsEventArgument => Some(from.as_any_ref().downcast_ref::<DvsEventArgument>()?),
            StructType::FolderEventArgument => Some(from.as_any_ref().downcast_ref::<FolderEventArgument>()?),
            StructType::HostEventArgument => Some(from.as_any_ref().downcast_ref::<HostEventArgument>()?),
            StructType::ManagedEntityEventArgument => Some(from.as_any_ref().downcast_ref::<ManagedEntityEventArgument>()?),
            StructType::NetworkEventArgument => Some(from.as_any_ref().downcast_ref::<NetworkEventArgument>()?),
            StructType::ResourcePoolEventArgument => Some(from.as_any_ref().downcast_ref::<ResourcePoolEventArgument>()?),
            StructType::ScheduledTaskEventArgument => Some(from.as_any_ref().downcast_ref::<ScheduledTaskEventArgument>()?),
            StructType::VmEventArgument => Some(from.as_any_ref().downcast_ref::<VmEventArgument>()?),
            StructType::ProfileEventArgument => Some(from.as_any_ref().downcast_ref::<ProfileEventArgument>()?),
            StructType::RoleEventArgument => Some(from.as_any_ref().downcast_ref::<RoleEventArgument>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::EventArgument => Ok(from.as_any_box().downcast::<EventArgument>()?),
            StructType::EntityEventArgument => Ok(from.as_any_box().downcast::<EntityEventArgument>()?),
            StructType::AlarmEventArgument => Ok(from.as_any_box().downcast::<AlarmEventArgument>()?),
            StructType::ComputeResourceEventArgument => Ok(from.as_any_box().downcast::<ComputeResourceEventArgument>()?),
            StructType::DatacenterEventArgument => Ok(from.as_any_box().downcast::<DatacenterEventArgument>()?),
            StructType::DatastoreEventArgument => Ok(from.as_any_box().downcast::<DatastoreEventArgument>()?),
            StructType::DvsEventArgument => Ok(from.as_any_box().downcast::<DvsEventArgument>()?),
            StructType::FolderEventArgument => Ok(from.as_any_box().downcast::<FolderEventArgument>()?),
            StructType::HostEventArgument => Ok(from.as_any_box().downcast::<HostEventArgument>()?),
            StructType::ManagedEntityEventArgument => Ok(from.as_any_box().downcast::<ManagedEntityEventArgument>()?),
            StructType::NetworkEventArgument => Ok(from.as_any_box().downcast::<NetworkEventArgument>()?),
            StructType::ResourcePoolEventArgument => Ok(from.as_any_box().downcast::<ResourcePoolEventArgument>()?),
            StructType::ScheduledTaskEventArgument => Ok(from.as_any_box().downcast::<ScheduledTaskEventArgument>()?),
            StructType::VmEventArgument => Ok(from.as_any_box().downcast::<VmEventArgument>()?),
            StructType::ProfileEventArgument => Ok(from.as_any_box().downcast::<ProfileEventArgument>()?),
            StructType::RoleEventArgument => Ok(from.as_any_box().downcast::<RoleEventArgument>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
