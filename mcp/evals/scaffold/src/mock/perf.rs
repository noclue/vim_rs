//! Fixture-backed `PerformanceManager` for the performance problems.
//!
//! Samples are stored as whole seconds before [`PerfStore::now`], which is captured once
//! when the mock is built. Query windows use `start < ts <= end`. Realtime `max_sample`
//! keeps the newest timestamps.

use std::collections::BTreeMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use bytes::Bytes;
use serde_json::{json, Value};

use vim_rs::core::client::Error;
use vim_rs::perf::Timestamp;
use vim_rs::types::struct_enum::StructType;
use vim_rs::types::structs::MethodFault;

#[derive(Clone, Debug)]
pub struct RecordedQuery {
    pub entity_type: String,
    pub entity: String,
    pub start: Option<SystemTime>,
    pub end: Option<SystemTime>,
    pub max_sample: Option<i32>,
    pub interval_id: Option<i32>,
    pub format: String,
    pub metrics: Vec<(i32, String)>,
}

#[derive(Clone)]
struct Sample {
    entity_type: String,
    entity: String,
    counter_id: i32,
    instance: String,
    offset_secs: u64,
    raw: i64,
}

#[derive(Clone)]
struct Provider {
    invalid: bool,
    current: bool,
    summary: bool,
    refresh: Option<i32>,
}

struct Fixture {
    providers: BTreeMap<String, Provider>,
    intervals_json: String,
    phases: Vec<Vec<Sample>>,
}

impl Fixture {
    fn empty() -> Self {
        Self {
            providers: BTreeMap::new(),
            intervals_json: "[]".to_string(),
            phases: vec![Vec::new()],
        }
    }

    fn load(bytes: &[u8]) -> Self {
        let Ok(text) = std::str::from_utf8(bytes) else {
            return Self::empty();
        };
        let Ok(root) = serde_json::from_str::<Value>(text) else {
            return Self::empty();
        };
        let Some(perf) = root.get("perf") else {
            return Self::empty();
        };
        let mut providers = BTreeMap::new();
        if let Some(map) = perf.get("providers").and_then(Value::as_object) {
            for (kind, spec) in map {
                providers.insert(
                    kind.clone(),
                    Provider {
                        invalid: spec.get("invalid_argument").and_then(Value::as_bool).unwrap_or(false),
                        current: spec.get("current_supported").and_then(Value::as_bool).unwrap_or(false),
                        summary: spec.get("summary_supported").and_then(Value::as_bool).unwrap_or(false),
                        refresh: spec.get("refresh_rate").and_then(Value::as_i64).map(|n| n as i32),
                    },
                );
            }
        }
        let phases = if let Some(phases) = perf.get("phases").and_then(Value::as_array) {
            if phases.is_empty() {
                vec![parse_samples(perf.get("samples"))]
            } else {
                phases
                    .iter()
                    .map(|phase| parse_samples(phase.get("samples")))
                    .collect()
            }
        } else {
            vec![parse_samples(perf.get("samples"))]
        };
        Self {
            providers,
            intervals_json: intervals_json(perf.get("intervals")),
            phases,
        }
    }

    fn samples(&self, phase: usize) -> &[Sample] {
        if self.phases.is_empty() {
            return &[];
        }
        let idx = phase.min(self.phases.len() - 1);
        &self.phases[idx]
    }
}

pub struct PerfStore {
    now: SystemTime,
    phase: usize,
    fixture: Fixture,
    recorded: Vec<RecordedQuery>,
}

impl PerfStore {
    pub fn new() -> Self {
        Self {
            now: SystemTime::now(),
            phase: 0,
            fixture: Fixture::empty(),
            recorded: Vec::new(),
        }
    }

    pub fn load_fixture(&mut self, bytes: &[u8]) {
        self.fixture = Fixture::load(bytes);
        self.phase = 0;
        self.recorded.clear();
    }

    pub fn now(&self) -> SystemTime {
        self.now
    }

    pub fn advance_phase(&mut self) {
        self.phase = self.phase.saturating_add(1);
    }

    pub fn recorded(&self) -> Vec<RecordedQuery> {
        self.recorded.clone()
    }

    pub fn counters_json(&self) -> Bytes {
        Bytes::from(counter_catalog_json().into_bytes())
    }

    pub fn intervals_json(&self) -> Bytes {
        Bytes::from(self.fixture.intervals_json.clone().into_bytes())
    }

