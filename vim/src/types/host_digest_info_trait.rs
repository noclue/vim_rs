use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type describes the digest information
pub trait HostDigestInfoTrait : super::data_object_trait::DataObjectTrait {
    /// Method in which the digest value is calculated.
    /// 
    /// The set of possible
    /// values is described in *HostDigestInfoDigestMethodType_enum*.
    fn get_digest_method(&self) -> &str;
    /// The variable length byte array containing the digest value calculated by
    /// the specified digestMethod.
    fn get_digest_value(&self) -> &Vec<i8>;
    /// The name of the object from which this digest value is calcaulated.
    fn get_object_name(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn HostDigestInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostDigestInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostDigestInfoVisitor)
            }
        }

struct HostDigestInfoVisitor;

impl<'de> de::Visitor<'de> for HostDigestInfoVisitor {
    type Value = Box<dyn HostDigestInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostDigestInfoTrait JSON object with a _typeName field")
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

impl HostDigestInfoTrait for HostDigestInfo {
    fn get_digest_method(&self) -> &str { &self.digest_method }
    fn get_digest_value(&self) -> &Vec<i8> { &self.digest_value }
    fn get_object_name(&self) -> &Option<String> { &self.object_name }
}
impl HostDigestInfoTrait for HostTpmDigestInfo {
    fn get_digest_method(&self) -> &str { &self.digest_method }
    fn get_digest_value(&self) -> &Vec<i8> { &self.digest_value }
    fn get_object_name(&self) -> &Option<String> { &self.object_name }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostDigestInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostDigestInfo => Some(from.as_any_ref().downcast_ref::<HostDigestInfo>()?),
            StructType::HostTpmDigestInfo => Some(from.as_any_ref().downcast_ref::<HostTpmDigestInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostDigestInfo => Ok(from.as_any_box().downcast::<HostDigestInfo>()?),
            StructType::HostTpmDigestInfo => Ok(from.as_any_box().downcast::<HostTpmDigestInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
