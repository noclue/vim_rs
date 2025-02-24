use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Deprecated not used since vSphere 6.5.
/// 
/// GatewayConnectFault is a base fault type for the faults thrown by a gateway
/// server used to connect to a host.
pub trait GatewayConnectFaultTrait : super::host_connect_fault_trait::HostConnectFaultTrait {
    /// The type of the gateway used for the connection to the host.
    fn get_gateway_type(&self) -> &str;
    /// Identifier of the gateway that is used for the connection to the host.
    fn get_gateway_id(&self) -> &str;
    /// Human-readable information about the host gateway server.
    fn get_gateway_info(&self) -> &str;
    /// Details of the cause for this fault.
    /// 
    /// This is the way in which Host
    /// Gateway servers propagate opaque error messages through vCenter Server.
    fn get_details(&self) -> &Option<LocalizableMessage>;
}
impl<'s> serde::Serialize for dyn GatewayConnectFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn GatewayConnectFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(GatewayConnectFaultVisitor)
            }
        }

struct GatewayConnectFaultVisitor;

impl<'de> de::Visitor<'de> for GatewayConnectFaultVisitor {
    type Value = Box<dyn GatewayConnectFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid GatewayConnectFaultTrait JSON object with a _typeName field")
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

impl GatewayConnectFaultTrait for GatewayConnectFault {
    fn get_gateway_type(&self) -> &str { &self.gateway_type }
    fn get_gateway_id(&self) -> &str { &self.gateway_id }
    fn get_gateway_info(&self) -> &str { &self.gateway_info }
    fn get_details(&self) -> &Option<LocalizableMessage> { &self.details }
}
impl GatewayConnectFaultTrait for GatewayNotFound {
    fn get_gateway_type(&self) -> &str { &self.gateway_type }
    fn get_gateway_id(&self) -> &str { &self.gateway_id }
    fn get_gateway_info(&self) -> &str { &self.gateway_info }
    fn get_details(&self) -> &Option<LocalizableMessage> { &self.details }
}
impl GatewayConnectFaultTrait for GatewayNotReachable {
    fn get_gateway_type(&self) -> &str { &self.gateway_type }
    fn get_gateway_id(&self) -> &str { &self.gateway_id }
    fn get_gateway_info(&self) -> &str { &self.gateway_info }
    fn get_details(&self) -> &Option<LocalizableMessage> { &self.details }
}
impl GatewayConnectFaultTrait for GatewayOperationRefused {
    fn get_gateway_type(&self) -> &str { &self.gateway_type }
    fn get_gateway_id(&self) -> &str { &self.gateway_id }
    fn get_gateway_info(&self) -> &str { &self.gateway_info }
    fn get_details(&self) -> &Option<LocalizableMessage> { &self.details }
}
impl GatewayConnectFaultTrait for GatewayToHostConnectFault {
    fn get_gateway_type(&self) -> &str { &self.gateway_type }
    fn get_gateway_id(&self) -> &str { &self.gateway_id }
    fn get_gateway_info(&self) -> &str { &self.gateway_info }
    fn get_details(&self) -> &Option<LocalizableMessage> { &self.details }
}
impl GatewayConnectFaultTrait for GatewayHostNotReachable {
    fn get_gateway_type(&self) -> &str { &self.gateway_type }
    fn get_gateway_id(&self) -> &str { &self.gateway_id }
    fn get_gateway_info(&self) -> &str { &self.gateway_info }
    fn get_details(&self) -> &Option<LocalizableMessage> { &self.details }
}
impl GatewayConnectFaultTrait for GatewayToHostAuthFault {
    fn get_gateway_type(&self) -> &str { &self.gateway_type }
    fn get_gateway_id(&self) -> &str { &self.gateway_id }
    fn get_gateway_info(&self) -> &str { &self.gateway_info }
    fn get_details(&self) -> &Option<LocalizableMessage> { &self.details }
}
impl GatewayConnectFaultTrait for GatewayToHostTrustVerifyFault {
    fn get_gateway_type(&self) -> &str { &self.gateway_type }
    fn get_gateway_id(&self) -> &str { &self.gateway_id }
    fn get_gateway_info(&self) -> &str { &self.gateway_info }
    fn get_details(&self) -> &Option<LocalizableMessage> { &self.details }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn GatewayConnectFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::GatewayConnectFault => Some(from.as_any_ref().downcast_ref::<GatewayConnectFault>()?),
            StructType::GatewayNotFound => Some(from.as_any_ref().downcast_ref::<GatewayNotFound>()?),
            StructType::GatewayNotReachable => Some(from.as_any_ref().downcast_ref::<GatewayNotReachable>()?),
            StructType::GatewayOperationRefused => Some(from.as_any_ref().downcast_ref::<GatewayOperationRefused>()?),
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
            StructType::GatewayConnectFault => Ok(from.as_any_box().downcast::<GatewayConnectFault>()?),
            StructType::GatewayNotFound => Ok(from.as_any_box().downcast::<GatewayNotFound>()?),
            StructType::GatewayNotReachable => Ok(from.as_any_box().downcast::<GatewayNotReachable>()?),
            StructType::GatewayOperationRefused => Ok(from.as_any_box().downcast::<GatewayOperationRefused>()?),
            StructType::GatewayToHostConnectFault => Ok(from.as_any_box().downcast::<GatewayToHostConnectFault>()?),
            StructType::GatewayHostNotReachable => Ok(from.as_any_box().downcast::<GatewayHostNotReachable>()?),
            StructType::GatewayToHostAuthFault => Ok(from.as_any_box().downcast::<GatewayToHostAuthFault>()?),
            StructType::GatewayToHostTrustVerifyFault => Ok(from.as_any_box().downcast::<GatewayToHostTrustVerifyFault>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
