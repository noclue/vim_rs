//! Errors, warnings, and per-entity status for `vim_rs::perf`.

use std::sync::Arc;

use crate::core::client;
use crate::perf::registry::CounterId;
use crate::perf::result::Timestamp;
use crate::types::enums::MoTypesEnum;
use crate::types::structs::ManagedObjectReference;

/// Failure of a performance query, or of one entity inside it.
#[derive(Debug, Clone, thiserror::Error)]
#[non_exhaustive]
pub enum PerfError {
    /// `ServiceContent.perf_manager` was missing.
    #[error("PerformanceManager is not available on this connection's ServiceContent")]
    NoPerformanceManager,

    /// The builder had no managed objects.
    #[error("no entities specified in performance query")]
    NoEntities,

    /// The builder had no counters, names, or availability filter.
    #[error("no metrics specified in performance query")]
    NoMetrics,

    /// Every entity was skipped by the interval table.
    #[error(
        "none of the requested entities support the requested performance interval on this server"
    )]
    NoSupportedEntities,

    /// `start` is strictly after `end`.
    #[error("invalid time range: start `{start}` is after end `{end}`")]
    InvalidTimeRange { start: Timestamp, end: Timestamp },

    /// An RFC 3339 timestamp could not be parsed.
    #[error("invalid RFC 3339 timestamp `{0}`")]
    InvalidTimestamp(String),

    /// No requested counter resolved on this server.
    #[error("counter `{0}` is not present in this server's counter catalog")]
    UnknownCounter(String),

    /// No requested counter had a unit the caller’s dimension accepts.
    #[error("counter `{counter}` reported unit `{unit}`, which is incompatible with the expected dimension")]
    DimensionMismatch { counter: String, unit: String },

    /// A typed entity wrapper was given a managed object of another kind.
    #[error("entity `{actual}` does not match expected type `{expected}`")]
    EntityKindMismatch {
        expected: MoTypesEnum,
        actual: MoTypesEnum,
    },

    /// Historical period `P` is not an enabled sampling period.
    #[error(
        "historical sampling period `{0}`s does not match any enabled PerfInterval on this server"
    )]
    InvalidInterval(i32),

    /// A [`crate::perf::PerfBatch`] resolved one object to two sampling periods.
    ///
    /// An object's samples share one timestamp grid and one `interval_secs`, so its
    /// groups must resolve to the same interval. Fetch the other period separately.
    #[error(
        "object `{}:{}` resolved to sampling periods {first_secs}s and {second_secs}s in one batch",
        entity.r#type.as_str(),
        entity.value
    )]
    MixedIntervals {
        entity: ManagedObjectReference,
        first_secs: i32,
        second_secs: i32,
    },

    /// Transport, session, or other client failure.
    #[error(transparent)]
    Client(#[from] client::Error),
}

/// A condition that does not fail the fetch.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PerfWarning {
    /// The name was not in the server catalog. Other counters were still queried.
    CounterNotOnServer { counter: String },
    /// The counter’s unit is outside the requested dimension. The counter was dropped.
    DimensionMismatch { counter: String, unit: String },
    /// A dynamic query sent a counter whose scope excludes this entity, and the server returned no series.
    ScopeMismatch {
        entity: ManagedObjectReference,
        counter: String,
    },
    /// Both levels were set and the counter level is higher than the interval level. The counter was still sent.
    LevelTooLow {
        counter: String,
        required_level: i32,
        interval_level: i32,
    },
    /// Both levels were set and the per-device level is higher than the interval level. The counter was still sent.
    PerDeviceLevelTooLow {
        counter: String,
        required_level: i32,
        interval_level: i32,
    },
    /// `Interval::Auto` selected a historical period for this kind.
    AutoIntervalFallback {
        entity_kind: MoTypesEnum,
        interval_secs: i32,
    },
    /// A realtime start is older than the retained hour. The query was still sent.
    RealtimeWindowTooOld { requested_start: Timestamp },
    /// A historical start is older than the interval keeps samples. The query was still sent,
    /// and the server returns only the retained part.
    HistoricalWindowTooOld {
        requested_start: Timestamp,
        interval_secs: i32,
        retention_secs: i32,
    },
    /// A sample interval differed from the period that was requested.
    IntervalSubstituted {
        requested_secs: i32,
        actual_secs: i32,
    },
    /// `AnyDimension` saw an `Other_` unit it cannot scale. The raw number is kept.
    UnscalableUnit { counter: String, unit: String },
    /// A decoded series named a counter id that is not in the registry.
    UnsupportedSeries { counter_id: CounterId },
    /// One chunk for this entity failed after another chunk succeeded. The status stays `Ok`.
    PartialChunk {
        entity: ManagedObjectReference,
        message: String,
    },
    /// `.metric` or `.metric_by_name` was set together with `.all_available()`. The named counters are not queried.
    ExplicitMetricsIgnored,
    /// `.groups()` was set together with explicit counters, and without `.all_available()`. The group filter is not applied.
    GroupsIgnored,
}

/// Why the interval table skipped an entity before `QueryPerf`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnsupportedReason {
    /// Realtime and historical summary are both unsupported.
    NotAProvider,
    /// Realtime was requested and this kind has no positive refresh rate.
    RealtimeUnsupported,
    /// A historical period was requested and this kind has no summary support, or the server has no intervals.
    HistoricalUnsupported,
}

/// Outcome for one requested managed object.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum EntityStatus {
    /// At least one timestamp was returned. Slots may still be `-1`.
    Ok,
    /// An availability filter produced no metric ids, so the entity was not sent.
    NoMetricsAvailable,
    /// The entity was sent and the server omitted it or returned no samples.
    NoSamples,
    /// Skipped by the interval table.
    Unsupported(UnsupportedReason),
    /// Every chunk that contained this entity failed locally.
    Failed(Arc<PerfError>),
}
