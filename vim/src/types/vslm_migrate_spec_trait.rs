use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base specification of moving or copying a virtual storage object.
pub trait VslmMigrateSpecTrait : super::data_object_trait::DataObjectTrait {
    /// Specification of the backings of the target virtual storage object.
    fn get_backing_spec(&self) -> &Box<dyn super::vslm_create_spec_backing_spec_trait::VslmCreateSpecBackingSpecTrait>;
    /// Virtual storage object Profile requirement.
    /// 
    /// If unset,
    /// the default behavior will apply.
    fn get_profile(&self) -> &Option<Vec<Box<dyn super::virtual_machine_profile_spec_trait::VirtualMachineProfileSpecTrait>>>;
    /// Flag indicates any delta disk backings will be consolidated
    /// during migration.
    /// 
    /// If unset, delta disk backings will not be
    /// consolidated.
    fn get_consolidate(&self) -> Option<bool>;
    /// Disk chain crypto information.
    /// 
    /// If unset and if *VslmMigrateSpec.profile* contains an encryption iofilter and if
    /// source VStorageObject is unencrypted, then disksCyrpto will be of type
    /// CryptoSpecEncrypt, and filled with keyId that is automatically generated
    /// and keyProviderId that is the default kms cluster. During the migration,
    /// the object will be encrypted.
    /// If unset and if *VslmMigrateSpec.profile* is a default policy and if source
    /// VStorageObject is unenrypted, then disksCrypto is treated as
    /// CryptoSpecNoOp. During migration, no cryptographic change.
    /// If unset and if *VslmMigrateSpec.profile* contains an encryption iofilter and if
    /// source VStorageObject is encrypted, then disksCyrpto is treated as
    /// CryptoSpecNoOp. During migration, no cryptographic change.
    /// If unset and if *VslmMigrateSpec.profile* is a default policy and if
    /// source VStorageObject is encrypted, then disksCyrpto is treated as
    /// CryptoSpecDecrypt, during migration, the object will be decrypted.
    /// To recrypt the disk during migration, disksCrypto has to be present.
    fn get_disks_crypto(&self) -> &Option<DiskCryptoSpec>;
}
impl<'s> serde::Serialize for dyn VslmMigrateSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VslmMigrateSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VslmMigrateSpecVisitor)
            }
        }

struct VslmMigrateSpecVisitor;

impl<'de> de::Visitor<'de> for VslmMigrateSpecVisitor {
    type Value = Box<dyn VslmMigrateSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VslmMigrateSpecTrait JSON object with a _typeName field")
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

impl VslmMigrateSpecTrait for VslmMigrateSpec {
    fn get_backing_spec(&self) -> &Box<dyn super::vslm_create_spec_backing_spec_trait::VslmCreateSpecBackingSpecTrait> { &self.backing_spec }
    fn get_profile(&self) -> &Option<Vec<Box<dyn super::virtual_machine_profile_spec_trait::VirtualMachineProfileSpecTrait>>> { &self.profile }
    fn get_consolidate(&self) -> Option<bool> { self.consolidate }
    fn get_disks_crypto(&self) -> &Option<DiskCryptoSpec> { &self.disks_crypto }
}
impl VslmMigrateSpecTrait for VslmCloneSpec {
    fn get_backing_spec(&self) -> &Box<dyn super::vslm_create_spec_backing_spec_trait::VslmCreateSpecBackingSpecTrait> { &self.backing_spec }
    fn get_profile(&self) -> &Option<Vec<Box<dyn super::virtual_machine_profile_spec_trait::VirtualMachineProfileSpecTrait>>> { &self.profile }
    fn get_consolidate(&self) -> Option<bool> { self.consolidate }
    fn get_disks_crypto(&self) -> &Option<DiskCryptoSpec> { &self.disks_crypto }
}
impl VslmMigrateSpecTrait for VslmRelocateSpec {
    fn get_backing_spec(&self) -> &Box<dyn super::vslm_create_spec_backing_spec_trait::VslmCreateSpecBackingSpecTrait> { &self.backing_spec }
    fn get_profile(&self) -> &Option<Vec<Box<dyn super::virtual_machine_profile_spec_trait::VirtualMachineProfileSpecTrait>>> { &self.profile }
    fn get_consolidate(&self) -> Option<bool> { self.consolidate }
    fn get_disks_crypto(&self) -> &Option<DiskCryptoSpec> { &self.disks_crypto }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VslmMigrateSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VslmMigrateSpec => Some(from.as_any_ref().downcast_ref::<VslmMigrateSpec>()?),
            StructType::VslmCloneSpec => Some(from.as_any_ref().downcast_ref::<VslmCloneSpec>()?),
            StructType::VslmRelocateSpec => Some(from.as_any_ref().downcast_ref::<VslmRelocateSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VslmMigrateSpec => Ok(from.as_any_box().downcast::<VslmMigrateSpec>()?),
            StructType::VslmCloneSpec => Ok(from.as_any_box().downcast::<VslmCloneSpec>()?),
            StructType::VslmRelocateSpec => Ok(from.as_any_box().downcast::<VslmRelocateSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
