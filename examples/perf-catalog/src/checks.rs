//! Read-only checklist probes. Nothing here clones, deletes, or powers a VM,
//! and nothing writes a vCenter option.

use anyhow::Result;
use serde_json::{json, Value};
use vim_rs::mo::OptionManager;
use vim_rs::types::structs::PerfQuerySpec;

use crate::catalog::{CounterRef, IntervalRecord};
use crate::discover::{vm_quick, Sample};
use crate::ledger::{LedgerRecord, Status};
use crate::query::{
    ago_minutes, copy_metric_ids, diff_series, fault_type_name, metric_id, query_perf, spec_for,
    unused_vm_moref, PerfCall, SentinelHistogram,
};
use crate::session::{moref_key, Session};

pub struct Lab<'a> {
    pub session: &'a Session,
    pub counters: Vec<CounterRef>,
    pub intervals: Vec<IntervalRecord>,
    pub vms: Vec<Sample>,
    pub live: &'a Sample,
    pub powered_off: Option<&'a Sample>,
    pub host: Option<&'a Sample>,
    pub refresh_rate: i32,
}

pub struct Check {
    pub id: &'static str,
    pub status: Status,
    pub summary: String,
    pub body: Value,
    /// `(relative file name under fixtures/, contents)`
    pub fixtures: Vec<(String, String)>,
}

impl Check {
    pub fn new(id: &'static str, status: Status, summary: impl Into<String>, body: Value) -> Self {
        Self {
            id,
            status,
            summary: summary.into(),
            body,
            fixtures: Vec::new(),
        }
    }
}

pub fn record(check: &Check, target: &str, transport: &str, artifact: String) -> LedgerRecord {
    LedgerRecord {
        id: check.id.to_string(),
        target: target.to_string(),
        transport: transport.to_string(),
        status: check.status.clone(),
        summary: check.summary.clone(),
        artifact: Some(artifact),
    }
}

pub async fn v01(lab: &Lab<'_>) -> Check {
    let Some(interval) = enabled_period(&lab.intervals, 300) else {
        return Check::new(
            "V-01",
            Status::Na,
            "no enabled historical interval with sampling_period 300; the option-write half is deferred",
            json!({ "mutation_deferred": true }),
        );
    };
    let mut single = Vec::new();
    let mut first_failing_single = None;
    for n in [255u32, 256, 257, 300] {
        let outcome = query_width(lab, n as usize, interval, "csv").await;
        if first_failing_single.is_none()
            && outcome.get("fault").and_then(|v| v.as_bool()) == Some(true)
        {
            first_failing_single = Some(n);
        }
        single.push(outcome);
    }
    let mut multi = Vec::new();
    let mut first_failing_multi = None;
    for m in [255u32, 256, 257, 300] {
        if lab.vms.len() < m as usize {
            multi.push(json!({
                "vms": m,
                "skipped": format!("inventory has {} VMs", lab.vms.len()),
            }));
            continue;
        }
        let specs = multi_vm_specs(lab, m as usize, interval);
        let call = query_perf(&lab.session.perf, &specs).await;
        let failed = call.is_fault();
        if first_failing_multi.is_none() && failed {
            first_failing_multi = Some(m);
        }
        multi.push(call_brief(&call, json!({ "vms": m })));
    }
    let status = match first_failing_single {
        Some(257) => Status::Confirmed,
        Some(_) => Status::Corrected,
        None if lab.counters.len() >= 300 => Status::Corrected,
        None => Status::Inconclusive,
    };
    let summary = format!(
        "first failing single-spec N={first_failing_single:?}, first failing M-VM spec={first_failing_multi:?}; setting maxQueryMetrics is deferred"
    );
    Check::new(
        "V-01",
        status,
        summary,
        json!({
            "interval_id": interval,
            "single_spec": single,
            "multi_vm": multi,
            "mutation_deferred": true,
        }),
    )
}

