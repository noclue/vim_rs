use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Fault thrown for the case that an attempt is made to move a host which
/// is enabled for VSAN into an unsuitable *ClusterComputeResource*.
/// 
/// The destination vim.ClusterComputeResource may be disabled for VSAN, or
/// may be using VSAN with a different cluster UUID.
/// 
/// See also *ClusterComputeResource.AddHost_Task*, *ClusterComputeResource.MoveHostInto_Task*, *ClusterComputeResource.MoveInto_Task*, *VsanClusterUuidMismatch*, *DestinationVsanDisabled*.
pub trait CannotMoveVsanEnabledHostTrait : super::vsan_fault_trait::VsanFaultTrait {
}
impl<'s> serde::Serialize for dyn CannotMoveVsanEnabledHostTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn CannotMoveVsanEnabledHostTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(CannotMoveVsanEnabledHostVisitor)
            }
        }

struct CannotMoveVsanEnabledHostVisitor;

impl<'de> de::Visitor<'de> for CannotMoveVsanEnabledHostVisitor {
    type Value = Box<dyn CannotMoveVsanEnabledHostTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid CannotMoveVsanEnabledHostTrait JSON object with a _typeName field")
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

impl CannotMoveVsanEnabledHostTrait for CannotMoveVsanEnabledHost {
}
impl CannotMoveVsanEnabledHostTrait for DestinationVsanDisabled {
}
impl CannotMoveVsanEnabledHostTrait for VsanClusterUuidMismatch {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn CannotMoveVsanEnabledHostTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::CannotMoveVsanEnabledHost => Some(from.as_any_ref().downcast_ref::<CannotMoveVsanEnabledHost>()?),
            StructType::DestinationVsanDisabled => Some(from.as_any_ref().downcast_ref::<DestinationVsanDisabled>()?),
            StructType::VsanClusterUuidMismatch => Some(from.as_any_ref().downcast_ref::<VsanClusterUuidMismatch>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::CannotMoveVsanEnabledHost => Ok(from.as_any_box().downcast::<CannotMoveVsanEnabledHost>()?),
            StructType::DestinationVsanDisabled => Ok(from.as_any_box().downcast::<DestinationVsanDisabled>()?),
            StructType::VsanClusterUuidMismatch => Ok(from.as_any_box().downcast::<VsanClusterUuidMismatch>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
