//! QueryPerf helpers, CSV/Normal decoding, and the synthetic missing moref.

use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;
use serde_json::Value;
use vim_rs::core::client::Error as ClientError;
use vim_rs::mo::PerformanceManager;
use vim_rs::types::structs::{
    ManagedObjectReference, PerfEntityMetric, PerfEntityMetricCsv, PerfMetricId,
    PerfMetricIntSeries, PerfQuerySpec,
};
use vim_rs::types::traits::{PerfEntityMetricBaseTrait, PerfMetricSeriesTrait};

use crate::session::clone_moref;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeriesDecoded {
    pub entity: String,
    pub counter_id: i32,
    pub instance: String,
    pub timestamps: Vec<String>,
    pub intervals: Vec<i32>,
    pub values: Vec<Option<i64>>,
}

#[derive(Debug, Clone)]
pub struct PerfCall {
    pub error_text: Option<String>,
    pub fault: Option<Value>,
    pub entities: Vec<String>,
    pub series: Vec<SeriesDecoded>,
    pub raw_timestamps: Vec<String>,
    /// First CSV entity, miniserde JSON, for the fixture file.
    pub raw_csv_entity: Option<String>,
    pub notes: Vec<String>,
}

impl PerfCall {
    pub fn failed(err: &ClientError) -> Self {
        Self {
            error_text: Some(err.to_string()),
            fault: Some(fault_value(err)),
            entities: Vec::new(),
            series: Vec::new(),
            raw_timestamps: Vec::new(),
            raw_csv_entity: None,
            notes: Vec::new(),
        }
    }

    pub fn is_fault(&self) -> bool {
        self.fault.is_some()
    }
}

pub fn fault_value(err: &ClientError) -> Value {
    match err {
        ClientError::MethodFault(fault) => {
            let raw = miniserde::json::to_string(fault.as_ref());
            serde_json::from_str(&raw).unwrap_or_else(|_| serde_json::json!({ "raw": raw }))
        }
        other => serde_json::json!({ "error": other.to_string() }),
    }
}

pub fn fault_type_name(fault: &Value) -> Option<String> {
    fault
        .get("_typeName")
        .and_then(Value::as_str)
        .map(str::to_string)
}

