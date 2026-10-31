//! Scripted `PerformanceManager` for `vim_rs::perf` tests.
//!
//! Records `QueryPerf` / `QueryPerfComposite` specs (format and whether `maxSample` was sent).
//! Injects `MethodFault`, transport, and `ParseError` results. `set_clock` stores the instant
//! tests also pass to [`vim_rs::perf::PerfClient::set_clock`].

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use bytes::Bytes;
use miniserde::json::{Number, Object, Value};

use vim_rs::core::client::{BoxFuture, Error, PropertyValue, Result, Transport, VimClient};
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::struct_enum::StructType;
use vim_rs::types::structs::{MethodFault, ServiceContent};

/// One `PerfQuerySpec` as it was serialized on the wire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedSpec {
    pub entity: String,
    pub entity_type: String,
    pub format: Option<String>,
    pub max_sample: Option<i64>,
    pub interval_id: Option<i64>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub counter_ids: Vec<i64>,
}

/// Dropped when an in-flight `QueryPerf` future is cancelled.
#[derive(Clone)]
pub struct HangGate {
    pub started: Arc<AtomicBool>,
    pub cancelled: Arc<AtomicBool>,
}

/// Blocks `QueryPerfProviderSummary` until [`SummaryGate::release`] is set.
#[derive(Clone)]
pub struct SummaryGate {
    pub started: Arc<AtomicUsize>,
    pub release: Arc<AtomicBool>,
}

#[derive(Clone)]
pub struct MockCounter {
    pub key: i32,
    pub group: String,
    pub name: String,
    pub rollup: String,
    pub stats: String,
    pub unit: String,
    pub level: Option<i32>,
    pub per_device_level: Option<i32>,
}

#[derive(Clone)]
pub struct MockInterval {
    pub key: i32,
    pub sampling_period: i32,
    pub name: String,
    pub length: i32,
    pub level: Option<i32>,
    pub enabled: bool,
}

#[derive(Clone)]
#[allow(dead_code)]
enum SummaryKind {
    Provider {
        current: bool,
        summary: bool,
        rate: Option<i32>,
    },
    Fault(StructType),
}

#[derive(Clone)]
#[allow(dead_code)]
enum Injected {
    Fault(StructType),
    Parse(String),
    Transport,
}

struct State {
    counters: Vec<MockCounter>,
    intervals: Vec<MockInterval>,
    queries: Vec<Vec<RecordedSpec>>,
    composites: Vec<RecordedSpec>,
    available: Vec<String>,
    summaries: Vec<String>,
    summary_by_entity: HashMap<String, SummaryKind>,
    available_empty: bool,
    available_faults: HashMap<String, StructType>,
    composite_samples: usize,
    omit_entities: HashSet<String>,
    omit_counters: HashSet<i32>,
    fail_entities: HashMap<String, Injected>,
    fail_counters: HashMap<i32, String>,
    fail_all_local: bool,
    fatal: Option<Injected>,
    hang: Option<HangGate>,
    gap_values: bool,
    bad_sample: HashSet<String>,
    summary_hang: Option<SummaryGate>,
    clock: SystemTime,
    transport: Transport,
}

/// `VimClient` whose `PerformanceManager` methods are scripted.
pub struct PerfMockClient {
    service_content: ServiceContent,
    state: Mutex<State>,
}

#[allow(dead_code)]
impl PerfMockClient {
    pub fn new() -> Arc<Self> {
        Self::build(true, "VirtualCenter")
    }

    /// Same mock, but `about.apiType` reports a standalone ESXi host.
    pub fn esxi() -> Arc<Self> {
        Self::build(true, "HostAgent")
    }

    pub fn without_perf_manager() -> Arc<Self> {
        Self::build(false, "VirtualCenter")
    }

    fn build(has_manager: bool, api_type: &str) -> Arc<Self> {
        Arc::new(Self {
            service_content: service_content(has_manager, api_type),
            state: Mutex::new(State {
                counters: vcenter_counters(),
                intervals: vcenter_intervals(),
                queries: Vec::new(),
                composites: Vec::new(),
                available: Vec::new(),
                summaries: Vec::new(),
                summary_by_entity: HashMap::new(),
                available_empty: false,
                available_faults: HashMap::new(),
                composite_samples: 1,
                omit_entities: HashSet::new(),
                omit_counters: HashSet::new(),
                fail_entities: HashMap::new(),
                fail_counters: HashMap::new(),
                fail_all_local: false,
                fatal: None,
                hang: None,
                gap_values: false,
                bad_sample: HashSet::new(),
                summary_hang: None,
                clock: SystemTime::UNIX_EPOCH,
                transport: Transport::Json,
            }),
        })
    }

