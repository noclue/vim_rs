//! Read-only study: is `ComputeResource` (a standalone host's compute resource) a perf provider?
//!
//! For every exact `ComputeResource`, plus one `ClusterComputeResource` and its first host for
//! contrast, records the provider summary, availability at realtime and each enabled historical
//! interval, and explicit `QueryPerf` results for a few aggregate counters. It also sends a
//! fabricated `ComputeResource` moref to see whether a stale id faults or is omitted.
//!
//! ```text
//! perf-catalog-compute-resource --env-file ../vc.env --target vcenter --transport json \
//!     --out out/probes/compute-resource-vcenter-json.json
//! ```

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{bail, Context, Result};
use perf_catalog::catalog::{
    counter_records, fetch_counters, fetch_intervals, interval_records, CounterRecord,
    IntervalRecord,
};
use perf_catalog::ledger::write_json;
use perf_catalog::query::{fault_type_name, fault_value, metric_id, query_perf, PerfCall};
use perf_catalog::session::{
    apply_log_env, clone_moref, connect, load_env_file, moref_key, ConnectOptions, Session,
    TransportKind,
};
use serde_json::{json, Value};
use vim_rs::core::client::Client;
use vim_rs::core::pc_retrieve::ObjectRetriever;
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::structs::{ManagedObjectReference, PerfQuerySpec};
use vim_rs::vim_retrievable;

vim_retrievable!(
    struct ComputeInfo: ComputeResource {
        name = "name",
        hosts = "host",
        num_hosts = "summary.num_hosts",
        effective_cpu = "summary.effective_cpu",
    }
);

/// Aggregate counters a compute resource could plausibly carry, `group.name.rollup`.
const COUNTERS: &[&str] = &[
    "cpu.usagemhz.average",
    "cpu.totalmhz.average",
    "mem.consumed.average",
    "mem.usage.average",
    "clusterServices.effectivecpu.average",
    "clusterServices.effectivemem.average",
];

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}

struct Args {
    env_file: PathBuf,
    target: String,
    transport: TransportKind,
    out: PathBuf,
}

async fn run() -> Result<()> {
    let args = parse_args()?;
    let env = load_env_file(&args.env_file)?;
    apply_log_env(&env);
    let _ = env_logger::try_init();
    let session = connect(&ConnectOptions {
        env_file: args.env_file.clone(),
        target: args.target.clone(),
        transport: args.transport,
        wire_log: false,
    })
    .await?;
    eprintln!(
        "connected to {} over {}",
        session.server,
        session.transport.as_str()
    );
    let result = execute(&session).await;
    let _ = session.client.close().await;
    let report = result?;
    write_json(&args.out, &report)?;
    eprintln!("wrote {}", args.out.display());
    Ok(())
}

