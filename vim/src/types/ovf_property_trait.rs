use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A base fault for property faults in the property section of the Ovf XML descriptor.
pub trait OvfPropertyTrait : super::ovf_invalid_package_trait::OvfInvalidPackageTrait {
    /// The type of the property
    fn get_type(&self) -> &str;
    /// The value of the property
    fn get_value(&self) -> &str;
}
impl<'s> serde::Serialize for dyn OvfPropertyTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn OvfPropertyTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(OvfPropertyVisitor)
            }
        }

struct OvfPropertyVisitor;

impl<'de> de::Visitor<'de> for OvfPropertyVisitor {
    type Value = Box<dyn OvfPropertyTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid OvfPropertyTrait JSON object with a _typeName field")
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

impl OvfPropertyTrait for OvfProperty {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_value(&self) -> &str { &self.value }
}
impl OvfPropertyTrait for OvfPropertyNetwork {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_value(&self) -> &str { &self.value }
}
impl OvfPropertyTrait for OvfPropertyQualifier {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_value(&self) -> &str { &self.value }
}
impl OvfPropertyTrait for OvfPropertyQualifierDuplicate {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_value(&self) -> &str { &self.value }
}
impl OvfPropertyTrait for OvfPropertyQualifierIgnored {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_value(&self) -> &str { &self.value }
}
impl OvfPropertyTrait for OvfPropertyType {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_value(&self) -> &str { &self.value }
}
impl OvfPropertyTrait for OvfPropertyValue {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_value(&self) -> &str { &self.value }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn OvfPropertyTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfProperty => Some(from.as_any_ref().downcast_ref::<OvfProperty>()?),
            StructType::OvfPropertyNetwork => Some(from.as_any_ref().downcast_ref::<OvfPropertyNetwork>()?),
            StructType::OvfPropertyQualifier => Some(from.as_any_ref().downcast_ref::<OvfPropertyQualifier>()?),
            StructType::OvfPropertyQualifierDuplicate => Some(from.as_any_ref().downcast_ref::<OvfPropertyQualifierDuplicate>()?),
            StructType::OvfPropertyQualifierIgnored => Some(from.as_any_ref().downcast_ref::<OvfPropertyQualifierIgnored>()?),
            StructType::OvfPropertyType => Some(from.as_any_ref().downcast_ref::<OvfPropertyType>()?),
            StructType::OvfPropertyValue => Some(from.as_any_ref().downcast_ref::<OvfPropertyValue>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfProperty => Ok(from.as_any_box().downcast::<OvfProperty>()?),
            StructType::OvfPropertyNetwork => Ok(from.as_any_box().downcast::<OvfPropertyNetwork>()?),
            StructType::OvfPropertyQualifier => Ok(from.as_any_box().downcast::<OvfPropertyQualifier>()?),
            StructType::OvfPropertyQualifierDuplicate => Ok(from.as_any_box().downcast::<OvfPropertyQualifierDuplicate>()?),
            StructType::OvfPropertyQualifierIgnored => Ok(from.as_any_box().downcast::<OvfPropertyQualifierIgnored>()?),
            StructType::OvfPropertyType => Ok(from.as_any_box().downcast::<OvfPropertyType>()?),
            StructType::OvfPropertyValue => Ok(from.as_any_box().downcast::<OvfPropertyValue>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
