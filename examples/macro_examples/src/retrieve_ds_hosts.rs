//! # Retrieve Datastore Hosts Example
//!
//! This example demonstrates how to find all ESXi hosts that have access to a specific
//! datastore and retrieve detailed information about those hosts.
//!
//! ## Key Concepts
//!
//! **Datastore-Host Relationship**: In vSphere, datastores can be accessed by multiple
//! hosts. The `Datastore.host` property returns an array of `DatastoreHostMount` objects
//! that describe which hosts have mounted the datastore.
//!
//! **Two-Step Retrieval Pattern**:
//! 1. First, use a managed object method (`datastore.host()`) to get references
//! 2. Then, use PropertyCollector (`retrieve_objects_from_list`) to fetch properties
//!
//! This is more efficient than calling methods for each host individually.
//!
//! **vim_retrievable! for Specific Properties**: Instead of fetching all host properties,
//! we define exactly which ones we need (name, connection state, version).
//!
//! ## How It Works
//!
//! 1. Read the DATASTORE environment variable (e.g., "datastore-107001")
//! 2. Create a Datastore managed object proxy
//! 3. Call `datastore.host()` to get all DatastoreHostMount objects
//! 4. Extract the ManagedObjectReference for each host
//! 5. Use ObjectRetriever to fetch all host properties in one efficient call
//! 6. Display host information (name, connection state, ESXi version)
//!
//! ## Use Cases
//!
//! - Storage capacity planning (which hosts can access which storage)
//! - Maintenance planning (finding hosts that need to be migrated before storage maintenance)
//! - Troubleshooting storage connectivity issues
//! - Validating datastore accessibility across a cluster
//!
//! ## Environment Variables Required
//!
//! - `DATASTORE`: Managed object ID of the datastore (e.g., "datastore-107001")

use anyhow::Context;
use log::info;
use std::env;
use utils::connect;
use vim_macros::vim_retrievable;
use vim_rs::core::pc_retrieve::ObjectRetriever;
use vim_rs::mo::Datastore;

vim_retrievable!(
    struct Host: HostSystem {
        name = "name",
        connection_state = "runtime.connection_state",
        version = "config.product.version",
    }
);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let ds_moref = env::var("DATASTORE").with_context(
        || "DATASTORE env var not set. It should be a valid datastore id like 'datastore-107001'",
    )?;
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;

    let datastore = Datastore::new(client.clone(), &ds_moref);

    let Some(host_mounts) = datastore.host().await? else {
        return Err(anyhow::anyhow!("No hosts found for datastore {}", ds_moref));
    };
    let hosts = host_mounts
        .into_iter()
        .map(|host| host.key)
        .collect::<Vec<_>>();

    let retriever = ObjectRetriever::new(client.clone())?;
    let hosts: Vec<Host> = retriever.retrieve_objects_from_list(&hosts).await?;

    for host in hosts {
        info!(
            "Host ({}): {}, {:?}, {:?}",
            host.id.value, host.name, host.connection_state, host.version
        );
    }

    Ok(())
}

