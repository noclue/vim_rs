use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Static strings used for describing an object model string or enumeration.
pub trait ElementDescriptionTrait : super::description_trait::DescriptionTrait {
    /// Enumeration or literal ID being described.
    fn get_key(&self) -> &str;
}
impl<'s> serde::Serialize for dyn ElementDescriptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ElementDescriptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ElementDescriptionVisitor)
            }
        }

struct ElementDescriptionVisitor;

impl<'de> de::Visitor<'de> for ElementDescriptionVisitor {
    type Value = Box<dyn ElementDescriptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ElementDescriptionTrait JSON object with a _typeName field")
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

impl ElementDescriptionTrait for ElementDescription {
    fn get_key(&self) -> &str { &self.key }
}
impl ElementDescriptionTrait for EvcMode {
    fn get_key(&self) -> &str { &self.key }
}
impl ElementDescriptionTrait for ExtendedElementDescription {
    fn get_key(&self) -> &str { &self.key }
}
impl ElementDescriptionTrait for FeatureEvcMode {
    fn get_key(&self) -> &str { &self.key }
}
impl ElementDescriptionTrait for OptionDef {
    fn get_key(&self) -> &str { &self.key }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ElementDescriptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ElementDescription => Some(from.as_any_ref().downcast_ref::<ElementDescription>()?),
            StructType::EvcMode => Some(from.as_any_ref().downcast_ref::<EvcMode>()?),
            StructType::ExtendedElementDescription => Some(from.as_any_ref().downcast_ref::<ExtendedElementDescription>()?),
            StructType::FeatureEvcMode => Some(from.as_any_ref().downcast_ref::<FeatureEvcMode>()?),
            StructType::OptionDef => Some(from.as_any_ref().downcast_ref::<OptionDef>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ElementDescription => Ok(from.as_any_box().downcast::<ElementDescription>()?),
            StructType::EvcMode => Ok(from.as_any_box().downcast::<EvcMode>()?),
            StructType::ExtendedElementDescription => Ok(from.as_any_box().downcast::<ExtendedElementDescription>()?),
            StructType::FeatureEvcMode => Ok(from.as_any_box().downcast::<FeatureEvcMode>()?),
            StructType::OptionDef => Ok(from.as_any_box().downcast::<OptionDef>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
