use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

pub trait HostSriovDevicePoolInfoTrait : super::data_object_trait::DataObjectTrait {
    fn get_key(&self) -> &str;
}
impl<'s> serde::Serialize for dyn HostSriovDevicePoolInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostSriovDevicePoolInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostSriovDevicePoolInfoVisitor)
            }
        }

struct HostSriovDevicePoolInfoVisitor;

impl<'de> de::Visitor<'de> for HostSriovDevicePoolInfoVisitor {
    type Value = Box<dyn HostSriovDevicePoolInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostSriovDevicePoolInfoTrait JSON object with a _typeName field")
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

impl HostSriovDevicePoolInfoTrait for HostSriovDevicePoolInfo {
    fn get_key(&self) -> &str { &self.key }
}
impl HostSriovDevicePoolInfoTrait for HostSriovNetworkDevicePoolInfo {
    fn get_key(&self) -> &str { &self.key }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostSriovDevicePoolInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostSriovDevicePoolInfo => Some(from.as_any_ref().downcast_ref::<HostSriovDevicePoolInfo>()?),
            StructType::HostSriovNetworkDevicePoolInfo => Some(from.as_any_ref().downcast_ref::<HostSriovNetworkDevicePoolInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostSriovDevicePoolInfo => Ok(from.as_any_box().downcast::<HostSriovDevicePoolInfo>()?),
            StructType::HostSriovNetworkDevicePoolInfo => Ok(from.as_any_box().downcast::<HostSriovNetworkDevicePoolInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
