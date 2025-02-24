use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object provides information about the state of PciPassthru
/// for all pci devices.
pub trait HostPciPassthruConfigTrait : super::data_object_trait::DataObjectTrait {
    /// The name ID of this PCI, composed of "bus:slot.function".
    fn get_id(&self) -> &str;
    /// Whether passThru has been configured for this device
    fn get_passthru_enabled(&self) -> bool;
    /// Whether the passThru config should take effect without rebooting ESX.
    /// 
    /// When unset, the behavior will be determined automatically
    /// based on *HostCapability.deviceRebindWithoutRebootSupported*.
    /// If the configuration can be applied immediately, it
    /// will be, otherwise the changes will take effect after reboot.
    fn get_apply_now(&self) -> Option<bool>;
    /// The hardware label of the this PCI device.
    /// 
    /// ***Since:*** vSphere API Release 7.0.2.0
    fn get_hardware_label(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn HostPciPassthruConfigTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostPciPassthruConfigTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostPciPassthruConfigVisitor)
            }
        }

struct HostPciPassthruConfigVisitor;

impl<'de> de::Visitor<'de> for HostPciPassthruConfigVisitor {
    type Value = Box<dyn HostPciPassthruConfigTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostPciPassthruConfigTrait JSON object with a _typeName field")
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

impl HostPciPassthruConfigTrait for HostPciPassthruConfig {
    fn get_id(&self) -> &str { &self.id }
    fn get_passthru_enabled(&self) -> bool { self.passthru_enabled }
    fn get_apply_now(&self) -> Option<bool> { self.apply_now }
    fn get_hardware_label(&self) -> &Option<String> { &self.hardware_label }
}
impl HostPciPassthruConfigTrait for HostSriovConfig {
    fn get_id(&self) -> &str { &self.id }
    fn get_passthru_enabled(&self) -> bool { self.passthru_enabled }
    fn get_apply_now(&self) -> Option<bool> { self.apply_now }
    fn get_hardware_label(&self) -> &Option<String> { &self.hardware_label }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostPciPassthruConfigTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostPciPassthruConfig => Some(from.as_any_ref().downcast_ref::<HostPciPassthruConfig>()?),
            StructType::HostSriovConfig => Some(from.as_any_ref().downcast_ref::<HostSriovConfig>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostPciPassthruConfig => Ok(from.as_any_box().downcast::<HostPciPassthruConfig>()?),
            StructType::HostSriovConfig => Ok(from.as_any_box().downcast::<HostSriovConfig>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
