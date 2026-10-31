//! Counter catalog loaded from `perf_counter` or a `vim_rs.perf-catalog/1` document.

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use miniserde::json::{Number, Object, Value};

use crate::perf::entity::EntityScope;
use crate::perf::error::PerfError;
use crate::perf::metric::{Metric, Rollup, StatsKind};
use crate::perf::units::{Dimension, UnitKey};
use crate::types::enums::{PerfStatsTypeEnum, PerfSummaryTypeEnum, PerformanceManagerUnitEnum};
use crate::types::structs::{PerfCounterInfo, PerfInterval};

static NEXT_GENERATION: AtomicU64 = AtomicU64::new(1);

fn next_generation() -> u64 {
    NEXT_GENERATION.fetch_add(1, Ordering::Relaxed)
}

/// Server-local counter id. Never hardcode this in a caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CounterId(pub i32);

/// `group.name.rollup` identity. The numeric id is not part of it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CounterName {
    /// Text before the first dot.
    pub group: Arc<str>,
    /// Text between the first and last dot.
    pub name: Arc<str>,
    /// Text after the last dot, lowercase.
    pub rollup: Arc<str>,
    /// `group.name.rollup`.
    pub full: Arc<str>,
}

/// One counter in a server catalog.
#[derive(Debug, Clone)]
pub struct CounterDef {
    /// Server id.
    pub id: CounterId,
    /// Stable name.
    pub name: CounterName,
    /// Display label from the server.
    pub label: Arc<str>,
    /// Wire unit.
    pub unit: UnitKey,
    /// Wire rollup, including `Other_`.
    pub rollup: PerfSummaryTypeEnum,
    /// Wire stats kind, including `Other_`.
    pub stats: PerfStatsTypeEnum,
    /// Collection level. `None` when the server left it unset (every ESXi counter).
    pub level: Option<i32>,
    /// Per-device collection level. `None` when unset.
    pub per_device_level: Option<i32>,
}

/// Realtime and historical support for one managed-object kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderCaps {
    /// `PerfProviderSummary.current_supported`.
    pub current_supported: bool,
    /// `PerfProviderSummary.summary_supported`.
    pub summary_supported: bool,
    /// Refresh rate in seconds, when realtime is supported.
    pub refresh_rate: Option<i32>,
}

/// Session catalog plus historical intervals.
#[derive(Debug)]
pub struct CounterRegistry {
    generation: u64,
    by_full_str: HashMap<Arc<str>, Arc<CounterDef>>,
    /// `group.name.rollup`, a NUL, and the lowercase stats token.
    /// Every stats-kind lookup reads this map, short names via a stack buffer.
    by_qualified: HashMap<Arc<str>, Arc<CounterDef>>,
    /// `group.name.rollup` keys that matched more than one stats kind and no rate.
    ambiguous: HashSet<Arc<str>>,
    by_id: HashMap<CounterId, Arc<CounterDef>>,
    intervals: Vec<PerfInterval>,
}

impl CounterRegistry {
    /// Build a registry from `PerformanceManager.perf_counter` and `historical_interval`.
    pub fn from_wire(counters: Vec<PerfCounterInfo>, intervals: Vec<PerfInterval>) -> Self {
        let mut registry = Self::empty(intervals);
        for counter in counters {
            let group = counter.group_info.key.clone();
            let name = counter.name_info.key.clone();
            let rollup = counter.rollup_type.as_str().to_string();
            let stats = counter.stats_type.as_str().to_string();
            let label = counter.name_info.label.clone();
            let unit = PerformanceManagerUnitEnum::from_str(counter.unit_info.key.as_str());
            registry.insert(RawCounter {
                id: counter.key,
                group,
                name,
                rollup,
                stats,
                unit,
                label,
                level: counter.level,
                per_device_level: counter.per_device_level,
                rollup_enum: counter.rollup_type.clone(),
                stats_enum: counter.stats_type.clone(),
            });
        }
        registry
    }

