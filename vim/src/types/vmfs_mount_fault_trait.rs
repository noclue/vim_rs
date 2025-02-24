use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This is a base class for all VMFS volume mount related faults.
pub trait VmfsMountFaultTrait : super::host_config_fault_trait::HostConfigFaultTrait {
    /// Vmfs volume uuid
    fn get_uuid(&self) -> &str;
}
impl<'s> serde::Serialize for dyn VmfsMountFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VmfsMountFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VmfsMountFaultVisitor)
            }
        }

struct VmfsMountFaultVisitor;

impl<'de> de::Visitor<'de> for VmfsMountFaultVisitor {
    type Value = Box<dyn VmfsMountFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VmfsMountFaultTrait JSON object with a _typeName field")
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

impl VmfsMountFaultTrait for VmfsMountFault {
    fn get_uuid(&self) -> &str { &self.uuid }
}
impl VmfsMountFaultTrait for VmfsAlreadyMounted {
    fn get_uuid(&self) -> &str { &self.uuid }
}
impl VmfsMountFaultTrait for VmfsAmbiguousMount {
    fn get_uuid(&self) -> &str { &self.uuid }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VmfsMountFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmfsMountFault => Some(from.as_any_ref().downcast_ref::<VmfsMountFault>()?),
            StructType::VmfsAlreadyMounted => Some(from.as_any_ref().downcast_ref::<VmfsAlreadyMounted>()?),
            StructType::VmfsAmbiguousMount => Some(from.as_any_ref().downcast_ref::<VmfsAmbiguousMount>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmfsMountFault => Ok(from.as_any_box().downcast::<VmfsMountFault>()?),
            StructType::VmfsAlreadyMounted => Ok(from.as_any_box().downcast::<VmfsAlreadyMounted>()?),
            StructType::VmfsAmbiguousMount => Ok(from.as_any_box().downcast::<VmfsAmbiguousMount>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
