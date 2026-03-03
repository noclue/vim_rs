//! # Host Information Retrieval Example
//!
//! This example demonstrates how to use the `vim_retrievable!` macro to efficiently
//! retrieve comprehensive ESXi host information from vSphere.
//!
//! ## Key Concepts
//!
//! **vim_retrievable! Macro**: Generates a Rust struct that maps to vSphere managed
//! object properties. Benefits include:
//! - Type-safe property access (compile-time checking)
//! - Automatic PropertyCollector spec generation
//! - Batch retrieval of all hosts in a single API call
//! - Clean deserialization from vSphere API responses
//!
//! **Property Paths**: Properties are specified using dot notation matching the
//! vSphere object model (e.g., `"summary.quick_stats.uptime"`). The macro converts
//! these to the correct PropertyCollector filter specifications.
//!
//! **ObjectRetriever**: A high-level abstraction over PropertyCollector that:
//! - Retrieves objects from containers (folders, datacenters, clusters)
//! - Handles pagination automatically
//! - Returns strongly-typed results
//!
//! ## How It Works
//!
//! 1. Define a Host struct with `vim_retrievable!` specifying desired properties
//! 2. Create an ObjectRetriever with the vSphere client
//! 3. Call `retrieve_objects_from_container()` with the root folder
//! 4. Iterate through the returned Vec<Host> to process results
//!
//! ## Properties Retrieved
//!
//! - `overall_status`: Health status (green/yellow/red)
//! - `connection_state`: Connected, disconnected, not responding
//! - `name`: ESXi host FQDN or IP
//! - `version`: ESXi version string
//! - `cpu_usage`: Current CPU usage in MHz
//! - `memory_usage`: Current memory usage in MB
//! - `uptime`: Seconds since last boot
//!
//! ## Use Cases
//!
//! - Health monitoring and alerting
//! - Capacity planning and resource utilization reports
//! - Inventory management
//! - Compliance verification (version checks)
use anyhow::Result;
use log::info;
use std::env;
use utils::connect;
use vim_rs::vim_retrievable;
use vim_rs::core::pc_retrieve::ObjectRetriever;

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