/// A well-formed VM id that is not in `existing`.
///
/// vCenter values look like `vm-N`. ESXi values are often bare integers.
/// An empty list defaults to `VirtualMachine:vm-2147483646`.
pub fn unused_vm_moref(existing: &[ManagedObjectReference]) -> ManagedObjectReference {
    let mo_type = existing
        .first()
        .map(|m| m.r#type.clone())
        .unwrap_or(vim_rs::types::enums::MoTypesEnum::VirtualMachine);
    let mut prefixed = existing.iter().any(|m| m.value.starts_with("vm-"));
    if existing.is_empty() {
        prefixed = true;
    }
    let mut max_n = 0i64;
    for moref in existing {
        let numeric = moref
            .value
            .strip_prefix("vm-")
            .unwrap_or(moref.value.as_str());
        if let Ok(n) = numeric.parse::<i64>() {
            max_n = max_n.max(n);
        }
    }
    let mut n = (max_n.saturating_add(1_000_000)).max(2_147_483_646);
    let taken: BTreeSet<&str> = existing.iter().map(|m| m.value.as_str()).collect();
    loop {
        let value = if prefixed {
            format!("vm-{n}")
        } else {
            n.to_string()
        };
        if !taken.contains(value.as_str()) {
            return ManagedObjectReference {
                r#type: mo_type,
                value,
            };
        }
        n = n.saturating_add(1);
    }
}

pub fn split_sample_info(csv: &str) -> Result<Vec<(i32, String)>, String> {
    if csv.is_empty() {
        return Ok(Vec::new());
    }
    let parts: Vec<&str> = csv.split(',').collect();
    if parts.len() % 2 != 0 {
        return Err(format!(
            "sample_info_csv has {} fields, expected interval,timestamp pairs",
            parts.len()
        ));
    }
    let mut out = Vec::with_capacity(parts.len() / 2);
    for chunk in parts.chunks(2) {
        let interval = chunk[0]
            .parse::<i32>()
            .map_err(|e| format!("interval '{}': {e}", chunk[0]))?;
        out.push((interval, chunk[1].to_string()));
    }
    Ok(out)
}

/// Empty fields are gaps. `-1` is kept so the sentinel scan can see it.
pub fn split_series(csv: &str) -> Vec<Option<i64>> {
    if csv.is_empty() {
        return Vec::new();
    }
    csv.split(',')
        .map(|part| {
            if part.is_empty() {
                None
            } else {
                part.parse::<i64>().ok()
            }
        })
        .collect()
}

pub async fn query_perf(perf: &PerformanceManager, specs: &[PerfQuerySpec]) -> PerfCall {
    match perf.query_perf(specs).await {
        Err(err) => PerfCall::failed(&err),
        Ok(None) => PerfCall {
            error_text: None,
            fault: None,
            entities: Vec::new(),
            series: Vec::new(),
            raw_timestamps: Vec::new(),
            raw_csv_entity: None,
            notes: vec!["QueryPerf returned null".into()],
        },
        Ok(Some(entities)) => decode_entities(&entities),
    }
}

fn decode_entities(entities: &[Box<dyn PerfEntityMetricBaseTrait>]) -> PerfCall {
    let mut call = PerfCall {
        error_text: None,
        fault: None,
        entities: Vec::new(),
        series: Vec::new(),
        raw_timestamps: Vec::new(),
        raw_csv_entity: None,
        notes: Vec::new(),
    };
    for entity in entities {
        let base = entity.get_perf_entity_metric_base();
        call.entities.push(format!(
            "{}:{}",
            base.entity.r#type.as_str(),
            base.entity.value
        ));
        // as_any_ref on the Box hits the blanket AsAny impl for Box itself.
        // Downcast the inner trait object, which is the PerfEntityMetric.
        let entity: &dyn PerfEntityMetricBaseTrait = entity.as_ref();
        if let Some(metric) = entity.as_any_ref().downcast_ref::<PerfEntityMetric>() {
            decode_normal(&mut call, metric);
        } else if let Some(csv) = entity.as_any_ref().downcast_ref::<PerfEntityMetricCsv>() {
            if call.raw_csv_entity.is_none() {
                call.raw_csv_entity = Some(miniserde::json::to_string(csv));
            }
            decode_csv(&mut call, csv);
        } else {
            let type_name = entity.data_type().as_str();
            call.notes
                .push(format!("unhandled entity subtype {type_name}"));
        }
    }
    call
}

fn decode_normal(call: &mut PerfCall, metric: &PerfEntityMetric) {
    let entity = format!("{}:{}", metric.entity.r#type.as_str(), metric.entity.value);
    let sample_info = metric.sample_info.as_deref().unwrap_or(&[]);
    for sample in sample_info {
        call.raw_timestamps.push(sample.timestamp.clone());
    }
    let timestamps: Vec<String> = sample_info.iter().map(|s| s.timestamp.clone()).collect();
    let intervals: Vec<i32> = sample_info.iter().map(|s| s.interval).collect();
    let Some(values) = metric.value.as_ref() else {
        call.notes.push(format!("{entity} normal value is null"));
        return;
    };
    for series in values {
        let series: &dyn PerfMetricSeriesTrait = series.as_ref();
        let Some(int_series) = series.as_any_ref().downcast_ref::<PerfMetricIntSeries>() else {
            call.notes.push(format!(
                "{entity} series subtype {}",
                series.data_type().as_str()
            ));
            continue;
        };
        let id = &int_series.id;
        let raw = int_series.value.clone().unwrap_or_default();
        if raw.len() != timestamps.len() {
            call.notes.push(format!(
                "{entity} counter {} length {} != {} timestamps",
                id.counter_id,
                raw.len(),
                timestamps.len()
            ));
        }
        call.series.push(SeriesDecoded {
            entity: entity.clone(),
            counter_id: id.counter_id,
            instance: id.instance.clone(),
            timestamps: timestamps.clone(),
            intervals: intervals.clone(),
            values: raw.into_iter().map(Some).collect(),
        });
    }
}

fn decode_csv(call: &mut PerfCall, csv: &PerfEntityMetricCsv) {
    let entity = format!("{}:{}", csv.entity.r#type.as_str(), csv.entity.value);
    let samples = match split_sample_info(&csv.sample_info_csv) {
        Ok(samples) => samples,
        Err(err) => {
            call.notes.push(format!("{entity} {err}"));
            return;
        }
    };
    for (_, ts) in &samples {
        call.raw_timestamps.push(ts.clone());
    }
    let timestamps: Vec<String> = samples.iter().map(|(_, ts)| ts.clone()).collect();
    let intervals: Vec<i32> = samples.iter().map(|(interval, _)| *interval).collect();
    let Some(values) = csv.value.as_ref() else {
        call.notes.push(format!("{entity} csv value is null"));
        return;
    };
    for series in values {
        let id = &series.id;
        let parsed = series
            .value
            .as_deref()
            .map(split_series)
            .unwrap_or_default();
        if parsed.len() != timestamps.len() && !timestamps.is_empty() {
            call.notes.push(format!(
                "{entity} csv counter {} length {} != {} timestamps",
                id.counter_id,
                parsed.len(),
                timestamps.len()
            ));
        }
        call.series.push(SeriesDecoded {
            entity: entity.clone(),
            counter_id: id.counter_id,
            instance: id.instance.clone(),
            timestamps: timestamps.clone(),
            intervals: intervals.clone(),
            values: parsed,
        });
    }
}

pub fn copy_metric_ids(ids: &[PerfMetricId]) -> Vec<PerfMetricId> {
    ids.iter()
        .map(|id| metric_id(id.counter_id, &id.instance))
        .collect()
}

pub fn metric_id(counter_id: i32, instance: &str) -> PerfMetricId {
    PerfMetricId {
        counter_id,
        instance: instance.to_string(),
    }
}

pub fn spec_for(
    entity: &ManagedObjectReference,
    metric_ids: Vec<PerfMetricId>,
    interval_id: i32,
    start_time: &str,
    max_sample: i32,
    format: &str,
) -> PerfQuerySpec {
    PerfQuerySpec {
        entity: clone_moref(entity),
        start_time: Some(start_time.to_string()),
        end_time: None,
        max_sample: Some(max_sample),
        metric_id: Some(metric_ids),
        interval_id: Some(interval_id),
        format: Some(format.to_string()),
    }
}

pub fn ago_minutes(minutes: i64) -> String {
    let when = chrono::Utc::now() - chrono::Duration::minutes(minutes);
    when.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

#[derive(Debug, Default)]
pub struct SentinelHistogram {
    pub samples: usize,
    pub minus_one: usize,
    pub empty_gaps: usize,
    pub other_negatives: BTreeMap<i64, usize>,
}

impl SentinelHistogram {
    pub fn observe(&mut self, series: &[SeriesDecoded]) {
        for item in series {
            for value in &item.values {
                self.samples += 1;
                match value {
                    None => self.empty_gaps += 1,
                    Some(-1) => self.minus_one += 1,
                    Some(n) if *n < 0 => *self.other_negatives.entry(*n).or_default() += 1,
                    Some(_) => {}
                }
            }
        }
    }
}

pub fn series_key(series: &SeriesDecoded) -> (String, i32, String) {
    (
        series.entity.clone(),
        series.counter_id,
        series.instance.clone(),
    )
}

/// Compare two decodes of the same query. Returns human-readable mismatches.
pub fn diff_series(left: &[SeriesDecoded], right: &[SeriesDecoded]) -> Vec<String> {
    let mut right_map: BTreeMap<(String, i32, String), &SeriesDecoded> = BTreeMap::new();
    for series in right {
        right_map.insert(series_key(series), series);
    }
    let mut notes = Vec::new();
    let mut seen = BTreeSet::new();
    for series in left {
        let key = series_key(series);
        seen.insert(key.clone());
        let Some(other) = right_map.get(&key) else {
            notes.push(format!("missing on right: {} {} {}", key.0, key.1, key.2));
            continue;
        };
        if series.values != other.values {
            notes.push(format!(
                "value mismatch {} counter {} instance '{}'",
                key.0, key.1, key.2
            ));
        }
    }
    for series in right {
        let key = series_key(series);
        if !seen.contains(&key) {
            notes.push(format!("missing on left: {} {} {}", key.0, key.1, key.2));
        }
    }
    notes
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mor(value: &str) -> ManagedObjectReference {
        ManagedObjectReference {
            r#type: vim_rs::types::enums::MoTypesEnum::VirtualMachine,
            value: value.to_string(),
        }
    }

    #[test]
    fn missing_id_uses_vm_prefix_and_avoids_collisions() {
        let missing = unused_vm_moref(&[mor("vm-10"), mor("vm-42")]);
        assert_eq!(missing.value, "vm-2147483646");
        let bumped = unused_vm_moref(&[mor("vm-2147483646")]);
        assert_eq!(bumped.value, "vm-2148483646");
    }

    #[test]
    fn missing_id_on_bare_esxi_values_has_no_prefix() {
        let missing = unused_vm_moref(&[mor("5"), mor("8")]);
        assert_eq!(missing.value, "2147483646");
    }

    #[test]
    fn csv_pairs_and_gaps() {
        let samples = split_sample_info("20,2026-10-24T07:30:20Z,20,2026-10-24T07:30:40Z").unwrap();
        assert_eq!(samples.len(), 2);
        assert_eq!(samples[0].0, 20);
        assert_eq!(
            split_series("1,,-1,4"),
            vec![Some(1), None, Some(-1), Some(4)]
        );
        assert!(split_sample_info("20,a,20").is_err());
    }
}