    pub fn set_clock(&self, now: SystemTime) {
        self.state().clock = now;
    }

    pub fn clock(&self) -> SystemTime {
        self.state().clock
    }

    pub fn set_transport(&self, transport: Transport) {
        self.state().transport = transport;
    }

    pub fn set_counters(&self, counters: Vec<MockCounter>) {
        self.state().counters = counters;
    }

    pub fn set_intervals(&self, intervals: Vec<MockInterval>) {
        self.state().intervals = intervals;
    }

    pub fn clear_counter_levels(&self) {
        for counter in &mut self.state().counters {
            counter.level = None;
            counter.per_device_level = None;
        }
    }

    pub fn set_summary_provider(
        &self,
        entity: &str,
        current: bool,
        summary: bool,
        rate: Option<i32>,
    ) {
        self.state().summary_by_entity.insert(
            entity.to_string(),
            SummaryKind::Provider {
                current,
                summary,
                rate,
            },
        );
    }

    pub fn set_summary_fault(&self, entity: &str, fault: StructType) {
        self.state()
            .summary_by_entity
            .insert(entity.to_string(), SummaryKind::Fault(fault));
    }

    pub fn set_available_empty(&self, empty: bool) {
        self.state().available_empty = empty;
    }

    /// `QueryAvailablePerfMetric` for `entity` returns `fault`. Other entities are unchanged.
    pub fn fail_available(&self, entity: &str, fault: StructType) {
        self.state()
            .available_faults
            .insert(entity.to_string(), fault);
    }

    /// `QueryPerfComposite` returns `n` samples per series. `1` keeps the fixture.
    pub fn set_composite_sample_count(&self, n: usize) {
        self.state().composite_samples = n.max(1);
    }

    pub fn omit_entity(&self, entity: &str) {
        self.state().omit_entities.insert(entity.to_string());
    }

    pub fn omit_counter(&self, counter_id: i32) {
        self.state().omit_counters.insert(counter_id);
    }

    pub fn fail_entity(&self, entity: &str, fault: StructType) {
        self.state()
            .fail_entities
            .insert(entity.to_string(), Injected::Fault(fault));
    }

    pub fn fail_batches_containing(&self, counter_id: i32, message: &str) {
        self.state()
            .fail_counters
            .insert(counter_id, message.to_string());
    }

    pub fn fail_all_local(&self) {
        self.state().fail_all_local = true;
    }

    pub fn fail_fatal(&self, fault: StructType) {
        self.state().fatal = Some(Injected::Fault(fault));
    }

    pub fn fail_transport(&self) {
        self.state().fatal = Some(Injected::Transport);
    }

    pub fn fail_parse(&self, message: &str) {
        self.state().fatal = Some(Injected::Parse(message.to_string()));
    }

    pub fn echo_gaps(&self) {
        self.state().gap_values = true;
    }

    pub fn bad_sample(&self, entity: &str) {
        self.state().bad_sample.insert(entity.to_string());
    }

    pub fn hang_summaries(&self) -> SummaryGate {
        let gate = SummaryGate {
            started: Arc::new(AtomicUsize::new(0)),
            release: Arc::new(AtomicBool::new(false)),
        };
        self.state().summary_hang = Some(gate.clone());
        gate
    }

    pub fn hang_query_perf(&self) -> HangGate {
        let gate = HangGate {
            started: Arc::new(AtomicBool::new(false)),
            cancelled: Arc::new(AtomicBool::new(false)),
        };
        self.state().hang = Some(gate.clone());
        gate
    }

    pub fn queries(&self) -> Vec<Vec<RecordedSpec>> {
        self.state().queries.clone()
    }

    pub fn composites(&self) -> Vec<RecordedSpec> {
        self.state().composites.clone()
    }

    pub fn available_calls(&self) -> Vec<String> {
        self.state().available.clone()
    }

    pub fn summary_calls(&self) -> Vec<String> {
        self.state().summaries.clone()
    }

