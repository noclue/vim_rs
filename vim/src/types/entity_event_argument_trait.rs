use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The event argument is a managed entity object.
/// 
/// Subclasses of this type distinguish the different managed entities
/// referenced in event objects.
pub trait EntityEventArgumentTrait : super::event_argument_trait::EventArgumentTrait {
    /// Name of the entity, including its full path from the root of the inventory.
    fn get_name(&self) -> &str;
}
impl<'s> serde::Serialize for dyn EntityEventArgumentTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn EntityEventArgumentTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(EntityEventArgumentVisitor)
            }
        }

struct EntityEventArgumentVisitor;

impl<'de> de::Visitor<'de> for EntityEventArgumentVisitor {
    type Value = Box<dyn EntityEventArgumentTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid EntityEventArgumentTrait JSON object with a _typeName field")
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

impl EntityEventArgumentTrait for EntityEventArgument {
    fn get_name(&self) -> &str { &self.name }
}
impl EntityEventArgumentTrait for AlarmEventArgument {
    fn get_name(&self) -> &str { &self.name }
}
impl EntityEventArgumentTrait for ComputeResourceEventArgument {
    fn get_name(&self) -> &str { &self.name }
}
impl EntityEventArgumentTrait for DatacenterEventArgument {
    fn get_name(&self) -> &str { &self.name }
}
impl EntityEventArgumentTrait for DatastoreEventArgument {
    fn get_name(&self) -> &str { &self.name }
}
impl EntityEventArgumentTrait for DvsEventArgument {
    fn get_name(&self) -> &str { &self.name }
}
impl EntityEventArgumentTrait for FolderEventArgument {
    fn get_name(&self) -> &str { &self.name }
}
impl EntityEventArgumentTrait for HostEventArgument {
    fn get_name(&self) -> &str { &self.name }
}
impl EntityEventArgumentTrait for ManagedEntityEventArgument {
    fn get_name(&self) -> &str { &self.name }
}
impl EntityEventArgumentTrait for NetworkEventArgument {
    fn get_name(&self) -> &str { &self.name }
}
impl EntityEventArgumentTrait for ResourcePoolEventArgument {
    fn get_name(&self) -> &str { &self.name }
}
impl EntityEventArgumentTrait for ScheduledTaskEventArgument {
    fn get_name(&self) -> &str { &self.name }
}
impl EntityEventArgumentTrait for VmEventArgument {
    fn get_name(&self) -> &str { &self.name }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn EntityEventArgumentTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
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
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
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
            _ => Err(from.as_any_box()),
        }
    }
}
