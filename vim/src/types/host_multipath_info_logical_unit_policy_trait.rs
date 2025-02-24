use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *HostMultipathInfoLogicalUnitPolicy* data object
/// describes a path selection policy for a device.
/// 
/// This policy determines
/// how paths should be utilized when accessing a device.
pub trait HostMultipathInfoLogicalUnitPolicyTrait : super::data_object_trait::DataObjectTrait {
    /// String representing the path selection policy for a device.
    /// 
    /// Use one of the following
    /// strings:
    /// For NMP plugin
    /// - <code>VMW\_PSP\_FIXED</code> - Use a preferred path whenever possible.
    /// - <code>VMW\_PSP\_RR</code> - Load balance.
    /// - <code>VMW\_PSP\_MRU</code> - Use the most recently used path.
    ///   
    /// For HPP plugin
    /// - <code>FIXED</code> - Use a preferred path whenever possible.
    /// - <code>LB-RR</code> - Load Balance - round robin.
    /// - <code>LB-IOPS</code> - Load Balance - iops.
    /// - <code>LB-BYTES</code> - Load Balance - bytes.
    /// - <code>LB--Latency</code> - Load balance - least latency.
    ///   
    /// You can also use the
    /// *HostStorageSystem.QueryPathSelectionPolicyOptions* method
    /// to retrieve the set of valid strings.
    /// Use the key from the resulting structure
    /// *HostPathSelectionPolicyOption*.
    fn get_policy(&self) -> &str;
}
impl<'s> serde::Serialize for dyn HostMultipathInfoLogicalUnitPolicyTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostMultipathInfoLogicalUnitPolicyTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostMultipathInfoLogicalUnitPolicyVisitor)
            }
        }

struct HostMultipathInfoLogicalUnitPolicyVisitor;

impl<'de> de::Visitor<'de> for HostMultipathInfoLogicalUnitPolicyVisitor {
    type Value = Box<dyn HostMultipathInfoLogicalUnitPolicyTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostMultipathInfoLogicalUnitPolicyTrait JSON object with a _typeName field")
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

impl HostMultipathInfoLogicalUnitPolicyTrait for HostMultipathInfoLogicalUnitPolicy {
    fn get_policy(&self) -> &str { &self.policy }
}
impl HostMultipathInfoLogicalUnitPolicyTrait for HostMultipathInfoFixedLogicalUnitPolicy {
    fn get_policy(&self) -> &str { &self.policy }
}
impl HostMultipathInfoLogicalUnitPolicyTrait for HostMultipathInfoHppLogicalUnitPolicy {
    fn get_policy(&self) -> &str { &self.policy }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostMultipathInfoLogicalUnitPolicyTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostMultipathInfoLogicalUnitPolicy => Some(from.as_any_ref().downcast_ref::<HostMultipathInfoLogicalUnitPolicy>()?),
            StructType::HostMultipathInfoFixedLogicalUnitPolicy => Some(from.as_any_ref().downcast_ref::<HostMultipathInfoFixedLogicalUnitPolicy>()?),
            StructType::HostMultipathInfoHppLogicalUnitPolicy => Some(from.as_any_ref().downcast_ref::<HostMultipathInfoHppLogicalUnitPolicy>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostMultipathInfoLogicalUnitPolicy => Ok(from.as_any_box().downcast::<HostMultipathInfoLogicalUnitPolicy>()?),
            StructType::HostMultipathInfoFixedLogicalUnitPolicy => Ok(from.as_any_box().downcast::<HostMultipathInfoFixedLogicalUnitPolicy>()?),
            StructType::HostMultipathInfoHppLogicalUnitPolicy => Ok(from.as_any_box().downcast::<HostMultipathInfoHppLogicalUnitPolicy>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
