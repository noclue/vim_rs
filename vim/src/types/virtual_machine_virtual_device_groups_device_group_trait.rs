use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base device group type.
pub trait VirtualMachineVirtualDeviceGroupsDeviceGroupTrait : super::data_object_trait::DataObjectTrait {
    /// Group instance key.
    /// 
    /// Unique integer referencing
    /// device group. During group creation client should
    /// use a temporary negative number. Once group is
    /// added to the virtual machine, server generates non-negative
    /// integer that stays constant during group lifetime.
    /// See *VirtualDevice.key* for details.
    fn get_group_instance_key(&self) -> i32;
    /// Provides a label and summary information for the device.
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>>;
}
impl<'s> serde::Serialize for dyn VirtualMachineVirtualDeviceGroupsDeviceGroupTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualMachineVirtualDeviceGroupsDeviceGroupTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualMachineVirtualDeviceGroupsDeviceGroupVisitor)
            }
        }

struct VirtualMachineVirtualDeviceGroupsDeviceGroupVisitor;

impl<'de> de::Visitor<'de> for VirtualMachineVirtualDeviceGroupsDeviceGroupVisitor {
    type Value = Box<dyn VirtualMachineVirtualDeviceGroupsDeviceGroupTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualMachineVirtualDeviceGroupsDeviceGroupTrait JSON object with a _typeName field")
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

impl VirtualMachineVirtualDeviceGroupsDeviceGroupTrait for VirtualMachineVirtualDeviceGroupsDeviceGroup {
    fn get_group_instance_key(&self) -> i32 { self.group_instance_key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
}
impl VirtualMachineVirtualDeviceGroupsDeviceGroupTrait for VirtualMachineVirtualDeviceGroupsVendorDeviceGroup {
    fn get_group_instance_key(&self) -> i32 { self.group_instance_key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualMachineVirtualDeviceGroupsDeviceGroupTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualMachineVirtualDeviceGroupsDeviceGroup => Some(from.as_any_ref().downcast_ref::<VirtualMachineVirtualDeviceGroupsDeviceGroup>()?),
            StructType::VirtualMachineVirtualDeviceGroupsVendorDeviceGroup => Some(from.as_any_ref().downcast_ref::<VirtualMachineVirtualDeviceGroupsVendorDeviceGroup>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualMachineVirtualDeviceGroupsDeviceGroup => Ok(from.as_any_box().downcast::<VirtualMachineVirtualDeviceGroupsDeviceGroup>()?),
            StructType::VirtualMachineVirtualDeviceGroupsVendorDeviceGroup => Ok(from.as_any_box().downcast::<VirtualMachineVirtualDeviceGroupsVendorDeviceGroup>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
