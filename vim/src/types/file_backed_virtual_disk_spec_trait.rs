use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Specification used to create a file based virtual disk
pub trait FileBackedVirtualDiskSpecTrait : super::virtual_disk_spec_trait::VirtualDiskSpecTrait {
    /// Specify the capacity of the virtual disk in Kb.
    fn get_capacity_kb(&self) -> i64;
    /// Virtual Disk Profile requirement.
    /// 
    /// Profiles are solution specifics.
    /// Profile Based Storage Management is a vSphere server extension.
    /// The API users who want to provision VMs using Storage Profiles, need to
    /// interact with it.
    /// This is an optional parameter and if user doesn't specify profile,
    /// the default behavior will apply.
    fn get_profile(&self) -> &Option<Vec<Box<dyn super::virtual_machine_profile_spec_trait::VirtualMachineProfileSpecTrait>>>;
    /// Encryption options for the new virtual disk.
    fn get_crypto(&self) -> &Option<Box<dyn super::crypto_spec_trait::CryptoSpecTrait>>;
}
impl<'s> serde::Serialize for dyn FileBackedVirtualDiskSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn FileBackedVirtualDiskSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(FileBackedVirtualDiskSpecVisitor)
            }
        }

struct FileBackedVirtualDiskSpecVisitor;

impl<'de> de::Visitor<'de> for FileBackedVirtualDiskSpecVisitor {
    type Value = Box<dyn FileBackedVirtualDiskSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid FileBackedVirtualDiskSpecTrait JSON object with a _typeName field")
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

impl FileBackedVirtualDiskSpecTrait for FileBackedVirtualDiskSpec {
    fn get_capacity_kb(&self) -> i64 { self.capacity_kb }
    fn get_profile(&self) -> &Option<Vec<Box<dyn super::virtual_machine_profile_spec_trait::VirtualMachineProfileSpecTrait>>> { &self.profile }
    fn get_crypto(&self) -> &Option<Box<dyn super::crypto_spec_trait::CryptoSpecTrait>> { &self.crypto }
}
impl FileBackedVirtualDiskSpecTrait for SeSparseVirtualDiskSpec {
    fn get_capacity_kb(&self) -> i64 { self.capacity_kb }
    fn get_profile(&self) -> &Option<Vec<Box<dyn super::virtual_machine_profile_spec_trait::VirtualMachineProfileSpecTrait>>> { &self.profile }
    fn get_crypto(&self) -> &Option<Box<dyn super::crypto_spec_trait::CryptoSpecTrait>> { &self.crypto }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn FileBackedVirtualDiskSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::FileBackedVirtualDiskSpec => Some(from.as_any_ref().downcast_ref::<FileBackedVirtualDiskSpec>()?),
            StructType::SeSparseVirtualDiskSpec => Some(from.as_any_ref().downcast_ref::<SeSparseVirtualDiskSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::FileBackedVirtualDiskSpec => Ok(from.as_any_box().downcast::<FileBackedVirtualDiskSpec>()?),
            StructType::SeSparseVirtualDiskSpec => Ok(from.as_any_box().downcast::<SeSparseVirtualDiskSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
