//! Instance selectors, interval presets, and wire format.

/// Which device series to request.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Instance {
    /// `""` — the aggregate series.
    Aggregate,
    /// `"*"` — aggregate plus every per-device series.
    All,
    /// A device key such as `vmnic0` or `scsi0:0`.
    Named(String),
}

impl Instance {
    /// `""` becomes [`Instance::Aggregate`] and `"*"` becomes [`Instance::All`].
    pub fn from_wire(s: &str) -> Self {
        match s {
            "" => Instance::Aggregate,
            "*" => Instance::All,
            other => Instance::Named(other.to_string()),
        }
    }

    /// Wire string sent as `PerfMetricId.instance`.
    pub fn as_wire_str(&self) -> &str {
        match self {
            Instance::Aggregate => "",
            Instance::All => "*",
            Instance::Named(s) => s,
        }
    }

    /// True only for the empty wire string.
    pub fn is_aggregate(&self) -> bool {
        self.as_wire_str().is_empty()
    }
}

impl Default for Instance {
    fn default() -> Self {
        Instance::Aggregate
    }
}

impl std::fmt::Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire_str())
    }
}

/// Sampling interval. Historical presets are sampling periods, not `PerfInterval.key`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum Interval {
    /// The provider refresh rate.
    Realtime,
    /// A historical sampling period in seconds.
    Historical(i32),
    /// Realtime when the kind supports it and the start is inside the last hour, otherwise the finest covering historical period.
    #[default]
    Auto,
}

impl Interval {
    /// 300-second sampling period (past day on the measured vCenter).
    pub const PAST_DAY: Interval = Interval::Historical(300);
    /// 1800-second sampling period.
    pub const PAST_WEEK: Interval = Interval::Historical(1800);
    /// 7200-second sampling period.
    pub const PAST_MONTH: Interval = Interval::Historical(7200);
    /// 86400-second sampling period.
    pub const PAST_YEAR: Interval = Interval::Historical(86400);
}

/// Format of `QueryPerf` samples.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum WireFormat {
    /// CSV on SOAP. CSV on JSON, except [`crate::perf::PerfQuery::latest`]`(1)`, which uses normal JSON.
    #[default]
    Auto,
    /// `PerfEntityMetric` / `PerfMetricIntSeries`.
    Normal,
    /// `PerfEntityMetricCSV`.
    Csv,
}

#[doc(hidden)]
pub use super::engine::{
    plan_intervals, EntityDecision, IntervalCase, PlannedEntity, PlannedQuery,
};
pub use super::engine::{
    CompositeQuery, PerfQuery, PreparedQuery, PreparedQueryBuilder, TypedPerfQuery,
};
