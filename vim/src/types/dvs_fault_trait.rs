use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base class for faults that can be thrown while invoking a distributed virtual switch
/// operation.
pub trait DvsFaultTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn DvsFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DvsFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DvsFaultVisitor)
            }
        }

struct DvsFaultVisitor;

impl<'de> de::Visitor<'de> for DvsFaultVisitor {
    type Value = Box<dyn DvsFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DvsFaultTrait JSON object with a _typeName field")
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

impl DvsFaultTrait for DvsFault {
}
impl DvsFaultTrait for BackupBlobReadFailure {
}
impl DvsFaultTrait for BackupBlobWriteFailure {
}
impl DvsFaultTrait for CollectorAddressUnset {
}
impl DvsFaultTrait for ConflictingConfiguration {
}
impl DvsFaultTrait for DvsApplyOperationFault {
}
impl DvsFaultTrait for DvsNotAuthorized {
}
impl DvsFaultTrait for DvsOperationBulkFault {
}
impl DvsFaultTrait for DvsScopeViolated {
}
impl DvsFaultTrait for ImportHostAddFailure {
}
impl DvsFaultTrait for ImportOperationBulkFault {
}
impl DvsFaultTrait for InvalidIpfixConfig {
}
impl DvsFaultTrait for RollbackFailure {
}
impl DvsFaultTrait for SwitchIpUnset {
}
impl DvsFaultTrait for SwitchNotInUpgradeMode {
}
impl DvsFaultTrait for VspanDestPortConflict {
}
impl DvsFaultTrait for VspanPortConflict {
}
impl DvsFaultTrait for VspanPortMoveFault {
}
impl DvsFaultTrait for VspanPortPromiscChangeFault {
}
impl DvsFaultTrait for VspanPortgroupPromiscChangeFault {
}
impl DvsFaultTrait for VspanPortgroupTypeChangeFault {
}
impl DvsFaultTrait for VspanPromiscuousPortNotSupported {
}
impl DvsFaultTrait for VspanSameSessionPortConflict {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DvsFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsFault => Some(from.as_any_ref().downcast_ref::<DvsFault>()?),
            StructType::BackupBlobReadFailure => Some(from.as_any_ref().downcast_ref::<BackupBlobReadFailure>()?),
            StructType::BackupBlobWriteFailure => Some(from.as_any_ref().downcast_ref::<BackupBlobWriteFailure>()?),
            StructType::CollectorAddressUnset => Some(from.as_any_ref().downcast_ref::<CollectorAddressUnset>()?),
            StructType::ConflictingConfiguration => Some(from.as_any_ref().downcast_ref::<ConflictingConfiguration>()?),
            StructType::DvsApplyOperationFault => Some(from.as_any_ref().downcast_ref::<DvsApplyOperationFault>()?),
            StructType::DvsNotAuthorized => Some(from.as_any_ref().downcast_ref::<DvsNotAuthorized>()?),
            StructType::DvsOperationBulkFault => Some(from.as_any_ref().downcast_ref::<DvsOperationBulkFault>()?),
            StructType::DvsScopeViolated => Some(from.as_any_ref().downcast_ref::<DvsScopeViolated>()?),
            StructType::ImportHostAddFailure => Some(from.as_any_ref().downcast_ref::<ImportHostAddFailure>()?),
            StructType::ImportOperationBulkFault => Some(from.as_any_ref().downcast_ref::<ImportOperationBulkFault>()?),
            StructType::InvalidIpfixConfig => Some(from.as_any_ref().downcast_ref::<InvalidIpfixConfig>()?),
            StructType::RollbackFailure => Some(from.as_any_ref().downcast_ref::<RollbackFailure>()?),
            StructType::SwitchIpUnset => Some(from.as_any_ref().downcast_ref::<SwitchIpUnset>()?),
            StructType::SwitchNotInUpgradeMode => Some(from.as_any_ref().downcast_ref::<SwitchNotInUpgradeMode>()?),
            StructType::VspanDestPortConflict => Some(from.as_any_ref().downcast_ref::<VspanDestPortConflict>()?),
            StructType::VspanPortConflict => Some(from.as_any_ref().downcast_ref::<VspanPortConflict>()?),
            StructType::VspanPortMoveFault => Some(from.as_any_ref().downcast_ref::<VspanPortMoveFault>()?),
            StructType::VspanPortPromiscChangeFault => Some(from.as_any_ref().downcast_ref::<VspanPortPromiscChangeFault>()?),
            StructType::VspanPortgroupPromiscChangeFault => Some(from.as_any_ref().downcast_ref::<VspanPortgroupPromiscChangeFault>()?),
            StructType::VspanPortgroupTypeChangeFault => Some(from.as_any_ref().downcast_ref::<VspanPortgroupTypeChangeFault>()?),
            StructType::VspanPromiscuousPortNotSupported => Some(from.as_any_ref().downcast_ref::<VspanPromiscuousPortNotSupported>()?),
            StructType::VspanSameSessionPortConflict => Some(from.as_any_ref().downcast_ref::<VspanSameSessionPortConflict>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsFault => Ok(from.as_any_box().downcast::<DvsFault>()?),
            StructType::BackupBlobReadFailure => Ok(from.as_any_box().downcast::<BackupBlobReadFailure>()?),
            StructType::BackupBlobWriteFailure => Ok(from.as_any_box().downcast::<BackupBlobWriteFailure>()?),
            StructType::CollectorAddressUnset => Ok(from.as_any_box().downcast::<CollectorAddressUnset>()?),
            StructType::ConflictingConfiguration => Ok(from.as_any_box().downcast::<ConflictingConfiguration>()?),
            StructType::DvsApplyOperationFault => Ok(from.as_any_box().downcast::<DvsApplyOperationFault>()?),
            StructType::DvsNotAuthorized => Ok(from.as_any_box().downcast::<DvsNotAuthorized>()?),
            StructType::DvsOperationBulkFault => Ok(from.as_any_box().downcast::<DvsOperationBulkFault>()?),
            StructType::DvsScopeViolated => Ok(from.as_any_box().downcast::<DvsScopeViolated>()?),
            StructType::ImportHostAddFailure => Ok(from.as_any_box().downcast::<ImportHostAddFailure>()?),
            StructType::ImportOperationBulkFault => Ok(from.as_any_box().downcast::<ImportOperationBulkFault>()?),
            StructType::InvalidIpfixConfig => Ok(from.as_any_box().downcast::<InvalidIpfixConfig>()?),
            StructType::RollbackFailure => Ok(from.as_any_box().downcast::<RollbackFailure>()?),
            StructType::SwitchIpUnset => Ok(from.as_any_box().downcast::<SwitchIpUnset>()?),
            StructType::SwitchNotInUpgradeMode => Ok(from.as_any_box().downcast::<SwitchNotInUpgradeMode>()?),
            StructType::VspanDestPortConflict => Ok(from.as_any_box().downcast::<VspanDestPortConflict>()?),
            StructType::VspanPortConflict => Ok(from.as_any_box().downcast::<VspanPortConflict>()?),
            StructType::VspanPortMoveFault => Ok(from.as_any_box().downcast::<VspanPortMoveFault>()?),
            StructType::VspanPortPromiscChangeFault => Ok(from.as_any_box().downcast::<VspanPortPromiscChangeFault>()?),
            StructType::VspanPortgroupPromiscChangeFault => Ok(from.as_any_box().downcast::<VspanPortgroupPromiscChangeFault>()?),
            StructType::VspanPortgroupTypeChangeFault => Ok(from.as_any_box().downcast::<VspanPortgroupTypeChangeFault>()?),
            StructType::VspanPromiscuousPortNotSupported => Ok(from.as_any_box().downcast::<VspanPromiscuousPortNotSupported>()?),
            StructType::VspanSameSessionPortConflict => Ok(from.as_any_box().downcast::<VspanSameSessionPortConflict>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