    /// Parse a `vim_rs.perf-catalog/1` document with `miniserde`.
    pub fn from_catalog_json(json: &str) -> Result<Self, PerfError> {
        let value: Value = miniserde::json::from_str(json).map_err(|_| {
            PerfError::Client(crate::core::client::Error::ParseError(
                "perf catalog is not JSON".into(),
            ))
        })?;
        let obj =
            as_object(&value).ok_or_else(|| catalog_error("catalog root is not an object"))?;
        let schema = obj
            .get("schema")
            .and_then(as_str)
            .ok_or_else(|| catalog_error("catalog schema is missing"))?;
        if schema != "vim_rs.perf-catalog/1" {
            return Err(catalog_error(format!(
                "unsupported catalog schema `{schema}`"
            )));
        }
        let intervals = obj
            .get("intervals")
            .and_then(as_array)
            .map(parse_intervals)
            .transpose()?
            .unwrap_or_default();
        let counters = obj
            .get("counters")
            .and_then(as_array)
            .ok_or_else(|| catalog_error("catalog counters array is missing"))?;
        let mut registry = Self::empty(intervals);
        let mut seen = HashSet::new();
        for counter in counters.iter() {
            let raw = parse_counter(counter)?;
            let ident = (
                raw.group.clone(),
                raw.name.clone(),
                raw.rollup.clone(),
                raw.stats.clone(),
            );
            if !seen.insert(ident) {
                return Err(catalog_error(format!(
                    "duplicate counter {}.{} {} {}",
                    raw.group, raw.name, raw.rollup, raw.stats
                )));
            }
            registry.insert(raw);
        }
        Ok(registry)
    }

    /// Monotonic id assigned when this registry was built.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Resolve a metric. `stats: None` uses the rate-preferred bare name.
    pub fn resolve<D: Dimension, S: EntityScope>(
        &self,
        metric: &Metric<D, S>,
    ) -> Result<Arc<CounterDef>, PerfError> {
        match self.lookup(metric) {
            Some(def) => Ok(Arc::clone(def)),
            None => Err(PerfError::UnknownCounter(metric.full_name())),
        }
    }

    /// Counter id for a metric this registry contains. A hit does not allocate.
    pub(crate) fn lookup_id<D: Dimension, S: EntityScope>(
        &self,
        metric: &Metric<D, S>,
    ) -> Option<CounterId> {
        self.lookup(metric).map(|def| def.id)
    }

    fn lookup<D: Dimension, S: EntityScope>(
        &self,
        metric: &Metric<D, S>,
    ) -> Option<&Arc<CounterDef>> {
        self.lookup_parts(metric.group, metric.name, metric.rollup, metric.stats)
    }

    /// Same rules as [`Self::resolve`], without formatting a name on a hit.
    pub(crate) fn lookup_parts(
        &self,
        group: &str,
        name: &str,
        rollup: Rollup,
        stats: Option<StatsKind>,
    ) -> Option<&Arc<CounterDef>> {
        match stats {
            Some(stats) => self.qualified(group, name, rollup.as_str(), stats.as_str()),
            None => self.bare(group, name, rollup.as_str()),
        }
    }

    fn bare(&self, group: &str, name: &str, rollup: &str) -> Option<&Arc<CounterDef>> {
        let mut buf = [0u8; LOOKUP_BUF];
        if let Some(key) = write_lookup_key(group, name, rollup, None, &mut buf) {
            return self.by_full_str.get(key);
        }
        let owned = format!("{group}.{name}.{}", rollup.to_ascii_lowercase());
        self.by_full_str.get(owned.as_str())
    }

    fn qualified(
        &self,
        group: &str,
        name: &str,
        rollup: &str,
        stats: &str,
    ) -> Option<&Arc<CounterDef>> {
        let mut buf = [0u8; LOOKUP_BUF];
        if let Some(key) = write_lookup_key(group, name, rollup, Some(stats), &mut buf) {
            return self.by_qualified.get(key);
        }
        let owned = format!("{group}.{name}.{}\0{stats}", rollup.to_ascii_lowercase());
        self.by_qualified.get(owned.as_str())
    }

    /// Bare `group.name.rollup` lookup. Rate wins when both rate and another kind exist.
    /// Returns `None` when rate is absent and more than one stats kind matches.
    pub fn by_name(&self, full_name: &str) -> Option<&Arc<CounterDef>> {
        let (group, name, rollup) = split_counter_name(full_name)?;
        self.bare(group, name, rollup)
    }

    /// Lookup that includes the stats kind. The stats token is compared case-insensitively.
    pub fn by_name_with_stats(
        &self,
        full_name: &str,
        stats: StatsKind,
    ) -> Option<&Arc<CounterDef>> {
        let (group, name, rollup) = split_counter_name(full_name)?;
        self.qualified(group, name, rollup, stats.as_str())
    }

