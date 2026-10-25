//! Decode `PerfEntityMetric` and `PerfEntityMetricCSV` onto one timestamp grid per entity.
//!
//! Downcast only through `as_ref()` / deref. `Box::as_any_ref` hits the blanket `AsAny` impl
//! for `Box<T>` and every concrete downcast fails.

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use std::time::SystemTime;

use indexmap::IndexMap;

use crate::core::client::Error as ClientError;
use crate::perf::error::{EntityStatus, PerfError, PerfWarning};
use crate::perf::query::Instance;
use crate::perf::registry::{CounterId, CounterRegistry};
use crate::perf::result::{CompositePerfResult, EntitySeries, PerfResult, RawSeries, Timestamp};
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::{PerfEntityMetric, PerfEntityMetricCsv, PerfMetricIntSeries};
use crate::types::traits::PerfEntityMetricBaseTrait;

/// One series column aligned to its payload's `sampleInfo`.
struct Column {
    counter_id: i32,
    instance: String,
    values: Vec<i64>,
}

/// One `QueryPerf` entity payload: a shared timestamp column and one value column per series.
struct Payload {
    stamps: Vec<(Timestamp, i32)>,
    series: Vec<Column>,
}

struct Incoming {
    entity: ManagedObjectReference,
    payload: Payload,
}

/// Decode a `QueryPerf` payload.
///
/// `requested` entities that the server omitted become [`EntityStatus::NoSamples`].
/// When `requested` is empty, only entities present in `raw` are returned.
/// `requested_interval` is the period sent for these samples; a different observed
/// interval records [`PerfWarning::IntervalSubstituted`].
pub fn decode_metrics(
    raw: Vec<Box<dyn PerfEntityMetricBaseTrait>>,
    registry: &Arc<CounterRegistry>,
    requested: &[ManagedObjectReference],
    requested_interval: Option<i32>,
    requested_at: SystemTime,
) -> Result<PerfResult, PerfError> {
    let mut warnings = Vec::new();
    let incoming = parse_all(raw)?;
    let mut grouped: IndexMap<ManagedObjectReference, Vec<Payload>> = IndexMap::new();
    for item in incoming {
        grouped.entry(item.entity).or_default().push(item.payload);
    }
    let order: Vec<ManagedObjectReference> = if requested.is_empty() {
        grouped.keys().cloned().collect()
    } else {
        requested.to_vec()
    };
    let mut entities = IndexMap::new();
    for moref in order {
        let Some(payloads) = grouped.shift_remove(&moref) else {
            entities.insert(
                moref.clone(),
                EntitySeries::new(
                    moref,
                    EntityStatus::NoSamples,
                    Arc::from([]),
                    requested_interval.unwrap_or(0),
                    Vec::new(),
                    Arc::clone(registry),
                ),
            );
            continue;
        };
        let built = build_entity(moref, payloads, registry, requested_interval, &mut warnings)?;
        entities.insert(built.entity.clone(), built);
    }
    dedup_interval_warnings(&mut warnings);
    Ok(PerfResult::new(
        entities,
        Arc::clone(registry),
        requested_at,
        warnings,
    ))
}

fn dedup_interval_warnings(warnings: &mut Vec<PerfWarning>) {
    let mut seen = std::collections::HashSet::new();
    warnings.retain(|warning| match warning {
        PerfWarning::IntervalSubstituted {
            requested_secs,
            actual_secs,
        } => seen.insert((*requested_secs, *actual_secs)),
        _ => true,
    });
}

/// Decode a `QueryPerfComposite` payload. A missing host is `None`, not an error.
pub fn decode_composite(
    host: Option<Box<dyn PerfEntityMetricBaseTrait>>,
    children: Vec<Box<dyn PerfEntityMetricBaseTrait>>,
    registry: &Arc<CounterRegistry>,
    requested_interval: Option<i32>,
    requested_at: SystemTime,
) -> Result<CompositePerfResult, PerfError> {
    let mut warnings = Vec::new();
    let host = match host {
        Some(raw) => {
            let incoming = parse_one(raw)?;
            Some(build_entity(
                incoming.entity,
                vec![incoming.payload],
                registry,
                requested_interval,
                &mut warnings,
            )?)
        }
        None => None,
    };
    let mut child_map = IndexMap::new();
    for child in parse_all(children)? {
        let built = build_entity(
            child.entity,
            vec![child.payload],
            registry,
            requested_interval,
            &mut warnings,
        )?;
        child_map.insert(built.entity.clone(), built);
    }
    let _ = requested_at;
    Ok(CompositePerfResult {
        host,
        children: child_map,
        warnings,
    })
}

