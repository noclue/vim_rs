use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Describes the validation results.
pub trait ClusterComputeResourceValidationResultBaseTrait : super::data_object_trait::DataObjectTrait {
    /// Describes the messages relevant to the validation result
    fn get_info(&self) -> &Option<Vec<LocalizableMessage>>;
}
impl<'s> serde::Serialize for dyn ClusterComputeResourceValidationResultBaseTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ClusterComputeResourceValidationResultBaseTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ClusterComputeResourceValidationResultBaseVisitor)
            }
        }

struct ClusterComputeResourceValidationResultBaseVisitor;

impl<'de> de::Visitor<'de> for ClusterComputeResourceValidationResultBaseVisitor {
    type Value = Box<dyn ClusterComputeResourceValidationResultBaseTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ClusterComputeResourceValidationResultBaseTrait JSON object with a _typeName field")
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

impl ClusterComputeResourceValidationResultBaseTrait for ClusterComputeResourceValidationResultBase {
    fn get_info(&self) -> &Option<Vec<LocalizableMessage>> { &self.info }
}
impl ClusterComputeResourceValidationResultBaseTrait for ClusterComputeResourceDvsConfigurationValidation {
    fn get_info(&self) -> &Option<Vec<LocalizableMessage>> { &self.info }
}
impl ClusterComputeResourceValidationResultBaseTrait for ClusterComputeResourceHostConfigurationValidation {
    fn get_info(&self) -> &Option<Vec<LocalizableMessage>> { &self.info }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ClusterComputeResourceValidationResultBaseTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterComputeResourceValidationResultBase => Some(from.as_any_ref().downcast_ref::<ClusterComputeResourceValidationResultBase>()?),
            StructType::ClusterComputeResourceDvsConfigurationValidation => Some(from.as_any_ref().downcast_ref::<ClusterComputeResourceDvsConfigurationValidation>()?),
            StructType::ClusterComputeResourceHostConfigurationValidation => Some(from.as_any_ref().downcast_ref::<ClusterComputeResourceHostConfigurationValidation>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterComputeResourceValidationResultBase => Ok(from.as_any_box().downcast::<ClusterComputeResourceValidationResultBase>()?),
            StructType::ClusterComputeResourceDvsConfigurationValidation => Ok(from.as_any_box().downcast::<ClusterComputeResourceDvsConfigurationValidation>()?),
            StructType::ClusterComputeResourceHostConfigurationValidation => Ok(from.as_any_box().downcast::<ClusterComputeResourceHostConfigurationValidation>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
