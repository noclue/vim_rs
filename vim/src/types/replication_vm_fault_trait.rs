use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A ReplicationVmFault is thrown when there is an issue with
/// an operation performed on a replicated *VirtualMachine*
pub trait ReplicationVmFaultTrait : super::replication_fault_trait::ReplicationFaultTrait {
    /// The reason for the failure.
    /// 
    /// One of the above.
    fn get_reason(&self) -> &str;
    /// The current *ReplicationVmState_enum* of the
    /// *VirtualMachine*
    fn get_state(&self) -> &Option<String>;
    /// The name of the instance currently being created.
    fn get_instance_id(&self) -> &Option<String>;
    /// The virtual machine, for identification purposes.
    /// 
    /// Refers instance of *VirtualMachine*.
    fn get_vm(&self) -> &ManagedObjectReference;
}
impl<'s> serde::Serialize for dyn ReplicationVmFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ReplicationVmFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ReplicationVmFaultVisitor)
            }
        }

struct ReplicationVmFaultVisitor;

impl<'de> de::Visitor<'de> for ReplicationVmFaultVisitor {
    type Value = Box<dyn ReplicationVmFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ReplicationVmFaultTrait JSON object with a _typeName field")
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

impl ReplicationVmFaultTrait for ReplicationVmFault {
    fn get_reason(&self) -> &str { &self.reason }
    fn get_state(&self) -> &Option<String> { &self.state }
    fn get_instance_id(&self) -> &Option<String> { &self.instance_id }
    fn get_vm(&self) -> &ManagedObjectReference { &self.vm }
}
impl ReplicationVmFaultTrait for ReplicationVmInProgressFault {
    fn get_reason(&self) -> &str { &self.reason }
    fn get_state(&self) -> &Option<String> { &self.state }
    fn get_instance_id(&self) -> &Option<String> { &self.instance_id }
    fn get_vm(&self) -> &ManagedObjectReference { &self.vm }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ReplicationVmFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ReplicationVmFault => Some(from.as_any_ref().downcast_ref::<ReplicationVmFault>()?),
            StructType::ReplicationVmInProgressFault => Some(from.as_any_ref().downcast_ref::<ReplicationVmInProgressFault>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ReplicationVmFault => Ok(from.as_any_box().downcast::<ReplicationVmFault>()?),
            StructType::ReplicationVmInProgressFault => Ok(from.as_any_box().downcast::<ReplicationVmInProgressFault>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