    pub fn provider_summary(&self, body: &str) -> Result<Bytes, Error> {
        let spec = serde_json::from_str::<Value>(body).unwrap_or(Value::Null);
        let entity = &spec["entity"];
        let kind = entity.get("type").and_then(Value::as_str).unwrap_or("");
        let value = entity.get("value").and_then(Value::as_str).unwrap_or("");
        let Some(provider) = self.fixture.providers.get(kind) else {
            return Err(invalid_argument());
        };
        if provider.invalid {
            return Err(invalid_argument());
        }
        let mut summary = json!({
            "_typeName": "PerfProviderSummary",
            "entity": {
                "_typeName": "ManagedObjectReference",
                "type": kind,
                "value": value,
            },
            "currentSupported": provider.current,
            "summarySupported": provider.summary,
        });
        if let Some(rate) = provider.refresh {
            summary["refreshRate"] = json!(rate);
        }
        Ok(Bytes::from(summary.to_string().into_bytes()))
    }

    pub fn available(&self, body: &str) -> Bytes {
        let spec = serde_json::from_str::<Value>(body).unwrap_or(Value::Null);
        let entity = &spec["entity"];
        let kind = entity.get("type").and_then(Value::as_str).unwrap_or("");
        let value = entity.get("value").and_then(Value::as_str).unwrap_or("");
        let mut seen = BTreeMap::<(i32, String), ()>::new();
        for sample in self.fixture.samples(self.phase) {
            if sample.entity_type == kind && sample.entity == value {
                seen.insert((sample.counter_id, sample.instance.clone()), ());
            }
        }
        let ids: Vec<Value> = seen
            .into_keys()
            .map(|(counter_id, instance)| {
                json!({
                    "_typeName": "PerfMetricId",
                    "counterId": counter_id,
                    "instance": instance,
                })
            })
            .collect();
        Bytes::from(Value::Array(ids).to_string().into_bytes())
    }

    pub fn query(&mut self, body: &str) -> Bytes {
        let spec = serde_json::from_str::<Value>(body).unwrap_or(Value::Null);
        let specs = spec
            .get("querySpec")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let mut payloads = Vec::new();
        for raw in specs {
            let query = parse_query(&raw);
            if let Some(payload) = self.payload_for(&query) {
                payloads.push(payload);
            }
            self.recorded.push(query);
        }
        Bytes::from(Value::Array(payloads).to_string().into_bytes())
    }

    fn payload_for(&self, query: &RecordedQuery) -> Option<Value> {
        let interval = query.interval_id.unwrap_or(20);
        let mut hits: Vec<Hit> = Vec::new();
        for sample in self.fixture.samples(self.phase) {
            if sample.entity != query.entity || sample.entity_type != query.entity_type {
                continue;
            }
            if !metric_selected(&query.metrics, sample.counter_id, &sample.instance) {
                continue;
            }
            let ts = sample_time(self.now, sample.offset_secs);
            if !in_window(ts, query.start, query.end) {
                continue;
            }
            hits.push(Hit {
                ts,
                counter_id: sample.counter_id,
                instance: sample.instance.clone(),
                raw: sample.raw,
            });
        }
        if hits.is_empty() {
            return None;
        }
        let mut stamps: Vec<SystemTime> = hits.iter().map(|hit| hit.ts).collect();
        stamps.sort();
        stamps.dedup();
        if let Some(limit) = query.max_sample {
            if limit >= 0 {
                let limit = limit as usize;
                if stamps.len() > limit {
                    stamps = stamps.split_off(stamps.len() - limit);
                }
            }
        }
        hits.retain(|hit| stamps.contains(&hit.ts));
        let stamp_text: Vec<String> = stamps
            .iter()
            .map(|ts| Timestamp(*ts).to_rfc3339_utc())
            .collect();
        let mut series: BTreeMap<(i32, String), Vec<i64>> = BTreeMap::new();
        for hit in &hits {
            series
                .entry((hit.counter_id, hit.instance.clone()))
                .or_insert_with(|| vec![-1; stamps.len()]);
        }
        for hit in &hits {
            let Some(slot) = stamps.iter().position(|ts| *ts == hit.ts) else {
                continue;
            };
            if let Some(column) = series.get_mut(&(hit.counter_id, hit.instance.clone())) {
                column[slot] = hit.raw;
            }
        }
        Some(entity_json(
            &query.entity_type,
            &query.entity,
            interval,
            &stamp_text,
            &series,
            &query.format,
        ))
    }
}

struct Hit {
    ts: SystemTime,
    counter_id: i32,
    instance: String,
    raw: i64,
}

