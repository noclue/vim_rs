//! Read-only check: can one QueryPerf call mix interval, start, and maxSample?
//!
//! Temporary probe. Does not change inventory.

use std::collections::BTreeSet;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use perf_catalog::catalog::{counter_records, fetch_counters, fetch_intervals, interval_records};
use perf_catalog::discover::discover;
use perf_catalog::query::{metric_id, query_perf, spec_for, PerfCall};
use perf_catalog::session::{moref_key, ConnectOptions, Session, TransportKind};
use vim_rs::types::structs::{ManagedObjectReference, PerfQuerySpec};

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let env_file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.env");
    let session = perf_catalog::session::connect(&ConnectOptions {
        env_file,
        target: "vcenter".into(),
        transport: TransportKind::Json,
        wire_log: false,
    })
    .await?;
    let transport = match &session.transport {
        TransportKind::Json => "json",
        TransportKind::Soap => "soap",
    };
    eprintln!("connected to {} over {transport}", session.server);
    let result = execute(&session).await;
    let _ = session.client.close().await;
    result
}

async fn execute(session: &Session) -> Result<()> {
    let counters = counter_records(&fetch_counters(session).await?);
    let cpu = counters
        .iter()
        .find(|counter| {
            counter.group == "cpu"
                && counter.name == "usage"
                && counter.rollup == "average"
        })
        .context("cpu.usage.average missing from catalog")?;
    let intervals = interval_records(&fetch_intervals(session).await?);
    println!("counter cpu.usage.average key={}", cpu.key);
    println!("historical intervals:");
    for interval in &intervals {
        println!(
            "  period={}s name={} enabled={} length={}s level={:?}",
            interval.sampling_period, interval.name, interval.enabled, interval.length, interval.level
        );
    }

    let discovery = discover(session).await?;
    let vms: Vec<_> = discovery
        .vms
        .iter()
        .filter(|vm| vm.power_state.as_deref() == Some("poweredOn"))
        .take(2)
        .collect();
    if vms.len() < 2 {
        bail!("need two powered-on VMs, found {}", vms.len());
    }
    let host = discovery
        .samples
        .get("HostSystem")
        .and_then(|samples| samples.first())
        .context("no HostSystem")?;
    let cluster = discovery.samples.get("ClusterComputeResource").and_then(|s| s.first());

    let vm_a = &vms[0].moref;
    let vm_b = &vms[1].moref;
    let vm_caps = provider(session, vm_a).await?;
    let host_caps = provider(session, &host.moref).await?;
    println!(
        "vm {} caps current={} summary={} refresh={:?}",
        moref_key(vm_a), vm_caps.0, vm_caps.1, vm_caps.2
    );
    println!(
        "vm {} selected as second",
        moref_key(vm_b)
    );
    println!(
        "host {} caps current={} summary={} refresh={:?}",
        moref_key(&host.moref), host_caps.0, host_caps.1, host_caps.2
    );
    let refresh = vm_caps.2.filter(|rate| *rate > 0).unwrap_or(20);
    let historical = intervals
        .iter()
        .find(|interval| interval.enabled && interval.sampling_period > refresh)
        .map(|interval| interval.sampling_period)
        .unwrap_or(300);
    let coarser = intervals
        .iter()
        .find(|interval| interval.enabled && interval.sampling_period > historical)
        .map(|interval| interval.sampling_period);

    let ids = || vec![metric_id(cpu.key, "")];
    let near = perf_catalog::query::ago_minutes(15);
    let far = perf_catalog::query::ago_minutes(90);
    let slight = shift_seconds(&near, -30);

    run_case(
        session,
        "same interval, start, maxSample",
        vec![
            spec_for(vm_a, ids(), refresh, &near, 2, "csv"),
            spec_for(vm_b, ids(), refresh, &near, 2, "csv"),
        ],
    )
    .await;

    run_case(
        session,
        "same interval, starts 75 minutes apart, no maxSample",
        vec![
            spec_open(vm_a, ids(), refresh, &near, "csv"),
            spec_open(vm_b, ids(), refresh, &far, "csv"),
        ],
    )
    .await;

    run_case(
        session,
        "historical alone, maxSample 2",
        vec![spec_for(vm_a, ids(), historical, &far, 2, "csv")],
    )
    .await;

    run_case(
        session,
        "same VM, two historical windows in one call",
        vec![
            spec_bounded(vm_a, ids(), historical, &far, "2026-10-25T08:30:00Z", "csv"),
            spec_bounded(
                vm_a,
                ids(),
                historical,
                "2026-10-25T08:30:00Z",
                "2026-10-25T09:10:00Z",
                "csv",
            ),
        ],
    )
    .await;

    run_case(
        session,
        "two historical windows, no maxSample",
        vec![
            spec_open(vm_a, ids(), historical, &near, "csv"),
            spec_open(vm_b, ids(), historical, &far, "csv"),
        ],
    )
    .await;

    run_case(
        session,
        "same interval, starts 30 seconds apart",
        vec![
            spec_for(vm_a, ids(), refresh, &near, 2, "csv"),
            spec_for(vm_b, ids(), refresh, &slight, 2, "csv"),
        ],
    )
    .await;

    run_case(
        session,
        "same interval and start, maxSample 1 vs 4",
        vec![
            spec_for(vm_a, ids(), refresh, &near, 1, "csv"),
            spec_for(vm_b, ids(), refresh, &near, 4, "csv"),
        ],
    )
    .await;

    run_case(
        session,
        "two VMs, realtime vs historical",
        vec![
            spec_for(vm_a, ids(), refresh, &near, 2, "csv"),
            spec_for(vm_b, ids(), historical, &far, 2, "csv"),
        ],
    )
    .await;

    run_case(
        session,
        "VM realtime plus host historical",
        vec![
            spec_for(vm_a, ids(), refresh, &near, 2, "csv"),
            spec_for(&host.moref, ids(), historical, &far, 2, "csv"),
        ],
    )
    .await;

    if let Some(coarser) = coarser {
        run_case(
            session,
            "two VMs, two historical periods",
            vec![
                spec_for(vm_a, ids(), historical, &far, 2, "csv"),
                spec_for(vm_b, ids(), coarser, &far, 2, "csv"),
            ],
        )
        .await;
    }

    run_case(
        session,
        "same window, csv plus normal",
        vec![
            spec_for(vm_a, ids(), refresh, &near, 1, "csv"),
            spec_for(vm_b, ids(), refresh, &near, 1, "normal"),
        ],
    )
    .await;

    if let Some(cluster) = cluster {
        let caps = provider(session, &cluster.moref).await?;
        println!(
            "cluster {} caps current={} summary={} refresh={:?}",
            moref_key(&cluster.moref), caps.0, caps.1, caps.2
        );
        run_case(
            session,
            "VM realtime plus cluster historical",
            vec![
                spec_for(vm_a, ids(), refresh, &near, 2, "csv"),
                spec_for(&cluster.moref, ids(), historical, &far, 2, "csv"),
            ],
        )
        .await;
    }

    Ok(())
}

