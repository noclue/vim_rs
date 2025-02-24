use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type describes the bus adapter for
/// the host.
/// 
/// A host bus adapter (HBA) is a hardware
/// or software adapter that connects the host to storage devices.
pub trait HostHostBusAdapterTrait : super::data_object_trait::DataObjectTrait {
    /// The linkable identifier.
    fn get_key(&self) -> &Option<String>;
    /// The device name of host bus adapter.
    fn get_device(&self) -> &str;
    /// The host bus number.
    fn get_bus(&self) -> i32;
    /// The operational status of the adapter.
    /// 
    /// Valid values include "online",
    /// "offline", "unbound", and "unknown".
    fn get_status(&self) -> &str;
    /// The model name of the host bus adapter.
    fn get_model(&self) -> &str;
    /// The name of the driver.
    fn get_driver(&self) -> &Option<String>;
    /// The Peripheral Connect Interface (PCI) ID of the device
    /// representing the host bus adapter.
    fn get_pci(&self) -> &Option<String>;
    /// The type of protocol supported by the host bus adapter.
    /// 
    /// The list of supported values is described in
    /// *HostStorageProtocol_enum*.
    /// When unset, a default value of "scsi" is assumed.
    fn get_storage_protocol(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn HostHostBusAdapterTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostHostBusAdapterTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostHostBusAdapterVisitor)
            }
        }

struct HostHostBusAdapterVisitor;

