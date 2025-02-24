use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The base data object type for all options.
pub trait OptionTypeTrait : super::data_object_trait::DataObjectTrait {
    /// The flag to indicate whether or not a user
    /// can modify a value belonging to this option type.
    /// 
    /// If
    /// the flag is not set, the value can be modified.
    fn get_value_is_readonly(&self) -> Option<bool>;
}
impl<'s> serde::Serialize for dyn OptionTypeTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn OptionTypeTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(OptionTypeVisitor)
            }
        }

struct OptionTypeVisitor;

impl<'de> de::Visitor<'de> for OptionTypeVisitor {
    type Value = Box<dyn OptionTypeTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid OptionTypeTrait JSON object with a _typeName field")
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

impl OptionTypeTrait for OptionType {
    fn get_value_is_readonly(&self) -> Option<bool> { self.value_is_readonly }
}
impl OptionTypeTrait for BoolOption {
    fn get_value_is_readonly(&self) -> Option<bool> { self.value_is_readonly }
}
impl OptionTypeTrait for ChoiceOption {
    fn get_value_is_readonly(&self) -> Option<bool> { self.value_is_readonly }
}
impl OptionTypeTrait for FloatOption {
    fn get_value_is_readonly(&self) -> Option<bool> { self.value_is_readonly }
}
impl OptionTypeTrait for IntOption {
    fn get_value_is_readonly(&self) -> Option<bool> { self.value_is_readonly }
}
impl OptionTypeTrait for LongOption {
    fn get_value_is_readonly(&self) -> Option<bool> { self.value_is_readonly }
}
impl OptionTypeTrait for StringOption {
    fn get_value_is_readonly(&self) -> Option<bool> { self.value_is_readonly }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn OptionTypeTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::OptionType => Some(from.as_any_ref().downcast_ref::<OptionType>()?),
            StructType::BoolOption => Some(from.as_any_ref().downcast_ref::<BoolOption>()?),
            StructType::ChoiceOption => Some(from.as_any_ref().downcast_ref::<ChoiceOption>()?),
            StructType::FloatOption => Some(from.as_any_ref().downcast_ref::<FloatOption>()?),
            StructType::IntOption => Some(from.as_any_ref().downcast_ref::<IntOption>()?),
            StructType::LongOption => Some(from.as_any_ref().downcast_ref::<LongOption>()?),
            StructType::StringOption => Some(from.as_any_ref().downcast_ref::<StringOption>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::OptionType => Ok(from.as_any_box().downcast::<OptionType>()?),
            StructType::BoolOption => Ok(from.as_any_box().downcast::<BoolOption>()?),
            StructType::ChoiceOption => Ok(from.as_any_box().downcast::<ChoiceOption>()?),
            StructType::FloatOption => Ok(from.as_any_box().downcast::<FloatOption>()?),
            StructType::IntOption => Ok(from.as_any_box().downcast::<IntOption>()?),
            StructType::LongOption => Ok(from.as_any_box().downcast::<LongOption>()?),
            StructType::StringOption => Ok(from.as_any_box().downcast::<StringOption>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
