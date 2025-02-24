use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// One of the virtual machine's virtual disks is not accessible.
pub trait CannotAccessVmDiskTrait : super::cannot_access_vm_device_trait::CannotAccessVmDeviceTrait {
    /// The reason why the disk could not be accessed
    fn get_fault(&self) -> &Box<dyn super::method_fault_trait::MethodFaultTrait>;
}
impl<'s> serde::Serialize for dyn CannotAccessVmDiskTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn CannotAccessVmDiskTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(CannotAccessVmDiskVisitor)
            }
        }

struct CannotAccessVmDiskVisitor;

impl<'de> de::Visitor<'de> for CannotAccessVmDiskVisitor {
    type Value = Box<dyn CannotAccessVmDiskTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid CannotAccessVmDiskTrait JSON object with a _typeName field")
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

impl CannotAccessVmDiskTrait for CannotAccessVmDisk {
    fn get_fault(&self) -> &Box<dyn super::method_fault_trait::MethodFaultTrait> { &self.fault }
}
impl CannotAccessVmDiskTrait for RdmPointsToInaccessibleDisk {
    fn get_fault(&self) -> &Box<dyn super::method_fault_trait::MethodFaultTrait> { &self.fault }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn CannotAccessVmDiskTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::CannotAccessVmDisk => Some(from.as_any_ref().downcast_ref::<CannotAccessVmDisk>()?),
            StructType::RdmPointsToInaccessibleDisk => Some(from.as_any_ref().downcast_ref::<RdmPointsToInaccessibleDisk>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::CannotAccessVmDisk => Ok(from.as_any_box().downcast::<CannotAccessVmDisk>()?),
            StructType::RdmPointsToInaccessibleDisk => Ok(from.as_any_box().downcast::<RdmPointsToInaccessibleDisk>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
