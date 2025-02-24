use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// VirtualDevice is the base data object type for devices in a virtual machine.
/// 
/// This type contains enough information about a virtual device to allow clients
/// to display devices they do not recognize. For example, a client with an
/// older version than the server to which it connects may see a device
/// without knowing what it is.
pub trait VirtualDeviceTrait : super::data_object_trait::DataObjectTrait {
    /// A unique key that distinguishes this device from other
    /// devices in the same virtual machine.
    /// 
    /// Keys are immutable but may be
    /// recycled; that is, a key does not change as long as the device is
    /// associated with a particular virtual machine. However, once a device is
    /// removed, its key may be used when another device is added.
    /// 
    /// This property is not read-only, but the client cannot control its value.
    /// Persistent device keys are always assigned and managed by the server, which
    /// guarantees that all devices will have non-negative key values.
    /// 
    /// When adding new devices, it may be necessary for a client to assign keys
    /// temporarily in order to associate controllers with devices in
    /// configuring a virtual machine. However, the server does not allow a
    /// client to reassign a device key, and the server may assign a different
    /// value from the one passed during configuration. Clients should ensure
    /// that existing device keys are not reused as temporary key values for the
    /// new device to be added (for example, by using unique negative integers as
    /// temporary keys).
    /// 
    /// When editing or deleting a device, clients must use the server-provided key
    /// to refer to an existing device.
    fn get_key(&self) -> i32;
    /// Provides a label and summary information for the device.
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>>;
    /// Information about the backing of this virtual device presented
    /// in the context of the virtual machine's environment.
    /// 
    /// Not all devices are required to have backing information.
    /// 
    /// See also *VirtualMachineConfigOption*.
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>>;
    /// Provides information about restrictions on removing this device while
    /// a virtual machine is running.
    /// 
    /// If the device is not removable, then
    /// this property is null.
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo>;
    /// Information about the bus slot of a device in a virtual machine.
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>>;
    /// Object key for the controller object for this device.
    /// 
    /// This property contains the key property value of the controller device
    /// object.
    fn get_controller_key(&self) -> Option<i32>;
    /// The unit number of this device on its controller.
    /// 
    /// This property is null if
    /// the controller property is null (for example, when the device is not
    /// attached to a specific controller object).
    /// 
    /// Normally, two devices on the same controller
    /// may not be assigned the same unit number. If
    /// multiple devices could exist on a controller,
    /// then unit number has to be specified to
    /// configure respective devices.
    fn get_unit_number(&self) -> Option<i32>;
    /// The virtual NUMA node.
    /// 
    /// A negative number means there is no
    /// affinity for the device. A positive number is a vNUMA node.
    /// An unset value of numaNode is status-quo during Reconfigure time.
    /// If numaNode is unset during ConfigInfo, then it means there is no
    /// affinity for the device.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    fn get_numa_node(&self) -> Option<i32>;
    /// Information about device group device is part of.
    /// 
    /// Devices in the device group cannot be added/removed individually,
    /// whole group has to be added/removed at once. Value can be set
    /// during device add, it cannot be modified later.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo>;
}
impl<'s> serde::Serialize for dyn VirtualDeviceTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualDeviceTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualDeviceVisitor)
            }
        }

struct VirtualDeviceVisitor;

