//! `perf-catalog.json` schema `vim_rs.perf-catalog/1`.

use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Context, Result};
use chrono::SecondsFormat;
use serde::Serialize;
use vim_rs::types::enums::{PerfStatsTypeEnum, PerfSummaryTypeEnum, PerformanceManagerUnitEnum};
use vim_rs::types::structs::{PerfCounterInfo, PerfInterval, PerfProviderSummary};
use vim_rs::types::traits::ElementDescriptionTrait;

use crate::discover::{Discovery, Sample};
use crate::session::{moref_key, Session};

pub const SCHEMA: &str = "vim_rs.perf-catalog/1";

#[derive(Debug, Serialize)]
pub struct Catalog {
    pub schema: String,
    pub source: Source,
    pub missing_types: Vec<String>,
    pub missing_samples: Vec<String>,
    pub intervals: Vec<IntervalRecord>,
    pub counters: Vec<CounterRecord>,
    pub providers: Vec<ProviderRecord>,
    pub availability: BTreeMap<String, TypeAvailability>,
    pub derived: Derived,
}

#[derive(Debug, Serialize)]
pub struct Source {
    pub server: String,
    pub full_name: String,
    pub version: String,
    pub build: String,
    pub api_version: String,
    pub api_type: String,
    pub product_line_id: String,
    pub instance_uuid: Option<String>,
    pub dumped_at: String,
    pub transport: String,
}

#[derive(Debug, Serialize)]
pub struct IntervalRecord {
    pub key: i32,
    pub sampling_period: i32,
    pub name: String,
    pub length: i32,
    pub level: Option<i32>,
    pub enabled: bool,
}

#[derive(Debug, Serialize)]
pub struct Labels {
    pub name: String,
    pub name_summary: String,
    pub group: String,
    pub group_summary: String,
    pub unit: String,
    pub unit_summary: String,
}

