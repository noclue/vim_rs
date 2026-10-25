//! Timestamps, aligned series, and row export.

use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use indexmap::IndexMap;

use crate::perf::entity::EntityScope;
use crate::perf::error::{EntityStatus, PerfError, PerfWarning};
use crate::perf::metric::Metric;
use crate::perf::query::Instance;
use crate::perf::registry::{CounterDef, CounterRegistry};
use crate::perf::units::{
    display_raw, raw_as_millis, Dimension, Percent, ScaledValue, Time, UnitKey,
};
use crate::types::structs::ManagedObjectReference;

/// UTC instant stored as [`SystemTime`]. The default build does not depend on `chrono`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(pub SystemTime);

impl Timestamp {
    /// Parse the VIM RFC 3339 subset: `Z` or a numeric offset, optional fraction.
    pub fn parse_rfc3339(text: &str) -> Result<Self, PerfError> {
        parse_rfc3339(text).map(Timestamp)
    }

    /// Format as UTC `YYYY-MM-DDTHH:MM:SSZ`, with a fraction when nanos are non-zero.
    pub fn to_rfc3339_utc(self) -> String {
        format_rfc3339_utc(self.0)
    }

    /// Elapsed time since the Unix epoch. `None` for instants before the epoch.
    pub fn duration_since_epoch(self) -> Option<Duration> {
        self.0.duration_since(UNIX_EPOCH).ok()
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_rfc3339_utc())
    }
}

#[cfg(feature = "perf-chrono")]
impl From<Timestamp> for chrono::DateTime<chrono::Utc> {
    fn from(ts: Timestamp) -> Self {
        chrono::DateTime::<chrono::Utc>::from(ts.0)
    }
}

#[cfg(feature = "perf-chrono")]
impl From<chrono::DateTime<chrono::Utc>> for Timestamp {
    fn from(dt: chrono::DateTime<chrono::Utc>) -> Self {
        Timestamp(SystemTime::from(dt))
    }
}

/// One raw sample on the entity grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sample {
    /// Sample time.
    pub ts: Timestamp,
    /// Raw counter value. Negative values are gaps.
    pub value: i64,
    /// Sampling interval in seconds.
    pub interval_secs: i32,
}

/// One counter on one entity, aligned to that entity’s timestamp grid.
#[derive(Debug, Clone)]
pub struct RawSeries {
    /// Catalog entry.
    pub counter: Arc<CounterDef>,
    /// Device instance. Aggregate is the empty wire string.
    pub instance: Instance,
    /// Shared with every other series on this entity.
    pub timestamps: Arc<[Timestamp]>,
    /// First observed sample interval, or the requested period.
    pub interval_secs: i32,
    /// Parallel to `timestamps`. Missing slots are `-1`.
    pub values: Vec<i64>,
}

impl RawSeries {
    /// Wire unit.
    pub fn unit(&self) -> &UnitKey {
        &self.counter.unit
    }

    /// Newest sample whose raw value is `>= 0`, skipping trailing gaps.
    pub fn last(&self) -> Option<Sample> {
        self.values.iter().enumerate().rev().find_map(|(i, value)| {
            (*value >= 0).then_some(Sample {
                ts: self.timestamps[i],
                value: *value,
                interval_secs: self.interval_secs,
            })
        })
    }

    /// The final slot only. `None` when the series is empty or that slot is negative.
    pub fn last_slot(&self) -> Option<Sample> {
        let i = self.values.len().checked_sub(1)?;
        let value = self.values[i];
        (value >= 0).then_some(Sample {
            ts: self.timestamps[i],
            value,
            interval_secs: self.interval_secs,
        })
    }

    /// Newest valid sample, scaled with [`AnyDimension`] to an `f64`.
    pub fn last_scaled(&self) -> Option<(Timestamp, f64)> {
        let sample = self.last()?;
        crate::perf::units::AnyDimension::scale(sample.value, &self.counter.unit.0)
            .map(|number| (sample.ts, number.as_f64()))
    }

    /// Display text of the newest valid sample, using the unit’s display rule.
    pub fn display_value(&self) -> String {
        match self.last() {
            Some(sample) => display_raw(sample.value, &self.counter.unit.0),
            None => "-".to_string(),
        }
    }