impl<'de> de::Visitor<'de> for VirtualDeviceVisitor {
    type Value = Box<dyn VirtualDeviceTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualDeviceTrait JSON object with a _typeName field")
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

impl VirtualDeviceTrait for VirtualDevice {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualCdrom {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualController {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualIdeController {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualNvdimmController {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualNvmeController {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualPciController {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualPs2Controller {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualSataController {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualAhciController {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualScsiController {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for ParaVirtualScsiController {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualBusLogicController {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualLsiLogicController {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualLsiLogicSasController {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualSioController {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualUsbController {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualUsbxhciController {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualDisk {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualEthernetCard {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualE1000 {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualE1000E {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualPcNet32 {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualSriovEthernetCard {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualVmxnet {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualVmxnet2 {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualVmxnet3 {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualVmxnet3Vrdma {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualFloppy {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualKeyboard {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualNvdimm {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualPciPassthrough {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualParallelPort {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualPointingDevice {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualPrecisionClock {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualScsiPassthrough {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualSerialPort {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualSoundCard {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualEnsoniq1371 {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualHdAudioCard {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualSoundBlaster16 {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualTpm {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualUsb {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualMachineVmciDevice {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualMachineVmirom {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualMachineVideoCard {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl VirtualDeviceTrait for VirtualWdt {
    fn get_key(&self) -> i32 { self.key }
    fn get_device_info(&self) -> &Option<Box<dyn super::description_trait::DescriptionTrait>> { &self.device_info }
    fn get_backing(&self) -> &Option<Box<dyn super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait>> { &self.backing }
    fn get_connectable(&self) -> &Option<VirtualDeviceConnectInfo> { &self.connectable }
    fn get_slot_info(&self) -> &Option<Box<dyn super::virtual_device_bus_slot_info_trait::VirtualDeviceBusSlotInfoTrait>> { &self.slot_info }
    fn get_controller_key(&self) -> Option<i32> { self.controller_key }
    fn get_unit_number(&self) -> Option<i32> { self.unit_number }
    fn get_numa_node(&self) -> Option<i32> { self.numa_node }
    fn get_device_group_info(&self) -> &Option<VirtualDeviceDeviceGroupInfo> { &self.device_group_info }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualDeviceTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDevice => Some(from.as_any_ref().downcast_ref::<VirtualDevice>()?),
            StructType::VirtualCdrom => Some(from.as_any_ref().downcast_ref::<VirtualCdrom>()?),
            StructType::VirtualController => Some(from.as_any_ref().downcast_ref::<VirtualController>()?),
            StructType::VirtualIdeController => Some(from.as_any_ref().downcast_ref::<VirtualIdeController>()?),
            StructType::VirtualNvdimmController => Some(from.as_any_ref().downcast_ref::<VirtualNvdimmController>()?),
            StructType::VirtualNvmeController => Some(from.as_any_ref().downcast_ref::<VirtualNvmeController>()?),
            StructType::VirtualPciController => Some(from.as_any_ref().downcast_ref::<VirtualPciController>()?),
            StructType::VirtualPs2Controller => Some(from.as_any_ref().downcast_ref::<VirtualPs2Controller>()?),
            StructType::VirtualSataController => Some(from.as_any_ref().downcast_ref::<VirtualSataController>()?),
            StructType::VirtualAhciController => Some(from.as_any_ref().downcast_ref::<VirtualAhciController>()?),
            StructType::VirtualScsiController => Some(from.as_any_ref().downcast_ref::<VirtualScsiController>()?),
            StructType::ParaVirtualScsiController => Some(from.as_any_ref().downcast_ref::<ParaVirtualScsiController>()?),
            StructType::VirtualBusLogicController => Some(from.as_any_ref().downcast_ref::<VirtualBusLogicController>()?),
            StructType::VirtualLsiLogicController => Some(from.as_any_ref().downcast_ref::<VirtualLsiLogicController>()?),
            StructType::VirtualLsiLogicSasController => Some(from.as_any_ref().downcast_ref::<VirtualLsiLogicSasController>()?),
            StructType::VirtualSioController => Some(from.as_any_ref().downcast_ref::<VirtualSioController>()?),
            StructType::VirtualUsbController => Some(from.as_any_ref().downcast_ref::<VirtualUsbController>()?),
            StructType::VirtualUsbxhciController => Some(from.as_any_ref().downcast_ref::<VirtualUsbxhciController>()?),
            StructType::VirtualDisk => Some(from.as_any_ref().downcast_ref::<VirtualDisk>()?),
            StructType::VirtualEthernetCard => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCard>()?),
            StructType::VirtualE1000 => Some(from.as_any_ref().downcast_ref::<VirtualE1000>()?),
            StructType::VirtualE1000E => Some(from.as_any_ref().downcast_ref::<VirtualE1000E>()?),
            StructType::VirtualPcNet32 => Some(from.as_any_ref().downcast_ref::<VirtualPcNet32>()?),
            StructType::VirtualSriovEthernetCard => Some(from.as_any_ref().downcast_ref::<VirtualSriovEthernetCard>()?),
            StructType::VirtualVmxnet => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet>()?),
            StructType::VirtualVmxnet2 => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet2>()?),
            StructType::VirtualVmxnet3 => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet3>()?),
            StructType::VirtualVmxnet3Vrdma => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet3Vrdma>()?),
            StructType::VirtualFloppy => Some(from.as_any_ref().downcast_ref::<VirtualFloppy>()?),
            StructType::VirtualKeyboard => Some(from.as_any_ref().downcast_ref::<VirtualKeyboard>()?),
            StructType::VirtualNvdimm => Some(from.as_any_ref().downcast_ref::<VirtualNvdimm>()?),
            StructType::VirtualPciPassthrough => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthrough>()?),
            StructType::VirtualParallelPort => Some(from.as_any_ref().downcast_ref::<VirtualParallelPort>()?),
            StructType::VirtualPointingDevice => Some(from.as_any_ref().downcast_ref::<VirtualPointingDevice>()?),
            StructType::VirtualPrecisionClock => Some(from.as_any_ref().downcast_ref::<VirtualPrecisionClock>()?),
            StructType::VirtualScsiPassthrough => Some(from.as_any_ref().downcast_ref::<VirtualScsiPassthrough>()?),
            StructType::VirtualSerialPort => Some(from.as_any_ref().downcast_ref::<VirtualSerialPort>()?),
            StructType::VirtualSoundCard => Some(from.as_any_ref().downcast_ref::<VirtualSoundCard>()?),
            StructType::VirtualEnsoniq1371 => Some(from.as_any_ref().downcast_ref::<VirtualEnsoniq1371>()?),
            StructType::VirtualHdAudioCard => Some(from.as_any_ref().downcast_ref::<VirtualHdAudioCard>()?),
            StructType::VirtualSoundBlaster16 => Some(from.as_any_ref().downcast_ref::<VirtualSoundBlaster16>()?),
            StructType::VirtualTpm => Some(from.as_any_ref().downcast_ref::<VirtualTpm>()?),
            StructType::VirtualUsb => Some(from.as_any_ref().downcast_ref::<VirtualUsb>()?),
            StructType::VirtualMachineVmciDevice => Some(from.as_any_ref().downcast_ref::<VirtualMachineVmciDevice>()?),
            StructType::VirtualMachineVmirom => Some(from.as_any_ref().downcast_ref::<VirtualMachineVmirom>()?),
            StructType::VirtualMachineVideoCard => Some(from.as_any_ref().downcast_ref::<VirtualMachineVideoCard>()?),
            StructType::VirtualWdt => Some(from.as_any_ref().downcast_ref::<VirtualWdt>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDevice => Ok(from.as_any_box().downcast::<VirtualDevice>()?),
            StructType::VirtualCdrom => Ok(from.as_any_box().downcast::<VirtualCdrom>()?),
            StructType::VirtualController => Ok(from.as_any_box().downcast::<VirtualController>()?),
            StructType::VirtualIdeController => Ok(from.as_any_box().downcast::<VirtualIdeController>()?),
            StructType::VirtualNvdimmController => Ok(from.as_any_box().downcast::<VirtualNvdimmController>()?),
            StructType::VirtualNvmeController => Ok(from.as_any_box().downcast::<VirtualNvmeController>()?),
            StructType::VirtualPciController => Ok(from.as_any_box().downcast::<VirtualPciController>()?),
            StructType::VirtualPs2Controller => Ok(from.as_any_box().downcast::<VirtualPs2Controller>()?),
            StructType::VirtualSataController => Ok(from.as_any_box().downcast::<VirtualSataController>()?),
            StructType::VirtualAhciController => Ok(from.as_any_box().downcast::<VirtualAhciController>()?),
            StructType::VirtualScsiController => Ok(from.as_any_box().downcast::<VirtualScsiController>()?),
            StructType::ParaVirtualScsiController => Ok(from.as_any_box().downcast::<ParaVirtualScsiController>()?),
            StructType::VirtualBusLogicController => Ok(from.as_any_box().downcast::<VirtualBusLogicController>()?),
            StructType::VirtualLsiLogicController => Ok(from.as_any_box().downcast::<VirtualLsiLogicController>()?),
            StructType::VirtualLsiLogicSasController => Ok(from.as_any_box().downcast::<VirtualLsiLogicSasController>()?),
            StructType::VirtualSioController => Ok(from.as_any_box().downcast::<VirtualSioController>()?),
            StructType::VirtualUsbController => Ok(from.as_any_box().downcast::<VirtualUsbController>()?),
            StructType::VirtualUsbxhciController => Ok(from.as_any_box().downcast::<VirtualUsbxhciController>()?),
            StructType::VirtualDisk => Ok(from.as_any_box().downcast::<VirtualDisk>()?),
            StructType::VirtualEthernetCard => Ok(from.as_any_box().downcast::<VirtualEthernetCard>()?),
            StructType::VirtualE1000 => Ok(from.as_any_box().downcast::<VirtualE1000>()?),
            StructType::VirtualE1000E => Ok(from.as_any_box().downcast::<VirtualE1000E>()?),
            StructType::VirtualPcNet32 => Ok(from.as_any_box().downcast::<VirtualPcNet32>()?),
            StructType::VirtualSriovEthernetCard => Ok(from.as_any_box().downcast::<VirtualSriovEthernetCard>()?),
            StructType::VirtualVmxnet => Ok(from.as_any_box().downcast::<VirtualVmxnet>()?),
            StructType::VirtualVmxnet2 => Ok(from.as_any_box().downcast::<VirtualVmxnet2>()?),
            StructType::VirtualVmxnet3 => Ok(from.as_any_box().downcast::<VirtualVmxnet3>()?),
            StructType::VirtualVmxnet3Vrdma => Ok(from.as_any_box().downcast::<VirtualVmxnet3Vrdma>()?),
            StructType::VirtualFloppy => Ok(from.as_any_box().downcast::<VirtualFloppy>()?),
            StructType::VirtualKeyboard => Ok(from.as_any_box().downcast::<VirtualKeyboard>()?),
            StructType::VirtualNvdimm => Ok(from.as_any_box().downcast::<VirtualNvdimm>()?),
            StructType::VirtualPciPassthrough => Ok(from.as_any_box().downcast::<VirtualPciPassthrough>()?),
            StructType::VirtualParallelPort => Ok(from.as_any_box().downcast::<VirtualParallelPort>()?),
            StructType::VirtualPointingDevice => Ok(from.as_any_box().downcast::<VirtualPointingDevice>()?),
            StructType::VirtualPrecisionClock => Ok(from.as_any_box().downcast::<VirtualPrecisionClock>()?),
            StructType::VirtualScsiPassthrough => Ok(from.as_any_box().downcast::<VirtualScsiPassthrough>()?),
            StructType::VirtualSerialPort => Ok(from.as_any_box().downcast::<VirtualSerialPort>()?),
            StructType::VirtualSoundCard => Ok(from.as_any_box().downcast::<VirtualSoundCard>()?),
            StructType::VirtualEnsoniq1371 => Ok(from.as_any_box().downcast::<VirtualEnsoniq1371>()?),
            StructType::VirtualHdAudioCard => Ok(from.as_any_box().downcast::<VirtualHdAudioCard>()?),
            StructType::VirtualSoundBlaster16 => Ok(from.as_any_box().downcast::<VirtualSoundBlaster16>()?),
            StructType::VirtualTpm => Ok(from.as_any_box().downcast::<VirtualTpm>()?),
            StructType::VirtualUsb => Ok(from.as_any_box().downcast::<VirtualUsb>()?),
            StructType::VirtualMachineVmciDevice => Ok(from.as_any_box().downcast::<VirtualMachineVmciDevice>()?),
            StructType::VirtualMachineVmirom => Ok(from.as_any_box().downcast::<VirtualMachineVmirom>()?),
            StructType::VirtualMachineVideoCard => Ok(from.as_any_box().downcast::<VirtualMachineVideoCard>()?),
            StructType::VirtualWdt => Ok(from.as_any_box().downcast::<VirtualWdt>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
