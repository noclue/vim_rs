use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// An InvalidArgument exception is thrown if the
/// set of arguments passed to the function is not
/// specified correctly.
pub trait InvalidArgumentTrait : super::runtime_fault_trait::RuntimeFaultTrait {
    /// Optional name of the invalid property.
    fn get_invalid_property(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn InvalidArgumentTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn InvalidArgumentTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(InvalidArgumentVisitor)
            }
        }

struct InvalidArgumentVisitor;

impl<'de> de::Visitor<'de> for InvalidArgumentVisitor {
    type Value = Box<dyn InvalidArgumentTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid InvalidArgumentTrait JSON object with a _typeName field")
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

impl InvalidArgumentTrait for InvalidArgument {
    fn get_invalid_property(&self) -> &Option<String> { &self.invalid_property }
}
impl InvalidArgumentTrait for IncompatibleSetting {
    fn get_invalid_property(&self) -> &Option<String> { &self.invalid_property }
}
impl InvalidArgumentTrait for InvalidDasConfigArgument {
    fn get_invalid_property(&self) -> &Option<String> { &self.invalid_property }
}
impl InvalidArgumentTrait for InvalidDasRestartPriorityForFtVm {
    fn get_invalid_property(&self) -> &Option<String> { &self.invalid_property }
}
impl InvalidArgumentTrait for InvalidDrsBehaviorForFtVm {
    fn get_invalid_property(&self) -> &Option<String> { &self.invalid_property }
}
impl InvalidArgumentTrait for InvalidIndexArgument {
    fn get_invalid_property(&self) -> &Option<String> { &self.invalid_property }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn InvalidArgumentTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidArgument => Some(from.as_any_ref().downcast_ref::<InvalidArgument>()?),
            StructType::IncompatibleSetting => Some(from.as_any_ref().downcast_ref::<IncompatibleSetting>()?),
            StructType::InvalidDasConfigArgument => Some(from.as_any_ref().downcast_ref::<InvalidDasConfigArgument>()?),
            StructType::InvalidDasRestartPriorityForFtVm => Some(from.as_any_ref().downcast_ref::<InvalidDasRestartPriorityForFtVm>()?),
            StructType::InvalidDrsBehaviorForFtVm => Some(from.as_any_ref().downcast_ref::<InvalidDrsBehaviorForFtVm>()?),
            StructType::InvalidIndexArgument => Some(from.as_any_ref().downcast_ref::<InvalidIndexArgument>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidArgument => Ok(from.as_any_box().downcast::<InvalidArgument>()?),
            StructType::IncompatibleSetting => Ok(from.as_any_box().downcast::<IncompatibleSetting>()?),
            StructType::InvalidDasConfigArgument => Ok(from.as_any_box().downcast::<InvalidDasConfigArgument>()?),
            StructType::InvalidDasRestartPriorityForFtVm => Ok(from.as_any_box().downcast::<InvalidDasRestartPriorityForFtVm>()?),
            StructType::InvalidDrsBehaviorForFtVm => Ok(from.as_any_box().downcast::<InvalidDrsBehaviorForFtVm>()?),
            StructType::InvalidIndexArgument => Ok(from.as_any_box().downcast::<InvalidIndexArgument>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
