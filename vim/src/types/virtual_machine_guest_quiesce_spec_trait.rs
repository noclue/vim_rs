use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type encapsulates configuration settings
/// when creating a virtual machine quiesced snapshot.
pub trait VirtualMachineGuestQuiesceSpecTrait : super::data_object_trait::DataObjectTrait {
    /// The property to indicate maximum time in minutes for snapshot operation
    /// to be performed on the virtual machine.
    /// 
    /// The timeout can not be less than 5 minutes or more than 240 minutes.
    fn get_timeout(&self) -> Option<i32>;
}
impl<'s> serde::Serialize for dyn VirtualMachineGuestQuiesceSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualMachineGuestQuiesceSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualMachineGuestQuiesceSpecVisitor)
            }
        }

struct VirtualMachineGuestQuiesceSpecVisitor;

impl<'de> de::Visitor<'de> for VirtualMachineGuestQuiesceSpecVisitor {
    type Value = Box<dyn VirtualMachineGuestQuiesceSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualMachineGuestQuiesceSpecTrait JSON object with a _typeName field")
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

impl VirtualMachineGuestQuiesceSpecTrait for VirtualMachineGuestQuiesceSpec {
    fn get_timeout(&self) -> Option<i32> { self.timeout }
}
impl VirtualMachineGuestQuiesceSpecTrait for VirtualMachineWindowsQuiesceSpec {
    fn get_timeout(&self) -> Option<i32> { self.timeout }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualMachineGuestQuiesceSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualMachineGuestQuiesceSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineGuestQuiesceSpec>()?),
            StructType::VirtualMachineWindowsQuiesceSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineWindowsQuiesceSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualMachineGuestQuiesceSpec => Ok(from.as_any_box().downcast::<VirtualMachineGuestQuiesceSpec>()?),
            StructType::VirtualMachineWindowsQuiesceSpec => Ok(from.as_any_box().downcast::<VirtualMachineWindowsQuiesceSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
