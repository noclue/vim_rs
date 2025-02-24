use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A NotEnoughLicensesFault occurs when an operation
/// fails because there are not enough licenses installed.
pub trait NotEnoughLicensesTrait : super::runtime_fault_trait::RuntimeFaultTrait {
}
impl<'s> serde::Serialize for dyn NotEnoughLicensesTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn NotEnoughLicensesTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(NotEnoughLicensesVisitor)
            }
        }

struct NotEnoughLicensesVisitor;

impl<'de> de::Visitor<'de> for NotEnoughLicensesVisitor {
    type Value = Box<dyn NotEnoughLicensesTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid NotEnoughLicensesTrait JSON object with a _typeName field")
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

impl NotEnoughLicensesTrait for NotEnoughLicenses {
}
impl NotEnoughLicensesTrait for ExpiredFeatureLicense {
}
impl NotEnoughLicensesTrait for ExpiredAddonLicense {
}
impl NotEnoughLicensesTrait for ExpiredEditionLicense {
}
impl NotEnoughLicensesTrait for FailToEnableSpbm {
}
impl NotEnoughLicensesTrait for HostInventoryFull {
}
impl NotEnoughLicensesTrait for InUseFeatureManipulationDisallowed {
}
impl NotEnoughLicensesTrait for IncorrectHostInformation {
}
impl NotEnoughLicensesTrait for InvalidEditionLicense {
}
impl NotEnoughLicensesTrait for InventoryHasStandardAloneHosts {
}
impl NotEnoughLicensesTrait for LicenseDowngradeDisallowed {
}
impl NotEnoughLicensesTrait for LicenseExpired {
}
impl NotEnoughLicensesTrait for LicenseKeyEntityMismatch {
}
impl NotEnoughLicensesTrait for LicenseRestricted {
}
impl NotEnoughLicensesTrait for LicenseSourceUnavailable {
}
impl NotEnoughLicensesTrait for NoLicenseServerConfigured {
}
impl NotEnoughLicensesTrait for VmLimitLicense {
}
impl NotEnoughLicensesTrait for VramLimitLicense {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn NotEnoughLicensesTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::NotEnoughLicenses => Some(from.as_any_ref().downcast_ref::<NotEnoughLicenses>()?),
            StructType::ExpiredFeatureLicense => Some(from.as_any_ref().downcast_ref::<ExpiredFeatureLicense>()?),
            StructType::ExpiredAddonLicense => Some(from.as_any_ref().downcast_ref::<ExpiredAddonLicense>()?),
            StructType::ExpiredEditionLicense => Some(from.as_any_ref().downcast_ref::<ExpiredEditionLicense>()?),
            StructType::FailToEnableSpbm => Some(from.as_any_ref().downcast_ref::<FailToEnableSpbm>()?),
            StructType::HostInventoryFull => Some(from.as_any_ref().downcast_ref::<HostInventoryFull>()?),
            StructType::InUseFeatureManipulationDisallowed => Some(from.as_any_ref().downcast_ref::<InUseFeatureManipulationDisallowed>()?),
            StructType::IncorrectHostInformation => Some(from.as_any_ref().downcast_ref::<IncorrectHostInformation>()?),
            StructType::InvalidEditionLicense => Some(from.as_any_ref().downcast_ref::<InvalidEditionLicense>()?),
            StructType::InventoryHasStandardAloneHosts => Some(from.as_any_ref().downcast_ref::<InventoryHasStandardAloneHosts>()?),
            StructType::LicenseDowngradeDisallowed => Some(from.as_any_ref().downcast_ref::<LicenseDowngradeDisallowed>()?),
            StructType::LicenseExpired => Some(from.as_any_ref().downcast_ref::<LicenseExpired>()?),
            StructType::LicenseKeyEntityMismatch => Some(from.as_any_ref().downcast_ref::<LicenseKeyEntityMismatch>()?),
            StructType::LicenseRestricted => Some(from.as_any_ref().downcast_ref::<LicenseRestricted>()?),
            StructType::LicenseSourceUnavailable => Some(from.as_any_ref().downcast_ref::<LicenseSourceUnavailable>()?),
            StructType::NoLicenseServerConfigured => Some(from.as_any_ref().downcast_ref::<NoLicenseServerConfigured>()?),
            StructType::VmLimitLicense => Some(from.as_any_ref().downcast_ref::<VmLimitLicense>()?),
            StructType::VramLimitLicense => Some(from.as_any_ref().downcast_ref::<VramLimitLicense>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::NotEnoughLicenses => Ok(from.as_any_box().downcast::<NotEnoughLicenses>()?),
            StructType::ExpiredFeatureLicense => Ok(from.as_any_box().downcast::<ExpiredFeatureLicense>()?),
            StructType::ExpiredAddonLicense => Ok(from.as_any_box().downcast::<ExpiredAddonLicense>()?),
            StructType::ExpiredEditionLicense => Ok(from.as_any_box().downcast::<ExpiredEditionLicense>()?),
            StructType::FailToEnableSpbm => Ok(from.as_any_box().downcast::<FailToEnableSpbm>()?),
            StructType::HostInventoryFull => Ok(from.as_any_box().downcast::<HostInventoryFull>()?),
            StructType::InUseFeatureManipulationDisallowed => Ok(from.as_any_box().downcast::<InUseFeatureManipulationDisallowed>()?),
            StructType::IncorrectHostInformation => Ok(from.as_any_box().downcast::<IncorrectHostInformation>()?),
            StructType::InvalidEditionLicense => Ok(from.as_any_box().downcast::<InvalidEditionLicense>()?),
            StructType::InventoryHasStandardAloneHosts => Ok(from.as_any_box().downcast::<InventoryHasStandardAloneHosts>()?),
            StructType::LicenseDowngradeDisallowed => Ok(from.as_any_box().downcast::<LicenseDowngradeDisallowed>()?),
            StructType::LicenseExpired => Ok(from.as_any_box().downcast::<LicenseExpired>()?),
            StructType::LicenseKeyEntityMismatch => Ok(from.as_any_box().downcast::<LicenseKeyEntityMismatch>()?),
            StructType::LicenseRestricted => Ok(from.as_any_box().downcast::<LicenseRestricted>()?),
            StructType::LicenseSourceUnavailable => Ok(from.as_any_box().downcast::<LicenseSourceUnavailable>()?),
            StructType::NoLicenseServerConfigured => Ok(from.as_any_box().downcast::<NoLicenseServerConfigured>()?),
            StructType::VmLimitLicense => Ok(from.as_any_box().downcast::<VmLimitLicense>()?),
            StructType::VramLimitLicense => Ok(from.as_any_box().downcast::<VramLimitLicense>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
