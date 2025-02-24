use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Fault indicating that the CAM server
/// for camServer cannot be reached,
/// or is not a valid IP address.
pub trait InvalidCamServerTrait : super::active_directory_fault_trait::ActiveDirectoryFaultTrait {
    /// The address of the CAM server.
    fn get_cam_server(&self) -> &str;
}
impl<'s> serde::Serialize for dyn InvalidCamServerTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn InvalidCamServerTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(InvalidCamServerVisitor)
            }
        }

struct InvalidCamServerVisitor;

impl<'de> de::Visitor<'de> for InvalidCamServerVisitor {
    type Value = Box<dyn InvalidCamServerTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid InvalidCamServerTrait JSON object with a _typeName field")
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

impl InvalidCamServerTrait for InvalidCamServer {
    fn get_cam_server(&self) -> &str { &self.cam_server }
}
impl InvalidCamServerTrait for CamServerRefusedConnection {
    fn get_cam_server(&self) -> &str { &self.cam_server }
}
impl InvalidCamServerTrait for InvalidCamCertificate {
    fn get_cam_server(&self) -> &str { &self.cam_server }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn InvalidCamServerTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidCamServer => Some(from.as_any_ref().downcast_ref::<InvalidCamServer>()?),
            StructType::CamServerRefusedConnection => Some(from.as_any_ref().downcast_ref::<CamServerRefusedConnection>()?),
            StructType::InvalidCamCertificate => Some(from.as_any_ref().downcast_ref::<InvalidCamCertificate>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidCamServer => Ok(from.as_any_box().downcast::<InvalidCamServer>()?),
            StructType::CamServerRefusedConnection => Ok(from.as_any_box().downcast::<CamServerRefusedConnection>()?),
            StructType::InvalidCamCertificate => Ok(from.as_any_box().downcast::<InvalidCamCertificate>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
