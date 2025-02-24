use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Static strings used for describing an object type.
pub trait TypeDescriptionTrait : super::description_trait::DescriptionTrait {
    /// Type being described
    fn get_key(&self) -> &str;
}
impl<'s> serde::Serialize for dyn TypeDescriptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn TypeDescriptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(TypeDescriptionVisitor)
            }
        }

struct TypeDescriptionVisitor;

impl<'de> de::Visitor<'de> for TypeDescriptionVisitor {
    type Value = Box<dyn TypeDescriptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid TypeDescriptionTrait JSON object with a _typeName field")
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

impl TypeDescriptionTrait for TypeDescription {
    fn get_key(&self) -> &str { &self.key }
}
impl TypeDescriptionTrait for ScheduledTaskDetail {
    fn get_key(&self) -> &str { &self.key }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn TypeDescriptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::TypeDescription => Some(from.as_any_ref().downcast_ref::<TypeDescription>()?),
            StructType::ScheduledTaskDetail => Some(from.as_any_ref().downcast_ref::<ScheduledTaskDetail>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::TypeDescription => Ok(from.as_any_box().downcast::<TypeDescription>()?),
            StructType::ScheduledTaskDetail => Ok(from.as_any_box().downcast::<ScheduledTaskDetail>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
