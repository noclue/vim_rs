use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// One of the virtual machine's components is not accessible on the execution host.
/// 
/// This is a base class. Subclasses will encode the type of component that is not
/// accessible.
pub trait CannotAccessVmComponentTrait : super::vm_config_fault_trait::VmConfigFaultTrait {
}
impl<'s> serde::Serialize for dyn CannotAccessVmComponentTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn CannotAccessVmComponentTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(CannotAccessVmComponentVisitor)
            }
        }

struct CannotAccessVmComponentVisitor;

impl<'de> de::Visitor<'de> for CannotAccessVmComponentVisitor {
    type Value = Box<dyn CannotAccessVmComponentTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid CannotAccessVmComponentTrait JSON object with a _typeName field")
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

impl CannotAccessVmComponentTrait for CannotAccessVmComponent {
}
impl CannotAccessVmComponentTrait for CannotAccessVmConfig {
}
impl CannotAccessVmComponentTrait for CannotAccessVmDevice {
}
impl CannotAccessVmComponentTrait for CannotAccessNetwork {
}
impl CannotAccessVmComponentTrait for DestinationSwitchFull {
}
impl CannotAccessVmComponentTrait for LegacyNetworkInterfaceInUse {
}
impl CannotAccessVmComponentTrait for VmOnConflictDvPort {
}
impl CannotAccessVmComponentTrait for VmOnVirtualIntranet {
}
impl CannotAccessVmComponentTrait for CannotAccessVmDisk {
}
impl CannotAccessVmComponentTrait for RdmPointsToInaccessibleDisk {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn CannotAccessVmComponentTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::CannotAccessVmComponent => Some(from.as_any_ref().downcast_ref::<CannotAccessVmComponent>()?),
            StructType::CannotAccessVmConfig => Some(from.as_any_ref().downcast_ref::<CannotAccessVmConfig>()?),
            StructType::CannotAccessVmDevice => Some(from.as_any_ref().downcast_ref::<CannotAccessVmDevice>()?),
            StructType::CannotAccessNetwork => Some(from.as_any_ref().downcast_ref::<CannotAccessNetwork>()?),
            StructType::DestinationSwitchFull => Some(from.as_any_ref().downcast_ref::<DestinationSwitchFull>()?),
            StructType::LegacyNetworkInterfaceInUse => Some(from.as_any_ref().downcast_ref::<LegacyNetworkInterfaceInUse>()?),
            StructType::VmOnConflictDvPort => Some(from.as_any_ref().downcast_ref::<VmOnConflictDvPort>()?),
            StructType::VmOnVirtualIntranet => Some(from.as_any_ref().downcast_ref::<VmOnVirtualIntranet>()?),
            StructType::CannotAccessVmDisk => Some(from.as_any_ref().downcast_ref::<CannotAccessVmDisk>()?),
            StructType::RdmPointsToInaccessibleDisk => Some(from.as_any_ref().downcast_ref::<RdmPointsToInaccessibleDisk>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::CannotAccessVmComponent => Ok(from.as_any_box().downcast::<CannotAccessVmComponent>()?),
            StructType::CannotAccessVmConfig => Ok(from.as_any_box().downcast::<CannotAccessVmConfig>()?),
            StructType::CannotAccessVmDevice => Ok(from.as_any_box().downcast::<CannotAccessVmDevice>()?),
            StructType::CannotAccessNetwork => Ok(from.as_any_box().downcast::<CannotAccessNetwork>()?),
            StructType::DestinationSwitchFull => Ok(from.as_any_box().downcast::<DestinationSwitchFull>()?),
            StructType::LegacyNetworkInterfaceInUse => Ok(from.as_any_box().downcast::<LegacyNetworkInterfaceInUse>()?),
            StructType::VmOnConflictDvPort => Ok(from.as_any_box().downcast::<VmOnConflictDvPort>()?),
            StructType::VmOnVirtualIntranet => Ok(from.as_any_box().downcast::<VmOnVirtualIntranet>()?),
            StructType::CannotAccessVmDisk => Ok(from.as_any_box().downcast::<CannotAccessVmDisk>()?),
            StructType::RdmPointsToInaccessibleDisk => Ok(from.as_any_box().downcast::<RdmPointsToInaccessibleDisk>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
