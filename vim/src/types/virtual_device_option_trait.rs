use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The VirtualDeviceOption data object type contains information about
/// a virtual device type, the options for configuring the virtual device,
/// and the relationship between this virtual device and other devices.
/// 
/// The vSphere API groups device configurations that are mutually exclusive
/// into different configuration objects; each of these configuration objects
/// may define subtypes for virtual device backing options
/// that are independent of the virtual device.
/// Backing-dependent options should appear in a subtype of
/// *VirtualDeviceBackingOption*.
pub trait VirtualDeviceOptionTrait : super::data_object_trait::DataObjectTrait {
    /// The name of the run-time class the client should instantiate
    /// to create a run-time instance of this device.
    fn get_type(&self) -> &str;
    /// If the device is connectable, then the connectOption
    /// describes the connect options and defaults.
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption>;
    /// If the device can use a bus slot configuration, then the busSlotOption
    /// describes the bus slot options.
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption>;
    /// Data object type that denotes the controller option object that is
    /// valid for controlling this device.
    fn get_controller_type(&self) -> &Option<String>;
    /// Flag to indicate whether or not this device will be auto-assigned a controller
    /// if one is required.
    /// 
    /// If this is true, then a client need not explicitly create
    /// the controller that this device will plug into.
    fn get_auto_assign_controller(&self) -> &Option<BoolOption>;
    /// A list of backing options that can be used to map the virtual
    /// device to the host.
    /// 
    /// The list is optional, since some devices exist only within
    /// the virtual machine; for example, a VirtualController.
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>>;
    /// Index into the backingOption list, indicating the default backing.
    fn get_default_backing_option_index(&self) -> Option<i32>;
    /// List of property names enforced by a licensing restriction
    /// of the underlying product.
    /// 
    /// For example, a limit that is not
    /// derived based on the product or hardware features; the
    /// property name "numCPU".
    fn get_licensing_limit(&self) -> &Option<Vec<String>>;
    /// Indicates whether this device is deprecated.
    /// 
    /// Hence, if set the device
    /// cannot be used when creating a new virtual machine or be added to an existing
    /// virtual machine. However, the device is still supported by the platform.
    fn get_deprecated(&self) -> bool;
    /// Indicates if this type of device can be hot-added to the virtual machine
    /// via a reconfigure operation when the virtual machine is powered on.
    fn get_plug_and_play(&self) -> bool;
    /// Indicates if this type of device can be hot-removed from the virtual machine
    /// via a reconfigure operation when the virtual machine is powered on.
    fn get_hot_remove_supported(&self) -> bool;
    /// ***Since:*** vSphere API Release 8.0.0.1
    fn get_numa_supported(&self) -> Option<bool>;
}
impl<'s> serde::Serialize for dyn VirtualDeviceOptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualDeviceOptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualDeviceOptionVisitor)
            }
        }

struct VirtualDeviceOptionVisitor;

