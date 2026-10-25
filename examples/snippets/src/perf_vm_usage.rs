//! Latest realtime CPU, memory and disk usage for every virtual machine, printed as a table.
//!
//! Same inventory walk as `perf_metrics`: a container view of `VirtualMachine`.
//! The query is typed, so only counters whose scope includes a virtual machine
//! compile. The three counters are `cpu.usage.average` (percent),
//! `mem.consumed.average` (bytes, shown as KiB, MiB, or GiB), and
//! `disk.usage.average` (kiloBytes per second, shown as KiB/s, MiB/s, or GiB/s). Each column is 15 characters.

use anyhow::{Error, Result};
use snippets::connect;
use vim_rs::mo::{ContainerView, ViewManager, VirtualMachine};
use vim_rs::perf::entity::EntityScope;
use vim_rs::perf::metrics::by_entity::virtual_machine;
use vim_rs::perf::result::EntitySeries;
use vim_rs::perf::typed::Entity;
use vim_rs::perf::units::Dimension;
use vim_rs::perf::{Interval, Metric, PerfClient};
use vim_rs::types::enums::MoTypesEnum;

const COLUMN: usize = 15;

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
        .create_container_view(
            &root,
            Some(&[MoTypesEnum::VirtualMachine.as_str().to_string()]),
            true,
        )
        .await?;
    let view = ContainerView::new(client.clone(), &view_moref.value);
    let vms = view.view().await?.unwrap_or_default();
    view.destroy_view().await?;
    if vms.is_empty() {
        return Err(Error::msg("No virtual machines found."));
    }

    let vms = vms
        .into_iter()
        .map(|moref| {
            Entity::<VirtualMachine>::try_from(moref).map_err(|err| Error::msg(err.to_string()))
        })
        .collect::<Result<Vec<_>>>()?;

    let result = perf
        .query()
        .typed_entities(&vms)
        .metric(virtual_machine::cpu::USAGE_AVERAGE)
        .metric(virtual_machine::mem::CONSUMED_AVERAGE)
        .metric(virtual_machine::disk::USAGE_AVERAGE)
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .map_err(|err| Error::msg(err.to_string()))?;

    println!(
        "{}{}{}{}",
        column("VM ID"),
        column("CPU usage"),
        column("Memory usage"),
        column("Disk usage")
    );
    for entity in result.entities() {
        println!(
            "{}{}{}{}",
            column(&entity.entity.value),
            column(&data(entity, virtual_machine::cpu::USAGE_AVERAGE)),
            column(&data(entity, virtual_machine::mem::CONSUMED_AVERAGE)),
            column(&data(entity, virtual_machine::disk::USAGE_AVERAGE)),
        );
    }

    client.close().await?;
    Ok(())
}

/// Newest `metric`, for example `12.50 %`. `-` when the VM returned no sample.
fn data<D: Dimension, S: EntityScope>(entity: &EntitySeries, metric: Metric<D, S>) -> String {
    entity
        .series(metric)
        .and_then(|series| series.scaled().last())
        .map(|sample| sample.value.to_string())
        .unwrap_or_else(|| "-".to_string())
}

/// Left-align `text` in a 15-character column. Longer values keep their extra characters.
fn column(text: &str) -> String {
    format!("{text:<width$}", width = COLUMN)
}
