use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *DvsVNicProfile* data object is the base object
/// for host and service console Virtual NIC subprofiles.
/// 
/// If a profile plug-in defines additional policies or subprofiles, use the
/// *ApplyProfile.policy* or *ApplyProfile.property*
/// list to access the configuration data.
pub trait DvsVNicProfileTrait : super::apply_profile_trait::ApplyProfileTrait {
    /// Linkable identifier.
    fn get_key(&self) -> &str;
    /// IP address for the Virtual NIC belonging to a distributed virtual switch.
    fn get_ip_config(&self) -> &IpAddressProfile;
}
impl<'s> serde::Serialize for dyn DvsVNicProfileTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DvsVNicProfileTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DvsVNicProfileVisitor)
            }
        }

struct DvsVNicProfileVisitor;

impl<'de> de::Visitor<'de> for DvsVNicProfileVisitor {
    type Value = Box<dyn DvsVNicProfileTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DvsVNicProfileTrait JSON object with a _typeName field")
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

impl DvsVNicProfileTrait for DvsVNicProfile {
    fn get_key(&self) -> &str { &self.key }
    fn get_ip_config(&self) -> &IpAddressProfile { &self.ip_config }
}
impl DvsVNicProfileTrait for DvsHostVNicProfile {
    fn get_key(&self) -> &str { &self.key }
    fn get_ip_config(&self) -> &IpAddressProfile { &self.ip_config }
}
impl DvsVNicProfileTrait for DvsServiceConsoleVNicProfile {
    fn get_key(&self) -> &str { &self.key }
    fn get_ip_config(&self) -> &IpAddressProfile { &self.ip_config }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DvsVNicProfileTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsVNicProfile => Some(from.as_any_ref().downcast_ref::<DvsVNicProfile>()?),
            StructType::DvsHostVNicProfile => Some(from.as_any_ref().downcast_ref::<DvsHostVNicProfile>()?),
            StructType::DvsServiceConsoleVNicProfile => Some(from.as_any_ref().downcast_ref::<DvsServiceConsoleVNicProfile>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsVNicProfile => Ok(from.as_any_box().downcast::<DvsVNicProfile>()?),
            StructType::DvsHostVNicProfile => Ok(from.as_any_box().downcast::<DvsHostVNicProfile>()?),
            StructType::DvsServiceConsoleVNicProfile => Ok(from.as_any_box().downcast::<DvsServiceConsoleVNicProfile>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
