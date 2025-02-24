use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The virtual machine uses a device type that is not supported on the
/// host.
/// 
/// If this fault is returned as a subfault of
/// *DisallowedMigrationDeviceAttached*, this indicates that although
/// this device may be supported on the destination host, the hosts do
/// not support the requested migration of the virtual machine while
/// using this device.
pub trait DeviceNotSupportedTrait : super::virtual_hardware_compatibility_issue_trait::VirtualHardwareCompatibilityIssueTrait {
    /// The label of the device.
    fn get_device(&self) -> &str;
    /// The specific reason why the device is not supported.
    /// 
    /// Values should come from *DeviceNotSupportedReason_enum*.
    /// This might not be set if we're not sure of the reason, or
    /// if this doesn't make sense in the context. For example,
    /// in the *DisallowedMigrationDeviceAttached* context
    /// we already know the problem.
    fn get_reason(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn DeviceNotSupportedTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DeviceNotSupportedTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DeviceNotSupportedVisitor)
            }
        }

struct DeviceNotSupportedVisitor;

impl<'de> de::Visitor<'de> for DeviceNotSupportedVisitor {
    type Value = Box<dyn DeviceNotSupportedTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DeviceNotSupportedTrait JSON object with a _typeName field")
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

impl DeviceNotSupportedTrait for DeviceNotSupported {
    fn get_device(&self) -> &str { &self.device }
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl DeviceNotSupportedTrait for DeviceBackingNotSupported {
    fn get_device(&self) -> &str { &self.device }
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl DeviceNotSupportedTrait for DvPortNotSupported {
    fn get_device(&self) -> &str { &self.device }
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl DeviceNotSupportedTrait for UnusedVirtualDiskBlocksNotScrubbed {
    fn get_device(&self) -> &str { &self.device }
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl DeviceNotSupportedTrait for VirtualDiskBlocksNotFullyProvisioned {
    fn get_device(&self) -> &str { &self.device }
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl DeviceNotSupportedTrait for DeviceControllerNotSupported {
    fn get_device(&self) -> &str { &self.device }
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl DeviceNotSupportedTrait for DigestNotSupported {
    fn get_device(&self) -> &str { &self.device }
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl DeviceNotSupportedTrait for FileBackedPortNotSupported {
    fn get_device(&self) -> &str { &self.device }
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl DeviceNotSupportedTrait for MultiWriterNotSupported {
    fn get_device(&self) -> &str { &self.device }
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl DeviceNotSupportedTrait for NonPersistentDisksNotSupported {
    fn get_device(&self) -> &str { &self.device }
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl DeviceNotSupportedTrait for RdmNotSupported {
    fn get_device(&self) -> &str { &self.device }
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl DeviceNotSupportedTrait for PhysCompatRdmNotSupported {
    fn get_device(&self) -> &str { &self.device }
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl DeviceNotSupportedTrait for RawDiskNotSupported {
    fn get_device(&self) -> &str { &self.device }
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl DeviceNotSupportedTrait for RemoteDeviceNotSupported {
    fn get_device(&self) -> &str { &self.device }
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl DeviceNotSupportedTrait for SharedBusControllerNotSupported {
    fn get_device(&self) -> &str { &self.device }
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl DeviceNotSupportedTrait for VmiNotSupported {
    fn get_device(&self) -> &str { &self.device }
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl DeviceNotSupportedTrait for VirtualDiskModeNotSupported {
    fn get_device(&self) -> &str { &self.device }
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl DeviceNotSupportedTrait for VirtualEthernetCardNotSupported {
    fn get_device(&self) -> &str { &self.device }
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DeviceNotSupportedTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DeviceNotSupported => Some(from.as_any_ref().downcast_ref::<DeviceNotSupported>()?),
            StructType::DeviceBackingNotSupported => Some(from.as_any_ref().downcast_ref::<DeviceBackingNotSupported>()?),
            StructType::DvPortNotSupported => Some(from.as_any_ref().downcast_ref::<DvPortNotSupported>()?),
            StructType::UnusedVirtualDiskBlocksNotScrubbed => Some(from.as_any_ref().downcast_ref::<UnusedVirtualDiskBlocksNotScrubbed>()?),
            StructType::VirtualDiskBlocksNotFullyProvisioned => Some(from.as_any_ref().downcast_ref::<VirtualDiskBlocksNotFullyProvisioned>()?),
            StructType::DeviceControllerNotSupported => Some(from.as_any_ref().downcast_ref::<DeviceControllerNotSupported>()?),
            StructType::DigestNotSupported => Some(from.as_any_ref().downcast_ref::<DigestNotSupported>()?),
            StructType::FileBackedPortNotSupported => Some(from.as_any_ref().downcast_ref::<FileBackedPortNotSupported>()?),
            StructType::MultiWriterNotSupported => Some(from.as_any_ref().downcast_ref::<MultiWriterNotSupported>()?),
            StructType::NonPersistentDisksNotSupported => Some(from.as_any_ref().downcast_ref::<NonPersistentDisksNotSupported>()?),
            StructType::RdmNotSupported => Some(from.as_any_ref().downcast_ref::<RdmNotSupported>()?),
            StructType::PhysCompatRdmNotSupported => Some(from.as_any_ref().downcast_ref::<PhysCompatRdmNotSupported>()?),
            StructType::RawDiskNotSupported => Some(from.as_any_ref().downcast_ref::<RawDiskNotSupported>()?),
            StructType::RemoteDeviceNotSupported => Some(from.as_any_ref().downcast_ref::<RemoteDeviceNotSupported>()?),
            StructType::SharedBusControllerNotSupported => Some(from.as_any_ref().downcast_ref::<SharedBusControllerNotSupported>()?),
            StructType::VmiNotSupported => Some(from.as_any_ref().downcast_ref::<VmiNotSupported>()?),
            StructType::VirtualDiskModeNotSupported => Some(from.as_any_ref().downcast_ref::<VirtualDiskModeNotSupported>()?),
            StructType::VirtualEthernetCardNotSupported => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardNotSupported>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DeviceNotSupported => Ok(from.as_any_box().downcast::<DeviceNotSupported>()?),
            StructType::DeviceBackingNotSupported => Ok(from.as_any_box().downcast::<DeviceBackingNotSupported>()?),
            StructType::DvPortNotSupported => Ok(from.as_any_box().downcast::<DvPortNotSupported>()?),
            StructType::UnusedVirtualDiskBlocksNotScrubbed => Ok(from.as_any_box().downcast::<UnusedVirtualDiskBlocksNotScrubbed>()?),
            StructType::VirtualDiskBlocksNotFullyProvisioned => Ok(from.as_any_box().downcast::<VirtualDiskBlocksNotFullyProvisioned>()?),
            StructType::DeviceControllerNotSupported => Ok(from.as_any_box().downcast::<DeviceControllerNotSupported>()?),
            StructType::DigestNotSupported => Ok(from.as_any_box().downcast::<DigestNotSupported>()?),
            StructType::FileBackedPortNotSupported => Ok(from.as_any_box().downcast::<FileBackedPortNotSupported>()?),
            StructType::MultiWriterNotSupported => Ok(from.as_any_box().downcast::<MultiWriterNotSupported>()?),
            StructType::NonPersistentDisksNotSupported => Ok(from.as_any_box().downcast::<NonPersistentDisksNotSupported>()?),
            StructType::RdmNotSupported => Ok(from.as_any_box().downcast::<RdmNotSupported>()?),
            StructType::PhysCompatRdmNotSupported => Ok(from.as_any_box().downcast::<PhysCompatRdmNotSupported>()?),
            StructType::RawDiskNotSupported => Ok(from.as_any_box().downcast::<RawDiskNotSupported>()?),
            StructType::RemoteDeviceNotSupported => Ok(from.as_any_box().downcast::<RemoteDeviceNotSupported>()?),
            StructType::SharedBusControllerNotSupported => Ok(from.as_any_box().downcast::<SharedBusControllerNotSupported>()?),
            StructType::VmiNotSupported => Ok(from.as_any_box().downcast::<VmiNotSupported>()?),
            StructType::VirtualDiskModeNotSupported => Ok(from.as_any_box().downcast::<VirtualDiskModeNotSupported>()?),
            StructType::VirtualEthernetCardNotSupported => Ok(from.as_any_box().downcast::<VirtualEthernetCardNotSupported>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