#[derive(Debug, Serialize)]
pub struct OtherEnums {
    pub rollup: Option<String>,
    pub stats: Option<String>,
    pub unit: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CounterRecord {
    pub key: i32,
    pub group: String,
    pub name: String,
    pub rollup: String,
    pub stats: String,
    pub unit: String,
    pub level: Option<i32>,
    pub per_device_level: Option<i32>,
    pub associated_counter_id: Vec<i32>,
    pub labels: Labels,
    pub other: OtherEnums,
}

#[derive(Debug, Serialize)]
pub struct ProviderRecord {
    pub r#type: String,
    pub moref: String,
    pub power_state: Option<String>,
    pub current_supported: bool,
    pub summary_supported: bool,
    pub refresh_rate: Option<i32>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TypeAvailability {
    pub samples: Vec<SampleAvailability>,
    pub counters: BTreeMap<String, CounterUse>,
}

#[derive(Debug, Serialize)]
pub struct SampleAvailability {
    pub moref: String,
    pub power_state: Option<String>,
    pub by_interval: BTreeMap<String, Vec<MetricInstance>>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MetricInstance {
    pub counter_id: i32,
    pub instance: String,
}

#[derive(Debug, Serialize)]
pub struct CounterUse {
    pub intervals: Vec<i32>,
    pub per_instance: bool,
}

#[derive(Debug, Serialize)]
pub struct Derived {
    pub units: Vec<String>,
    pub other_enums: Vec<String>,
    pub counters_missing_level: usize,
    pub counters_missing_per_device_level: usize,
}

#[derive(Debug, Clone)]
pub struct CounterRef {
    pub key: i32,
    pub group: String,
    pub name: String,
    pub rollup: String,
    pub unit: String,
    pub level: Option<i32>,
    pub per_device_level: Option<i32>,
}

impl CounterRef {
    pub fn full_name(&self) -> String {
        format!("{}.{}.{}", self.group, self.name, self.rollup)
    }
}

pub fn source_from(session: &Session) -> Source {
    let about = &session.client.service_content().about;
    Source {
        server: session.server.clone(),
        full_name: about.full_name.clone(),
        version: about.version.clone(),
        build: about.build.clone(),
        api_version: about.api_version.clone(),
        api_type: about.api_type.clone(),
        product_line_id: about.product_line_id.clone(),
        instance_uuid: about.instance_uuid.clone(),
        dumped_at: chrono::Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
        transport: session.transport.as_str().to_string(),
    }
}

pub fn counter_records(counters: &[PerfCounterInfo]) -> Vec<CounterRecord> {
    let mut records: Vec<CounterRecord> = counters.iter().map(counter_record).collect();
    records.sort_by(|a, b| {
        (&a.group, &a.name, &a.rollup, a.key).cmp(&(&b.group, &b.name, &b.rollup, b.key))
    });
    records
}

pub fn counter_refs(records: &[CounterRecord]) -> Vec<CounterRef> {
    records
        .iter()
        .map(|record| CounterRef {
            key: record.key,
            group: record.group.clone(),
            name: record.name.clone(),
            rollup: record.rollup.clone(),
            unit: record.unit.clone(),
            level: record.level,
            per_device_level: record.per_device_level,
        })
        .collect()
}

fn counter_record(counter: &PerfCounterInfo) -> CounterRecord {
    let (group, group_label, group_summary) = description(&*counter.group_info);
    let (name, name_label, name_summary) = description(&*counter.name_info);
    let (unit, unit_label, unit_summary) = description(&*counter.unit_info);
    let unit_other = match PerformanceManagerUnitEnum::from_str(&unit) {
        PerformanceManagerUnitEnum::Other_(raw) => Some(raw),
        _ => None,
    };
    CounterRecord {
        key: counter.key,
        group,
        name,
        rollup: counter.rollup_type.as_str().to_string(),
        stats: counter.stats_type.as_str().to_string(),
        unit,
        level: counter.level,
        per_device_level: counter.per_device_level,
        associated_counter_id: counter.associated_counter_id.clone().unwrap_or_default(),
        labels: Labels {
            name: name_label,
            name_summary,
            group: group_label,
            group_summary,
            unit: unit_label,
            unit_summary,
        },
        other: OtherEnums {
            rollup: other_summary(&counter.rollup_type),
            stats: other_stats(&counter.stats_type),
            unit: unit_other,
        },
    }
}

fn description(info: &dyn ElementDescriptionTrait) -> (String, String, String) {
    let element = info.get_element_description();
    (
        element.key.clone(),
        element.label.clone(),
        element.summary.clone(),
    )
}

fn other_summary(value: &PerfSummaryTypeEnum) -> Option<String> {
    match value {
        PerfSummaryTypeEnum::Other_(raw) => Some(raw.clone()),
        _ => None,
    }
}

fn other_stats(value: &PerfStatsTypeEnum) -> Option<String> {
    match value {
        PerfStatsTypeEnum::Other_(raw) => Some(raw.clone()),
        _ => None,
    }
}

pub fn interval_records(intervals: &[PerfInterval]) -> Vec<IntervalRecord> {
    let mut records: Vec<IntervalRecord> = intervals
        .iter()
        .map(|interval| IntervalRecord {
            key: interval.key,
            sampling_period: interval.sampling_period,
            name: interval.name.clone(),
            length: interval.length,
            level: interval.level,
            enabled: interval.enabled,
        })
        .collect();
    records.sort_by_key(|interval| interval.sampling_period);
    records
}

pub fn derived_from(counters: &[CounterRecord]) -> Derived {
    let mut units = BTreeSet::new();
    let mut other_enums = BTreeSet::new();
    let mut missing_level = 0;
    let mut missing_device = 0;
    for counter in counters {
        units.insert(counter.unit.clone());
        if counter.level.is_none() {
            missing_level += 1;
        }
        if counter.per_device_level.is_none() {
            missing_device += 1;
        }
        if let Some(raw) = &counter.other.rollup {
            other_enums.insert(format!("rollup:{raw}"));
        }
        if let Some(raw) = &counter.other.stats {
            other_enums.insert(format!("stats:{raw}"));
        }
        if let Some(raw) = &counter.other.unit {
            other_enums.insert(format!("unit:{raw}"));
        }
    }
    Derived {
        units: units.into_iter().collect(),
        other_enums: other_enums.into_iter().collect(),
        counters_missing_level: missing_level,
        counters_missing_per_device_level: missing_device,
    }
}

pub async fn fetch_intervals(session: &Session) -> Result<Vec<PerfInterval>> {
    Ok(session
        .perf
        .historical_interval()
        .await
        .context("historical_interval")?
        .unwrap_or_default())
}

pub async fn fetch_counters(session: &Session) -> Result<Vec<PerfCounterInfo>> {
    session
        .perf
        .perf_counter()
        .await
        .context("perf_counter")?
        .ok_or_else(|| anyhow::anyhow!("perf_counter returned no catalog"))
}

pub async fn collect_providers_and_availability(
    session: &Session,
    discovery: &Discovery,
    intervals: &[IntervalRecord],
) -> (Vec<ProviderRecord>, BTreeMap<String, TypeAvailability>) {
    let mut providers = Vec::new();
    let mut availability = BTreeMap::new();
    for (type_name, samples) in &discovery.samples {
        let mut type_avail = TypeAvailability {
            samples: Vec::new(),
            counters: BTreeMap::new(),
        };
        for sample in samples {
            let summary = session
                .perf
                .query_perf_provider_summary(&sample.moref)
                .await;
            let provider = match summary {
                Ok(summary) => provider_record(type_name, sample, &summary, None),
                Err(err) => ProviderRecord {
                    r#type: type_name.clone(),
                    moref: sample.moref.value.clone(),
                    power_state: sample.power_state.clone(),
                    current_supported: false,
                    summary_supported: false,
                    refresh_rate: None,
                    error: Some(err.to_string()),
                },
            };
            let current = provider.current_supported;
            let refresh = provider.refresh_rate.filter(|rate| *rate > 0);
            providers.push(provider);

            let mut by_interval = BTreeMap::new();
            let mut sample_error = None;
            if current {
                if let Some(rate) = refresh {
                    match available(session, sample, rate).await {
                        Ok(metrics) => {
                            by_interval.insert(rate.to_string(), metrics);
                        }
                        Err(err) => sample_error = Some(err),
                    }
                }
            }
            for interval in intervals.iter().filter(|interval| interval.enabled) {
                if by_interval.contains_key(&interval.sampling_period.to_string()) {
                    continue;
                }
                match available(session, sample, interval.sampling_period).await {
                    Ok(metrics) => {
                        by_interval.insert(interval.sampling_period.to_string(), metrics);
                    }
                    Err(err) => {
                        sample_error.get_or_insert(err);
                    }
                }
            }
            merge_counter_use(&mut type_avail.counters, &by_interval);
            type_avail.samples.push(SampleAvailability {
                moref: sample.moref.value.clone(),
                power_state: sample.power_state.clone(),
                by_interval,
                error: sample_error,
            });
        }
        availability.insert(type_name.clone(), type_avail);
    }
    providers.sort_by(|a, b| (&a.r#type, &a.moref).cmp(&(&b.r#type, &b.moref)));
    (providers, availability)
}

fn provider_record(
    type_name: &str,
    sample: &Sample,
    summary: &PerfProviderSummary,
    error: Option<String>,
) -> ProviderRecord {
    ProviderRecord {
        r#type: type_name.to_string(),
        moref: sample.moref.value.clone(),
        power_state: sample.power_state.clone(),
        current_supported: summary.current_supported,
        summary_supported: summary.summary_supported,
        refresh_rate: summary.refresh_rate,
        error,
    }
}

async fn available(
    session: &Session,
    sample: &Sample,
    interval_id: i32,
) -> Result<Vec<MetricInstance>, String> {
    eprintln!(
        "  available {} interval {interval_id}",
        moref_key(&sample.moref)
    );
    let metrics = session
        .perf
        .query_available_perf_metric(&sample.moref, None, None, Some(interval_id))
        .await
        .map_err(|err| err.to_string())?;
    let mut metrics: Vec<MetricInstance> = metrics
        .unwrap_or_default()
        .into_iter()
        .map(|metric| MetricInstance {
            counter_id: metric.counter_id,
            instance: metric.instance,
        })
        .collect();
    metrics.sort_by(|a, b| (a.counter_id, &a.instance).cmp(&(b.counter_id, &b.instance)));
    Ok(metrics)
}

fn merge_counter_use(
    into: &mut BTreeMap<String, CounterUse>,
    by_interval: &BTreeMap<String, Vec<MetricInstance>>,
) {
    for (interval, metrics) in by_interval {
        let Ok(interval_id) = interval.parse::<i32>() else {
            continue;
        };
        for metric in metrics {
            let entry = into
                .entry(metric.counter_id.to_string())
                .or_insert(CounterUse {
                    intervals: Vec::new(),
                    per_instance: false,
                });
            if !entry.intervals.contains(&interval_id) {
                entry.intervals.push(interval_id);
                entry.intervals.sort();
            }
            if !metric.instance.is_empty() {
                entry.per_instance = true;
            }
        }
    }
}
