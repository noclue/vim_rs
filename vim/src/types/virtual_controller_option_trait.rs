use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The VirtualControllerOption data object type contains information about
/// a virtual controller type.
pub trait VirtualControllerOptionTrait : super::virtual_device_option_trait::VirtualDeviceOptionTrait {
    /// The minimum and maximum number of devices this controller can control
    /// at run time.
    fn get_devices(&self) -> &IntOption;
    /// Array of supported device options for this controller.
    fn get_supported_device(&self) -> &Option<Vec<String>>;
}
impl<'s> serde::Serialize for dyn VirtualControllerOptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualControllerOptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualControllerOptionVisitor)
            }
        }

struct VirtualControllerOptionVisitor;

impl<'de> de::Visitor<'de> for VirtualControllerOptionVisitor {
    type Value = Box<dyn VirtualControllerOptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualControllerOptionTrait JSON object with a _typeName field")
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

impl VirtualControllerOptionTrait for VirtualControllerOption {
    fn get_devices(&self) -> &IntOption { &self.devices }
    fn get_supported_device(&self) -> &Option<Vec<String>> { &self.supported_device }
}
impl VirtualControllerOptionTrait for VirtualIdeControllerOption {
    fn get_devices(&self) -> &IntOption { &self.devices }
    fn get_supported_device(&self) -> &Option<Vec<String>> { &self.supported_device }
}
impl VirtualControllerOptionTrait for VirtualNvdimmControllerOption {
    fn get_devices(&self) -> &IntOption { &self.devices }
    fn get_supported_device(&self) -> &Option<Vec<String>> { &self.supported_device }
}
impl VirtualControllerOptionTrait for VirtualNvmeControllerOption {
    fn get_devices(&self) -> &IntOption { &self.devices }
    fn get_supported_device(&self) -> &Option<Vec<String>> { &self.supported_device }
}
impl VirtualControllerOptionTrait for VirtualPciControllerOption {
    fn get_devices(&self) -> &IntOption { &self.devices }
    fn get_supported_device(&self) -> &Option<Vec<String>> { &self.supported_device }
}
impl VirtualControllerOptionTrait for VirtualPs2ControllerOption {
    fn get_devices(&self) -> &IntOption { &self.devices }
    fn get_supported_device(&self) -> &Option<Vec<String>> { &self.supported_device }
}
impl VirtualControllerOptionTrait for VirtualSataControllerOption {
    fn get_devices(&self) -> &IntOption { &self.devices }
    fn get_supported_device(&self) -> &Option<Vec<String>> { &self.supported_device }
}
impl VirtualControllerOptionTrait for VirtualAhciControllerOption {
    fn get_devices(&self) -> &IntOption { &self.devices }
    fn get_supported_device(&self) -> &Option<Vec<String>> { &self.supported_device }
}
impl VirtualControllerOptionTrait for VirtualScsiControllerOption {
    fn get_devices(&self) -> &IntOption { &self.devices }
    fn get_supported_device(&self) -> &Option<Vec<String>> { &self.supported_device }
}
impl VirtualControllerOptionTrait for ParaVirtualScsiControllerOption {
    fn get_devices(&self) -> &IntOption { &self.devices }
    fn get_supported_device(&self) -> &Option<Vec<String>> { &self.supported_device }
}
impl VirtualControllerOptionTrait for VirtualBusLogicControllerOption {
    fn get_devices(&self) -> &IntOption { &self.devices }
    fn get_supported_device(&self) -> &Option<Vec<String>> { &self.supported_device }
}
impl VirtualControllerOptionTrait for VirtualLsiLogicControllerOption {
    fn get_devices(&self) -> &IntOption { &self.devices }
    fn get_supported_device(&self) -> &Option<Vec<String>> { &self.supported_device }
}
impl VirtualControllerOptionTrait for VirtualLsiLogicSasControllerOption {
    fn get_devices(&self) -> &IntOption { &self.devices }
    fn get_supported_device(&self) -> &Option<Vec<String>> { &self.supported_device }
}
impl VirtualControllerOptionTrait for VirtualSioControllerOption {
    fn get_devices(&self) -> &IntOption { &self.devices }
    fn get_supported_device(&self) -> &Option<Vec<String>> { &self.supported_device }
}
impl VirtualControllerOptionTrait for VirtualUsbControllerOption {
    fn get_devices(&self) -> &IntOption { &self.devices }
    fn get_supported_device(&self) -> &Option<Vec<String>> { &self.supported_device }
}
impl VirtualControllerOptionTrait for VirtualUsbxhciControllerOption {
    fn get_devices(&self) -> &IntOption { &self.devices }
    fn get_supported_device(&self) -> &Option<Vec<String>> { &self.supported_device }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualControllerOptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualControllerOption>()?),
            StructType::VirtualIdeControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualIdeControllerOption>()?),
            StructType::VirtualNvdimmControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualNvdimmControllerOption>()?),
            StructType::VirtualNvmeControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualNvmeControllerOption>()?),
            StructType::VirtualPciControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualPciControllerOption>()?),
            StructType::VirtualPs2ControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualPs2ControllerOption>()?),
            StructType::VirtualSataControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualSataControllerOption>()?),
            StructType::VirtualAhciControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualAhciControllerOption>()?),
            StructType::VirtualScsiControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualScsiControllerOption>()?),
            StructType::ParaVirtualScsiControllerOption => Some(from.as_any_ref().downcast_ref::<ParaVirtualScsiControllerOption>()?),
            StructType::VirtualBusLogicControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualBusLogicControllerOption>()?),
            StructType::VirtualLsiLogicControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualLsiLogicControllerOption>()?),
            StructType::VirtualLsiLogicSasControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualLsiLogicSasControllerOption>()?),
            StructType::VirtualSioControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualSioControllerOption>()?),
            StructType::VirtualUsbControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualUsbControllerOption>()?),
            StructType::VirtualUsbxhciControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualUsbxhciControllerOption>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualControllerOption => Ok(from.as_any_box().downcast::<VirtualControllerOption>()?),
            StructType::VirtualIdeControllerOption => Ok(from.as_any_box().downcast::<VirtualIdeControllerOption>()?),
            StructType::VirtualNvdimmControllerOption => Ok(from.as_any_box().downcast::<VirtualNvdimmControllerOption>()?),
            StructType::VirtualNvmeControllerOption => Ok(from.as_any_box().downcast::<VirtualNvmeControllerOption>()?),
            StructType::VirtualPciControllerOption => Ok(from.as_any_box().downcast::<VirtualPciControllerOption>()?),
            StructType::VirtualPs2ControllerOption => Ok(from.as_any_box().downcast::<VirtualPs2ControllerOption>()?),
            StructType::VirtualSataControllerOption => Ok(from.as_any_box().downcast::<VirtualSataControllerOption>()?),
            StructType::VirtualAhciControllerOption => Ok(from.as_any_box().downcast::<VirtualAhciControllerOption>()?),
            StructType::VirtualScsiControllerOption => Ok(from.as_any_box().downcast::<VirtualScsiControllerOption>()?),
            StructType::ParaVirtualScsiControllerOption => Ok(from.as_any_box().downcast::<ParaVirtualScsiControllerOption>()?),
            StructType::VirtualBusLogicControllerOption => Ok(from.as_any_box().downcast::<VirtualBusLogicControllerOption>()?),
            StructType::VirtualLsiLogicControllerOption => Ok(from.as_any_box().downcast::<VirtualLsiLogicControllerOption>()?),
            StructType::VirtualLsiLogicSasControllerOption => Ok(from.as_any_box().downcast::<VirtualLsiLogicSasControllerOption>()?),
            StructType::VirtualSioControllerOption => Ok(from.as_any_box().downcast::<VirtualSioControllerOption>()?),
            StructType::VirtualUsbControllerOption => Ok(from.as_any_box().downcast::<VirtualUsbControllerOption>()?),
            StructType::VirtualUsbxhciControllerOption => Ok(from.as_any_box().downcast::<VirtualUsbxhciControllerOption>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
