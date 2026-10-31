//! Past-day 5-minute CPU and memory rollups for every virtual machine, exported as tidy rows.
//!
//! Historical intervals ignore `max_sample`. `time_chunk` bounds each RPC.
//! A multi-day run uses `Interval::PAST_WEEK` with `.since_ago`.

use std::time::{Duration, SystemTime};

use anyhow::{Error, Result};
use snippets::connect;
use vim_rs::mo::{ContainerView, ViewManager};
use vim_rs::perf::metrics;
use vim_rs::perf::{Interval, PerfClient};
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

    let now = SystemTime::now();
    let start = now
        .checked_sub(Duration::from_secs(24 * 3600))
        .ok_or_else(|| Error::msg("clock is before the 24-hour window"))?;
    let res = perf
        .query()
        .entities(&vms)
        .metric(metrics::cpu::USAGE_AVERAGE)
        .metric(metrics::mem::CONSUMED_AVERAGE)
        .interval(Interval::PAST_DAY)
        .between(start, now)
        .time_chunk(Duration::from_secs(6 * 3600))
        .fetch()
        .await
        .map_err(|err| Error::msg(err.to_string()))?;

    println!("entity,counter,instance,rfc3339_ts,raw,scaled");
    for row in res.rows() {
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
    println!("{:?}", res.warnings);
    client.close().await?;
    Ok(())
}