    fn state(&self) -> std::sync::MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|err| err.into_inner())
    }
}

impl VimClient for PerfMockClient {
    fn service_content(&self) -> &ServiceContent {
        &self.service_content
    }

    fn transport(&self) -> Transport {
        self.state().transport
    }

    fn api_release(&self) -> String {
        self.service_content.about.api_version.clone()
    }

    fn invoke<'a>(
        &'a self,
        _svc: &'a str,
        _mo_type: &'a str,
        _mo_id: &'a str,
        method_name: &'a str,
        params: Option<&'a (dyn miniserde::Serialize + Send + Sync)>,
    ) -> BoxFuture<'a, Result<Bytes>> {
        let gate = if method_name == "QueryPerfProviderSummary" {
            self.state().summary_hang.clone()
        } else {
            None
        };
        Box::pin(async move {
            if let Some(gate) = gate {
                gate.started.fetch_add(1, Ordering::SeqCst);
                while !gate.release.load(Ordering::SeqCst) {
                    tokio::task::yield_now().await;
                }
            }
            match method_name {
                "QueryPerfProviderSummary" => self.provider_summary(params).map(Bytes::from),
                "QueryPerfComposite" => self.composite(params).map(Bytes::from),
                other => Err(Error::ParseError(format!(
                    "PerfMockClient: unhandled invoke {other}"
                ))),
            }
        })
    }

    fn invoke_optional<'a>(
        &'a self,
        _svc: &'a str,
        _mo_type: &'a str,
        _mo_id: &'a str,
        method_name: &'a str,
        params: Option<&'a (dyn miniserde::Serialize + Send + Sync)>,
    ) -> BoxFuture<'a, Result<Option<Bytes>>> {
        match method_name {
            "QueryPerf" => self.query_perf(params),
            "QueryAvailablePerfMetric" => match self.available(params) {
                Ok(body) => Box::pin(async move { Ok(Some(Bytes::from(body))) }),
                Err(err) => Box::pin(async move { Err(err) }),
            },
            other => Box::pin(async move {
                Err(Error::ParseError(format!(
                    "PerfMockClient: unhandled invoke_optional {other}"
                )))
            }),
        }
    }

    fn invoke_void<'a>(
        &'a self,
        _svc: &'a str,
        _mo_type: &'a str,
        _mo_id: &'a str,
        method_name: &'a str,
        _params: Option<&'a (dyn miniserde::Serialize + Send + Sync)>,
    ) -> BoxFuture<'a, Result<()>> {
        let method_name = method_name.to_string();
        Box::pin(async move {
            Err(Error::ParseError(format!(
                "PerfMockClient: unhandled void {method_name}"
            )))
        })
    }

    fn fetch_property_raw<'a>(
        &'a self,
        _svc: &'a str,
        _mo_type: &'a str,
        _mo_id: &'a str,
        property: &'a str,
    ) -> BoxFuture<'a, Result<Option<PropertyValue>>> {
        let property = property.to_string();
        Box::pin(async move {
            let state = self.state();
            let body = match property.as_str() {
                "perfCounter" => counters_json(&state.counters),
                "historicalInterval" => intervals_json(&state.intervals),
                other => {
                    return Err(Error::ParseError(format!(
                        "PerfMockClient: unhandled property {other}"
                    )))
                }
            };
            Ok(Some(PropertyValue::Json(Bytes::from(body))))
        })
    }
}

