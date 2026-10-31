//! Collect realtime performance samples for every virtual machine.
//!
//! Lists virtual machines with [`vim_retrievable!`](vim_rs::vim_retrievable), then asks
//! [`vim_rs::perf::PerfClient`] for the latest realtime sample of every available
//! counter. Gaps and powered-off machines stay in the result as no-samples so a
//! caller can clear a previous row.
//!
//! `perf_metrics_manual` is the same walk written against `PerformanceManager` directly.

use anyhow::{Error, Result};
use log::{debug, info};
use snippets::connect;
use vim_rs::core::pc_retrieve::ObjectRetriever;
use vim_rs::perf::{EntityStatus, Instance, Interval, PerfClient};
use vim_rs::vim_retrievable;

vim_retrievable!(
    struct VmName: VirtualMachine {
        name = "name",
    }
);

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;
    let perf = PerfClient::new(client.clone()).map_err(|err| Error::msg(err.to_string()))?;
    let retriever = ObjectRetriever::new(client.clone())?;
    let mut vms: Vec<VmName> = retriever
        .retrieve_objects_from_container(&client.service_content().root_folder)
        .await?;
    vms.sort_by(|left, right| left.name.cmp(&right.name));
    if vms.is_empty() {
        return Err(Error::msg("No virtual machines found."));
    }
    let morefs: Vec<_> = vms.iter().map(|vm| vm.id.clone()).collect();

    let result = perf
        .query()
        .entities(&morefs)
        .all_available()
        .instances(Instance::All)
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .map_err(|err| Error::msg(err.to_string()))?;

    for (vm, entity) in vms.iter().zip(result.entities()) {
        match &entity.status {
            EntityStatus::Ok => {
                info!("VM: {}", vm.name);
                for series in &entity.series {
                    info!(
                        "{} - {}[{}] - {}",
                        vm.name,
                        series.counter.name.full,
                        series.instance,
                        series.display_value()
                    );
                }
            }
            EntityStatus::NoSamples | EntityStatus::NoMetricsAvailable => {
                debug!("No samples for {}", vm.name);
            }
            status => debug!("{} ended as {status:?}", vm.name),
        }
    }

    client.close().await?;
    Ok(())
}
