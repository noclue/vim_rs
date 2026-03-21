//! # Datastore-attached hosts (`vim_retrievable!`)
//!
//! Lists ESXi hosts that have access to a given datastore: first resolve mounts via the
//! [`Datastore::host`](vim_rs::mo::Datastore) property, then batch-fetch host fields with
//! `vim_retrievable!` and [`ObjectRetriever::retrieve_objects_from_list`](vim_rs::core::pc_retrieve::ObjectRetriever::retrieve_objects_from_list).
//!
//! ## Environment
//!
//! - `DATASTORE` — managed object id (e.g. `datastore-107001`)
//!
//! ## Typical uses
//!
//! Storage maintenance planning, connectivity troubleshooting, and “which hosts see this LUN?” reporting.

use anyhow::Context;
use log::info;
use std::env;
use snippets::connect;
use vim_rs::core::pc_retrieve::ObjectRetriever;
use vim_rs::mo::Datastore;
use vim_rs::vim_retrievable;

vim_retrievable!(
    struct Host: HostSystem {
        name = "name",
        connection_state = "runtime.connection_state",
        version = "config.product.version",
    }
);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
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
