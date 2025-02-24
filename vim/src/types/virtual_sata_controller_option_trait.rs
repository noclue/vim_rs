use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The VirtualSATAControllerOption data object type contains the options
/// for a virtual SATA controller defined by the
/// *VirtualSATAController*
/// data object type.
pub trait VirtualSataControllerOptionTrait : super::virtual_controller_option_trait::VirtualControllerOptionTrait {
    /// Three properties (numSATADisks.min, numSATADisks.max, and
    /// numSATADisks.defaultValue) define the minimum, maximum, and default
    /// number of SATA VirtualDisk instances available at any given time in the
    /// SATA controller.
    /// 
    /// The number of SATA VirtualDisk instances is
    /// also limited by the number of available slots in the SATA controller.
    fn get_num_sata_disks(&self) -> &IntOption;
    /// Three properties (numSATACdroms.min, numSATACdroms.max, and
    /// numSATACdroms.defaultValue) define the minimum, maximum, and default
    /// number of SATA VirtualCdrom instances available
    /// in the SATA controller.
    /// 
    /// The number of SATA VirtualCdrom instances is
    /// also limited by the number of available slots in the SATA controller.
    fn get_num_sata_cdroms(&self) -> &IntOption;
}
impl<'s> serde::Serialize for dyn VirtualSataControllerOptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualSataControllerOptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualSataControllerOptionVisitor)
            }
        }

struct VirtualSataControllerOptionVisitor;

impl<'de> de::Visitor<'de> for VirtualSataControllerOptionVisitor {
    type Value = Box<dyn VirtualSataControllerOptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualSataControllerOptionTrait JSON object with a _typeName field")
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

impl VirtualSataControllerOptionTrait for VirtualSataControllerOption {
    fn get_num_sata_disks(&self) -> &IntOption { &self.num_sata_disks }
    fn get_num_sata_cdroms(&self) -> &IntOption { &self.num_sata_cdroms }
}
impl VirtualSataControllerOptionTrait for VirtualAhciControllerOption {
    fn get_num_sata_disks(&self) -> &IntOption { &self.num_sata_disks }
    fn get_num_sata_cdroms(&self) -> &IntOption { &self.num_sata_cdroms }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualSataControllerOptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualSataControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualSataControllerOption>()?),
            StructType::VirtualAhciControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualAhciControllerOption>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualSataControllerOption => Ok(from.as_any_box().downcast::<VirtualSataControllerOption>()?),
            StructType::VirtualAhciControllerOption => Ok(from.as_any_box().downcast::<VirtualAhciControllerOption>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
