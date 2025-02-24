use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type contains the options for the
/// virtual ethernet card data object type.
pub trait VirtualEthernetCardOptionTrait : super::virtual_device_option_trait::VirtualDeviceOptionTrait {
    /// The valid Organizational Unique Identifiers (OUIs)
    /// supported by this virtual Ethernet card.
    /// 
    /// <dl>
    /// <dt>Supported OUIs for statically assigned MAC addresses:</dt>
    /// <dd>"00:50:56"</dd>
    /// </dl>
    fn get_supported_oui(&self) -> &ChoiceOption;
    /// The supported MAC address types.
    fn get_mac_type(&self) -> &ChoiceOption;
    /// Flag to indicate whether or not wake-on-LAN is settable on this device.
    fn get_wake_on_lan_enabled(&self) -> &BoolOption;
    /// Deprecated as of vSphere API 8.0. VMDirectPath Gen 2 is no longer supported and
    /// there is no replacement.
    /// 
    /// Flag to indicate whether VMDirectPath Gen 2 is available on this device.
    fn get_vm_direct_path_gen_2_supported(&self) -> Option<bool>;
    /// Deprecated as of vSphere API 8.0. VMDirectPath Gen 2 is no longer supported and
    /// there is no replacement.
    /// 
    /// Flag to indicate whether Universal Pass-through(UPT) is settable on this device.
    fn get_upt_compatibility_enabled(&self) -> &Option<BoolOption>;
}
impl<'s> serde::Serialize for dyn VirtualEthernetCardOptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualEthernetCardOptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualEthernetCardOptionVisitor)
            }
        }

struct VirtualEthernetCardOptionVisitor;

