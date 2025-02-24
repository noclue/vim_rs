use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object provides information about the state of PciPassthru
/// for all pci devices.
pub trait HostPciPassthruInfoTrait : super::data_object_trait::DataObjectTrait {
    /// The name ID of this PCI, composed of "bus:slot.function".
    fn get_id(&self) -> &str;
    /// Device which needs to be unclaimed by vmkernel (may be bridge)
    fn get_dependent_device(&self) -> &str;
    /// Whether passThru has been configured by the user
    fn get_passthru_enabled(&self) -> bool;
    /// Whether passThru is even possible for this device (decided by vmkctl)
    fn get_passthru_capable(&self) -> bool;
    /// Whether passThru is active for this device (meaning enabled + rebooted)
    fn get_passthru_active(&self) -> bool;
    /// The hardware label of this PCI device.
    /// 
    /// ***Since:*** vSphere API Release 7.0.2.0
    fn get_hardware_label(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn HostPciPassthruInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostPciPassthruInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostPciPassthruInfoVisitor)
            }
        }

struct HostPciPassthruInfoVisitor;

impl<'de> de::Visitor<'de> for HostPciPassthruInfoVisitor {
    type Value = Box<dyn HostPciPassthruInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostPciPassthruInfoTrait JSON object with a _typeName field")
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

impl HostPciPassthruInfoTrait for HostPciPassthruInfo {
    fn get_id(&self) -> &str { &self.id }
    fn get_dependent_device(&self) -> &str { &self.dependent_device }
    fn get_passthru_enabled(&self) -> bool { self.passthru_enabled }
    fn get_passthru_capable(&self) -> bool { self.passthru_capable }
    fn get_passthru_active(&self) -> bool { self.passthru_active }
    fn get_hardware_label(&self) -> &Option<String> { &self.hardware_label }
}
impl HostPciPassthruInfoTrait for HostSriovInfo {
    fn get_id(&self) -> &str { &self.id }
    fn get_dependent_device(&self) -> &str { &self.dependent_device }
    fn get_passthru_enabled(&self) -> bool { self.passthru_enabled }
    fn get_passthru_capable(&self) -> bool { self.passthru_capable }
    fn get_passthru_active(&self) -> bool { self.passthru_active }
    fn get_hardware_label(&self) -> &Option<String> { &self.hardware_label }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostPciPassthruInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostPciPassthruInfo => Some(from.as_any_ref().downcast_ref::<HostPciPassthruInfo>()?),
            StructType::HostSriovInfo => Some(from.as_any_ref().downcast_ref::<HostSriovInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostPciPassthruInfo => Ok(from.as_any_box().downcast::<HostPciPassthruInfo>()?),
            StructType::HostSriovInfo => Ok(from.as_any_box().downcast::<HostSriovInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
