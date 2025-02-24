use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Health check status of an switch is changed.
pub trait DvsHealthStatusChangeEventTrait : super::host_event_trait::HostEventTrait {
    /// UUID of the DVS the host is connected to.
    fn get_switch_uuid(&self) -> &str;
    /// Health check status.
    fn get_health_result(&self) -> &Option<Box<dyn super::host_member_health_check_result_trait::HostMemberHealthCheckResultTrait>>;
}
impl<'s> serde::Serialize for dyn DvsHealthStatusChangeEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DvsHealthStatusChangeEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DvsHealthStatusChangeEventVisitor)
            }
        }

struct DvsHealthStatusChangeEventVisitor;

impl<'de> de::Visitor<'de> for DvsHealthStatusChangeEventVisitor {
    type Value = Box<dyn DvsHealthStatusChangeEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DvsHealthStatusChangeEventTrait JSON object with a _typeName field")
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

impl DvsHealthStatusChangeEventTrait for DvsHealthStatusChangeEvent {
    fn get_switch_uuid(&self) -> &str { &self.switch_uuid }
    fn get_health_result(&self) -> &Option<Box<dyn super::host_member_health_check_result_trait::HostMemberHealthCheckResultTrait>> { &self.health_result }
}
impl DvsHealthStatusChangeEventTrait for MtuMatchEvent {
    fn get_switch_uuid(&self) -> &str { &self.switch_uuid }
    fn get_health_result(&self) -> &Option<Box<dyn super::host_member_health_check_result_trait::HostMemberHealthCheckResultTrait>> { &self.health_result }
}
impl DvsHealthStatusChangeEventTrait for MtuMismatchEvent {
    fn get_switch_uuid(&self) -> &str { &self.switch_uuid }
    fn get_health_result(&self) -> &Option<Box<dyn super::host_member_health_check_result_trait::HostMemberHealthCheckResultTrait>> { &self.health_result }
}
impl DvsHealthStatusChangeEventTrait for TeamingMatchEvent {
    fn get_switch_uuid(&self) -> &str { &self.switch_uuid }
    fn get_health_result(&self) -> &Option<Box<dyn super::host_member_health_check_result_trait::HostMemberHealthCheckResultTrait>> { &self.health_result }
}
impl DvsHealthStatusChangeEventTrait for TeamingMisMatchEvent {
    fn get_switch_uuid(&self) -> &str { &self.switch_uuid }
    fn get_health_result(&self) -> &Option<Box<dyn super::host_member_health_check_result_trait::HostMemberHealthCheckResultTrait>> { &self.health_result }
}
impl DvsHealthStatusChangeEventTrait for UplinkPortMtuNotSupportEvent {
    fn get_switch_uuid(&self) -> &str { &self.switch_uuid }
    fn get_health_result(&self) -> &Option<Box<dyn super::host_member_health_check_result_trait::HostMemberHealthCheckResultTrait>> { &self.health_result }
}
impl DvsHealthStatusChangeEventTrait for UplinkPortMtuSupportEvent {
    fn get_switch_uuid(&self) -> &str { &self.switch_uuid }
    fn get_health_result(&self) -> &Option<Box<dyn super::host_member_health_check_result_trait::HostMemberHealthCheckResultTrait>> { &self.health_result }
}
impl DvsHealthStatusChangeEventTrait for UplinkPortVlanTrunkedEvent {
    fn get_switch_uuid(&self) -> &str { &self.switch_uuid }
    fn get_health_result(&self) -> &Option<Box<dyn super::host_member_health_check_result_trait::HostMemberHealthCheckResultTrait>> { &self.health_result }
}
impl DvsHealthStatusChangeEventTrait for UplinkPortVlanUntrunkedEvent {
    fn get_switch_uuid(&self) -> &str { &self.switch_uuid }
    fn get_health_result(&self) -> &Option<Box<dyn super::host_member_health_check_result_trait::HostMemberHealthCheckResultTrait>> { &self.health_result }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DvsHealthStatusChangeEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsHealthStatusChangeEvent => Some(from.as_any_ref().downcast_ref::<DvsHealthStatusChangeEvent>()?),
            StructType::MtuMatchEvent => Some(from.as_any_ref().downcast_ref::<MtuMatchEvent>()?),
            StructType::MtuMismatchEvent => Some(from.as_any_ref().downcast_ref::<MtuMismatchEvent>()?),
            StructType::TeamingMatchEvent => Some(from.as_any_ref().downcast_ref::<TeamingMatchEvent>()?),
            StructType::TeamingMisMatchEvent => Some(from.as_any_ref().downcast_ref::<TeamingMisMatchEvent>()?),
            StructType::UplinkPortMtuNotSupportEvent => Some(from.as_any_ref().downcast_ref::<UplinkPortMtuNotSupportEvent>()?),
            StructType::UplinkPortMtuSupportEvent => Some(from.as_any_ref().downcast_ref::<UplinkPortMtuSupportEvent>()?),
            StructType::UplinkPortVlanTrunkedEvent => Some(from.as_any_ref().downcast_ref::<UplinkPortVlanTrunkedEvent>()?),
            StructType::UplinkPortVlanUntrunkedEvent => Some(from.as_any_ref().downcast_ref::<UplinkPortVlanUntrunkedEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsHealthStatusChangeEvent => Ok(from.as_any_box().downcast::<DvsHealthStatusChangeEvent>()?),
            StructType::MtuMatchEvent => Ok(from.as_any_box().downcast::<MtuMatchEvent>()?),
            StructType::MtuMismatchEvent => Ok(from.as_any_box().downcast::<MtuMismatchEvent>()?),
            StructType::TeamingMatchEvent => Ok(from.as_any_box().downcast::<TeamingMatchEvent>()?),
            StructType::TeamingMisMatchEvent => Ok(from.as_any_box().downcast::<TeamingMisMatchEvent>()?),
            StructType::UplinkPortMtuNotSupportEvent => Ok(from.as_any_box().downcast::<UplinkPortMtuNotSupportEvent>()?),
            StructType::UplinkPortMtuSupportEvent => Ok(from.as_any_box().downcast::<UplinkPortMtuSupportEvent>()?),
            StructType::UplinkPortVlanTrunkedEvent => Ok(from.as_any_box().downcast::<UplinkPortVlanTrunkedEvent>()?),
            StructType::UplinkPortVlanUntrunkedEvent => Ok(from.as_any_box().downcast::<UplinkPortVlanUntrunkedEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
