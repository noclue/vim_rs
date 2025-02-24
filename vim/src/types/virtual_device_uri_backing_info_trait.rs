use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The <code>*VirtualDeviceURIBackingInfo*</code> data object type
/// defines information for using a network socket as backing for a virtual device.
pub trait VirtualDeviceUriBackingInfoTrait : super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait {
    /// Identifies the local host or a system on the network,
    /// depending on the value of <code>*VirtualDeviceURIBackingInfo.direction*</code>.
    /// - If you use the virtual machine as a server, the URI identifies
    ///   the host on which the virtual machine runs. In this case,
    ///   the host name part of the URI should be empty, or it should
    ///   specify the address of the local host.
    /// - If you use the virtual machine as a client, the URI identifies
    ///   the remote system on the network.
    fn get_service_uri(&self) -> &str;
    /// The direction of the connection.
    /// 
    /// For possible values see
    /// *VirtualDeviceURIBackingOptionDirection_enum*
    fn get_direction(&self) -> &str;
    /// Identifies a proxy service that provides network access to the
    /// <code>*VirtualDeviceURIBackingInfo.serviceURI*</code>.
    /// 
    /// If you specify a proxy URI, the virtual machine initiates
    /// a connection with the proxy service and forwards the
    /// *VirtualDeviceURIBackingInfo.serviceURI* and *VirtualDeviceURIBackingInfo.direction* to the proxy.
    fn get_proxy_uri(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn VirtualDeviceUriBackingInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualDeviceUriBackingInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualDeviceUriBackingInfoVisitor)
            }
        }

struct VirtualDeviceUriBackingInfoVisitor;

impl<'de> de::Visitor<'de> for VirtualDeviceUriBackingInfoVisitor {
    type Value = Box<dyn VirtualDeviceUriBackingInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualDeviceUriBackingInfoTrait JSON object with a _typeName field")
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

impl VirtualDeviceUriBackingInfoTrait for VirtualDeviceUriBackingInfo {
    fn get_service_uri(&self) -> &str { &self.service_uri }
    fn get_direction(&self) -> &str { &self.direction }
    fn get_proxy_uri(&self) -> &Option<String> { &self.proxy_uri }
}
impl VirtualDeviceUriBackingInfoTrait for VirtualSerialPortUriBackingInfo {
    fn get_service_uri(&self) -> &str { &self.service_uri }
    fn get_direction(&self) -> &str { &self.direction }
    fn get_proxy_uri(&self) -> &Option<String> { &self.proxy_uri }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualDeviceUriBackingInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceUriBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualDeviceUriBackingInfo>()?),
            StructType::VirtualSerialPortUriBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortUriBackingInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceUriBackingInfo => Ok(from.as_any_box().downcast::<VirtualDeviceUriBackingInfo>()?),
            StructType::VirtualSerialPortUriBackingInfo => Ok(from.as_any_box().downcast::<VirtualSerialPortUriBackingInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
