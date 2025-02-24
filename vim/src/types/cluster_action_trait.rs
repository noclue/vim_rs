use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base class for all action recommendations in VirtualCenter.
pub trait ClusterActionTrait : super::data_object_trait::DataObjectTrait {
    /// Type of the action.
    /// 
    /// This is encoded to differentiate between
    /// different types of actions aimed at achieving different goals.
    fn get_type(&self) -> &str;
    /// The target object on which this action will be applied.
    /// 
    /// For
    /// instance, a migration action will have a virtual machine as its
    /// target object, while a host power action will have a host as its
    /// target action.
    fn get_target(&self) -> &Option<ManagedObjectReference>;
}
impl<'s> serde::Serialize for dyn ClusterActionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ClusterActionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ClusterActionVisitor)
            }
        }

struct ClusterActionVisitor;

impl<'de> de::Visitor<'de> for ClusterActionVisitor {
    type Value = Box<dyn ClusterActionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ClusterActionTrait JSON object with a _typeName field")
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

impl ClusterActionTrait for ClusterAction {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_target(&self) -> &Option<ManagedObjectReference> { &self.target }
}
impl ClusterActionTrait for ClusterClusterInitialPlacementAction {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_target(&self) -> &Option<ManagedObjectReference> { &self.target }
}
impl ClusterActionTrait for ClusterHostInfraUpdateHaModeAction {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_target(&self) -> &Option<ManagedObjectReference> { &self.target }
}
impl ClusterActionTrait for ClusterHostPowerAction {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_target(&self) -> &Option<ManagedObjectReference> { &self.target }
}
impl ClusterActionTrait for ClusterInitialPlacementAction {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_target(&self) -> &Option<ManagedObjectReference> { &self.target }
}
impl ClusterActionTrait for ClusterMigrationAction {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_target(&self) -> &Option<ManagedObjectReference> { &self.target }
}
impl ClusterActionTrait for PlacementAction {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_target(&self) -> &Option<ManagedObjectReference> { &self.target }
}
impl ClusterActionTrait for HbrDiskMigrationAction {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_target(&self) -> &Option<ManagedObjectReference> { &self.target }
}
impl ClusterActionTrait for StorageMigrationAction {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_target(&self) -> &Option<ManagedObjectReference> { &self.target }
}
impl ClusterActionTrait for StoragePlacementAction {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_target(&self) -> &Option<ManagedObjectReference> { &self.target }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ClusterActionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterAction => Some(from.as_any_ref().downcast_ref::<ClusterAction>()?),
            StructType::ClusterClusterInitialPlacementAction => Some(from.as_any_ref().downcast_ref::<ClusterClusterInitialPlacementAction>()?),
            StructType::ClusterHostInfraUpdateHaModeAction => Some(from.as_any_ref().downcast_ref::<ClusterHostInfraUpdateHaModeAction>()?),
            StructType::ClusterHostPowerAction => Some(from.as_any_ref().downcast_ref::<ClusterHostPowerAction>()?),
            StructType::ClusterInitialPlacementAction => Some(from.as_any_ref().downcast_ref::<ClusterInitialPlacementAction>()?),
            StructType::ClusterMigrationAction => Some(from.as_any_ref().downcast_ref::<ClusterMigrationAction>()?),
            StructType::PlacementAction => Some(from.as_any_ref().downcast_ref::<PlacementAction>()?),
            StructType::HbrDiskMigrationAction => Some(from.as_any_ref().downcast_ref::<HbrDiskMigrationAction>()?),
            StructType::StorageMigrationAction => Some(from.as_any_ref().downcast_ref::<StorageMigrationAction>()?),
            StructType::StoragePlacementAction => Some(from.as_any_ref().downcast_ref::<StoragePlacementAction>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterAction => Ok(from.as_any_box().downcast::<ClusterAction>()?),
            StructType::ClusterClusterInitialPlacementAction => Ok(from.as_any_box().downcast::<ClusterClusterInitialPlacementAction>()?),
            StructType::ClusterHostInfraUpdateHaModeAction => Ok(from.as_any_box().downcast::<ClusterHostInfraUpdateHaModeAction>()?),
            StructType::ClusterHostPowerAction => Ok(from.as_any_box().downcast::<ClusterHostPowerAction>()?),
            StructType::ClusterInitialPlacementAction => Ok(from.as_any_box().downcast::<ClusterInitialPlacementAction>()?),
            StructType::ClusterMigrationAction => Ok(from.as_any_box().downcast::<ClusterMigrationAction>()?),
            StructType::PlacementAction => Ok(from.as_any_box().downcast::<PlacementAction>()?),
            StructType::HbrDiskMigrationAction => Ok(from.as_any_box().downcast::<HbrDiskMigrationAction>()?),
            StructType::StorageMigrationAction => Ok(from.as_any_box().downcast::<StorageMigrationAction>()?),
            StructType::StoragePlacementAction => Ok(from.as_any_box().downcast::<StoragePlacementAction>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
