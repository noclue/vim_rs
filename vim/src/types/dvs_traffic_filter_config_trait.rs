use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This class defines Traffic Filter configuration.
/// 
/// ** Supported Qualifier and Actions **
/// <table border="1"width="100%">
/// <tr>
/// <th>Traffic Filter Config</th>
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
pub trait DvsTrafficFilterConfigTrait : super::dvs_filter_config_trait::DvsFilterConfigTrait {
    /// Network Traffic Ruleset
    fn get_traffic_ruleset(&self) -> &Option<DvsTrafficRuleset>;
}
impl<'s> serde::Serialize for dyn DvsTrafficFilterConfigTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DvsTrafficFilterConfigTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DvsTrafficFilterConfigVisitor)
            }
        }

struct DvsTrafficFilterConfigVisitor;

impl<'de> de::Visitor<'de> for DvsTrafficFilterConfigVisitor {
    type Value = Box<dyn DvsTrafficFilterConfigTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DvsTrafficFilterConfigTrait JSON object with a _typeName field")
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

impl DvsTrafficFilterConfigTrait for DvsTrafficFilterConfig {
    fn get_traffic_ruleset(&self) -> &Option<DvsTrafficRuleset> { &self.traffic_ruleset }
}
impl DvsTrafficFilterConfigTrait for DvsTrafficFilterConfigSpec {
    fn get_traffic_ruleset(&self) -> &Option<DvsTrafficRuleset> { &self.traffic_ruleset }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DvsTrafficFilterConfigTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsTrafficFilterConfig => Some(from.as_any_ref().downcast_ref::<DvsTrafficFilterConfig>()?),
            StructType::DvsTrafficFilterConfigSpec => Some(from.as_any_ref().downcast_ref::<DvsTrafficFilterConfigSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsTrafficFilterConfig => Ok(from.as_any_box().downcast::<DvsTrafficFilterConfig>()?),
            StructType::DvsTrafficFilterConfigSpec => Ok(from.as_any_box().downcast::<DvsTrafficFilterConfigSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
