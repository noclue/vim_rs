use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The VirtualSCSIControllerOption data object type contains the options
/// for a virtual SCSI controller defined by the
/// *VirtualSCSIController*
/// data object type.
pub trait VirtualScsiControllerOptionTrait : super::virtual_controller_option_trait::VirtualControllerOptionTrait {
    /// Three properties (numSCSIDisks.min, numSCSIDisks.max, and
    /// numSCSIDisks.defaultValue) define the minimum, maximum, and default
    /// number of SCSI VirtualDisk instances available at any given time in the
    /// SCSI controller.
    /// 
    /// The number of SCSI VirtualDisk instances is
    /// also limited by the number of available slots in the SCSI controller.
    fn get_num_scsi_disks(&self) -> &IntOption;
    /// Three properties (numSCSICdroms.min, numSCSICdroms.max, and
    /// numSCSICdroms.defaultValue) define the minimum, maximum, and default
    /// number of SCSI VirtualCdrom instances available
    /// in the SCSI controller.
    /// 
    /// The number of SCSI VirtualCdrom instances is
    /// also limited by the number of available slots in the SCSI controller.
    fn get_num_scsi_cdroms(&self) -> &IntOption;
    /// Three properties (numSCSIPassthrough.min, numSCSIPassthrough.max, and
    /// numSCSIPassthrough.defaultValue) define the minimum, maximum, and
    /// default number of VirtualSCSIPassthrough instances available
    /// have at any given time in the SCSI controller.
    /// 
    /// The number of
    /// VirtualSCSIPassthrough instances is also limited by the number of
    /// available slots in the SCSI controller.
    fn get_num_scsi_passthrough(&self) -> &IntOption;
    /// Supported shared bus modes.
    fn get_sharing(&self) -> &Vec<super::enums::VirtualScsiSharingEnum>;
    /// Index into sharing array specifying the default value.
    fn get_default_shared_index(&self) -> i32;
    /// All SCSI controllers support hot adding and removing of devices.
    /// 
    /// This
    /// support can't be toggled in the current implementation. Therefore, this
    /// option is ignored when reconfiguring a SCSI controller and is always set
    /// to "true" when reading an existing configuration.
    fn get_hot_add_remove(&self) -> &BoolOption;
    /// The unit number of the SCSI controller.
    /// 
    /// The SCSI controller sits on its
    /// own bus, so that this field defines which slot the controller will use.
    fn get_scsi_ctlr_unit_number(&self) -> i32;
}
impl<'s> serde::Serialize for dyn VirtualScsiControllerOptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualScsiControllerOptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualScsiControllerOptionVisitor)
            }
        }

struct VirtualScsiControllerOptionVisitor;

