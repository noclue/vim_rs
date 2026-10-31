//! Live realtime CPU and memory monitor for every virtual machine, with sparklines.
//!
//! Each tick builds a [`TypedPerfQuery`](vim_rs::perf::TypedPerfQuery)`<VirtualMachine>`.
//! `metric` accepts only counters whose scope includes a virtual machine. `fetch`
//! takes ownership, so the next tick builds the query again. A failed tick is logged
//! and the loop continues. A virtual machine with no samples draws a blank row.
//! `PERF_TICKS` defaults to 3.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Error, Result};
use log::{error, warn};
use snippets::connect;
use vim_rs::core::pc_retrieve::ObjectRetriever;
use vim_rs::mo::{ContainerView, ViewManager, VirtualMachine};
use vim_rs::perf::metrics::by_entity::virtual_machine;
use vim_rs::perf::typed::Entity;
use vim_rs::perf::units::Percent;
use vim_rs::perf::{EntityStatus, Interval, PerfClient, TypedPerfQuery};
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::structs::ManagedObjectReference;
use vim_rs::vim_retrievable;

vim_retrievable!(
    struct VmCpu: VirtualMachine {
        num_cpu = "config.hardware.num_cpu",
    }
);

const GLYPHS: [char; 7] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇'];

fn perf_ticks() -> u32 {
    std::env::var("PERF_TICKS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(3)
}

fn sparkline(window: [Option<Percent>; 6]) -> String {
    window
        .into_iter()
        .map(|slot| match slot {
            None => ' ',
            Some(percent) => {
                let index = (percent.0.clamp(0.0, 100.0) * 6.0 / 100.0).round() as usize;
                GLYPHS[index]
            }
        })
        .collect()
}

async fn list_vms(client: &Arc<vim_rs::core::Client>) -> Result<Vec<Entity<VirtualMachine>>> {
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
    vms.into_iter()
        .map(|moref| {
            Entity::<VirtualMachine>::try_from(moref).map_err(|err| Error::msg(err.to_string()))
        })
        .collect()
}

fn vm_query(perf: &PerfClient, vms: &[Entity<VirtualMachine>]) -> TypedPerfQuery<VirtualMachine> {
    perf.query()
        .typed_entities(vms)
        .metric(virtual_machine::cpu::USAGE_AVERAGE)
        .metric(virtual_machine::mem::USAGE_AVERAGE)
        .metric(virtual_machine::cpu::USAGEMHZ_AVERAGE)
        .metric(virtual_machine::mem::CONSUMED_AVERAGE)
        .metric(virtual_machine::cpu::READY_SUMMATION)
        .interval(Interval::Realtime)
        .latest(6)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;
    let perf = PerfClient::new(client.clone()).map_err(|err| Error::msg(err.to_string()))?;
    let vms = list_vms(&client).await?;
    if vms.is_empty() {
        return Err(Error::msg("No virtual machines found."));
    }
    let morefs: Vec<ManagedObjectReference> = vms.iter().map(|vm| vm.as_moref().clone()).collect();

    let retriever = ObjectRetriever::new(client.clone())?;
    let cpus: Vec<VmCpu> = retriever.retrieve_objects_from_list(&morefs).await?;
    let vcpus: HashMap<String, u32> = cpus
        .into_iter()
        .map(|vm| (vm.id.value, vm.num_cpu.unwrap_or(0) as u32))
        .collect();

    let period = match perf.realtime_refresh_rate(vms[0].as_moref()).await {
        Ok(period) => period,
        Err(err) => {
            warn!("realtime refresh rate unavailable ({err}); using 20s");
            Duration::from_secs(20)
        }
    };
    let mut ticker = tokio::time::interval(period);
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    println!("VM  CPU%  MEM%  MHz  Consumed  Ready%");
    for _tick in 0..perf_ticks() {
        ticker.tick().await;
        match vm_query(&perf, &vms).fetch().await {
            Err(err) => error!("perf tick failed: {err}"),
            Ok(result) => {
                for entity in result.entities() {
                    match &entity.status {
                        EntityStatus::Ok => {
                            let cpu = entity
                                .series(virtual_machine::cpu::USAGE_AVERAGE)
                                .map(|series| sparkline(series.scaled().window::<6>()))
                                .unwrap_or_else(|| "      ".into());
                            let mem = entity
                                .series(virtual_machine::mem::USAGE_AVERAGE)
                                .map(|series| sparkline(series.scaled().window::<6>()))
                                .unwrap_or_else(|| "      ".into());
                            let mhz = entity
                                .series(virtual_machine::cpu::USAGEMHZ_AVERAGE)
                                .and_then(|series| series.scaled().last_slot())
                                .map(|sample| format!("{:.0} MHz", sample.value.as_mhz()))
                                .unwrap_or_default();
                            let consumed = entity
                                .series(virtual_machine::mem::CONSUMED_AVERAGE)
                                .and_then(|series| series.scaled().last_slot())
                                .map(|sample| sample.value.to_string())
                                .unwrap_or_default();
                            let ready = entity
                                .series(virtual_machine::cpu::READY_SUMMATION)
                                .and_then(|series| {
                                    let vcpu =
                                        vcpus.get(&entity.entity.value).copied().unwrap_or(0);
                                    series.ready_percent(vcpu).last().copied().flatten()
                                })
                                .map(|percent| percent.to_string())
                                .unwrap_or_default();
                            println!(
                                "{}  {cpu}  {mem}  {mhz}  {consumed}  {ready}",
                                entity.entity.value
                            );
                        }
                        EntityStatus::NoSamples | EntityStatus::NoMetricsAvailable => {
                            println!("{}       ", entity.entity.value);
                        }
                        other => println!("{}  {other:?}", entity.entity.value),
                    }
                }
            }
        }
    }

    client.close().await?;
    Ok(())
}
