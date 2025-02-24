use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This class is the base class for network rule action.
pub trait DvsNetworkRuleActionTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn DvsNetworkRuleActionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DvsNetworkRuleActionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DvsNetworkRuleActionVisitor)
            }
        }

struct DvsNetworkRuleActionVisitor;

impl<'de> de::Visitor<'de> for DvsNetworkRuleActionVisitor {
    type Value = Box<dyn DvsNetworkRuleActionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DvsNetworkRuleActionTrait JSON object with a _typeName field")
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

impl DvsNetworkRuleActionTrait for DvsNetworkRuleAction {
}
impl DvsNetworkRuleActionTrait for DvsAcceptNetworkRuleAction {
}
impl DvsNetworkRuleActionTrait for DvsCopyNetworkRuleAction {
}
impl DvsNetworkRuleActionTrait for DvsDropNetworkRuleAction {
}
impl DvsNetworkRuleActionTrait for DvsGreEncapNetworkRuleAction {
}
impl DvsNetworkRuleActionTrait for DvsLogNetworkRuleAction {
}
impl DvsNetworkRuleActionTrait for DvsMacRewriteNetworkRuleAction {
}
impl DvsNetworkRuleActionTrait for DvsPuntNetworkRuleAction {
}
impl DvsNetworkRuleActionTrait for DvsRateLimitNetworkRuleAction {
}
impl DvsNetworkRuleActionTrait for DvsUpdateTagNetworkRuleAction {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DvsNetworkRuleActionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsNetworkRuleAction => Some(from.as_any_ref().downcast_ref::<DvsNetworkRuleAction>()?),
            StructType::DvsAcceptNetworkRuleAction => Some(from.as_any_ref().downcast_ref::<DvsAcceptNetworkRuleAction>()?),
            StructType::DvsCopyNetworkRuleAction => Some(from.as_any_ref().downcast_ref::<DvsCopyNetworkRuleAction>()?),
            StructType::DvsDropNetworkRuleAction => Some(from.as_any_ref().downcast_ref::<DvsDropNetworkRuleAction>()?),
            StructType::DvsGreEncapNetworkRuleAction => Some(from.as_any_ref().downcast_ref::<DvsGreEncapNetworkRuleAction>()?),
            StructType::DvsLogNetworkRuleAction => Some(from.as_any_ref().downcast_ref::<DvsLogNetworkRuleAction>()?),
            StructType::DvsMacRewriteNetworkRuleAction => Some(from.as_any_ref().downcast_ref::<DvsMacRewriteNetworkRuleAction>()?),
            StructType::DvsPuntNetworkRuleAction => Some(from.as_any_ref().downcast_ref::<DvsPuntNetworkRuleAction>()?),
            StructType::DvsRateLimitNetworkRuleAction => Some(from.as_any_ref().downcast_ref::<DvsRateLimitNetworkRuleAction>()?),
            StructType::DvsUpdateTagNetworkRuleAction => Some(from.as_any_ref().downcast_ref::<DvsUpdateTagNetworkRuleAction>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsNetworkRuleAction => Ok(from.as_any_box().downcast::<DvsNetworkRuleAction>()?),
            StructType::DvsAcceptNetworkRuleAction => Ok(from.as_any_box().downcast::<DvsAcceptNetworkRuleAction>()?),
            StructType::DvsCopyNetworkRuleAction => Ok(from.as_any_box().downcast::<DvsCopyNetworkRuleAction>()?),
            StructType::DvsDropNetworkRuleAction => Ok(from.as_any_box().downcast::<DvsDropNetworkRuleAction>()?),
            StructType::DvsGreEncapNetworkRuleAction => Ok(from.as_any_box().downcast::<DvsGreEncapNetworkRuleAction>()?),
            StructType::DvsLogNetworkRuleAction => Ok(from.as_any_box().downcast::<DvsLogNetworkRuleAction>()?),
            StructType::DvsMacRewriteNetworkRuleAction => Ok(from.as_any_box().downcast::<DvsMacRewriteNetworkRuleAction>()?),
            StructType::DvsPuntNetworkRuleAction => Ok(from.as_any_box().downcast::<DvsPuntNetworkRuleAction>()?),
            StructType::DvsRateLimitNetworkRuleAction => Ok(from.as_any_box().downcast::<DvsRateLimitNetworkRuleAction>()?),
            StructType::DvsUpdateTagNetworkRuleAction => Ok(from.as_any_box().downcast::<DvsUpdateTagNetworkRuleAction>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
