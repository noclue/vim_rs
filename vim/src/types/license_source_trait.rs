use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Deprecated as of vSphere API 4.0, this is not used by the system.
/// 
/// This data object type is used to communicate configuration about
/// where to find licenses to use for this system.
pub trait LicenseSourceTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn LicenseSourceTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn LicenseSourceTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(LicenseSourceVisitor)
            }
        }

struct LicenseSourceVisitor;

impl<'de> de::Visitor<'de> for LicenseSourceVisitor {
    type Value = Box<dyn LicenseSourceTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid LicenseSourceTrait JSON object with a _typeName field")
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

impl LicenseSourceTrait for LicenseSource {
}
impl LicenseSourceTrait for EvaluationLicenseSource {
}
impl LicenseSourceTrait for LicenseServerSource {
}
impl LicenseSourceTrait for LocalLicenseSource {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn LicenseSourceTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::LicenseSource => Some(from.as_any_ref().downcast_ref::<LicenseSource>()?),
            StructType::EvaluationLicenseSource => Some(from.as_any_ref().downcast_ref::<EvaluationLicenseSource>()?),
            StructType::LicenseServerSource => Some(from.as_any_ref().downcast_ref::<LicenseServerSource>()?),
            StructType::LocalLicenseSource => Some(from.as_any_ref().downcast_ref::<LocalLicenseSource>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::LicenseSource => Ok(from.as_any_box().downcast::<LicenseSource>()?),
            StructType::EvaluationLicenseSource => Ok(from.as_any_box().downcast::<EvaluationLicenseSource>()?),
            StructType::LicenseServerSource => Ok(from.as_any_box().downcast::<LicenseServerSource>()?),
            StructType::LocalLicenseSource => Ok(from.as_any_box().downcast::<LocalLicenseSource>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
