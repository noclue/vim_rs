use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base type for sysprep, sysprepText, or linuxPrep object type.
pub trait CustomizationIdentitySettingsTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn CustomizationIdentitySettingsTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn CustomizationIdentitySettingsTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(CustomizationIdentitySettingsVisitor)
            }
        }

struct CustomizationIdentitySettingsVisitor;

impl<'de> de::Visitor<'de> for CustomizationIdentitySettingsVisitor {
    type Value = Box<dyn CustomizationIdentitySettingsTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid CustomizationIdentitySettingsTrait JSON object with a _typeName field")
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

impl CustomizationIdentitySettingsTrait for CustomizationIdentitySettings {
}
impl CustomizationIdentitySettingsTrait for CustomizationCloudinitPrep {
}
impl CustomizationIdentitySettingsTrait for CustomizationLinuxPrep {
}
impl CustomizationIdentitySettingsTrait for CustomizationSysprep {
}
impl CustomizationIdentitySettingsTrait for CustomizationSysprepText {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn CustomizationIdentitySettingsTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomizationIdentitySettings => Some(from.as_any_ref().downcast_ref::<CustomizationIdentitySettings>()?),
            StructType::CustomizationCloudinitPrep => Some(from.as_any_ref().downcast_ref::<CustomizationCloudinitPrep>()?),
            StructType::CustomizationLinuxPrep => Some(from.as_any_ref().downcast_ref::<CustomizationLinuxPrep>()?),
            StructType::CustomizationSysprep => Some(from.as_any_ref().downcast_ref::<CustomizationSysprep>()?),
            StructType::CustomizationSysprepText => Some(from.as_any_ref().downcast_ref::<CustomizationSysprepText>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::CustomizationIdentitySettings => Ok(from.as_any_box().downcast::<CustomizationIdentitySettings>()?),
            StructType::CustomizationCloudinitPrep => Ok(from.as_any_box().downcast::<CustomizationCloudinitPrep>()?),
            StructType::CustomizationLinuxPrep => Ok(from.as_any_box().downcast::<CustomizationLinuxPrep>()?),
            StructType::CustomizationSysprep => Ok(from.as_any_box().downcast::<CustomizationSysprep>()?),
            StructType::CustomizationSysprepText => Ok(from.as_any_box().downcast::<CustomizationSysprepText>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
