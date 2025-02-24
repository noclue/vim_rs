use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// One of the virtual machine's devices uses a backing that is not accessible
/// on the host.
/// 
/// Following is a discussion of this fault's use in migration validation.
/// This is an error if the device is currently connected and a warning
/// otherwise. Devices that can be disconnected can only be connected if the virtual
/// machine is powered on.
/// 
/// The usage of this fault is slightly different if the backing of a device is
/// inherently host-local, and therefore not shared or globally named among
/// hosts. (Examples of such backings: physical CD-ROM drive, physical serial
/// port.) If a device with such a backing is currently connected, that will
/// be a migration error. If the device is disconnected, there will be a
/// warning if no backing with the same name exists on the destination host.
/// If the device is disconnected and a backing with the same name exists on
/// the destination host, this is neither a warning nor an error case, even
/// though the destination host's backing is not the same instance as the
/// source host's. It is assumed that use of the host-local backing is what is
/// desired for the device.
pub trait CannotAccessVmDeviceTrait : super::cannot_access_vm_component_trait::CannotAccessVmComponentTrait {
    /// The label of the device.
    fn get_device(&self) -> &str;
    /// The backing of the device.
    fn get_backing(&self) -> &str;
    /// The connected/disconnected state of the device.
    fn get_connected(&self) -> bool;
}
impl<'s> serde::Serialize for dyn CannotAccessVmDeviceTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn CannotAccessVmDeviceTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(CannotAccessVmDeviceVisitor)
            }
        }

struct CannotAccessVmDeviceVisitor;

impl<'de> de::Visitor<'de> for CannotAccessVmDeviceVisitor {
    type Value = Box<dyn CannotAccessVmDeviceTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid CannotAccessVmDeviceTrait JSON object with a _typeName field")
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

impl CannotAccessVmDeviceTrait for CannotAccessVmDevice {
    fn get_device(&self) -> &str { &self.device }
    fn get_backing(&self) -> &str { &self.backing }
    fn get_connected(&self) -> bool { self.connected }
}
impl CannotAccessVmDeviceTrait for CannotAccessNetwork {
    fn get_device(&self) -> &str { &self.device }
    fn get_backing(&self) -> &str { &self.backing }
    fn get_connected(&self) -> bool { self.connected }
}
impl CannotAccessVmDeviceTrait for DestinationSwitchFull {
    fn get_device(&self) -> &str { &self.device }
    fn get_backing(&self) -> &str { &self.backing }
    fn get_connected(&self) -> bool { self.connected }
}
impl CannotAccessVmDeviceTrait for LegacyNetworkInterfaceInUse {
    fn get_device(&self) -> &str { &self.device }
    fn get_backing(&self) -> &str { &self.backing }
    fn get_connected(&self) -> bool { self.connected }
}
impl CannotAccessVmDeviceTrait for VmOnConflictDvPort {
    fn get_device(&self) -> &str { &self.device }
    fn get_backing(&self) -> &str { &self.backing }
    fn get_connected(&self) -> bool { self.connected }
}
impl CannotAccessVmDeviceTrait for VmOnVirtualIntranet {
    fn get_device(&self) -> &str { &self.device }
    fn get_backing(&self) -> &str { &self.backing }
    fn get_connected(&self) -> bool { self.connected }
}
impl CannotAccessVmDeviceTrait for CannotAccessVmDisk {
    fn get_device(&self) -> &str { &self.device }
    fn get_backing(&self) -> &str { &self.backing }
    fn get_connected(&self) -> bool { self.connected }
}
impl CannotAccessVmDeviceTrait for RdmPointsToInaccessibleDisk {
    fn get_device(&self) -> &str { &self.device }
    fn get_backing(&self) -> &str { &self.backing }
    fn get_connected(&self) -> bool { self.connected }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn CannotAccessVmDeviceTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
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
