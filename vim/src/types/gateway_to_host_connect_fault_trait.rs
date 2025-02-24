use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Deprecated not used since vSphere 6.5.
/// 
/// GatewayToHostConnectFault is thrown by the gateway used to communicate
/// with a host, if an error occurs in the communication between the gateway and
/// the host.
/// 
/// More details may be provided by a subfault.
pub trait GatewayToHostConnectFaultTrait : super::gateway_connect_fault_trait::GatewayConnectFaultTrait {
    /// Hostname of the host that the gateway is communicating with.
    fn get_hostname(&self) -> &str;
    /// Port specified for the connection between the gateway and the host.
    fn get_port(&self) -> Option<i32>;
}
impl<'s> serde::Serialize for dyn GatewayToHostConnectFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn GatewayToHostConnectFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(GatewayToHostConnectFaultVisitor)
            }
        }

struct GatewayToHostConnectFaultVisitor;

impl<'de> de::Visitor<'de> for GatewayToHostConnectFaultVisitor {
    type Value = Box<dyn GatewayToHostConnectFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid GatewayToHostConnectFaultTrait JSON object with a _typeName field")
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

impl GatewayToHostConnectFaultTrait for GatewayToHostConnectFault {
    fn get_hostname(&self) -> &str { &self.hostname }
    fn get_port(&self) -> Option<i32> { self.port }
}
impl GatewayToHostConnectFaultTrait for GatewayHostNotReachable {
    fn get_hostname(&self) -> &str { &self.hostname }
    fn get_port(&self) -> Option<i32> { self.port }
}
impl GatewayToHostConnectFaultTrait for GatewayToHostAuthFault {
    fn get_hostname(&self) -> &str { &self.hostname }
    fn get_port(&self) -> Option<i32> { self.port }
}
impl GatewayToHostConnectFaultTrait for GatewayToHostTrustVerifyFault {
    fn get_hostname(&self) -> &str { &self.hostname }
    fn get_port(&self) -> Option<i32> { self.port }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn GatewayToHostConnectFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::GatewayToHostConnectFault => Some(from.as_any_ref().downcast_ref::<GatewayToHostConnectFault>()?),
            StructType::GatewayHostNotReachable => Some(from.as_any_ref().downcast_ref::<GatewayHostNotReachable>()?),
            StructType::GatewayToHostAuthFault => Some(from.as_any_ref().downcast_ref::<GatewayToHostAuthFault>()?),
            StructType::GatewayToHostTrustVerifyFault => Some(from.as_any_ref().downcast_ref::<GatewayToHostTrustVerifyFault>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::GatewayToHostConnectFault => Ok(from.as_any_box().downcast::<GatewayToHostConnectFault>()?),
            StructType::GatewayHostNotReachable => Ok(from.as_any_box().downcast::<GatewayHostNotReachable>()?),
            StructType::GatewayToHostAuthFault => Ok(from.as_any_box().downcast::<GatewayToHostAuthFault>()?),
            StructType::GatewayToHostTrustVerifyFault => Ok(from.as_any_box().downcast::<GatewayToHostTrustVerifyFault>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
