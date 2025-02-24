use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *ProfileSerializedCreateSpec* data object
/// defines a string that contains a serialized representation of a host profile.
pub trait ProfileSerializedCreateSpecTrait : super::profile_create_spec_trait::ProfileCreateSpecTrait {
    /// Representation of the profile in the string form.
    fn get_profile_config_string(&self) -> &str;
}
impl<'s> serde::Serialize for dyn ProfileSerializedCreateSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ProfileSerializedCreateSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ProfileSerializedCreateSpecVisitor)
            }
        }

struct ProfileSerializedCreateSpecVisitor;

impl<'de> de::Visitor<'de> for ProfileSerializedCreateSpecVisitor {
    type Value = Box<dyn ProfileSerializedCreateSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ProfileSerializedCreateSpecTrait JSON object with a _typeName field")
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

impl ProfileSerializedCreateSpecTrait for ProfileSerializedCreateSpec {
    fn get_profile_config_string(&self) -> &str { &self.profile_config_string }
}
impl ProfileSerializedCreateSpecTrait for HostProfileSerializedHostProfileSpec {
    fn get_profile_config_string(&self) -> &str { &self.profile_config_string }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ProfileSerializedCreateSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ProfileSerializedCreateSpec => Some(from.as_any_ref().downcast_ref::<ProfileSerializedCreateSpec>()?),
            StructType::HostProfileSerializedHostProfileSpec => Some(from.as_any_ref().downcast_ref::<HostProfileSerializedHostProfileSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ProfileSerializedCreateSpec => Ok(from.as_any_box().downcast::<ProfileSerializedCreateSpec>()?),
            StructType::HostProfileSerializedHostProfileSpec => Ok(from.as_any_box().downcast::<HostProfileSerializedHostProfileSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
