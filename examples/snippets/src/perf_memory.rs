//! Latest realtime memory for each host and the virtual machines on it.
//!
//! [`PerfClient::composite`](vim_rs::perf::PerfClient::composite) calls
//! `QueryPerfComposite`. One call takes a host and returns that host plus the
//! child entities the server includes, usually the virtual machines on the host.
//! A machine that is not a child of the host is absent. Each host is its own call.
//! The counters apply to both kinds: usage, consumed, active, granted, overhead,
//! and balloon.

use std::collections::HashMap;

use anyhow::{Error, Result};
use snippets::connect;
use vim_rs::core::pc_retrieve::ObjectRetriever;
use vim_rs::perf::metrics::mem;
use vim_rs::perf::result::{CompositePerfResult, EntitySeries, Series};
use vim_rs::perf::units::Dimension;
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

fn memory_line(entity: &EntitySeries) -> String {
    let mut line = String::new();
    push_stat(&mut line, "usage", entity.series(mem::USAGE_AVERAGE));
    push_stat(&mut line, "consumed", entity.series(mem::CONSUMED_AVERAGE));
    push_stat(&mut line, "active", entity.series(mem::ACTIVE_AVERAGE));
    push_stat(&mut line, "granted", entity.series(mem::GRANTED_AVERAGE));
    push_stat(&mut line, "overhead", entity.series(mem::OVERHEAD_AVERAGE));
    push_stat(&mut line, "balloon", entity.series(mem::VMMEMCTL_AVERAGE));
    line
}

fn print_entity(entity: &EntitySeries, name: &str, indent: &str) {
    println!("{indent}{name}  {}", entity.entity.value);
    match &entity.status {
        EntityStatus::Ok => {
            let line = memory_line(entity);
            if !line.is_empty() {
                println!("{indent}  {line}");
            }
        }
        other => println!("{indent}  {other:?}"),
    }
}

fn print_host(name: &str, names: &HashMap<String, String>, result: &CompositePerfResult) {
    match &result.host {
        Some(host) => print_entity(host, name, ""),
        None => println!("{name}\n  (server omitted the host)"),
    }
    let mut children: Vec<&EntitySeries> = result.children.values().collect();
    children.sort_by(|left, right| {
        let left_name = names
            .get(&left.entity.value)
            .map(String::as_str)
            .unwrap_or(left.entity.value.as_str());
        let right_name = names
            .get(&right.entity.value)
            .map(String::as_str)
            .unwrap_or(right.entity.value.as_str());
        left_name
            .cmp(right_name)
            .then_with(|| left.entity.value.cmp(&right.entity.value))
    });
    if children.is_empty() {
        println!("  (no virtual machines)");
        return;
    }
    for child in children {
        let child_name = names
            .get(&child.entity.value)
            .map(String::as_str)
            .unwrap_or(child.entity.value.as_str());
        print_entity(child, child_name, "  ");
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;
    let perf = PerfClient::new(client.clone()).map_err(|err| Error::msg(err.to_string()))?;

    let retriever = ObjectRetriever::new(client.clone())?;
    let root = &client.service_content().root_folder;
    let mut hosts: Vec<HostName> = retriever.retrieve_objects_from_container(root).await?;
    let vms: Vec<VmName> = retriever.retrieve_objects_from_container(root).await?;
    if hosts.is_empty() {
        return Err(Error::msg("No hosts found."));
    }
    hosts.sort_by(|left, right| left.name.cmp(&right.name));
    let names: HashMap<String, String> = vms.into_iter().map(|vm| (vm.id.value, vm.name)).collect();

    for host in &hosts {
        let result = perf
            .composite(&host.id)
            .metric(mem::USAGE_AVERAGE)
            .metric(mem::CONSUMED_AVERAGE)
            .metric(mem::ACTIVE_AVERAGE)
            .metric(mem::GRANTED_AVERAGE)
            .metric(mem::OVERHEAD_AVERAGE)
            .metric(mem::VMMEMCTL_AVERAGE)
            .interval(Interval::Realtime)
            .latest(1)
            .fetch()
            .await
            .map_err(|err| Error::msg(err.to_string()))?;
        print_host(&host.name, &names, &result);
        if !result.warnings.is_empty() {
            println!("  {:?}", result.warnings);
        }
        println!();
    }

    client.close().await?;
    Ok(())
}
