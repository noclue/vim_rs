use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// VirtualController is the base data object type for a device controller in
/// a virtual machine.
/// 
/// VirtualController extends
/// *VirtualDevice* to inherit
/// general information about a controller (such as name and description), and to allow
/// controllers to appear in a generic list of virtual devices.
pub trait VirtualControllerTrait : super::virtual_device_trait::VirtualDeviceTrait {
    /// Bus number associated with this controller.
    fn get_bus_number(&self) -> i32;
    /// List of devices currently controlled by this controller.
    /// 
    /// Each entry contains the *VirtualDevice.key* property of the
    /// corresponding device object.
    fn get_device(&self) -> &Option<Vec<i32>>;
}
impl<'s> serde::Serialize for dyn VirtualControllerTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualControllerTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualControllerVisitor)
            }
        }

struct VirtualControllerVisitor;

impl<'de> de::Visitor<'de> for VirtualControllerVisitor {
    type Value = Box<dyn VirtualControllerTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualControllerTrait JSON object with a _typeName field")
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

impl VirtualControllerTrait for VirtualController {
    fn get_bus_number(&self) -> i32 { self.bus_number }
    fn get_device(&self) -> &Option<Vec<i32>> { &self.device }
}
impl VirtualControllerTrait for VirtualIdeController {
    fn get_bus_number(&self) -> i32 { self.bus_number }
    fn get_device(&self) -> &Option<Vec<i32>> { &self.device }
}
impl VirtualControllerTrait for VirtualNvdimmController {
    fn get_bus_number(&self) -> i32 { self.bus_number }
    fn get_device(&self) -> &Option<Vec<i32>> { &self.device }
}
impl VirtualControllerTrait for VirtualNvmeController {
    fn get_bus_number(&self) -> i32 { self.bus_number }
    fn get_device(&self) -> &Option<Vec<i32>> { &self.device }
}
impl VirtualControllerTrait for VirtualPciController {
    fn get_bus_number(&self) -> i32 { self.bus_number }
    fn get_device(&self) -> &Option<Vec<i32>> { &self.device }
}
impl VirtualControllerTrait for VirtualPs2Controller {
    fn get_bus_number(&self) -> i32 { self.bus_number }
    fn get_device(&self) -> &Option<Vec<i32>> { &self.device }
}
impl VirtualControllerTrait for VirtualSataController {
    fn get_bus_number(&self) -> i32 { self.bus_number }
    fn get_device(&self) -> &Option<Vec<i32>> { &self.device }
}
impl VirtualControllerTrait for VirtualAhciController {
    fn get_bus_number(&self) -> i32 { self.bus_number }
    fn get_device(&self) -> &Option<Vec<i32>> { &self.device }
}
impl VirtualControllerTrait for VirtualScsiController {
    fn get_bus_number(&self) -> i32 { self.bus_number }
    fn get_device(&self) -> &Option<Vec<i32>> { &self.device }
}
impl VirtualControllerTrait for ParaVirtualScsiController {
    fn get_bus_number(&self) -> i32 { self.bus_number }
    fn get_device(&self) -> &Option<Vec<i32>> { &self.device }
}
impl VirtualControllerTrait for VirtualBusLogicController {
    fn get_bus_number(&self) -> i32 { self.bus_number }
    fn get_device(&self) -> &Option<Vec<i32>> { &self.device }
}
impl VirtualControllerTrait for VirtualLsiLogicController {
    fn get_bus_number(&self) -> i32 { self.bus_number }
    fn get_device(&self) -> &Option<Vec<i32>> { &self.device }
}
impl VirtualControllerTrait for VirtualLsiLogicSasController {
    fn get_bus_number(&self) -> i32 { self.bus_number }
    fn get_device(&self) -> &Option<Vec<i32>> { &self.device }
}
impl VirtualControllerTrait for VirtualSioController {
    fn get_bus_number(&self) -> i32 { self.bus_number }
    fn get_device(&self) -> &Option<Vec<i32>> { &self.device }
}
impl VirtualControllerTrait for VirtualUsbController {
    fn get_bus_number(&self) -> i32 { self.bus_number }
    fn get_device(&self) -> &Option<Vec<i32>> { &self.device }
}
impl VirtualControllerTrait for VirtualUsbxhciController {
    fn get_bus_number(&self) -> i32 { self.bus_number }
    fn get_device(&self) -> &Option<Vec<i32>> { &self.device }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualControllerTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualController => Some(from.as_any_ref().downcast_ref::<VirtualController>()?),
            StructType::VirtualIdeController => Some(from.as_any_ref().downcast_ref::<VirtualIdeController>()?),
            StructType::VirtualNvdimmController => Some(from.as_any_ref().downcast_ref::<VirtualNvdimmController>()?),
            StructType::VirtualNvmeController => Some(from.as_any_ref().downcast_ref::<VirtualNvmeController>()?),
            StructType::VirtualPciController => Some(from.as_any_ref().downcast_ref::<VirtualPciController>()?),
            StructType::VirtualPs2Controller => Some(from.as_any_ref().downcast_ref::<VirtualPs2Controller>()?),
            StructType::VirtualSataController => Some(from.as_any_ref().downcast_ref::<VirtualSataController>()?),
            StructType::VirtualAhciController => Some(from.as_any_ref().downcast_ref::<VirtualAhciController>()?),
            StructType::VirtualScsiController => Some(from.as_any_ref().downcast_ref::<VirtualScsiController>()?),
            StructType::ParaVirtualScsiController => Some(from.as_any_ref().downcast_ref::<ParaVirtualScsiController>()?),
            StructType::VirtualBusLogicController => Some(from.as_any_ref().downcast_ref::<VirtualBusLogicController>()?),
            StructType::VirtualLsiLogicController => Some(from.as_any_ref().downcast_ref::<VirtualLsiLogicController>()?),
            StructType::VirtualLsiLogicSasController => Some(from.as_any_ref().downcast_ref::<VirtualLsiLogicSasController>()?),
            StructType::VirtualSioController => Some(from.as_any_ref().downcast_ref::<VirtualSioController>()?),
            StructType::VirtualUsbController => Some(from.as_any_ref().downcast_ref::<VirtualUsbController>()?),
            StructType::VirtualUsbxhciController => Some(from.as_any_ref().downcast_ref::<VirtualUsbxhciController>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualController => Ok(from.as_any_box().downcast::<VirtualController>()?),
            StructType::VirtualIdeController => Ok(from.as_any_box().downcast::<VirtualIdeController>()?),
            StructType::VirtualNvdimmController => Ok(from.as_any_box().downcast::<VirtualNvdimmController>()?),
            StructType::VirtualNvmeController => Ok(from.as_any_box().downcast::<VirtualNvmeController>()?),
            StructType::VirtualPciController => Ok(from.as_any_box().downcast::<VirtualPciController>()?),
            StructType::VirtualPs2Controller => Ok(from.as_any_box().downcast::<VirtualPs2Controller>()?),
            StructType::VirtualSataController => Ok(from.as_any_box().downcast::<VirtualSataController>()?),
            StructType::VirtualAhciController => Ok(from.as_any_box().downcast::<VirtualAhciController>()?),
            StructType::VirtualScsiController => Ok(from.as_any_box().downcast::<VirtualScsiController>()?),
            StructType::ParaVirtualScsiController => Ok(from.as_any_box().downcast::<ParaVirtualScsiController>()?),
            StructType::VirtualBusLogicController => Ok(from.as_any_box().downcast::<VirtualBusLogicController>()?),
            StructType::VirtualLsiLogicController => Ok(from.as_any_box().downcast::<VirtualLsiLogicController>()?),
            StructType::VirtualLsiLogicSasController => Ok(from.as_any_box().downcast::<VirtualLsiLogicSasController>()?),
            StructType::VirtualSioController => Ok(from.as_any_box().downcast::<VirtualSioController>()?),
            StructType::VirtualUsbController => Ok(from.as_any_box().downcast::<VirtualUsbController>()?),
            StructType::VirtualUsbxhciController => Ok(from.as_any_box().downcast::<VirtualUsbxhciController>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
