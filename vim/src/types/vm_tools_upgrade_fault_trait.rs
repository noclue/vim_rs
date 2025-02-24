use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A base fault to indicate that something went wrong when upgrading tools.
pub trait VmToolsUpgradeFaultTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn VmToolsUpgradeFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VmToolsUpgradeFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VmToolsUpgradeFaultVisitor)
            }
        }

struct VmToolsUpgradeFaultVisitor;

impl<'de> de::Visitor<'de> for VmToolsUpgradeFaultVisitor {
    type Value = Box<dyn VmToolsUpgradeFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VmToolsUpgradeFaultTrait JSON object with a _typeName field")
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

impl VmToolsUpgradeFaultTrait for VmToolsUpgradeFault {
}
impl VmToolsUpgradeFaultTrait for ToolsAlreadyUpgraded {
}
impl VmToolsUpgradeFaultTrait for ToolsAutoUpgradeNotSupported {
}
impl VmToolsUpgradeFaultTrait for ToolsImageCopyFailed {
}
impl VmToolsUpgradeFaultTrait for ToolsImageNotAvailable {
}
impl VmToolsUpgradeFaultTrait for ToolsImageSignatureCheckFailed {
}
impl VmToolsUpgradeFaultTrait for ToolsUpgradeCancelled {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VmToolsUpgradeFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmToolsUpgradeFault => Some(from.as_any_ref().downcast_ref::<VmToolsUpgradeFault>()?),
            StructType::ToolsAlreadyUpgraded => Some(from.as_any_ref().downcast_ref::<ToolsAlreadyUpgraded>()?),
            StructType::ToolsAutoUpgradeNotSupported => Some(from.as_any_ref().downcast_ref::<ToolsAutoUpgradeNotSupported>()?),
            StructType::ToolsImageCopyFailed => Some(from.as_any_ref().downcast_ref::<ToolsImageCopyFailed>()?),
            StructType::ToolsImageNotAvailable => Some(from.as_any_ref().downcast_ref::<ToolsImageNotAvailable>()?),
            StructType::ToolsImageSignatureCheckFailed => Some(from.as_any_ref().downcast_ref::<ToolsImageSignatureCheckFailed>()?),
            StructType::ToolsUpgradeCancelled => Some(from.as_any_ref().downcast_ref::<ToolsUpgradeCancelled>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmToolsUpgradeFault => Ok(from.as_any_box().downcast::<VmToolsUpgradeFault>()?),
            StructType::ToolsAlreadyUpgraded => Ok(from.as_any_box().downcast::<ToolsAlreadyUpgraded>()?),
            StructType::ToolsAutoUpgradeNotSupported => Ok(from.as_any_box().downcast::<ToolsAutoUpgradeNotSupported>()?),
            StructType::ToolsImageCopyFailed => Ok(from.as_any_box().downcast::<ToolsImageCopyFailed>()?),
            StructType::ToolsImageNotAvailable => Ok(from.as_any_box().downcast::<ToolsImageNotAvailable>()?),
            StructType::ToolsImageSignatureCheckFailed => Ok(from.as_any_box().downcast::<ToolsImageSignatureCheckFailed>()?),
            StructType::ToolsUpgradeCancelled => Ok(from.as_any_box().downcast::<ToolsUpgradeCancelled>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
