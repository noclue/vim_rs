//! Read-only QueryPerf probes. See docs/vim_rs-perf-poc.md.
//!
//! Delete this binary once the ledger is filled and the fixtures have been copied.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use perf_catalog::catalog::{
    counter_records, counter_refs, fetch_counters, fetch_intervals, interval_records,
};
use perf_catalog::checks::{self, Check, Lab};
use perf_catalog::discover::discover;
use perf_catalog::ledger::{write_json, Ledger, LedgerRecord, Status};
use perf_catalog::session::{
    apply_log_env, connect, load_env_file, moref_key, parse_mo_ref, ConnectOptions, Session,
    TransportKind,
};
use serde_json::Value;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("run") => run_probes(args.collect()).await,
        Some("diff-keys") => diff_keys(args.collect()).await,
        Some("--help") | Some("-h") | None => {
            print_help();
            Ok(())
        }
        Some(other) => bail!("unknown command {other}; expected run or diff-keys"),
    }
}

struct RunArgs {
    env_file: PathBuf,
    target: String,
    transport: TransportKind,
    out_dir: PathBuf,
    only: Option<BTreeSet<String>>,
    vm: Option<String>,
}

async fn run_probes(raw: Vec<String>) -> Result<()> {
    let args = parse_run(raw)?;
    let env = load_env_file(&args.env_file)?;
    apply_log_env(&env);
    let _ = env_logger::try_init();
    let mut session = connect(&ConnectOptions {
        env_file: args.env_file.clone(),
        target: args.target.clone(),
        transport: args.transport,
        wire_log: true,
    })
    .await?;
    if let Some(vm) = &args.vm {
        session.mo_ref = Some(
            parse_mo_ref(vm).ok_or_else(|| anyhow::anyhow!("--vm must be Type:value, got {vm}"))?,
        );
    }
    eprintln!(
        "connected to {} over {}",
        session.server,
        session.transport.as_str()
    );
    let result = execute(&session, &args).await;
    let _ = session.client.close().await;
    result
}