async fn execute(session: &Session) -> Result<Value> {
    let about = &session.client.service_content().about;
    let counters = counter_records(&fetch_counters(session).await?);
    let intervals = interval_records(&fetch_intervals(session).await?);
    let picked: Vec<&CounterRecord> = COUNTERS
        .iter()
        .filter_map(|full| {
            counters
                .iter()
                .find(|c| format!("{}.{}.{}", c.group, c.name, c.rollup) == *full)
        })
        .collect();
    println!("counters probed:");
    for counter in &picked {
        println!(
            "  {}.{}.{} key={} level={:?}",
            counter.group, counter.name, counter.rollup, counter.key, counter.level
        );
    }

    let all = list_compute(&session.client).await?;
    let standalone: Vec<&ComputeInfo> = all
        .iter()
        .filter(|cr| cr.id.r#type == MoTypesEnum::ComputeResource)
        .collect();
    let cluster = all
        .iter()
        .find(|cr| cr.id.r#type == MoTypesEnum::ClusterComputeResource);
    println!(
        "inventory: {} ComputeResource, {} ClusterComputeResource",
        standalone.len(),
        all.len() - standalone.len()
    );

    let mut subjects = Vec::new();
    for cr in &standalone {
        subjects.push(("ComputeResource", describe(cr), clone_moref(&cr.id)));
        if let Some(host) = cr.hosts.as_ref().and_then(|hosts| hosts.first()) {
            subjects.push(("standalone host", json!({}), clone_moref(host)));
        }
    }
    if let Some(cluster) = cluster {
        subjects.push(("ClusterComputeResource", describe(cluster), clone_moref(&cluster.id)));
    }
    let fake = ManagedObjectReference {
        r#type: MoTypesEnum::ComputeResource,
        value: if about.api_type == "HostAgent" {
            "ha-compute-res-missing".into()
        } else {
            "domain-s1".into()
        },
    };
    subjects.push(("fabricated ComputeResource", json!({}), fake));

    let mut results = Vec::new();
    for (role, info, moref) in &subjects {
        println!("== {role} {}", moref_key(moref));
        results.push(study(session, role, info, moref, &picked, &intervals).await);
    }

    Ok(json!({
        "schema": "perf-catalog.compute-resource/1",
        "source": {
            "server": session.server,
            "full_name": about.full_name,
            "api_type": about.api_type,
            "api_version": about.api_version,
            "transport": session.transport.as_str(),
        },
        "captured_at": chrono::Utc::now().to_rfc3339(),
        "counters": picked.iter().map(|c| json!({
            "name": format!("{}.{}.{}", c.group, c.name, c.rollup),
            "key": c.key,
            "level": c.level,
        })).collect::<Vec<_>>(),
        "subjects": results,
    }))
}

async fn study(
    session: &Session,
    role: &str,
    info: &Value,
    moref: &ManagedObjectReference,
    counters: &[&CounterRecord],
    intervals: &[IntervalRecord],
) -> Value {
    let summary = match session.perf.query_perf_provider_summary(moref).await {
        Ok(summary) => {
            println!(
                "  summary current={} summary={} refresh={:?}",
                summary.current_supported, summary.summary_supported, summary.refresh_rate
            );
            json!({
                "current_supported": summary.current_supported,
                "summary_supported": summary.summary_supported,
                "refresh_rate": summary.refresh_rate,
            })
        }
        Err(err) => {
            let fault = fault_value(&err);
            println!("  summary fault {:?}", fault_type_name(&fault));
            json!({ "fault": fault_type_name(&fault), "error": err.to_string() })
        }
    };

    let mut periods = vec![20];
    periods.extend(
        intervals
            .iter()
            .filter(|interval| interval.enabled)
            .map(|interval| interval.sampling_period),
    );
    let mut availability = serde_json::Map::new();
    for period in &periods {
        let entry = match session
            .perf
            .query_available_perf_metric(moref, None, None, Some(*period))
            .await
        {
            Ok(listed) => {
                let listed = listed.unwrap_or_default();
                let mut keys: Vec<i32> = listed.iter().map(|m| m.counter_id).collect();
                keys.sort_unstable();
                keys.dedup();
                println!(
                    "  available {period}s: {} metrics, {} counters",
                    listed.len(),
                    keys.len()
                );
                json!({ "metrics": listed.len(), "counter_keys": keys })
            }
            Err(err) => {
                let fault = fault_value(&err);
                println!("  available {period}s: fault {:?}", fault_type_name(&fault));
                json!({ "fault": fault_type_name(&fault), "error": err.to_string() })
            }
        };
        availability.insert(period.to_string(), entry);
    }

    let ids = || {
        counters
            .iter()
            .map(|counter| metric_id(counter.key, ""))
            .collect::<Vec<_>>()
    };
    let realtime = query_perf(
        &session.perf,
        &[spec(moref, ids(), 20, Some(minutes_ago(10)), Some(3))],
    )
    .await;
    println!("  QueryPerf 20s: {}", outline(&realtime));
    let mut queries = serde_json::Map::new();
    queries.insert("20".into(), call_json(&realtime, counters));
    if let Some(historical) = intervals.iter().find(|interval| interval.enabled) {
        let period = historical.sampling_period;
        let call = query_perf(
            &session.perf,
            &[spec(moref, ids(), period, Some(minutes_ago(180)), None)],
        )
        .await;
        println!("  QueryPerf {period}s: {}", outline(&call));
        queries.insert(period.to_string(), call_json(&call, counters));
    }

    json!({
        "role": role,
        "moref": moref_key(moref),
        "info": info,
        "provider_summary": summary,
        "availability": availability,
        "query_perf": queries,
    })
}

fn spec(
    moref: &ManagedObjectReference,
    metric_ids: Vec<vim_rs::types::structs::PerfMetricId>,
    interval_id: i32,
    start_time: Option<String>,
    max_sample: Option<i32>,
) -> PerfQuerySpec {
    PerfQuerySpec {
        entity: clone_moref(moref),
        start_time,
        end_time: None,
        max_sample,
        metric_id: Some(metric_ids),
        interval_id: Some(interval_id),
        format: Some("csv".into()),
    }
}

fn minutes_ago(minutes: i64) -> String {
    perf_catalog::query::ago_minutes(minutes)
}

fn outline(call: &PerfCall) -> String {
    if let Some(fault) = &call.fault {
        return format!("fault {:?}", fault_type_name(fault));
    }
    if call.entities.is_empty() {
        return "ok, entity omitted".into();
    }
    format!("ok, {} series", call.series.len())
}

fn call_json(call: &PerfCall, counters: &[&CounterRecord]) -> Value {
    if let Some(fault) = &call.fault {
        return json!({ "fault": fault_type_name(fault), "error": call.error_text });
    }
    let series: Vec<Value> = call
        .series
        .iter()
        .map(|series| {
            let name = counters
                .iter()
                .find(|c| c.key == series.counter_id)
                .map(|c| format!("{}.{}.{}", c.group, c.name, c.rollup));
            json!({
                "counter": name,
                "counter_id": series.counter_id,
                "instance": series.instance,
                "samples": series.values.len(),
                "last": series.values.last().copied().flatten(),
            })
        })
        .collect();
    json!({
        "entities": call.entities,
        "series": series,
        "notes": call.notes,
    })
}

fn describe(cr: &ComputeInfo) -> Value {
    json!({
        "name": cr.name,
        "hosts": cr.hosts.as_ref().map(|hosts| hosts.iter().map(moref_key).collect::<Vec<_>>()),
        "num_hosts": cr.num_hosts,
        "effective_cpu": cr.effective_cpu,
    })
}

async fn list_compute(client: &Arc<Client>) -> Result<Vec<ComputeInfo>> {
    let retriever = ObjectRetriever::new(client.clone()).context("building property retriever")?;
    retriever
        .retrieve_objects_from_container::<ComputeInfo>(&client.service_content().root_folder)
        .await
        .context("retrieving compute resources")
}

fn parse_args() -> Result<Args> {
    let mut env_file = None;
    let mut target = None;
    let mut transport = TransportKind::Json;
    let mut out = None;
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        let mut value = || {
            args.next()
                .ok_or_else(|| anyhow::anyhow!("{flag} needs a value"))
        };
        match flag.as_str() {
            "--env-file" => env_file = Some(PathBuf::from(value()?)),
            "--target" => target = Some(value()?),
            "--transport" => transport = TransportKind::parse(&value()?)?,
            "--out" => out = Some(PathBuf::from(value()?)),
            other => bail!("unknown flag {other}"),
        }
    }
    let target = target.unwrap_or_else(|| "vcenter".into());
    let out = out.unwrap_or_else(|| {
        PathBuf::from(format!(
            "out/probes/compute-resource-{target}-{}.json",
            transport.as_str()
        ))
    });
    Ok(Args {
        env_file: env_file.ok_or_else(|| anyhow::anyhow!("--env-file is required"))?,
        target,
        transport,
        out,
    })
}
