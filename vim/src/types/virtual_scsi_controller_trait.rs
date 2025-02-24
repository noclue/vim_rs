use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The VirtualSCSIController data object type represents
/// a SCSI controller in a virtual machine.
pub trait VirtualScsiControllerTrait : super::virtual_controller_trait::VirtualControllerTrait {
    /// All SCSI controllers support hot adding and removing of devices.
    /// 
    /// This
    /// support can't be toggled in the current implementation. Therefore, this
    /// option is ignored when reconfiguring a SCSI controller and is always set
    /// to "true" when reading an existing configuration.
    fn get_hot_add_remove(&self) -> Option<bool>;
    /// Mode for sharing the SCSI bus.
    /// 
    /// The modes are physicalSharing,
    /// virtualSharing, and noSharing. See the
    /// *Sharing*
    /// data object type for an explanation of these modes.
    fn get_shared_bus(&self) -> &super::enums::VirtualScsiSharingEnum;
    /// The unit number of the SCSI controller.
    /// 
    /// The SCSI controller sits on its
    /// own bus, so this field defines which slot the controller is using.
    fn get_scsi_ctlr_unit_number(&self) -> Option<i32>;
}
impl<'s> serde::Serialize for dyn VirtualScsiControllerTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualScsiControllerTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualScsiControllerVisitor)
            }
        }

struct VirtualScsiControllerVisitor;

impl<'de> de::Visitor<'de> for VirtualScsiControllerVisitor {
    type Value = Box<dyn VirtualScsiControllerTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualScsiControllerTrait JSON object with a _typeName field")
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

impl VirtualScsiControllerTrait for VirtualScsiController {
    fn get_hot_add_remove(&self) -> Option<bool> { self.hot_add_remove }
    fn get_shared_bus(&self) -> &super::enums::VirtualScsiSharingEnum { &self.shared_bus }
    fn get_scsi_ctlr_unit_number(&self) -> Option<i32> { self.scsi_ctlr_unit_number }
}
impl VirtualScsiControllerTrait for ParaVirtualScsiController {
    fn get_hot_add_remove(&self) -> Option<bool> { self.hot_add_remove }
    fn get_shared_bus(&self) -> &super::enums::VirtualScsiSharingEnum { &self.shared_bus }
    fn get_scsi_ctlr_unit_number(&self) -> Option<i32> { self.scsi_ctlr_unit_number }
}
impl VirtualScsiControllerTrait for VirtualBusLogicController {
    fn get_hot_add_remove(&self) -> Option<bool> { self.hot_add_remove }
    fn get_shared_bus(&self) -> &super::enums::VirtualScsiSharingEnum { &self.shared_bus }
    fn get_scsi_ctlr_unit_number(&self) -> Option<i32> { self.scsi_ctlr_unit_number }
}
impl VirtualScsiControllerTrait for VirtualLsiLogicController {
    fn get_hot_add_remove(&self) -> Option<bool> { self.hot_add_remove }
    fn get_shared_bus(&self) -> &super::enums::VirtualScsiSharingEnum { &self.shared_bus }
    fn get_scsi_ctlr_unit_number(&self) -> Option<i32> { self.scsi_ctlr_unit_number }
}
impl VirtualScsiControllerTrait for VirtualLsiLogicSasController {
    fn get_hot_add_remove(&self) -> Option<bool> { self.hot_add_remove }
    fn get_shared_bus(&self) -> &super::enums::VirtualScsiSharingEnum { &self.shared_bus }
    fn get_scsi_ctlr_unit_number(&self) -> Option<i32> { self.scsi_ctlr_unit_number }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualScsiControllerTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualScsiController => Some(from.as_any_ref().downcast_ref::<VirtualScsiController>()?),
            StructType::ParaVirtualScsiController => Some(from.as_any_ref().downcast_ref::<ParaVirtualScsiController>()?),
            StructType::VirtualBusLogicController => Some(from.as_any_ref().downcast_ref::<VirtualBusLogicController>()?),
            StructType::VirtualLsiLogicController => Some(from.as_any_ref().downcast_ref::<VirtualLsiLogicController>()?),
            StructType::VirtualLsiLogicSasController => Some(from.as_any_ref().downcast_ref::<VirtualLsiLogicSasController>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualScsiController => Ok(from.as_any_box().downcast::<VirtualScsiController>()?),
            StructType::ParaVirtualScsiController => Ok(from.as_any_box().downcast::<ParaVirtualScsiController>()?),
            StructType::VirtualBusLogicController => Ok(from.as_any_box().downcast::<VirtualBusLogicController>()?),
            StructType::VirtualLsiLogicController => Ok(from.as_any_box().downcast::<VirtualLsiLogicController>()?),
            StructType::VirtualLsiLogicSasController => Ok(from.as_any_box().downcast::<VirtualLsiLogicSasController>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