pub async fn v02(lab: &Lab<'_>) -> Check {
    let interval = enabled_period(&lab.intervals, 300).unwrap_or(lab.refresh_rate);
    let mut shapes = Vec::new();
    shapes.push(named_shape(lab, "one_spec_257", 257, 1, interval).await);
    shapes.push(named_shape(lab, "specs_257_by_1", 1, 257, interval).await);
    shapes.push(named_shape(lab, "specs_16_by_16", 16, 16, interval).await);
    shapes.push(named_shape(lab, "realtime_one_spec_257", 257, 1, lab.refresh_rate).await);

    let star = star_expansion(lab).await;
    let one_spec_failed = shape_failed(&shapes, "one_spec_257");
    let grid_failed = shape_failed(&shapes, "specs_16_by_16");
    let star_failed = star.get("fault").and_then(|v| v.as_bool()) == Some(true);
    let star_inconclusive = star.get("inconclusive").and_then(|v| v.as_bool()) == Some(true);
    let status = if star_inconclusive {
        Status::Inconclusive
    } else if one_spec_failed && !grid_failed && !star_failed {
        Status::Confirmed
    } else {
        Status::Corrected
    };
    Check::new(
        "V-02",
        status,
        "cost shapes recorded; confirmed only when 257-in-one-spec faults, 16x16 does not, and instance * does not fault",
        json!({ "shapes": shapes, "star": star, "historical_interval": interval }),
    )
}

pub async fn v03(lab: &Lab<'_>) -> Check {
    let Some(setting) = lab.session.client.service_content().setting.clone() else {
        return Check::new(
            "V-03",
            Status::Na,
            "ServiceContent.setting is absent (expected on a direct ESXi connection); read-only role login is deferred",
            json!({ "read_only_role_deferred": true }),
        );
    };
    let options = OptionManager::new(lab.session.client.clone(), &setting.value);
    match options
        .query_options(Some("config.vpxd.stats.maxQueryMetrics"))
        .await
    {
        Ok(Some(values)) => {
            let rendered: Vec<String> = values
                .iter()
                .map(|value| miniserde::json::to_string(&**value))
                .collect();
            let summary = format!(
                "QueryOptions returned {} value(s); read-only role login is deferred",
                rendered.len()
            );
            let status = if rendered.iter().any(|v| v.contains("256")) {
                Status::Confirmed
            } else {
                Status::Corrected
            };
            Check::new(
                "V-03",
                status,
                summary,
                json!({ "values": rendered, "read_only_role_deferred": true }),
            )
        }
        Ok(None) => Check::new(
            "V-03",
            Status::Corrected,
            "QueryOptions returned null; read-only role login is deferred",
            json!({ "read_only_role_deferred": true }),
        ),
        Err(err) => Check::new(
            "V-03",
            Status::Corrected,
            format!("QueryOptions failed: {err}; read-only role login is deferred"),
            json!({
                "fault": crate::query::fault_value(&err),
                "read_only_role_deferred": true,
            }),
        ),
    }
}

pub async fn v04(lab: &Lab<'_>) -> Check {
    let interval = enabled_period(&lab.intervals, 300).unwrap_or(lab.refresh_rate);
    let width = lab.counters.len().clamp(257, 300);
    let outcome = query_width(lab, width, interval, "csv").await;
    let fault = outcome.get("fault_body").cloned();
    let type_name = fault.as_ref().and_then(fault_type_name);
    let status = if type_name.is_some() {
        Status::Confirmed
    } else if outcome.get("fault").and_then(|v| v.as_bool()) == Some(true) {
        Status::Corrected
    } else {
        Status::Inconclusive
    };
    let summary = match &type_name {
        Some(name) => format!("over-limit fault type is {name}"),
        None => "over-limit query did not return a MethodFault".to_string(),
    };
    Check::new("V-04", status, summary, outcome)
}

pub async fn missing_entity(lab: &Lab<'_>, id: &'static str) -> Check {
    let missing = unused_vm_moref(
        &lab.vms
            .iter()
            .map(|vm| crate::session::clone_moref(&vm.moref))
            .collect::<Vec<_>>(),
    );
    let start = ago_minutes(5);
    let metric = metric_id(named_key(&lab.counters, "cpu.usage.average"), "");
    let specs = vec![
        spec_for(
            &lab.live.moref,
            vec![metric_id(metric.counter_id, &metric.instance)],
            lab.refresh_rate,
            &start,
            1,
            "csv",
        ),
        spec_for(&missing, vec![metric], lab.refresh_rate, &start, 1, "csv"),
    ];
    let call = query_perf(&lab.session.perf, &specs).await;
    let type_name = call.fault.as_ref().and_then(fault_type_name);
    let obj = call
        .fault
        .as_ref()
        .and_then(|fault| fault.get("obj"))
        .cloned();
    let whole_call_failed = call.is_fault();
    let status = if type_name.as_deref() == Some("ManagedObjectNotFound")
        && obj.is_some()
        && whole_call_failed
    {
        Status::Confirmed
    } else if call.is_fault() {
        Status::Corrected
    } else {
        Status::Corrected
    };
    let summary = format!(
        "missing {} with live {}; fault={type_name:?}; whole_call_failed={whole_call_failed}; obj present={}",
        moref_key(&missing),
        moref_key(&lab.live.moref),
        obj.is_some()
    );
    let mut check = Check::new(
        id,
        status,
        summary,
        json!({
            "missing": moref_key(&missing),
            "live": moref_key(&lab.live.moref),
            "whole_call_failed": whole_call_failed,
            "entities": call.entities,
            "fault_type": type_name,
            "obj": obj,
            "fault": call.fault,
            "notes": call.notes,
        }),
    );
    if let Some(fault) = &call.fault {
        if let Ok(text) = serde_json::to_string_pretty(fault) {
            check.fixtures.push(("missing-fault.json".into(), text));
        }
    }
    check
}

