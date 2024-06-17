use std::fs::File;
use std::io::Read;
use openapiv3::OpenAPI;
use serde_json;

mod output;


fn main() {
    let Login = output::LoginRequestType {
        user_name: "administrator@vsphere.local".to_string(),
        password: "Admin!23".to_string(),
        locale: None,
    };
    let data = serde_json::to_string(&Login).unwrap();
    println!("{}", data);
//    let data = from_path("data/vi_json_openapi_specification_v8_0_2_0.yaml");
    // let mut file = File::open("data/vi_json_openapi_specification_v8_0_2_0.json").expect("unable to open file");

    // let mut data = String::new();
    // file.read_to_string(&mut data).expect("unable to read file");    
    // //let data = include_str!("data/vi_json_openapi_specification_v8_0_2_0.yaml");
    // let openapi: OpenAPI = serde_json::from_str(&data).expect("Could not deserialize input"); // Change OpenAPI to Value
    // println!("{:?}", openapi);

}


#[cfg(test)]
mod tests {
    use super::*;

    const DEVICE: &str = r#"
            {
                "_typeName": "VirtualVmxnet3",
                "key": 4000,
                "deviceInfo": {
                    "_typeName": "Description",
                    "label": "Network adapter 1",
                    "summary": "VM Network"
                },
                "backing": {
                    "_typeName": "VirtualEthernetCardNetworkBackingInfo",
                    "deviceName": "VM Network",
                    "useAutoDetect": false,
                    "network": {
                        "_typeName": "ManagedObjectReference",
                        "value": "network-27",
                        "type": "Network"
                    }
                },
                "connectable": {
                    "_typeName": "VirtualDeviceConnectInfo",
                    "migrateConnect": "unset",
                    "startConnected": true,
                    "allowGuestControl": false,
                    "connected": false,
                    "status": "untried"
                },
                "controllerKey": 100,
                "unitNumber": 7,
                "addressType": "assigned",
                "macAddress": "00:50:56:ac:4d:ed",
                "wakeOnLanEnabled": true,
                "resourceAllocation": {
                    "_typeName": "VirtualEthernetCardResourceAllocation",
                    "reservation": 0,
                    "share": {
                        "_typeName": "SharesInfo",
                        "shares": 50,
                        "level": "normal"
                    },
                    "limit": -1
                },
                "uptCompatibilityEnabled": true,
                "uptv2Enabled": false
            }
        "#;

    #[test]
    fn test_parse_device() {
        let device: output::BaseVirtualDevice = serde_json::from_str(DEVICE).unwrap();
        let output::BaseVirtualDevice::VirtualEthernetCard(ethernet_card) = device else {
            panic!("Expected VirtualEthernetCard, got {:?}", device);
        };
        let output::BaseVirtualEthernetCard::VirtualVmxnet(vmxnet) = ethernet_card else {
            panic!("Expected VirtualVmxnet, got {:?}", ethernet_card);
        };
    }
}