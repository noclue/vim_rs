use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The ProfileSpec data object is used to specify the Storage Policy to be
/// associated with a Virtual Machine Home or a Virtual Disk.
pub trait VirtualMachineProfileSpecTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn VirtualMachineProfileSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualMachineProfileSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualMachineProfileSpecVisitor)
            }
        }

struct VirtualMachineProfileSpecVisitor;

impl<'de> de::Visitor<'de> for VirtualMachineProfileSpecVisitor {
    type Value = Box<dyn VirtualMachineProfileSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualMachineProfileSpecTrait JSON object with a _typeName field")
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

impl VirtualMachineProfileSpecTrait for VirtualMachineProfileSpec {
}
impl VirtualMachineProfileSpecTrait for VirtualMachineDefaultProfileSpec {
}
impl VirtualMachineProfileSpecTrait for VirtualMachineDefinedProfileSpec {
}
impl VirtualMachineProfileSpecTrait for VirtualMachineEmptyProfileSpec {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualMachineProfileSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualMachineProfileSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineProfileSpec>()?),
            StructType::VirtualMachineDefaultProfileSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineDefaultProfileSpec>()?),
            StructType::VirtualMachineDefinedProfileSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineDefinedProfileSpec>()?),
            StructType::VirtualMachineEmptyProfileSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineEmptyProfileSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualMachineProfileSpec => Ok(from.as_any_box().downcast::<VirtualMachineProfileSpec>()?),
            StructType::VirtualMachineDefaultProfileSpec => Ok(from.as_any_box().downcast::<VirtualMachineDefaultProfileSpec>()?),
            StructType::VirtualMachineDefinedProfileSpec => Ok(from.as_any_box().downcast::<VirtualMachineDefinedProfileSpec>()?),
            StructType::VirtualMachineEmptyProfileSpec => Ok(from.as_any_box().downcast::<VirtualMachineEmptyProfileSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
