use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A migration operation that requires feature support on source and
/// destination hosts is lacking support on the host.
pub trait MigrationFeatureNotSupportedTrait : super::migration_fault_trait::MigrationFaultTrait {
    /// Whether this error is for the source host.
    fn get_at_source_host(&self) -> bool;
    /// The name of the host.
    fn get_failed_host_name(&self) -> &str;
    /// The host.
    /// 
    /// Refers instance of *HostSystem*.
    fn get_failed_host(&self) -> &ManagedObjectReference;
}
impl<'s> serde::Serialize for dyn MigrationFeatureNotSupportedTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn MigrationFeatureNotSupportedTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(MigrationFeatureNotSupportedVisitor)
            }
        }

struct MigrationFeatureNotSupportedVisitor;

impl<'de> de::Visitor<'de> for MigrationFeatureNotSupportedVisitor {
    type Value = Box<dyn MigrationFeatureNotSupportedTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid MigrationFeatureNotSupportedTrait JSON object with a _typeName field")
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

impl MigrationFeatureNotSupportedTrait for MigrationFeatureNotSupported {
    fn get_at_source_host(&self) -> bool { self.at_source_host }
    fn get_failed_host_name(&self) -> &str { &self.failed_host_name }
    fn get_failed_host(&self) -> &ManagedObjectReference { &self.failed_host }
}
impl MigrationFeatureNotSupportedTrait for FullStorageVMotionNotSupported {
    fn get_at_source_host(&self) -> bool { self.at_source_host }
    fn get_failed_host_name(&self) -> &str { &self.failed_host_name }
    fn get_failed_host(&self) -> &ManagedObjectReference { &self.failed_host }
}
impl MigrationFeatureNotSupportedTrait for IndependentDiskVMotionNotSupported {
    fn get_at_source_host(&self) -> bool { self.at_source_host }
    fn get_failed_host_name(&self) -> &str { &self.failed_host_name }
    fn get_failed_host(&self) -> &ManagedObjectReference { &self.failed_host }
}
impl MigrationFeatureNotSupportedTrait for NonHomeRdmvMotionNotSupported {
    fn get_at_source_host(&self) -> bool { self.at_source_host }
    fn get_failed_host_name(&self) -> &str { &self.failed_host_name }
    fn get_failed_host(&self) -> &ManagedObjectReference { &self.failed_host }
}
impl MigrationFeatureNotSupportedTrait for StorageVMotionNotSupported {
    fn get_at_source_host(&self) -> bool { self.at_source_host }
    fn get_failed_host_name(&self) -> &str { &self.failed_host_name }
    fn get_failed_host(&self) -> &ManagedObjectReference { &self.failed_host }
}
impl MigrationFeatureNotSupportedTrait for UnsharedSwapVMotionNotSupported {
    fn get_at_source_host(&self) -> bool { self.at_source_host }
    fn get_failed_host_name(&self) -> &str { &self.failed_host_name }
    fn get_failed_host(&self) -> &ManagedObjectReference { &self.failed_host }
}
impl MigrationFeatureNotSupportedTrait for VMotionAcrossNetworkNotSupported {
    fn get_at_source_host(&self) -> bool { self.at_source_host }
    fn get_failed_host_name(&self) -> &str { &self.failed_host_name }
    fn get_failed_host(&self) -> &ManagedObjectReference { &self.failed_host }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn MigrationFeatureNotSupportedTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::MigrationFeatureNotSupported => Some(from.as_any_ref().downcast_ref::<MigrationFeatureNotSupported>()?),
            StructType::FullStorageVMotionNotSupported => Some(from.as_any_ref().downcast_ref::<FullStorageVMotionNotSupported>()?),
            StructType::IndependentDiskVMotionNotSupported => Some(from.as_any_ref().downcast_ref::<IndependentDiskVMotionNotSupported>()?),
            StructType::NonHomeRdmvMotionNotSupported => Some(from.as_any_ref().downcast_ref::<NonHomeRdmvMotionNotSupported>()?),
            StructType::StorageVMotionNotSupported => Some(from.as_any_ref().downcast_ref::<StorageVMotionNotSupported>()?),
            StructType::UnsharedSwapVMotionNotSupported => Some(from.as_any_ref().downcast_ref::<UnsharedSwapVMotionNotSupported>()?),
            StructType::VMotionAcrossNetworkNotSupported => Some(from.as_any_ref().downcast_ref::<VMotionAcrossNetworkNotSupported>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::MigrationFeatureNotSupported => Ok(from.as_any_box().downcast::<MigrationFeatureNotSupported>()?),
            StructType::FullStorageVMotionNotSupported => Ok(from.as_any_box().downcast::<FullStorageVMotionNotSupported>()?),
            StructType::IndependentDiskVMotionNotSupported => Ok(from.as_any_box().downcast::<IndependentDiskVMotionNotSupported>()?),
            StructType::NonHomeRdmvMotionNotSupported => Ok(from.as_any_box().downcast::<NonHomeRdmvMotionNotSupported>()?),
            StructType::StorageVMotionNotSupported => Ok(from.as_any_box().downcast::<StorageVMotionNotSupported>()?),
            StructType::UnsharedSwapVMotionNotSupported => Ok(from.as_any_box().downcast::<UnsharedSwapVMotionNotSupported>()?),
            StructType::VMotionAcrossNetworkNotSupported => Ok(from.as_any_box().downcast::<VMotionAcrossNetworkNotSupported>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
