use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The base class for any type of setting or configuration that may get a
/// inherited value.
/// 
/// When used in a reconfigure operation specification, if *InheritablePolicy.inherited* is true,
/// it specifies the intention to change the values of subclass's properties to the
/// inherited values from the level above. In this case, users don't need to specify
/// the values and any set property in the subclass will be ignored.
/// if *InheritablePolicy.inherited* is false, it specifies the intention to explicitly set
/// subclass's properties to user specified values. Users should set the properties in
/// the subclass with the desired values.
/// 
/// When used in a configuration information object, The values of the properties in
/// the subclass are the effective values. if *InheritablePolicy.inherited* is true, the object
/// is getting the effective values from upper level. If false, the values are
/// explicitly set by a user.
pub trait InheritablePolicyTrait : super::data_object_trait::DataObjectTrait {
    /// Whether the configuration is set to inherited value.
    fn get_inherited(&self) -> bool;
}
impl<'s> serde::Serialize for dyn InheritablePolicyTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn InheritablePolicyTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(InheritablePolicyVisitor)
            }
        }

struct InheritablePolicyVisitor;

impl<'de> de::Visitor<'de> for InheritablePolicyVisitor {
    type Value = Box<dyn InheritablePolicyTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid InheritablePolicyTrait JSON object with a _typeName field")
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

impl InheritablePolicyTrait for InheritablePolicy {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for BoolPolicy {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for IntPolicy {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for LongPolicy {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for StringPolicy {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for DvsFilterConfig {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for DvsFilterConfigSpec {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for DvsTrafficFilterConfig {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for DvsTrafficFilterConfigSpec {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for DvsFilterPolicy {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for DvsTrafficShapingPolicy {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for DvsVendorSpecificConfig {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for DvsFailureCriteria {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for DvsMacLearningPolicy {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for DvsMacManagementPolicy {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for DvsSecurityPolicy {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for VMwareUplinkLacpPolicy {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for VMwareUplinkPortOrderPolicy {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for VmwareUplinkPortTeamingPolicy {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for VmwareDistributedVirtualSwitchVlanSpec {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for VmwareDistributedVirtualSwitchPvlanSpec {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for VmwareDistributedVirtualSwitchTrunkVlanSpec {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl InheritablePolicyTrait for VmwareDistributedVirtualSwitchVlanIdSpec {
    fn get_inherited(&self) -> bool { self.inherited }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn InheritablePolicyTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::InheritablePolicy => Some(from.as_any_ref().downcast_ref::<InheritablePolicy>()?),
            StructType::BoolPolicy => Some(from.as_any_ref().downcast_ref::<BoolPolicy>()?),
            StructType::IntPolicy => Some(from.as_any_ref().downcast_ref::<IntPolicy>()?),
            StructType::LongPolicy => Some(from.as_any_ref().downcast_ref::<LongPolicy>()?),
            StructType::StringPolicy => Some(from.as_any_ref().downcast_ref::<StringPolicy>()?),
            StructType::DvsFilterConfig => Some(from.as_any_ref().downcast_ref::<DvsFilterConfig>()?),
            StructType::DvsFilterConfigSpec => Some(from.as_any_ref().downcast_ref::<DvsFilterConfigSpec>()?),
            StructType::DvsTrafficFilterConfig => Some(from.as_any_ref().downcast_ref::<DvsTrafficFilterConfig>()?),
            StructType::DvsTrafficFilterConfigSpec => Some(from.as_any_ref().downcast_ref::<DvsTrafficFilterConfigSpec>()?),
            StructType::DvsFilterPolicy => Some(from.as_any_ref().downcast_ref::<DvsFilterPolicy>()?),
            StructType::DvsTrafficShapingPolicy => Some(from.as_any_ref().downcast_ref::<DvsTrafficShapingPolicy>()?),
            StructType::DvsVendorSpecificConfig => Some(from.as_any_ref().downcast_ref::<DvsVendorSpecificConfig>()?),
            StructType::DvsFailureCriteria => Some(from.as_any_ref().downcast_ref::<DvsFailureCriteria>()?),
            StructType::DvsMacLearningPolicy => Some(from.as_any_ref().downcast_ref::<DvsMacLearningPolicy>()?),
            StructType::DvsMacManagementPolicy => Some(from.as_any_ref().downcast_ref::<DvsMacManagementPolicy>()?),
            StructType::DvsSecurityPolicy => Some(from.as_any_ref().downcast_ref::<DvsSecurityPolicy>()?),
            StructType::VMwareUplinkLacpPolicy => Some(from.as_any_ref().downcast_ref::<VMwareUplinkLacpPolicy>()?),
            StructType::VMwareUplinkPortOrderPolicy => Some(from.as_any_ref().downcast_ref::<VMwareUplinkPortOrderPolicy>()?),
            StructType::VmwareUplinkPortTeamingPolicy => Some(from.as_any_ref().downcast_ref::<VmwareUplinkPortTeamingPolicy>()?),
            StructType::VmwareDistributedVirtualSwitchVlanSpec => Some(from.as_any_ref().downcast_ref::<VmwareDistributedVirtualSwitchVlanSpec>()?),
            StructType::VmwareDistributedVirtualSwitchPvlanSpec => Some(from.as_any_ref().downcast_ref::<VmwareDistributedVirtualSwitchPvlanSpec>()?),
            StructType::VmwareDistributedVirtualSwitchTrunkVlanSpec => Some(from.as_any_ref().downcast_ref::<VmwareDistributedVirtualSwitchTrunkVlanSpec>()?),
            StructType::VmwareDistributedVirtualSwitchVlanIdSpec => Some(from.as_any_ref().downcast_ref::<VmwareDistributedVirtualSwitchVlanIdSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::InheritablePolicy => Ok(from.as_any_box().downcast::<InheritablePolicy>()?),
            StructType::BoolPolicy => Ok(from.as_any_box().downcast::<BoolPolicy>()?),
            StructType::IntPolicy => Ok(from.as_any_box().downcast::<IntPolicy>()?),
            StructType::LongPolicy => Ok(from.as_any_box().downcast::<LongPolicy>()?),
            StructType::StringPolicy => Ok(from.as_any_box().downcast::<StringPolicy>()?),
            StructType::DvsFilterConfig => Ok(from.as_any_box().downcast::<DvsFilterConfig>()?),
            StructType::DvsFilterConfigSpec => Ok(from.as_any_box().downcast::<DvsFilterConfigSpec>()?),
            StructType::DvsTrafficFilterConfig => Ok(from.as_any_box().downcast::<DvsTrafficFilterConfig>()?),
            StructType::DvsTrafficFilterConfigSpec => Ok(from.as_any_box().downcast::<DvsTrafficFilterConfigSpec>()?),
            StructType::DvsFilterPolicy => Ok(from.as_any_box().downcast::<DvsFilterPolicy>()?),
            StructType::DvsTrafficShapingPolicy => Ok(from.as_any_box().downcast::<DvsTrafficShapingPolicy>()?),
            StructType::DvsVendorSpecificConfig => Ok(from.as_any_box().downcast::<DvsVendorSpecificConfig>()?),
            StructType::DvsFailureCriteria => Ok(from.as_any_box().downcast::<DvsFailureCriteria>()?),
            StructType::DvsMacLearningPolicy => Ok(from.as_any_box().downcast::<DvsMacLearningPolicy>()?),
            StructType::DvsMacManagementPolicy => Ok(from.as_any_box().downcast::<DvsMacManagementPolicy>()?),
            StructType::DvsSecurityPolicy => Ok(from.as_any_box().downcast::<DvsSecurityPolicy>()?),
            StructType::VMwareUplinkLacpPolicy => Ok(from.as_any_box().downcast::<VMwareUplinkLacpPolicy>()?),
            StructType::VMwareUplinkPortOrderPolicy => Ok(from.as_any_box().downcast::<VMwareUplinkPortOrderPolicy>()?),
            StructType::VmwareUplinkPortTeamingPolicy => Ok(from.as_any_box().downcast::<VmwareUplinkPortTeamingPolicy>()?),
            StructType::VmwareDistributedVirtualSwitchVlanSpec => Ok(from.as_any_box().downcast::<VmwareDistributedVirtualSwitchVlanSpec>()?),
            StructType::VmwareDistributedVirtualSwitchPvlanSpec => Ok(from.as_any_box().downcast::<VmwareDistributedVirtualSwitchPvlanSpec>()?),
            StructType::VmwareDistributedVirtualSwitchTrunkVlanSpec => Ok(from.as_any_box().downcast::<VmwareDistributedVirtualSwitchTrunkVlanSpec>()?),
            StructType::VmwareDistributedVirtualSwitchVlanIdSpec => Ok(from.as_any_box().downcast::<VmwareDistributedVirtualSwitchVlanIdSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
