use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Information about an IO Filter.
pub trait IoFilterInfoTrait : super::data_object_trait::DataObjectTrait {
    /// IO Filter identifier.
    fn get_id(&self) -> &str;
    /// Name of the IO Filter.
    fn get_name(&self) -> &str;
    /// Vendor of the IO Filter.
    fn get_vendor(&self) -> &str;
    /// Version of the IO Filter.
    fn get_version(&self) -> &str;
    /// Type of the IO Filter.
    /// 
    /// The set of possible values are listed in
    /// *IoFilterType_enum*.
    /// The property is unset if the information is not available.
    fn get_type(&self) -> &Option<String>;
    /// Short description of the IO Filter.
    /// 
    /// The property is unset if the information is not available.
    fn get_summary(&self) -> &Option<String>;
    /// Release date of the IO Filter.
    /// 
    /// The property is unset if the information is not available.
    fn get_release_date(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn IoFilterInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn IoFilterInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(IoFilterInfoVisitor)
            }
        }

struct IoFilterInfoVisitor;

impl<'de> de::Visitor<'de> for IoFilterInfoVisitor {
    type Value = Box<dyn IoFilterInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid IoFilterInfoTrait JSON object with a _typeName field")
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

impl IoFilterInfoTrait for IoFilterInfo {
    fn get_id(&self) -> &str { &self.id }
    fn get_name(&self) -> &str { &self.name }
    fn get_vendor(&self) -> &str { &self.vendor }
    fn get_version(&self) -> &str { &self.version }
    fn get_type(&self) -> &Option<String> { &self.r#type }
    fn get_summary(&self) -> &Option<String> { &self.summary }
    fn get_release_date(&self) -> &Option<String> { &self.release_date }
}
impl IoFilterInfoTrait for ClusterIoFilterInfo {
    fn get_id(&self) -> &str { &self.id }
    fn get_name(&self) -> &str { &self.name }
    fn get_vendor(&self) -> &str { &self.vendor }
    fn get_version(&self) -> &str { &self.version }
    fn get_type(&self) -> &Option<String> { &self.r#type }
    fn get_summary(&self) -> &Option<String> { &self.summary }
    fn get_release_date(&self) -> &Option<String> { &self.release_date }
}
impl IoFilterInfoTrait for HostIoFilterInfo {
    fn get_id(&self) -> &str { &self.id }
    fn get_name(&self) -> &str { &self.name }
    fn get_vendor(&self) -> &str { &self.vendor }
    fn get_version(&self) -> &str { &self.version }
    fn get_type(&self) -> &Option<String> { &self.r#type }
    fn get_summary(&self) -> &Option<String> { &self.summary }
    fn get_release_date(&self) -> &Option<String> { &self.release_date }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn IoFilterInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::IoFilterInfo => Some(from.as_any_ref().downcast_ref::<IoFilterInfo>()?),
            StructType::ClusterIoFilterInfo => Some(from.as_any_ref().downcast_ref::<ClusterIoFilterInfo>()?),
            StructType::HostIoFilterInfo => Some(from.as_any_ref().downcast_ref::<HostIoFilterInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::IoFilterInfo => Ok(from.as_any_box().downcast::<IoFilterInfo>()?),
            StructType::ClusterIoFilterInfo => Ok(from.as_any_box().downcast::<ClusterIoFilterInfo>()?),
            StructType::HostIoFilterInfo => Ok(from.as_any_box().downcast::<HostIoFilterInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
