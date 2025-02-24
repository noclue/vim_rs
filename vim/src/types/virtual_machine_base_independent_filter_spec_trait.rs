use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The BaseIndependentFilterSpec is base class for two different types
/// of independent filter specs *VirtualMachineIndependentFilterSpec*
/// and *VirtualMachineEmptyIndependentFilterSpec* which are used to specify
/// independent filters to be attached/removed on VMs virtual disk.
/// 
/// ***Since:*** vSphere API Release 7.0.2.1
pub trait VirtualMachineBaseIndependentFilterSpecTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn VirtualMachineBaseIndependentFilterSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualMachineBaseIndependentFilterSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualMachineBaseIndependentFilterSpecVisitor)
            }
        }

struct VirtualMachineBaseIndependentFilterSpecVisitor;

impl<'de> de::Visitor<'de> for VirtualMachineBaseIndependentFilterSpecVisitor {
    type Value = Box<dyn VirtualMachineBaseIndependentFilterSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualMachineBaseIndependentFilterSpecTrait JSON object with a _typeName field")
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

impl VirtualMachineBaseIndependentFilterSpecTrait for VirtualMachineBaseIndependentFilterSpec {
}
impl VirtualMachineBaseIndependentFilterSpecTrait for VirtualMachineEmptyIndependentFilterSpec {
}
impl VirtualMachineBaseIndependentFilterSpecTrait for VirtualMachineIndependentFilterSpec {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualMachineBaseIndependentFilterSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualMachineBaseIndependentFilterSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineBaseIndependentFilterSpec>()?),
            StructType::VirtualMachineEmptyIndependentFilterSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineEmptyIndependentFilterSpec>()?),
            StructType::VirtualMachineIndependentFilterSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineIndependentFilterSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualMachineBaseIndependentFilterSpec => Ok(from.as_any_box().downcast::<VirtualMachineBaseIndependentFilterSpec>()?),
            StructType::VirtualMachineEmptyIndependentFilterSpec => Ok(from.as_any_box().downcast::<VirtualMachineEmptyIndependentFilterSpec>()?),
            StructType::VirtualMachineIndependentFilterSpec => Ok(from.as_any_box().downcast::<VirtualMachineIndependentFilterSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
