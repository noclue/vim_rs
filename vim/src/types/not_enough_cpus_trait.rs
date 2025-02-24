use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The host hardware does not have enough CPU cores to support the number of
/// virtual CPUs in the virtual machine.
/// 
/// If the host is using hyperthreading, NotEnoughLogicalCpus is
/// employed instead of NotEnoughCpus.
pub trait NotEnoughCpusTrait : super::virtual_hardware_compatibility_issue_trait::VirtualHardwareCompatibilityIssueTrait {
    /// The number of CPUs present on the host.
    fn get_num_cpu_dest(&self) -> i32;
    /// The number of virtual CPUs present in the virtual machine.
    fn get_num_cpu_vm(&self) -> i32;
}
impl<'s> serde::Serialize for dyn NotEnoughCpusTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn NotEnoughCpusTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(NotEnoughCpusVisitor)
            }
        }

struct NotEnoughCpusVisitor;

impl<'de> de::Visitor<'de> for NotEnoughCpusVisitor {
    type Value = Box<dyn NotEnoughCpusTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid NotEnoughCpusTrait JSON object with a _typeName field")
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

impl NotEnoughCpusTrait for NotEnoughCpus {
    fn get_num_cpu_dest(&self) -> i32 { self.num_cpu_dest }
    fn get_num_cpu_vm(&self) -> i32 { self.num_cpu_vm }
}
impl NotEnoughCpusTrait for NotEnoughLogicalCpus {
    fn get_num_cpu_dest(&self) -> i32 { self.num_cpu_dest }
    fn get_num_cpu_vm(&self) -> i32 { self.num_cpu_vm }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn NotEnoughCpusTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::NotEnoughCpus => Some(from.as_any_ref().downcast_ref::<NotEnoughCpus>()?),
            StructType::NotEnoughLogicalCpus => Some(from.as_any_ref().downcast_ref::<NotEnoughLogicalCpus>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::NotEnoughCpus => Ok(from.as_any_box().downcast::<NotEnoughCpus>()?),
            StructType::NotEnoughLogicalCpus => Ok(from.as_any_box().downcast::<NotEnoughLogicalCpus>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
