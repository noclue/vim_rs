use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base class for filters to check host compatibility.
pub trait DistributedVirtualSwitchManagerHostDvsFilterSpecTrait : super::data_object_trait::DataObjectTrait {
    /// If this flag is true, then the filter returns the hosts in the
    /// *DistributedVirtualSwitchManagerHostContainer*
    /// that satisfy the criteria specified by this filter, otherwise
    /// it returns hosts that don't meet the criteria.
    fn get_inclusive(&self) -> bool;
}
impl<'s> serde::Serialize for dyn DistributedVirtualSwitchManagerHostDvsFilterSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DistributedVirtualSwitchManagerHostDvsFilterSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DistributedVirtualSwitchManagerHostDvsFilterSpecVisitor)
            }
        }

struct DistributedVirtualSwitchManagerHostDvsFilterSpecVisitor;

impl<'de> de::Visitor<'de> for DistributedVirtualSwitchManagerHostDvsFilterSpecVisitor {
    type Value = Box<dyn DistributedVirtualSwitchManagerHostDvsFilterSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DistributedVirtualSwitchManagerHostDvsFilterSpecTrait JSON object with a _typeName field")
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

impl DistributedVirtualSwitchManagerHostDvsFilterSpecTrait for DistributedVirtualSwitchManagerHostDvsFilterSpec {
    fn get_inclusive(&self) -> bool { self.inclusive }
}
impl DistributedVirtualSwitchManagerHostDvsFilterSpecTrait for DistributedVirtualSwitchManagerHostArrayFilter {
    fn get_inclusive(&self) -> bool { self.inclusive }
}
impl DistributedVirtualSwitchManagerHostDvsFilterSpecTrait for DistributedVirtualSwitchManagerHostContainerFilter {
    fn get_inclusive(&self) -> bool { self.inclusive }
}
impl DistributedVirtualSwitchManagerHostDvsFilterSpecTrait for DistributedVirtualSwitchManagerHostDvsMembershipFilter {
    fn get_inclusive(&self) -> bool { self.inclusive }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DistributedVirtualSwitchManagerHostDvsFilterSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DistributedVirtualSwitchManagerHostDvsFilterSpec => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchManagerHostDvsFilterSpec>()?),
            StructType::DistributedVirtualSwitchManagerHostArrayFilter => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchManagerHostArrayFilter>()?),
            StructType::DistributedVirtualSwitchManagerHostContainerFilter => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchManagerHostContainerFilter>()?),
            StructType::DistributedVirtualSwitchManagerHostDvsMembershipFilter => Some(from.as_any_ref().downcast_ref::<DistributedVirtualSwitchManagerHostDvsMembershipFilter>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DistributedVirtualSwitchManagerHostDvsFilterSpec => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchManagerHostDvsFilterSpec>()?),
            StructType::DistributedVirtualSwitchManagerHostArrayFilter => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchManagerHostArrayFilter>()?),
            StructType::DistributedVirtualSwitchManagerHostContainerFilter => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchManagerHostContainerFilter>()?),
            StructType::DistributedVirtualSwitchManagerHostDvsMembershipFilter => Ok(from.as_any_box().downcast::<DistributedVirtualSwitchManagerHostDvsMembershipFilter>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
