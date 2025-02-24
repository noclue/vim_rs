use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Captures one "log entry" of an upgrade process.
pub trait VsanUpgradeSystemUpgradeHistoryItemTrait : super::data_object_trait::DataObjectTrait {
    /// Time stamp when the history is record.
    fn get_timestamp(&self) -> &str;
    /// The host a history item pertains to.
    /// 
    /// May be unset when item related
    /// to no particular host.
    /// 
    /// Refers instance of *HostSystem*.
    fn get_host(&self) -> &Option<ManagedObjectReference>;
    /// Description of the history item.
    fn get_message(&self) -> &str;
    /// A task associated with the history item.
    /// 
    /// May be unset if no task is
    /// associated.
    /// 
    /// Refers instance of *Task*.
    fn get_task(&self) -> &Option<ManagedObjectReference>;
}
impl<'s> serde::Serialize for dyn VsanUpgradeSystemUpgradeHistoryItemTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VsanUpgradeSystemUpgradeHistoryItemTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VsanUpgradeSystemUpgradeHistoryItemVisitor)
            }
        }

struct VsanUpgradeSystemUpgradeHistoryItemVisitor;

impl<'de> de::Visitor<'de> for VsanUpgradeSystemUpgradeHistoryItemVisitor {
    type Value = Box<dyn VsanUpgradeSystemUpgradeHistoryItemTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VsanUpgradeSystemUpgradeHistoryItemTrait JSON object with a _typeName field")
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

impl VsanUpgradeSystemUpgradeHistoryItemTrait for VsanUpgradeSystemUpgradeHistoryItem {
    fn get_timestamp(&self) -> &str { &self.timestamp }
    fn get_host(&self) -> &Option<ManagedObjectReference> { &self.host }
    fn get_message(&self) -> &str { &self.message }
    fn get_task(&self) -> &Option<ManagedObjectReference> { &self.task }
}
impl VsanUpgradeSystemUpgradeHistoryItemTrait for VsanUpgradeSystemUpgradeHistoryDiskGroupOp {
    fn get_timestamp(&self) -> &str { &self.timestamp }
    fn get_host(&self) -> &Option<ManagedObjectReference> { &self.host }
    fn get_message(&self) -> &str { &self.message }
    fn get_task(&self) -> &Option<ManagedObjectReference> { &self.task }
}
impl VsanUpgradeSystemUpgradeHistoryItemTrait for VsanUpgradeSystemUpgradeHistoryPreflightFail {
    fn get_timestamp(&self) -> &str { &self.timestamp }
    fn get_host(&self) -> &Option<ManagedObjectReference> { &self.host }
    fn get_message(&self) -> &str { &self.message }
    fn get_task(&self) -> &Option<ManagedObjectReference> { &self.task }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VsanUpgradeSystemUpgradeHistoryItemTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VsanUpgradeSystemUpgradeHistoryItem => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemUpgradeHistoryItem>()?),
            StructType::VsanUpgradeSystemUpgradeHistoryDiskGroupOp => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemUpgradeHistoryDiskGroupOp>()?),
            StructType::VsanUpgradeSystemUpgradeHistoryPreflightFail => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemUpgradeHistoryPreflightFail>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VsanUpgradeSystemUpgradeHistoryItem => Ok(from.as_any_box().downcast::<VsanUpgradeSystemUpgradeHistoryItem>()?),
            StructType::VsanUpgradeSystemUpgradeHistoryDiskGroupOp => Ok(from.as_any_box().downcast::<VsanUpgradeSystemUpgradeHistoryDiskGroupOp>()?),
            StructType::VsanUpgradeSystemUpgradeHistoryPreflightFail => Ok(from.as_any_box().downcast::<VsanUpgradeSystemUpgradeHistoryPreflightFail>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
