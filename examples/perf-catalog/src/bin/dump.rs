//! Read-only counter catalog dump. See docs/vim_rs-perf-poc.md.

use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use perf_catalog::catalog::{
    collect_providers_and_availability, counter_records, derived_from, fetch_counters,
    fetch_intervals, interval_records, source_from, Catalog, SCHEMA,
};
use perf_catalog::discover::discover;
use perf_catalog::ledger::{write_json, Ledger, LedgerRecord, Status};
use perf_catalog::session::{
    apply_log_env, connect, load_env_file, parse_mo_ref, ConnectOptions, Session, TransportKind,
};

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
    ledger: PathBuf,
    counters_only: bool,
    vm: Option<String>,
}

async fn run() -> Result<()> {
    let args = parse_args()?;
    let env = load_env_file(&args.env_file)?;
    apply_log_env(&env);
    let _ = env_logger::try_init();

    let mut session = match connect(&ConnectOptions {
        env_file: args.env_file.clone(),
        target: args.target.clone(),
        transport: args.transport,
        wire_log: false,
    })
    .await
    {
        Ok(session) => session,
        Err(err) => {
            record_connect_failure(&args, &err)?;
            return Err(err);
        }
    };
    if let Some(vm) = &args.vm {
        session.mo_ref = Some(
            parse_mo_ref(vm).ok_or_else(|| anyhow::anyhow!("--vm must be Type:value, got {vm}"))?,
        );
    }
    eprintln!(
        "connected to {} ({}) over {}",
        session.server,
        session.client.service_content().about.full_name,
        session.transport.as_str()
    );

    let result = dump(&session, &args).await;
    let _ = session.client.close().await;
    result
}

async fn dump(session: &Session, args: &Args) -> Result<()> {
    let counters_raw = fetch_counters(session).await?;
    if counters_raw.is_empty() {
        bail!("perf_counter returned an empty catalog");
    }
    let counters = counter_records(&counters_raw);
    let intervals = interval_records(&fetch_intervals(session).await?);
    eprintln!(
        "{} counters, {} historical intervals",
        counters.len(),
        intervals.len()
    );

    let (missing_types, missing_samples, providers, availability) = if args.counters_only {
        eprintln!("counters-only: skipping provider summaries and availability");
        Default::default()
    } else {
        let discovery = discover(session).await?;
        eprintln!(
            "samples: {}",
            discovery
                .samples
                .iter()
                .map(|(ty, samples)| format!("{ty}={}", samples.len()))
                .collect::<Vec<_>>()
                .join(", ")
        );
        let (providers, availability) =
            collect_providers_and_availability(session, &discovery, &intervals).await;
        (
            discovery.missing_types,
            discovery.missing_samples,
            providers,
            availability,
        )
    };

    let derived = derived_from(&counters);
    let catalog = Catalog {
        schema: SCHEMA.to_string(),
        source: source_from(session),
        missing_types,
        missing_samples,
        intervals,
        counters,
        providers,
        availability,
        derived,
    };
    write_json(&args.out, &catalog)?;
    eprintln!("wrote {}", args.out.display());

    let mut ledger = Ledger::load(&args.ledger)?;
    for record in dump_ledger(session, &catalog, args.counters_only, &args.out) {
        ledger.upsert(record);
    }
    ledger.save(&args.ledger)?;
    eprintln!("updated {}", args.ledger.display());
    Ok(())
}

fn dump_ledger(
    session: &Session,
    catalog: &Catalog,
    counters_only: bool,
    out: &PathBuf,
) -> Vec<LedgerRecord> {
    let artifact = Some(out.display().to_string());
    let target = session.target.clone();
    let transport = session.transport.as_str().to_string();
    let mut rows = Vec::new();
    let other = catalog.derived.other_enums.len();
    rows.push(row(
        "V-07",
        &target,
        &transport,
        if other == 0 {
            Status::Confirmed
        } else {
            Status::Corrected
        },
        format!(
            "{} counters; other enum values: {other}",
            catalog.counters.len()
        ),
        artifact.clone(),
    ));
    rows.push(row(
        "V-10",
        &target,
        &transport,
        if catalog.derived.counters_missing_level == 0
            && catalog.derived.counters_missing_per_device_level == 0
        {
            Status::Confirmed
        } else {
            Status::Corrected
        },
        format!(
            "missing level={}, missing per_device_level={}; changing an interval level is deferred",
            catalog.derived.counters_missing_level,
            catalog.derived.counters_missing_per_device_level
        ),
        artifact.clone(),
    ));
    rows.push(ledger_v18(&target, &transport, catalog, artifact.clone()));
    rows.push(ledger_v19(&target, &transport, catalog, artifact.clone()));
    if !counters_only {
        let types = catalog
            .availability
            .keys()
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");
        rows.push(row(
            "V-09",
            &target,
            &transport,
            Status::Inconclusive,
            format!("availability captured for: {types}; EntityScope constants are still derived later from this file"),
            artifact.clone(),
        ));
        rows.push(ledger_v12(&target, &transport, catalog, artifact.clone()));
        if session.target == "esxi" {
            let empty = catalog.intervals.is_empty();
            rows.push(row(
                "V-21",
                &target,
                &transport,
                if empty {
                    Status::Confirmed
                } else {
                    Status::Corrected
                },
                format!(
                    "historical intervals={}; providers={}; CSV and Normal QueryPerf are V-15",
                    catalog.intervals.len(),
                    catalog.providers.len()
                ),
                artifact.clone(),
            ));
        }
    }
    if session.target == "vcsim" {
        rows.push(row(
            "V-22",
            &target,
            &transport,
            Status::Confirmed,
            format!(
                "perf_counter returned {} counters; historical intervals={}; missing level={}",
                catalog.counters.len(),
                catalog.intervals.len(),
                catalog.derived.counters_missing_level
            ),
            artifact,
        ));
    }
    rows
}

