//! # Host information retrieval (`vim_retrievable!`)
//!
//! This example shows how to use the `vim_retrievable!` macro to pull a focused set of
//! [`HostSystem`](vim_rs::mo::HostSystem) properties for every host under the inventory root in one
//! PropertyCollector round-trip.
//!
//! ## What you will see
//!
//! 1. Define a `Host` struct with `vim_retrievable!` and dot-path properties matching the vSphere model.
//! 2. Build an [`ObjectRetriever`](vim_rs::core::pc_retrieve::ObjectRetriever) from the client.
//! 3. Call `retrieve_objects_from_container()` on the root folder and log each host.
//!
//! ## Retrieved properties
//!
//! - `overall_status`, `connection_state`, `name`, `version`
//! - `cpu_usage`, `memory_usage`, `uptime` from quick stats
//!
//! ## Typical uses
//!
//! Health dashboards, capacity summaries, inventory reports, and version compliance checks.

use anyhow::Result;
use log::info;
use snippets::connect;
use vim_rs::core::pc_retrieve::ObjectRetriever;
use vim_rs::vim_retrievable;

vim_retrievable!(
    struct Host: HostSystem {
        overall_status = "summary.overall_status",
        connection_state = "runtime.connection_state",
        name = "name",
        version = "config.product.version",
        cpu_usage = "summary.quick_stats.overall_cpu_usage",
        memory_usage = "summary.quick_stats.overall_memory_usage",
        uptime = "summary.quick_stats.uptime",
    }
);

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;
    let retriever = ObjectRetriever::new(client.clone())?;
    let hosts: Vec<Host> = retriever
        .retrieve_objects_from_container(&client.service_content().root_folder)
        .await?;

    for host in hosts {
        info!(
            "Host ({}): {}, {:?}, {:?}, {:?}",
            host.id.value, host.name, host.connection_state, host.overall_status, host.version
        );
    }

    Ok(())
}
