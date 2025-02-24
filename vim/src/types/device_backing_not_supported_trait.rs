use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The device is backed by a backing type which is not supported
/// for this particular device.
/// 
/// If this fault is returned as a subfault of
/// DisallowedMigrationDeviceAttached, this indicates that although
/// this backing for the device may be supported on the destination
/// host, the hosts do not support the requested migration of the
/// virtual machine while using this device with this backing.
pub trait DeviceBackingNotSupportedTrait : super::device_not_supported_trait::DeviceNotSupportedTrait {
    /// The type of the backing.
    fn get_backing(&self) -> &str;
}
impl<'s> serde::Serialize for dyn DeviceBackingNotSupportedTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DeviceBackingNotSupportedTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DeviceBackingNotSupportedVisitor)
            }
        }

struct DeviceBackingNotSupportedVisitor;

impl<'de> de::Visitor<'de> for DeviceBackingNotSupportedVisitor {
    type Value = Box<dyn DeviceBackingNotSupportedTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DeviceBackingNotSupportedTrait JSON object with a _typeName field")
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

impl DeviceBackingNotSupportedTrait for DeviceBackingNotSupported {
    fn get_backing(&self) -> &str { &self.backing }
}
impl DeviceBackingNotSupportedTrait for DvPortNotSupported {
    fn get_backing(&self) -> &str { &self.backing }
}
impl DeviceBackingNotSupportedTrait for UnusedVirtualDiskBlocksNotScrubbed {
    fn get_backing(&self) -> &str { &self.backing }
}
impl DeviceBackingNotSupportedTrait for VirtualDiskBlocksNotFullyProvisioned {
    fn get_backing(&self) -> &str { &self.backing }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DeviceBackingNotSupportedTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DeviceBackingNotSupported => Some(from.as_any_ref().downcast_ref::<DeviceBackingNotSupported>()?),
            StructType::DvPortNotSupported => Some(from.as_any_ref().downcast_ref::<DvPortNotSupported>()?),
            StructType::UnusedVirtualDiskBlocksNotScrubbed => Some(from.as_any_ref().downcast_ref::<UnusedVirtualDiskBlocksNotScrubbed>()?),
            StructType::VirtualDiskBlocksNotFullyProvisioned => Some(from.as_any_ref().downcast_ref::<VirtualDiskBlocksNotFullyProvisioned>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DeviceBackingNotSupported => Ok(from.as_any_box().downcast::<DeviceBackingNotSupported>()?),
            StructType::DvPortNotSupported => Ok(from.as_any_box().downcast::<DvPortNotSupported>()?),
            StructType::UnusedVirtualDiskBlocksNotScrubbed => Ok(from.as_any_box().downcast::<UnusedVirtualDiskBlocksNotScrubbed>()?),
            StructType::VirtualDiskBlocksNotFullyProvisioned => Ok(from.as_any_box().downcast::<VirtualDiskBlocksNotFullyProvisioned>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
