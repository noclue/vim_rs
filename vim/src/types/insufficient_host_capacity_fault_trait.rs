use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The host does not have enough capacity for running the virtual machine.
pub trait InsufficientHostCapacityFaultTrait : super::insufficient_resources_fault_trait::InsufficientResourcesFaultTrait {
    /// The host which does not have the enough capacity.
    /// 
    /// Refers instance of *HostSystem*.
    fn get_host(&self) -> &Option<ManagedObjectReference>;
}
impl<'s> serde::Serialize for dyn InsufficientHostCapacityFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn InsufficientHostCapacityFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(InsufficientHostCapacityFaultVisitor)
            }
        }

struct InsufficientHostCapacityFaultVisitor;

impl<'de> de::Visitor<'de> for InsufficientHostCapacityFaultVisitor {
    type Value = Box<dyn InsufficientHostCapacityFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid InsufficientHostCapacityFaultTrait JSON object with a _typeName field")
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

impl InsufficientHostCapacityFaultTrait for InsufficientHostCapacityFault {
    fn get_host(&self) -> &Option<ManagedObjectReference> { &self.host }
}
impl InsufficientHostCapacityFaultTrait for InsufficientHostCpuCapacityFault {
    fn get_host(&self) -> &Option<ManagedObjectReference> { &self.host }
}
impl InsufficientHostCapacityFaultTrait for InsufficientHostMemoryCapacityFault {
    fn get_host(&self) -> &Option<ManagedObjectReference> { &self.host }
}
impl InsufficientHostCapacityFaultTrait for InsufficientPerCpuCapacity {
    fn get_host(&self) -> &Option<ManagedObjectReference> { &self.host }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn InsufficientHostCapacityFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::InsufficientHostCapacityFault => Some(from.as_any_ref().downcast_ref::<InsufficientHostCapacityFault>()?),
            StructType::InsufficientHostCpuCapacityFault => Some(from.as_any_ref().downcast_ref::<InsufficientHostCpuCapacityFault>()?),
            StructType::InsufficientHostMemoryCapacityFault => Some(from.as_any_ref().downcast_ref::<InsufficientHostMemoryCapacityFault>()?),
            StructType::InsufficientPerCpuCapacity => Some(from.as_any_ref().downcast_ref::<InsufficientPerCpuCapacity>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::InsufficientHostCapacityFault => Ok(from.as_any_box().downcast::<InsufficientHostCapacityFault>()?),
            StructType::InsufficientHostCpuCapacityFault => Ok(from.as_any_box().downcast::<InsufficientHostCpuCapacityFault>()?),
            StructType::InsufficientHostMemoryCapacityFault => Ok(from.as_any_box().downcast::<InsufficientHostMemoryCapacityFault>()?),
            StructType::InsufficientPerCpuCapacity => Ok(from.as_any_box().downcast::<InsufficientPerCpuCapacity>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
