use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// When searching for users, the search results in
/// some additional information.
/// 
/// This object describes
/// the additional information.
pub trait UserSearchResultTrait : super::data_object_trait::DataObjectTrait {
    /// Login name of a user or the name of a group.
    /// 
    /// This key is
    /// the user within the searched domain.
    fn get_principal(&self) -> &str;
    /// Full name of the user found by the search, or the description
    /// of a group, if available.
    fn get_full_name(&self) -> &Option<String>;
    /// If this is true, then the result is a group.
    /// 
    /// If this is false, then the
    /// result is a user.
    fn get_group(&self) -> bool;
}
impl<'s> serde::Serialize for dyn UserSearchResultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn UserSearchResultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(UserSearchResultVisitor)
            }
        }

struct UserSearchResultVisitor;

impl<'de> de::Visitor<'de> for UserSearchResultVisitor {
    type Value = Box<dyn UserSearchResultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid UserSearchResultTrait JSON object with a _typeName field")
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

impl UserSearchResultTrait for UserSearchResult {
    fn get_principal(&self) -> &str { &self.principal }
    fn get_full_name(&self) -> &Option<String> { &self.full_name }
    fn get_group(&self) -> bool { self.group }
}
impl UserSearchResultTrait for PosixUserSearchResult {
    fn get_principal(&self) -> &str { &self.principal }
    fn get_full_name(&self) -> &Option<String> { &self.full_name }
    fn get_group(&self) -> bool { self.group }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn UserSearchResultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::UserSearchResult => Some(from.as_any_ref().downcast_ref::<UserSearchResult>()?),
            StructType::PosixUserSearchResult => Some(from.as_any_ref().downcast_ref::<PosixUserSearchResult>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::UserSearchResult => Ok(from.as_any_box().downcast::<UserSearchResult>()?),
            StructType::PosixUserSearchResult => Ok(from.as_any_box().downcast::<PosixUserSearchResult>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
