use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *ProfilePolicyOptionMetadata* data object contains the metadata information
/// for a *PolicyOption*.
pub trait ProfilePolicyOptionMetadataTrait : super::data_object_trait::DataObjectTrait {
    /// Identifier for the policy option.
    /// - The <code>id.key</code> value
    ///   (*ExtendedElementDescription*.*ElementDescription.key*)
    ///   identifies the policy option type.
    /// - The <code>id.label</code> property
    ///   (*ExtendedElementDescription*.*Description.label*)
    ///   contains a brief localizable message describing the policy option.
    /// - The <code>id.summary</code> property
    ///   (*ExtendedElementDescription*.*Description.summary*)
    ///   contains a localizable summary of the policy option.
    ///   Summary information can contain embedded variable names which can
    ///   be replaced with values from the <code>parameter</code> property.
    fn get_id(&self) -> &ExtendedElementDescription;
    /// Metadata about the parameters for the policy option.
    fn get_parameter(&self) -> &Option<Vec<ProfileParameterMetadata>>;
}
impl<'s> serde::Serialize for dyn ProfilePolicyOptionMetadataTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ProfilePolicyOptionMetadataTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ProfilePolicyOptionMetadataVisitor)
            }
        }

struct ProfilePolicyOptionMetadataVisitor;

impl<'de> de::Visitor<'de> for ProfilePolicyOptionMetadataVisitor {
    type Value = Box<dyn ProfilePolicyOptionMetadataTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ProfilePolicyOptionMetadataTrait JSON object with a _typeName field")
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

impl ProfilePolicyOptionMetadataTrait for ProfilePolicyOptionMetadata {
    fn get_id(&self) -> &ExtendedElementDescription { &self.id }
    fn get_parameter(&self) -> &Option<Vec<ProfileParameterMetadata>> { &self.parameter }
}
impl ProfilePolicyOptionMetadataTrait for ProfileCompositePolicyOptionMetadata {
    fn get_id(&self) -> &ExtendedElementDescription { &self.id }
    fn get_parameter(&self) -> &Option<Vec<ProfileParameterMetadata>> { &self.parameter }
}
impl ProfilePolicyOptionMetadataTrait for UserInputRequiredParameterMetadata {
    fn get_id(&self) -> &ExtendedElementDescription { &self.id }
    fn get_parameter(&self) -> &Option<Vec<ProfileParameterMetadata>> { &self.parameter }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ProfilePolicyOptionMetadataTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ProfilePolicyOptionMetadata => Some(from.as_any_ref().downcast_ref::<ProfilePolicyOptionMetadata>()?),
            StructType::ProfileCompositePolicyOptionMetadata => Some(from.as_any_ref().downcast_ref::<ProfileCompositePolicyOptionMetadata>()?),
            StructType::UserInputRequiredParameterMetadata => Some(from.as_any_ref().downcast_ref::<UserInputRequiredParameterMetadata>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ProfilePolicyOptionMetadata => Ok(from.as_any_box().downcast::<ProfilePolicyOptionMetadata>()?),
            StructType::ProfileCompositePolicyOptionMetadata => Ok(from.as_any_box().downcast::<ProfileCompositePolicyOptionMetadata>()?),
            StructType::UserInputRequiredParameterMetadata => Ok(from.as_any_box().downcast::<UserInputRequiredParameterMetadata>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
