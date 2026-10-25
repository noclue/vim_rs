//! Collect realtime performance samples for every virtual machine.
//!
//! Lists virtual machines with a container view, then asks [`vim_rs::perf::PerfClient`]
//! for the latest realtime sample of every available counter. Gaps and powered-off
//! machines stay in the result as no-samples so a caller can clear a previous row.
//!
//! `perf_metrics_manual` is the same walk written against `PerformanceManager` directly.
//! `perf_vm_usage` asks only for CPU percent and consumed memory, and prints a table.

use anyhow::{Error, Result};
use log::{debug, info};
use snippets::connect;
use vim_rs::mo::{ContainerView, ViewManager};
use vim_rs::perf::{EntityStatus, Instance, Interval, PerfClient};
use vim_rs::types::enums::MoTypesEnum;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;
    let perf = PerfClient::new(client.clone()).map_err(|err| Error::msg(err.to_string()))?;

    let Some(view_manager_moref) = client.service_content().view_manager.clone() else {
        return Err(Error::msg("View manager not found in service content."));
    };
    let view_manager = ViewManager::new(client.clone(), &view_manager_moref.value);
    let root = client.service_content().root_folder.clone();
    let view_moref = view_manager
        .create_container_view(&root, Some(&[MoTypesEnum::VirtualMachine.as_str().to_string()]), true)
        .await?;
    let view = ContainerView::new(client.clone(), &view_moref.value);
    let vms = view.view().await?.unwrap_or_default();
    view.destroy_view().await?;
    if vms.is_empty() {
        return Err(Error::msg("No virtual machines found."));
    }

    let result = perf
        .query()
        .entities(&vms)
        .all_available()
        .instances(Instance::All)
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .map_err(|err| Error::msg(err.to_string()))?;

    for entity in result.entities() {
        match &entity.status {
            EntityStatus::Ok => {
                info!("VM: {}", entity.entity.value);
                for series in &entity.series {
                    info!(
                        "VM: {} - {}[{}] - {}",
                        entity.entity.value,
                        series.counter.name.full,
                        series.instance,
                        series.display_value()
                    );
                }
            }
            EntityStatus::NoSamples | EntityStatus::NoMetricsAvailable => {
                debug!("No samples for {}", entity.entity.value);
            }
            status => debug!("VM {} ended as {status:?}", entity.entity.value),
        }
    }

    client.close().await?;
    Ok(())
}
