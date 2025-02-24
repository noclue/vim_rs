use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// An ExpiredFeatureLicense fault is thrown if an attempt to acquire an Addon license
/// 'feature failed for count 'count'.
pub trait ExpiredFeatureLicenseTrait : super::not_enough_licenses_trait::NotEnoughLicensesTrait {
    fn get_feature(&self) -> &str;
    fn get_count(&self) -> i32;
    fn get_expiration_date(&self) -> &str;
}
impl<'s> serde::Serialize for dyn ExpiredFeatureLicenseTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ExpiredFeatureLicenseTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ExpiredFeatureLicenseVisitor)
            }
        }

struct ExpiredFeatureLicenseVisitor;

impl<'de> de::Visitor<'de> for ExpiredFeatureLicenseVisitor {
    type Value = Box<dyn ExpiredFeatureLicenseTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ExpiredFeatureLicenseTrait JSON object with a _typeName field")
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

impl ExpiredFeatureLicenseTrait for ExpiredFeatureLicense {
    fn get_feature(&self) -> &str { &self.feature }
    fn get_count(&self) -> i32 { self.count }
    fn get_expiration_date(&self) -> &str { &self.expiration_date }
}
impl ExpiredFeatureLicenseTrait for ExpiredAddonLicense {
    fn get_feature(&self) -> &str { &self.feature }
    fn get_count(&self) -> i32 { self.count }
    fn get_expiration_date(&self) -> &str { &self.expiration_date }
}
impl ExpiredFeatureLicenseTrait for ExpiredEditionLicense {
    fn get_feature(&self) -> &str { &self.feature }
    fn get_count(&self) -> i32 { self.count }
    fn get_expiration_date(&self) -> &str { &self.expiration_date }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ExpiredFeatureLicenseTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ExpiredFeatureLicense => Some(from.as_any_ref().downcast_ref::<ExpiredFeatureLicense>()?),
            StructType::ExpiredAddonLicense => Some(from.as_any_ref().downcast_ref::<ExpiredAddonLicense>()?),
            StructType::ExpiredEditionLicense => Some(from.as_any_ref().downcast_ref::<ExpiredEditionLicense>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ExpiredFeatureLicense => Ok(from.as_any_box().downcast::<ExpiredFeatureLicense>()?),
            StructType::ExpiredAddonLicense => Ok(from.as_any_box().downcast::<ExpiredAddonLicense>()?),
            StructType::ExpiredEditionLicense => Ok(from.as_any_box().downcast::<ExpiredEditionLicense>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
