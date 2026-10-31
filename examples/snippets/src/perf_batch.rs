//! One `QueryPerf` for virtual-machine CPU and host memory.
//!
//! [`PerfBatch`](vim_rs::perf::PerfBatch) plans each group on its own, then
//! packs the specs into as few `QueryPerf` calls as the metric-id budget allows.
//! [`typed_group`](vim_rs::perf::PerfBatch::typed_group) locks a group to one
//! managed-object kind, so `metric` accepts only counters whose scope includes
//! that kind. Virtual machines are asked for CPU. Hosts are asked for memory.

use anyhow::{Error, Result};
use snippets::connect;
use vim_rs::core::pc_retrieve::ObjectRetriever;
use vim_rs::mo::{HostSystem, VirtualMachine};
use vim_rs::perf::metrics::by_entity::{host_system, virtual_machine};
use vim_rs::perf::result::EntitySeries;
use vim_rs::perf::typed::Entity;
use vim_rs::perf::{EntityStatus, Interval, PerfClient};
use vim_rs::vim_retrievable;

vim_retrievable!(
    struct HostName: HostSystem {
        name = "name",
    }
);

vim_retrievable!(
    struct VmName: VirtualMachine {
        name = "name",
    }
);

fn print_status(name: &str, entity: &EntitySeries) {
    println!("{name}  {}", entity.entity.value);
    match &entity.status {
        EntityStatus::Ok => {}
        other => println!("  {other:?}"),
    }
}

fn print_vm(name: &str, entity: &EntitySeries) {
    print_status(name, entity);
    if !matches!(entity.status, EntityStatus::Ok) {
        return;
    }
    let cpu = entity
        .series(virtual_machine::cpu::USAGE_AVERAGE)
        .and_then(|series| series.scaled().last())
        .map(|sample| sample.value.to_string())
        .unwrap_or_else(|| "-".to_string());
    let mhz = entity
        .series(virtual_machine::cpu::USAGEMHZ_AVERAGE)
        .and_then(|series| series.scaled().last())
        .map(|sample| format!("{:.0} MHz", sample.value.as_mhz()))
        .unwrap_or_else(|| "-".to_string());
    println!("  cpu {cpu}   {mhz}");
}

fn print_host(name: &str, entity: &EntitySeries) {
    print_status(name, entity);
    if !matches!(entity.status, EntityStatus::Ok) {
        return;
    }
    let usage = entity
        .series(host_system::mem::USAGE_AVERAGE)
        .and_then(|series| series.scaled().last())
        .map(|sample| sample.value.to_string())
        .unwrap_or_else(|| "-".to_string());
    let consumed = entity
        .series(host_system::mem::CONSUMED_AVERAGE)
        .and_then(|series| series.scaled().last())
        .map(|sample| sample.value.to_string())
        .unwrap_or_else(|| "-".to_string());
    println!("  mem {usage}   consumed {consumed}");
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;
    let perf = PerfClient::new(client.clone()).map_err(|err| Error::msg(err.to_string()))?;
    let retriever = ObjectRetriever::new(client.clone())?;
    let root = &client.service_content().root_folder;

    let mut vms: Vec<VmName> = retriever.retrieve_objects_from_container(root).await?;
    let mut hosts: Vec<HostName> = retriever.retrieve_objects_from_container(root).await?;
    vms.sort_by(|left, right| left.name.cmp(&right.name));
    hosts.sort_by(|left, right| left.name.cmp(&right.name));
    if vms.is_empty() && hosts.is_empty() {
        return Err(Error::msg("No virtual machines or hosts found."));
    }

    let vm_entities: Vec<Entity<VirtualMachine>> = vms
        .iter()
        .map(|vm| Entity::try_from(vm.id.clone()).map_err(|err| Error::msg(err.to_string())))
        .collect::<Result<_>>()?;
    let host_entities: Vec<Entity<HostSystem>> = hosts
        .iter()
        .map(|host| Entity::try_from(host.id.clone()).map_err(|err| Error::msg(err.to_string())))
        .collect::<Result<_>>()?;

    let mut batch = perf.batch();
    if !vm_entities.is_empty() {
        batch = batch.typed_group(|query| {
            query
                .typed_entities(&vm_entities)
                .metric(virtual_machine::cpu::USAGE_AVERAGE)
                .metric(virtual_machine::cpu::USAGEMHZ_AVERAGE)
                .interval(Interval::Realtime)
                .latest(1)
        });
    }
    if !host_entities.is_empty() {
        batch = batch.typed_group(|query| {
            query
                .typed_entities(&host_entities)
                .metric(host_system::mem::USAGE_AVERAGE)
                .metric(host_system::mem::CONSUMED_AVERAGE)
                .interval(Interval::Realtime)
                .latest(1)
        });
    }
    let result = batch
        .fetch()
        .await
        .map_err(|err| Error::msg(err.to_string()))?;

    if !vm_entities.is_empty() {
        println!("Virtual machines");
        for vm in &vms {
            match result.entity(&vm.id) {
                Some(entity) => print_vm(&vm.name, entity),
                None => println!("{}  {}\n  missing", vm.name, vm.id.value),
            }
        }
        println!();
    }
    if !host_entities.is_empty() {
        println!("Hosts");
        for host in &hosts {
            match result.entity(&host.id) {
                Some(entity) => print_host(&host.name, entity),
                None => println!("{}  {}\n  missing", host.name, host.id.value),
            }
        }
        println!();
    }
    if !result.warnings.is_empty() {
        println!("{:?}", result.warnings);
    }

    client.close().await?;
    Ok(())
}
