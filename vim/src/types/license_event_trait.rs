use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This is a base licensing event to group all license events.
pub trait LicenseEventTrait : super::event_trait::EventTrait {
}
impl<'s> serde::Serialize for dyn LicenseEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn LicenseEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(LicenseEventVisitor)
            }
        }

struct LicenseEventVisitor;

impl<'de> de::Visitor<'de> for LicenseEventVisitor {
    type Value = Box<dyn LicenseEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid LicenseEventTrait JSON object with a _typeName field")
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

impl LicenseEventTrait for LicenseEvent {
}
impl LicenseEventTrait for AllVirtualMachinesLicensedEvent {
}
impl LicenseEventTrait for HostInventoryFullEvent {
}
impl LicenseEventTrait for HostLicenseExpiredEvent {
}
impl LicenseEventTrait for IncorrectHostInformationEvent {
}
impl LicenseEventTrait for InvalidEditionEvent {
}
impl LicenseEventTrait for LicenseNonComplianceEvent {
}
impl LicenseEventTrait for LicenseRestrictedEvent {
}
impl LicenseEventTrait for LicenseServerAvailableEvent {
}
impl LicenseEventTrait for LicenseServerUnavailableEvent {
}
impl LicenseEventTrait for NoLicenseEvent {
}
impl LicenseEventTrait for ServerLicenseExpiredEvent {
}
impl LicenseEventTrait for UnlicensedVirtualMachinesEvent {
}
impl LicenseEventTrait for UnlicensedVirtualMachinesFoundEvent {
}
impl LicenseEventTrait for VMotionLicenseExpiredEvent {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn LicenseEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::LicenseEvent => Some(from.as_any_ref().downcast_ref::<LicenseEvent>()?),
            StructType::AllVirtualMachinesLicensedEvent => Some(from.as_any_ref().downcast_ref::<AllVirtualMachinesLicensedEvent>()?),
            StructType::HostInventoryFullEvent => Some(from.as_any_ref().downcast_ref::<HostInventoryFullEvent>()?),
            StructType::HostLicenseExpiredEvent => Some(from.as_any_ref().downcast_ref::<HostLicenseExpiredEvent>()?),
            StructType::IncorrectHostInformationEvent => Some(from.as_any_ref().downcast_ref::<IncorrectHostInformationEvent>()?),
            StructType::InvalidEditionEvent => Some(from.as_any_ref().downcast_ref::<InvalidEditionEvent>()?),
            StructType::LicenseNonComplianceEvent => Some(from.as_any_ref().downcast_ref::<LicenseNonComplianceEvent>()?),
            StructType::LicenseRestrictedEvent => Some(from.as_any_ref().downcast_ref::<LicenseRestrictedEvent>()?),
            StructType::LicenseServerAvailableEvent => Some(from.as_any_ref().downcast_ref::<LicenseServerAvailableEvent>()?),
            StructType::LicenseServerUnavailableEvent => Some(from.as_any_ref().downcast_ref::<LicenseServerUnavailableEvent>()?),
            StructType::NoLicenseEvent => Some(from.as_any_ref().downcast_ref::<NoLicenseEvent>()?),
            StructType::ServerLicenseExpiredEvent => Some(from.as_any_ref().downcast_ref::<ServerLicenseExpiredEvent>()?),
            StructType::UnlicensedVirtualMachinesEvent => Some(from.as_any_ref().downcast_ref::<UnlicensedVirtualMachinesEvent>()?),
            StructType::UnlicensedVirtualMachinesFoundEvent => Some(from.as_any_ref().downcast_ref::<UnlicensedVirtualMachinesFoundEvent>()?),
            StructType::VMotionLicenseExpiredEvent => Some(from.as_any_ref().downcast_ref::<VMotionLicenseExpiredEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::LicenseEvent => Ok(from.as_any_box().downcast::<LicenseEvent>()?),
            StructType::AllVirtualMachinesLicensedEvent => Ok(from.as_any_box().downcast::<AllVirtualMachinesLicensedEvent>()?),
            StructType::HostInventoryFullEvent => Ok(from.as_any_box().downcast::<HostInventoryFullEvent>()?),
            StructType::HostLicenseExpiredEvent => Ok(from.as_any_box().downcast::<HostLicenseExpiredEvent>()?),
            StructType::IncorrectHostInformationEvent => Ok(from.as_any_box().downcast::<IncorrectHostInformationEvent>()?),
            StructType::InvalidEditionEvent => Ok(from.as_any_box().downcast::<InvalidEditionEvent>()?),
            StructType::LicenseNonComplianceEvent => Ok(from.as_any_box().downcast::<LicenseNonComplianceEvent>()?),
            StructType::LicenseRestrictedEvent => Ok(from.as_any_box().downcast::<LicenseRestrictedEvent>()?),
            StructType::LicenseServerAvailableEvent => Ok(from.as_any_box().downcast::<LicenseServerAvailableEvent>()?),
            StructType::LicenseServerUnavailableEvent => Ok(from.as_any_box().downcast::<LicenseServerUnavailableEvent>()?),
            StructType::NoLicenseEvent => Ok(from.as_any_box().downcast::<NoLicenseEvent>()?),
            StructType::ServerLicenseExpiredEvent => Ok(from.as_any_box().downcast::<ServerLicenseExpiredEvent>()?),
            StructType::UnlicensedVirtualMachinesEvent => Ok(from.as_any_box().downcast::<UnlicensedVirtualMachinesEvent>()?),
            StructType::UnlicensedVirtualMachinesFoundEvent => Ok(from.as_any_box().downcast::<UnlicensedVirtualMachinesFoundEvent>()?),
            StructType::VMotionLicenseExpiredEvent => Ok(from.as_any_box().downcast::<VMotionLicenseExpiredEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