pub async fn v08(lab: &Lab<'_>) -> Result<Check> {
    let cpu = find_counter(&lab.counters, "cpu.usage.average");
    let mem = find_counter(&lab.counters, "mem.consumed.average");
    let (Some(cpu), Some(mem)) = (cpu, mem) else {
        return Ok(Check::new(
            "V-08",
            Status::Failed,
            "cpu.usage.average or mem.consumed.average is not in the counter catalog",
            json!({}),
        ));
    };
    let start = ago_minutes(10);
    let spec = spec_for(
        &lab.live.moref,
        vec![metric_id(cpu.key, ""), metric_id(mem.key, "")],
        lab.refresh_rate,
        &start,
        1,
        "normal",
    );
    let call = query_perf(&lab.session.perf, &[spec]).await;
    let quick = vm_quick(lab.session, &lab.live.moref).await;
    let cpu_raw = latest_value(&call, cpu.key);
    let mem_raw = latest_value(&call, mem.key);
    let body = match &quick {
        Ok(vm) => json!({
            "cpu_usage_average_raw": cpu_raw,
            "mem_consumed_average_raw": mem_raw,
            "overall_cpu_usage_mhz": vm.overall_cpu_usage,
            "max_cpu_usage_mhz": vm.max_cpu_usage,
            "guest_memory_usage_mb": vm.guest_memory_usage,
            "host_memory_usage_mb": vm.host_memory_usage,
            "units_note": "distinct units are in the catalog derived.units list",
        }),
        Err(err) => json!({
            "cpu_usage_average_raw": cpu_raw,
            "mem_consumed_average_raw": mem_raw,
            "quick_stats_error": err.to_string(),
        }),
    };
    let status = judge_units(cpu_raw, mem_raw, quick.as_ref().ok());
    Ok(Check::new("V-08", status.0, status.1, body))
}

pub async fn v11(lab: &Lab<'_>) -> Check {
    let mut windows = Vec::new();
    for minutes in [50i64, 65, 120] {
        let spec = spec_for(
            &lab.live.moref,
            vec![metric_id(named_key(&lab.counters, "cpu.usage.average"), "")],
            lab.refresh_rate,
            &ago_minutes(minutes),
            300,
            "normal",
        );
        let call = query_perf(&lab.session.perf, &[spec]).await;
        let oldest = call.raw_timestamps.iter().min().cloned();
        windows.push(json!({
            "start_minutes_ago": minutes,
            "oldest": oldest,
            "samples": call.raw_timestamps.len(),
            "fault": call.is_fault(),
            "error": call.error_text,
        }));
    }
    let samples_for = |minutes: i64| {
        windows
            .iter()
            .find(|window| {
                window
                    .get("start_minutes_ago")
                    .and_then(|value| value.as_i64())
                    == Some(minutes)
            })
            .and_then(|window| window.get("samples").and_then(|value| value.as_u64()))
            .unwrap_or(0)
    };
    let (s50, s65, s120) = (samples_for(50), samples_for(65), samples_for(120));
    // The speculation is realtime retention of about one hour: the 50-minute
    // window has samples and the 65- and 120-minute windows do not.
    let status = if s50 == 0 && s65 == 0 && s120 == 0 {
        Status::Inconclusive
    } else if s50 > 0 && s65 == 0 && s120 == 0 {
        Status::Confirmed
    } else {
        Status::Corrected
    };
    Check::new(
        "V-11",
        status,
        format!(
            "refresh_rate={}s; oldest timestamps are in the artifact (assumption is ~20s and ~1h retention)",
            lab.refresh_rate
        ),
        json!({ "refresh_rate": lab.refresh_rate, "windows": windows }),
    )
}

