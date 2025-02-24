use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The base class for any type of setting or configuration to which negation
/// can be applied.
/// 
/// When used in a configuration information object:
/// if *NegatableExpression.negate* is true, then ~(objectValue) will be used for the
/// configuration. If false, then objectValue will be used as it is.
pub trait NegatableExpressionTrait : super::data_object_trait::DataObjectTrait {
    /// Whether the configuration needs to be negated or not.
    fn get_negate(&self) -> Option<bool>;
}
impl<'s> serde::Serialize for dyn NegatableExpressionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn NegatableExpressionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(NegatableExpressionVisitor)
            }
        }

struct NegatableExpressionVisitor;

impl<'de> de::Visitor<'de> for NegatableExpressionVisitor {
    type Value = Box<dyn NegatableExpressionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid NegatableExpressionTrait JSON object with a _typeName field")
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

impl NegatableExpressionTrait for NegatableExpression {
    fn get_negate(&self) -> Option<bool> { self.negate }
}
impl NegatableExpressionTrait for IntExpression {
    fn get_negate(&self) -> Option<bool> { self.negate }
}
impl NegatableExpressionTrait for IpAddress {
    fn get_negate(&self) -> Option<bool> { self.negate }
}
impl NegatableExpressionTrait for IpRange {
    fn get_negate(&self) -> Option<bool> { self.negate }
}
impl NegatableExpressionTrait for SingleIp {
    fn get_negate(&self) -> Option<bool> { self.negate }
}
impl NegatableExpressionTrait for MacAddress {
    fn get_negate(&self) -> Option<bool> { self.negate }
}
impl NegatableExpressionTrait for MacRange {
    fn get_negate(&self) -> Option<bool> { self.negate }
}
impl NegatableExpressionTrait for SingleMac {
    fn get_negate(&self) -> Option<bool> { self.negate }
}
impl NegatableExpressionTrait for StringExpression {
    fn get_negate(&self) -> Option<bool> { self.negate }
}
impl NegatableExpressionTrait for DvsIpPort {
    fn get_negate(&self) -> Option<bool> { self.negate }
}
impl NegatableExpressionTrait for DvsIpPortRange {
    fn get_negate(&self) -> Option<bool> { self.negate }
}
impl NegatableExpressionTrait for DvsSingleIpPort {
    fn get_negate(&self) -> Option<bool> { self.negate }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn NegatableExpressionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::NegatableExpression => Some(from.as_any_ref().downcast_ref::<NegatableExpression>()?),
            StructType::IntExpression => Some(from.as_any_ref().downcast_ref::<IntExpression>()?),
            StructType::IpAddress => Some(from.as_any_ref().downcast_ref::<IpAddress>()?),
            StructType::IpRange => Some(from.as_any_ref().downcast_ref::<IpRange>()?),
            StructType::SingleIp => Some(from.as_any_ref().downcast_ref::<SingleIp>()?),
            StructType::MacAddress => Some(from.as_any_ref().downcast_ref::<MacAddress>()?),
            StructType::MacRange => Some(from.as_any_ref().downcast_ref::<MacRange>()?),
            StructType::SingleMac => Some(from.as_any_ref().downcast_ref::<SingleMac>()?),
            StructType::StringExpression => Some(from.as_any_ref().downcast_ref::<StringExpression>()?),
            StructType::DvsIpPort => Some(from.as_any_ref().downcast_ref::<DvsIpPort>()?),
            StructType::DvsIpPortRange => Some(from.as_any_ref().downcast_ref::<DvsIpPortRange>()?),
            StructType::DvsSingleIpPort => Some(from.as_any_ref().downcast_ref::<DvsSingleIpPort>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::NegatableExpression => Ok(from.as_any_box().downcast::<NegatableExpression>()?),
            StructType::IntExpression => Ok(from.as_any_box().downcast::<IntExpression>()?),
            StructType::IpAddress => Ok(from.as_any_box().downcast::<IpAddress>()?),
            StructType::IpRange => Ok(from.as_any_box().downcast::<IpRange>()?),
            StructType::SingleIp => Ok(from.as_any_box().downcast::<SingleIp>()?),
            StructType::MacAddress => Ok(from.as_any_box().downcast::<MacAddress>()?),
            StructType::MacRange => Ok(from.as_any_box().downcast::<MacRange>()?),
            StructType::SingleMac => Ok(from.as_any_box().downcast::<SingleMac>()?),
            StructType::StringExpression => Ok(from.as_any_box().downcast::<StringExpression>()?),
            StructType::DvsIpPort => Ok(from.as_any_box().downcast::<DvsIpPort>()?),
            StructType::DvsIpPortRange => Ok(from.as_any_box().downcast::<DvsIpPortRange>()?),
            StructType::DvsSingleIpPort => Ok(from.as_any_box().downcast::<DvsSingleIpPort>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
