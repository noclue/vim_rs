use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A common super-class for basic OVF descriptor parameters
pub trait OvfManagerCommonParamsTrait : super::data_object_trait::DataObjectTrait {
    /// The locale-identifier to choose from the descriptor.
    /// 
    /// If empty, the
    /// default locale on the server is used.
    fn get_locale(&self) -> &str;
    /// The key of the chosen deployment option.
    /// 
    /// If empty, the default option is
    /// chosen. The list of possible deployment options is returned in the result of
    /// parseDescriptor.
    fn get_deployment_option(&self) -> &str;
    /// An optional set of localization strings to be used.
    /// 
    /// The server will use
    /// these message strings to localize information in the result and in
    /// error and warning messages.
    /// 
    /// This argument allows a client to pass messages from external
    /// string bundles. The client is responsible for selecting the right string
    /// bundle (based on locale) and parsing the external string bundle. The
    /// passed in key/value pairs are looked up before any messages
    /// included in the OVF descriptor itself.
    fn get_msg_bundle(&self) -> &Option<Vec<KeyValue>>;
    /// An optional argument for modifing the OVF parsing.
    /// 
    /// When the server parses an OVF
    /// descriptor a set of options can be used to modify the parsing. The argument is a list
    /// of keywords.
    /// 
    /// To get a list of supported keywords see *OvfManager.ovfImportOption*. Unknown
    /// options will be ignored by the server.
    fn get_import_option(&self) -> &Option<Vec<String>>;
}
impl<'s> serde::Serialize for dyn OvfManagerCommonParamsTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn OvfManagerCommonParamsTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(OvfManagerCommonParamsVisitor)
            }
        }

struct OvfManagerCommonParamsVisitor;

impl<'de> de::Visitor<'de> for OvfManagerCommonParamsVisitor {
    type Value = Box<dyn OvfManagerCommonParamsTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid OvfManagerCommonParamsTrait JSON object with a _typeName field")
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

impl OvfManagerCommonParamsTrait for OvfManagerCommonParams {
    fn get_locale(&self) -> &str { &self.locale }
    fn get_deployment_option(&self) -> &str { &self.deployment_option }
    fn get_msg_bundle(&self) -> &Option<Vec<KeyValue>> { &self.msg_bundle }
    fn get_import_option(&self) -> &Option<Vec<String>> { &self.import_option }
}
impl OvfManagerCommonParamsTrait for OvfCreateImportSpecParams {
    fn get_locale(&self) -> &str { &self.locale }
    fn get_deployment_option(&self) -> &str { &self.deployment_option }
    fn get_msg_bundle(&self) -> &Option<Vec<KeyValue>> { &self.msg_bundle }
    fn get_import_option(&self) -> &Option<Vec<String>> { &self.import_option }
}
impl OvfManagerCommonParamsTrait for OvfParseDescriptorParams {
    fn get_locale(&self) -> &str { &self.locale }
    fn get_deployment_option(&self) -> &str { &self.deployment_option }
    fn get_msg_bundle(&self) -> &Option<Vec<KeyValue>> { &self.msg_bundle }
    fn get_import_option(&self) -> &Option<Vec<String>> { &self.import_option }
}
impl OvfManagerCommonParamsTrait for OvfValidateHostParams {
    fn get_locale(&self) -> &str { &self.locale }
    fn get_deployment_option(&self) -> &str { &self.deployment_option }
    fn get_msg_bundle(&self) -> &Option<Vec<KeyValue>> { &self.msg_bundle }
    fn get_import_option(&self) -> &Option<Vec<String>> { &self.import_option }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn OvfManagerCommonParamsTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfManagerCommonParams => Some(from.as_any_ref().downcast_ref::<OvfManagerCommonParams>()?),
            StructType::OvfCreateImportSpecParams => Some(from.as_any_ref().downcast_ref::<OvfCreateImportSpecParams>()?),
            StructType::OvfParseDescriptorParams => Some(from.as_any_ref().downcast_ref::<OvfParseDescriptorParams>()?),
            StructType::OvfValidateHostParams => Some(from.as_any_ref().downcast_ref::<OvfValidateHostParams>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfManagerCommonParams => Ok(from.as_any_box().downcast::<OvfManagerCommonParams>()?),
            StructType::OvfCreateImportSpecParams => Ok(from.as_any_box().downcast::<OvfCreateImportSpecParams>()?),
            StructType::OvfParseDescriptorParams => Ok(from.as_any_box().downcast::<OvfParseDescriptorParams>()?),
            StructType::OvfValidateHostParams => Ok(from.as_any_box().downcast::<OvfValidateHostParams>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