pub async fn v13(lab: &Lab<'_>) -> Check {
    let Some(off) = lab.powered_off else {
        return Check::new(
            "V-13",
            Status::Deferred,
            "no powered-off VM in inventory; power operations are deferred",
            json!({ "power_change_deferred": true }),
        );
    };
    let counter = named_key(&lab.counters, "cpu.usage.average");
    let start = ago_minutes(10);
    let mut modes = Vec::new();
    for (name, interval) in [
        ("realtime", lab.refresh_rate),
        (
            "historical",
            enabled_period(&lab.intervals, 300).unwrap_or(lab.refresh_rate),
        ),
    ] {
        let spec = spec_for(
            &off.moref,
            vec![metric_id(counter, "")],
            interval,
            &start,
            5,
            "normal",
        );
        let call = query_perf(&lab.session.perf, &[spec]).await;
        modes.push(json!({
            "mode": name,
            "interval_id": interval,
            "fault": call.is_fault(),
            "entities": call.entities,
            "series": call.series.len(),
            "values": call.series.first().map(|s| s.values.clone()),
            "error": call.error_text,
        }));
    }
    Check::new(
        "V-13",
        Status::Confirmed,
        format!(
            "powered-off {} queried; see artifact for empty vs -1 vs absent",
            moref_key(&off.moref)
        ),
        json!({ "entity": moref_key(&off.moref), "modes": modes, "recent_power_on_deferred": true }),
    )
}

pub async fn v14(lab: &Lab<'_>) -> Check {
    // The live VM is the one that was powered off and is back on. Real-time
    // covers the last hour. Historical intervals cover a day and a week when
    // this target has them, which is where an older power-off still shows up.
    let mut windows = vec![scan_window(lab, "realtime", lab.refresh_rate, 70, 200).await];
    if let Some(period) = enabled_period(&lab.intervals, 300) {
        windows.push(scan_window(lab, "historical-300", period, 26 * 60, 320).await);
    }
    if let Some(period) = enabled_period(&lab.intervals, 1800) {
        windows.push(scan_window(lab, "historical-1800", period, 8 * 24 * 60, 400).await);
    }
    let total = |field: &str| {
        windows
            .iter()
            .map(|window| window.get(field).and_then(|v| v.as_u64()).unwrap_or(0))
            .sum::<u64>()
    };
    let samples = total("samples");
    let minus_one = total("minus_one");
    let empty_gaps = total("empty_gaps");
    let time_holes = total("time_holes");
    let other_negatives: u64 = windows
        .iter()
        .map(|window| {
            window
                .get("other_negatives")
                .and_then(|v| v.as_object())
                .map(|map| map.values().filter_map(|v| v.as_u64()).sum::<u64>())
                .unwrap_or(0)
        })
        .sum();
    let entity = moref_key(&lab.live.moref);
    let (status, summary) = if other_negatives > 0 {
        (
            Status::Corrected,
            format!(
                "{entity}: negatives other than -1 are present ({other_negatives}); -1={minus_one}, empty_gaps={empty_gaps}, time_holes={time_holes}"
            ),
        )
    } else if minus_one > 0 || empty_gaps > 0 {
        (
            Status::Confirmed,
            format!(
                "{entity}: power-cycle gap is encoded in the samples (-1={minus_one}, empty_gaps={empty_gaps}, time_holes={time_holes}, values={samples})"
            ),
        )
    } else if time_holes > 0 {
        (
            Status::Corrected,
            format!(
                "{entity}: power-cycle leaves {time_holes} timestamp holes and no -1 or empty CSV fields ({samples} values)"
            ),
        )
    } else if samples == 0 {
        (
            Status::Inconclusive,
            format!("{entity}: power-cycle query returned no samples"),
        )
    } else {
        (
            Status::Inconclusive,
            format!(
                "{entity}: {samples} values, no -1, no empty fields, no timestamp holes; the off period is outside this window"
            ),
        )
    };
    Check::new(
        "V-14",
        status,
        summary,
        json!({
            "entity": entity,
            "power_cycle_deferred": false,
            "windows": windows,
        }),
    )
}

async fn scan_window(
    lab: &Lab<'_>,
    name: &str,
    interval: i32,
    minutes: i64,
    max_sample: i32,
) -> Value {
    let spec = spec_for(
        &lab.live.moref,
        sample_ids(&lab.counters, 8, ""),
        interval,
        &ago_minutes(minutes),
        max_sample,
        "csv",
    );
    let call = query_perf(&lab.session.perf, &[spec]).await;
    let mut hist = SentinelHistogram::default();
    hist.observe(&call.series);
    let holes = timestamp_holes(&call.series, i64::from(interval));
    let mut stamps: Vec<&str> = call
        .series
        .first()
        .map(|series| series.timestamps.iter().map(String::as_str).collect())
        .unwrap_or_default();
    stamps.sort();
    json!({
        "mode": name,
        "interval_id": interval,
        "start_minutes_ago": minutes,
        "max_sample": max_sample,
        "fault": call.is_fault(),
        "error": call.error_text,
        "series": call.series.len(),
        "samples": hist.samples,
        "minus_one": hist.minus_one,
        "empty_gaps": hist.empty_gaps,
        "other_negatives": hist.other_negatives,
        "time_holes": holes,
        "oldest": stamps.first().copied(),
        "newest": stamps.last().copied(),
        "examples": sentinel_examples(&call.series),
    })
}

