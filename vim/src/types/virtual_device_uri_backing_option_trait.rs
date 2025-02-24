use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *VirtualDeviceURIBackingOption* data object type describes network communication
/// options for virtual devices.
/// 
/// When establishing a connection with a remote system on the network,
/// the virtual machine can act as a server or a client.
/// When the virtual machine acts as a server, it accepts a connection.
/// When the virtual machine acts as a client, it initiates the connection.
pub trait VirtualDeviceUriBackingOptionTrait : super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait {
    /// List of possible directions.
    /// 
    /// Valid directions are:
    /// - *server*
    /// - *client*
    fn get_directions(&self) -> &ChoiceOption;
}
impl<'s> serde::Serialize for dyn VirtualDeviceUriBackingOptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualDeviceUriBackingOptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualDeviceUriBackingOptionVisitor)
            }
        }

struct VirtualDeviceUriBackingOptionVisitor;

impl<'de> de::Visitor<'de> for VirtualDeviceUriBackingOptionVisitor {
    type Value = Box<dyn VirtualDeviceUriBackingOptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualDeviceUriBackingOptionTrait JSON object with a _typeName field")
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

impl VirtualDeviceUriBackingOptionTrait for VirtualDeviceUriBackingOption {
    fn get_directions(&self) -> &ChoiceOption { &self.directions }
}
impl VirtualDeviceUriBackingOptionTrait for VirtualSerialPortUriBackingOption {
    fn get_directions(&self) -> &ChoiceOption { &self.directions }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualDeviceUriBackingOptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceUriBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualDeviceUriBackingOption>()?),
            StructType::VirtualSerialPortUriBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortUriBackingOption>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceUriBackingOption => Ok(from.as_any_box().downcast::<VirtualDeviceUriBackingOption>()?),
            StructType::VirtualSerialPortUriBackingOption => Ok(from.as_any_box().downcast::<VirtualSerialPortUriBackingOption>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
