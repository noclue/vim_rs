//! Latest realtime throughput, IOPS, and latency for every disk on every host.
//!
//! `instances(Instance::All)` requests the aggregate series and each device
//! (`t10.ATA_…`, `naa.…`, `mpx.vmhba*`). `series()` reads the aggregate.
//! `devices()` returns one series per disk. `series_instance` reads that disk
//! on the other counters. A missing series is omitted: `disk.usage.average`
//! is the host rollup, while read, write, IOPS, and latency are per disk.
//! Historical intervals return per-disk series at statistics level 3.

use std::collections::{BTreeSet, HashMap};

use anyhow::{Error, Result};
use snippets::connect;
use vim_rs::core::pc_retrieve::ObjectRetriever;
use vim_rs::perf::entity::EntityScope;
use vim_rs::perf::metrics::by_entity::host_system::disk;
use vim_rs::perf::result::{EntitySeries, Series};
use vim_rs::perf::units::Dimension;
use vim_rs::perf::{EntityStatus, Instance, Interval, Metric, PerfClient};
use vim_rs::types::structs::ManagedObjectReference;
use vim_rs::vim_retrievable;

vim_retrievable!(
    struct HostName: HostSystem {
        name = "name",
    }
);

fn push_stat<D: Dimension>(line: &mut String, label: &str, series: Option<Series<'_, D>>) {
    let Some(text) = series
        .and_then(|series| series.scaled().last())
        .map(|sample| sample.value.to_string())
    else {
        return;
    };
    if !line.is_empty() {
        line.push_str("   ");
    }
    line.push_str(label);
    line.push(' ');
    line.push_str(&text);
}

fn collect_devices<D: Dimension, S: EntityScope>(
    names: &mut BTreeSet<String>,
    entity: &EntitySeries,
    metric: Metric<D, S>,
) {
    for device in entity.devices(metric) {
        names.insert(device.instance.as_wire_str().to_string());
    }
}

/// Device names reported by any of the counters in this sample.
fn device_names(entity: &EntitySeries) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    collect_devices(&mut names, entity, disk::USAGE_AVERAGE);
    collect_devices(&mut names, entity, disk::READ_AVERAGE);
    collect_devices(&mut names, entity, disk::WRITE_AVERAGE);
    collect_devices(&mut names, entity, disk::NUMBER_READ_AVERAGED_AVERAGE);
    collect_devices(&mut names, entity, disk::NUMBER_WRITE_AVERAGED_AVERAGE);
    collect_devices(&mut names, entity, disk::DEVICE_READ_LATENCY_AVERAGE);
    collect_devices(&mut names, entity, disk::DEVICE_WRITE_LATENCY_AVERAGE);
    names
}

fn aggregate_line(entity: &EntitySeries) -> String {
    let mut line = String::new();
    push_stat(&mut line, "usage", entity.series(disk::USAGE_AVERAGE));
    push_stat(&mut line, "read", entity.series(disk::READ_AVERAGE));
    push_stat(&mut line, "write", entity.series(disk::WRITE_AVERAGE));
    push_stat(
        &mut line,
        "read IO/s",
        entity.series(disk::NUMBER_READ_AVERAGED_AVERAGE),
    );
    push_stat(
        &mut line,
        "write IO/s",
        entity.series(disk::NUMBER_WRITE_AVERAGED_AVERAGE),
    );
    push_stat(
        &mut line,
        "read lat",
        entity.series(disk::DEVICE_READ_LATENCY_AVERAGE),
    );
    push_stat(
        &mut line,
        "write lat",
        entity.series(disk::DEVICE_WRITE_LATENCY_AVERAGE),
    );
    line
}

fn device_line(entity: &EntitySeries, instance: &Instance) -> String {
    let mut line = String::new();
    push_stat(
        &mut line,
        "usage",
        entity.series_instance(disk::USAGE_AVERAGE, instance),
    );
    push_stat(
        &mut line,
        "read",
        entity.series_instance(disk::READ_AVERAGE, instance),
    );
    push_stat(
        &mut line,
        "write",
        entity.series_instance(disk::WRITE_AVERAGE, instance),
    );
    push_stat(
        &mut line,
        "read IO/s",
        entity.series_instance(disk::NUMBER_READ_AVERAGED_AVERAGE, instance),
    );
    push_stat(
        &mut line,
        "write IO/s",
        entity.series_instance(disk::NUMBER_WRITE_AVERAGED_AVERAGE, instance),
    );
    push_stat(
        &mut line,
        "read lat",
        entity.series_instance(disk::DEVICE_READ_LATENCY_AVERAGE, instance),
    );
    push_stat(
        &mut line,
        "write lat",
        entity.series_instance(disk::DEVICE_WRITE_LATENCY_AVERAGE, instance),
    );
    line
}

fn print_host(entity: &EntitySeries, name: &str) {
    println!("{name}  {}", entity.entity.value);
    match &entity.status {
        EntityStatus::Ok => {
            println!("  aggregate");
            let aggregate = aggregate_line(entity);
            if !aggregate.is_empty() {
                println!("    {aggregate}");
            }
            let names = device_names(entity);
            if names.is_empty() {
                println!("  (no per-disk series)");
            }
            for name in names {
                let instance = Instance::Named(name);
                println!("  {}", instance.as_wire_str());
                let stats = device_line(entity, &instance);
                if !stats.is_empty() {
                    println!("    {stats}");
                }
            }
        }
        other => println!("  {other:?}"),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;
    let perf = PerfClient::new(client.clone()).map_err(|err| Error::msg(err.to_string()))?;

    let retriever = ObjectRetriever::new(client.clone())?;
    let mut hosts: Vec<HostName> = retriever
        .retrieve_objects_from_container(&client.service_content().root_folder)
        .await?;
    if hosts.is_empty() {
        return Err(Error::msg("No hosts found."));
    }
    hosts.sort_by(|left, right| left.name.cmp(&right.name));
    let names: HashMap<String, String> = hosts
        .iter()
        .map(|host| (host.id.value.clone(), host.name.clone()))
        .collect();
    let morefs: Vec<ManagedObjectReference> = hosts.into_iter().map(|host| host.id).collect();

    let result = perf
        .query()
        .entities(&morefs)
        .metric(disk::USAGE_AVERAGE)
        .metric(disk::READ_AVERAGE)
        .metric(disk::WRITE_AVERAGE)
        .metric(disk::NUMBER_READ_AVERAGED_AVERAGE)
        .metric(disk::NUMBER_WRITE_AVERAGED_AVERAGE)
        .metric(disk::DEVICE_READ_LATENCY_AVERAGE)
        .metric(disk::DEVICE_WRITE_LATENCY_AVERAGE)
        .instances(Instance::All)
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .map_err(|err| Error::msg(err.to_string()))?;

    for entity in result.entities() {
        let name = names
            .get(&entity.entity.value)
            .map(String::as_str)
            .unwrap_or(entity.entity.value.as_str());
        print_host(entity, name);
        println!();
    }
    if !result.warnings.is_empty() {
        println!("{:?}", result.warnings);
    }

    client.close().await?;
    Ok(())
}
