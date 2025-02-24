use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A NoCompatibleHost fault is thrown when DRS cannot find a compatible
/// host in a given compute resource to run a virtual machine on.
pub trait NoCompatibleHostTrait : super::vim_fault_trait::VimFaultTrait {
    /// The list of hosts that are not compatible, each element has a
    /// corresponding fault in the error array.
    /// 
    /// Refers instances of *HostSystem*.
    fn get_host(&self) -> &Option<Vec<ManagedObjectReference>>;
    /// An error in this array indicates why the corresponding host in the
    /// host array is incompatible.
    fn get_error(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>>;
}
impl<'s> serde::Serialize for dyn NoCompatibleHostTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn NoCompatibleHostTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(NoCompatibleHostVisitor)
            }
        }

struct NoCompatibleHostVisitor;

impl<'de> de::Visitor<'de> for NoCompatibleHostVisitor {
    type Value = Box<dyn NoCompatibleHostTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid NoCompatibleHostTrait JSON object with a _typeName field")
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

impl NoCompatibleHostTrait for NoCompatibleHost {
    fn get_host(&self) -> &Option<Vec<ManagedObjectReference>> { &self.host }
    fn get_error(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>> { &self.error }
}
impl NoCompatibleHostTrait for NoCompatibleHostWithAccessToDevice {
    fn get_host(&self) -> &Option<Vec<ManagedObjectReference>> { &self.host }
    fn get_error(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>> { &self.error }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn NoCompatibleHostTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::NoCompatibleHost => Some(from.as_any_ref().downcast_ref::<NoCompatibleHost>()?),
            StructType::NoCompatibleHostWithAccessToDevice => Some(from.as_any_ref().downcast_ref::<NoCompatibleHostWithAccessToDevice>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::NoCompatibleHost => Ok(from.as_any_box().downcast::<NoCompatibleHost>()?),
            StructType::NoCompatibleHostWithAccessToDevice => Ok(from.as_any_box().downcast::<NoCompatibleHostWithAccessToDevice>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
