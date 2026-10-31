//! Named, unit-scaled queries over `PerformanceManager`.
//!
//! Counters are addressed as `group.name.rollup` (for example `cpu.usage.average`).
//! The number before the unit scale is the server’s raw integer. A raw value below `0`
//! is a gap: [`Series::last`](result::Series::last) skips trailing gaps, and
//! [`Series::last_slot`](result::Series::last_slot) reads only the newest timestamp.
//!
//! Historical presets ([`Interval::PAST_DAY`](query::Interval::PAST_DAY) and the week, month,
//! and year constants) are sampling periods in seconds. They are not `PerfInterval.key`.
//! The default chunk size is 256 metric ids. [`PerfBatch`](query::PerfBatch) packs
//! groups that need different counters or intervals into the same `QueryPerf` calls.
//! The library does not own a poll timer; callers tick and call
//! [`PreparedQuery::fetch`](query::PreparedQuery::fetch).
//!
//! This module is the only public path. Enable Cargo feature `perf`. There is no `vim_rs::core::perf`.

mod client;
pub mod decode;
mod engine;
pub mod entity;
pub mod error;
pub mod metric;
pub mod metrics;
pub mod query;
pub mod registry;
pub mod result;
pub mod typed;
pub mod units;
pub mod watermark;

#[cfg(feature = "perf-polars")]
pub mod polars;

pub use client::PerfClient;
pub use entity::{
    AnyEntity, ClusterDc, ClusterOnly, ClusterPool, ComputeScope, DatastoreOnly, DatastoreVm, DatastoreVmHost,
    HostCluster, HostOnly, VmHostCluster, VmOnly, VmOrHost, VmOrPool,
};
pub use error::{EntityStatus, PerfError, PerfWarning, UnsupportedReason};
pub use metric::{Metric, Rollup, StatsKind};
pub use query::{
    CompositeQuery, Instance, Interval, PerfBatch, PerfQuery, PreparedQuery, PreparedQueryBuilder,
    TypedPerfQuery, WireFormat,
};
pub use registry::{CounterId, CounterRegistry};
pub use result::{CompositePerfResult, EntitySeries, PerfResult, Row, Sample, Timestamp};
pub use units::{
    AnyDimension, Bytes, BytesPerSecond, Celsius, Count, Energy, Frequency, Hertz, Joules, Memory,
    Number, Percent, Power, Ratio, Seconds, Temperature, Throughput, Time, Watts,
};
pub use watermark::SampleWatermark;