    /// Lookup by server id.
    pub fn by_id(&self, id: CounterId) -> Option<&Arc<CounterDef>> {
        self.by_id.get(&id)
    }

    /// Historical intervals from the server. Empty on ESXi.
    pub fn intervals(&self) -> &[PerfInterval] {
        &self.intervals
    }

    /// Every counter, in insertion order of the id map (not sorted).
    pub fn iter(&self) -> impl Iterator<Item = &Arc<CounterDef>> {
        self.by_id.values()
    }

    /// Number of counters.
    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    /// Whether the catalog has no counters.
    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }

    fn empty(intervals: Vec<PerfInterval>) -> Self {
        Self {
            generation: next_generation(),
            by_full_str: HashMap::new(),
            by_qualified: HashMap::new(),
            ambiguous: HashSet::new(),
            by_id: HashMap::new(),
            intervals,
        }
    }

    fn insert(&mut self, raw: RawCounter) {
        let name = make_name(&raw.group, &raw.name, &raw.rollup);
        let def = Arc::new(CounterDef {
            id: CounterId(raw.id),
            name,
            label: Arc::from(raw.label),
            unit: UnitKey(raw.unit),
            rollup: raw.rollup_enum,
            stats: raw.stats_enum,
            level: raw.level,
            per_device_level: raw.per_device_level,
        });
        let stats_lower = raw.stats.to_ascii_lowercase();
        self.by_id.insert(def.id, Arc::clone(&def));
        let qualified: Arc<str> = format!("{}\0{stats_lower}", def.name.full).into();
        // A server that lists one name and stats kind twice keeps the first, as the bare lookup does.
        self.by_qualified
            .entry(qualified)
            .or_insert_with(|| Arc::clone(&def));
        prefer_rate(&mut self.by_full_str, &mut self.ambiguous, def);
    }
}

struct RawCounter {
    id: i32,
    group: String,
    name: String,
    rollup: String,
    stats: String,
    unit: PerformanceManagerUnitEnum,
    label: String,
    level: Option<i32>,
    per_device_level: Option<i32>,
    rollup_enum: PerfSummaryTypeEnum,
    stats_enum: PerfStatsTypeEnum,
}

fn prefer_rate(
    map: &mut HashMap<Arc<str>, Arc<CounterDef>>,
    ambiguous: &mut HashSet<Arc<str>>,
    def: Arc<CounterDef>,
) {
    let full = Arc::clone(&def.name.full);
    if ambiguous.contains(&full) {
        if def.stats.as_str() == "rate" {
            ambiguous.remove(&full);
            map.insert(full, def);
        }
        return;
    }
    match map.get(&full) {
        None => {
            map.insert(full, def);
        }
        Some(existing) if existing.stats.as_str() == "rate" => {}
        // Same name and stats kind listed twice is not a stats-kind ambiguity.
        Some(existing) if existing.stats == def.stats => {}
        Some(_) if def.stats.as_str() == "rate" => {
            map.insert(full, def);
        }
        Some(_) => {
            // Rate is absent and more than one kind matches: bare lookup must fail.
            map.remove(&full);
            ambiguous.insert(full);
        }
    }
}

fn make_name(group: &str, name: &str, rollup: &str) -> CounterName {
    let rollup = rollup.to_ascii_lowercase();
    let full: Arc<str> = Arc::from(format!("{group}.{name}.{rollup}"));
    CounterName {
        group: Arc::from(group),
        name: Arc::from(name),
        rollup: Arc::from(rollup),
        full,
    }
}

/// Longest generated `group.name.rollup`, plus a NUL and a stats token, fits.
/// A longer name allocates that same key and reads the same map.
const LOOKUP_BUF: usize = 512;

