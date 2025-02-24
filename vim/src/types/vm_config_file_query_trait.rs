use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type describes query specification for the virtual machine
/// configuration file.
pub trait VmConfigFileQueryTrait : super::file_query_trait::FileQueryTrait {
    /// The filter specification for the virtual machine configuration file query.
    fn get_filter(&self) -> &Option<VmConfigFileQueryFilter>;
    /// The details specification for the virtual machine configuration file query.
    fn get_details(&self) -> &Option<VmConfigFileQueryFlags>;
}
impl<'s> serde::Serialize for dyn VmConfigFileQueryTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VmConfigFileQueryTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VmConfigFileQueryVisitor)
            }
        }

struct VmConfigFileQueryVisitor;

impl<'de> de::Visitor<'de> for VmConfigFileQueryVisitor {
    type Value = Box<dyn VmConfigFileQueryTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VmConfigFileQueryTrait JSON object with a _typeName field")
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

impl VmConfigFileQueryTrait for VmConfigFileQuery {
    fn get_filter(&self) -> &Option<VmConfigFileQueryFilter> { &self.filter }
    fn get_details(&self) -> &Option<VmConfigFileQueryFlags> { &self.details }
}
impl VmConfigFileQueryTrait for TemplateConfigFileQuery {
    fn get_filter(&self) -> &Option<VmConfigFileQueryFilter> { &self.filter }
    fn get_details(&self) -> &Option<VmConfigFileQueryFlags> { &self.details }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VmConfigFileQueryTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmConfigFileQuery => Some(from.as_any_ref().downcast_ref::<VmConfigFileQuery>()?),
            StructType::TemplateConfigFileQuery => Some(from.as_any_ref().downcast_ref::<TemplateConfigFileQuery>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmConfigFileQuery => Ok(from.as_any_box().downcast::<VmConfigFileQuery>()?),
            StructType::TemplateConfigFileQuery => Ok(from.as_any_box().downcast::<TemplateConfigFileQuery>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
