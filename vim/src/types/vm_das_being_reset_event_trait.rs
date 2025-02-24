use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This event records when a virtual machine is reset by
/// HA VM Health Monitoring on hosts that do not support the
/// create screenshot api or if the createscreenshot api fails.
pub trait VmDasBeingResetEventTrait : super::vm_event_trait::VmEventTrait {
    /// The reason why this vm is being reset
    fn get_reason(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn VmDasBeingResetEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VmDasBeingResetEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VmDasBeingResetEventVisitor)
            }
        }

struct VmDasBeingResetEventVisitor;

impl<'de> de::Visitor<'de> for VmDasBeingResetEventVisitor {
    type Value = Box<dyn VmDasBeingResetEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VmDasBeingResetEventTrait JSON object with a _typeName field")
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

impl VmDasBeingResetEventTrait for VmDasBeingResetEvent {
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl VmDasBeingResetEventTrait for VmDasBeingResetWithScreenshotEvent {
    fn get_reason(&self) -> &Option<String> { &self.reason }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VmDasBeingResetEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmDasBeingResetEvent => Some(from.as_any_ref().downcast_ref::<VmDasBeingResetEvent>()?),
            StructType::VmDasBeingResetWithScreenshotEvent => Some(from.as_any_ref().downcast_ref::<VmDasBeingResetWithScreenshotEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmDasBeingResetEvent => Ok(from.as_any_box().downcast::<VmDasBeingResetEvent>()?),
            StructType::VmDasBeingResetWithScreenshotEvent => Ok(from.as_any_box().downcast::<VmDasBeingResetWithScreenshotEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
