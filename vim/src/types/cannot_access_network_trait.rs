use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A network associated with the virtual machine is not accessible.
/// 
/// If returned as
/// part of migration checks, this is an error if either of the following is true,
/// a warning otherwise:
/// - The virtual ethernet card device backing is a distributed virtual switch,
///   of which the destination host is not a member
/// - The virtual ethernet card device backing is a standard network and the
///   the device is connected
pub trait CannotAccessNetworkTrait : super::cannot_access_vm_device_trait::CannotAccessVmDeviceTrait {
    /// A reference to the network that cannot be accessed
    /// 
    /// Refers instance of *Network*.
    fn get_network(&self) -> &Option<ManagedObjectReference>;
}
impl<'s> serde::Serialize for dyn CannotAccessNetworkTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn CannotAccessNetworkTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(CannotAccessNetworkVisitor)
            }
        }

struct CannotAccessNetworkVisitor;

impl<'de> de::Visitor<'de> for CannotAccessNetworkVisitor {
    type Value = Box<dyn CannotAccessNetworkTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid CannotAccessNetworkTrait JSON object with a _typeName field")
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

impl CannotAccessNetworkTrait for CannotAccessNetwork {
    fn get_network(&self) -> &Option<ManagedObjectReference> { &self.network }
}
impl CannotAccessNetworkTrait for DestinationSwitchFull {
    fn get_network(&self) -> &Option<ManagedObjectReference> { &self.network }
}
impl CannotAccessNetworkTrait for LegacyNetworkInterfaceInUse {
    fn get_network(&self) -> &Option<ManagedObjectReference> { &self.network }
}
impl CannotAccessNetworkTrait for VmOnConflictDvPort {
    fn get_network(&self) -> &Option<ManagedObjectReference> { &self.network }
}
impl CannotAccessNetworkTrait for VmOnVirtualIntranet {
    fn get_network(&self) -> &Option<ManagedObjectReference> { &self.network }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn CannotAccessNetworkTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::CannotAccessNetwork => Some(from.as_any_ref().downcast_ref::<CannotAccessNetwork>()?),
            StructType::DestinationSwitchFull => Some(from.as_any_ref().downcast_ref::<DestinationSwitchFull>()?),
            StructType::LegacyNetworkInterfaceInUse => Some(from.as_any_ref().downcast_ref::<LegacyNetworkInterfaceInUse>()?),
            StructType::VmOnConflictDvPort => Some(from.as_any_ref().downcast_ref::<VmOnConflictDvPort>()?),
            StructType::VmOnVirtualIntranet => Some(from.as_any_ref().downcast_ref::<VmOnVirtualIntranet>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::CannotAccessNetwork => Ok(from.as_any_box().downcast::<CannotAccessNetwork>()?),
            StructType::DestinationSwitchFull => Ok(from.as_any_box().downcast::<DestinationSwitchFull>()?),
            StructType::LegacyNetworkInterfaceInUse => Ok(from.as_any_box().downcast::<LegacyNetworkInterfaceInUse>()?),
            StructType::VmOnConflictDvPort => Ok(from.as_any_box().downcast::<VmOnConflictDvPort>()?),
            StructType::VmOnVirtualIntranet => Ok(from.as_any_box().downcast::<VmOnVirtualIntranet>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
