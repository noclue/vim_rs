use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A VMotion interface has a problem.
/// 
/// This may be an error or warning depending
/// on the specific fault subclass. This is an error or warning only when
/// migrating a powered-on virtual machine.
pub trait VMotionInterfaceIssueTrait : super::migration_fault_trait::MigrationFaultTrait {
    /// Whether this error is for the source host.
    fn get_at_source_host(&self) -> bool;
    /// The name of the host with the bad interface.
    fn get_failed_host(&self) -> &str;
    /// The host with the bad interface.
    /// 
    /// Refers instance of *HostSystem*.
    fn get_failed_host_entity(&self) -> &Option<ManagedObjectReference>;
}
impl<'s> serde::Serialize for dyn VMotionInterfaceIssueTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VMotionInterfaceIssueTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VMotionInterfaceIssueVisitor)
            }
        }

struct VMotionInterfaceIssueVisitor;

impl<'de> de::Visitor<'de> for VMotionInterfaceIssueVisitor {
    type Value = Box<dyn VMotionInterfaceIssueTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VMotionInterfaceIssueTrait JSON object with a _typeName field")
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

impl VMotionInterfaceIssueTrait for VMotionInterfaceIssue {
    fn get_at_source_host(&self) -> bool { self.at_source_host }
    fn get_failed_host(&self) -> &str { &self.failed_host }
    fn get_failed_host_entity(&self) -> &Option<ManagedObjectReference> { &self.failed_host_entity }
}
impl VMotionInterfaceIssueTrait for VMotionLinkCapacityLow {
    fn get_at_source_host(&self) -> bool { self.at_source_host }
    fn get_failed_host(&self) -> &str { &self.failed_host }
    fn get_failed_host_entity(&self) -> &Option<ManagedObjectReference> { &self.failed_host_entity }
}
impl VMotionInterfaceIssueTrait for VMotionLinkDown {
    fn get_at_source_host(&self) -> bool { self.at_source_host }
    fn get_failed_host(&self) -> &str { &self.failed_host }
    fn get_failed_host_entity(&self) -> &Option<ManagedObjectReference> { &self.failed_host_entity }
}
impl VMotionInterfaceIssueTrait for VMotionNotConfigured {
    fn get_at_source_host(&self) -> bool { self.at_source_host }
    fn get_failed_host(&self) -> &str { &self.failed_host }
    fn get_failed_host_entity(&self) -> &Option<ManagedObjectReference> { &self.failed_host_entity }
}
impl VMotionInterfaceIssueTrait for VMotionNotLicensed {
    fn get_at_source_host(&self) -> bool { self.at_source_host }
    fn get_failed_host(&self) -> &str { &self.failed_host }
    fn get_failed_host_entity(&self) -> &Option<ManagedObjectReference> { &self.failed_host_entity }
}
impl VMotionInterfaceIssueTrait for VMotionNotSupported {
    fn get_at_source_host(&self) -> bool { self.at_source_host }
    fn get_failed_host(&self) -> &str { &self.failed_host }
    fn get_failed_host_entity(&self) -> &Option<ManagedObjectReference> { &self.failed_host_entity }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VMotionInterfaceIssueTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VMotionInterfaceIssue => Some(from.as_any_ref().downcast_ref::<VMotionInterfaceIssue>()?),
            StructType::VMotionLinkCapacityLow => Some(from.as_any_ref().downcast_ref::<VMotionLinkCapacityLow>()?),
            StructType::VMotionLinkDown => Some(from.as_any_ref().downcast_ref::<VMotionLinkDown>()?),
            StructType::VMotionNotConfigured => Some(from.as_any_ref().downcast_ref::<VMotionNotConfigured>()?),
            StructType::VMotionNotLicensed => Some(from.as_any_ref().downcast_ref::<VMotionNotLicensed>()?),
            StructType::VMotionNotSupported => Some(from.as_any_ref().downcast_ref::<VMotionNotSupported>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VMotionInterfaceIssue => Ok(from.as_any_box().downcast::<VMotionInterfaceIssue>()?),
            StructType::VMotionLinkCapacityLow => Ok(from.as_any_box().downcast::<VMotionLinkCapacityLow>()?),
            StructType::VMotionLinkDown => Ok(from.as_any_box().downcast::<VMotionLinkDown>()?),
            StructType::VMotionNotConfigured => Ok(from.as_any_box().downcast::<VMotionNotConfigured>()?),
            StructType::VMotionNotLicensed => Ok(from.as_any_box().downcast::<VMotionNotLicensed>()?),
            StructType::VMotionNotSupported => Ok(from.as_any_box().downcast::<VMotionNotSupported>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