fn timestamp_holes(series: &[crate::query::SeriesDecoded], interval_secs: i64) -> usize {
    let Some(series) = series.first() else {
        return 0;
    };
    let mut stamps = Vec::new();
    for stamp in &series.timestamps {
        if let Ok(parsed) = chrono::DateTime::parse_from_rfc3339(stamp) {
            stamps.push(parsed);
        }
    }
    stamps.sort();
    let limit = interval_secs.saturating_mul(3) / 2;
    stamps
        .windows(2)
        .filter(|pair| (pair[1] - pair[0]).num_seconds() > limit)
        .count()
}

fn sentinel_examples(series: &[crate::query::SeriesDecoded]) -> Vec<Value> {
    let mut out = Vec::new();
    for item in series {
        let Some(idx) = item
            .values
            .iter()
            .position(|value| value.is_none() || *value == Some(-1))
        else {
            continue;
        };
        let from = idx.saturating_sub(3);
        let to = (idx + 4).min(item.values.len());
        let stamp_to = to.min(item.timestamps.len());
        let stamp_from = from.min(stamp_to);
        let around: Vec<Value> = item.timestamps[stamp_from..stamp_to]
            .iter()
            .zip(item.values[from..to].iter())
            .map(|(stamp, value)| json!({ "t": stamp, "v": value }))
            .collect();
        out.push(json!({
            "counter_id": item.counter_id,
            "instance": item.instance,
            "minus_one": item.values.iter().filter(|value| **value == Some(-1)).count(),
            "empty": item.values.iter().filter(|value| value.is_none()).count(),
            "around": around,
        }));
        if out.len() == 4 {
            break;
        }
    }
    out
}

pub async fn v15_v16(lab: &Lab<'_>) -> (Check, Check) {
    let ids = sample_ids(&lab.counters, 4, "");
    let start = ago_minutes(15);
    let interval = lab.refresh_rate;
    let normal_spec = spec_for(
        &lab.live.moref,
        copy_metric_ids(&ids),
        interval,
        &start,
        5,
        "normal",
    );
    let csv_spec = spec_for(&lab.live.moref, ids, interval, &start, 5, "csv");
    let normal = query_perf(&lab.session.perf, &[normal_spec]).await;
    let csv = query_perf(&lab.session.perf, &[csv_spec]).await;
    let mismatches = if normal.is_fault() || csv.is_fault() {
        Vec::new()
    } else {
        diff_series(&normal.series, &csv.series)
    };
    let gap_note = describe_gaps(&csv);
    let v15_status = if normal.is_fault() || csv.is_fault() {
        Status::Failed
    } else if normal.series.is_empty() && csv.series.is_empty() {
        Status::Inconclusive
    } else if mismatches.is_empty() {
        Status::Confirmed
    } else {
        Status::Corrected
    };
    let mut v15 = Check::new(
        "V-15",
        v15_status,
        format!(
            "normal/csv mismatch count {}; gap encoding: {gap_note}",
            mismatches.len()
        ),
        json!({
            "mismatches": mismatches.iter().take(20).cloned().collect::<Vec<_>>(),
            "mismatch_count": mismatches.len(),
            "gap_encoding": gap_note,
            "normal_fault": normal.fault,
            "csv_fault": csv.fault,
            "normal_notes": normal.notes,
            "csv_notes": csv.notes,
            "csv_preview": csv.raw_csv_entity.as_ref().map(|raw| truncate(raw, 4000)),
        }),
    );
    if let Some(raw) = &csv.raw_csv_entity {
        v15.fixtures.push(("csv-entity.json".into(), raw.clone()));
    }

    let mut examples: Vec<String> = normal.raw_timestamps.iter().take(8).cloned().collect();
    examples.extend(csv.raw_timestamps.iter().take(8).cloned());
    examples.sort();
    examples.dedup();
    let has_z = examples.iter().any(|ts| ts.ends_with('Z'));
    let has_offset = examples.iter().any(|ts| timestamp_has_offset(ts));
    let fractional = examples.iter().any(|ts| ts.contains('.'));
    let v16_status = if examples.is_empty() {
        Status::Failed
    } else if has_offset && !has_z {
        Status::Corrected
    } else {
        Status::Confirmed
    };
    let mut v16 = Check::new(
        "V-16",
        v16_status,
        format!(
            "{} timestamp examples; Z={has_z}; offset_or_dash={has_offset}; fractional={fractional}",
            examples.len()
        ),
        json!({ "examples": examples, "has_z": has_z, "fractional": fractional }),
    );
    if !examples.is_empty() {
        v16.fixtures.push((
            "timestamps.json".into(),
            serde_json::to_string_pretty(&examples).unwrap_or_default(),
        ));
    }
    (v15, v16)
}