    /// Borrow this series as a typed [`Series`].
    pub fn view<D: Dimension>(&self) -> Series<'_, D> {
        Series {
            entity: None,
            counter: &self.counter,
            instance: &self.instance,
            raw: self,
            _d: std::marker::PhantomData,
        }
    }
}

/// A borrowed raw series tagged with dimension `D`.
pub struct Series<'a, D: Dimension> {
    /// Entity that owns the series, when known.
    pub entity: Option<&'a ManagedObjectReference>,
    /// Counter definition.
    pub counter: &'a Arc<CounterDef>,
    /// Device instance.
    pub instance: &'a Instance,
    raw: &'a RawSeries,
    _d: std::marker::PhantomData<D>,
}

impl<'a, D: Dimension> Series<'a, D> {
    /// Every slot, including negative gaps.
    pub fn samples(&self) -> impl Iterator<Item = Sample> + 'a {
        let interval = self.raw.interval_secs;
        self.raw
            .timestamps
            .iter()
            .zip(self.raw.values.iter())
            .map(move |(ts, value)| Sample {
                ts: *ts,
                value: *value,
                interval_secs: interval,
            })
    }

    /// Newest non-negative sample.
    pub fn last(&self) -> Option<Sample> {
        self.raw.last()
    }

    /// Final slot, or `None` when it is empty or negative.
    pub fn last_slot(&self) -> Option<Sample> {
        self.raw.last_slot()
    }

    /// Newest `N` slots, oldest first, left-padded with `None`. Negative raw values are `None`.
    pub fn window<const N: usize>(&self) -> [Option<i64>; N] {
        window_raw::<N>(&self.raw.values)
    }

    /// Raw values parallel to the timestamp grid.
    pub fn raw(&self) -> &'a [i64] {
        &self.raw.values
    }

    /// Wire unit.
    pub fn unit(&self) -> &'a UnitKey {
        &self.raw.counter.unit
    }

    /// The same grid, scaled in `D`. Negative slots are not scaled.
    pub fn scaled(&self) -> ScaledSeries<'a, D> {
        ScaledSeries {
            raw: self.raw,
            _d: std::marker::PhantomData,
        }
    }
}

impl<'a> Series<'a, Time> {
    /// CPU-ready percentage for each slot.
    ///
    /// Blank when the sample is negative, `vcpus` is `0`, the interval is not positive,
    /// or the instance is not the aggregate while `vcpus > 1`.
    pub fn ready_percent(&self, vcpus: u32) -> Vec<Option<Percent>> {
        self.raw
            .values
            .iter()
            .map(|raw| {
                ready_slot(
                    *raw,
                    self.raw.interval_secs,
                    vcpus,
                    self.instance,
                    &self.raw.counter.unit,
                )
            })
            .collect()
    }
}

/// One scaled sample.
#[derive(Debug, Clone, Copy)]
pub struct ScaledSample<D: Dimension> {
    /// Sample time.
    pub ts: Timestamp,
    /// Scaled value.
    pub value: D::Scaled,
    /// Sampling interval in seconds.
    pub interval_secs: i32,
}

/// Scaled view of a [`Series`].
pub struct ScaledSeries<'a, D: Dimension> {
    raw: &'a RawSeries,
    _d: std::marker::PhantomData<D>,
}

impl<'a, D: Dimension> ScaledSeries<'a, D> {
    /// Scaled slots. Negative raw values are skipped.
    pub fn samples(&self) -> impl Iterator<Item = ScaledSample<D>> + 'a {
        let unit = &self.raw.counter.unit.0;
        let interval = self.raw.interval_secs;
        self.raw
            .timestamps
            .iter()
            .zip(self.raw.values.iter())
            .filter_map(move |(ts, value)| {
                if *value < 0 {
                    return None;
                }
                D::scale(*value, unit).map(|scaled| ScaledSample {
                    ts: *ts,
                    value: scaled,
                    interval_secs: interval,
                })
            })
    }

    /// Newest scaled sample, skipping trailing gaps.
    pub fn last(&self) -> Option<ScaledSample<D>> {
        let sample = self.raw.last()?;
        D::scale(sample.value, &self.raw.counter.unit.0).map(|value| ScaledSample {
            ts: sample.ts,
            value,
            interval_secs: sample.interval_secs,
        })
    }

    /// Final slot, scaled. `None` when the slot is blank or the unit does not scale.
    pub fn last_slot(&self) -> Option<ScaledSample<D>> {
        let sample = self.raw.last_slot()?;
        D::scale(sample.value, &self.raw.counter.unit.0).map(|value| ScaledSample {
            ts: sample.ts,
            value,
            interval_secs: sample.interval_secs,
        })
    }

    /// Newest `N` scaled slots, oldest first, left-padded. Negative raw values are `None`.
    pub fn window<const N: usize>(&self) -> [Option<D::Scaled>; N] {
        let mut out = [None; N];
        let raw = window_raw::<N>(&self.raw.values);
        for (slot, value) in out.iter_mut().zip(raw) {
            if let Some(value) = value {
                *slot = D::scale(value, &self.raw.counter.unit.0);
            }
        }
        out
    }

    /// Scaled numeric values for non-negative slots that this dimension accepts.
    pub fn to_vec_f64(&self) -> Vec<f64> {
        self.samples().map(|sample| sample.value.as_f64()).collect()
    }
}

