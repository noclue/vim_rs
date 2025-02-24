use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// *ClusterGroupInfo* is the base type for all virtual machine
/// and host groups.
/// 
/// All virtual machines and hosts that are part of a group
/// must be part of the same cluster.
pub trait ClusterGroupInfoTrait : super::data_object_trait::DataObjectTrait {
    /// Unique name of the group.
    fn get_name(&self) -> &str;
    /// Flag to indicate whether the group is created by the user or the system.
    fn get_user_created(&self) -> Option<bool>;
    /// Unique ID for the group.
    /// 
    /// uniqueID is unique within a cluster.
    /// Groups residing in different clusters might share a uniqueID.
    fn get_unique_id(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn ClusterGroupInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ClusterGroupInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ClusterGroupInfoVisitor)
            }
        }

struct ClusterGroupInfoVisitor;

impl<'de> de::Visitor<'de> for ClusterGroupInfoVisitor {
    type Value = Box<dyn ClusterGroupInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ClusterGroupInfoTrait JSON object with a _typeName field")
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

impl ClusterGroupInfoTrait for ClusterGroupInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_user_created(&self) -> Option<bool> { self.user_created }
    fn get_unique_id(&self) -> &Option<String> { &self.unique_id }
}
impl ClusterGroupInfoTrait for ClusterHostGroup {
    fn get_name(&self) -> &str { &self.name }
    fn get_user_created(&self) -> Option<bool> { self.user_created }
    fn get_unique_id(&self) -> &Option<String> { &self.unique_id }
}
impl ClusterGroupInfoTrait for ClusterVmGroup {
    fn get_name(&self) -> &str { &self.name }
    fn get_user_created(&self) -> Option<bool> { self.user_created }
    fn get_unique_id(&self) -> &Option<String> { &self.unique_id }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ClusterGroupInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterGroupInfo => Some(from.as_any_ref().downcast_ref::<ClusterGroupInfo>()?),
            StructType::ClusterHostGroup => Some(from.as_any_ref().downcast_ref::<ClusterHostGroup>()?),
            StructType::ClusterVmGroup => Some(from.as_any_ref().downcast_ref::<ClusterVmGroup>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterGroupInfo => Ok(from.as_any_box().downcast::<ClusterGroupInfo>()?),
            StructType::ClusterHostGroup => Ok(from.as_any_box().downcast::<ClusterHostGroup>()?),
            StructType::ClusterVmGroup => Ok(from.as_any_box().downcast::<ClusterVmGroup>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
