use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Specification describing the parameters during Profile creation
pub trait ProfileCreateSpecTrait : super::data_object_trait::DataObjectTrait {
    /// Name of the profile
    fn get_name(&self) -> &Option<String>;
    /// User Provided description of the profile
    fn get_annotation(&self) -> &Option<String>;
    /// Flag indicating if the Profile is enabled
    fn get_enabled(&self) -> Option<bool>;
}
impl<'s> serde::Serialize for dyn ProfileCreateSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ProfileCreateSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ProfileCreateSpecVisitor)
            }
        }

struct ProfileCreateSpecVisitor;

impl<'de> de::Visitor<'de> for ProfileCreateSpecVisitor {
    type Value = Box<dyn ProfileCreateSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ProfileCreateSpecTrait JSON object with a _typeName field")
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

impl ProfileCreateSpecTrait for ProfileCreateSpec {
    fn get_name(&self) -> &Option<String> { &self.name }
    fn get_annotation(&self) -> &Option<String> { &self.annotation }
    fn get_enabled(&self) -> Option<bool> { self.enabled }
}
impl ProfileCreateSpecTrait for ProfileSerializedCreateSpec {
    fn get_name(&self) -> &Option<String> { &self.name }
    fn get_annotation(&self) -> &Option<String> { &self.annotation }
    fn get_enabled(&self) -> Option<bool> { self.enabled }
}
impl ProfileCreateSpecTrait for HostProfileSerializedHostProfileSpec {
    fn get_name(&self) -> &Option<String> { &self.name }
    fn get_annotation(&self) -> &Option<String> { &self.annotation }
    fn get_enabled(&self) -> Option<bool> { self.enabled }
}
impl ProfileCreateSpecTrait for ClusterProfileCreateSpec {
    fn get_name(&self) -> &Option<String> { &self.name }
    fn get_annotation(&self) -> &Option<String> { &self.annotation }
    fn get_enabled(&self) -> Option<bool> { self.enabled }
}
impl ProfileCreateSpecTrait for ClusterProfileConfigSpec {
    fn get_name(&self) -> &Option<String> { &self.name }
    fn get_annotation(&self) -> &Option<String> { &self.annotation }
    fn get_enabled(&self) -> Option<bool> { self.enabled }
}
impl ProfileCreateSpecTrait for ClusterProfileCompleteConfigSpec {
    fn get_name(&self) -> &Option<String> { &self.name }
    fn get_annotation(&self) -> &Option<String> { &self.annotation }
    fn get_enabled(&self) -> Option<bool> { self.enabled }
}
impl ProfileCreateSpecTrait for ClusterProfileConfigServiceCreateSpec {
    fn get_name(&self) -> &Option<String> { &self.name }
    fn get_annotation(&self) -> &Option<String> { &self.annotation }
    fn get_enabled(&self) -> Option<bool> { self.enabled }
}
impl ProfileCreateSpecTrait for HostProfileConfigSpec {
    fn get_name(&self) -> &Option<String> { &self.name }
    fn get_annotation(&self) -> &Option<String> { &self.annotation }
    fn get_enabled(&self) -> Option<bool> { self.enabled }
}
impl ProfileCreateSpecTrait for HostProfileCompleteConfigSpec {
    fn get_name(&self) -> &Option<String> { &self.name }
    fn get_annotation(&self) -> &Option<String> { &self.annotation }
    fn get_enabled(&self) -> Option<bool> { self.enabled }
}
impl ProfileCreateSpecTrait for HostProfileHostBasedConfigSpec {
    fn get_name(&self) -> &Option<String> { &self.name }
    fn get_annotation(&self) -> &Option<String> { &self.annotation }
    fn get_enabled(&self) -> Option<bool> { self.enabled }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ProfileCreateSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ProfileCreateSpec => Some(from.as_any_ref().downcast_ref::<ProfileCreateSpec>()?),
            StructType::ProfileSerializedCreateSpec => Some(from.as_any_ref().downcast_ref::<ProfileSerializedCreateSpec>()?),
            StructType::HostProfileSerializedHostProfileSpec => Some(from.as_any_ref().downcast_ref::<HostProfileSerializedHostProfileSpec>()?),
            StructType::ClusterProfileCreateSpec => Some(from.as_any_ref().downcast_ref::<ClusterProfileCreateSpec>()?),
            StructType::ClusterProfileConfigSpec => Some(from.as_any_ref().downcast_ref::<ClusterProfileConfigSpec>()?),
            StructType::ClusterProfileCompleteConfigSpec => Some(from.as_any_ref().downcast_ref::<ClusterProfileCompleteConfigSpec>()?),
            StructType::ClusterProfileConfigServiceCreateSpec => Some(from.as_any_ref().downcast_ref::<ClusterProfileConfigServiceCreateSpec>()?),
            StructType::HostProfileConfigSpec => Some(from.as_any_ref().downcast_ref::<HostProfileConfigSpec>()?),
            StructType::HostProfileCompleteConfigSpec => Some(from.as_any_ref().downcast_ref::<HostProfileCompleteConfigSpec>()?),
            StructType::HostProfileHostBasedConfigSpec => Some(from.as_any_ref().downcast_ref::<HostProfileHostBasedConfigSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ProfileCreateSpec => Ok(from.as_any_box().downcast::<ProfileCreateSpec>()?),
            StructType::ProfileSerializedCreateSpec => Ok(from.as_any_box().downcast::<ProfileSerializedCreateSpec>()?),
            StructType::HostProfileSerializedHostProfileSpec => Ok(from.as_any_box().downcast::<HostProfileSerializedHostProfileSpec>()?),
            StructType::ClusterProfileCreateSpec => Ok(from.as_any_box().downcast::<ClusterProfileCreateSpec>()?),
            StructType::ClusterProfileConfigSpec => Ok(from.as_any_box().downcast::<ClusterProfileConfigSpec>()?),
            StructType::ClusterProfileCompleteConfigSpec => Ok(from.as_any_box().downcast::<ClusterProfileCompleteConfigSpec>()?),
            StructType::ClusterProfileConfigServiceCreateSpec => Ok(from.as_any_box().downcast::<ClusterProfileConfigServiceCreateSpec>()?),
            StructType::HostProfileConfigSpec => Ok(from.as_any_box().downcast::<HostProfileConfigSpec>()?),
            StructType::HostProfileCompleteConfigSpec => Ok(from.as_any_box().downcast::<HostProfileCompleteConfigSpec>()?),
            StructType::HostProfileHostBasedConfigSpec => Ok(from.as_any_box().downcast::<HostProfileHostBasedConfigSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