fn entity_json(
    kind: &str,
    value: &str,
    interval: i32,
    stamps: &[String],
    series: &BTreeMap<(i32, String), Vec<i64>>,
    format: &str,
) -> Value {
    let entity = json!({
        "_typeName": "ManagedObjectReference",
        "type": kind,
        "value": value,
    });
    if format == "normal" {
        let sample_info: Vec<Value> = stamps
            .iter()
            .map(|ts| {
                json!({
                    "_typeName": "PerfSampleInfo",
                    "timestamp": ts,
                    "interval": interval,
                })
            })
            .collect();
        let columns: Vec<Value> = series
            .iter()
            .map(|((counter_id, instance), values)| {
                json!({
                    "_typeName": "PerfMetricIntSeries",
                    "id": {
                        "_typeName": "PerfMetricId",
                        "counterId": counter_id,
                        "instance": instance,
                    },
                    "value": values,
                })
            })
            .collect();
        json!({
            "_typeName": "PerfEntityMetric",
            "entity": entity,
            "sampleInfo": sample_info,
            "value": columns,
        })
    } else {
        let mut csv = String::new();
        for (index, ts) in stamps.iter().enumerate() {
            if index > 0 {
                csv.push(',');
            }
            csv.push_str(&interval.to_string());
            csv.push(',');
            csv.push_str(ts);
        }
        let columns: Vec<Value> = series
            .iter()
            .map(|((counter_id, instance), values)| {
                let text = values
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                json!({
                    "_typeName": "PerfMetricSeriesCSV",
                    "id": {
                        "_typeName": "PerfMetricId",
                        "counterId": counter_id,
                        "instance": instance,
                    },
                    "value": text,
                })
            })
            .collect();
        json!({
            "_typeName": "PerfEntityMetricCSV",
            "entity": entity,
            "sampleInfoCSV": csv,
            "value": columns,
        })
    }
}

fn parse_query(raw: &Value) -> RecordedQuery {
    let entity = &raw["entity"];
    let kind = entity.get("type").and_then(Value::as_str).unwrap_or("VirtualMachine");
    let value = entity.get("value").and_then(Value::as_str).unwrap_or("");
    let metrics = raw
        .get("metricId")
        .and_then(Value::as_array)
        .map(|ids| {
            ids.iter()
                .map(|id| {
                    (
                        id.get("counterId").and_then(Value::as_i64).unwrap_or(0) as i32,
                        id.get("instance").and_then(Value::as_str).unwrap_or("").to_string(),
                    )
                })
                .collect()
        })
        .unwrap_or_default();
    RecordedQuery {
        entity_type: kind.to_string(),
        entity: value.to_string(),
        start: parse_time(raw, "startTime"),
        end: parse_time(raw, "endTime"),
        max_sample: raw.get("maxSample").and_then(Value::as_i64).map(|n| n as i32),
        interval_id: raw.get("intervalId").and_then(Value::as_i64).map(|n| n as i32),
        format: raw
            .get("format")
            .and_then(Value::as_str)
            .unwrap_or("csv")
            .to_string(),
        metrics,
    }
}

fn parse_time(raw: &Value, key: &str) -> Option<SystemTime> {
    let text = raw.get(key)?.as_str()?;
    Timestamp::parse_rfc3339(text).ok().map(|ts| ts.0)
}

fn metric_selected(metrics: &[(i32, String)], counter_id: i32, instance: &str) -> bool {
    if metrics.is_empty() {
        return true;
    }
    metrics.iter().any(|(id, requested)| {
        *id == counter_id && (requested == "*" || requested == instance)
    })
}

fn in_window(ts: SystemTime, start: Option<SystemTime>, end: Option<SystemTime>) -> bool {
    if let Some(start) = start {
        if ts <= start {
            return false;
        }
    }
    if let Some(end) = end {
        if ts > end {
            return false;
        }
    }
    true
}

fn sample_time(now: SystemTime, offset_secs: u64) -> SystemTime {
    now.checked_sub(Duration::from_secs(offset_secs))
        .unwrap_or(UNIX_EPOCH)
}

fn parse_samples(value: Option<&Value>) -> Vec<Sample> {
    value
        .and_then(Value::as_array)
        .map(|rows| rows.iter().map(parse_sample).collect())
        .unwrap_or_default()
}

fn parse_sample(value: &Value) -> Sample {
    let counter = value.get("counter").and_then(Value::as_str).unwrap_or("");
    Sample {
        entity_type: value
            .get("entity_type")
            .and_then(Value::as_str)
            .unwrap_or("VirtualMachine")
            .to_string(),
        entity: value.get("entity").and_then(Value::as_str).unwrap_or("").to_string(),
        counter_id: counter_id(counter).unwrap_or(0),
        instance: value.get("instance").and_then(Value::as_str).unwrap_or("").to_string(),
        offset_secs: value.get("offset_secs").and_then(Value::as_u64).unwrap_or(0),
        raw: value.get("raw").and_then(Value::as_i64).unwrap_or(-1),
    }
}

