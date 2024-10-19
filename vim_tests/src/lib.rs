
#[cfg(test)]
mod tests {
    use reqwest::ClientBuilder;
    use vim::vim;
    use ::vim::vim::{CastInto, VirtualDeviceTrait};

    #[test]
    fn it_works() {
        let login = vim::LoginRequestType{
            user_name: "administrator@vsphere.local".to_string(),
            password: "secret!".to_string(),
            locale: None,
        };
        let s = serde_json::to_string(&login).unwrap();
        println!("{}", s);
        let login1 = serde_json::from_str::<vim::LoginRequestType>(&s).unwrap();
        assert_eq!(login.user_name, login1.user_name);
        print!("login1: {:?}", login1);
    }

    #[test]
    fn test_ethernet() {
        let e1000 = vim::VirtualE1000 {
            mac_address: Some("00:50:56:aa:bb:cc".to_string()),
            wake_on_lan_enabled: Some(true),
            address_type: Some("Generated".to_string()),
            numa_node: None,
            device_group_info: None,
            resource_allocation: None,
            upt_compatibility_enabled: Some(false),
            external_id: None,
            unit_number: None,
            backing: None,
            controller_key: None,
            slot_info: None,
            key: 1000,
            device_info: None,
            connectable: None,
            dynamic_property: None,
        };
        let s = serde_json::to_string(&e1000).unwrap();
        println!("{}", s);
        let vd: Box<dyn VirtualDeviceTrait> = serde_json::from_str(&s).unwrap();
        assert_eq!(vd.get_key(), 1000);
        let eth: Box<dyn vim::VirtualEthernetCardTrait> = vd.into_box().unwrap();
        assert_eq!(*eth.get_mac_address(), Some("00:50:56:aa:bb:cc".to_string()));
        println!("{:?}", eth);
    }

    #[tokio::test]
    async fn get_content() {
        let builder = ClientBuilder::new();
        let client = builder.danger_accept_invalid_certs(true)
                                .danger_accept_invalid_hostnames(true)
                                .build()
                                .unwrap();
        let res = client.get("https://vc8.home/sdk/vim25/8.0.3.0/ServiceInstance/ServiceInstance/content").send().await.unwrap();
        if res.status() != 200 {
            let fault: Box<dyn vim::MethodFaultTrait> = res.json().await.unwrap();
            panic!("Failed to get content: {:?}", fault);
        }
        let content: vim::ServiceContent = res.json().await.unwrap();
        println!("{:?}", content.about);

    }
}