pub async fn v17(lab: &Lab<'_>) -> Check {
    let Some(host) = lab.host else {
        return Check::new("V-17", Status::Na, "no HostSystem in inventory", json!({}));
    };
    let ids = sample_ids(&lab.counters, 2, "");
    let start = ago_minutes(10);
    let realtime = spec_for(
        &host.moref,
        copy_metric_ids(&ids),
        lab.refresh_rate,
        &start,
        1,
        "normal",
    );
    let realtime_result = composite(lab, &realtime).await;
    let historical_id = enabled_period(&lab.intervals, 300).unwrap_or(lab.refresh_rate);
    let historical = spec_for(
        &host.moref,
        copy_metric_ids(&ids),
        historical_id,
        &start,
        1,
        "normal",
    );
    let historical_result = composite(lab, &historical).await;
    let vm_spec = spec_for(&lab.live.moref, ids, lab.refresh_rate, &start, 1, "normal");
    let vm_result = composite(lab, &vm_spec).await;
    let host_ok = realtime_result.get("fault").and_then(|v| v.as_bool()) != Some(true);
    let vm_fault = vm_result.get("fault").and_then(|v| v.as_bool()) == Some(true);
    let status = if host_ok && vm_fault {
        Status::Confirmed
    } else {
        Status::Corrected
    };
    Check::new(
        "V-17",
        status,
        format!(
            "host realtime ok={host_ok}; vm entity fault={vm_fault}; historical interval {historical_id}"
        ),
        json!({
            "host": moref_key(&host.moref),
            "realtime": realtime_result,
            "historical": historical_result,
            "vm_entity": vm_result,
        }),
    )
}

pub fn v19(intervals: &[IntervalRecord], target: &str) -> Check {
    let periods: Vec<i32> = intervals
        .iter()
        .filter(|interval| interval.enabled)
        .map(|interval| interval.sampling_period)
        .collect();
    if periods.is_empty() {
        return Check::new(
            "V-19",
            Status::Na,
            "no enabled historical intervals; PAST_* presets do not apply on this target",
            json!({ "target": target, "periods": periods }),
        );
    }
    let expected = [300, 1800, 7200, 86400];
    let status = if periods == expected {
        Status::Confirmed
    } else {
        Status::Corrected
    };
    Check::new(
        "V-19",
        status,
        format!("enabled sampling periods {periods:?}; design presets are {expected:?}"),
        json!({ "periods": periods, "expected": expected }),
    )
}

