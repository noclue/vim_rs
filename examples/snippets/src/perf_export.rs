//! Incremental realtime export of VM metrics with per-entity watermarks (no duplicate rows).
//!
//! Marks are per entity, so a powered-off VM does not hold the others.
//! `fetch_incremental` restarts an entity from the earliest mark across that entity's series
//! (`entity_high_water`), and `filter_new` drops the duplicates that causes.
//! `filter_new` advances the marks before it returns the rows, so delivery is at most once:
//! rows that a crash or a failed write loses are not fetched again.
//! A real exporter refreshes inventory every few minutes. `INVENTORY_EVERY` is 2 so a short demo shows the call.
//! `PERF_TICKS` defaults to 3.

use std::collections::HashSet;
use std::sync::Arc;

use anyhow::{Error, Result};
use log::error;
use snippets::connect;
use vim_rs::mo::{ContainerView, ViewManager};
use vim_rs::perf::metrics::by_entity::virtual_machine;
use vim_rs::perf::{Interval, PerfClient, SampleWatermark};
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::structs::ManagedObjectReference;

const INVENTORY_EVERY: u32 = 2;

fn perf_ticks() -> u32 {
    std::env::var("PERF_TICKS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(3)
}

async fn list_vms(client: &Arc<vim_rs::core::Client>) -> Result<Vec<ManagedObjectReference>> {
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
    Ok(vms)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;
    let perf = PerfClient::new(client.clone()).map_err(|err| Error::msg(err.to_string()))?;
    let mut vms = list_vms(&client).await?;
    if vms.is_empty() {
        return Err(Error::msg("No virtual machines found."));
    }

    let query = perf
        .prepare()
        .metric(virtual_machine::cpu::USAGE_AVERAGE)
        .metric(virtual_machine::mem::CONSUMED_AVERAGE)
        .interval(Interval::Realtime)
        .latest(4)
        .build();
    let mut watermark = SampleWatermark::new();
    let ticks = perf_ticks();

    let mut ticker = tokio::time::interval(perf.realtime_refresh_rate(&vms[0]).await?);
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    println!("entity,counter,instance,rfc3339_ts,raw,scaled");
    for tick in 0..ticks {
        ticker.tick().await;
        if tick % INVENTORY_EVERY == 0 {
            match list_vms(&client).await {
                Ok(listed) => {
                    vms = listed;
                    let active: HashSet<ManagedObjectReference> = vms.iter().cloned().collect();
                    watermark.retain_entities(&active);
                }
                Err(err) => error!("inventory refresh failed, keeping the previous list: {err}"),
            }
        }
        if vms.is_empty() {
            error!("perf tick skipped: no virtual machines");
            continue;
        }
        match query.fetch_incremental(&vms, &watermark).await {
            Ok(res) => {
                for row in watermark.filter_new(&res) {
                    let scaled = row
                        .scaled()
                        .map(|value| value.to_string())
                        .unwrap_or_default();
                    println!(
                        "{},{},{},{},{},{scaled}",
                        row.entity.value,
                        row.counter.name.full,
                        row.instance,
                        row.ts.to_rfc3339_utc(),
                        row.raw
                    );
                }
            }
            Err(err) => error!("perf tick failed: {err}"),
        }
    }

    client.close().await?;
    Ok(())
}
