use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type represents a sound card in
/// a virtual machine.
pub trait VirtualSoundCardTrait : super::virtual_device_trait::VirtualDeviceTrait {
}
impl<'s> serde::Serialize for dyn VirtualSoundCardTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualSoundCardTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualSoundCardVisitor)
            }
        }

struct VirtualSoundCardVisitor;

impl<'de> de::Visitor<'de> for VirtualSoundCardVisitor {
    type Value = Box<dyn VirtualSoundCardTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualSoundCardTrait JSON object with a _typeName field")
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

impl VirtualSoundCardTrait for VirtualSoundCard {
}
impl VirtualSoundCardTrait for VirtualEnsoniq1371 {
}
impl VirtualSoundCardTrait for VirtualHdAudioCard {
}
impl VirtualSoundCardTrait for VirtualSoundBlaster16 {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualSoundCardTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualSoundCard => Some(from.as_any_ref().downcast_ref::<VirtualSoundCard>()?),
            StructType::VirtualEnsoniq1371 => Some(from.as_any_ref().downcast_ref::<VirtualEnsoniq1371>()?),
            StructType::VirtualHdAudioCard => Some(from.as_any_ref().downcast_ref::<VirtualHdAudioCard>()?),
            StructType::VirtualSoundBlaster16 => Some(from.as_any_ref().downcast_ref::<VirtualSoundBlaster16>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualSoundCard => Ok(from.as_any_box().downcast::<VirtualSoundCard>()?),
            StructType::VirtualEnsoniq1371 => Ok(from.as_any_box().downcast::<VirtualEnsoniq1371>()?),
            StructType::VirtualHdAudioCard => Ok(from.as_any_box().downcast::<VirtualHdAudioCard>()?),
            StructType::VirtualSoundBlaster16 => Ok(from.as_any_box().downcast::<VirtualSoundBlaster16>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
