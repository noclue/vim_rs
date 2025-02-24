use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Data type used to contain a representation of host or cluster customization
/// data in a *HostProfilesCustomizationData* object.
/// 
/// Subclasses of this must be defined to provide host or cluster customization
/// data in specific formats.
pub trait HostProfilesEntityCustomizationsTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn HostProfilesEntityCustomizationsTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostProfilesEntityCustomizationsTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostProfilesEntityCustomizationsVisitor)
            }
        }

struct HostProfilesEntityCustomizationsVisitor;

impl<'de> de::Visitor<'de> for HostProfilesEntityCustomizationsVisitor {
    type Value = Box<dyn HostProfilesEntityCustomizationsTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostProfilesEntityCustomizationsTrait JSON object with a _typeName field")
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

impl HostProfilesEntityCustomizationsTrait for HostProfilesEntityCustomizations {
}
impl HostProfilesEntityCustomizationsTrait for StructuredCustomizations {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostProfilesEntityCustomizationsTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostProfilesEntityCustomizations => Some(from.as_any_ref().downcast_ref::<HostProfilesEntityCustomizations>()?),
            StructType::StructuredCustomizations => Some(from.as_any_ref().downcast_ref::<StructuredCustomizations>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostProfilesEntityCustomizations => Ok(from.as_any_box().downcast::<HostProfilesEntityCustomizations>()?),
            StructType::StructuredCustomizations => Ok(from.as_any_box().downcast::<StructuredCustomizations>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
