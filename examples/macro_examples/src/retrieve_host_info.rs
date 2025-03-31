//! # Host Information Retrieval Example
//!
//! This example demonstrates how to use the `vim_retrievable` macro to efficiently
//! retrieve host information from a vSphere environment.
//!
//! The `vim_retrievable` macro generates a struct with specified properties that
//! allows API results to be seamlessly converted into the defined structure. This
//!  simplifies the process of collecting specific properties from managed objects
//! without having to manually construct property specs and narrow the dynamically
//! typed `PropertyCollector` responses.
//!
//! In this example:
//! 1. We define a `Host` struct mapped to the vSphere `HostSystem` object type
//! 2. We specify which properties to retrieve (name, power state, etc.)
//! 3. We use `ObjectRetriever` to fetch all host objects from the root folder
//! 4. We print the information for each retrieved host
//!
//! The advantage of this approach is that it handles batched property retrieval
//! automatically, reducing API calls and improving performance.
use anyhow::Result;
use std::env;
use utils::connect;
use vim_macros::vim_retrievable;
use vim_rs::core::pc_retrieve::ObjectRetriever;

vim_retrievable!(
    struct Host: HostSystem {
        name = "name",
        power_state = "runtime.power_state",
        connected = "runtime.connection_state",
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
        println!("Host: {:?}", host)
    }

    Ok(())
}
