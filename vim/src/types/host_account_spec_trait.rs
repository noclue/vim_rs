use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type contains common parameters
/// for local account creation.
/// 
/// The password and description properties
/// are not supported for group accounts on POSIX hosts.
pub trait HostAccountSpecTrait : super::data_object_trait::DataObjectTrait {
    /// The ID of the specified account.
    fn get_id(&self) -> &str;
    /// The password for a user or group.
    fn get_password(&self) -> &Option<String>;
    /// The description of the specified account.
    fn get_description(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn HostAccountSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostAccountSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostAccountSpecVisitor)
            }
        }

struct HostAccountSpecVisitor;

impl<'de> de::Visitor<'de> for HostAccountSpecVisitor {
    type Value = Box<dyn HostAccountSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostAccountSpecTrait JSON object with a _typeName field")
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

impl HostAccountSpecTrait for HostAccountSpec {
    fn get_id(&self) -> &str { &self.id }
    fn get_password(&self) -> &Option<String> { &self.password }
    fn get_description(&self) -> &Option<String> { &self.description }
}
impl HostAccountSpecTrait for HostPosixAccountSpec {
    fn get_id(&self) -> &str { &self.id }
    fn get_password(&self) -> &Option<String> { &self.password }
    fn get_description(&self) -> &Option<String> { &self.description }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostAccountSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostAccountSpec => Some(from.as_any_ref().downcast_ref::<HostAccountSpec>()?),
            StructType::HostPosixAccountSpec => Some(from.as_any_ref().downcast_ref::<HostPosixAccountSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostAccountSpec => Ok(from.as_any_box().downcast::<HostAccountSpec>()?),
            StructType::HostPosixAccountSpec => Ok(from.as_any_box().downcast::<HostPosixAccountSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
