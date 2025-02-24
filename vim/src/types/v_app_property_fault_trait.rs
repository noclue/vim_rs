use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The base fault for all vApp property configuration issues
pub trait VAppPropertyFaultTrait : super::vm_config_fault_trait::VmConfigFaultTrait {
    /// The fully-qualified id of the property, including instance and class
    /// identifiers.
    fn get_id(&self) -> &str;
    /// The user-readable category
    fn get_category(&self) -> &str;
    /// The user-readable label
    fn get_label(&self) -> &str;
    /// The type specified for the property
    fn get_type(&self) -> &str;
    /// The value of the property
    fn get_value(&self) -> &str;
}
impl<'s> serde::Serialize for dyn VAppPropertyFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VAppPropertyFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VAppPropertyFaultVisitor)
            }
        }

struct VAppPropertyFaultVisitor;

impl<'de> de::Visitor<'de> for VAppPropertyFaultVisitor {
    type Value = Box<dyn VAppPropertyFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VAppPropertyFaultTrait JSON object with a _typeName field")
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

impl VAppPropertyFaultTrait for VAppPropertyFault {
    fn get_id(&self) -> &str { &self.id }
    fn get_category(&self) -> &str { &self.category }
    fn get_label(&self) -> &str { &self.label }
    fn get_type(&self) -> &str { &self.r#type }
    fn get_value(&self) -> &str { &self.value }
}
impl VAppPropertyFaultTrait for InvalidNetworkInType {
    fn get_id(&self) -> &str { &self.id }
    fn get_category(&self) -> &str { &self.category }
    fn get_label(&self) -> &str { &self.label }
    fn get_type(&self) -> &str { &self.r#type }
    fn get_value(&self) -> &str { &self.value }
}
impl VAppPropertyFaultTrait for InvalidPropertyType {
    fn get_id(&self) -> &str { &self.id }
    fn get_category(&self) -> &str { &self.category }
    fn get_label(&self) -> &str { &self.label }
    fn get_type(&self) -> &str { &self.r#type }
    fn get_value(&self) -> &str { &self.value }
}
impl VAppPropertyFaultTrait for InvalidPropertyValue {
    fn get_id(&self) -> &str { &self.id }
    fn get_category(&self) -> &str { &self.category }
    fn get_label(&self) -> &str { &self.label }
    fn get_type(&self) -> &str { &self.r#type }
    fn get_value(&self) -> &str { &self.value }
}
impl VAppPropertyFaultTrait for UnconfiguredPropertyValue {
    fn get_id(&self) -> &str { &self.id }
    fn get_category(&self) -> &str { &self.category }
    fn get_label(&self) -> &str { &self.label }
    fn get_type(&self) -> &str { &self.r#type }
    fn get_value(&self) -> &str { &self.value }
}
impl VAppPropertyFaultTrait for MissingIpPool {
    fn get_id(&self) -> &str { &self.id }
    fn get_category(&self) -> &str { &self.category }
    fn get_label(&self) -> &str { &self.label }
    fn get_type(&self) -> &str { &self.r#type }
    fn get_value(&self) -> &str { &self.value }
}
impl VAppPropertyFaultTrait for MissingNetworkIpConfig {
    fn get_id(&self) -> &str { &self.id }
    fn get_category(&self) -> &str { &self.category }
    fn get_label(&self) -> &str { &self.label }
    fn get_type(&self) -> &str { &self.r#type }
    fn get_value(&self) -> &str { &self.value }
}
impl VAppPropertyFaultTrait for NoAvailableIp {
    fn get_id(&self) -> &str { &self.id }
    fn get_category(&self) -> &str { &self.category }
    fn get_label(&self) -> &str { &self.label }
    fn get_type(&self) -> &str { &self.r#type }
    fn get_value(&self) -> &str { &self.value }
}
impl VAppPropertyFaultTrait for NoVcManagedIpConfigured {
    fn get_id(&self) -> &str { &self.id }
    fn get_category(&self) -> &str { &self.category }
    fn get_label(&self) -> &str { &self.label }
    fn get_type(&self) -> &str { &self.r#type }
    fn get_value(&self) -> &str { &self.value }
}
impl VAppPropertyFaultTrait for NotUserConfigurableProperty {
    fn get_id(&self) -> &str { &self.id }
    fn get_category(&self) -> &str { &self.category }
    fn get_label(&self) -> &str { &self.label }
    fn get_type(&self) -> &str { &self.r#type }
    fn get_value(&self) -> &str { &self.value }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VAppPropertyFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VAppPropertyFault => Some(from.as_any_ref().downcast_ref::<VAppPropertyFault>()?),
            StructType::InvalidNetworkInType => Some(from.as_any_ref().downcast_ref::<InvalidNetworkInType>()?),
            StructType::InvalidPropertyType => Some(from.as_any_ref().downcast_ref::<InvalidPropertyType>()?),
            StructType::InvalidPropertyValue => Some(from.as_any_ref().downcast_ref::<InvalidPropertyValue>()?),
            StructType::UnconfiguredPropertyValue => Some(from.as_any_ref().downcast_ref::<UnconfiguredPropertyValue>()?),
            StructType::MissingIpPool => Some(from.as_any_ref().downcast_ref::<MissingIpPool>()?),
            StructType::MissingNetworkIpConfig => Some(from.as_any_ref().downcast_ref::<MissingNetworkIpConfig>()?),
            StructType::NoAvailableIp => Some(from.as_any_ref().downcast_ref::<NoAvailableIp>()?),
            StructType::NoVcManagedIpConfigured => Some(from.as_any_ref().downcast_ref::<NoVcManagedIpConfigured>()?),
            StructType::NotUserConfigurableProperty => Some(from.as_any_ref().downcast_ref::<NotUserConfigurableProperty>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VAppPropertyFault => Ok(from.as_any_box().downcast::<VAppPropertyFault>()?),
            StructType::InvalidNetworkInType => Ok(from.as_any_box().downcast::<InvalidNetworkInType>()?),
            StructType::InvalidPropertyType => Ok(from.as_any_box().downcast::<InvalidPropertyType>()?),
            StructType::InvalidPropertyValue => Ok(from.as_any_box().downcast::<InvalidPropertyValue>()?),
            StructType::UnconfiguredPropertyValue => Ok(from.as_any_box().downcast::<UnconfiguredPropertyValue>()?),
            StructType::MissingIpPool => Ok(from.as_any_box().downcast::<MissingIpPool>()?),
            StructType::MissingNetworkIpConfig => Ok(from.as_any_box().downcast::<MissingNetworkIpConfig>()?),
            StructType::NoAvailableIp => Ok(from.as_any_box().downcast::<NoAvailableIp>()?),
            StructType::NoVcManagedIpConfigured => Ok(from.as_any_box().downcast::<NoVcManagedIpConfigured>()?),
            StructType::NotUserConfigurableProperty => Ok(from.as_any_box().downcast::<NotUserConfigurableProperty>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