async fn provider(session: &Session, entity: &ManagedObjectReference) -> Result<(bool, bool, Option<i32>)> {
    let summary = session
        .perf
        .query_perf_provider_summary(entity)
        .await
        .with_context(|| format!("provider summary {}", moref_key(entity)))?;
    Ok((
        summary.current_supported,
        summary.summary_supported,
        summary.refresh_rate,
    ))
}

fn spec_open(
    entity: &ManagedObjectReference,
    metric_ids: Vec<vim_rs::types::structs::PerfMetricId>,
    interval_id: i32,
    start_time: &str,
    format: &str,
) -> PerfQuerySpec {
    let mut spec = spec_for(entity, metric_ids, interval_id, start_time, 1, format);
    spec.max_sample = None;
    spec
}

fn spec_bounded(
    entity: &ManagedObjectReference,
    metric_ids: Vec<vim_rs::types::structs::PerfMetricId>,
    interval_id: i32,
    start_time: &str,
    end_time: &str,
    format: &str,
) -> PerfQuerySpec {
    let mut spec = spec_open(entity, metric_ids, interval_id, start_time, format);
    spec.end_time = Some(end_time.to_string());
    spec
}

fn shift_seconds(rfc3339: &str, seconds: i64) -> String {
    let when = chrono::DateTime::parse_from_rfc3339(rfc3339).expect("start timestamp");
    (when + chrono::Duration::seconds(seconds)).to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

async fn run_case(session: &Session, name: &str, specs: Vec<PerfQuerySpec>) {
    let requested: Vec<String> = specs
        .iter()
        .map(|spec| {
            format!(
                "{} interval={} start={} max={:?} format={:?}",
                moref_key(&spec.entity),
                spec.interval_id.unwrap_or(-1),
                spec.start_time.as_deref().unwrap_or("-"),
                spec.max_sample,
                spec.format
            )
        })
        .collect();
    let call = query_perf(&session.perf, &specs).await;
    println!("\n== {name}");
    for line in &requested {
        println!("  req {line}");
    }
    print_call(&call);
}

fn print_call(call: &PerfCall) {
    if call.is_fault() {
        let kind = call
            .fault
            .as_ref()
            .and_then(perf_catalog::query::fault_type_name)
            .unwrap_or_else(|| "non-fault".into());
        println!("  FAULT {kind}");
        if let Some(text) = &call.error_text {
            let trimmed = text.lines().next().unwrap_or(text);
            println!("  {trimmed}");
        }
        return;
    }
    if call.entities.is_empty() {
        println!("  ok, no entities returned");
        return;
    }
    println!("  ok entities={}", call.entities.len());
    for item in &call.series {
        let intervals: BTreeSet<_> = item.intervals.iter().copied().collect();
        let intervals: Vec<_> = intervals.into_iter().map(|n| n.to_string()).collect();
        println!(
            "  ok {} samples={} intervals={} first={} last={}",
            item.entity,
            item.timestamps.len(),
            intervals.join(","),
            item.timestamps.first().map(String::as_str).unwrap_or("-"),
            item.timestamps.last().map(String::as_str).unwrap_or("-")
        );
    }
}
