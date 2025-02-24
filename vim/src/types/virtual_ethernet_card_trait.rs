use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *VirtualEthernetCard* data object contains the properties
/// of an Ethernet adapter attached to a virtual machine.
pub trait VirtualEthernetCardTrait : super::virtual_device_trait::VirtualDeviceTrait {
    /// Set of dynamic properties.
    /// 
    /// This property is optional because only the
    /// properties of an object that are unknown to a client will be part of this set.
    /// This property is not readonly just in case we want to send such properties
    /// from a client in the future.
    fn get_dynamic_property(&self) -> &Option<Vec<DynamicProperty>>;
    /// MAC address type.
    /// 
    /// Valid values for address type are:
    /// <dl>
    /// <dt>Manual</dt>
    /// <dd>Statically assigned MAC address.</dd>
    /// <dt>Generated</dt>
    /// <dd>Automatically generated MAC address.</dd>
    /// <dt>Assigned</dt>
    /// <dd>MAC address assigned by VirtualCenter.</dd>
    /// </dl>
    fn get_address_type(&self) -> &Option<String>;
    /// MAC address assigned to the virtual network adapter.
    /// 
    /// Clients can
    /// set this property to any of the allowed address types. The server might
    /// override the specified value for "Generated" or "Assigned" if it does not
    /// fall in the right ranges or is determined to be a duplicate.
    fn get_mac_address(&self) -> &Option<String>;
    /// Indicates whether wake-on-LAN is enabled on this virtual network adapter.
    /// 
    /// Clients
    /// can set this property to selectively enable or disable wake-on-LAN.
    fn get_wake_on_lan_enabled(&self) -> Option<bool>;
    /// Resource requirements of the virtual network adapter
    fn get_resource_allocation(&self) -> &Option<VirtualEthernetCardResourceAllocation>;
    /// An ID assigned to the virtual network adapter by external management plane or
    /// controller.
    /// 
    /// The value and format of this property is determined by external
    /// management plane or controller, and vSphere doesn't do any validation. It's
    /// also up to external management plane or controller to set, unset or maintain
    /// this property. Setting this property with an empty string value will unset the
    /// property.
    fn get_external_id(&self) -> &Option<String>;
    /// Deprecated as of vSphere API 8.0. VMDirectPath Gen 2 is no longer supported and
    /// there is no replacement.
    /// 
    /// Indicates whether UPT(Universal Pass-through) compatibility is enabled
    /// on this network adapter.
    /// 
    /// UPT is only compatible for Vmxnet3 adapter.
    /// Clients can set this property enabled or disabled if ethernet
    /// virtual device is Vmxnet3.
    fn get_upt_compatibility_enabled(&self) -> Option<bool>;
}
impl<'s> serde::Serialize for dyn VirtualEthernetCardTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualEthernetCardTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualEthernetCardVisitor)
            }
        }

struct VirtualEthernetCardVisitor;