/// One valid (`raw >= 0`) sample in long form.
#[derive(Debug, Clone, Copy)]
pub struct Row<'a> {
    /// Managed object.
    pub entity: &'a ManagedObjectReference,
    /// Counter.
    pub counter: &'a Arc<CounterDef>,
    /// Device instance.
    pub instance: &'a Instance,
    /// Sample time.
    pub ts: Timestamp,
    /// Sampling interval in seconds.
    pub interval_secs: i32,
    /// Raw value. Never negative.
    pub raw: i64,
    /// Wire unit.
    pub unit: &'a UnitKey,
}

impl<'a> Row<'a> {
    /// Numeric value through [`crate::perf::units::AnyDimension`].
    pub fn scaled(&self) -> Option<f64> {
        crate::perf::units::AnyDimension::scale(self.raw, &self.unit.0).map(|n| n.as_f64())
    }
}

/// One managed object’s aligned series.
#[derive(Debug, Clone)]
pub struct EntitySeries {
    /// Managed object.
    pub entity: ManagedObjectReference,
    /// How this object ended the fetch.
    pub status: EntityStatus,
    /// Union timestamp grid. Shared with every [`RawSeries`] in `series`.
    pub timestamps: Arc<[Timestamp]>,
    /// First observed sample interval, otherwise the requested period.
    pub interval_secs: i32,
    /// Series aligned to `timestamps`.
    pub series: Vec<RawSeries>,
    registry: Arc<CounterRegistry>,
}

impl EntitySeries {
    pub(crate) fn new(
        entity: ManagedObjectReference,
        status: EntityStatus,
        timestamps: Arc<[Timestamp]>,
        interval_secs: i32,
        series: Vec<RawSeries>,
        registry: Arc<CounterRegistry>,
    ) -> Self {
        Self {
            entity,
            status,
            timestamps,
            interval_secs,
            series,
            registry,
        }
    }

    /// Aggregate series for `metric`, if the server returned one.
    pub fn series<D: Dimension, S: EntityScope>(
        &self,
        metric: Metric<D, S>,
    ) -> Option<Series<'_, D>> {
        self.series_instance(metric, &Instance::Aggregate)
    }

    /// One instance of `metric`.
    pub fn series_instance<D: Dimension, S: EntityScope>(
        &self,
        metric: Metric<D, S>,
        instance: &Instance,
    ) -> Option<Series<'_, D>> {
        let id = self.registry.lookup_id(&metric)?;
        let wire = instance.as_wire_str();
        let raw = self
            .series
            .iter()
            .find(|series| series.counter.id == id && series.instance.as_wire_str() == wire)?;
        Some(Series {
            entity: Some(&self.entity),
            counter: &raw.counter,
            instance: &raw.instance,
            raw,
            _d: std::marker::PhantomData,
        })
    }

    /// Non-aggregate instances of `metric`.
    pub fn devices<D: Dimension, S: EntityScope>(
        &self,
        metric: Metric<D, S>,
    ) -> impl Iterator<Item = Series<'_, D>> {
        let id = self.registry.lookup_id(&metric);
        (0..self.series.len()).filter_map(move |index| {
            let raw = &self.series[index];
            if Some(raw.counter.id) != id || raw.instance.is_aggregate() {
                return None;
            }
            Some(Series {
                entity: Some(&self.entity),
                counter: &raw.counter,
                instance: &raw.instance,
                raw,
                _d: std::marker::PhantomData,
            })
        })
    }
}