fn ledger_v18(
    target: &str,
    transport: &str,
    catalog: &Catalog,
    artifact: Option<String>,
) -> LedgerRecord {
    if catalog.intervals.is_empty() {
        return row(
            "V-18",
            target,
            transport,
            Status::Na,
            "historical_interval is empty",
            artifact,
        );
    }
    let summary = catalog
        .intervals
        .iter()
        .map(|interval| format!("{}s/{}s", interval.sampling_period, interval.length))
        .collect::<Vec<_>>()
        .join(", ");
    row(
        "V-18",
        target,
        transport,
        Status::Confirmed,
        summary,
        artifact,
    )
}

fn ledger_v19(
    target: &str,
    transport: &str,
    catalog: &Catalog,
    artifact: Option<String>,
) -> LedgerRecord {
    let periods: Vec<i32> = catalog
        .intervals
        .iter()
        .filter(|interval| interval.enabled)
        .map(|interval| interval.sampling_period)
        .collect();
    if periods.is_empty() {
        return row(
            "V-19",
            target,
            transport,
            Status::Na,
            "no enabled historical intervals",
            artifact,
        );
    }
    let expected = vec![300, 1800, 7200, 86400];
    let status = if periods == expected {
        Status::Confirmed
    } else {
        Status::Corrected
    };
    row(
        "V-19",
        target,
        transport,
        status,
        format!("enabled periods {periods:?}"),
        artifact,
    )
}

fn ledger_v12(
    target: &str,
    transport: &str,
    catalog: &Catalog,
    artifact: Option<String>,
) -> LedgerRecord {
    let mut pairs = Vec::new();
    for provider in &catalog.providers {
        pairs.push(format!(
            "{} current={} refresh={:?}",
            provider.r#type, provider.current_supported, provider.refresh_rate
        ));
    }
    row(
        "V-12",
        target,
        transport,
        Status::Confirmed,
        if pairs.is_empty() {
            "no provider summaries".into()
        } else {
            pairs.join("; ")
        },
        artifact,
    )
}

fn record_connect_failure(args: &Args, err: &anyhow::Error) -> Result<()> {
    if let Some(parent) = args.out.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(
        &args.out,
        format!(
            "connection failed\ntarget={}\ntransport={}\n{err:#}\n",
            args.target,
            args.transport.as_str()
        ),
    )
    .with_context(|| format!("writing {}", args.out.display()))?;
    eprintln!("wrote connection error to {}", args.out.display());
    if args.target == "vcsim" && args.transport == TransportKind::Json {
        let mut ledger = Ledger::load(&args.ledger)?;
        ledger.upsert(row(
            "V-22",
            "vcsim",
            "json",
            Status::Na,
            format!("VI JSON login failed, as expected for vcsim: {err:#}"),
            Some(args.out.display().to_string()),
        ));
        ledger.save(&args.ledger)?;
    }
    Ok(())
}

fn row(
    id: &str,
    target: &str,
    transport: &str,
    status: Status,
    summary: impl Into<String>,
    artifact: Option<String>,
) -> LedgerRecord {
    LedgerRecord {
        id: id.to_string(),
        target: target.to_string(),
        transport: transport.to_string(),
        status,
        summary: summary.into(),
        artifact,
    }
}

fn parse_args() -> Result<Args> {
    let mut env_file = None;
    let mut target = None;
    let mut transport = None;
    let mut out = None;
    let mut ledger = PathBuf::from("out/ledger.json");
    let mut counters_only = false;
    let mut vm = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--env-file" => env_file = Some(need(&mut args, "--env-file")?),
            "--target" => target = Some(need(&mut args, "--target")?),
            "--transport" => transport = Some(need(&mut args, "--transport")?),
            "--out" => out = Some(need(&mut args, "--out")?),
            "--ledger" => ledger = PathBuf::from(need(&mut args, "--ledger")?),
            "--counters-only" => counters_only = true,
            "--vm" => vm = Some(need(&mut args, "--vm")?),
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
    Ok(Args {
        env_file: PathBuf::from(env_file.ok_or_else(|| anyhow::anyhow!("--env-file is required"))?),
        target,
        transport: TransportKind::parse(
            &transport.ok_or_else(|| anyhow::anyhow!("--transport is required"))?,
        )?,
        out: PathBuf::from(out.ok_or_else(|| anyhow::anyhow!("--out is required"))?),
        ledger,
        counters_only,
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
perf-catalog-dump --env-file ../vc.env --target vcenter --transport json --out out/catalog/vcenter-json.json

--transport is required and overrides VIM_PROTOCOL.
--counters-only skips provider summaries and QueryAvailablePerfMetric.
"
    );
}
