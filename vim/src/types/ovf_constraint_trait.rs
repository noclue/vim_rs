use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A base fault for Ovf descriptor constraints
pub trait OvfConstraintTrait : super::ovf_invalid_package_trait::OvfInvalidPackageTrait {
    /// The name of the element
    fn get_name(&self) -> &str;
}
impl<'s> serde::Serialize for dyn OvfConstraintTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn OvfConstraintTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(OvfConstraintVisitor)
            }
        }

struct OvfConstraintVisitor;

impl<'de> de::Visitor<'de> for OvfConstraintVisitor {
    type Value = Box<dyn OvfConstraintTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid OvfConstraintTrait JSON object with a _typeName field")
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

impl OvfConstraintTrait for OvfConstraint {
    fn get_name(&self) -> &str { &self.name }
}
impl OvfConstraintTrait for OvfDiskOrderConstraint {
    fn get_name(&self) -> &str { &self.name }
}
impl OvfConstraintTrait for OvfHostResourceConstraint {
    fn get_name(&self) -> &str { &self.name }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn OvfConstraintTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfConstraint => Some(from.as_any_ref().downcast_ref::<OvfConstraint>()?),
            StructType::OvfDiskOrderConstraint => Some(from.as_any_ref().downcast_ref::<OvfDiskOrderConstraint>()?),
            StructType::OvfHostResourceConstraint => Some(from.as_any_ref().downcast_ref::<OvfHostResourceConstraint>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::OvfConstraint => Ok(from.as_any_box().downcast::<OvfConstraint>()?),
            StructType::OvfDiskOrderConstraint => Ok(from.as_any_box().downcast::<OvfDiskOrderConstraint>()?),
            StructType::OvfHostResourceConstraint => Ok(from.as_any_box().downcast::<OvfHostResourceConstraint>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
