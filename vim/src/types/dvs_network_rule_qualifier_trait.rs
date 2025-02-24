use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This class is the base class for identifying network traffic.
pub trait DvsNetworkRuleQualifierTrait : super::data_object_trait::DataObjectTrait {
    /// The key of the Qualifier
    fn get_key(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn DvsNetworkRuleQualifierTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DvsNetworkRuleQualifierTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DvsNetworkRuleQualifierVisitor)
            }
        }

struct DvsNetworkRuleQualifierVisitor;

impl<'de> de::Visitor<'de> for DvsNetworkRuleQualifierVisitor {
    type Value = Box<dyn DvsNetworkRuleQualifierTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DvsNetworkRuleQualifierTrait JSON object with a _typeName field")
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

impl DvsNetworkRuleQualifierTrait for DvsNetworkRuleQualifier {
    fn get_key(&self) -> &Option<String> { &self.key }
}
impl DvsNetworkRuleQualifierTrait for DvsIpNetworkRuleQualifier {
    fn get_key(&self) -> &Option<String> { &self.key }
}
impl DvsNetworkRuleQualifierTrait for DvsMacNetworkRuleQualifier {
    fn get_key(&self) -> &Option<String> { &self.key }
}
impl DvsNetworkRuleQualifierTrait for DvsSystemTrafficNetworkRuleQualifier {
    fn get_key(&self) -> &Option<String> { &self.key }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DvsNetworkRuleQualifierTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsNetworkRuleQualifier => Some(from.as_any_ref().downcast_ref::<DvsNetworkRuleQualifier>()?),
            StructType::DvsIpNetworkRuleQualifier => Some(from.as_any_ref().downcast_ref::<DvsIpNetworkRuleQualifier>()?),
            StructType::DvsMacNetworkRuleQualifier => Some(from.as_any_ref().downcast_ref::<DvsMacNetworkRuleQualifier>()?),
            StructType::DvsSystemTrafficNetworkRuleQualifier => Some(from.as_any_ref().downcast_ref::<DvsSystemTrafficNetworkRuleQualifier>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsNetworkRuleQualifier => Ok(from.as_any_box().downcast::<DvsNetworkRuleQualifier>()?),
            StructType::DvsIpNetworkRuleQualifier => Ok(from.as_any_box().downcast::<DvsIpNetworkRuleQualifier>()?),
            StructType::DvsMacNetworkRuleQualifier => Ok(from.as_any_box().downcast::<DvsMacNetworkRuleQualifier>()?),
            StructType::DvsSystemTrafficNetworkRuleQualifier => Ok(from.as_any_box().downcast::<DvsSystemTrafficNetworkRuleQualifier>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