impl<'de> de::Visitor<'de> for VirtualEthernetCardOptionVisitor {
    type Value = Box<dyn VirtualEthernetCardOptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualEthernetCardOptionTrait JSON object with a _typeName field")
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

impl VirtualEthernetCardOptionTrait for VirtualEthernetCardOption {
    fn get_supported_oui(&self) -> &ChoiceOption { &self.supported_oui }
    fn get_mac_type(&self) -> &ChoiceOption { &self.mac_type }
    fn get_wake_on_lan_enabled(&self) -> &BoolOption { &self.wake_on_lan_enabled }
    fn get_vm_direct_path_gen_2_supported(&self) -> Option<bool> { self.vm_direct_path_gen_2_supported }
    fn get_upt_compatibility_enabled(&self) -> &Option<BoolOption> { &self.upt_compatibility_enabled }
}
impl VirtualEthernetCardOptionTrait for VirtualE1000Option {
    fn get_supported_oui(&self) -> &ChoiceOption { &self.supported_oui }
    fn get_mac_type(&self) -> &ChoiceOption { &self.mac_type }
    fn get_wake_on_lan_enabled(&self) -> &BoolOption { &self.wake_on_lan_enabled }
    fn get_vm_direct_path_gen_2_supported(&self) -> Option<bool> { self.vm_direct_path_gen_2_supported }
    fn get_upt_compatibility_enabled(&self) -> &Option<BoolOption> { &self.upt_compatibility_enabled }
}
impl VirtualEthernetCardOptionTrait for VirtualE1000EOption {
    fn get_supported_oui(&self) -> &ChoiceOption { &self.supported_oui }
    fn get_mac_type(&self) -> &ChoiceOption { &self.mac_type }
    fn get_wake_on_lan_enabled(&self) -> &BoolOption { &self.wake_on_lan_enabled }
    fn get_vm_direct_path_gen_2_supported(&self) -> Option<bool> { self.vm_direct_path_gen_2_supported }
    fn get_upt_compatibility_enabled(&self) -> &Option<BoolOption> { &self.upt_compatibility_enabled }
}
impl VirtualEthernetCardOptionTrait for VirtualPcNet32Option {
    fn get_supported_oui(&self) -> &ChoiceOption { &self.supported_oui }
    fn get_mac_type(&self) -> &ChoiceOption { &self.mac_type }
    fn get_wake_on_lan_enabled(&self) -> &BoolOption { &self.wake_on_lan_enabled }
    fn get_vm_direct_path_gen_2_supported(&self) -> Option<bool> { self.vm_direct_path_gen_2_supported }
    fn get_upt_compatibility_enabled(&self) -> &Option<BoolOption> { &self.upt_compatibility_enabled }
}
impl VirtualEthernetCardOptionTrait for VirtualSriovEthernetCardOption {
    fn get_supported_oui(&self) -> &ChoiceOption { &self.supported_oui }
    fn get_mac_type(&self) -> &ChoiceOption { &self.mac_type }
    fn get_wake_on_lan_enabled(&self) -> &BoolOption { &self.wake_on_lan_enabled }
    fn get_vm_direct_path_gen_2_supported(&self) -> Option<bool> { self.vm_direct_path_gen_2_supported }
    fn get_upt_compatibility_enabled(&self) -> &Option<BoolOption> { &self.upt_compatibility_enabled }
}
impl VirtualEthernetCardOptionTrait for VirtualVmxnetOption {
    fn get_supported_oui(&self) -> &ChoiceOption { &self.supported_oui }
    fn get_mac_type(&self) -> &ChoiceOption { &self.mac_type }
    fn get_wake_on_lan_enabled(&self) -> &BoolOption { &self.wake_on_lan_enabled }
    fn get_vm_direct_path_gen_2_supported(&self) -> Option<bool> { self.vm_direct_path_gen_2_supported }
    fn get_upt_compatibility_enabled(&self) -> &Option<BoolOption> { &self.upt_compatibility_enabled }
}
impl VirtualEthernetCardOptionTrait for VirtualVmxnet2Option {
    fn get_supported_oui(&self) -> &ChoiceOption { &self.supported_oui }
    fn get_mac_type(&self) -> &ChoiceOption { &self.mac_type }
    fn get_wake_on_lan_enabled(&self) -> &BoolOption { &self.wake_on_lan_enabled }
    fn get_vm_direct_path_gen_2_supported(&self) -> Option<bool> { self.vm_direct_path_gen_2_supported }
    fn get_upt_compatibility_enabled(&self) -> &Option<BoolOption> { &self.upt_compatibility_enabled }
}
impl VirtualEthernetCardOptionTrait for VirtualVmxnet3Option {
    fn get_supported_oui(&self) -> &ChoiceOption { &self.supported_oui }
    fn get_mac_type(&self) -> &ChoiceOption { &self.mac_type }
    fn get_wake_on_lan_enabled(&self) -> &BoolOption { &self.wake_on_lan_enabled }
    fn get_vm_direct_path_gen_2_supported(&self) -> Option<bool> { self.vm_direct_path_gen_2_supported }
    fn get_upt_compatibility_enabled(&self) -> &Option<BoolOption> { &self.upt_compatibility_enabled }
}
impl VirtualEthernetCardOptionTrait for VirtualVmxnet3VrdmaOption {
    fn get_supported_oui(&self) -> &ChoiceOption { &self.supported_oui }
    fn get_mac_type(&self) -> &ChoiceOption { &self.mac_type }
    fn get_wake_on_lan_enabled(&self) -> &BoolOption { &self.wake_on_lan_enabled }
    fn get_vm_direct_path_gen_2_supported(&self) -> Option<bool> { self.vm_direct_path_gen_2_supported }
    fn get_upt_compatibility_enabled(&self) -> &Option<BoolOption> { &self.upt_compatibility_enabled }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualEthernetCardOptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualEthernetCardOption => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardOption>()?),
            StructType::VirtualE1000Option => Some(from.as_any_ref().downcast_ref::<VirtualE1000Option>()?),
            StructType::VirtualE1000EOption => Some(from.as_any_ref().downcast_ref::<VirtualE1000EOption>()?),
            StructType::VirtualPcNet32Option => Some(from.as_any_ref().downcast_ref::<VirtualPcNet32Option>()?),
            StructType::VirtualSriovEthernetCardOption => Some(from.as_any_ref().downcast_ref::<VirtualSriovEthernetCardOption>()?),
            StructType::VirtualVmxnetOption => Some(from.as_any_ref().downcast_ref::<VirtualVmxnetOption>()?),
            StructType::VirtualVmxnet2Option => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet2Option>()?),
            StructType::VirtualVmxnet3Option => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet3Option>()?),
            StructType::VirtualVmxnet3VrdmaOption => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet3VrdmaOption>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualEthernetCardOption => Ok(from.as_any_box().downcast::<VirtualEthernetCardOption>()?),
            StructType::VirtualE1000Option => Ok(from.as_any_box().downcast::<VirtualE1000Option>()?),
            StructType::VirtualE1000EOption => Ok(from.as_any_box().downcast::<VirtualE1000EOption>()?),
            StructType::VirtualPcNet32Option => Ok(from.as_any_box().downcast::<VirtualPcNet32Option>()?),
            StructType::VirtualSriovEthernetCardOption => Ok(from.as_any_box().downcast::<VirtualSriovEthernetCardOption>()?),
            StructType::VirtualVmxnetOption => Ok(from.as_any_box().downcast::<VirtualVmxnetOption>()?),
            StructType::VirtualVmxnet2Option => Ok(from.as_any_box().downcast::<VirtualVmxnet2Option>()?),
            StructType::VirtualVmxnet3Option => Ok(from.as_any_box().downcast::<VirtualVmxnet3Option>()?),
            StructType::VirtualVmxnet3VrdmaOption => Ok(from.as_any_box().downcast::<VirtualVmxnet3VrdmaOption>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
