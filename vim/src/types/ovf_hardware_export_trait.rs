use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A common base class to host all the OvfLib Export Exceptions for hardware.
pub trait OvfHardwareExportTrait : super::ovf_export_trait::OvfExportTrait {
    /// The virtual device we are exporting to OVF
    fn get_device(&self) -> &Option<Box<dyn super::virtual_device_trait::VirtualDeviceTrait>>;
    /// The path to the VM containing the device.
    /// 
    /// This path shows the location of the VM in the vApp hierarchy, on the form:
    /// 
    /// /ParentVApp/ChildVApp/VMName
    fn get_vm_path(&self) -> &str;
}
impl<'s> serde::Serialize for dyn OvfHardwareExportTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn OvfHardwareExportTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(OvfHardwareExportVisitor)
            }
        }

struct OvfHardwareExportVisitor;

impl<'de> de::Visitor<'de> for OvfHardwareExportVisitor {
    type Value = Box<dyn OvfHardwareExportTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid OvfHardwareExportTrait JSON object with a _typeName field")
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

impl OvfHardwareExportTrait for OvfHardwareExport {
    fn get_device(&self) -> &Option<Box<dyn super::virtual_device_trait::VirtualDeviceTrait>> { &self.device }
    fn get_vm_path(&self) -> &str { &self.vm_path }
}
impl OvfHardwareExportTrait for OvfConnectedDevice {
    fn get_device(&self) -> &Option<Box<dyn super::virtual_device_trait::VirtualDeviceTrait>> { &self.device }
    fn get_vm_path(&self) -> &str { &self.vm_path }
}
impl OvfHardwareExportTrait for OvfConnectedDeviceFloppy {
    fn get_device(&self) -> &Option<Box<dyn super::virtual_device_trait::VirtualDeviceTrait>> { &self.device }
    fn get_vm_path(&self) -> &str { &self.vm_path }
}
impl OvfHardwareExportTrait for OvfConnectedDeviceIso {
    fn get_device(&self) -> &Option<Box<dyn super::virtual_device_trait::VirtualDeviceTrait>> { &self.device }
    fn get_vm_path(&self) -> &str { &self.vm_path }
}
impl OvfHardwareExportTrait for OvfUnableToExportDisk {
    fn get_device(&self) -> &Option<Box<dyn super::virtual_device_trait::VirtualDeviceTrait>> { &self.device }
    fn get_vm_path(&self) -> &str { &self.vm_path }
}
impl OvfHardwareExportTrait for OvfUnknownDeviceBacking {
    fn get_device(&self) -> &Option<Box<dyn super::virtual_device_trait::VirtualDeviceTrait>> { &self.device }
    fn get_vm_path(&self) -> &str { &self.vm_path }
}
impl OvfHardwareExportTrait for OvfUnsupportedDeviceExport {
    fn get_device(&self) -> &Option<Box<dyn super::virtual_device_trait::VirtualDeviceTrait>> { &self.device }
    fn get_vm_path(&self) -> &str { &self.vm_path }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn OvfHardwareExportTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfHardwareExport => Some(from.as_any_ref().downcast_ref::<OvfHardwareExport>()?),
            StructType::OvfConnectedDevice => Some(from.as_any_ref().downcast_ref::<OvfConnectedDevice>()?),
            StructType::OvfConnectedDeviceFloppy => Some(from.as_any_ref().downcast_ref::<OvfConnectedDeviceFloppy>()?),
            StructType::OvfConnectedDeviceIso => Some(from.as_any_ref().downcast_ref::<OvfConnectedDeviceIso>()?),
            StructType::OvfUnableToExportDisk => Some(from.as_any_ref().downcast_ref::<OvfUnableToExportDisk>()?),
            StructType::OvfUnknownDeviceBacking => Some(from.as_any_ref().downcast_ref::<OvfUnknownDeviceBacking>()?),
            StructType::OvfUnsupportedDeviceExport => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedDeviceExport>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfHardwareExport => Ok(from.as_any_box().downcast::<OvfHardwareExport>()?),
            StructType::OvfConnectedDevice => Ok(from.as_any_box().downcast::<OvfConnectedDevice>()?),
            StructType::OvfConnectedDeviceFloppy => Ok(from.as_any_box().downcast::<OvfConnectedDeviceFloppy>()?),
            StructType::OvfConnectedDeviceIso => Ok(from.as_any_box().downcast::<OvfConnectedDeviceIso>()?),
            StructType::OvfUnableToExportDisk => Ok(from.as_any_box().downcast::<OvfUnableToExportDisk>()?),
            StructType::OvfUnknownDeviceBacking => Ok(from.as_any_box().downcast::<OvfUnknownDeviceBacking>()?),
            StructType::OvfUnsupportedDeviceExport => Ok(from.as_any_box().downcast::<OvfUnsupportedDeviceExport>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