async fn execute(session: &Session, args: &RunArgs) -> Result<()> {
    let counters = counter_refs(&counter_records(&fetch_counters(session).await?));
    if counters.is_empty() {
        bail!("perf_counter returned an empty catalog");
    }
    let intervals = interval_records(&fetch_intervals(session).await?);
    let discovery = discover(session).await?;
    let live = discovery
        .samples
        .get("VirtualMachine")
        .and_then(|samples| {
            samples
                .iter()
                .find(|sample| sample.power_state.as_deref() == Some("poweredOn"))
        })
        .ok_or_else(|| anyhow::anyhow!("no powered-on virtual machine in inventory"))?;
    let powered_off = discovery.samples.get("VirtualMachine").and_then(|samples| {
        samples
            .iter()
            .find(|sample| sample.power_state.as_deref() == Some("poweredOff"))
    });
    let host = discovery
        .samples
        .get("HostSystem")
        .and_then(|samples| samples.first());
    if let Some(vm) = &args.vm {
        let selected = moref_key(&live.moref);
        if selected != *vm {
            let state = discovery
                .vms
                .iter()
                .find(|sample| moref_key(&sample.moref) == *vm)
                .map(|sample| {
                    sample
                        .power_state
                        .clone()
                        .unwrap_or_else(|| "unknown".into())
                });
            bail!(
                "--vm {vm} was not selected as the live VM (power state {state:?}); it has to be powered on"
            );
        }
    }
    let refresh_rate = refresh_rate(session, live).await?;
    let lab = Lab {
        session,
        counters,
        intervals,
        vms: discovery.vms,
        live,
        powered_off,
        host,
        refresh_rate,
    };
    eprintln!(
        "live VM {} refresh_rate {refresh_rate}s",
        lab.live.moref.value
    );

    let want = |id: &str| {
        args.only
            .as_ref()
            .map(|set| set.contains(id))
            .unwrap_or(true)
    };
    let mut checks = Vec::new();
    if want("V-01") {
        eprintln!("V-01");
        checks.push(checks::v01(&lab).await);
    }
    if want("V-02") {
        eprintln!("V-02");
        checks.push(checks::v02(&lab).await);
    }
    if want("V-03") {
        eprintln!("V-03");
        checks.push(checks::v03(&lab).await);
    }
    if want("V-04") {
        eprintln!("V-04");
        checks.push(checks::v04(&lab).await);
    }
    if want("V-05") {
        checks.push(if args.transport == TransportKind::Json {
            eprintln!("V-05");
            checks::missing_entity(&lab, "V-05").await
        } else {
            Check::new(
                "V-05",
                Status::Na,
                "V-05 is the JSON transport; this run is SOAP",
                serde_json::json!({}),
            )
        });
    }
    if want("V-06") {
        checks.push(if args.transport == TransportKind::Soap {
            eprintln!("V-06");
            checks::missing_entity(&lab, "V-06").await
        } else {
            Check::new(
                "V-06",
                Status::Na,
                "V-06 is the SOAP transport; this run is JSON",
                serde_json::json!({}),
            )
        });
    }
    if want("V-08") {
        eprintln!("V-08");
        checks.push(checks::v08(&lab).await?);
    }
    if want("V-11") {
        eprintln!("V-11");
        checks.push(checks::v11(&lab).await);
    }
    if want("V-13") {
        eprintln!("V-13");
        checks.push(checks::v13(&lab).await);
    }
    if want("V-14") {
        eprintln!("V-14");
        checks.push(checks::v14(&lab).await);
    }
    if want("V-15") || want("V-16") {
        eprintln!("V-15 V-16");
        let (v15, v16) = checks::v15_v16(&lab).await;
        if want("V-15") {
            checks.push(v15);
        }
        if want("V-16") {
            checks.push(v16);
        }
    }
    if want("V-17") {
        eprintln!("V-17");
        checks.push(checks::v17(&lab).await);
    }
    if want("V-18") {
        checks.push(checks::v18(&lab.intervals));
    }
    if want("V-19") {
        checks.push(checks::v19(&lab.intervals, &session.target));
    }
    if args.only.is_none() {
        checks.push(Check::new(
            "V-20",
            Status::Deferred,
            "run perf-catalog-probe diff-keys on two catalog files",
            serde_json::json!({}),
        ));
        if session.target == "vcsim" {
            checks.push(Check::new(
                "V-22",
                Status::Confirmed,
                "see the vcsim dump ledger row and the probe rows from this run; JSON is a separate expected failure",
                serde_json::json!({ "refresh_rate": lab.refresh_rate }),
            ));
        } else {
            checks.push(Check::new(
                "V-22",
                Status::Na,
                "V-22 is the vcsim compatibility check",
                serde_json::json!({}),
            ));
        }
    }

    let probe_dir = args.out_dir.join("probes").join(format!(
        "{}-{}",
        session.target,
        session.transport.as_str()
    ));
    let fixture_dir = args.out_dir.join("fixtures").join(format!(
        "{}-{}",
        session.target,
        session.transport.as_str()
    ));
    let mut ledger = Ledger::load(&args.out_dir.join("ledger.json"))?;
    for check in &checks {
        let path = probe_dir.join(format!("{}.json", check.id));
        write_json(&path, &check.body)?;
        for (name, contents) in &check.fixtures {
            let fixture = fixture_dir.join(name);
            if let Some(parent) = fixture.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&fixture, contents)
                .with_context(|| format!("writing {}", fixture.display()))?;
            eprintln!("fixture {}", fixture.display());
        }
        eprintln!("{} {}", check.id, check.status.as_str());
        ledger.upsert(checks::record(
            check,
            &session.target,
            session.transport.as_str(),
            path.display().to_string(),
        ));
    }
    let ledger_path = args.out_dir.join("ledger.json");
    ledger.save(&ledger_path)?;
    eprintln!("updated {}", ledger_path.display());
    Ok(())
}

async fn refresh_rate(session: &Session, live: &perf_catalog::discover::Sample) -> Result<i32> {
    let summary = session
        .perf
        .query_perf_provider_summary(&live.moref)
        .await
        .context("QueryPerfProviderSummary")?;
    Ok(summary.refresh_rate.filter(|rate| *rate > 0).unwrap_or(20))
}

