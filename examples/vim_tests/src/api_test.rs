

#[cfg(test)]
mod tests {
    use std::env;
    use vim::mo::{AlarmManager, ContainerView, PropertyCollector, ServiceInstance, SessionManager, ViewManager};
    use vim::types::structs;
    use vim::core::client::Client;

    use reqwest::ClientBuilder;
    use vim::types::structs::ManagedObjectReference;
    use vim::types::boxed_types::ValueElements;
    use vim::types::vim_any::VimAny;
    use vim::types::method_fault_trait::MethodFaultTrait;
    use vim::types::enums::MoTypesEnum;
    use log::{debug, info};

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn get_content() {
        init();
        let builder = ClientBuilder::new();
        let client = builder.danger_accept_invalid_certs(true)
                                .danger_accept_invalid_hostnames(true)
                                .build()
                                .unwrap();
        let vc_server = env::var("VC_SERVER").expect("VC_SERVER environment variable not set");
        let uri = format!("https://{vc_server}/sdk/vim25/8.0.3.0/ServiceInstance/ServiceInstance/content");
        let res = client.get(uri).send().await.unwrap();
        if res.status() != 200 {
            let fault: Box<dyn MethodFaultTrait> = res.json().await.unwrap();
            panic!("Failed to get content: {:?}", fault);
        }
        let content: structs::ServiceContent = res.json().await.unwrap();
        info!("{:?}", content.about);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn vm_inventory() {
        init();
        let client = ClientBuilder::new()
                                .danger_accept_invalid_certs(true)
                                .danger_accept_invalid_hostnames(true)
                                .build()
                                .unwrap();
        let vc_server = env::var("VC_SERVER").expect("VC_SERVER environment variable not set");
        let client = Client::new(client, &vc_server, None);
        let service_instance = ServiceInstance::new(client.clone(), "ServiceInstance");
        let content = service_instance.content().await.unwrap();
        let session_manager_mo_ref = content.session_manager.unwrap();
        let session_manager = SessionManager::new(client.clone(), &session_manager_mo_ref.value);
        let vc_username = env::var("VC_USERNAME").expect("VC_USERNAME environment variable not set");
        let vc_password = env::var("VC_PASSWORD").expect("VC_PASSWORD environment variable not set");
        let session = session_manager.login(
                            &vc_username,
                            &vc_password, 
                            None).await.unwrap();
        debug!("{:?}", session);

        let view_manager = ViewManager::new(client.clone(), &content.view_manager.unwrap().value);
        
        let view_moref = view_manager.create_container_view(
            &content.root_folder,
            Some(&vec![Into::<&str>::into(MoTypesEnum::VirtualMachine).to_string()]),
            true,
        ).await.unwrap();

        let view = ContainerView::new(client.clone(), &view_moref.value);

        let property_collector = PropertyCollector::new(client.clone(), &content.property_collector.value);

        let spec_set = vec![structs::PropertyFilterSpec {
            object_set: vec![structs::ObjectSpec {
                obj: view_moref.clone(),
                skip: Some(false),
                select_set: Some(vec![Box::new(structs::TraversalSpec {
                    name: Some("traverseEntities".to_string()), 
                    r#type: Into::<&str>::into(MoTypesEnum::ContainerView).to_string(), 
                    path: "view".to_string(), 
                    skip: Some(false), 
                    select_set: None,
                 })]),
            }],
            prop_set: vec![structs::PropertySpec {
                all: Some(false),
                path_set: Some(vec!["name".to_string()]),
                r#type: Into::<&str>::into(MoTypesEnum::VirtualMachine).to_string(),
            }],
            report_missing_objects_in_results: Some(true),
        }];
        let options = structs::RetrieveOptions {
            max_objects: Some(100),
        };
        let retrieve_result = property_collector.retrieve_properties_ex(&spec_set, &options).await.unwrap().unwrap();
        let first_vm_id: Option<String> = retrieve_result.objects.first().map(|obj| obj.obj.value.clone());
        for obj in retrieve_result.objects {
            let propset = &obj.prop_set.unwrap();
            let val = &propset.get(0).unwrap().val;
            let name = match val {
                VimAny::Value(ValueElements::PrimitiveString(s)) => &s,
                _ => "Unexpected value type",
            };

            info!("{vm_name}  ->  {mo_id}", vm_name = name, mo_id = obj.obj.value);
        }
        if retrieve_result.token.is_some() {
            let token = retrieve_result.token.unwrap();
            property_collector.cancel_retrieve_properties_ex(&token).await.unwrap();
        }
        view.destroy_view().await.unwrap();
        if let Some(vm) = first_vm_id {
            let alarm_manager_mo_ref = content.alarm_manager.unwrap();
            let alarm_manager = AlarmManager::new(client.clone(), &alarm_manager_mo_ref.value);
            let entity = ManagedObjectReference {
                r#type: MoTypesEnum::VirtualMachine,
                value: vm,
            };
            let alarm = alarm_manager.get_alarm(Some(&entity)).await.unwrap();
            debug!("Alarms for {} are: {:?}", entity.value, alarm);
        }


    }


}