impl<'de> de::Visitor<'de> for VirtualDeviceOptionVisitor {
    type Value = Box<dyn VirtualDeviceOptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualDeviceOptionTrait JSON object with a _typeName field")
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

impl VirtualDeviceOptionTrait for VirtualDeviceOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualCdromOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualControllerOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualIdeControllerOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualNvdimmControllerOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualNvmeControllerOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualPciControllerOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualPs2ControllerOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualSataControllerOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualAhciControllerOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualScsiControllerOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for ParaVirtualScsiControllerOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualBusLogicControllerOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualLsiLogicControllerOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualLsiLogicSasControllerOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualSioControllerOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualUsbControllerOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualUsbxhciControllerOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualDiskOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualEthernetCardOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualE1000Option {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualE1000EOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualPcNet32Option {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualSriovEthernetCardOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualVmxnetOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualVmxnet2Option {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualVmxnet3Option {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualVmxnet3VrdmaOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualFloppyOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualKeyboardOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualNvdimmOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualPciPassthroughOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualParallelPortOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualPointingDeviceOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualPrecisionClockOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualScsiPassthroughOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualSerialPortOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualSoundCardOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualEnsoniq1371Option {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualHdAudioCardOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualSoundBlaster16Option {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualTpmOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualUsbOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualMachineVmciDeviceOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualVmiromOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualVideoCardOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl VirtualDeviceOptionTrait for VirtualWdtOption {
    fn get_type(&self) -> &str { &self.r#type }
    fn get_connect_option(&self) -> &Option<VirtualDeviceConnectOption> { &self.connect_option }
    fn get_bus_slot_option(&self) -> &Option<VirtualDeviceBusSlotOption> { &self.bus_slot_option }
    fn get_controller_type(&self) -> &Option<String> { &self.controller_type }
    fn get_auto_assign_controller(&self) -> &Option<BoolOption> { &self.auto_assign_controller }
    fn get_backing_option(&self) -> &Option<Vec<Box<dyn super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait>>> { &self.backing_option }
    fn get_default_backing_option_index(&self) -> Option<i32> { self.default_backing_option_index }
    fn get_licensing_limit(&self) -> &Option<Vec<String>> { &self.licensing_limit }
    fn get_deprecated(&self) -> bool { self.deprecated }
    fn get_plug_and_play(&self) -> bool { self.plug_and_play }
    fn get_hot_remove_supported(&self) -> bool { self.hot_remove_supported }
    fn get_numa_supported(&self) -> Option<bool> { self.numa_supported }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualDeviceOptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceOption => Some(from.as_any_ref().downcast_ref::<VirtualDeviceOption>()?),
            StructType::VirtualCdromOption => Some(from.as_any_ref().downcast_ref::<VirtualCdromOption>()?),
            StructType::VirtualControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualControllerOption>()?),
            StructType::VirtualIdeControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualIdeControllerOption>()?),
            StructType::VirtualNvdimmControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualNvdimmControllerOption>()?),
            StructType::VirtualNvmeControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualNvmeControllerOption>()?),
            StructType::VirtualPciControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualPciControllerOption>()?),
            StructType::VirtualPs2ControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualPs2ControllerOption>()?),
            StructType::VirtualSataControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualSataControllerOption>()?),
            StructType::VirtualAhciControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualAhciControllerOption>()?),
            StructType::VirtualScsiControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualScsiControllerOption>()?),
            StructType::ParaVirtualScsiControllerOption => Some(from.as_any_ref().downcast_ref::<ParaVirtualScsiControllerOption>()?),
            StructType::VirtualBusLogicControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualBusLogicControllerOption>()?),
            StructType::VirtualLsiLogicControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualLsiLogicControllerOption>()?),
            StructType::VirtualLsiLogicSasControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualLsiLogicSasControllerOption>()?),
            StructType::VirtualSioControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualSioControllerOption>()?),
            StructType::VirtualUsbControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualUsbControllerOption>()?),
            StructType::VirtualUsbxhciControllerOption => Some(from.as_any_ref().downcast_ref::<VirtualUsbxhciControllerOption>()?),
            StructType::VirtualDiskOption => Some(from.as_any_ref().downcast_ref::<VirtualDiskOption>()?),
            StructType::VirtualEthernetCardOption => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardOption>()?),
            StructType::VirtualE1000Option => Some(from.as_any_ref().downcast_ref::<VirtualE1000Option>()?),
            StructType::VirtualE1000EOption => Some(from.as_any_ref().downcast_ref::<VirtualE1000EOption>()?),
            StructType::VirtualPcNet32Option => Some(from.as_any_ref().downcast_ref::<VirtualPcNet32Option>()?),
            StructType::VirtualSriovEthernetCardOption => Some(from.as_any_ref().downcast_ref::<VirtualSriovEthernetCardOption>()?),
            StructType::VirtualVmxnetOption => Some(from.as_any_ref().downcast_ref::<VirtualVmxnetOption>()?),
            StructType::VirtualVmxnet2Option => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet2Option>()?),
            StructType::VirtualVmxnet3Option => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet3Option>()?),
            StructType::VirtualVmxnet3VrdmaOption => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet3VrdmaOption>()?),
            StructType::VirtualFloppyOption => Some(from.as_any_ref().downcast_ref::<VirtualFloppyOption>()?),
            StructType::VirtualKeyboardOption => Some(from.as_any_ref().downcast_ref::<VirtualKeyboardOption>()?),
            StructType::VirtualNvdimmOption => Some(from.as_any_ref().downcast_ref::<VirtualNvdimmOption>()?),
            StructType::VirtualPciPassthroughOption => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughOption>()?),
            StructType::VirtualParallelPortOption => Some(from.as_any_ref().downcast_ref::<VirtualParallelPortOption>()?),
            StructType::VirtualPointingDeviceOption => Some(from.as_any_ref().downcast_ref::<VirtualPointingDeviceOption>()?),
            StructType::VirtualPrecisionClockOption => Some(from.as_any_ref().downcast_ref::<VirtualPrecisionClockOption>()?),
            StructType::VirtualScsiPassthroughOption => Some(from.as_any_ref().downcast_ref::<VirtualScsiPassthroughOption>()?),
            StructType::VirtualSerialPortOption => Some(from.as_any_ref().downcast_ref::<VirtualSerialPortOption>()?),
            StructType::VirtualSoundCardOption => Some(from.as_any_ref().downcast_ref::<VirtualSoundCardOption>()?),
            StructType::VirtualEnsoniq1371Option => Some(from.as_any_ref().downcast_ref::<VirtualEnsoniq1371Option>()?),
            StructType::VirtualHdAudioCardOption => Some(from.as_any_ref().downcast_ref::<VirtualHdAudioCardOption>()?),
            StructType::VirtualSoundBlaster16Option => Some(from.as_any_ref().downcast_ref::<VirtualSoundBlaster16Option>()?),
            StructType::VirtualTpmOption => Some(from.as_any_ref().downcast_ref::<VirtualTpmOption>()?),
            StructType::VirtualUsbOption => Some(from.as_any_ref().downcast_ref::<VirtualUsbOption>()?),
            StructType::VirtualMachineVmciDeviceOption => Some(from.as_any_ref().downcast_ref::<VirtualMachineVmciDeviceOption>()?),
            StructType::VirtualVmiromOption => Some(from.as_any_ref().downcast_ref::<VirtualVmiromOption>()?),
            StructType::VirtualVideoCardOption => Some(from.as_any_ref().downcast_ref::<VirtualVideoCardOption>()?),
            StructType::VirtualWdtOption => Some(from.as_any_ref().downcast_ref::<VirtualWdtOption>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceOption => Ok(from.as_any_box().downcast::<VirtualDeviceOption>()?),
            StructType::VirtualCdromOption => Ok(from.as_any_box().downcast::<VirtualCdromOption>()?),
            StructType::VirtualControllerOption => Ok(from.as_any_box().downcast::<VirtualControllerOption>()?),
            StructType::VirtualIdeControllerOption => Ok(from.as_any_box().downcast::<VirtualIdeControllerOption>()?),
            StructType::VirtualNvdimmControllerOption => Ok(from.as_any_box().downcast::<VirtualNvdimmControllerOption>()?),
            StructType::VirtualNvmeControllerOption => Ok(from.as_any_box().downcast::<VirtualNvmeControllerOption>()?),
            StructType::VirtualPciControllerOption => Ok(from.as_any_box().downcast::<VirtualPciControllerOption>()?),
            StructType::VirtualPs2ControllerOption => Ok(from.as_any_box().downcast::<VirtualPs2ControllerOption>()?),
            StructType::VirtualSataControllerOption => Ok(from.as_any_box().downcast::<VirtualSataControllerOption>()?),
            StructType::VirtualAhciControllerOption => Ok(from.as_any_box().downcast::<VirtualAhciControllerOption>()?),
            StructType::VirtualScsiControllerOption => Ok(from.as_any_box().downcast::<VirtualScsiControllerOption>()?),
            StructType::ParaVirtualScsiControllerOption => Ok(from.as_any_box().downcast::<ParaVirtualScsiControllerOption>()?),
            StructType::VirtualBusLogicControllerOption => Ok(from.as_any_box().downcast::<VirtualBusLogicControllerOption>()?),
            StructType::VirtualLsiLogicControllerOption => Ok(from.as_any_box().downcast::<VirtualLsiLogicControllerOption>()?),
            StructType::VirtualLsiLogicSasControllerOption => Ok(from.as_any_box().downcast::<VirtualLsiLogicSasControllerOption>()?),
            StructType::VirtualSioControllerOption => Ok(from.as_any_box().downcast::<VirtualSioControllerOption>()?),
            StructType::VirtualUsbControllerOption => Ok(from.as_any_box().downcast::<VirtualUsbControllerOption>()?),
            StructType::VirtualUsbxhciControllerOption => Ok(from.as_any_box().downcast::<VirtualUsbxhciControllerOption>()?),
            StructType::VirtualDiskOption => Ok(from.as_any_box().downcast::<VirtualDiskOption>()?),
            StructType::VirtualEthernetCardOption => Ok(from.as_any_box().downcast::<VirtualEthernetCardOption>()?),
            StructType::VirtualE1000Option => Ok(from.as_any_box().downcast::<VirtualE1000Option>()?),
            StructType::VirtualE1000EOption => Ok(from.as_any_box().downcast::<VirtualE1000EOption>()?),
            StructType::VirtualPcNet32Option => Ok(from.as_any_box().downcast::<VirtualPcNet32Option>()?),
            StructType::VirtualSriovEthernetCardOption => Ok(from.as_any_box().downcast::<VirtualSriovEthernetCardOption>()?),
            StructType::VirtualVmxnetOption => Ok(from.as_any_box().downcast::<VirtualVmxnetOption>()?),
            StructType::VirtualVmxnet2Option => Ok(from.as_any_box().downcast::<VirtualVmxnet2Option>()?),
            StructType::VirtualVmxnet3Option => Ok(from.as_any_box().downcast::<VirtualVmxnet3Option>()?),
            StructType::VirtualVmxnet3VrdmaOption => Ok(from.as_any_box().downcast::<VirtualVmxnet3VrdmaOption>()?),
            StructType::VirtualFloppyOption => Ok(from.as_any_box().downcast::<VirtualFloppyOption>()?),
            StructType::VirtualKeyboardOption => Ok(from.as_any_box().downcast::<VirtualKeyboardOption>()?),
            StructType::VirtualNvdimmOption => Ok(from.as_any_box().downcast::<VirtualNvdimmOption>()?),
            StructType::VirtualPciPassthroughOption => Ok(from.as_any_box().downcast::<VirtualPciPassthroughOption>()?),
            StructType::VirtualParallelPortOption => Ok(from.as_any_box().downcast::<VirtualParallelPortOption>()?),
            StructType::VirtualPointingDeviceOption => Ok(from.as_any_box().downcast::<VirtualPointingDeviceOption>()?),
            StructType::VirtualPrecisionClockOption => Ok(from.as_any_box().downcast::<VirtualPrecisionClockOption>()?),
            StructType::VirtualScsiPassthroughOption => Ok(from.as_any_box().downcast::<VirtualScsiPassthroughOption>()?),
            StructType::VirtualSerialPortOption => Ok(from.as_any_box().downcast::<VirtualSerialPortOption>()?),
            StructType::VirtualSoundCardOption => Ok(from.as_any_box().downcast::<VirtualSoundCardOption>()?),
            StructType::VirtualEnsoniq1371Option => Ok(from.as_any_box().downcast::<VirtualEnsoniq1371Option>()?),
            StructType::VirtualHdAudioCardOption => Ok(from.as_any_box().downcast::<VirtualHdAudioCardOption>()?),
            StructType::VirtualSoundBlaster16Option => Ok(from.as_any_box().downcast::<VirtualSoundBlaster16Option>()?),
            StructType::VirtualTpmOption => Ok(from.as_any_box().downcast::<VirtualTpmOption>()?),
            StructType::VirtualUsbOption => Ok(from.as_any_box().downcast::<VirtualUsbOption>()?),
            StructType::VirtualMachineVmciDeviceOption => Ok(from.as_any_box().downcast::<VirtualMachineVmciDeviceOption>()?),
            StructType::VirtualVmiromOption => Ok(from.as_any_box().downcast::<VirtualVmiromOption>()?),
            StructType::VirtualVideoCardOption => Ok(from.as_any_box().downcast::<VirtualVideoCardOption>()?),
            StructType::VirtualWdtOption => Ok(from.as_any_box().downcast::<VirtualWdtOption>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
