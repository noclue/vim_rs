

#[cfg(test)]
mod tests {
    use vim::types::structs::VirtualE1000;
    use vim::types::boxed_types::ValueElements;
    use vim::types::vim_any::VimAny;
    use vim::types::traits::VirtualDeviceTrait;
    use vim::types::traits::VirtualEthernetCardTrait;
    use vim::types::enums::MoTypesEnum;
    use vim::types::convert::CastInto;
    use log::{debug, info};
    use std::convert::AsRef;
    use vim::types::vim_object_trait::VimObjectTrait;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_vim_any_box() {
        init();
        let e1000 = create_virtual_e1000();
        let s = serde_json::to_string(&e1000).unwrap();
        debug!("{}", s);
        let any: VimAny = serde_json::from_str(&s).unwrap();
        info!("{:?}", any);
    }

    
    #[test]
    fn test_eth_json() {
        init();
        let e1000 = create_virtual_e1000();
        let s = serde_json::to_string(&e1000).unwrap();
        debug!("{}", s);
        let eth: VirtualE1000 = serde_json::from_str(&s).unwrap();
        info!("{:?}", eth);
    }

    #[test]
    fn test_ethernet_box() {
        init();
        let e1000 = create_virtual_e1000();
        let s = serde_json::to_string(&e1000).unwrap();
        debug!("{}", s);
        let vd: Box<dyn VirtualDeviceTrait> = serde_json::from_str(&s).unwrap();
        assert_eq!(vd.get_key(), 1000);
        let eth: Box<dyn VirtualEthernetCardTrait> = vd.into_box().unwrap();
        assert_eq!(*eth.get_mac_address(), Some("00:50:56:aa:bb:cc".to_string()));
        info!("{:?}", eth);
    }

    #[test]
    fn test_ethernet_ref() {
        init();
        let e1000 = &create_virtual_e1000();
        let vd: &dyn VirtualDeviceTrait = e1000;
        assert_eq!(vd.get_key(), 1000);
        let eth: &dyn VirtualEthernetCardTrait = vd.into_ref().unwrap();
        assert_eq!(*eth.get_mac_address(), Some("00:50:56:aa:bb:cc".to_string()));
        info!("{:?}", eth);
    }

    #[test]
    fn consume_vim_any_as_box() {
        init();
        let vim_any = eth_as_any();

        let e1000: Box<VirtualE1000> = match vim_any {
            VimAny::Object(e) => e.as_any_box().downcast().unwrap(),
            _ => panic!("Unexpected type"),
        };
        assert_eq!(e1000.key, 1000);
        info!("{:?}", e1000);
    }

    #[test]
    fn consume_vim_any_as_ref() {
        init();
        let vim_any = eth_as_any();
        let VimAny::Object(vim_obj) = vim_any else {
            panic!("Unexpected type");
        };
        let e1000: &VirtualE1000 = vim_obj.as_any_ref().downcast_ref().unwrap();
        assert_eq!(e1000.key, 1000);
        info!("{:?}", e1000);
    }
    
    #[test]
    fn consume_vim_any_as_trait_box() {
        init();
        let vim_any = eth_as_any();
        let VimAny::Object(vim_obj) = vim_any else {
            panic!("Unexpected type");
        };
        let eth: Box<dyn VirtualEthernetCardTrait> = vim_obj.into_box().unwrap();

        assert_eq!(eth.get_key(), 1000);
        info!("{:?}", eth);
    }

    #[test]
    fn consume_vim_any_as_trait_ref() {
        init();
        let vim_any = eth_as_any();
        let VimAny::Object(vim_obj) = vim_any else {
            panic!("Unexpected type");
        };
        let eth: &dyn VirtualEthernetCardTrait = vim_obj.as_ref().into_ref().unwrap();

        assert_eq!(eth.get_key(), 1000);
        info!("{:?}", eth);
    }

    #[test]
    fn consume_vim_any_array() {
        init();
        let vim_any = create_virtual_device_array();
        let VimAny::Value(ValueElements::ArrayOfVirtualDevice(vd)) = vim_any else {
            panic!("Unexpected type");
        };
        let e1000 = vd[0].as_any_ref().downcast_ref::<VirtualE1000>().unwrap();
        assert_eq!(e1000.key, 1000);
        info!("{:?}", e1000);
    }

    #[test]
    fn enum_as_string() {
        init();
        let e = MoTypesEnum::VirtualMachine;
        // Below is the declared conversion and Rust is somehow ok if no lifetime is specified
        // I gather 'static fits all....
        // let s: &'static str = e.as_ref(); 
        let s: &str = e.into();
        assert_eq!(s, "VirtualMachine");
    }

    #[test]
    fn enum_unknown_as_string() {
        init();
        let e = MoTypesEnum::Other_(String::from("Container"));
        assert_eq!(Into::<&'static str>::into(e), "__OTHER__");
    }

    #[test]
    fn type_name() {
        init();
        let e1000 = create_virtual_e1000();
        let e1000_type: &'static str = e1000.data_type().into();
        assert_eq!(e1000_type, "VirtualE1000");
    }

    fn create_virtual_device_array() -> VimAny {
        VimAny::Value(ValueElements::ArrayOfVirtualDevice(vec![Box::new(create_virtual_e1000())]))
    }


    fn eth_as_any() -> VimAny {
        VimAny::Object(Box::new(create_virtual_e1000()))
    }
    
    fn create_virtual_e1000() -> VirtualE1000 {
        VirtualE1000 {
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
        }
    }

}