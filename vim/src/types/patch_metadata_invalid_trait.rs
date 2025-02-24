use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This fault is thrown if a patch query or
/// installation operation fails because of a problem with the metadata
/// associated with the patch.
/// 
/// Typically, a subclass of this exception is
/// thrown, indicating a problem such as the metadata is not found or the
/// metadata is corrupted.
pub trait PatchMetadataInvalidTrait : super::vim_fault_trait::VimFaultTrait {
    /// The patch ID whose associated metadata is invalid.
    fn get_patch_id(&self) -> &str;
    /// The metadata file that is not available or corrupted.
    fn get_meta_data(&self) -> &Option<Vec<String>>;
}
impl<'s> serde::Serialize for dyn PatchMetadataInvalidTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn PatchMetadataInvalidTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(PatchMetadataInvalidVisitor)
            }
        }

struct PatchMetadataInvalidVisitor;

impl<'de> de::Visitor<'de> for PatchMetadataInvalidVisitor {
    type Value = Box<dyn PatchMetadataInvalidTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid PatchMetadataInvalidTrait JSON object with a _typeName field")
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

impl PatchMetadataInvalidTrait for PatchMetadataInvalid {
    fn get_patch_id(&self) -> &str { &self.patch_id }
    fn get_meta_data(&self) -> &Option<Vec<String>> { &self.meta_data }
}
impl PatchMetadataInvalidTrait for PatchMetadataCorrupted {
    fn get_patch_id(&self) -> &str { &self.patch_id }
    fn get_meta_data(&self) -> &Option<Vec<String>> { &self.meta_data }
}
impl PatchMetadataInvalidTrait for PatchMetadataNotFound {
    fn get_patch_id(&self) -> &str { &self.patch_id }
    fn get_meta_data(&self) -> &Option<Vec<String>> { &self.meta_data }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn PatchMetadataInvalidTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::PatchMetadataInvalid => Some(from.as_any_ref().downcast_ref::<PatchMetadataInvalid>()?),
            StructType::PatchMetadataCorrupted => Some(from.as_any_ref().downcast_ref::<PatchMetadataCorrupted>()?),
            StructType::PatchMetadataNotFound => Some(from.as_any_ref().downcast_ref::<PatchMetadataNotFound>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::PatchMetadataInvalid => Ok(from.as_any_box().downcast::<PatchMetadataInvalid>()?),
            StructType::PatchMetadataCorrupted => Ok(from.as_any_box().downcast::<PatchMetadataCorrupted>()?),
            StructType::PatchMetadataNotFound => Ok(from.as_any_box().downcast::<PatchMetadataNotFound>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