impl<'de> de::Visitor<'de> for VirtualScsiControllerOptionVisitor {
    type Value = Box<dyn VirtualScsiControllerOptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualScsiControllerOptionTrait JSON object with a _typeName field")
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

impl VirtualScsiControllerOptionTrait for VirtualScsiControllerOption {
    fn get_num_scsi_disks(&self) -> &IntOption { &self.num_scsi_disks }
    fn get_num_scsi_cdroms(&self) -> &IntOption { &self.num_scsi_cdroms }
    fn get_num_scsi_passthrough(&self) -> &IntOption { &self.num_scsi_passthrough }
    fn get_sharing(&self) -> &Vec<super::enums::VirtualScsiSharingEnum> { &self.sharing }
    fn get_default_shared_index(&self) -> i32 { self.default_shared_index }
    fn get_hot_add_remove(&self) -> &BoolOption { &self.hot_add_remove }
    fn get_scsi_ctlr_unit_number(&self) -> i32 { self.scsi_ctlr_unit_number }
}
impl VirtualScsiControllerOptionTrait for ParaVirtualScsiControllerOption {
    fn get_num_scsi_disks(&self) -> &IntOption { &self.num_scsi_disks }
    fn get_num_scsi_cdroms(&self) -> &IntOption { &self.num_scsi_cdroms }
    fn get_num_scsi_passthrough(&self) -> &IntOption { &self.num_scsi_passthrough }
    fn get_sharing(&self) -> &Vec<super::enums::VirtualScsiSharingEnum> { &self.sharing }
    fn get_default_shared_index(&self) -> i32 { self.default_shared_index }
    fn get_hot_add_remove(&self) -> &BoolOption { &self.hot_add_remove }
    fn get_scsi_ctlr_unit_number(&self) -> i32 { self.scsi_ctlr_unit_number }
}
impl VirtualScsiControllerOptionTrait for VirtualBusLogicControllerOption {
    fn get_num_scsi_disks(&self) -> &IntOption { &self.num_scsi_disks }
    fn get_num_scsi_cdroms(&self) -> &IntOption { &self.num_scsi_cdroms }
    fn get_num_scsi_passthrough(&self) -> &IntOption { &self.num_scsi_passthrough }
    fn get_sharing(&self) -> &Vec<super::enums::VirtualScsiSharingEnum> { &self.sharing }
    fn get_default_shared_index(&self) -> i32 { self.default_shared_index }
    fn get_hot_add_remove(&self) -> &BoolOption { &self.hot_add_remove }
    fn get_scsi_ctlr_unit_number(&self) -> i32 { self.scsi_ctlr_unit_number }
}
impl VirtualScsiControllerOptionTrait for VirtualLsiLogicControllerOption {
    fn get_num_scsi_disks(&self) -> &IntOption { &self.num_scsi_disks }
    fn get_num_scsi_cdroms(&self) -> &IntOption { &self.num_scsi_cdroms }
    fn get_num_scsi_passthrough(&self) -> &IntOption { &self.num_scsi_passthrough }
    fn get_sharing(&self) -> &Vec<super::enums::VirtualScsiSharingEnum> { &self.sharing }
    fn get_default_shared_index(&self) -> i32 { self.default_shared_index }
    fn get_hot_add_remove(&self) -> &BoolOption { &self.hot_add_remove }
    fn get_scsi_ctlr_unit_number(&self) -> i32 { self.scsi_ctlr_unit_number }
}
impl VirtualScsiControllerOptionTrait for VirtualLsiLogicSasControllerOption {
    fn get_num_scsi_disks(&self) -> &IntOption { &self.num_scsi_disks }
    fn get_num_scsi_cdroms(&self) -> &IntOption { &self.num_scsi_cdroms }
    fn get_num_scsi_passthrough(&self) -> &IntOption { &self.num_scsi_passthrough }
    fn get_sharing(&self) -> &Vec<super::enums::VirtualScsiSharingEnum> { &self.sharing }
    fn get_default_shared_index(&self) -> i32 { self.default_shared_index }
    fn get_hot_add_remove(&self) -> &BoolOption { &self.hot_add_remove }
    fn get_scsi_ctlr_unit_number(&self) -> i32 { self.scsi_ctlr_unit_number }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualScsiControllerOptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualScsiControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualScsiControllerOption>()?),
            StructType::ParaVirtualScsiControllerOption => Some(from.as_any_ref().downcast_ref::<ParaVirtualScsiControllerOption>()?),
            StructType::VirtualBusLogicControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualBusLogicControllerOption>()?),
            StructType::VirtualLsiLogicControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualLsiLogicControllerOption>()?),
            StructType::VirtualLsiLogicSasControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualLsiLogicSasControllerOption>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualScsiControllerOption => Ok(from.as_any_box().downcast::<VirtualScsiControllerOption>()?),
            StructType::ParaVirtualScsiControllerOption => Ok(from.as_any_box().downcast::<ParaVirtualScsiControllerOption>()?),
            StructType::VirtualBusLogicControllerOption => Ok(from.as_any_box().downcast::<VirtualBusLogicControllerOption>()?),
            StructType::VirtualLsiLogicControllerOption => Ok(from.as_any_box().downcast::<VirtualLsiLogicControllerOption>()?),
            StructType::VirtualLsiLogicSasControllerOption => Ok(from.as_any_box().downcast::<VirtualLsiLogicSasControllerOption>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
