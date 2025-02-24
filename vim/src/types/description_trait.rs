use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Static strings used for describing an object or property.
pub trait DescriptionTrait : super::data_object_trait::DataObjectTrait {
    /// Display label.
    fn get_label(&self) -> &str;
    /// Summary description.
    fn get_summary(&self) -> &str;
}
impl<'s> serde::Serialize for dyn DescriptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DescriptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DescriptionVisitor)
            }
        }

struct DescriptionVisitor;

impl<'de> de::Visitor<'de> for DescriptionVisitor {
    type Value = Box<dyn DescriptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DescriptionTrait JSON object with a _typeName field")
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

impl DescriptionTrait for Description {
    fn get_label(&self) -> &str { &self.label }
    fn get_summary(&self) -> &str { &self.summary }
}
impl DescriptionTrait for ElementDescription {
    fn get_label(&self) -> &str { &self.label }
    fn get_summary(&self) -> &str { &self.summary }
}
impl DescriptionTrait for EvcMode {
    fn get_label(&self) -> &str { &self.label }
    fn get_summary(&self) -> &str { &self.summary }
}
impl DescriptionTrait for ExtendedElementDescription {
    fn get_label(&self) -> &str { &self.label }
    fn get_summary(&self) -> &str { &self.summary }
}
impl DescriptionTrait for FeatureEvcMode {
    fn get_label(&self) -> &str { &self.label }
    fn get_summary(&self) -> &str { &self.summary }
}
impl DescriptionTrait for OptionDef {
    fn get_label(&self) -> &str { &self.label }
    fn get_summary(&self) -> &str { &self.summary }
}
impl DescriptionTrait for ExtendedDescription {
    fn get_label(&self) -> &str { &self.label }
    fn get_summary(&self) -> &str { &self.summary }
}
impl DescriptionTrait for MethodDescription {
    fn get_label(&self) -> &str { &self.label }
    fn get_summary(&self) -> &str { &self.summary }
}
impl DescriptionTrait for TypeDescription {
    fn get_label(&self) -> &str { &self.label }
    fn get_summary(&self) -> &str { &self.summary }
}
impl DescriptionTrait for ScheduledTaskDetail {
    fn get_label(&self) -> &str { &self.label }
    fn get_summary(&self) -> &str { &self.summary }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DescriptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::Description => Some(from.as_any_ref().downcast_ref::<Description>()?),
            StructType::ElementDescription => Some(from.as_any_ref().downcast_ref::<ElementDescription>()?),
            StructType::EvcMode => Some(from.as_any_ref().downcast_ref::<EvcMode>()?),
            StructType::ExtendedElementDescription => Some(from.as_any_ref().downcast_ref::<ExtendedElementDescription>()?),
            StructType::FeatureEvcMode => Some(from.as_any_ref().downcast_ref::<FeatureEvcMode>()?),
            StructType::OptionDef => Some(from.as_any_ref().downcast_ref::<OptionDef>()?),
            StructType::ExtendedDescription => Some(from.as_any_ref().downcast_ref::<ExtendedDescription>()?),
            StructType::MethodDescription => Some(from.as_any_ref().downcast_ref::<MethodDescription>()?),
            StructType::TypeDescription => Some(from.as_any_ref().downcast_ref::<TypeDescription>()?),
            StructType::ScheduledTaskDetail => Some(from.as_any_ref().downcast_ref::<ScheduledTaskDetail>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::Description => Ok(from.as_any_box().downcast::<Description>()?),
            StructType::ElementDescription => Ok(from.as_any_box().downcast::<ElementDescription>()?),
            StructType::EvcMode => Ok(from.as_any_box().downcast::<EvcMode>()?),
            StructType::ExtendedElementDescription => Ok(from.as_any_box().downcast::<ExtendedElementDescription>()?),
            StructType::FeatureEvcMode => Ok(from.as_any_box().downcast::<FeatureEvcMode>()?),
            StructType::OptionDef => Ok(from.as_any_box().downcast::<OptionDef>()?),
            StructType::ExtendedDescription => Ok(from.as_any_box().downcast::<ExtendedDescription>()?),
            StructType::MethodDescription => Ok(from.as_any_box().downcast::<MethodDescription>()?),
            StructType::TypeDescription => Ok(from.as_any_box().downcast::<TypeDescription>()?),
            StructType::ScheduledTaskDetail => Ok(from.as_any_box().downcast::<ScheduledTaskDetail>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
