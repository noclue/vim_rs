use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base class for host-specific answer file options.
pub trait AnswerFileCreateSpecTrait : super::data_object_trait::DataObjectTrait {
    /// If "false", then the answer file will be saved without being validated.
    /// 
    /// The default if not specified is "true".
    /// This option should be used with caution, since the resulting answer
    /// file will not be checked for errors.
    fn get_validating(&self) -> Option<bool>;
}
impl<'s> serde::Serialize for dyn AnswerFileCreateSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn AnswerFileCreateSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(AnswerFileCreateSpecVisitor)
            }
        }

struct AnswerFileCreateSpecVisitor;

impl<'de> de::Visitor<'de> for AnswerFileCreateSpecVisitor {
    type Value = Box<dyn AnswerFileCreateSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid AnswerFileCreateSpecTrait JSON object with a _typeName field")
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

impl AnswerFileCreateSpecTrait for AnswerFileCreateSpec {
    fn get_validating(&self) -> Option<bool> { self.validating }
}
impl AnswerFileCreateSpecTrait for AnswerFileOptionsCreateSpec {
    fn get_validating(&self) -> Option<bool> { self.validating }
}
impl AnswerFileCreateSpecTrait for AnswerFileSerializedCreateSpec {
    fn get_validating(&self) -> Option<bool> { self.validating }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn AnswerFileCreateSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::AnswerFileCreateSpec => Some(from.as_any_ref().downcast_ref::<AnswerFileCreateSpec>()?),
            StructType::AnswerFileOptionsCreateSpec => Some(from.as_any_ref().downcast_ref::<AnswerFileOptionsCreateSpec>()?),
            StructType::AnswerFileSerializedCreateSpec => Some(from.as_any_ref().downcast_ref::<AnswerFileSerializedCreateSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::AnswerFileCreateSpec => Ok(from.as_any_box().downcast::<AnswerFileCreateSpec>()?),
            StructType::AnswerFileOptionsCreateSpec => Ok(from.as_any_box().downcast::<AnswerFileOptionsCreateSpec>()?),
            StructType::AnswerFileSerializedCreateSpec => Ok(from.as_any_box().downcast::<AnswerFileSerializedCreateSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
