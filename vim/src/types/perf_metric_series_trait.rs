use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This is a generic data object type that stores values for a specific
/// performance metric.
/// 
/// Useful data objects that store actual metric values
/// extend this data object (see *PerfMetricIntSeries*).
pub trait PerfMetricSeriesTrait : super::data_object_trait::DataObjectTrait {
    /// An identifier for the performance metric.
    fn get_id(&self) -> &PerfMetricId;
}
impl<'s> serde::Serialize for dyn PerfMetricSeriesTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn PerfMetricSeriesTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(PerfMetricSeriesVisitor)
            }
        }

struct PerfMetricSeriesVisitor;

impl<'de> de::Visitor<'de> for PerfMetricSeriesVisitor {
    type Value = Box<dyn PerfMetricSeriesTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid PerfMetricSeriesTrait JSON object with a _typeName field")
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

impl PerfMetricSeriesTrait for PerfMetricSeries {
    fn get_id(&self) -> &PerfMetricId { &self.id }
}
impl PerfMetricSeriesTrait for PerfMetricIntSeries {
    fn get_id(&self) -> &PerfMetricId { &self.id }
}
impl PerfMetricSeriesTrait for PerfMetricSeriesCsv {
    fn get_id(&self) -> &PerfMetricId { &self.id }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn PerfMetricSeriesTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::PerfMetricSeries => Some(from.as_any_ref().downcast_ref::<PerfMetricSeries>()?),
            StructType::PerfMetricIntSeries => Some(from.as_any_ref().downcast_ref::<PerfMetricIntSeries>()?),
            StructType::PerfMetricSeriesCsv => Some(from.as_any_ref().downcast_ref::<PerfMetricSeriesCsv>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::PerfMetricSeries => Ok(from.as_any_box().downcast::<PerfMetricSeries>()?),
            StructType::PerfMetricIntSeries => Ok(from.as_any_box().downcast::<PerfMetricIntSeries>()?),
            StructType::PerfMetricSeriesCsv => Ok(from.as_any_box().downcast::<PerfMetricSeriesCsv>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
