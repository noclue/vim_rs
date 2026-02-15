//! # Environment Browser Example
//!
//! This example demonstrates how to use the vSphere EnvironmentBrowser interface to
//! retrieve configuration options available in a specific compute environment.
//!
//! The EnvironmentBrowser provides metadata about the capabilities and constraints
//! of hosts within a compute resource, which is essential for creating properly
//! configured virtual machines that are compatible with the target environment.
//!
//! In this example:
//! 1. We connect to a vSphere server using credentials from environment variables
//! 2. We locate a specific ComputeResource by name using PropertyCollector
//! 3. We access the EnvironmentBrowser associated with the ComputeResource
//! 4. We query configuration option descriptors to find the default descriptor
//! 5. We retrieve detailed configuration options for a specific host
//!
//! This information is valuable when you need to programmatically determine what
//! virtual hardware configurations are supported before deploying or reconfiguring
//! virtual machines in a specific environment.

use std::env;
use vim_rs::mo::{ComputeResource, EnvironmentBrowser};

use anyhow::{Context, Result};
use log::{error, info};
use utils::connect;
use vim_macros::vim_retrievable;
use vim_rs::core::pc_retrieve::ObjectRetriever;

vim_retrievable!(
    struct Compute: ComputeResource {
        name = "name",
    }
);

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;
    let compute_resource =
        env::var("COMPUTE_RESOURCE").with_context(|| "COMPUTE_RESOURCE env var not set")?;

    let retriever = ObjectRetriever::new(client.clone())?;
    let compute_resources: Vec<Compute> = retriever
        .retrieve_objects_from_container(&client.service_content().root_folder)
        .await?;

    let mut cr_moref: Option<String> = None;
    for obj in compute_resources {
        if obj.name == compute_resource {
            cr_moref = Some(obj.id.value.clone());
            info!("Found ComputeResource: {} -> {}", obj.name, obj.id.value);
        } else {
            info!("{name}  ->  {mo_id}", name = obj.name, mo_id = obj.id.value);
        }
    }

    let Some(cr_moref) = cr_moref else {
        error!("ComputeResource not found");
        return Err(anyhow::anyhow!("ComputeResource not found"));
    };
    let cr = ComputeResource::new(client.clone(), &cr_moref);

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

    let mut config_option_descriptor = None;
    for desc in &cod {
        if desc.default_config_option {
            config_option_descriptor = Some(desc);
            info!("Found Default ConfigOption: {} - {:?}", desc.key, desc);
        }
        info!("Key: {} -> {:?}", desc.key, desc);
    }
    let Some(config_option_descriptor) = config_option_descriptor else {
        error!("Default ConfigOption not found");
        return Err(anyhow::anyhow!("Default ConfigOption not found"));
    };

    let Some(ref host) = config_option_descriptor.host else {
        error!("Config Option Descriptor Host not set");
        return Err(anyhow::anyhow!("Host not found"));
    };
    let Some(first_host) = host.first() else {
        error!("No hosts set for default config option");
        return Err(anyhow::anyhow!("No hosts set for default config option"));
    };
    let cfg_option = eb
        .query_config_option(Some(&config_option_descriptor.key), Some(first_host))
        .await?;

    info!("VM Config Option: {:?}", cfg_option);
    Ok(())
}