impl<'de> de::Visitor<'de> for VirtualEthernetCardVisitor {
    type Value = Box<dyn VirtualEthernetCardTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualEthernetCardTrait JSON object with a _typeName field")
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

impl VirtualEthernetCardTrait for VirtualEthernetCard {
    fn get_dynamic_property(&self) -> &Option<Vec<DynamicProperty>> { &self.dynamic_property }
    fn get_address_type(&self) -> &Option<String> { &self.address_type }
    fn get_mac_address(&self) -> &Option<String> { &self.mac_address }
    fn get_wake_on_lan_enabled(&self) -> Option<bool> { self.wake_on_lan_enabled }
    fn get_resource_allocation(&self) -> &Option<VirtualEthernetCardResourceAllocation> { &self.resource_allocation }
    fn get_external_id(&self) -> &Option<String> { &self.external_id }
    fn get_upt_compatibility_enabled(&self) -> Option<bool> { self.upt_compatibility_enabled }
}
impl VirtualEthernetCardTrait for VirtualE1000 {
    fn get_dynamic_property(&self) -> &Option<Vec<DynamicProperty>> { &self.dynamic_property }
    fn get_address_type(&self) -> &Option<String> { &self.address_type }
    fn get_mac_address(&self) -> &Option<String> { &self.mac_address }
    fn get_wake_on_lan_enabled(&self) -> Option<bool> { self.wake_on_lan_enabled }
    fn get_resource_allocation(&self) -> &Option<VirtualEthernetCardResourceAllocation> { &self.resource_allocation }
    fn get_external_id(&self) -> &Option<String> { &self.external_id }
    fn get_upt_compatibility_enabled(&self) -> Option<bool> { self.upt_compatibility_enabled }
}
impl VirtualEthernetCardTrait for VirtualE1000E {
    fn get_dynamic_property(&self) -> &Option<Vec<DynamicProperty>> { &self.dynamic_property }
    fn get_address_type(&self) -> &Option<String> { &self.address_type }
    fn get_mac_address(&self) -> &Option<String> { &self.mac_address }
    fn get_wake_on_lan_enabled(&self) -> Option<bool> { self.wake_on_lan_enabled }
    fn get_resource_allocation(&self) -> &Option<VirtualEthernetCardResourceAllocation> { &self.resource_allocation }
    fn get_external_id(&self) -> &Option<String> { &self.external_id }
    fn get_upt_compatibility_enabled(&self) -> Option<bool> { self.upt_compatibility_enabled }
}
impl VirtualEthernetCardTrait for VirtualPcNet32 {
    fn get_dynamic_property(&self) -> &Option<Vec<DynamicProperty>> { &self.dynamic_property }
    fn get_address_type(&self) -> &Option<String> { &self.address_type }
    fn get_mac_address(&self) -> &Option<String> { &self.mac_address }
    fn get_wake_on_lan_enabled(&self) -> Option<bool> { self.wake_on_lan_enabled }
    fn get_resource_allocation(&self) -> &Option<VirtualEthernetCardResourceAllocation> { &self.resource_allocation }
    fn get_external_id(&self) -> &Option<String> { &self.external_id }
    fn get_upt_compatibility_enabled(&self) -> Option<bool> { self.upt_compatibility_enabled }
}
impl VirtualEthernetCardTrait for VirtualSriovEthernetCard {
    fn get_dynamic_property(&self) -> &Option<Vec<DynamicProperty>> { &self.dynamic_property }
    fn get_address_type(&self) -> &Option<String> { &self.address_type }
    fn get_mac_address(&self) -> &Option<String> { &self.mac_address }
    fn get_wake_on_lan_enabled(&self) -> Option<bool> { self.wake_on_lan_enabled }
    fn get_resource_allocation(&self) -> &Option<VirtualEthernetCardResourceAllocation> { &self.resource_allocation }
    fn get_external_id(&self) -> &Option<String> { &self.external_id }
    fn get_upt_compatibility_enabled(&self) -> Option<bool> { self.upt_compatibility_enabled }
}
impl VirtualEthernetCardTrait for VirtualVmxnet {
    fn get_dynamic_property(&self) -> &Option<Vec<DynamicProperty>> { &self.dynamic_property }
    fn get_address_type(&self) -> &Option<String> { &self.address_type }
    fn get_mac_address(&self) -> &Option<String> { &self.mac_address }
    fn get_wake_on_lan_enabled(&self) -> Option<bool> { self.wake_on_lan_enabled }
    fn get_resource_allocation(&self) -> &Option<VirtualEthernetCardResourceAllocation> { &self.resource_allocation }
    fn get_external_id(&self) -> &Option<String> { &self.external_id }
    fn get_upt_compatibility_enabled(&self) -> Option<bool> { self.upt_compatibility_enabled }
}
impl VirtualEthernetCardTrait for VirtualVmxnet2 {
    fn get_dynamic_property(&self) -> &Option<Vec<DynamicProperty>> { &self.dynamic_property }
    fn get_address_type(&self) -> &Option<String> { &self.address_type }
    fn get_mac_address(&self) -> &Option<String> { &self.mac_address }
    fn get_wake_on_lan_enabled(&self) -> Option<bool> { self.wake_on_lan_enabled }
    fn get_resource_allocation(&self) -> &Option<VirtualEthernetCardResourceAllocation> { &self.resource_allocation }
    fn get_external_id(&self) -> &Option<String> { &self.external_id }
    fn get_upt_compatibility_enabled(&self) -> Option<bool> { self.upt_compatibility_enabled }
}
impl VirtualEthernetCardTrait for VirtualVmxnet3 {
    fn get_dynamic_property(&self) -> &Option<Vec<DynamicProperty>> { &self.dynamic_property }
    fn get_address_type(&self) -> &Option<String> { &self.address_type }
    fn get_mac_address(&self) -> &Option<String> { &self.mac_address }
    fn get_wake_on_lan_enabled(&self) -> Option<bool> { self.wake_on_lan_enabled }
    fn get_resource_allocation(&self) -> &Option<VirtualEthernetCardResourceAllocation> { &self.resource_allocation }
    fn get_external_id(&self) -> &Option<String> { &self.external_id }
    fn get_upt_compatibility_enabled(&self) -> Option<bool> { self.upt_compatibility_enabled }
}
impl VirtualEthernetCardTrait for VirtualVmxnet3Vrdma {
    fn get_dynamic_property(&self) -> &Option<Vec<DynamicProperty>> { &self.dynamic_property }
    fn get_address_type(&self) -> &Option<String> { &self.address_type }
    fn get_mac_address(&self) -> &Option<String> { &self.mac_address }
    fn get_wake_on_lan_enabled(&self) -> Option<bool> { self.wake_on_lan_enabled }
    fn get_resource_allocation(&self) -> &Option<VirtualEthernetCardResourceAllocation> { &self.resource_allocation }
    fn get_external_id(&self) -> &Option<String> { &self.external_id }
    fn get_upt_compatibility_enabled(&self) -> Option<bool> { self.upt_compatibility_enabled }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualEthernetCardTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualEthernetCard => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCard>()?),
            StructType::VirtualE1000 => Some(from.as_any_ref().downcast_ref::<VirtualE1000>()?),
            StructType::VirtualE1000E => Some(from.as_any_ref().downcast_ref::<VirtualE1000E>()?),
            StructType::VirtualPcNet32 => Some(from.as_any_ref().downcast_ref::<VirtualPcNet32>()?),
            StructType::VirtualSriovEthernetCard => Some(from.as_any_ref().downcast_ref::<VirtualSriovEthernetCard>()?),
            StructType::VirtualVmxnet => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet>()?),
            StructType::VirtualVmxnet2 => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet2>()?),
            StructType::VirtualVmxnet3 => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet3>()?),
            StructType::VirtualVmxnet3Vrdma => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet3Vrdma>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualEthernetCard => Ok(from.as_any_box().downcast::<VirtualEthernetCard>()?),
            StructType::VirtualE1000 => Ok(from.as_any_box().downcast::<VirtualE1000>()?),
            StructType::VirtualE1000E => Ok(from.as_any_box().downcast::<VirtualE1000E>()?),
            StructType::VirtualPcNet32 => Ok(from.as_any_box().downcast::<VirtualPcNet32>()?),
            StructType::VirtualSriovEthernetCard => Ok(from.as_any_box().downcast::<VirtualSriovEthernetCard>()?),
            StructType::VirtualVmxnet => Ok(from.as_any_box().downcast::<VirtualVmxnet>()?),
            StructType::VirtualVmxnet2 => Ok(from.as_any_box().downcast::<VirtualVmxnet2>()?),
            StructType::VirtualVmxnet3 => Ok(from.as_any_box().downcast::<VirtualVmxnet3>()?),
            StructType::VirtualVmxnet3Vrdma => Ok(from.as_any_box().downcast::<VirtualVmxnet3Vrdma>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
