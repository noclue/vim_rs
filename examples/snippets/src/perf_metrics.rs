//! # VMware vSphere Performance Metrics Example
//!
//! This sample demonstrates how to collect and display performance metrics for virtual machines
//! running in a VMware vSphere environment. It's a Rust implementation inspired by the PyVmomi
//! Community Samples project.
//!
//! The example demonstrates:
//
//! 1. Connecting to a vCenter Server
//! 2. Creating a mapping between performance counter names and their IDs
//! 3. Retrieving all VMs in the inventory using ContainerView
//! 4. For each virtual machine:
//!    - Fetching available performance metrics
//!    - Getting the real-time collection interval
//!    - Querying the last hour of performance statistics
//!    - Displaying formatted metrics with counter names, instances and values
//!
//! This code provides a foundation for building monitoring tools, performance analyzers, or
//! dashboards that track VM resource utilization in vSphere environments.

// See also https://github.com/vmware/pyvmomi-community-samples/blob/ec890d5286c966ddd8fe48f4eedda2e20620610f/samples/vm_perf_example.py#L66

use std::collections::HashMap;
use std::env;
use std::ops::Deref;
use vim_rs::mo::{ContainerView, PerformanceManager, ViewManager};

use anyhow::{Error, Result};
use chrono::{Duration as ChronoDuration, Utc};
use log::{debug, info};
use utils::connect;
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::structs::{PerfEntityMetric, PerfMetricId, PerfMetricIntSeries, PerfQuerySpec};

/// Demonstrates how to fetch performance statistics for virtual machines in a vCenter server.
/// The sample first creates a mapping from performance counter descriptive names to their counter
/// ids.
/// It then fetches list of all VirtualMachine objects using a ContainerView.
/// Lastly the sample iterates over all virtual machines and for each one:
/// * discovers available performance metrics,
/// * Discovers the real time collection interval value from the performance provider summary.
/// * Queries performance statistics for the virtual machine.
/// * Iterates over the statistics and prints the counter name, instance and value.
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;

    let Some(perf_manager_moref) = client.service_content().perf_manager.clone() else {
        return Err(Error::msg(
            "Performance manager not found in service content.",
        ));
    };
    let perf_manager = PerformanceManager::new(client.clone(), &perf_manager_moref.value.clone());

    // create a mapping from performance stats to their counter ids
    // counter_info: performance stats -> counter id
    // performance stats example: cpu.usagemhz.LATEST
    // counter_id example: 6
    let mut counter_info: HashMap<i32, String> = HashMap::new();
    let counters = perf_manager.perf_counter().await?;
    let Some(counters) = counters else {
        return Err(Error::msg("No performance counters found."));
    };
    for counter in counters {
        let rollup: &'static str = counter.rollup_type.into();
        let counter_name = format!(
            "{}.{}.{}",
            counter.group_info.key,
            counter.name_info.key,
            rollup
        );
        counter_info.insert(counter.key, counter_name);
    }

    // create a list of VirtualMachine objects so that we can query them for statistics
    let Some(view_manager_moref) = client.service_content().view_manager.clone() else {
        return Err(Error::msg("View manager not found in service content."));
    };

    let view_manager = ViewManager::new(client.clone(), &view_manager_moref.value.clone());

    let root_fld = client.service_content().root_folder.clone();
    let vm_type = Into::<&str>::into(MoTypesEnum::VirtualMachine).to_string();
    let view_moref = view_manager
        .create_container_view(&root_fld, Some(&[vm_type]), true)
        .await?;
    let view = ContainerView::new(client.clone(), &view_moref.value);
    let vms = view.view().await?;
    let Some(vms) = vms else {
        return Err(Error::msg("No virtual machines found."));
    };
    view.destroy_view().await?;

    let hour_ago = Utc::now() - ChronoDuration::hours(1);
    let unknown = "<Unknown>".to_string();
    for ref vm in vms {
        debug!("Fetch stats for VM: {}", vm.value);
        let Some(vm_metrics) = perf_manager
            .query_available_perf_metric(vm, Some(hour_ago.to_rfc3339().as_str()), None, None)
            .await?
        else {
            debug!("No metrics found for vm: {}", vm.value);
            continue;
        };
        let summary = perf_manager.query_perf_provider_summary(vm).await?;
        //debug!("VM {} metrics: {:?}", vm.value, vm_metrics.iter().map(|m| (counter_info.get(&m.counter_id).unwrap_or(&unknown), &m.instance) ).collect::<Vec<_>>());

        if vm_metrics.len() < 20 {
            debug!(
                "Skipping. Not enough metrics (<20) found for vm: {}. Likely powered off.",
                vm.value
            );
            continue;
        }

        let metric_ids = vm_metrics
            .iter()
            .map(|metric| PerfMetricId {
                counter_id: metric.counter_id,
                instance: "*".to_string(),
            })
            .collect();

        let spec = PerfQuerySpec {
            entity: vm.clone(),
            metric_id: Some(metric_ids),
            max_sample: Some(1),
            start_time: None,
            end_time: None,
            interval_id: summary.refresh_rate, // Use the most detailed collection level for real-time stats
            format: None,
        };
        let stats = perf_manager.query_perf(&[spec]).await?;
        let Some(stats) = stats else {
            debug!("No stats found for vm: {}", vm.value);
            continue;
        };
        if stats.is_empty() {
            debug!("Empty stat list found for vm: {}", vm.value);
            continue;
        }
        for stat in stats {
            let Some(stat) = stat.deref().as_any_ref().downcast_ref::<PerfEntityMetric>() else {
                debug!(
                    "Stat not in expected format found for: {}",
                    stat.entity.value
                );
                continue;
            };
            let Some(ref stat_value) = stat.value else {
                debug!("Stat value not found in: {:?}", stat);
                continue;
            };

            if stat_value.is_empty() {
                debug!("No stat values found for: {}", stat.entity.value);
                continue;
            }
            info!("VM: {}, stats: {} ", stat.entity.value, stat_value.len());
            for series in stat_value {
                let Some(series) = series.as_any_ref().downcast_ref::<PerfMetricIntSeries>() else {
                    debug!(
                        "Stat series not in expected format found for: {}",
                        stat.entity.value
                    );
                    continue;
                };
                let counter_name = counter_info.get(&series.id.counter_id).unwrap_or(&unknown);
                let Some(ref value) = series.value else {
                    debug!("Stat value not found in: {:?}", series);
                    continue;
                };
                // Get the first value in the series or 0 if no values are found
                let value = value.first().unwrap_or(&0);

                info!(
                    "VM: {} - {}[{}] - {}",
                    vm.value, counter_name, series.id.instance, value
                );
            }
        }
        //break;
    }
    Ok(())
}
