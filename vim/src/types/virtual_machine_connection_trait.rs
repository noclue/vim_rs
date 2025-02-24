use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *VirtualMachineConnection* object describes a connection to the virtual
/// machine.
/// 
/// ***Since:*** vSphere API Release 7.0.1.0
pub trait VirtualMachineConnectionTrait : super::data_object_trait::DataObjectTrait {
    /// The unique identifier associated with the connection.
    /// 
    /// The label is a UTF-8 string which specifies a unique identifier for
    /// a connection.
    fn get_label(&self) -> &str;
    /// The client identifer.
    /// 
    /// This identifier is a UTF-8 string which is semantically meaningful
    /// for the connection. Examples of the client identifier are an IP
    /// address (V4 or V6) with or without a port specification, a machine
    /// name that requires a DNS lookup, or any other network oriented
    /// identification scheme.
    fn get_client(&self) -> &str;
    /// The name of the user authorizing the connection.
    /// 
    /// This is used for auditing.
    fn get_user_name(&self) -> &str;
}
impl<'s> serde::Serialize for dyn VirtualMachineConnectionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualMachineConnectionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualMachineConnectionVisitor)
            }
        }

struct VirtualMachineConnectionVisitor;

impl<'de> de::Visitor<'de> for VirtualMachineConnectionVisitor {
    type Value = Box<dyn VirtualMachineConnectionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualMachineConnectionTrait JSON object with a _typeName field")
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

impl VirtualMachineConnectionTrait for VirtualMachineConnection {
    fn get_label(&self) -> &str { &self.label }
    fn get_client(&self) -> &str { &self.client }
    fn get_user_name(&self) -> &str { &self.user_name }
}
impl VirtualMachineConnectionTrait for VirtualMachineMksConnection {
    fn get_label(&self) -> &str { &self.label }
    fn get_client(&self) -> &str { &self.client }
    fn get_user_name(&self) -> &str { &self.user_name }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualMachineConnectionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualMachineConnection => Some(from.as_any_ref().downcast_ref::<VirtualMachineConnection>()?),
            StructType::VirtualMachineMksConnection => Some(from.as_any_ref().downcast_ref::<VirtualMachineMksConnection>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualMachineConnection => Ok(from.as_any_box().downcast::<VirtualMachineConnection>()?),
            StructType::VirtualMachineMksConnection => Ok(from.as_any_box().downcast::<VirtualMachineMksConnection>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