pub fn v18(intervals: &[IntervalRecord]) -> Check {
    if intervals.is_empty() {
        return Check::new(
            "V-18",
            Status::Na,
            "historical_interval is empty",
            json!({ "intervals": [] }),
        );
    }
    let summary = intervals
        .iter()
        .map(|interval| {
            format!(
                "{}s/{}s level {:?} enabled={}",
                interval.sampling_period, interval.length, interval.level, interval.enabled
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    Check::new(
        "V-18",
        Status::Confirmed,
        summary,
        json!({ "count": intervals.len() }),
    )
}

fn judge_units(
    cpu_raw: Option<i64>,
    mem_raw: Option<i64>,
    quick: Option<&crate::discover::VmQuick>,
) -> (Status, String) {
    let Some(vm) = quick else {
        return (
            Status::Inconclusive,
            "quick stats were not retrieved".into(),
        );
    };
    let cpu_note = match (cpu_raw, vm.overall_cpu_usage, vm.max_cpu_usage) {
        (Some(raw), Some(overall), Some(max)) if max > 0 => {
            let expected = (overall as f64 / max as f64) * 10_000.0;
            let delta = (raw as f64 - expected).abs();
            format!(
                "cpu raw {raw} vs hundredths-of-percent estimate {expected:.0} (delta {delta:.0})"
            )
        }
        _ => "cpu cross-check missing a sample or maxCpuUsage".into(),
    };
    let mem_note = match (mem_raw, vm.host_memory_usage.or(vm.guest_memory_usage)) {
        (Some(raw), Some(mb)) if mb > 0 => {
            let as_mb = raw as f64 / 1024.0;
            let kib_delta = (as_mb - mb as f64).abs() / mb as f64;
            let already_mb = (raw as f64 - mb as f64).abs() / mb as f64;
            if kib_delta < 0.25 {
                format!("mem raw {raw} / 1024 is within 25% of {mb} MB")
            } else if already_mb < 0.25 {
                return (
                    Status::Corrected,
                    format!("mem raw {raw} matches {mb} MB without a 1024 factor; {cpu_note}"),
                );
            } else {
                format!("mem raw {raw} vs {mb} MB is outside 25% either as KiB or as MB")
            }
        }
        _ => "mem cross-check missing a sample or quick stat".into(),
    };
    let status = if cpu_note.contains("missing")
        || mem_note.contains("missing")
        || mem_note.contains("outside")
    {
        Status::Inconclusive
    } else {
        Status::Confirmed
    };
    (status, format!("{cpu_note}; {mem_note}"))
}

fn describe_gaps(call: &PerfCall) -> String {
    let mut empty = 0;
    let mut minus = 0;
    for series in &call.series {
        for value in &series.values {
            match value {
                None => empty += 1,
                Some(-1) => minus += 1,
                _ => {}
            }
        }
    }
    format!("empty_fields={empty}, minus_one={minus}")
}

async fn composite(lab: &Lab<'_>, spec: &PerfQuerySpec) -> Value {
    match lab.session.perf.query_perf_composite(spec).await {
        Ok(metric) => {
            let child_count = metric.child_entity.as_ref().map(|c| c.len()).unwrap_or(0);
            let entity = metric
                .entity
                .as_ref()
                .map(|entity| moref_key(&entity.get_perf_entity_metric_base().entity));
            json!({
                "fault": false,
                "entity": entity,
                "child_count": child_count,
            })
        }
        Err(err) => json!({
            "fault": true,
            "fault_body": crate::query::fault_value(&err),
            "error": err.to_string(),
        }),
    }
}

async fn query_width(lab: &Lab<'_>, n: usize, interval: i32, format: &str) -> Value {
    if lab.counters.len() < n {
        return json!({
            "n": n,
            "skipped": format!("catalog has {} counters", lab.counters.len()),
            "fault": false,
        });
    }
    let spec = spec_for(
        &lab.live.moref,
        first_ids(&lab.counters, n, ""),
        interval,
        &ago_minutes(10),
        1,
        format,
    );
    let call = query_perf(&lab.session.perf, &[spec]).await;
    call_brief(&call, json!({ "n": n, "interval_id": interval }))
}

async fn named_shape(
    lab: &Lab<'_>,
    name: &str,
    metrics_per_spec: usize,
    spec_count: usize,
    interval: i32,
) -> Value {
    let needed = metrics_per_spec.saturating_mul(spec_count);
    if lab.counters.len() < needed {
        return json!({
            "name": name,
            "skipped": format!("need {needed} counters, have {}", lab.counters.len()),
            "fault": false,
        });
    }
    let mut specs = Vec::with_capacity(spec_count);
    for index in 0..spec_count {
        let start = index * metrics_per_spec;
        let ids = lab.counters[start..start + metrics_per_spec]
            .iter()
            .map(|counter| metric_id(counter.key, ""))
            .collect();
        specs.push(spec_for(
            &lab.live.moref,
            ids,
            interval,
            &ago_minutes(10),
            1,
            "csv",
        ));
    }
    let call = query_perf(&lab.session.perf, &specs).await;
    call_brief(
        &call,
        json!({ "name": name, "specs": spec_count, "metrics_per_spec": metrics_per_spec }),
    )
}

async fn star_expansion(lab: &Lab<'_>) -> Value {
    let Some(host) = lab.host else {
        return json!({ "inconclusive": true, "reason": "no host" });
    };
    let available = match lab
        .session
        .perf
        .query_available_perf_metric(&host.moref, None, None, Some(lab.refresh_rate))
        .await
    {
        Ok(rows) => rows.unwrap_or_default(),
        Err(err) => return json!({ "inconclusive": true, "reason": err.to_string() }),
    };
    let mut per_instance = Vec::new();
    for metric in available {
        if !metric.instance.is_empty() && !per_instance.contains(&metric.counter_id) {
            per_instance.push(metric.counter_id);
        }
    }
    if per_instance.len() < 8 {
        return json!({
            "inconclusive": true,
            "per_instance_counters": per_instance.len(),
            "reason": "too few per-instance counters to judge * expansion",
        });
    }
    let take = per_instance.len().min(64);
    let ids = per_instance
        .iter()
        .take(take)
        .map(|id| metric_id(*id, "*"))
        .collect();
    let spec = spec_for(
        &host.moref,
        ids,
        lab.refresh_rate,
        &ago_minutes(10),
        1,
        "csv",
    );
    let call = query_perf(&lab.session.perf, &[spec]).await;
    let series = call.series.len();
    json!({
        "inconclusive": !call.is_fault() && series <= 256,
        "requested_counters": take,
        "returned_series": series,
        "fault": call.is_fault(),
        "fault_type": call.fault.as_ref().and_then(fault_type_name),
        "error": call.error_text,
    })
}

fn multi_vm_specs(lab: &Lab<'_>, count: usize, interval: i32) -> Vec<PerfQuerySpec> {
    let counter = named_key(&lab.counters, "cpu.usage.average");
    let start = ago_minutes(10);
    lab.vms
        .iter()
        .take(count)
        .map(|vm| {
            spec_for(
                &vm.moref,
                vec![metric_id(counter, "")],
                interval,
                &start,
                1,
                "csv",
            )
        })
        .collect()
}

fn call_brief(call: &PerfCall, mut extra: Value) -> Value {
    if let Some(map) = extra.as_object_mut() {
        map.insert("fault".into(), json!(call.is_fault()));
        map.insert(
            "fault_type".into(),
            json!(call.fault.as_ref().and_then(fault_type_name)),
        );
        map.insert("entities".into(), json!(call.entities));
        map.insert("error".into(), json!(call.error_text));
        map.insert("series".into(), json!(call.series.len()));
        if call.is_fault() {
            map.insert("fault_body".into(), json!(call.fault));
        }
    }
    extra
}

fn shape_failed(shapes: &[Value], name: &str) -> bool {
    shapes.iter().any(|shape| {
        shape.get("name").and_then(|v| v.as_str()) == Some(name)
            && shape.get("fault").and_then(|v| v.as_bool()) == Some(true)
    })
}

fn named_key(counters: &[CounterRef], full: &str) -> i32 {
    find_counter(counters, full)
        .map(|counter| counter.key)
        .or_else(|| counters.first().map(|counter| counter.key))
        .unwrap_or(0)
}

/// Prefer counters that exist on a VM, then fill from catalog order.
fn sample_ids(
    counters: &[CounterRef],
    n: usize,
    instance: &str,
) -> Vec<vim_rs::types::structs::PerfMetricId> {
    const NAMES: &[&str] = &[
        "cpu.usage.average",
        "cpu.usagemhz.average",
        "mem.usage.average",
        "mem.consumed.average",
    ];
    let mut ids = Vec::new();
    for name in NAMES {
        if ids.len() >= n {
            break;
        }
        if let Some(counter) = find_counter(counters, name) {
            ids.push(metric_id(counter.key, instance));
        }
    }
    if ids.len() < n {
        for counter in counters {
            if ids.len() >= n {
                break;
            }
            if ids.iter().any(|id| id.counter_id == counter.key) {
                continue;
            }
            ids.push(metric_id(counter.key, instance));
        }
    }
    ids
}

fn timestamp_has_offset(ts: &str) -> bool {
    ts.split_once('T')
        .map(|(_, time)| time.contains('+') || time.contains('-'))
        .unwrap_or(false)
}

fn first_ids(
    counters: &[CounterRef],
    n: usize,
    instance: &str,
) -> Vec<vim_rs::types::structs::PerfMetricId> {
    counters
        .iter()
        .take(n)
        .map(|counter| metric_id(counter.key, instance))
        .collect()
}

fn find_counter<'a>(counters: &'a [CounterRef], full: &str) -> Option<&'a CounterRef> {
    counters.iter().find(|counter| counter.full_name() == full)
}

fn latest_value(call: &PerfCall, counter_id: i32) -> Option<i64> {
    call.series
        .iter()
        .find(|series| series.counter_id == counter_id && series.instance.is_empty())
        .and_then(|series| series.values.iter().rev().find_map(|value| *value))
}

fn enabled_period(intervals: &[IntervalRecord], period: i32) -> Option<i32> {
    intervals
        .iter()
        .find(|interval| interval.enabled && interval.sampling_period == period)
        .map(|interval| interval.sampling_period)
}

fn truncate(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        text.to_string()
    } else {
        text.chars().take(max).collect()
    }
}