/// `group.name.rollup`, or that plus `NUL` plus `stats`, written in lowercase rollup.
fn write_lookup_key<'a>(
    group: &str,
    name: &str,
    rollup: &str,
    stats: Option<&str>,
    buf: &'a mut [u8],
) -> Option<&'a str> {
    let stats_len = stats.map(str::len).unwrap_or(0);
    let extra = if stats.is_some() { 1 + stats_len } else { 0 };
    let need = group
        .len()
        .checked_add(name.len())?
        .checked_add(rollup.len())?;
    let need = need.checked_add(2 + extra)?;
    if need > buf.len() {
        return None;
    }
    let mut i = 0;
    buf[i..i + group.len()].copy_from_slice(group.as_bytes());
    i += group.len();
    buf[i] = b'.';
    i += 1;
    buf[i..i + name.len()].copy_from_slice(name.as_bytes());
    i += name.len();
    buf[i] = b'.';
    i += 1;
    for byte in rollup.bytes() {
        buf[i] = byte.to_ascii_lowercase();
        i += 1;
    }
    if let Some(stats) = stats {
        buf[i] = 0;
        i += 1;
        buf[i..i + stats.len()].copy_from_slice(stats.as_bytes());
        i += stats.len();
    }
    // SAFETY: `group`, `name`, `rollup`, and `stats` are `&str`. ASCII lowercasing
    // leaves non-ASCII bytes unchanged, and `.` and NUL are ASCII, so `buf[..i]` is UTF-8.
    Some(unsafe { std::str::from_utf8_unchecked(&buf[..i]) })
}

/// Group is before the first dot, rollup after the last, name is the middle.
pub fn split_counter_name(full: &str) -> Option<(&str, &str, &str)> {
    let (group, rest) = full.split_once('.')?;
    let (name, rollup) = rest.rsplit_once('.')?;
    if group.is_empty() || name.is_empty() || rollup.is_empty() {
        None
    } else {
        Some((group, name, rollup))
    }
}

fn parse_intervals(values: &[Value]) -> Result<Vec<PerfInterval>, PerfError> {
    values.iter().map(parse_interval).collect()
}

fn parse_interval(value: &Value) -> Result<PerfInterval, PerfError> {
    let obj = as_object(value).ok_or_else(|| catalog_error("interval is not an object"))?;
    Ok(PerfInterval {
        key: required_i32(obj, "key")?,
        sampling_period: required_i32(obj, "sampling_period")?,
        name: required_string(obj, "name")?,
        length: required_i32(obj, "length")?,
        level: optional_i32(obj, "level")?,
        enabled: match obj.get("enabled") {
            Some(Value::Bool(v)) => *v,
            _ => true,
        },
    })
}

fn parse_counter(value: &Value) -> Result<RawCounter, PerfError> {
    let obj = as_object(value).ok_or_else(|| catalog_error("counter is not an object"))?;
    let unit_token = required_string(obj, "unit")?;
    let rollup = required_string(obj, "rollup")?;
    let stats = required_string(obj, "stats")?;
    let label = obj
        .get("labels")
        .and_then(as_object)
        .and_then(|labels| labels.get("name"))
        .and_then(as_str)
        .unwrap_or("")
        .to_string();
    Ok(RawCounter {
        id: required_i32(obj, "key")?,
        group: required_string(obj, "group")?,
        name: required_string(obj, "name")?,
        rollup: rollup.clone(),
        stats: stats.clone(),
        unit: PerformanceManagerUnitEnum::from_str(&unit_token),
        label,
        level: optional_i32(obj, "level")?,
        per_device_level: optional_i32(obj, "per_device_level")?,
        rollup_enum: PerfSummaryTypeEnum::from_str(&rollup),
        stats_enum: PerfStatsTypeEnum::from_str(&stats),
    })
}

fn required_string(obj: &Object, key: &str) -> Result<String, PerfError> {
    obj.get(key)
        .and_then(as_str)
        .map(str::to_string)
        .ok_or_else(|| catalog_error(format!("missing string `{key}`")))
}

fn required_i32(obj: &Object, key: &str) -> Result<i32, PerfError> {
    obj.get(key)
        .and_then(as_i64)
        .and_then(|n| i32::try_from(n).ok())
        .ok_or_else(|| catalog_error(format!("missing integer `{key}`")))
}

fn optional_i32(obj: &Object, key: &str) -> Result<Option<i32>, PerfError> {
    match obj.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => as_i64(value)
            .and_then(|n| i32::try_from(n).ok())
            .map(Some)
            .ok_or_else(|| catalog_error(format!("integer `{key}` is invalid"))),
    }
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

fn catalog_error(message: impl Into<String>) -> PerfError {
    PerfError::Client(crate::core::client::Error::ParseError(message.into()))
}