impl PerfMockClient {
    fn query_perf(
        &self,
        params: Option<&(dyn miniserde::Serialize + Send + Sync)>,
    ) -> BoxFuture<'_, Result<Option<Bytes>>> {
        let specs = parse_query_specs(params);
        let mut state = self.state();
        state.queries.push(specs.clone());
        if let Some(injected) = state.fatal.clone() {
            return Box::pin(async move { Err(injected_error(injected)) });
        }
        if let Some(gate) = state.hang.clone() {
            return Box::pin(async move {
                let _guard = CancelOnDrop(Arc::clone(&gate.cancelled));
                gate.started.store(true, Ordering::SeqCst);
                std::future::pending::<()>().await;
                Ok(None)
            });
        }
        if state.fail_all_local {
            let entity = specs
                .first()
                .map(|spec| spec.entity.clone())
                .unwrap_or_default();
            return Box::pin(async move { Err(Error::ParseError(format!("local:{entity}"))) });
        }
        if let Some(injected) = specs
            .iter()
            .find_map(|spec| state.fail_entities.get(&spec.entity).cloned())
        {
            return Box::pin(async move { Err(injected_error(injected)) });
        }
        if let Some(message) = specs.iter().find_map(|spec| {
            spec.counter_ids
                .iter()
                .find_map(|id| state.fail_counters.get(&(*id as i32)).cloned())
        }) {
            return Box::pin(async move { Err(Error::ParseError(message)) });
        }
        let gap = state.gap_values;
        let bad_sample = state.bad_sample.clone();
        let omit_entities = state.omit_entities.clone();
        let omit_counters = state.omit_counters.clone();
        let soap = is_soap(state.transport);
        drop(state);
        let body = if soap {
            soap_query_perf()
        } else {
            echo_metrics(&specs, gap, &bad_sample, &omit_entities, &omit_counters)
        };
        Box::pin(async move { Ok(Some(Bytes::from(body))) })
    }

    fn provider_summary(
        &self,
        params: Option<&(dyn miniserde::Serialize + Send + Sync)>,
    ) -> Result<String> {
        let (entity_type, entity) = parse_entity(params);
        let mut state = self.state();
        state.summaries.push(entity.clone());
        let kind = state
            .summary_by_entity
            .get(&entity)
            .cloned()
            .unwrap_or(SummaryKind::Provider {
                current: true,
                summary: true,
                rate: Some(20),
            });
        match kind {
            SummaryKind::Fault(fault) => Err(injected_error(Injected::Fault(fault))),
            SummaryKind::Provider {
                current,
                summary,
                rate,
            } => {
                let soap = is_soap(state.transport);
                drop(state);
                if soap {
                    #[cfg(feature = "xml")]
                    {
                        Ok(provider_xml(&entity_type, &entity, current, summary, rate))
                    }
                    #[cfg(not(feature = "xml"))]
                    {
                        let _ = (current, summary, rate);
                        unreachable!("SOAP transport requires the xml feature");
                    }
                } else {
                    Ok(provider_json(&entity_type, &entity, current, summary, rate))
                }
            }
        }
    }

    fn composite(
        &self,
        params: Option<&(dyn miniserde::Serialize + Send + Sync)>,
    ) -> Result<String> {
        let spec = parse_one_spec(params);
        let mut state = self.state();
        let samples = state.composite_samples;
        state.composites.push(spec);
        drop(state);
        if samples == 1 {
            Ok(include_str!("../fixtures/perf/query_composite.json").to_string())
        } else {
            Ok(composite_samples(samples))
        }
    }

    fn available(
        &self,
        params: Option<&(dyn miniserde::Serialize + Send + Sync)>,
    ) -> Result<String> {
        let (_ty, entity) = parse_entity(params);
        let mut state = self.state();
        state.available.push(entity.clone());
        if let Some(fault) = state.available_faults.get(&entity).cloned() {
            return Err(injected_error(Injected::Fault(fault)));
        }
        if state.available_empty {
            return Ok("[]".to_string());
        }
        let id = state
            .counters
            .first()
            .map(|counter| counter.key)
            .unwrap_or(2);
        Ok(format!(
            r#"[{{"_typeName":"PerfMetricId","counterId":{id},"instance":""}}]"#
        ))
    }
}

fn composite_samples(n: usize) -> String {
    let values: Vec<i32> = (1..=n as i32).collect();
    format!(
        r#"{{"_typeName":"PerfCompositeMetric","entity":{},"childEntity":[{}]}}"#,
        composite_entity("HostSystem", "host-1", &values),
        composite_entity("VirtualMachine", "vm-1", &values),
    )
}

fn composite_entity(entity_type: &str, entity: &str, values: &[i32]) -> String {
    let infos: Vec<String> = (0..values.len())
        .map(|i| {
            let total = 20 + i * 20;
            let minute = 48 + total / 60;
            let second = total % 60;
            format!(
                r#"{{"_typeName":"PerfSampleInfo","timestamp":"2026-10-24T10:{minute:02}:{second:02}Z","interval":20}}"#
            )
        })
        .collect();
    let nums: Vec<String> = values.iter().map(|value| value.to_string()).collect();
    format!(
        r#"{{"_typeName":"PerfEntityMetric","entity":{{"_typeName":"ManagedObjectReference","type":"{entity_type}","value":"{entity}"}},"sampleInfo":[{}],"value":[{{"_typeName":"PerfMetricIntSeries","id":{{"_typeName":"PerfMetricId","counterId":2,"instance":""}},"value":[{}]}}]}}"#,
        infos.join(","),
        nums.join(","),
    )
}

