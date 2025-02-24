use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base class for all system swap options.
/// 
/// This class is not supposed to be used directly.  
/// These values are to be used in a *SystemSwapConfiguration.option*
/// array.
pub trait HostSystemSwapConfigurationSystemSwapOptionTrait : super::data_object_trait::DataObjectTrait {
    /// Specifies the order the options are preferred among each other.
    /// 
    /// The lower the value the more important.
    fn get_key(&self) -> i32;
}
impl<'s> serde::Serialize for dyn HostSystemSwapConfigurationSystemSwapOptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostSystemSwapConfigurationSystemSwapOptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostSystemSwapConfigurationSystemSwapOptionVisitor)
            }
        }

struct HostSystemSwapConfigurationSystemSwapOptionVisitor;

impl<'de> de::Visitor<'de> for HostSystemSwapConfigurationSystemSwapOptionVisitor {
    type Value = Box<dyn HostSystemSwapConfigurationSystemSwapOptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostSystemSwapConfigurationSystemSwapOptionTrait JSON object with a _typeName field")
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

impl HostSystemSwapConfigurationSystemSwapOptionTrait for HostSystemSwapConfigurationSystemSwapOption {
    fn get_key(&self) -> i32 { self.key }
}
impl HostSystemSwapConfigurationSystemSwapOptionTrait for HostSystemSwapConfigurationDatastoreOption {
    fn get_key(&self) -> i32 { self.key }
}
impl HostSystemSwapConfigurationSystemSwapOptionTrait for HostSystemSwapConfigurationDisabledOption {
    fn get_key(&self) -> i32 { self.key }
}
impl HostSystemSwapConfigurationSystemSwapOptionTrait for HostSystemSwapConfigurationHostCacheOption {
    fn get_key(&self) -> i32 { self.key }
}
impl HostSystemSwapConfigurationSystemSwapOptionTrait for HostSystemSwapConfigurationHostLocalSwapOption {
    fn get_key(&self) -> i32 { self.key }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostSystemSwapConfigurationSystemSwapOptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostSystemSwapConfigurationSystemSwapOption => Some(from.as_any_ref().downcast_ref::<HostSystemSwapConfigurationSystemSwapOption>()?),
            StructType::HostSystemSwapConfigurationDatastoreOption => Some(from.as_any_ref().downcast_ref::<HostSystemSwapConfigurationDatastoreOption>()?),
            StructType::HostSystemSwapConfigurationDisabledOption => Some(from.as_any_ref().downcast_ref::<HostSystemSwapConfigurationDisabledOption>()?),
            StructType::HostSystemSwapConfigurationHostCacheOption => Some(from.as_any_ref().downcast_ref::<HostSystemSwapConfigurationHostCacheOption>()?),
            StructType::HostSystemSwapConfigurationHostLocalSwapOption => Some(from.as_any_ref().downcast_ref::<HostSystemSwapConfigurationHostLocalSwapOption>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostSystemSwapConfigurationSystemSwapOption => Ok(from.as_any_box().downcast::<HostSystemSwapConfigurationSystemSwapOption>()?),
            StructType::HostSystemSwapConfigurationDatastoreOption => Ok(from.as_any_box().downcast::<HostSystemSwapConfigurationDatastoreOption>()?),
            StructType::HostSystemSwapConfigurationDisabledOption => Ok(from.as_any_box().downcast::<HostSystemSwapConfigurationDisabledOption>()?),
            StructType::HostSystemSwapConfigurationHostCacheOption => Ok(from.as_any_box().downcast::<HostSystemSwapConfigurationHostCacheOption>()?),
            StructType::HostSystemSwapConfigurationHostLocalSwapOption => Ok(from.as_any_box().downcast::<HostSystemSwapConfigurationHostLocalSwapOption>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
