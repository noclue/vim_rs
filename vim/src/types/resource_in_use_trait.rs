use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A ResourceInUse fault indicating that some error has occurred because a
/// resource was in use.
/// 
/// Information about the resource that is in use may
/// be supplied.
pub trait ResourceInUseTrait : super::vim_fault_trait::VimFaultTrait {
    /// Type of resource that is in use.
    fn get_type(&self) -> &Option<String>;
    /// Name of the instance of the resource that is in use.
    fn get_name(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn ResourceInUseTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ResourceInUseTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ResourceInUseVisitor)
            }
        }

struct ResourceInUseVisitor;

impl<'de> de::Visitor<'de> for ResourceInUseVisitor {
    type Value = Box<dyn ResourceInUseTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ResourceInUseTrait JSON object with a _typeName field")
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

impl ResourceInUseTrait for ResourceInUse {
    fn get_type(&self) -> &Option<String> { &self.r#type }
    fn get_name(&self) -> &Option<String> { &self.name }
}
impl ResourceInUseTrait for FilterInUse {
    fn get_type(&self) -> &Option<String> { &self.r#type }
    fn get_name(&self) -> &Option<String> { &self.name }
}
impl ResourceInUseTrait for QuiesceDatastoreIoForHaFailed {
    fn get_type(&self) -> &Option<String> { &self.r#type }
    fn get_name(&self) -> &Option<String> { &self.name }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ResourceInUseTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ResourceInUse => Some(from.as_any_ref().downcast_ref::<ResourceInUse>()?),
            StructType::FilterInUse => Some(from.as_any_ref().downcast_ref::<FilterInUse>()?),
            StructType::QuiesceDatastoreIoForHaFailed => Some(from.as_any_ref().downcast_ref::<QuiesceDatastoreIoForHaFailed>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ResourceInUse => Ok(from.as_any_box().downcast::<ResourceInUse>()?),
            StructType::FilterInUse => Ok(from.as_any_box().downcast::<FilterInUse>()?),
            StructType::QuiesceDatastoreIoForHaFailed => Ok(from.as_any_box().downcast::<QuiesceDatastoreIoForHaFailed>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