struct CancelOnDrop(Arc<AtomicBool>);

impl Drop for CancelOnDrop {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

fn injected_error(injected: Injected) -> Error {
    match injected {
        Injected::Fault(kind) => Error::MethodFault(Arc::new(MethodFault {
            fault_cause: None,
            fault_message: None,
            type_: Some(kind),
            extra_fields_: HashMap::new(),
        })),
        Injected::Parse(message) => Error::ParseError(message),
        Injected::Transport => Error::MissingOrInvalidSessionKey,
    }
}

fn service_content(has_manager: bool, api_type: &str) -> ServiceContent {
    let manager = if has_manager {
        r#","perfManager":{"_typeName":"ManagedObjectReference","type":"PerformanceManager","value":"PerfMgr"}"#
    } else {
        ""
    };
    let json = format!(
        r#"{{
        "_typeName":"ServiceContent",
        "rootFolder":{{"_typeName":"ManagedObjectReference","type":"Folder","value":"root-1"}},
        "propertyCollector":{{"_typeName":"ManagedObjectReference","type":"PropertyCollector","value":"pc-1"}},
        "viewManager":{{"_typeName":"ManagedObjectReference","type":"ViewManager","value":"vmgr-1"}}
        {manager},
        "about":{{
            "_typeName":"AboutInfo",
            "name":"n","fullName":"f","vendor":"v","version":"1","build":"b",
            "osType":"o","productLineId":"p","apiType":"{api_type}","apiVersion":"9.0.0.0"
        }}
    }}"#
    );
    miniserde::json::from_str(&json).expect("service content")
}

pub fn vcenter_counters() -> Vec<MockCounter> {
    vec![
        counter(
            2,
            "cpu",
            "usage",
            "average",
            "rate",
            "percent",
            Some(1),
            Some(3),
        ),
        counter(
            6,
            "cpu",
            "usage",
            "maximum",
            "rate",
            "percent",
            Some(1),
            None,
        ),
        counter(
            12,
            "cpu",
            "ready",
            "summation",
            "delta",
            "millisecond",
            Some(1),
            None,
        ),
        counter(
            24,
            "mem",
            "usage",
            "average",
            "absolute",
            "percent",
            Some(1),
            None,
        ),
        counter(
            98,
            "mem",
            "consumed",
            "average",
            "absolute",
            "kiloBytes",
            Some(1),
            None,
        ),
        counter(
            100,
            "disk",
            "usage",
            "average",
            "rate",
            "percent",
            Some(4),
            None,
        ),
        counter(
            200,
            "weird",
            "thing",
            "average",
            "rate",
            "furlongs",
            Some(1),
            None,
        ),
    ]
}

fn counter(
    key: i32,
    group: &str,
    name: &str,
    rollup: &str,
    stats: &str,
    unit: &str,
    level: Option<i32>,
    per_device_level: Option<i32>,
) -> MockCounter {
    MockCounter {
        key,
        group: group.to_string(),
        name: name.to_string(),
        rollup: rollup.to_string(),
        stats: stats.to_string(),
        unit: unit.to_string(),
        level,
        per_device_level,
    }
}

pub fn vcenter_intervals() -> Vec<MockInterval> {
    vec![
        interval(1, 300, "Past Day", 86400, Some(2)),
        interval(2, 1800, "Past Week", 604800, Some(1)),
        interval(3, 7200, "Past Month", 2592000, Some(1)),
        interval(4, 86400, "Past Year", 31536000, Some(1)),
    ]
}

fn interval(
    key: i32,
    sampling_period: i32,
    name: &str,
    length: i32,
    level: Option<i32>,
) -> MockInterval {
    MockInterval {
        key,
        sampling_period,
        name: name.to_string(),
        length,
        level,
        enabled: true,
    }
}