impl<'de> de::Visitor<'de> for HostHostBusAdapterVisitor {
    type Value = Box<dyn HostHostBusAdapterTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostHostBusAdapterTrait JSON object with a _typeName field")
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

impl HostHostBusAdapterTrait for HostHostBusAdapter {
    fn get_key(&self) -> &Option<String> { &self.key }
    fn get_device(&self) -> &str { &self.device }
    fn get_bus(&self) -> i32 { self.bus }
    fn get_status(&self) -> &str { &self.status }
    fn get_model(&self) -> &str { &self.model }
    fn get_driver(&self) -> &Option<String> { &self.driver }
    fn get_pci(&self) -> &Option<String> { &self.pci }
    fn get_storage_protocol(&self) -> &Option<String> { &self.storage_protocol }
}
impl HostHostBusAdapterTrait for HostBlockHba {
    fn get_key(&self) -> &Option<String> { &self.key }
    fn get_device(&self) -> &str { &self.device }
    fn get_bus(&self) -> i32 { self.bus }
    fn get_status(&self) -> &str { &self.status }
    fn get_model(&self) -> &str { &self.model }
    fn get_driver(&self) -> &Option<String> { &self.driver }
    fn get_pci(&self) -> &Option<String> { &self.pci }
    fn get_storage_protocol(&self) -> &Option<String> { &self.storage_protocol }
}
impl HostHostBusAdapterTrait for HostFibreChannelHba {
    fn get_key(&self) -> &Option<String> { &self.key }
    fn get_device(&self) -> &str { &self.device }
    fn get_bus(&self) -> i32 { self.bus }
    fn get_status(&self) -> &str { &self.status }
    fn get_model(&self) -> &str { &self.model }
    fn get_driver(&self) -> &Option<String> { &self.driver }
    fn get_pci(&self) -> &Option<String> { &self.pci }
    fn get_storage_protocol(&self) -> &Option<String> { &self.storage_protocol }
}
impl HostHostBusAdapterTrait for HostFibreChannelOverEthernetHba {
    fn get_key(&self) -> &Option<String> { &self.key }
    fn get_device(&self) -> &str { &self.device }
    fn get_bus(&self) -> i32 { self.bus }
    fn get_status(&self) -> &str { &self.status }
    fn get_model(&self) -> &str { &self.model }
    fn get_driver(&self) -> &Option<String> { &self.driver }
    fn get_pci(&self) -> &Option<String> { &self.pci }
    fn get_storage_protocol(&self) -> &Option<String> { &self.storage_protocol }
}
impl HostHostBusAdapterTrait for HostInternetScsiHba {
    fn get_key(&self) -> &Option<String> { &self.key }
    fn get_device(&self) -> &str { &self.device }
    fn get_bus(&self) -> i32 { self.bus }
    fn get_status(&self) -> &str { &self.status }
    fn get_model(&self) -> &str { &self.model }
    fn get_driver(&self) -> &Option<String> { &self.driver }
    fn get_pci(&self) -> &Option<String> { &self.pci }
    fn get_storage_protocol(&self) -> &Option<String> { &self.storage_protocol }
}
impl HostHostBusAdapterTrait for HostParallelScsiHba {
    fn get_key(&self) -> &Option<String> { &self.key }
    fn get_device(&self) -> &str { &self.device }
    fn get_bus(&self) -> i32 { self.bus }
    fn get_status(&self) -> &str { &self.status }
    fn get_model(&self) -> &str { &self.model }
    fn get_driver(&self) -> &Option<String> { &self.driver }
    fn get_pci(&self) -> &Option<String> { &self.pci }
    fn get_storage_protocol(&self) -> &Option<String> { &self.storage_protocol }
}
impl HostHostBusAdapterTrait for HostPcieHba {
    fn get_key(&self) -> &Option<String> { &self.key }
    fn get_device(&self) -> &str { &self.device }
    fn get_bus(&self) -> i32 { self.bus }
    fn get_status(&self) -> &str { &self.status }
    fn get_model(&self) -> &str { &self.model }
    fn get_driver(&self) -> &Option<String> { &self.driver }
    fn get_pci(&self) -> &Option<String> { &self.pci }
    fn get_storage_protocol(&self) -> &Option<String> { &self.storage_protocol }
}
impl HostHostBusAdapterTrait for HostRdmaHba {
    fn get_key(&self) -> &Option<String> { &self.key }
    fn get_device(&self) -> &str { &self.device }
    fn get_bus(&self) -> i32 { self.bus }
    fn get_status(&self) -> &str { &self.status }
    fn get_model(&self) -> &str { &self.model }
    fn get_driver(&self) -> &Option<String> { &self.driver }
    fn get_pci(&self) -> &Option<String> { &self.pci }
    fn get_storage_protocol(&self) -> &Option<String> { &self.storage_protocol }
}
impl HostHostBusAdapterTrait for HostSerialAttachedHba {
    fn get_key(&self) -> &Option<String> { &self.key }
    fn get_device(&self) -> &str { &self.device }
    fn get_bus(&self) -> i32 { self.bus }
    fn get_status(&self) -> &str { &self.status }
    fn get_model(&self) -> &str { &self.model }
    fn get_driver(&self) -> &Option<String> { &self.driver }
    fn get_pci(&self) -> &Option<String> { &self.pci }
    fn get_storage_protocol(&self) -> &Option<String> { &self.storage_protocol }
}
impl HostHostBusAdapterTrait for HostTcpHba {
    fn get_key(&self) -> &Option<String> { &self.key }
    fn get_device(&self) -> &str { &self.device }
    fn get_bus(&self) -> i32 { self.bus }
    fn get_status(&self) -> &str { &self.status }
    fn get_model(&self) -> &str { &self.model }
    fn get_driver(&self) -> &Option<String> { &self.driver }
    fn get_pci(&self) -> &Option<String> { &self.pci }
    fn get_storage_protocol(&self) -> &Option<String> { &self.storage_protocol }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostHostBusAdapterTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostHostBusAdapter => Some(from.as_any_ref().downcast_ref::<HostHostBusAdapter>()?),
            StructType::HostBlockHba => Some(from.as_any_ref().downcast_ref::<HostBlockHba>()?),
            StructType::HostFibreChannelHba => Some(from.as_any_ref().downcast_ref::<HostFibreChannelHba>()?),
            StructType::HostFibreChannelOverEthernetHba => Some(from.as_any_ref().downcast_ref::<HostFibreChannelOverEthernetHba>()?),
            StructType::HostInternetScsiHba => Some(from.as_any_ref().downcast_ref::<HostInternetScsiHba>()?),
            StructType::HostParallelScsiHba => Some(from.as_any_ref().downcast_ref::<HostParallelScsiHba>()?),
            StructType::HostPcieHba => Some(from.as_any_ref().downcast_ref::<HostPcieHba>()?),
            StructType::HostRdmaHba => Some(from.as_any_ref().downcast_ref::<HostRdmaHba>()?),
            StructType::HostSerialAttachedHba => Some(from.as_any_ref().downcast_ref::<HostSerialAttachedHba>()?),
            StructType::HostTcpHba => Some(from.as_any_ref().downcast_ref::<HostTcpHba>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostHostBusAdapter => Ok(from.as_any_box().downcast::<HostHostBusAdapter>()?),
            StructType::HostBlockHba => Ok(from.as_any_box().downcast::<HostBlockHba>()?),
            StructType::HostFibreChannelHba => Ok(from.as_any_box().downcast::<HostFibreChannelHba>()?),
            StructType::HostFibreChannelOverEthernetHba => Ok(from.as_any_box().downcast::<HostFibreChannelOverEthernetHba>()?),
            StructType::HostInternetScsiHba => Ok(from.as_any_box().downcast::<HostInternetScsiHba>()?),
            StructType::HostParallelScsiHba => Ok(from.as_any_box().downcast::<HostParallelScsiHba>()?),
            StructType::HostPcieHba => Ok(from.as_any_box().downcast::<HostPcieHba>()?),
            StructType::HostRdmaHba => Ok(from.as_any_box().downcast::<HostRdmaHba>()?),
            StructType::HostSerialAttachedHba => Ok(from.as_any_box().downcast::<HostSerialAttachedHba>()?),
            StructType::HostTcpHba => Ok(from.as_any_box().downcast::<HostTcpHba>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