fn parse_all(raw: Vec<Box<dyn PerfEntityMetricBaseTrait>>) -> Result<Vec<Incoming>, PerfError> {
    raw.into_iter().map(parse_one).collect()
}

fn parse_one(raw: Box<dyn PerfEntityMetricBaseTrait>) -> Result<Incoming, PerfError> {
    // Deref first. Calling `as_any_ref` on the `Box` downcasts to `Box<dyn _>`.
    if let Some(metric) = raw.as_ref().as_any_ref().downcast_ref::<PerfEntityMetric>() {
        return parse_normal(metric);
    }
    if let Some(metric) = raw
        .as_ref()
        .as_any_ref()
        .downcast_ref::<PerfEntityMetricCsv>()
    {
        return parse_csv(metric);
    }
    Err(PerfError::Client(ClientError::ParseError(
        "QueryPerf entity was neither PerfEntityMetric nor PerfEntityMetricCSV".into(),
    )))
}

fn parse_normal(metric: &PerfEntityMetric) -> Result<Incoming, PerfError> {
    let samples = metric.sample_info.as_deref().unwrap_or(&[]);
    let mut stamps = Vec::with_capacity(samples.len());
    for sample in samples {
        stamps.push((
            Timestamp::parse_rfc3339(&sample.timestamp)?,
            sample.interval,
        ));
    }
    let mut series = Vec::new();
    for value in metric.value.as_deref().unwrap_or(&[]) {
        let Some(int_series) = value
            .as_ref()
            .as_any_ref()
            .downcast_ref::<PerfMetricIntSeries>()
        else {
            return Err(PerfError::Client(ClientError::ParseError(
                "PerfEntityMetric series was not PerfMetricIntSeries".into(),
            )));
        };
        let values = int_series.value.as_deref().unwrap_or(&[]);
        series.push(Column {
            counter_id: int_series.id.counter_id,
            instance: int_series.id.instance.clone(),
            values: column_values(stamps.len(), values),
        });
    }
    Ok(Incoming {
        entity: metric.entity.clone(),
        payload: Payload { stamps, series },
    })
}

fn parse_csv(metric: &PerfEntityMetricCsv) -> Result<Incoming, PerfError> {
    let stamps = parse_sample_info_csv(&metric.sample_info_csv)?;
    let mut series = Vec::new();
    for value in metric.value.as_deref().unwrap_or(&[]) {
        let numbers = parse_value_csv(value.value.as_deref().unwrap_or(""));
        series.push(Column {
            counter_id: value.id.counter_id,
            instance: value.id.instance.clone(),
            values: column_values(stamps.len(), &numbers),
        });
    }
    Ok(Incoming {
        entity: metric.entity.clone(),
        payload: Payload { stamps, series },
    })
}

/// Value column the same length as `sampleInfo`. A short column is padded with `-1`.
/// Values past the timestamp column are dropped.
fn column_values(stamps: usize, values: &[i64]) -> Vec<i64> {
    let mut out = Vec::with_capacity(stamps);
    out.extend(values.iter().copied().take(stamps));
    out.resize(stamps, -1);
    out
}

fn parse_sample_info_csv(text: &str) -> Result<Vec<(Timestamp, i32)>, PerfError> {
    if text.is_empty() {
        return Ok(Vec::new());
    }
    let tokens: Vec<&str> = text.split(',').collect();
    if tokens.len() % 2 != 0 {
        return Err(PerfError::Client(ClientError::ParseError(
            "sampleInfoCSV has an odd number of fields".into(),
        )));
    }
    let mut out = Vec::with_capacity(tokens.len() / 2);
    for pair in tokens.chunks(2) {
        let interval: i32 = pair[0].parse().map_err(|_| {
            PerfError::Client(ClientError::ParseError(format!(
                "sample interval `{}` is not an integer",
                pair[0]
            )))
        })?;
        out.push((Timestamp::parse_rfc3339(pair[1])?, interval));
    }
    Ok(out)
}

