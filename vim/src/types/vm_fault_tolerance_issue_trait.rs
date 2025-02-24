use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base object type for issues that can occur during operations
/// related to fault tolerance protection for virtual machines.
pub trait VmFaultToleranceIssueTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn VmFaultToleranceIssueTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VmFaultToleranceIssueTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VmFaultToleranceIssueVisitor)
            }
        }

struct VmFaultToleranceIssueVisitor;

impl<'de> de::Visitor<'de> for VmFaultToleranceIssueVisitor {
    type Value = Box<dyn VmFaultToleranceIssueTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VmFaultToleranceIssueTrait JSON object with a _typeName field")
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

impl VmFaultToleranceIssueTrait for VmFaultToleranceIssue {
}
impl VmFaultToleranceIssueTrait for CannotChangeDrsBehaviorForFtSecondary {
}
impl VmFaultToleranceIssueTrait for CannotChangeHaSettingsForFtSecondary {
}
impl VmFaultToleranceIssueTrait for CannotComputeFtCompatibleHosts {
}
impl VmFaultToleranceIssueTrait for FaultToleranceNotLicensed {
}
impl VmFaultToleranceIssueTrait for FaultTolerancePrimaryPowerOnNotAttempted {
}
impl VmFaultToleranceIssueTrait for FtIssuesOnHost {
}
impl VmFaultToleranceIssueTrait for HostIncompatibleForFaultTolerance {
}
impl VmFaultToleranceIssueTrait for IncompatibleHostForFtSecondary {
}
impl VmFaultToleranceIssueTrait for InvalidOperationOnSecondaryVm {
}
impl VmFaultToleranceIssueTrait for NoHostSuitableForFtSecondary {
}
impl VmFaultToleranceIssueTrait for NotSupportedDeviceForFt {
}
impl VmFaultToleranceIssueTrait for PowerOnFtSecondaryFailed {
}
impl VmFaultToleranceIssueTrait for SecondaryVmAlreadyDisabled {
}
impl VmFaultToleranceIssueTrait for SecondaryVmAlreadyEnabled {
}
impl VmFaultToleranceIssueTrait for SecondaryVmAlreadyRegistered {
}
impl VmFaultToleranceIssueTrait for SecondaryVmNotRegistered {
}
impl VmFaultToleranceIssueTrait for VmFaultToleranceConfigIssue {
}
impl VmFaultToleranceIssueTrait for VmFaultToleranceConfigIssueWrapper {
}
impl VmFaultToleranceIssueTrait for VmFaultToleranceInvalidFileBacking {
}
impl VmFaultToleranceIssueTrait for VmFaultToleranceOpIssuesList {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VmFaultToleranceIssueTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmFaultToleranceIssue => Some(from.as_any_ref().downcast_ref::<VmFaultToleranceIssue>()?),
            StructType::CannotChangeDrsBehaviorForFtSecondary => Some(from.as_any_ref().downcast_ref::<CannotChangeDrsBehaviorForFtSecondary>()?),
            StructType::CannotChangeHaSettingsForFtSecondary => Some(from.as_any_ref().downcast_ref::<CannotChangeHaSettingsForFtSecondary>()?),
            StructType::CannotComputeFtCompatibleHosts => Some(from.as_any_ref().downcast_ref::<CannotComputeFtCompatibleHosts>()?),
            StructType::FaultToleranceNotLicensed => Some(from.as_any_ref().downcast_ref::<FaultToleranceNotLicensed>()?),
            StructType::FaultTolerancePrimaryPowerOnNotAttempted => Some(from.as_any_ref().downcast_ref::<FaultTolerancePrimaryPowerOnNotAttempted>()?),
            StructType::FtIssuesOnHost => Some(from.as_any_ref().downcast_ref::<FtIssuesOnHost>()?),
            StructType::HostIncompatibleForFaultTolerance => Some(from.as_any_ref().downcast_ref::<HostIncompatibleForFaultTolerance>()?),
            StructType::IncompatibleHostForFtSecondary => Some(from.as_any_ref().downcast_ref::<IncompatibleHostForFtSecondary>()?),
            StructType::InvalidOperationOnSecondaryVm => Some(from.as_any_ref().downcast_ref::<InvalidOperationOnSecondaryVm>()?),
            StructType::NoHostSuitableForFtSecondary => Some(from.as_any_ref().downcast_ref::<NoHostSuitableForFtSecondary>()?),
            StructType::NotSupportedDeviceForFt => Some(from.as_any_ref().downcast_ref::<NotSupportedDeviceForFt>()?),
            StructType::PowerOnFtSecondaryFailed => Some(from.as_any_ref().downcast_ref::<PowerOnFtSecondaryFailed>()?),
            StructType::SecondaryVmAlreadyDisabled => Some(from.as_any_ref().downcast_ref::<SecondaryVmAlreadyDisabled>()?),
            StructType::SecondaryVmAlreadyEnabled => Some(from.as_any_ref().downcast_ref::<SecondaryVmAlreadyEnabled>()?),
            StructType::SecondaryVmAlreadyRegistered => Some(from.as_any_ref().downcast_ref::<SecondaryVmAlreadyRegistered>()?),
            StructType::SecondaryVmNotRegistered => Some(from.as_any_ref().downcast_ref::<SecondaryVmNotRegistered>()?),
            StructType::VmFaultToleranceConfigIssue => Some(from.as_any_ref().downcast_ref::<VmFaultToleranceConfigIssue>()?),
            StructType::VmFaultToleranceConfigIssueWrapper => Some(from.as_any_ref().downcast_ref::<VmFaultToleranceConfigIssueWrapper>()?),
            StructType::VmFaultToleranceInvalidFileBacking => Some(from.as_any_ref().downcast_ref::<VmFaultToleranceInvalidFileBacking>()?),
            StructType::VmFaultToleranceOpIssuesList => Some(from.as_any_ref().downcast_ref::<VmFaultToleranceOpIssuesList>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmFaultToleranceIssue => Ok(from.as_any_box().downcast::<VmFaultToleranceIssue>()?),
            StructType::CannotChangeDrsBehaviorForFtSecondary => Ok(from.as_any_box().downcast::<CannotChangeDrsBehaviorForFtSecondary>()?),
            StructType::CannotChangeHaSettingsForFtSecondary => Ok(from.as_any_box().downcast::<CannotChangeHaSettingsForFtSecondary>()?),
            StructType::CannotComputeFtCompatibleHosts => Ok(from.as_any_box().downcast::<CannotComputeFtCompatibleHosts>()?),
            StructType::FaultToleranceNotLicensed => Ok(from.as_any_box().downcast::<FaultToleranceNotLicensed>()?),
            StructType::FaultTolerancePrimaryPowerOnNotAttempted => Ok(from.as_any_box().downcast::<FaultTolerancePrimaryPowerOnNotAttempted>()?),
            StructType::FtIssuesOnHost => Ok(from.as_any_box().downcast::<FtIssuesOnHost>()?),
            StructType::HostIncompatibleForFaultTolerance => Ok(from.as_any_box().downcast::<HostIncompatibleForFaultTolerance>()?),
            StructType::IncompatibleHostForFtSecondary => Ok(from.as_any_box().downcast::<IncompatibleHostForFtSecondary>()?),
            StructType::InvalidOperationOnSecondaryVm => Ok(from.as_any_box().downcast::<InvalidOperationOnSecondaryVm>()?),
            StructType::NoHostSuitableForFtSecondary => Ok(from.as_any_box().downcast::<NoHostSuitableForFtSecondary>()?),
            StructType::NotSupportedDeviceForFt => Ok(from.as_any_box().downcast::<NotSupportedDeviceForFt>()?),
            StructType::PowerOnFtSecondaryFailed => Ok(from.as_any_box().downcast::<PowerOnFtSecondaryFailed>()?),
            StructType::SecondaryVmAlreadyDisabled => Ok(from.as_any_box().downcast::<SecondaryVmAlreadyDisabled>()?),
            StructType::SecondaryVmAlreadyEnabled => Ok(from.as_any_box().downcast::<SecondaryVmAlreadyEnabled>()?),
            StructType::SecondaryVmAlreadyRegistered => Ok(from.as_any_box().downcast::<SecondaryVmAlreadyRegistered>()?),
            StructType::SecondaryVmNotRegistered => Ok(from.as_any_box().downcast::<SecondaryVmNotRegistered>()?),
            StructType::VmFaultToleranceConfigIssue => Ok(from.as_any_box().downcast::<VmFaultToleranceConfigIssue>()?),
            StructType::VmFaultToleranceConfigIssueWrapper => Ok(from.as_any_box().downcast::<VmFaultToleranceConfigIssueWrapper>()?),
            StructType::VmFaultToleranceInvalidFileBacking => Ok(from.as_any_box().downcast::<VmFaultToleranceInvalidFileBacking>()?),
            StructType::VmFaultToleranceOpIssuesList => Ok(from.as_any_box().downcast::<VmFaultToleranceOpIssuesList>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
