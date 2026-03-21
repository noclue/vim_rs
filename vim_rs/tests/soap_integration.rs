//! End-to-end SOAP/XML integration tests against a live ESXi host.
//!
//! These tests exercise the `SoapClient` through the standard `VimClient` trait
//! and the generated MO proxy layer — the same code path applications use.
//!
//! Run with:
//!   source .govc_env && cargo test --features xml --test soap_integration -- --ignored --nocapture
//!
//! Requires GOVC_URL, GOVC_USERNAME, GOVC_PASSWORD env vars.

#![cfg(feature = "xml")]

use vim_rs::core::client::{ClientBuilder, TransportMode, VimClient};
use vim_rs::mo::{PropertyCollector, SessionManager};
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::structs::{
    ObjectSpec, PropertyFilterSpec, PropertySpec, RetrieveOptions,
    SelectionSpec, TraversalSpec,
};

fn server() -> String {
    std::env::var("GOVC_URL")
        .expect("GOVC_URL not set")
        .trim_start_matches("https://")
        .trim_end_matches('/')
        .to_string()
}

fn user() -> String {
    std::env::var("GOVC_USERNAME").expect("GOVC_USERNAME not set")
}

fn pass() -> String {
    std::env::var("GOVC_PASSWORD").expect("GOVC_PASSWORD not set")
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires live ESXi (source .govc_env)"]
async fn soap_builder_login_logout() {
    let client = ClientBuilder::new(&server())
        .insecure(true)
        .basic_authn(&user(), &pass())
        .transport(TransportMode::Soap)
        .build()
        .await
        .expect("SOAP build failed");

    let sc = client.service_content();
    eprintln!("Connected via SOAP to: {}", sc.about.full_name);
    assert!(!sc.about.full_name.is_empty());

    let sm = sc.session_manager.as_ref().expect("sessionManager");
    let sm_proxy = SessionManager::new(client.clone(), &sm.value);
    let session = sm_proxy.current_session().await.expect("current_session failed");
    assert!(session.is_some(), "expected an active session");
    eprintln!("Session user: {}", session.unwrap().user_name);

    // Explicit logout (Drop will also logout, but let's test the method)
    sm_proxy.logout().await.expect("logout failed");
    eprintln!("Logged out successfully");
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires live ESXi (source .govc_env)"]
async fn soap_retrieve_service_content() {
    let client = ClientBuilder::new(&server())
        .insecure(true)
        .transport(TransportMode::Soap)
        .build()
        .await
        .expect("SOAP build failed");

    let sc = client.service_content();
    eprintln!("rootFolder: {}", sc.root_folder.value);
    eprintln!("propertyCollector: {}", sc.property_collector.value);
    eprintln!("about.fullName: {}", sc.about.full_name);
    assert!(!sc.root_folder.value.is_empty());
    assert!(!sc.property_collector.value.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires live ESXi (source .govc_env)"]
async fn soap_property_collector_vm_list() {
    let client = ClientBuilder::new(&server())
        .insecure(true)
        .basic_authn(&user(), &pass())
        .transport(TransportMode::Soap)
        .build()
        .await
        .expect("SOAP build failed");

    let sc = client.service_content();
    let pc = PropertyCollector::new(client.clone(), &sc.property_collector.value);

    let spec = PropertyFilterSpec {
        prop_set: vec![PropertySpec {
            all: Some(false),
            path_set: Some(vec!["name".to_string()]),
            r#type: MoTypesEnum::VirtualMachine.as_str().to_string(),
        }],
        object_set: vec![ObjectSpec {
            obj: sc.root_folder.clone(),
            skip: Some(true),
            select_set: Some(vec![
                Box::new(TraversalSpec {
                    selection_spec_: SelectionSpec {
                        name: Some("visitFolders".to_string()),
                    },
                    r#type: MoTypesEnum::Folder.as_str().to_string(),
                    path: "childEntity".to_string(),
                    skip: Some(false),
                    select_set: Some(vec![
                        Box::new(SelectionSpec {
                            name: Some("visitFolders".to_string()),
                        }),
                        Box::new(SelectionSpec {
                            name: Some("dcToVmFolder".to_string()),
                        }),
                    ]),
                }),
                Box::new(TraversalSpec {
                    selection_spec_: SelectionSpec {
                        name: Some("dcToVmFolder".to_string()),
                    },
                    r#type: MoTypesEnum::Datacenter.as_str().to_string(),
                    path: "vmFolder".to_string(),
                    skip: Some(true),
                    select_set: Some(vec![Box::new(SelectionSpec {
                        name: Some("visitFolders".to_string()),
                    })]),
                }),
            ]),
        }],
        report_missing_objects_in_results: None,
    };

    let options = RetrieveOptions { max_objects: Some(100) };
    let result = pc
        .retrieve_properties_ex(&[spec], &options)
        .await
        .expect("RetrievePropertiesEx failed")
        .expect("RetrievePropertiesEx returned None");

    eprintln!("Found {} VMs via SOAP PropertyCollector", result.objects.len());
    for oc in &result.objects {
        if let Some(ref ps) = oc.prop_set {
            for dp in ps {
                eprintln!("  {} = {:?}", dp.name, dp.val);
            }
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires live ESXi (source .govc_env)"]
async fn soap_auto_mode() {
    let client = ClientBuilder::new(&server())
        .insecure(true)
        .basic_authn(&user(), &pass())
        .transport(TransportMode::Auto)
        .build()
        .await
        .expect("Auto build failed");

    let sc = client.service_content();
    eprintln!(
        "Auto mode connected to: {} (transport: {:?})",
        sc.about.full_name,
        VimClient::transport(&*client)
    );
    assert!(!sc.about.full_name.is_empty());
}
