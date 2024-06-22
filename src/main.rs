//mod output;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "_typeName")]
pub enum BaseAny {
    #[serde(untagged)]
    VirtualDevice(BaseVirtualDevice),
    #[serde(untagged)]
    Array(Arrays),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "_typeName", content = "_value")]
pub enum Arrays {
    ArrayOfString(Vec<String>),
    ArrayOfInt(Vec<i64>),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VirtualDevice {
    // #[serde(flatten)]
    // pub data_object_: DataObject,
    #[serde(rename = "key")]
    pub key: i64,
    // #[serde(rename = "deviceInfo")]
    // pub device_info: Option<Box<BaseDescription>>,
    // #[serde(rename = "backing")]
    // pub backing: Option<Box<BaseVirtualDeviceBackingInfo>>,
    // #[serde(rename = "connectable")]
    // pub connectable: Option<Box<VirtualDeviceConnectInfo>>,
    // #[serde(rename = "slotInfo")]
    // pub slot_info: Option<Box<BaseVirtualDeviceBusSlotInfo>>,
    #[serde(rename = "controllerKey")]
    pub controller_key: Option<i64>,
    #[serde(rename = "unitNumber")]
    pub unit_number: Option<i64>,
    #[serde(rename = "numaNode")]
    pub numa_node: Option<i64>,
    // #[serde(rename = "deviceGroupInfo")]
    // pub device_group_info: Option<Box<VirtualDeviceDeviceGroupInfo>>,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "_typeName")]
pub enum BaseVirtualDevice {
    VirtualDevice(VirtualDevice),
    VirtualCdrom(VirtualCdrom),
    // VirtualDisk(VirtualDisk),
    // VirtualFloppy(VirtualFloppy),
    // VirtualKeyboard(VirtualKeyboard),
    // #[serde(rename = "VirtualNVDIMM")]
    // VirtualNvdimm(VirtualNvdimm),
    // #[serde(rename = "VirtualPCIPassthrough")]
    // VirtualPciPassthrough(VirtualPciPassthrough),
    // VirtualParallelPort(VirtualParallelPort),
    // VirtualPointingDevice(VirtualPointingDevice),
    // VirtualPrecisionClock(VirtualPrecisionClock),
    // #[serde(rename = "VirtualSCSIPassthrough")]
    // VirtualScsiPassthrough(VirtualScsiPassthrough),
    // VirtualSerialPort(VirtualSerialPort),
    // #[serde(rename = "VirtualTPM")]
    // VirtualTpm(VirtualTpm),
    // #[serde(rename = "VirtualUSB")]
    // VirtualUsb(VirtualUsb),
    // #[serde(rename = "VirtualMachineVMCIDevice")]
    // VirtualMachineVmciDevice(VirtualMachineVmciDevice),
    // #[serde(rename = "VirtualMachineVMIROM")]
    // VirtualMachineVmirom(VirtualMachineVmirom),
    // VirtualMachineVideoCard(VirtualMachineVideoCard),
    // #[serde(rename = "VirtualWDT")]
    // VirtualWdt(VirtualWdt),
    // #[serde(untagged)]
    // VirtualController(BaseVirtualController),
    #[serde(untagged)]
    VirtualEthernetCard(BaseVirtualEthernetCard),
    // #[serde(untagged)]
    // VirtualSoundCard(BaseVirtualSoundCard),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VirtualCdrom {
    #[serde(flatten)]
    pub virtual_device_: VirtualDevice,
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VirtualEthernetCard {
    #[serde(flatten)]
    pub virtual_device_: VirtualDevice,
    // #[serde(rename = "dynamicProperty")]
    // pub dynamic_property: Option<Vec<Box<DynamicProperty>>>,
    #[serde(rename = "addressType")]
    pub address_type: Option<String>,
    #[serde(rename = "macAddress")]
    pub mac_address: Option<String>,
    #[serde(rename = "wakeOnLanEnabled")]
    pub wake_on_lan_enabled: Option<bool>,
    // #[serde(rename = "resourceAllocation")]
    // pub resource_allocation: Option<Box<VirtualEthernetCardResourceAllocation>>,
    #[serde(rename = "externalId")]
    pub external_id: Option<String>,
    #[serde(rename = "uptCompatibilityEnabled")]
    pub upt_compatibility_enabled: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "_typeName")]
pub enum BaseVirtualEthernetCard {
    VirtualEthernetCard(VirtualEthernetCard),
    VirtualE1000(VirtualE1000),
    // #[serde(rename = "VirtualE1000e")]
    // VirtualE1000E(VirtualE1000E),
    // #[serde(rename = "VirtualPCNet32")]
    // VirtualPcNet32(VirtualPcNet32),
    // VirtualSriovEthernetCard(VirtualSriovEthernetCard),
    #[serde(untagged)]
    VirtualVmxnet(BaseVirtualVmxnet),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VirtualE1000 {
    #[serde(flatten)]
    pub virtual_ethernet_card_: VirtualEthernetCard,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VirtualVmxnet {
    #[serde(flatten)]
    pub virtual_ethernet_card_: VirtualEthernetCard,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "_typeName")]
pub enum BaseVirtualVmxnet {
    VirtualVmxnet(VirtualVmxnet),
    VirtualVmxnet2(VirtualVmxnet2),
    #[serde(untagged)]
    VirtualVmxnet3(BaseVirtualVmxnet3),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VirtualVmxnet2 {
    #[serde(flatten)]
    pub virtual_vmxnet_: VirtualVmxnet,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VirtualVmxnet3 {
    #[serde(flatten)]
    pub virtual_vmxnet_: VirtualVmxnet,
    #[serde(rename = "uptv2Enabled")]
    pub uptv_2_enabled: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "_typeName")]
pub enum BaseVirtualVmxnet3 {
    VirtualVmxnet3(VirtualVmxnet3),
    VirtualVmxnet3Vrdma(VirtualVmxnet3Vrdma),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VirtualVmxnet3Vrdma {
    #[serde(flatten)]
    pub virtual_vmxnet_3_: VirtualVmxnet3,
    #[serde(rename = "deviceProtocol")]
    pub device_protocol: Option<String>,
}




fn main() {
 print!("Hello World");
}


#[cfg(test)]
mod tests {
    use super::*;
    //use crate::output::*;

    const DEVICE: &str = r#"
            {
                "_typeName": "VirtualVmxnet3",
                "key": 100,
                "controllerKey": 200,
                "addressType": "assigned",
                "macAddress": "00:50:56:ac:4d:ed",
                "wakeOnLanEnabled": true,
                "uptCompatibilityEnabled": true,
                "uptv2Enabled": false
            }
        "#;

    #[test]
    fn test_parse_device() {
        let device: BaseVirtualDevice = serde_json::from_str(DEVICE).unwrap();
        dbg!(&device);
        // if let BaseVirtualDevice::VirtualEthernetCard(
        //         BaseVirtualEthernetCard::VirtualVmxnet(
        //             BaseVirtualVmxnet::VirtualVmxnet3(
        //                 BaseVirtualVmxnet3::VirtualVmxnet3(vmxnet3)))) = device {
        //     assert_eq!(vmxnet3.virtual_vmxnet_.virtual_ethernet_card_.virtual_device_.key, 100);
        //     assert_eq!(vmxnet3.virtual_vmxnet_.virtual_ethernet_card_.virtual_device_.controller_key, Some(200));
        //     assert_eq!(vmxnet3.virtual_vmxnet_.virtual_ethernet_card_.address_type, Some("assigned".to_string()));
        //     assert_eq!(vmxnet3.virtual_vmxnet_.virtual_ethernet_card_.mac_address, Some("00:50:56:ac:4d:ed".to_string()));
        //     assert_eq!(vmxnet3.virtual_vmxnet_.virtual_ethernet_card_.wake_on_lan_enabled, Some(true));
        //     assert_eq!(vmxnet3.virtual_vmxnet_.virtual_ethernet_card_.upt_compatibility_enabled, Some(true));
        //     assert_eq!(vmxnet3.uptv_2_enabled, Some(false));
        // } else {
        //     panic!("unexpected device type: {:?}", device);
        // }
    }

    #[test]
    fn test_parse_array_of_int() {
        let array: BaseAny = serde_json::from_str(r#"{"_typeName":"ArrayOfInt","_value":[1,2,3]}"#).unwrap();
        dbg!(&array);
        if let BaseAny::Array(Arrays::ArrayOfInt(array)) = array {
            assert_eq!(array, vec![1, 2, 3]);
        } else {
            panic!("unexpected array type: {:?}", array);
        }
    }
}