async fn diff_keys(raw: Vec<String>) -> Result<()> {
    let mut left = None;
    let mut right = None;
    let mut out = None;
    let mut ledger_path = PathBuf::from("out/ledger.json");
    let mut args = raw.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--left" => left = Some(PathBuf::from(need(&mut args, "--left")?)),
            "--right" => right = Some(PathBuf::from(need(&mut args, "--right")?)),
            "--out" => out = Some(PathBuf::from(need(&mut args, "--out")?)),
            "--ledger" => ledger_path = PathBuf::from(need(&mut args, "--ledger")?),
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            other => bail!("unknown argument {other}"),
        }
    }
    let left = left.ok_or_else(|| anyhow::anyhow!("--left is required"))?;
    let right = right.ok_or_else(|| anyhow::anyhow!("--right is required"))?;
    let out = out.unwrap_or_else(|| PathBuf::from("out/probes/V-20.json"));
    let left_map = load_counters(&left)?;
    let right_map = load_counters(&right)?;
    let mut only_left = Vec::new();
    let mut only_right = Vec::new();
    let mut key_mismatches = Vec::new();
    let mut unit_mismatches = Vec::new();
    let mut level_mismatches = Vec::new();
    for (name, left_counter) in &left_map {
        let Some(right_counter) = right_map.get(name) else {
            only_left.push(name.clone());
            continue;
        };
        if left_counter.key != right_counter.key {
            key_mismatches.push(serde_json::json!({
                "name": name,
                "left": left_counter.key,
                "right": right_counter.key,
            }));
        }
        if left_counter.unit != right_counter.unit {
            unit_mismatches.push(name.clone());
        }
        if left_counter.level != right_counter.level {
            level_mismatches.push(name.clone());
        }
    }
    for name in right_map.keys() {
        if !left_map.contains_key(name) {
            only_right.push(name.clone());
        }
    }
    let body = serde_json::json!({
        "left": left.display().to_string(),
        "right": right.display().to_string(),
        "shared": left_map.len() - only_left.len(),
        "only_left": only_left.len(),
        "only_right": only_right.len(),
        "key_mismatch_count": key_mismatches.len(),
        "key_mismatches": key_mismatches.iter().take(50).cloned().collect::<Vec<_>>(),
        "unit_mismatches": unit_mismatches,
        "level_mismatches": level_mismatches,
        "only_left_names": only_left.iter().take(50).cloned().collect::<Vec<_>>(),
        "only_right_names": only_right.iter().take(50).cloned().collect::<Vec<_>>(),
    });
    write_json(&out, &body)?;
    let status = if !key_mismatches.is_empty() {
        Status::Confirmed
    } else {
        Status::Na
    };
    let summary = format!(
        "{} shared names, {} counter-id mismatches, {} only in left, {} only in right",
        left_map.len() - only_left.len(),
        key_mismatches.len(),
        only_left.len(),
        only_right.len()
    );
    let mut ledger = Ledger::load(&ledger_path)?;
    ledger.upsert(LedgerRecord {
        id: "V-20".into(),
        target: "catalog".into(),
        transport: "diff".into(),
        status,
        summary,
        artifact: Some(out.display().to_string()),
    });
    ledger.save(&ledger_path)?;
    eprintln!("wrote {}", out.display());
    Ok(())
}

struct CounterSide {
    key: i32,
    unit: String,
    level: Option<i64>,
}

fn load_counters(path: &Path) -> Result<BTreeMap<String, CounterSide>> {
    let text =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let value: Value =
        serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
    let counters = value
        .get("counters")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow::anyhow!("{} has no counters array", path.display()))?;
    let mut map = BTreeMap::new();
    for counter in counters {
        let group = counter.get("group").and_then(Value::as_str).unwrap_or("");
        let name = counter.get("name").and_then(Value::as_str).unwrap_or("");
        let rollup = counter.get("rollup").and_then(Value::as_str).unwrap_or("");
        let key = counter.get("key").and_then(Value::as_i64).unwrap_or(0) as i32;
        let unit = counter
            .get("unit")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let level = counter.get("level").and_then(Value::as_i64);
        map.insert(
            format!("{group}.{name}.{rollup}"),
            CounterSide { key, unit, level },
        );
    }
    Ok(map)
}

fn parse_run(raw: Vec<String>) -> Result<RunArgs> {
    let mut env_file = None;
    let mut target = None;
    let mut transport = None;
    let mut out_dir = PathBuf::from("out");
    let mut only = None;
    let mut vm = None;
    let mut args = raw.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--env-file" => env_file = Some(need(&mut args, "--env-file")?),
            "--target" => target = Some(need(&mut args, "--target")?),
            "--transport" => transport = Some(need(&mut args, "--transport")?),
            "--out-dir" => out_dir = PathBuf::from(need(&mut args, "--out-dir")?),
            "--vm" => vm = Some(need(&mut args, "--vm")?),
            "--only" => {
                let value = need(&mut args, "--only")?;
                only = Some(
                    value
                        .split(',')
                        .map(|id| id.trim().to_string())
                        .filter(|id| !id.is_empty())
                        .collect(),
                );
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => bail!("unknown argument {other}"),
        }
    }
    let target = target.ok_or_else(|| anyhow::anyhow!("--target is required"))?;
    if !matches!(target.as_str(), "vcenter" | "esxi" | "vcsim") {
        bail!("--target must be vcenter, esxi, or vcsim");
    }
    Ok(RunArgs {
        env_file: PathBuf::from(env_file.ok_or_else(|| anyhow::anyhow!("--env-file is required"))?),
        target,
        transport: TransportKind::parse(
            &transport.ok_or_else(|| anyhow::anyhow!("--transport is required"))?,
        )?,
        out_dir,
        only,
        vm,
    })
}

fn need(args: &mut impl Iterator<Item = String>, flag: &str) -> Result<String> {
    args.next()
        .ok_or_else(|| anyhow::anyhow!("{flag} needs a value"))
}

fn print_help() {
    eprintln!(
        "\
perf-catalog-probe run --env-file ../.env --target esxi --transport soap --vm VirtualMachine:51
perf-catalog-probe run --env-file ../vcsim.env --target vcsim --transport soap --only V-15,V-16
perf-catalog-probe diff-keys --left out/catalog/vcenter-json.json --right out/catalog/esxi-soap.json --out out/probes/V-20-vcenter-esxi.json

Reads stay bounded: every QueryPerf sets start_time. No clone, delete, or power change.
"
    );
}