fn parse_value_csv(text: &str) -> Vec<i64> {
    if text.is_empty() {
        return Vec::new();
    }
    text.split(',')
        .map(|token| {
            if token.is_empty() || token == "-1" {
                -1
            } else {
                token.parse().unwrap_or(-1)
            }
        })
        .collect()
}

fn build_entity(
    entity: ManagedObjectReference,
    mut payloads: Vec<Payload>,
    registry: &Arc<CounterRegistry>,
    requested_interval: Option<i32>,
    warnings: &mut Vec<PerfWarning>,
) -> Result<EntitySeries, PerfError> {
    // sampleInfo with no value series used to contribute no points. Keep that:
    // those timestamps must not become Ok, stretch the grid, or warn.
    payloads.retain(|payload| !payload.series.is_empty() && !payload.stamps.is_empty());
    if payloads.is_empty() {
        return Ok(no_samples(entity, requested_interval, registry));
    }
    if payloads.len() == 1 && strictly_increasing(&payloads[0].stamps) {
        let payload = payloads.into_iter().next().expect("one payload");
        return Ok(build_aligned(
            entity,
            payload,
            registry,
            requested_interval,
            warnings,
        ));
    }
    Ok(build_grid(
        entity,
        payloads,
        registry,
        requested_interval,
        warnings,
    ))
}

fn strictly_increasing(stamps: &[(Timestamp, i32)]) -> bool {
    stamps.windows(2).all(|pair| pair[0].0 < pair[1].0)
}

/// Column index equals grid slot. Used when one payload is already strictly increasing.
#[derive(Clone, Copy)]
enum Slots<'a> {
    Aligned,
    Grid(&'a HashMap<Timestamp, usize>),
}

/// One payload whose timestamps are already the sorted unique grid.
fn build_aligned(
    entity: ManagedObjectReference,
    payload: Payload,
    registry: &Arc<CounterRegistry>,
    requested_interval: Option<i32>,
    warnings: &mut Vec<PerfWarning>,
) -> EntitySeries {
    note_intervals(
        payload.stamps.iter().map(|(_, interval)| *interval),
        requested_interval,
        warnings,
    );
    let interval_secs = payload.stamps[0].1;
    let timestamps: Arc<[Timestamp]> = payload.stamps.iter().map(|(ts, _)| *ts).collect();
    let merged = merge_payloads(timestamps.len(), std::iter::once(payload), Slots::Aligned);
    finish_entity(
        entity,
        timestamps,
        interval_secs,
        merged,
        registry,
        warnings,
    )
}

/// Several payloads, or one payload whose timestamps are not strictly increasing.
fn build_grid(
    entity: ManagedObjectReference,
    payloads: Vec<Payload>,
    registry: &Arc<CounterRegistry>,
    requested_interval: Option<i32>,
    warnings: &mut Vec<PerfWarning>,
) -> EntitySeries {
    let mut grid: BTreeMap<Timestamp, i32> = BTreeMap::new();
    for payload in &payloads {
        for (ts, interval) in &payload.stamps {
            grid.entry(*ts).or_insert(*interval);
        }
    }
    if grid.is_empty() {
        return no_samples(entity, requested_interval, registry);
    }
    note_intervals(grid.values().copied(), requested_interval, warnings);
    let interval_secs = grid.values().next().copied().unwrap_or(0);
    let timestamps: Arc<[Timestamp]> = grid.keys().copied().collect();
    let index_of: HashMap<Timestamp, usize> = timestamps
        .iter()
        .enumerate()
        .map(|(i, ts)| (*ts, i))
        .collect();
    let merged = merge_payloads(timestamps.len(), payloads, Slots::Grid(&index_of));
    finish_entity(
        entity,
        timestamps,
        interval_secs,
        merged,
        registry,
        warnings,
    )
}

