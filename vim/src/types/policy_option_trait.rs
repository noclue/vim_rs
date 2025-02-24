use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *PolicyOption* data object represents one or more configuration
/// values.
/// 
/// A policy option is one of the configuration options from the
/// *ProfilePolicyMetadata*.*ProfilePolicyMetadata.possibleOption*
/// list.
pub trait PolicyOptionTrait : super::data_object_trait::DataObjectTrait {
    /// Identifier for the policy option.
    /// 
    /// This value matches one of the
    /// keys from the list of possible options in the policy metadata
    /// (*ProfilePolicyMetadata*.*ProfilePolicyMetadata.possibleOption*\[\].*ProfilePolicyOptionMetadata.id*.*ElementDescription.key*).
    fn get_id(&self) -> &str;
    /// Parameters for the policy option.
    /// 
    /// This list must include all parameters that are not marked as optional
    /// in the policy option metadata parameter list
    /// (*ProfilePolicyMetadata*.*ProfilePolicyMetadata.possibleOption*\[\].*ProfilePolicyOptionMetadata.parameter*\[\].*ProfileParameterMetadata.optional*).
    fn get_parameter(&self) -> &Option<Vec<KeyAnyValue>>;
}
impl<'s> serde::Serialize for dyn PolicyOptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn PolicyOptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(PolicyOptionVisitor)
            }
        }

struct PolicyOptionVisitor;

impl<'de> de::Visitor<'de> for PolicyOptionVisitor {
    type Value = Box<dyn PolicyOptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid PolicyOptionTrait JSON object with a _typeName field")
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

impl PolicyOptionTrait for PolicyOption {
    fn get_id(&self) -> &str { &self.id }
    fn get_parameter(&self) -> &Option<Vec<KeyAnyValue>> { &self.parameter }
}
impl PolicyOptionTrait for CompositePolicyOption {
    fn get_id(&self) -> &str { &self.id }
    fn get_parameter(&self) -> &Option<Vec<KeyAnyValue>> { &self.parameter }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn PolicyOptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::PolicyOption => Some(from.as_any_ref().downcast_ref::<PolicyOption>()?),
            StructType::CompositePolicyOption => Some(from.as_any_ref().downcast_ref::<CompositePolicyOption>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::PolicyOption => Ok(from.as_any_box().downcast::<PolicyOption>()?),
            StructType::CompositePolicyOption => Ok(from.as_any_box().downcast::<CompositePolicyOption>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
