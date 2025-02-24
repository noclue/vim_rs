use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Describes the key/value pair of a configured option.
pub trait OptionValueTrait : super::data_object_trait::DataObjectTrait {
    /// The name of the option using dot notation to reflect the
    /// option's position in a hierarchy.
    /// 
    /// For example, you might
    /// have an option called "Ethernet" and another option that is
    /// a child of that called "Connection". In this case, the key
    /// for the latter could be defined as "Ethernet.Connection"
    fn get_key(&self) -> &str;
    /// The value of the option.
    /// 
    /// The Any data object type enables you to
    /// define any value for the option. Typically, however, the value
    /// of an option is of type String or Integer.
    fn get_value(&self) -> &Option<VimAny>;
}
impl<'s> serde::Serialize for dyn OptionValueTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn OptionValueTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(OptionValueVisitor)
            }
        }

struct OptionValueVisitor;

impl<'de> de::Visitor<'de> for OptionValueVisitor {
    type Value = Box<dyn OptionValueTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid OptionValueTrait JSON object with a _typeName field")
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

impl OptionValueTrait for OptionValue {
    fn get_key(&self) -> &str { &self.key }
    fn get_value(&self) -> &Option<VimAny> { &self.value }
}
impl OptionValueTrait for HostInternetScsiHbaParamValue {
    fn get_key(&self) -> &str { &self.key }
    fn get_value(&self) -> &Option<VimAny> { &self.value }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn OptionValueTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::OptionValue => Some(from.as_any_ref().downcast_ref::<OptionValue>()?),
            StructType::HostInternetScsiHbaParamValue => Some(from.as_any_ref().downcast_ref::<HostInternetScsiHbaParamValue>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::OptionValue => Ok(from.as_any_box().downcast::<OptionValue>()?),
            StructType::HostInternetScsiHbaParamValue => Ok(from.as_any_box().downcast::<HostInternetScsiHbaParamValue>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
