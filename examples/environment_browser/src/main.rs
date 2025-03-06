use std::env;
use vim_rs::mo::{ComputeResource, ContainerView, EnvironmentBrowser, PropertyCollector, ViewManager};
use vim_rs::types::structs;
use vim_rs::core::client::ClientBuilder;

use vim_rs::types::boxed_types::ValueElements;
use vim_rs::types::vim_any::VimAny;
use vim_rs::types::enums::MoTypesEnum;
use log::{debug, info, error};
use anyhow::{Result, Context};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let vc_server = env::var("VIM_SERVER").with_context(||"VIM_SERVER env var not set")?;
    let username = env::var("VIM_USERNAME").with_context(||"VIM_USERNAME env var not set")?;
    let pwd = env::var("VIM_PASSWORD").with_context(||"VIM_PASSWORD env var not set")?;
    let compute_resource = env::var("COMPUTE_RESOURCE").with_context(||"COMPUTE_RESOURCE env var not set")?;

    let client = ClientBuilder::new(vc_server.as_str())
        .insecure(true)
        .basic_authn(username.as_str(), pwd.as_str())
        .app_details(APP_NAME, APP_VERSION)
        .build().await?;
    debug!("Connected to {}", client.service_content().about.full_name);
    let content = client.service_content();
    let view_manager = ViewManager::new(client.clone(), content.view_manager.clone().unwrap().value.as_str());
    
    let view_moref = view_manager.create_container_view(
        &content.root_folder,
        Some(&vec![Into::<&str>::into(MoTypesEnum::ComputeResource).to_string()]),
        true,
    ).await?;

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
            r#type: Into::<&str>::into(MoTypesEnum::ComputeResource).to_string(),
        }],
        report_missing_objects_in_results: Some(true),
    }];
    let options = structs::RetrieveOptions {
        max_objects: Some(100),
    };
    let retrieve_result = property_collector.retrieve_properties_ex(&spec_set, &options).await.unwrap().unwrap();
    if retrieve_result.token.is_some() {
        let token = retrieve_result.token.unwrap();
        property_collector.cancel_retrieve_properties_ex(&token).await.unwrap();
    }
    view.destroy_view().await?;

    let mut cr_moref : Option<String> = None;
    for obj in retrieve_result.objects {
        let propset = &obj.prop_set.unwrap();
        let val = &propset.get(0).unwrap().val;
        let name = match val {
            VimAny::Value(ValueElements::PrimitiveString(s)) => &s,
            _ => "Unexpected value type",
        };
        if name == compute_resource {
            cr_moref = Some(obj.obj.value.clone());
            info!("Found ComputeResource: {} -> {}", name, obj.obj.value);
        } else {
            info!("{name}  ->  {mo_id}", name = name, mo_id = obj.obj.value);
        }
    }

    let Some(cr_moref) = cr_moref else {
        error!("ComputeResource not found");
        return Err(anyhow::anyhow!("ComputeResource not found"));
    };
    let cr  = ComputeResource::new(client.clone(), &cr_moref);

    let eb = cr.environment_browser().await?;
    let Some(eb) = eb else {
        error!("EnvironmentBrowser not found");
        return Err(anyhow::anyhow!("EnvironmentBrowser not found"));
    };

    let eb = EnvironmentBrowser::new(client.clone(), &eb.value);
    let cod = eb.query_config_option_descriptor().await?;
    let Some(cod) = cod else {
        error!("ConfigOptionDescriptor not found");
        return Err(anyhow::anyhow!("ConfigOptionDescriptor not found"));
    };
    for desc in cod {
        info!("Key: {} -> {:?}", desc.key, desc);
    }

    Ok(())
}