/// Result of a one-shot or prepared fetch.
#[derive(Debug, Clone)]
pub struct PerfResult {
    entities: IndexMap<ManagedObjectReference, EntitySeries>,
    registry: Arc<CounterRegistry>,
    requested_at: SystemTime,
    /// Warnings recorded while resolving and decoding. They do not fail the fetch.
    pub warnings: Vec<PerfWarning>,
}

impl PerfResult {
    pub(crate) fn new(
        entities: IndexMap<ManagedObjectReference, EntitySeries>,
        registry: Arc<CounterRegistry>,
        requested_at: SystemTime,
        warnings: Vec<PerfWarning>,
    ) -> Self {
        Self {
            entities,
            registry,
            requested_at,
            warnings,
        }
    }

    /// One requested entity.
    pub fn entity(&self, moref: &ManagedObjectReference) -> Option<&EntitySeries> {
        self.entities.get(moref)
    }

    /// Aggregate series for `metric` on every entity that returned one.
    pub fn series<D: Dimension, S: EntityScope>(
        &self,
        metric: Metric<D, S>,
    ) -> impl Iterator<Item = Series<'_, D>> {
        self.entities
            .values()
            .filter_map(move |entity| entity.series(metric))
    }

    /// Entities in request order.
    pub fn entities(&self) -> impl Iterator<Item = &EntitySeries> {
        self.entities.values()
    }

    /// Consume the result and yield entities in request order.
    pub fn into_entities(self) -> impl Iterator<Item = EntitySeries> {
        self.entities.into_values()
    }

    /// Every entity paired with each of its series.
    pub fn iter_all(&self) -> impl Iterator<Item = (&EntitySeries, &RawSeries)> {
        self.entities
            .values()
            .flat_map(|entity| entity.series.iter().map(move |series| (entity, series)))
    }

    /// Valid samples only (`raw >= 0`).
    pub fn rows(&self) -> impl Iterator<Item = Row<'_>> {
        self.entities.values().flat_map(|entity| {
            entity.series.iter().flat_map(move |series| {
                series
                    .timestamps
                    .iter()
                    .zip(series.values.iter())
                    .filter_map(move |(ts, raw)| {
                        (*raw >= 0).then_some(Row {
                            entity: &entity.entity,
                            counter: &series.counter,
                            instance: &series.instance,
                            ts: *ts,
                            interval_secs: series.interval_secs,
                            raw: *raw,
                            unit: &series.counter.unit,
                        })
                    })
            })
        })
    }

    /// When the fetch started.
    pub fn requested_at(&self) -> SystemTime {
        self.requested_at
    }

    /// Registry used to resolve this result.
    pub fn registry(&self) -> &Arc<CounterRegistry> {
        &self.registry
    }
}

/// Host series plus child series from `QueryPerfComposite`.
#[derive(Debug, Clone)]
pub struct CompositePerfResult {
    /// Host payload. `None` when the server omitted it.
    pub host: Option<EntitySeries>,
    /// Child entities the server returned. Omitted children are absent.
    pub children: IndexMap<ManagedObjectReference, EntitySeries>,
    /// Decode warnings.
    pub warnings: Vec<PerfWarning>,
}

fn window_raw<const N: usize>(values: &[i64]) -> [Option<i64>; N] {
    let mut out = [None; N];
    if N == 0 {
        return out;
    }
    let start = values.len().saturating_sub(N);
    let slice = &values[start..];
    let pad = N - slice.len();
    for (i, value) in slice.iter().enumerate() {
        if *value >= 0 {
            out[pad + i] = Some(*value);
        }
    }
    out
}

fn ready_slot(
    raw: i64,
    interval_secs: i32,
    vcpus: u32,
    instance: &Instance,
    unit: &UnitKey,
) -> Option<Percent> {
    if raw < 0 || vcpus == 0 || interval_secs <= 0 {
        return None;
    }
    if !instance.is_aggregate() && vcpus > 1 {
        return None;
    }
    let ready_ms = raw_as_millis(raw, &unit.0)?;
    let denom = f64::from(interval_secs) * 1000.0 * f64::from(vcpus);
    if denom <= 0.0 {
        return None;
    }
    Some(Percent(ready_ms / denom * 100.0))
}

