use serde;

#[derive(Debug, serde::Deserialize)]
struct Device {
    key: i32,
    controller_key: Option<i32>,
    unit_number: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "_typeName")]
enum DeviceEnum {
    Device(Device),
    Disk(Disk),
    #[serde(untagged)]
    EthernetCard(EthernetCardEnum),
}

trait DeviceTrait {
    fn device(&self) -> &Device;
    fn device_mut(&mut self) -> &mut Device;
}

impl DeviceTrait for DeviceEnum {
    fn device(&self) -> &Device {
        match self {
            DeviceEnum::Device(device) => device,
            DeviceEnum::Disk(disk) => &disk.device,
            DeviceEnum::EthernetCard(ethernet_card) => ethernet_card.device(),
        }
    }
    fn device_mut(&mut self) -> &mut Device {
        match self {
            DeviceEnum::Device(device) => device,
            DeviceEnum::Disk(disk) => &mut disk.device,
            DeviceEnum::EthernetCard(ethernet_card) => ethernet_card.device_mut(),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct EthernetCard {
    #[serde(flatten)]
    device: Device,
    mac_address: String,
    ip_address: String,
}

trait EthernetCardTrait : DeviceTrait {
    fn ethernet_card(&self) -> &EthernetCard;
    fn ethernet_card_mut(&mut self) -> &mut EthernetCard;
}

#[derive(Debug, serde::Deserialize)]
struct Net4000 {
    #[serde(flatten)]
    ethernet_card: EthernetCard,
    wake_on_lan_enabled: bool,
}

#[derive(Debug, serde::Deserialize)]
struct Disk {
    #[serde(flatten)]
    device: Device,
    capacity: i64,
}


#[derive(Debug, serde::Deserialize)]
#[serde(tag = "_typeName")]
enum EthernetCardEnum {
    EthernetCard(EthernetCard),
    Net4000(Net4000),
}

impl DeviceTrait for EthernetCardEnum {
    fn device(&self) -> &Device {
        &self.ethernet_card().device
    }

    fn device_mut(&mut self) -> &mut Device {
        &mut self.ethernet_card_mut().device
    }
}

impl EthernetCardTrait for EthernetCardEnum {
    fn ethernet_card(&self) -> &EthernetCard {
        match self {
            EthernetCardEnum::EthernetCard(ethernet_card) => ethernet_card,
            EthernetCardEnum::Net4000(net4000) => &net4000.ethernet_card,
        }
    }

    fn ethernet_card_mut(&mut self) -> &mut EthernetCard {
        match self {
            EthernetCardEnum::EthernetCard(ethernet_card) => ethernet_card,
            EthernetCardEnum::Net4000(net4000) => &mut net4000.ethernet_card,
        }
    }
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device() {
        let s: String = "Hello, world!".into();
        println!("str: {}", s);
        let json = r#"[{"_typeName":"Net4000",
                "key": 1,
                "controller_key": 1,
                "unit_number": 1,
                "mac_address": "00:00:00:00:00:02",
                "ip_address": "10.0.0.24",
                "wake_on_lan_enabled": true
            },
            {
                "_typeName": "Disk",
                "key": 2,
                "controller_key": 2,
                "unit_number": 2,
                "capacity": 1000000000000
            },
            {
                "_typeName": "Device",
                "key": 3,
                "controller_key": 3,
                "unit_number": 3
            },
            {
                "_typeName": "EthernetCard",
                "key": 4,
                "controller_key": 4,
                "unit_number": 4,
                "mac_address": "00:00:00:00:00:03",
                "ip_address": "10.0.0.25"
            }
        ]"#;
        let data = serde_json::from_str::<Vec<DeviceEnum>>(json).unwrap();
        assert_eq!(data.len(), 4);
        let net4000 = match &data[0] {
            DeviceEnum::EthernetCard(EthernetCardEnum::Net4000(net4000)) => net4000,
            _ => panic!("Expected Net4000"),
        };
        assert_eq!(net4000.ethernet_card.device.key, 1);
        assert_eq!(net4000.ethernet_card.device.controller_key, Some(1));
        assert_eq!(net4000.ethernet_card.device.unit_number, Some(1));
        assert_eq!(net4000.ethernet_card.mac_address, "00:00:00:00:00:02");
        assert_eq!(net4000.ethernet_card.ip_address, "10.0.0.24");
        assert_eq!(net4000.wake_on_lan_enabled, true);

        // Print all ethernet card mac addresses and ip addresses
        for device in data {
            match device {
                DeviceEnum::EthernetCard(ethernet_card) => {
                    println!("MAC Address: {}", ethernet_card.ethernet_card().ip_address);
                    println!("IP Address: {}", ethernet_card.ethernet_card().mac_address);
                    println!("key: {}", ethernet_card.device().key);
                },
                _ => (),
            }
        }
    }

    #[test]
    fn test_device_to_net4000() {
        let device = DeviceEnum::EthernetCard(EthernetCardEnum::Net4000(Net4000 {
            ethernet_card: EthernetCard {
                device: Device {
                    key: 1,
                    controller_key: Some(1),
                    unit_number: Some(1),
                },
                mac_address: "00:00:00:00:00:02".into(),
                ip_address: "10.0.0.24".into(),
            },
            wake_on_lan_enabled: true,
        }));
        if !matches!(device, DeviceEnum::EthernetCard(EthernetCardEnum::Net4000(_))) {
            panic!("Expected Net4000");
        }
        if let DeviceEnum::EthernetCard(EthernetCardEnum::Net4000(net4000)) = device {
            println!("net4000 MAC Address: {}", net4000.ethernet_card.mac_address);
        } else {
            panic!("Expected Net4000");
        }
    }

    fn print_ethernet_card(ethernet_card: &EthernetCardEnum) {
        println!("MAC Address: {}", ethernet_card.ethernet_card().mac_address);
        println!("IP Address: {}", ethernet_card.ethernet_card().ip_address);
        println!("key: {}", ethernet_card.device().key);
    }

    #[test]
    fn test_print_ethernet_card() {
        let json = r#"{
            "_typeName": "EthernetCard",
            "key": 4,
            "controller_key": 4,
            "unit_number": 4,
            "mac_address": "00:00:00:00:00:03",
            "ip_address": "10.0.0.25"
        }"#;
        let device = serde_json::from_str::<DeviceEnum>(json).unwrap();

        // if device instanceof EthernetCard
        if let DeviceEnum::EthernetCard(ethernet_card) = device {
            print_ethernet_card(&ethernet_card);
        } else {
            panic!("Expected EthernetCard");
        }

    }

    #[test]
    fn test_print_net4000() {
        // Upcast the Net4000 to an EthernetCardEnum
        let mut ec = EthernetCardEnum::Net4000(Net4000 { 
            ethernet_card: EthernetCard { 
                device: Device { 
                    key: 5, 
                    controller_key: Some(2), 
                    unit_number: Some(4), 
                },
                mac_address: "00:00:00:00:00:02".to_string(), 
                ip_address: "10.0.0.24".to_string(), 
            }, 
            wake_on_lan_enabled: true,
        });
        // Mutate the device key via the EthernetCardEnum
        let d = ec.device_mut();
        d.key = 2;

        print_ethernet_card(&ec);
    }
}