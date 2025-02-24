use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// An InvalidState fault is thrown if the
/// operation failed due to the current state of the system.
pub trait InvalidStateTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn InvalidStateTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn InvalidStateTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(InvalidStateVisitor)
            }
        }

struct InvalidStateVisitor;

impl<'de> de::Visitor<'de> for InvalidStateVisitor {
    type Value = Box<dyn InvalidStateTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid InvalidStateTrait JSON object with a _typeName field")
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

impl InvalidStateTrait for InvalidState {
}
impl InvalidStateTrait for CannotPowerOffVmInCluster {
}
impl InvalidStateTrait for EncryptionKeyRequired {
}
impl InvalidStateTrait for InvalidDatastoreState {
}
impl InvalidStateTrait for InvalidHostState {
}
impl InvalidStateTrait for InvalidHostConnectionState {
}
impl InvalidStateTrait for InvalidPowerState {
}
impl InvalidStateTrait for InvalidVmState {
}
impl InvalidStateTrait for MksConnectionLimitReached {
}
impl InvalidStateTrait for NoActiveHostInCluster {
}
impl InvalidStateTrait for OvfConsumerPowerOnFault {
}
impl InvalidStateTrait for QuestionPending {
}
impl InvalidStateTrait for VmPowerOnDisabled {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn InvalidStateTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidState => Some(from.as_any_ref().downcast_ref::<InvalidState>()?),
            StructType::CannotPowerOffVmInCluster => Some(from.as_any_ref().downcast_ref::<CannotPowerOffVmInCluster>()?),
            StructType::EncryptionKeyRequired => Some(from.as_any_ref().downcast_ref::<EncryptionKeyRequired>()?),
            StructType::InvalidDatastoreState => Some(from.as_any_ref().downcast_ref::<InvalidDatastoreState>()?),
            StructType::InvalidHostState => Some(from.as_any_ref().downcast_ref::<InvalidHostState>()?),
            StructType::InvalidHostConnectionState => Some(from.as_any_ref().downcast_ref::<InvalidHostConnectionState>()?),
            StructType::InvalidPowerState => Some(from.as_any_ref().downcast_ref::<InvalidPowerState>()?),
            StructType::InvalidVmState => Some(from.as_any_ref().downcast_ref::<InvalidVmState>()?),
            StructType::MksConnectionLimitReached => Some(from.as_any_ref().downcast_ref::<MksConnectionLimitReached>()?),
            StructType::NoActiveHostInCluster => Some(from.as_any_ref().downcast_ref::<NoActiveHostInCluster>()?),
            StructType::OvfConsumerPowerOnFault => Some(from.as_any_ref().downcast_ref::<OvfConsumerPowerOnFault>()?),
            StructType::QuestionPending => Some(from.as_any_ref().downcast_ref::<QuestionPending>()?),
            StructType::VmPowerOnDisabled => Some(from.as_any_ref().downcast_ref::<VmPowerOnDisabled>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::InvalidState => Ok(from.as_any_box().downcast::<InvalidState>()?),
            StructType::CannotPowerOffVmInCluster => Ok(from.as_any_box().downcast::<CannotPowerOffVmInCluster>()?),
            StructType::EncryptionKeyRequired => Ok(from.as_any_box().downcast::<EncryptionKeyRequired>()?),
            StructType::InvalidDatastoreState => Ok(from.as_any_box().downcast::<InvalidDatastoreState>()?),
            StructType::InvalidHostState => Ok(from.as_any_box().downcast::<InvalidHostState>()?),
            StructType::InvalidHostConnectionState => Ok(from.as_any_box().downcast::<InvalidHostConnectionState>()?),
            StructType::InvalidPowerState => Ok(from.as_any_box().downcast::<InvalidPowerState>()?),
            StructType::InvalidVmState => Ok(from.as_any_box().downcast::<InvalidVmState>()?),
            StructType::MksConnectionLimitReached => Ok(from.as_any_box().downcast::<MksConnectionLimitReached>()?),
            StructType::NoActiveHostInCluster => Ok(from.as_any_box().downcast::<NoActiveHostInCluster>()?),
            StructType::OvfConsumerPowerOnFault => Ok(from.as_any_box().downcast::<OvfConsumerPowerOnFault>()?),
            StructType::QuestionPending => Ok(from.as_any_box().downcast::<QuestionPending>()?),
            StructType::VmPowerOnDisabled => Ok(from.as_any_box().downcast::<VmPowerOnDisabled>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
