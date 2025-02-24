use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The VirtualSoundCardOption data class contains the options for the
/// virtual sound card class.
pub trait VirtualSoundCardOptionTrait : super::virtual_device_option_trait::VirtualDeviceOptionTrait {
}
impl<'s> serde::Serialize for dyn VirtualSoundCardOptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualSoundCardOptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualSoundCardOptionVisitor)
            }
        }

struct VirtualSoundCardOptionVisitor;

impl<'de> de::Visitor<'de> for VirtualSoundCardOptionVisitor {
    type Value = Box<dyn VirtualSoundCardOptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualSoundCardOptionTrait JSON object with a _typeName field")
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

impl VirtualSoundCardOptionTrait for VirtualSoundCardOption {
}
impl VirtualSoundCardOptionTrait for VirtualEnsoniq1371Option {
}
impl VirtualSoundCardOptionTrait for VirtualHdAudioCardOption {
}
impl VirtualSoundCardOptionTrait for VirtualSoundBlaster16Option {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualSoundCardOptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualSoundCardOption => Some(from.as_any_ref().downcast_ref::<VirtualSoundCardOption>()?),
            StructType::VirtualEnsoniq1371Option => Some(from.as_any_ref().downcast_ref::<VirtualEnsoniq1371Option>()?),
            StructType::VirtualHdAudioCardOption => Some(from.as_any_ref().downcast_ref::<VirtualHdAudioCardOption>()?),
            StructType::VirtualSoundBlaster16Option => Some(from.as_any_ref().downcast_ref::<VirtualSoundBlaster16Option>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualSoundCardOption => Ok(from.as_any_box().downcast::<VirtualSoundCardOption>()?),
            StructType::VirtualEnsoniq1371Option => Ok(from.as_any_box().downcast::<VirtualEnsoniq1371Option>()?),
            StructType::VirtualHdAudioCardOption => Ok(from.as_any_box().downcast::<VirtualHdAudioCardOption>()?),
            StructType::VirtualSoundBlaster16Option => Ok(from.as_any_box().downcast::<VirtualSoundBlaster16Option>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
