use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base type for the various *PerfEntityMetric*
/// encodings.
pub trait PerfEntityMetricBaseTrait : super::data_object_trait::DataObjectTrait {
    /// Performance provider ID.
    fn get_entity(&self) -> &ManagedObjectReference;
}
impl<'s> serde::Serialize for dyn PerfEntityMetricBaseTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn PerfEntityMetricBaseTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(PerfEntityMetricBaseVisitor)
            }
        }

struct PerfEntityMetricBaseVisitor;

impl<'de> de::Visitor<'de> for PerfEntityMetricBaseVisitor {
    type Value = Box<dyn PerfEntityMetricBaseTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid PerfEntityMetricBaseTrait JSON object with a _typeName field")
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

impl PerfEntityMetricBaseTrait for PerfEntityMetricBase {
    fn get_entity(&self) -> &ManagedObjectReference { &self.entity }
}
impl PerfEntityMetricBaseTrait for PerfEntityMetric {
    fn get_entity(&self) -> &ManagedObjectReference { &self.entity }
}
impl PerfEntityMetricBaseTrait for PerfEntityMetricCsv {
    fn get_entity(&self) -> &ManagedObjectReference { &self.entity }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn PerfEntityMetricBaseTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::PerfEntityMetricBase => Some(from.as_any_ref().downcast_ref::<PerfEntityMetricBase>()?),
            StructType::PerfEntityMetric => Some(from.as_any_ref().downcast_ref::<PerfEntityMetric>()?),
            StructType::PerfEntityMetricCsv => Some(from.as_any_ref().downcast_ref::<PerfEntityMetricCsv>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::PerfEntityMetricBase => Ok(from.as_any_box().downcast::<PerfEntityMetricBase>()?),
            StructType::PerfEntityMetric => Ok(from.as_any_box().downcast::<PerfEntityMetric>()?),
            StructType::PerfEntityMetricCsv => Ok(from.as_any_box().downcast::<PerfEntityMetricCsv>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
