use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// FaultToleranceConfigInfo is a data object type containing Fault Tolerance
/// settings for this virtual machine.
/// 
/// role, instanceUuids and configPaths contain information about the
/// whole fault tolerance group.
pub trait FaultToleranceConfigInfoTrait : super::data_object_trait::DataObjectTrait {
    /// The index of the current VM in instanceUuids array starting from 1, so
    /// 1 means that it is the primary VM.
    fn get_role(&self) -> i32;
    /// The instanceUuid of all the VMs in this fault tolerance group.
    /// 
    /// The
    /// first element is the instanceUuid of the primary VM.
    fn get_instance_uuids(&self) -> &Vec<String>;
    /// The configuration file path for all the VMs in this fault tolerance
    /// group.
    fn get_config_paths(&self) -> &Vec<String>;
    /// Indicates whether a secondary VM is orphaned (no longer associated with
    /// the primary VM).
    fn get_orphaned(&self) -> Option<bool>;
}
impl<'s> serde::Serialize for dyn FaultToleranceConfigInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn FaultToleranceConfigInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(FaultToleranceConfigInfoVisitor)
            }
        }

struct FaultToleranceConfigInfoVisitor;

impl<'de> de::Visitor<'de> for FaultToleranceConfigInfoVisitor {
    type Value = Box<dyn FaultToleranceConfigInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid FaultToleranceConfigInfoTrait JSON object with a _typeName field")
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

impl FaultToleranceConfigInfoTrait for FaultToleranceConfigInfo {
    fn get_role(&self) -> i32 { self.role }
    fn get_instance_uuids(&self) -> &Vec<String> { &self.instance_uuids }
    fn get_config_paths(&self) -> &Vec<String> { &self.config_paths }
    fn get_orphaned(&self) -> Option<bool> { self.orphaned }
}
impl FaultToleranceConfigInfoTrait for FaultTolerancePrimaryConfigInfo {
    fn get_role(&self) -> i32 { self.role }
    fn get_instance_uuids(&self) -> &Vec<String> { &self.instance_uuids }
    fn get_config_paths(&self) -> &Vec<String> { &self.config_paths }
    fn get_orphaned(&self) -> Option<bool> { self.orphaned }
}
impl FaultToleranceConfigInfoTrait for FaultToleranceSecondaryConfigInfo {
    fn get_role(&self) -> i32 { self.role }
    fn get_instance_uuids(&self) -> &Vec<String> { &self.instance_uuids }
    fn get_config_paths(&self) -> &Vec<String> { &self.config_paths }
    fn get_orphaned(&self) -> Option<bool> { self.orphaned }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn FaultToleranceConfigInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::FaultToleranceConfigInfo => Some(from.as_any_ref().downcast_ref::<FaultToleranceConfigInfo>()?),
            StructType::FaultTolerancePrimaryConfigInfo => Some(from.as_any_ref().downcast_ref::<FaultTolerancePrimaryConfigInfo>()?),
            StructType::FaultToleranceSecondaryConfigInfo => Some(from.as_any_ref().downcast_ref::<FaultToleranceSecondaryConfigInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::FaultToleranceConfigInfo => Ok(from.as_any_box().downcast::<FaultToleranceConfigInfo>()?),
            StructType::FaultTolerancePrimaryConfigInfo => Ok(from.as_any_box().downcast::<FaultTolerancePrimaryConfigInfo>()?),
            StructType::FaultToleranceSecondaryConfigInfo => Ok(from.as_any_box().downcast::<FaultToleranceSecondaryConfigInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
