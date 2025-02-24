use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

pub trait ProfileExpressionTrait : super::data_object_trait::DataObjectTrait {
    /// Identifier of this expression.
    /// 
    /// The id has to be unique within a Profile.
    /// The id can be used as a key while building composite expressions.
    fn get_id(&self) -> &str;
    /// User visible display name
    fn get_display_name(&self) -> &str;
    /// Flag indicating if the condition of the expression should be negated.
    /// 
    /// e.g: conditions like VSwitch0 has vmnic0 connected to it can be turned into
    /// VSwitch0 doesn't have vmnic0 connected to it.
    fn get_negated(&self) -> bool;
}
impl<'s> serde::Serialize for dyn ProfileExpressionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ProfileExpressionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ProfileExpressionVisitor)
            }
        }

struct ProfileExpressionVisitor;

impl<'de> de::Visitor<'de> for ProfileExpressionVisitor {
    type Value = Box<dyn ProfileExpressionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ProfileExpressionTrait JSON object with a _typeName field")
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

impl ProfileExpressionTrait for ProfileExpression {
    fn get_id(&self) -> &str { &self.id }
    fn get_display_name(&self) -> &str { &self.display_name }
    fn get_negated(&self) -> bool { self.negated }
}
impl ProfileExpressionTrait for ProfileCompositeExpression {
    fn get_id(&self) -> &str { &self.id }
    fn get_display_name(&self) -> &str { &self.display_name }
    fn get_negated(&self) -> bool { self.negated }
}
impl ProfileExpressionTrait for ProfileSimpleExpression {
    fn get_id(&self) -> &str { &self.id }
    fn get_display_name(&self) -> &str { &self.display_name }
    fn get_negated(&self) -> bool { self.negated }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ProfileExpressionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ProfileExpression => Some(from.as_any_ref().downcast_ref::<ProfileExpression>()?),
            StructType::ProfileCompositeExpression => Some(from.as_any_ref().downcast_ref::<ProfileCompositeExpression>()?),
            StructType::ProfileSimpleExpression => Some(from.as_any_ref().downcast_ref::<ProfileSimpleExpression>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ProfileExpression => Ok(from.as_any_box().downcast::<ProfileExpression>()?),
            StructType::ProfileCompositeExpression => Ok(from.as_any_box().downcast::<ProfileCompositeExpression>()?),
            StructType::ProfileSimpleExpression => Ok(from.as_any_box().downcast::<ProfileSimpleExpression>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
