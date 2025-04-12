use std::env;
use anyhow::Context;
use log::info;
use vim_macros::vim_retrievable;
use vim_rs::core::pc_retrieve::ObjectRetriever;
use vim_rs::mo::Datastore;
use utils::connect;

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
    let ds_moref = env::var("DATASTORE").with_context(|| "DATASTORE env var not set. It should be a valid datastore id like 'datastore-107001'")?;
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
    let hosts: Vec<Host> = retriever
        .retrieve_objects_from_list(&hosts)
        .await?;

    for host in hosts {
        info!("Host ({}): {}, {:?}, {:?}", host.id.value, host.name, host.connection_state, host.version);
    }

    Ok(())
}