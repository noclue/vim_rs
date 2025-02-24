use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This class defines Network Filter configuration.
/// 
/// ** Supported Qualifier and Actions **
/// <table border="1"width="100%">
/// <tr>
/// <th>Network Filter Config</th>
/// <th>Supported classes</th>
/// </tr>
/// <tr>
/// <td>Qualifiers supported</td>
/// <td>*SingleIp*, *IpRange*,
/// *SingleMac*, *MacRange*,
/// *DvsSingleIpPort*,
/// *DvsSystemTrafficNetworkRuleQualifier*
/// </td>
/// </tr>
/// <tr>
/// <td>Actions Supported</td>
/// <td>*DvsDropNetworkRuleAction*,
/// *DvsAcceptNetworkRuleAction*,
/// *DvsPuntNetworkRuleAction*,
/// *DvsCopyNetworkRuleAction*,
/// *DvsMacRewriteNetworkRuleAction*,
/// *DvsGreEncapNetworkRuleAction*,
/// *DvsLogNetworkRuleAction*,
/// *DvsUpdateTagNetworkRuleAction*,
/// *DvsRateLimitNetworkRuleAction*
/// </td>
/// </tr>
pub trait DvsFilterConfigTrait : super::inheritable_policy_trait::InheritablePolicyTrait {
    /// The key of Network Filter Config.
    fn get_key(&self) -> &Option<String>;
    /// The name of the network traffic filter agent.
    fn get_agent_name(&self) -> &Option<String>;
    /// The slot number of the network filter agent.
    fn get_slot_number(&self) -> &Option<String>;
    /// Network Filter Parameter
    fn get_parameters(&self) -> &Option<DvsFilterParameter>;
    /// This property specifies whether to allow all traffic or to deny all
    /// traffic when a Network Filter fails to configure.
    /// 
    /// Please see *DvsFilterOnFailure_enum*
    /// for more details.
    fn get_on_failure(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn DvsFilterConfigTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DvsFilterConfigTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DvsFilterConfigVisitor)
            }
        }

struct DvsFilterConfigVisitor;

impl<'de> de::Visitor<'de> for DvsFilterConfigVisitor {
    type Value = Box<dyn DvsFilterConfigTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DvsFilterConfigTrait JSON object with a _typeName field")
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

impl DvsFilterConfigTrait for DvsFilterConfig {
    fn get_key(&self) -> &Option<String> { &self.key }
    fn get_agent_name(&self) -> &Option<String> { &self.agent_name }
    fn get_slot_number(&self) -> &Option<String> { &self.slot_number }
    fn get_parameters(&self) -> &Option<DvsFilterParameter> { &self.parameters }
    fn get_on_failure(&self) -> &Option<String> { &self.on_failure }
}
impl DvsFilterConfigTrait for DvsFilterConfigSpec {
    fn get_key(&self) -> &Option<String> { &self.key }
    fn get_agent_name(&self) -> &Option<String> { &self.agent_name }
    fn get_slot_number(&self) -> &Option<String> { &self.slot_number }
    fn get_parameters(&self) -> &Option<DvsFilterParameter> { &self.parameters }
    fn get_on_failure(&self) -> &Option<String> { &self.on_failure }
}
impl DvsFilterConfigTrait for DvsTrafficFilterConfig {
    fn get_key(&self) -> &Option<String> { &self.key }
    fn get_agent_name(&self) -> &Option<String> { &self.agent_name }
    fn get_slot_number(&self) -> &Option<String> { &self.slot_number }
    fn get_parameters(&self) -> &Option<DvsFilterParameter> { &self.parameters }
    fn get_on_failure(&self) -> &Option<String> { &self.on_failure }
}
impl DvsFilterConfigTrait for DvsTrafficFilterConfigSpec {
    fn get_key(&self) -> &Option<String> { &self.key }
    fn get_agent_name(&self) -> &Option<String> { &self.agent_name }
    fn get_slot_number(&self) -> &Option<String> { &self.slot_number }
    fn get_parameters(&self) -> &Option<DvsFilterParameter> { &self.parameters }
    fn get_on_failure(&self) -> &Option<String> { &self.on_failure }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DvsFilterConfigTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsFilterConfig => Some(from.as_any_ref().downcast_ref::<DvsFilterConfig>()?),
            StructType::DvsFilterConfigSpec => Some(from.as_any_ref().downcast_ref::<DvsFilterConfigSpec>()?),
            StructType::DvsTrafficFilterConfig => Some(from.as_any_ref().downcast_ref::<DvsTrafficFilterConfig>()?),
            StructType::DvsTrafficFilterConfigSpec => Some(from.as_any_ref().downcast_ref::<DvsTrafficFilterConfigSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsFilterConfig => Ok(from.as_any_box().downcast::<DvsFilterConfig>()?),
            StructType::DvsFilterConfigSpec => Ok(from.as_any_box().downcast::<DvsFilterConfigSpec>()?),
            StructType::DvsTrafficFilterConfig => Ok(from.as_any_box().downcast::<DvsTrafficFilterConfig>()?),
            StructType::DvsTrafficFilterConfigSpec => Ok(from.as_any_box().downcast::<DvsTrafficFilterConfigSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