fn parse_rfc3339(text: &str) -> Result<SystemTime, PerfError> {
    let invalid = || PerfError::InvalidTimestamp(text.to_string());
    let bytes = text.as_bytes();
    if bytes.len() < 20 {
        return Err(invalid());
    }
    let year = parse_n(&bytes[0..4]).ok_or_else(invalid)?;
    let month = parse_n(&bytes[5..7]).ok_or_else(invalid)?;
    let day = parse_n(&bytes[8..10]).ok_or_else(invalid)?;
    if bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
    {
        return Err(invalid());
    }
    let hour = parse_n(&bytes[11..13]).ok_or_else(invalid)?;
    let minute = parse_n(&bytes[14..16]).ok_or_else(invalid)?;
    let second = parse_n(&bytes[17..19]).ok_or_else(invalid)?;
    if !(1..=12).contains(&month)
        || !(1..=31).contains(&day)
        || hour > 23
        || minute > 59
        || second > 60
    {
        return Err(invalid());
    }
    let mut idx = 19;
    let mut nanos: u32 = 0;
    if bytes.get(idx) == Some(&b'.') {
        idx += 1;
        let start = idx;
        while idx < bytes.len() && bytes[idx].is_ascii_digit() {
            idx += 1;
        }
        if idx == start {
            return Err(invalid());
        }
        let digits = &text[start..idx];
        let mut padded = [b'0'; 9];
        let take = digits.len().min(9);
        padded[..take].copy_from_slice(&digits.as_bytes()[..take]);
        nanos = parse_n(&padded).ok_or_else(invalid)? as u32;
    }
    let (offset_secs, rest_ok) = parse_offset(&text[idx..]).ok_or_else(invalid)?;
    if !rest_ok {
        return Err(invalid());
    }
    let days = days_from_civil(year, month, day);
    let tod = i64::from(hour) * 3600 + i64::from(minute) * 60 + i64::from(second);
    let unix = days * 86_400 + tod - offset_secs;
    if unix < 0 {
        return Err(invalid());
    }
    Ok(UNIX_EPOCH + Duration::new(unix as u64, nanos))
}

fn parse_offset(text: &str) -> Option<(i64, bool)> {
    if text == "Z" || text == "z" {
        return Some((0, true));
    }
    let bytes = text.as_bytes();
    if bytes.len() != 6 && bytes.len() != 5 {
        return None;
    }
    let sign = match bytes[0] {
        b'+' => 1,
        b'-' => -1,
        _ => return None,
    };
    let (hour, minute) = if bytes.len() == 6 {
        if bytes[3] != b':' {
            return None;
        }
        (parse_n(&bytes[1..3])?, parse_n(&bytes[4..6])?)
    } else {
        (parse_n(&bytes[1..3])?, parse_n(&bytes[3..5])?)
    };
    if hour > 23 || minute > 59 {
        return None;
    }
    Some((
        sign * (i64::from(hour) * 3600 + i64::from(minute) * 60),
        true,
    ))
}

fn parse_n(bytes: &[u8]) -> Option<i32> {
    let mut n: i32 = 0;
    if bytes.is_empty() {
        return None;
    }
    for byte in bytes {
        let digit = (*byte).checked_sub(b'0')?;
        if digit > 9 {
            return None;
        }
        n = n.checked_mul(10)?.checked_add(i32::from(digit))?;
    }
    Some(n)
}

fn format_rfc3339_utc(time: SystemTime) -> String {
    let Ok(duration) = time.duration_since(UNIX_EPOCH) else {
        return "1970-01-01T00:00:00Z".to_string();
    };
    let secs = duration.as_secs() as i64;
    let nanos = duration.subsec_nanos();
    let days = secs.div_euclid(86_400);
    let tod = secs.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    let hour = tod / 3600;
    let minute = (tod % 3600) / 60;
    let second = tod % 60;
    if nanos == 0 {
        format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
    } else {
        format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{nanos:09}Z")
    }
}

/// Howard Hinnant’s `days_from_civil`, days since 1970-01-01.
fn days_from_civil(mut year: i32, month: i32, day: i32) -> i64 {
    year -= i32::from(month <= 2);
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = (year - era * 400) as u32;
    let mp = if month > 2 { month - 3 } else { month + 9 } as u32;
    let doy = (153 * mp + 2) / 5 + day as u32 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    i64::from(era) * 146_097 + i64::from(doe) - 719_468
}

fn civil_from_days(days: i64) -> (i32, i32, i32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe as i32 + era as i32 * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m as i32, d as i32)
}
