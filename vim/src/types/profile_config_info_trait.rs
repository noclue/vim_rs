use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

pub trait ProfileConfigInfoTrait : super::data_object_trait::DataObjectTrait {
    /// Name of the profile
    fn get_name(&self) -> &str;
    /// User Provided description of the profile
    fn get_annotation(&self) -> &Option<String>;
    /// Flag indicating if the Profile is enabled
    fn get_enabled(&self) -> bool;
}
impl<'s> serde::Serialize for dyn ProfileConfigInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ProfileConfigInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ProfileConfigInfoVisitor)
            }
        }

struct ProfileConfigInfoVisitor;

impl<'de> de::Visitor<'de> for ProfileConfigInfoVisitor {
    type Value = Box<dyn ProfileConfigInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ProfileConfigInfoTrait JSON object with a _typeName field")
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

impl ProfileConfigInfoTrait for ProfileConfigInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_annotation(&self) -> &Option<String> { &self.annotation }
    fn get_enabled(&self) -> bool { self.enabled }
}
impl ProfileConfigInfoTrait for ClusterProfileConfigInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_annotation(&self) -> &Option<String> { &self.annotation }
    fn get_enabled(&self) -> bool { self.enabled }
}
impl ProfileConfigInfoTrait for HostProfileConfigInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_annotation(&self) -> &Option<String> { &self.annotation }
    fn get_enabled(&self) -> bool { self.enabled }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ProfileConfigInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ProfileConfigInfo => Some(from.as_any_ref().downcast_ref::<ProfileConfigInfo>()?),
            StructType::ClusterProfileConfigInfo => Some(from.as_any_ref().downcast_ref::<ClusterProfileConfigInfo>()?),
            StructType::HostProfileConfigInfo => Some(from.as_any_ref().downcast_ref::<HostProfileConfigInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ProfileConfigInfo => Ok(from.as_any_box().downcast::<ProfileConfigInfo>()?),
            StructType::ClusterProfileConfigInfo => Ok(from.as_any_box().downcast::<ClusterProfileConfigInfo>()?),
            StructType::HostProfileConfigInfo => Ok(from.as_any_box().downcast::<HostProfileConfigInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
