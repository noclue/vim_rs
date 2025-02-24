use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base class for a pre-flight check issue.
/// 
/// Can be used directly
/// but usually a derived class with a specific issue type is used.
pub trait VsanUpgradeSystemPreflightCheckIssueTrait : super::data_object_trait::DataObjectTrait {
    /// Message describing the issue.
    fn get_msg(&self) -> &str;
}
impl<'s> serde::Serialize for dyn VsanUpgradeSystemPreflightCheckIssueTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VsanUpgradeSystemPreflightCheckIssueTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VsanUpgradeSystemPreflightCheckIssueVisitor)
            }
        }

struct VsanUpgradeSystemPreflightCheckIssueVisitor;

impl<'de> de::Visitor<'de> for VsanUpgradeSystemPreflightCheckIssueVisitor {
    type Value = Box<dyn VsanUpgradeSystemPreflightCheckIssueTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VsanUpgradeSystemPreflightCheckIssueTrait JSON object with a _typeName field")
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

impl VsanUpgradeSystemPreflightCheckIssueTrait for VsanUpgradeSystemPreflightCheckIssue {
    fn get_msg(&self) -> &str { &self.msg }
}
impl VsanUpgradeSystemPreflightCheckIssueTrait for VsanUpgradeSystemApiBrokenIssue {
    fn get_msg(&self) -> &str { &self.msg }
}
impl VsanUpgradeSystemPreflightCheckIssueTrait for VsanUpgradeSystemAutoClaimEnabledOnHostsIssue {
    fn get_msg(&self) -> &str { &self.msg }
}
impl VsanUpgradeSystemPreflightCheckIssueTrait for VsanUpgradeSystemHostsDisconnectedIssue {
    fn get_msg(&self) -> &str { &self.msg }
}
impl VsanUpgradeSystemPreflightCheckIssueTrait for VsanUpgradeSystemMissingHostsInClusterIssue {
    fn get_msg(&self) -> &str { &self.msg }
}
impl VsanUpgradeSystemPreflightCheckIssueTrait for VsanUpgradeSystemNetworkPartitionIssue {
    fn get_msg(&self) -> &str { &self.msg }
}
impl VsanUpgradeSystemPreflightCheckIssueTrait for VsanUpgradeSystemNotEnoughFreeCapacityIssue {
    fn get_msg(&self) -> &str { &self.msg }
}
impl VsanUpgradeSystemPreflightCheckIssueTrait for VsanUpgradeSystemRogueHostsInClusterIssue {
    fn get_msg(&self) -> &str { &self.msg }
}
impl VsanUpgradeSystemPreflightCheckIssueTrait for VsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue {
    fn get_msg(&self) -> &str { &self.msg }
}
impl VsanUpgradeSystemPreflightCheckIssueTrait for VsanUpgradeSystemWrongEsxVersionIssue {
    fn get_msg(&self) -> &str { &self.msg }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VsanUpgradeSystemPreflightCheckIssueTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VsanUpgradeSystemPreflightCheckIssue => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemPreflightCheckIssue>()?),
            StructType::VsanUpgradeSystemApiBrokenIssue => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemApiBrokenIssue>()?),
            StructType::VsanUpgradeSystemAutoClaimEnabledOnHostsIssue => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemAutoClaimEnabledOnHostsIssue>()?),
            StructType::VsanUpgradeSystemHostsDisconnectedIssue => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemHostsDisconnectedIssue>()?),
            StructType::VsanUpgradeSystemMissingHostsInClusterIssue => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemMissingHostsInClusterIssue>()?),
            StructType::VsanUpgradeSystemNetworkPartitionIssue => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemNetworkPartitionIssue>()?),
            StructType::VsanUpgradeSystemNotEnoughFreeCapacityIssue => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemNotEnoughFreeCapacityIssue>()?),
            StructType::VsanUpgradeSystemRogueHostsInClusterIssue => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemRogueHostsInClusterIssue>()?),
            StructType::VsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue>()?),
            StructType::VsanUpgradeSystemWrongEsxVersionIssue => Some(from.as_any_ref().downcast_ref::<VsanUpgradeSystemWrongEsxVersionIssue>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VsanUpgradeSystemPreflightCheckIssue => Ok(from.as_any_box().downcast::<VsanUpgradeSystemPreflightCheckIssue>()?),
            StructType::VsanUpgradeSystemApiBrokenIssue => Ok(from.as_any_box().downcast::<VsanUpgradeSystemApiBrokenIssue>()?),
            StructType::VsanUpgradeSystemAutoClaimEnabledOnHostsIssue => Ok(from.as_any_box().downcast::<VsanUpgradeSystemAutoClaimEnabledOnHostsIssue>()?),
            StructType::VsanUpgradeSystemHostsDisconnectedIssue => Ok(from.as_any_box().downcast::<VsanUpgradeSystemHostsDisconnectedIssue>()?),
            StructType::VsanUpgradeSystemMissingHostsInClusterIssue => Ok(from.as_any_box().downcast::<VsanUpgradeSystemMissingHostsInClusterIssue>()?),
            StructType::VsanUpgradeSystemNetworkPartitionIssue => Ok(from.as_any_box().downcast::<VsanUpgradeSystemNetworkPartitionIssue>()?),
            StructType::VsanUpgradeSystemNotEnoughFreeCapacityIssue => Ok(from.as_any_box().downcast::<VsanUpgradeSystemNotEnoughFreeCapacityIssue>()?),
            StructType::VsanUpgradeSystemRogueHostsInClusterIssue => Ok(from.as_any_box().downcast::<VsanUpgradeSystemRogueHostsInClusterIssue>()?),
            StructType::VsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue => Ok(from.as_any_box().downcast::<VsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue>()?),
            StructType::VsanUpgradeSystemWrongEsxVersionIssue => Ok(from.as_any_box().downcast::<VsanUpgradeSystemWrongEsxVersionIssue>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