fn intervals_json(value: Option<&Value>) -> String {
    let Some(rows) = value.and_then(Value::as_array) else {
        return "[]".to_string();
    };
    let out: Vec<Value> = rows
        .iter()
        .enumerate()
        .map(|(index, item)| {
            json!({
                "_typeName": "PerfInterval",
                "key": item.get("key").and_then(Value::as_i64).unwrap_or(index as i64 + 1),
                "samplingPeriod": item.get("sampling_period").and_then(Value::as_i64).unwrap_or(300),
                "name": item.get("name").and_then(Value::as_str).unwrap_or("Past day"),
                "length": item.get("length").and_then(Value::as_i64).unwrap_or(86400),
                "level": item.get("level").and_then(Value::as_i64).unwrap_or(1),
                "enabled": item.get("enabled").and_then(Value::as_bool).unwrap_or(true),
            })
        })
        .collect();
    Value::Array(out).to_string()
}

fn counter_id(full: &str) -> Option<i32> {
    Some(match full {
        "cpu.costop.summation" => 448,
        "cpu.idle.summation" => 13,
        "cpu.ready.summation" => 12,
        "cpu.readiness.average" => 453,
        "cpu.usage.average" => 2,
        "cpu.usagemhz.average" => 6,
        "cpu.wait.summation" => 11,
        "mem.vmmemctl.average" => 90,
        "mem.vmmemctltarget.average" => 94,
        "mem.swapinRate.average" => 85,
        "mem.swapin.average" => 456,
        "mem.consumed.average" => 98,
        "power.power.average" => 164,
        "disk.deviceLatency.average" => 505,
        "disk.deviceReadLatency.average" => 497,
        "disk.deviceWriteLatency.average" => 501,
        "virtualDisk.totalReadLatency.average" => 182,
        _ => return None,
    })
}

fn counter_catalog_json() -> String {
    let rows = [
        (448, "cpu", "costop", "millisecond", "delta", "summation", 2, 3),
        (13, "cpu", "idle", "millisecond", "delta", "summation", 2, 3),
        (12, "cpu", "ready", "millisecond", "delta", "summation", 1, 3),
        (453, "cpu", "readiness", "percent", "rate", "average", 4, 4),
        (2, "cpu", "usage", "percent", "rate", "average", 1, 3),
        (6, "cpu", "usagemhz", "megaHertz", "rate", "average", 1, 3),
        (11, "cpu", "wait", "millisecond", "delta", "summation", 3, 3),
        (90, "mem", "vmmemctl", "kiloBytes", "absolute", "average", 1, 3),
        (94, "mem", "vmmemctltarget", "kiloBytes", "absolute", "average", 2, 3),
        (85, "mem", "swapinRate", "kiloBytesPerSecond", "rate", "average", 1, 3),
        (456, "mem", "swapin", "kiloBytes", "absolute", "average", 2, 3),
        (98, "mem", "consumed", "kiloBytes", "absolute", "average", 1, 3),
        (164, "power", "power", "watt", "rate", "average", 2, 3),
        (505, "disk", "deviceLatency", "millisecond", "absolute", "average", 1, 3),
        (497, "disk", "deviceReadLatency", "millisecond", "absolute", "average", 2, 3),
        (501, "disk", "deviceWriteLatency", "millisecond", "absolute", "average", 2, 3),
        (182, "virtualDisk", "totalReadLatency", "millisecond", "absolute", "average", 1, 3),
    ];
    let out: Vec<Value> = rows
        .into_iter()
        .map(|(key, group, name, unit, stats, rollup, level, per_device)| {
            json!({
                "_typeName": "PerfCounterInfo",
                "key": key,
                "nameInfo": desc(name),
                "groupInfo": desc(group),
                "unitInfo": desc(unit),
                "rollupType": rollup,
                "statsType": stats,
                "level": level,
                "perDeviceLevel": per_device,
            })
        })
        .collect();
    Value::Array(out).to_string()
}

fn desc(key: &str) -> Value {
    json!({
        "_typeName": "ElementDescription",
        "label": key,
        "summary": key,
        "key": key,
    })
}

fn invalid_argument() -> Error {
    Error::MethodFault(std::sync::Arc::new(MethodFault {
        fault_cause: None,
        fault_message: None,
        type_: Some(StructType::InvalidArgument),
        extra_fields_: Default::default(),
    }))
}
