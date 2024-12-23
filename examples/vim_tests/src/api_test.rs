

#[cfg(test)]
mod tests {
    use std::env;
    use vim::alarm_manager::AlarmManager;
    use vim::container_view::ContainerView;
    use vim::property_collector::PropertyCollector;
    use vim::service_instance::ServiceInstance;
    use vim::session_manager::SessionManager;
    use vim::vim_client::VimClient;

    use reqwest::ClientBuilder;
    use vim::{types, view_manager};
    use vim::types::{ManagedObjectReference, ValueElements, VimAny};
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
        let uri = format!("https://{vc_server}/sdk/vim25/8.0.3.0/ServiceInstance/ServiceInstance/content", vc_server=vc_server);
        let res = client.get(uri).send().await.unwrap();
        if res.status() != 200 {
            let fault: Box<dyn types::MethodFaultTrait> = res.json().await.unwrap();
            panic!("Failed to get content: {:?}", fault);
        }
        let content: types::ServiceContent = res.json().await.unwrap();
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
        let client = VimClient::new(client, &vc_server, None);
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

        let alarm_manager_mo_ref = content.alarm_manager.unwrap();
        let alarm_manager = AlarmManager::new(client.clone(), &alarm_manager_mo_ref.value);
        let entity = types::ManagedObjectReference {
            r#type: types::MoTypesEnum::VirtualMachine,
            value: "vm-1".to_string(),
        };
        let alarm = alarm_manager.get_alarm(Some(&entity)).await.unwrap();
        debug!("{:?}", alarm);

        let view_manager = view_manager::ViewManager::new(client.clone(), &content.view_manager.unwrap().value);
        
        let view_moref = view_manager.create_container_view(
            &content.root_folder,
            Some(&vec!["VirtualMachine".to_string()]),
            true,
        ).await.unwrap();

        let view = ContainerView::new(client.clone(), &view_moref.value);

        let property_collector = PropertyCollector::new(client.clone(), &content.property_collector.value);

        let spec_set = vec![types::PropertyFilterSpec {
            object_set: vec![types::ObjectSpec {
                obj: ManagedObjectReference {
                    r#type: types::MoTypesEnum::ContainerView,
                    value: view_moref.value.clone(),
                },
                skip: Some(false),
                select_set: Some(vec![Box::new(types::TraversalSpec {
                    name: Some("traverseEntities".to_string()), 
                    r#type: "ContainerView".to_string(), 
                    path: "view".to_string(), 
                    skip: Some(false), 
                    select_set: None,
                 })]),
            }],
            prop_set: vec![types::PropertySpec {
                all: Some(false),
                path_set: Some(vec!["name".to_string()]),
                r#type: "VirtualMachine".to_string(),
            }],
            report_missing_objects_in_results: Some(true),
        }];
        let options = types::RetrieveOptions {
            max_objects: Some(100),
        };
        let retrieve_result = property_collector.retrieve_properties_ex(&spec_set, &options).await.unwrap();

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
        //session_manager.logout().await.unwrap();
        debug!("Logged out");
    }


}