/// One vector per `(counter, instance)`. The fast path moves a column in the first time
/// its key is seen. The grid path writes each sample into that vector through `slots`.
fn merge_payloads(
    width: usize,
    payloads: impl IntoIterator<Item = Payload>,
    slots: Slots<'_>,
) -> IndexMap<(i32, String), Vec<i64>> {
    let mut merged: IndexMap<(i32, String), Vec<i64>> = IndexMap::new();
    for payload in payloads {
        let stamps = payload.stamps;
        for column in payload.series {
            match slots {
                Slots::Aligned => place_aligned(&mut merged, column),
                Slots::Grid(index) => place_scattered(&mut merged, width, &stamps, column, index),
            }
        }
    }
    merged
}

fn place_aligned(merged: &mut IndexMap<(i32, String), Vec<i64>>, column: Column) {
    match merged.entry((column.counter_id, column.instance)) {
        indexmap::map::Entry::Vacant(slot) => {
            slot.insert(column.values);
        }
        indexmap::map::Entry::Occupied(mut slot) => {
            let dest = slot.get_mut();
            for (i, value) in column.values.into_iter().enumerate() {
                if i >= dest.len() {
                    break;
                }
                write_slot(dest, i, value);
            }
        }
    }
}

fn place_scattered(
    merged: &mut IndexMap<(i32, String), Vec<i64>>,
    width: usize,
    stamps: &[(Timestamp, i32)],
    column: Column,
    index: &HashMap<Timestamp, usize>,
) {
    let dest = merged
        .entry((column.counter_id, column.instance))
        .or_insert_with(|| vec![-1; width]);
    for (i, value) in column.values.into_iter().enumerate() {
        let Some(idx) = stamps.get(i).and_then(|(ts, _)| index.get(ts).copied()) else {
            continue;
        };
        write_slot(dest, idx, value);
    }
}

/// A new sample replaces a gap. A gap does not replace a real sample.
fn write_slot(dest: &mut [i64], idx: usize, value: i64) {
    if value >= 0 || dest[idx] < 0 {
        dest[idx] = value;
    }
}

fn finish_entity(
    entity: ManagedObjectReference,
    timestamps: Arc<[Timestamp]>,
    interval_secs: i32,
    merged: IndexMap<(i32, String), Vec<i64>>,
    registry: &Arc<CounterRegistry>,
    warnings: &mut Vec<PerfWarning>,
) -> EntitySeries {
    let mut raw_series = Vec::with_capacity(merged.len());
    for ((counter_id, instance), values) in merged {
        let Some(counter) = registry.by_id(CounterId(counter_id)).cloned() else {
            warnings.push(PerfWarning::UnsupportedSeries {
                counter_id: CounterId(counter_id),
            });
            continue;
        };
        raw_series.push(RawSeries {
            counter,
            instance: Instance::from_wire(&instance),
            timestamps: Arc::clone(&timestamps),
            interval_secs,
            values,
        });
    }
    let status = if timestamps.is_empty() {
        EntityStatus::NoSamples
    } else {
        EntityStatus::Ok
    };
    EntitySeries::new(
        entity,
        status,
        timestamps,
        interval_secs,
        raw_series,
        Arc::clone(registry),
    )
}

fn no_samples(
    entity: ManagedObjectReference,
    requested_interval: Option<i32>,
    registry: &Arc<CounterRegistry>,
) -> EntitySeries {
    EntitySeries::new(
        entity,
        EntityStatus::NoSamples,
        Arc::from([]),
        requested_interval.unwrap_or(0),
        Vec::new(),
        Arc::clone(registry),
    )
}

fn note_intervals(
    intervals: impl Iterator<Item = i32>,
    requested_interval: Option<i32>,
    warnings: &mut Vec<PerfWarning>,
) {
    let Some(requested) = requested_interval else {
        return;
    };
    let mut seen = Vec::new();
    for interval in intervals {
        if interval == requested || seen.contains(&interval) {
            continue;
        }
        seen.push(interval);
        warnings.push(PerfWarning::IntervalSubstituted {
            requested_secs: requested,
            actual_secs: interval,
        });
    }
}
