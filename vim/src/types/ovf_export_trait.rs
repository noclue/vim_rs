use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A common base class to host all the Ovf Lib Export Exceptions.
pub trait OvfExportTrait : super::ovf_fault_trait::OvfFaultTrait {
}
impl<'s> serde::Serialize for dyn OvfExportTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn OvfExportTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(OvfExportVisitor)
            }
        }

struct OvfExportVisitor;

impl<'de> de::Visitor<'de> for OvfExportVisitor {
    type Value = Box<dyn OvfExportTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid OvfExportTrait JSON object with a _typeName field")
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

impl OvfExportTrait for OvfExport {
}
impl OvfExportTrait for ConnectedIso {
}
impl OvfExportTrait for OvfDuplicatedPropertyIdExport {
}
impl OvfExportTrait for OvfDuplicatedPropertyIdImport {
}
impl OvfExportTrait for OvfExportFailed {
}
impl OvfExportTrait for OvfHardwareExport {
}
impl OvfExportTrait for OvfConnectedDevice {
}
impl OvfExportTrait for OvfConnectedDeviceFloppy {
}
impl OvfExportTrait for OvfConnectedDeviceIso {
}
impl OvfExportTrait for OvfUnableToExportDisk {
}
impl OvfExportTrait for OvfUnknownDeviceBacking {
}
impl OvfExportTrait for OvfUnsupportedDeviceExport {
}
impl OvfExportTrait for OvfPropertyExport {
}
impl OvfExportTrait for OvfPropertyNetworkExport {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn OvfExportTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfExport => Some(from.as_any_ref().downcast_ref::<OvfExport>()?),
            StructType::ConnectedIso => Some(from.as_any_ref().downcast_ref::<ConnectedIso>()?),
            StructType::OvfDuplicatedPropertyIdExport => Some(from.as_any_ref().downcast_ref::<OvfDuplicatedPropertyIdExport>()?),
            StructType::OvfDuplicatedPropertyIdImport => Some(from.as_any_ref().downcast_ref::<OvfDuplicatedPropertyIdImport>()?),
            StructType::OvfExportFailed => Some(from.as_any_ref().downcast_ref::<OvfExportFailed>()?),
            StructType::OvfHardwareExport => Some(from.as_any_ref().downcast_ref::<OvfHardwareExport>()?),
            StructType::OvfConnectedDevice => Some(from.as_any_ref().downcast_ref::<OvfConnectedDevice>()?),
            StructType::OvfConnectedDeviceFloppy => Some(from.as_any_ref().downcast_ref::<OvfConnectedDeviceFloppy>()?),
            StructType::OvfConnectedDeviceIso => Some(from.as_any_ref().downcast_ref::<OvfConnectedDeviceIso>()?),
            StructType::OvfUnableToExportDisk => Some(from.as_any_ref().downcast_ref::<OvfUnableToExportDisk>()?),
            StructType::OvfUnknownDeviceBacking => Some(from.as_any_ref().downcast_ref::<OvfUnknownDeviceBacking>()?),
            StructType::OvfUnsupportedDeviceExport => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedDeviceExport>()?),
            StructType::OvfPropertyExport => Some(from.as_any_ref().downcast_ref::<OvfPropertyExport>()?),
            StructType::OvfPropertyNetworkExport => Some(from.as_any_ref().downcast_ref::<OvfPropertyNetworkExport>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfExport => Ok(from.as_any_box().downcast::<OvfExport>()?),
            StructType::ConnectedIso => Ok(from.as_any_box().downcast::<ConnectedIso>()?),
            StructType::OvfDuplicatedPropertyIdExport => Ok(from.as_any_box().downcast::<OvfDuplicatedPropertyIdExport>()?),
            StructType::OvfDuplicatedPropertyIdImport => Ok(from.as_any_box().downcast::<OvfDuplicatedPropertyIdImport>()?),
            StructType::OvfExportFailed => Ok(from.as_any_box().downcast::<OvfExportFailed>()?),
            StructType::OvfHardwareExport => Ok(from.as_any_box().downcast::<OvfHardwareExport>()?),
            StructType::OvfConnectedDevice => Ok(from.as_any_box().downcast::<OvfConnectedDevice>()?),
            StructType::OvfConnectedDeviceFloppy => Ok(from.as_any_box().downcast::<OvfConnectedDeviceFloppy>()?),
            StructType::OvfConnectedDeviceIso => Ok(from.as_any_box().downcast::<OvfConnectedDeviceIso>()?),
            StructType::OvfUnableToExportDisk => Ok(from.as_any_box().downcast::<OvfUnableToExportDisk>()?),
            StructType::OvfUnknownDeviceBacking => Ok(from.as_any_box().downcast::<OvfUnknownDeviceBacking>()?),
            StructType::OvfUnsupportedDeviceExport => Ok(from.as_any_box().downcast::<OvfUnsupportedDeviceExport>()?),
            StructType::OvfPropertyExport => Ok(from.as_any_box().downcast::<OvfPropertyExport>()?),
            StructType::OvfPropertyNetworkExport => Ok(from.as_any_box().downcast::<OvfPropertyNetworkExport>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
