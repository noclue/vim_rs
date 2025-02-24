use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type defines a device on the host.
pub trait HostDeviceTrait : super::data_object_trait::DataObjectTrait {
    /// The name of the device on the host.
    /// 
    /// For example,
    /// /dev/cdrom or \\\\serverX\\device\_name.
    fn get_device_name(&self) -> &str;
    /// Device type when available:
    /// floppy, mouse, cdrom, disk, scsi device, or adapter.
    fn get_device_type(&self) -> &str;
}
impl<'s> serde::Serialize for dyn HostDeviceTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostDeviceTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostDeviceVisitor)
            }
        }

struct HostDeviceVisitor;

impl<'de> de::Visitor<'de> for HostDeviceVisitor {
    type Value = Box<dyn HostDeviceTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostDeviceTrait JSON object with a _typeName field")
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

impl HostDeviceTrait for HostDevice {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_device_type(&self) -> &str { &self.device_type }
}
impl HostDeviceTrait for ScsiLun {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_device_type(&self) -> &str { &self.device_type }
}
impl HostDeviceTrait for HostScsiDisk {
    fn get_device_name(&self) -> &str { &self.device_name }
    fn get_device_type(&self) -> &str { &self.device_type }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostDeviceTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostDevice => Some(from.as_any_ref().downcast_ref::<HostDevice>()?),
            StructType::ScsiLun => Some(from.as_any_ref().downcast_ref::<ScsiLun>()?),
            StructType::HostScsiDisk => Some(from.as_any_ref().downcast_ref::<HostScsiDisk>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostDevice => Ok(from.as_any_box().downcast::<HostDevice>()?),
            StructType::ScsiLun => Ok(from.as_any_box().downcast::<ScsiLun>()?),
            StructType::HostScsiDisk => Ok(from.as_any_box().downcast::<HostScsiDisk>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