fn counters_json(counters: &[MockCounter]) -> String {
    let body = counters
        .iter()
        .map(counter_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn counter_json(counter: &MockCounter) -> String {
    let level = match counter.level {
        Some(level) => format!(r#","level":{level}"#),
        None => String::new(),
    };
    let device = match counter.per_device_level {
        Some(level) => format!(r#","perDeviceLevel":{level}"#),
        None => String::new(),
    };
    format!(
        r#"{{"_typeName":"PerfCounterInfo","key":{},"nameInfo":{},"groupInfo":{},"unitInfo":{},"rollupType":"{}","statsType":"{}"{level}{device}}}"#,
        counter.key,
        element(&counter.name),
        element(&counter.group),
        element(&counter.unit),
        counter.rollup,
        counter.stats,
    )
}

fn element(key: &str) -> String {
    format!(
        r#"{{"_typeName":"ElementDescription","key":"{key}","label":"{key}","summary":"{key}"}}"#
    )
}

fn intervals_json(intervals: &[MockInterval]) -> String {
    let body = intervals
        .iter()
        .map(|interval| {
            let level = match interval.level {
                Some(level) => format!(r#","level":{level}"#),
                None => String::new(),
            };
            format!(
                r#"{{"_typeName":"PerfInterval","key":{},"samplingPeriod":{},"name":"{}","length":{},"enabled":{}{level}}}"#,
                interval.key, interval.sampling_period, interval.name, interval.length, interval.enabled
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn is_soap(transport: Transport) -> bool {
    #[cfg(feature = "xml")]
    {
        transport == Transport::Soap
    }
    #[cfg(not(feature = "xml"))]
    {
        let _ = transport;
        false
    }
}

#[cfg(feature = "xml")]
fn provider_xml(
    entity_type: &str,
    entity: &str,
    current: bool,
    summary: bool,
    rate: Option<i32>,
) -> String {
    let rate = match rate {
        Some(rate) => format!("<refreshRate>{rate}</refreshRate>"),
        None => String::new(),
    };
    let current = if current { "true" } else { "false" };
    let summary = if summary { "true" } else { "false" };
    vim_rs::xml::soap::envelope(&format!(
        r#"<QueryPerfProviderSummaryResponse xmlns="urn:vim25"><returnval xsi:type="PerfProviderSummary"><entity type="{entity_type}">{entity}</entity><currentSupported>{current}</currentSupported><summarySupported>{summary}</summarySupported>{rate}</returnval></QueryPerfProviderSummaryResponse>"#
    ))
}

#[cfg(feature = "xml")]
fn soap_query_perf() -> String {
    vim_rs::xml::soap::envelope(r#"<QueryPerfResponse xmlns="urn:vim25"></QueryPerfResponse>"#)
}

#[cfg(not(feature = "xml"))]
fn soap_query_perf() -> String {
    String::new()
}

fn provider_json(
    entity_type: &str,
    entity: &str,
    current: bool,
    summary: bool,
    rate: Option<i32>,
) -> String {
    let rate = match rate {
        Some(rate) => format!(r#","refreshRate":{rate}"#),
        None => String::new(),
    };
    format!(
        r#"{{"_typeName":"PerfProviderSummary","entity":{{"_typeName":"ManagedObjectReference","type":"{entity_type}","value":"{entity}"}},"currentSupported":{current},"summarySupported":{summary}{rate}}}"#
    )
}

fn echo_metrics(
    specs: &[RecordedSpec],
    gap: bool,
    bad_sample: &HashSet<String>,
    omit_entities: &HashSet<String>,
    omit_counters: &HashSet<i32>,
) -> String {
    let value = if gap { -1 } else { 1 };
    let mut entities = Vec::new();
    for spec in specs {
        if omit_entities.contains(&spec.entity) {
            continue;
        }
        let stamp = if bad_sample.contains(&spec.entity) {
            "not-a-timestamp".to_string()
        } else {
            spec.end
                .clone()
                .unwrap_or_else(|| "2026-10-24T12:00:00Z".to_string())
        };
        let interval = spec.interval_id.unwrap_or(20);
        let series = spec
            .counter_ids
            .iter()
            .filter(|id| !omit_counters.contains(&(**id as i32)))
            .map(|id| {
                format!(
                    r#"{{"_typeName":"PerfMetricIntSeries","id":{{"_typeName":"PerfMetricId","counterId":{id},"instance":""}},"value":[{value}]}}"#
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        entities.push(format!(
            r#"{{"_typeName":"PerfEntityMetric","entity":{{"_typeName":"ManagedObjectReference","type":"{}","value":"{}"}},"sampleInfo":[{{"_typeName":"PerfSampleInfo","timestamp":"{stamp}","interval":{interval}}}],"value":[{series}]}}"#,
            spec.entity_type, spec.entity
        ));
    }
    format!("[{}]", entities.join(","))
}

fn parse_query_specs(
    params: Option<&(dyn miniserde::Serialize + Send + Sync)>,
) -> Vec<RecordedSpec> {
    let Some(root) = parse_params(params) else {
        return Vec::new();
    };
    let Some(list) = as_object(&root)
        .and_then(|obj| obj.get("querySpec"))
        .and_then(as_array)
    else {
        return Vec::new();
    };
    list.iter().filter_map(spec_from_value).collect()
}

fn parse_one_spec(params: Option<&(dyn miniserde::Serialize + Send + Sync)>) -> RecordedSpec {
    parse_params(params)
        .as_ref()
        .and_then(as_object)
        .and_then(|obj| obj.get("querySpec"))
        .and_then(spec_from_value)
        .unwrap_or(RecordedSpec {
            entity: String::new(),
            entity_type: String::new(),
            format: None,
            max_sample: None,
            interval_id: None,
            start: None,
            end: None,
            counter_ids: Vec::new(),
        })
}

fn parse_entity(params: Option<&(dyn miniserde::Serialize + Send + Sync)>) -> (String, String) {
    let Some(root) = parse_params(params) else {
        return (
            MoTypesEnum::VirtualMachine.as_str().to_string(),
            String::new(),
        );
    };
    let Some(entity) = as_object(&root)
        .and_then(|obj| obj.get("entity"))
        .and_then(as_object)
    else {
        return (
            MoTypesEnum::VirtualMachine.as_str().to_string(),
            String::new(),
        );
    };
    let ty = entity
        .get("type")
        .and_then(as_str)
        .unwrap_or("VirtualMachine")
        .to_string();
    let value = entity
        .get("value")
        .and_then(as_str)
        .unwrap_or("")
        .to_string();
    (ty, value)
}

fn spec_from_value(value: &Value) -> Option<RecordedSpec> {
    let obj = as_object(value)?;
    let entity = obj.get("entity").and_then(as_object)?;
    let counter_ids = obj
        .get("metricId")
        .and_then(as_array)
        .map(|ids| {
            ids.iter()
                .filter_map(|id| {
                    as_object(id)
                        .and_then(|obj| obj.get("counterId"))
                        .and_then(as_i64)
                })
                .collect()
        })
        .unwrap_or_default();
    Some(RecordedSpec {
        entity: entity
            .get("value")
            .and_then(as_str)
            .unwrap_or("")
            .to_string(),
        entity_type: entity
            .get("type")
            .and_then(as_str)
            .unwrap_or("")
            .to_string(),
        format: obj.get("format").and_then(as_str).map(str::to_string),
        max_sample: obj.get("maxSample").and_then(as_i64),
        interval_id: obj.get("intervalId").and_then(as_i64),
        start: obj.get("startTime").and_then(as_str).map(str::to_string),
        end: obj.get("endTime").and_then(as_str).map(str::to_string),
        counter_ids,
    })
}

fn parse_params(params: Option<&(dyn miniserde::Serialize + Send + Sync)>) -> Option<Value> {
    let params = params?;
    let text = miniserde::json::to_string(params);
    miniserde::json::from_str(&text).ok()
}

fn as_object(value: &Value) -> Option<&Object> {
    match value {
        Value::Object(object) => Some(object),
        _ => None,
    }
}

fn as_array(value: &Value) -> Option<&[Value]> {
    match value {
        Value::Array(array) => Some(array),
        _ => None,
    }
}

fn as_str(value: &Value) -> Option<&str> {
    match value {
        Value::String(text) => Some(text),
        _ => None,
    }
}

fn as_i64(value: &Value) -> Option<i64> {
    match value {
        Value::Number(Number::I64(n)) => Some(*n),
        Value::Number(Number::U64(n)) => i64::try_from(*n).ok(),
        Value::Number(Number::F64(n)) if n.fract() == 0.0 => Some(*n as i64),
        _ => None,
    }
}
