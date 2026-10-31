//! Mix a moref that does not exist into an otherwise valid realtime query.
//!
//! Lists virtual machines with [`vim_retrievable!`](vim_rs::vim_retrievable), takes the
//! first three by name, and puts a fake `VirtualMachine` id in front of them.
//! `PERF_FAKE_VM` sets the fake id and defaults to `vm-1`.
//!
//! The fake id goes first, so the per-kind provider probe sees it before the live
//! machines. Each scenario builds a new [`PerfClient`] so the probe runs again.
//!
//! - `all_available` resolves availability per entity before `QueryPerf`.
//! - `explicit` sends `cpu.usage.average` for every entity, so the fake id shares
//!   one `QueryPerf` chunk with the live machines.
//!
//! Expected per the probe notes: the server omits the fake id without a fault, so it
//! ends as no-samples or failed on its own while the live machines stay `Ok`.
//! A whole-fetch error, or live machines marked failed, means one fault took down
//! the chunk.

use std::sync::Arc;

use anyhow::{Error, Result};
use log::info;
use snippets::connect;
use vim_rs::core::pc_retrieve::ObjectRetriever;
use vim_rs::core::Client;
use vim_rs::perf::metrics::by_entity::virtual_machine;
use vim_rs::perf::{EntityStatus, Interval, PerfClient, PerfQuery};
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::structs::ManagedObjectReference;
use vim_rs::vim_retrievable;

vim_retrievable!(
    struct VmName: VirtualMachine {
        name = "name",
    }
);

async fn run(
    client: &Arc<Client>,
    label: &str,
    morefs: &[ManagedObjectReference],
    build: impl FnOnce(PerfQuery) -> PerfQuery,
) -> Result<()> {
    let perf = PerfClient::new(client.clone()).map_err(|err| Error::msg(err.to_string()))?;
    let query = build(perf.query().entities(morefs))
        .interval(Interval::Realtime)
        .latest(1);
    match query.fetch().await {
        Ok(result) => {
            for entity in result.entities() {
                match &entity.status {
                    EntityStatus::Ok => info!(
                        "[{label}] {}: Ok, {} series",
                        entity.entity.value,
                        entity.series.len()
                    ),
                    status => info!("[{label}] {}: {status:?}", entity.entity.value),
                }
            }
            for warning in &result.warnings {
                info!("[{label}] warning: {warning:?}");
            }
        }
        Err(err) => info!("[{label}] fetch failed: {err}"),
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;
    let retriever = ObjectRetriever::new(client.clone())?;
    let mut vms: Vec<VmName> = retriever
        .retrieve_objects_from_container(&client.service_content().root_folder)
        .await?;
    vms.sort_by(|left, right| left.name.cmp(&right.name));
    if vms.is_empty() {
        return Err(Error::msg("No virtual machines found."));
    }

    let fake = ManagedObjectReference {
        r#type: MoTypesEnum::VirtualMachine,
        value: std::env::var("PERF_FAKE_VM").unwrap_or_else(|_| "vm-1".to_string()),
    };
    let live: Vec<_> = vms.iter().take(3).map(|vm| vm.id.clone()).collect();
    for vm in vms.iter().take(3) {
        info!("{} = {}", vm.id.value, vm.name);
    }
    let fake_first: Vec<_> = std::iter::once(fake.clone())
        .chain(live.iter().cloned())
        .collect();
    let fake_last: Vec<_> = live.iter().cloned().chain(std::iter::once(fake)).collect();

    for (order, morefs) in [("fake first", &fake_first), ("fake last", &fake_last)] {
        run(
            &client,
            &format!("all_available, {order}"),
            morefs,
            |query| query.all_available(),
        )
        .await?;
        run(&client, &format!("explicit, {order}"), morefs, |query| {
            query.metric(virtual_machine::cpu::USAGE_AVERAGE)
        })
        .await?;
    }

    client.close().await?;
    Ok(())
}
