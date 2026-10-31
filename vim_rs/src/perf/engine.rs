//! Query builders and the interval, packing, and fetch engine behind [`crate::perf::PerfClient`].
//!
//! Five builders share the counter, instance, and interval methods:
//!
//! * [`PerfQuery`] stores its entities. [`PerfQuery::fetch`] consumes it.
//! * [`PreparedQueryBuilder`] stores the spec only. [`PreparedQuery`] takes the
//!   entity list on each fetch and caches counter-id resolution until
//!   [`crate::perf::PerfClient::refresh_registry`] replaces the catalog.
//! * [`TypedPerfQuery`] is a [`PerfQuery`] locked to one [`crate::perf::typed::EntityKind`].
//!   [`TypedPerfQuery::metric`] requires [`crate::perf::typed::Admits`].
//! * [`CompositeQuery`] is one `QueryPerfComposite` call for a host and the
//!   children the server returns. It does not chunk, and it always uses normal format.
//! * [`PerfBatch`] is several groups. Each group has its own objects, counters,
//!   and interval. [`PerfBatch::fetch`] packs them into `QueryPerf`.
//!
//! Wire `start_time` is exclusive and `end_time` is inclusive. The client sends
//! the instants it was given. Historical queries omit `max_sample`. Realtime
//! `latest(n)` sends it. A raw sample below `0` is a gap.

use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use indexmap::IndexMap;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::core::client::{self, Transport};
use crate::perf::client::{self as session, PerfClient};
use crate::perf::decode::{self, decode_metrics};
use crate::perf::entity::{mo_type_bit, EntityScope, MASK_ALL};
use crate::perf::error::{EntityStatus, PerfError, PerfWarning, UnsupportedReason};
use crate::perf::metric::{Metric, Rollup, StatsKind};
use crate::perf::query::{Instance, Interval, WireFormat};
use crate::perf::registry::{CounterDef, CounterRegistry};
use crate::perf::result::{EntitySeries, PerfResult, Timestamp};
use crate::perf::typed::{Admits, Entity, EntityKind};
use crate::perf::units::{unit_is_scalable, Dimension};
use crate::types::enums::{MoTypesEnum, PerfFormatEnum};
use crate::types::structs::{ManagedObjectReference, PerfInterval, PerfMetricId, PerfQuerySpec};
use crate::types::traits::PerfEntityMetricBaseTrait;

#[derive(Clone)]
pub(crate) struct QuerySpec {
    metrics: Vec<ErasedMetric>,
    names: Vec<String>,
    all_available: bool,
    groups: Vec<String>,
    default_instance: Instance,
    interval: Interval,
    time: TimeBound,
    latest: Option<i32>,
    format: WireFormat,
    chunk_size: usize,
    time_chunk: Option<Duration>,
    concurrency: usize,
}

#[derive(Clone)]
enum TimeBound {
    None,
    Since(SystemTime),
    Between { start: SystemTime, end: SystemTime },
    SinceAgo(Duration),
}

#[derive(Clone)]
struct ErasedMetric {
    group: &'static str,
    name: &'static str,
    rollup: Rollup,
    stats: Option<StatsKind>,
    accepts: fn(&crate::types::enums::PerformanceManagerUnitEnum) -> bool,
    unconstrained: bool,
    scope: u8,
    instance: Option<Instance>,
}

#[derive(Clone)]
pub(crate) struct ResolvedMetric {
    def: Arc<CounterDef>,
    instance: Instance,
    scope: u8,
}

pub(crate) struct CachedResolve {
    generation: u64,
    metrics: Arc<[ResolvedMetric]>,
    ids: Arc<[PerfMetricId]>,
    warnings: Arc<[PerfWarning]>,
}

pub(crate) enum WindowOverride {
    FromSpec,
    Since(SystemTime),
    Incremental(HashMap<ManagedObjectReference, Timestamp>),
}

impl Default for QuerySpec {
    fn default() -> Self {
        Self {
            metrics: Vec::new(),
            names: Vec::new(),
            all_available: false,
            groups: Vec::new(),
            default_instance: Instance::Aggregate,
            interval: Interval::Auto,
            time: TimeBound::None,
            latest: None,
            format: WireFormat::Auto,
            chunk_size: 256,
            time_chunk: None,
            concurrency: 4,
        }
    }
}

impl QuerySpec {
    fn push<D: Dimension, S: EntityScope>(
        &mut self,
        metric: Metric<D, S>,
        instance: Option<Instance>,
    ) {
        self.metrics.push(ErasedMetric {
            group: metric.group,
            name: metric.name,
            rollup: metric.rollup,
            stats: metric.stats,
            accepts: D::accepts,
            unconstrained: std::any::TypeId::of::<D>()
                == std::any::TypeId::of::<crate::perf::units::AnyDimension>(),
            scope: S::MASK,
            instance,
        });
    }
}

/// One-shot `QueryPerf`.
///
/// Entities are stored here and may mix managed-object kinds. Every entity
/// uses this builder’s counters, interval, and window. [`PerfBatch`] gives
/// each group its own counters and interval, then packs those specs into
/// `QueryPerf`. [`Self::fetch`] consumes the builder and does not cache
/// counter resolution. [`Self::typed_entities`] replaces the entity list and
/// continues as a [`TypedPerfQuery`].
pub struct PerfQuery {
    client: PerfClient,
    pub(crate) spec: QuerySpec,
    entities: Vec<ManagedObjectReference>,
}

/// Spec for a query that will be fetched more than once.
///
/// [`Self::build`] does not call the server. This builder has `since_ago` and
/// no absolute `since` or `between`; those bounds are [`PreparedQuery::fetch_since`].
/// `since_ago` is stored and subtracted from the clock at fetch time.
pub struct PreparedQueryBuilder {
    client: PerfClient,
    spec: QuerySpec,
}

/// A built spec plus a cache of resolved counter ids.
///
/// The cache is reused while [`crate::perf::CounterRegistry`] keeps the same
/// generation. [`Self::fetch`] takes the objects for that call, so the inventory
/// can change between ticks. This type does not sleep; the caller owns the timer.
pub struct PreparedQuery {
    client: PerfClient,
    spec: QuerySpec,
    cache: Mutex<Option<CachedResolve>>,
}

/// One `QueryPerfComposite` call: a host and the children the server includes.
///
/// The host must be a `HostSystem`. [`Self::fetch`] checks that before any RPC
/// and returns [`crate::perf::PerfError::EntityKindMismatch`] otherwise.
/// [`crate::perf::CompositePerfResult::host`] is `None` when the server omits
/// the host. [`crate::perf::CompositePerfResult::children`] contains only the
/// entities the server returned, so a powered-off virtual machine that is not
/// a child of the host is absent.
///
/// The call is a single RPC in normal format. [`Self::format`], [`Self::chunk_size`],
/// [`Self::time_chunk`], [`Self::concurrency`], and [`Self::all_available`] do not
/// change that. [`Self::latest`] truncates each decoded series after the call;
/// `max_sample` is not sent.
pub struct CompositeQuery {
    client: PerfClient,
    host: ManagedObjectReference,
    spec: QuerySpec,
}

/// One-shot query locked to entity kind `K`.
///
/// Produced by [`PerfQuery::typed_entity`] or [`PerfQuery::typed_entities`],
/// which discard any entities already added. Further entities must be `K`.
/// [`Self::metric`] compiles only when the counter scope [`crate::perf::typed::Admits`] `K`.
/// [`Self::metric_unchecked`] skips that check for a counter outside the generated scope.
/// [`Self::fetch`] consumes the builder, like [`PerfQuery::fetch`].
pub struct TypedPerfQuery<K: EntityKind> {
    client: PerfClient,
    spec: QuerySpec,
    entities: Vec<ManagedObjectReference>,
    _k: PhantomData<fn() -> K>,
}

macro_rules! common_methods {
    ($($head:tt)*) => {
        $($head)* {
            /// Default device instance for later [`Self::metric`] and [`Self::metric_by_name`] calls.
            ///
            /// [`Instance::Aggregate`] is the empty wire string, [`Instance::All`] is `"*"`,
            /// and [`Instance::Named`] is one device key such as `naa.…` or `vmnic0`.
            /// [`Self::metric_with_instance`] stores its own instance and ignores this default.
            pub fn instances(mut self, instance: Instance) -> Self {
                self.spec.default_instance = instance;
                self
            }
            /// Sampling period. The default is [`Interval::Auto`].
            ///
            /// Presets such as [`Interval::PAST_DAY`] are periods in seconds, not
            /// `PerfInterval.key`. `Auto` uses realtime when that provider supports
            /// it and the start lies inside the last hour; otherwise it uses the
            /// finest enabled historical period that covers the window.
            pub fn interval(mut self, interval: Interval) -> Self {
                self.spec.interval = interval;
                self
            }
            /// Start at `now - ago` on each fetch.
            ///
            /// On a [`PreparedQuery`] the subtraction happens when [`PreparedQuery::fetch`]
            /// runs, not when the query was built.
            pub fn since_ago(mut self, ago: Duration) -> Self {
                self.spec.time = TimeBound::SinceAgo(ago);
                self
            }
            /// Keep the newest `n` samples. `n <= 0` clears the limit.
            ///
            /// Realtime `QueryPerf` sends `max_sample` and leaves the time fields unset
            /// when no window was set. A realtime window is sent as given, and
            /// `max_sample` is added only when `latest` is also set. Historical queries
            /// omit `max_sample` and the decoded series is truncated to `n`.
            /// [`PreparedQuery::fetch_since`] and [`PreparedQuery::fetch_incremental`]
            /// omit `max_sample` and do not truncate. [`CompositeQuery::fetch`] also
            /// omits `max_sample` and truncates after decode. Without a window, a realtime
            /// composite asks for the last `(n + 2)` periods, and never less than five minutes.
            pub fn latest(mut self, n: i32) -> Self {
                self.spec.latest = (n > 0).then_some(n);
                self
            }
            /// Wire format of `QueryPerf` samples.
            ///
            /// [`WireFormat::Auto`] sends CSV. On JSON, `latest(1)` sends normal format
            /// instead. With the `xml` feature, SOAP always sends CSV.
            /// [`CompositeQuery::fetch`] always sends normal format.
            pub fn format(mut self, format: WireFormat) -> Self {
                self.spec.format = format;
                self
            }
            /// Maximum metric ids in one `QueryPerf` call. The default is 256. Values below 1 become 1.
            ///
            /// Entities are packed in request order until that budget is full, so a host
            /// and a virtual machine can share a round trip. [`CompositeQuery::fetch`]
            /// is one RPC and ignores this limit. On a [`PerfBatch`], set the budget
            /// with [`PerfBatch::chunk_size`]; the group builder’s value is not used.
            pub fn chunk_size(mut self, n: usize) -> Self {
                self.spec.chunk_size = n.max(1);
                self
            }
            /// Split a historical window into slices of at most `window`.
            ///
            /// Realtime ignores this. A zero window is ignored. Each slice is its own
            /// spec: `start_time` is exclusive and `end_time` is inclusive.
            /// [`CompositeQuery::fetch`] does not slice.
            pub fn time_chunk(mut self, window: Duration) -> Self {
                self.spec.time_chunk = Some(window);
                self
            }
            /// How many `QueryPerf` calls run at once. The default is 4. Values below 1 become 1.
            ///
            /// [`CompositeQuery::fetch`] is a single call.
            pub fn concurrency(mut self, n: usize) -> Self {
                self.spec.concurrency = n.max(1);
                self
            }
            /// Query every counter the server lists for each object.
            ///
            /// Named counters from [`Self::metric`] or [`Self::metric_by_name`] are dropped,
            /// and the fetch records [`crate::perf::PerfWarning::ExplicitMetricsIgnored`].
            /// An object with no available ids becomes [`crate::perf::EntityStatus::NoMetricsAvailable`]
            /// and is not sent. [`CompositeQuery`] does not query availability; name its counters.
            ///
            /// The list comes from `QueryAvailablePerfMetric` with no begin or end, so it is the
            /// object's current counters and devices at that interval, not those of the query
            /// window. A historical window can miss a device that has since been removed. The
            /// list is cached per object and interval until [`crate::perf::PerfClient::refresh_registry`].
            pub fn all_available(mut self) -> Self {
                self.spec.all_available = true;
                self
            }
            /// Availability filter. Group names are catalog groups such as `"cpu"` and `"mem"`.
            ///
            /// With no explicit metrics this queries every counter in these groups.
            /// Combined with [`Self::metric`] or [`Self::metric_by_name`] and without
            /// [`Self::all_available`], the filter is ignored and the fetch records
            /// [`crate::perf::PerfWarning::GroupsIgnored`].
            ///
            /// Uses the same availability list as [`Self::all_available`], which does not
            /// depend on the query window.
            pub fn groups(mut self, groups: &[&str]) -> Self {
                self.spec.groups = groups.iter().map(|group| (*group).to_string()).collect();
                self
            }
            /// One `group.name.rollup` string, such as `cpu.usage.average`.
            ///
            /// When several stats kinds share that name, the rate counter is preferred.
            /// An unknown name is dropped with [`crate::perf::PerfWarning::CounterNotOnServer`].
            /// The fetch fails with [`crate::perf::PerfError::UnknownCounter`] when nothing remains.
            pub fn metric_by_name(mut self, name: &str) -> Self {
                self.spec.names.push(name.to_string());
                self
            }
            /// Several `group.name.rollup` strings. See [`Self::metric_by_name`].
            pub fn metrics_by_name(mut self, names: &[&str]) -> Self {
                self.spec.names.extend(names.iter().map(|name| (*name).to_string()));
                self
            }
        }
    };
}

macro_rules! metric_methods {
    ($($head:tt)*) => {
        $($head)* {
            /// Counter to query. The [`Self::instances`] default is applied when the query is fetched.
            ///
            /// The scope parameter does not filter objects on this builder. A counter
            /// whose scope excludes an entity is still sent; a missing series records
            /// [`crate::perf::PerfWarning::ScopeMismatch`]. [`TypedPerfQuery::metric`]
            /// rejects that mismatch at compile time.
            pub fn metric<D: Dimension, S: EntityScope>(mut self, metric: Metric<D, S>) -> Self {
                self.spec.push(metric, None);
                self
            }
            /// Counter plus a device instance. This instance ignores [`Self::instances`].
            ///
            /// [`Instance::All`] requests the aggregate series and every device. Read
            /// the devices back with [`crate::perf::result::EntitySeries::devices`].
            pub fn metric_with_instance<D: Dimension, S: EntityScope>(
                mut self,
                metric: Metric<D, S>,
                instance: Instance,
            ) -> Self {
                self.spec.push(metric, Some(instance));
                self
            }
        }
    };
}

macro_rules! absolute_time {
    ($($head:tt)*) => {
        $($head)* {
            /// Exclusive start. The end is the time of the fetch.
            ///
            /// [`PreparedQueryBuilder`] has no absolute start. Use [`PreparedQuery::fetch_since`].
            pub fn since(mut self, start: SystemTime) -> Self {
                self.spec.time = TimeBound::Since(start);
                self
            }
            /// Exclusive start and inclusive end. The instants are sent unchanged.
            ///
            /// `start > end` fails the fetch with [`crate::perf::PerfError::InvalidTimeRange`].
            pub fn between(mut self, start: SystemTime, end: SystemTime) -> Self {
                self.spec.time = TimeBound::Between { start, end };
                self
            }
        }
    };
}

impl PerfQuery {
    pub(crate) fn new(client: PerfClient) -> Self {
        Self {
            client,
            spec: QuerySpec::default(),
            entities: Vec::new(),
        }
    }

    /// Append one managed object.
    ///
    /// Kinds may be mixed. They share `QueryPerf` calls until [`Self::chunk_size`] is reached.
    pub fn entity(mut self, entity: &ManagedObjectReference) -> Self {
        self.entities.push(entity.clone());
        self
    }

    /// Replace the object list.
    ///
    /// Order is the order of [`crate::perf::PerfResult::entities`].
    pub fn entities(mut self, entities: &[ManagedObjectReference]) -> Self {
        self.entities = entities.to_vec();
        self
    }

    /// Replace the entity list with one typed object and continue as a [`TypedPerfQuery`].
    ///
    /// Any objects already added with [`Self::entity`] are discarded.
    pub fn typed_entity<K: EntityKind>(self, entity: Entity<K>) -> TypedPerfQuery<K> {
        self.typed_entities(std::slice::from_ref(&entity))
    }

    /// Replace the entity list with typed objects of one kind and continue as a [`TypedPerfQuery`].
    ///
    /// Further [`TypedPerfQuery::entity`] calls must use that same kind. Any objects
    /// already added are discarded.
    pub fn typed_entities<K: EntityKind>(self, entities: &[Entity<K>]) -> TypedPerfQuery<K> {
        TypedPerfQuery {
            client: self.client,
            spec: self.spec,
            entities: entities
                .iter()
                .map(|entity| entity.as_moref().clone())
                .collect(),
            _k: PhantomData,
        }
    }

    /// Resolve counters, probe provider support per kind, and run `QueryPerf`.
    ///
    /// Every requested object is in the result. An object the server omits is
    /// [`crate::perf::EntityStatus::NoSamples`]. The builder is consumed and counter
    /// resolution is not cached; use [`PreparedQuery`] to poll.
    pub async fn fetch(self) -> Result<PerfResult, PerfError> {
        execute(
            &self.client,
            &self.spec,
            &self.entities,
            WindowOverride::FromSpec,
            None,
        )
        .await
    }
}
common_methods!(impl PerfQuery);
metric_methods!(impl PerfQuery);
absolute_time!(impl PerfQuery);

impl PreparedQueryBuilder {
    pub(crate) fn new(client: PerfClient) -> Self {
        Self {
            client,
            spec: QuerySpec::default(),
        }
    }

    /// Freeze the spec.
    ///
    /// Metric names are resolved on the first fetch and reused until
    /// [`crate::perf::PerfClient::refresh_registry`] replaces the catalog.
    pub fn build(self) -> PreparedQuery {
        PreparedQuery {
            client: self.client,
            spec: self.spec,
            cache: Mutex::new(None),
        }
    }
}
common_methods!(impl PreparedQueryBuilder);
metric_methods!(impl PreparedQueryBuilder);

impl PreparedQuery {
    /// Client that owns the registry, provider-cap cache, and availability cache.
    pub fn client(&self) -> &PerfClient {
        &self.client
    }

    /// Fetch `entities` with the built interval and window.
    ///
    /// `since_ago` is subtracted from the clock on this call, not when the query
    /// was built. The object list may differ from the previous tick.
    pub async fn fetch(
        &self,
        entities: &[ManagedObjectReference],
    ) -> Result<PerfResult, PerfError> {
        execute(
            &self.client,
            &self.spec,
            entities,
            WindowOverride::FromSpec,
            Some(&self.cache),
        )
        .await
    }

    /// One exclusive start for every object. Overrides `since_ago`.
    ///
    /// Omits `max_sample` and does not truncate [`PreparedQueryBuilder::latest`].
    /// The end is the time of this fetch.
    pub async fn fetch_since(
        &self,
        entities: &[ManagedObjectReference],
        since: SystemTime,
    ) -> Result<PerfResult, PerfError> {
        execute(
            &self.client,
            &self.spec,
            entities,
            WindowOverride::Since(since),
            Some(&self.cache),
        )
        .await
    }

    /// Per-object start from [`crate::perf::SampleWatermark::entity_high_water`].
    /// Does not modify `watermark`.
    ///
    /// That instant is the earliest mark across the object's series, so a series
    /// that is ahead is queried again from the slower series. Those repeats are
    /// duplicates. [`crate::perf::SampleWatermark::filter_new`] keeps rows strictly
    /// newer than each series' own mark. An object with a mark omits `max_sample`.
    /// An object with no mark uses the built window and `latest`.
    pub async fn fetch_incremental(
        &self,
        entities: &[ManagedObjectReference],
        watermark: &crate::perf::watermark::SampleWatermark,
    ) -> Result<PerfResult, PerfError> {
        let marks = entities
            .iter()
            .filter_map(|entity| {
                watermark
                    .entity_high_water(entity)
                    .map(|ts| (entity.clone(), ts))
            })
            .collect();
        execute(
            &self.client,
            &self.spec,
            entities,
            WindowOverride::Incremental(marks),
            Some(&self.cache),
        )
        .await
    }
}

impl CompositeQuery {
    pub(crate) fn new(client: PerfClient, host: ManagedObjectReference) -> Self {
        Self {
            client,
            host,
            spec: QuerySpec::default(),
        }
    }

    /// Run `QueryPerfComposite`.
    ///
    /// A host whose type is not `HostSystem` returns
    /// [`crate::perf::PerfError::EntityKindMismatch`] and does not call the server.
    pub async fn fetch(self) -> Result<crate::perf::result::CompositePerfResult, PerfError> {
        if self.host.r#type != MoTypesEnum::HostSystem {
            return Err(PerfError::EntityKindMismatch {
                expected: MoTypesEnum::HostSystem,
                actual: self.host.r#type,
            });
        }
        execute_composite(&self.client, &self.spec, &self.host).await
    }
}
common_methods!(impl CompositeQuery);
metric_methods!(impl CompositeQuery);
absolute_time!(impl CompositeQuery);

impl<K: EntityKind> TypedPerfQuery<K> {
    /// Append one object of kind `K`.
    pub fn entity(mut self, entity: &Entity<K>) -> Self {
        self.entities.push(entity.as_moref().clone());
        self
    }

    /// Replace the object list. Every element is already kind `K`.
    pub fn entities(mut self, entities: &[Entity<K>]) -> Self {
        self.entities = entities
            .iter()
            .map(|entity| entity.as_moref().clone())
            .collect();
        self
    }

    /// Counter whose scope includes `K`.
    ///
    /// A scope that does not [`crate::perf::typed::Admits`] `K` fails to compile.
    /// The diagnostic names the scope and the kind. Widen a Tier 3b counter with
    /// [`Metric::with_scope`](crate::perf::metric::Metric::with_scope) or pass it to
    /// [`Self::metric_unchecked`].
    pub fn metric<D: Dimension, S: EntityScope>(mut self, metric: Metric<D, S>) -> Self
    where
        S: Admits<K>,
    {
        self.spec.push(metric, None);
        self
    }

    /// [`Self::metric`] plus a device instance. This instance ignores [`Self::instances`].
    pub fn metric_with_instance<D: Dimension, S: EntityScope>(
        mut self,
        metric: Metric<D, S>,
        instance: Instance,
    ) -> Self
    where
        S: Admits<K>,
    {
        self.spec.push(metric, Some(instance));
        self
    }

    /// Counter without the [`crate::perf::typed::Admits`] check.
    ///
    /// The counter is still sent. A missing series records
    /// [`crate::perf::PerfWarning::ScopeMismatch`] when the compiled scope excludes
    /// this entity's kind.
    pub fn metric_unchecked<D: Dimension, S: EntityScope>(mut self, metric: Metric<D, S>) -> Self {
        self.spec.push(metric, None);
        self
    }

    /// [`Self::metric_unchecked`] plus a device instance.
    pub fn metric_with_instance_unchecked<D: Dimension, S: EntityScope>(
        mut self,
        metric: Metric<D, S>,
        instance: Instance,
    ) -> Self {
        self.spec.push(metric, Some(instance));
        self
    }

    /// Resolve counters and run `QueryPerf` for the stored `K` objects.
    ///
    /// Consumed, like [`PerfQuery::fetch`]. Counter resolution is not cached.
    pub async fn fetch(self) -> Result<PerfResult, PerfError> {
        execute(
            &self.client,
            &self.spec,
            &self.entities,
            WindowOverride::FromSpec,
            None,
        )
        .await
    }
}
common_methods!(impl<K: EntityKind> TypedPerfQuery<K>);
absolute_time!(impl<K: EntityKind> TypedPerfQuery<K>);

struct BatchGroup {
    spec: QuerySpec,
    entities: Vec<ManagedObjectReference>,
}

impl PerfQuery {
    fn into_group(self) -> BatchGroup {
        BatchGroup {
            spec: self.spec,
            entities: self.entities,
        }
    }
}

impl<K: EntityKind> TypedPerfQuery<K> {
    fn into_group(self) -> BatchGroup {
        BatchGroup {
            spec: self.spec,
            entities: self.entities,
        }
    }
}

/// Several `QueryPerf` groups in one fetch.
///
/// Each group has its own objects, counters, interval, and time window.
/// [`Self::fetch`] plans the groups separately, then packs one `PerfQuerySpec`
/// per object into as few `QueryPerf` calls as [`Self::chunk_size`] allows.
/// An object that appears in more than one group keeps every counter. Its
/// samples share one timestamp grid. `latest` truncates that grid only when
/// every group that names the object sets the same limit. Those groups must
/// resolve to the same sampling period for that object, otherwise [`Self::fetch`]
/// fails with [`PerfError::MixedIntervals`] before any `QueryPerf` call.
///
/// [`Self::format`], [`Self::chunk_size`], and [`Self::concurrency`] apply to
/// the whole batch. The same methods on a group builder are ignored.
/// [`PerfQuery::time_chunk`] on a group still slices that group’s window.
/// [`WireFormat::Auto`] sends CSV, including when a group uses `latest(1)`.
pub struct PerfBatch {
    client: PerfClient,
    groups: Vec<BatchGroup>,
    format: WireFormat,
    chunk_size: usize,
    concurrency: usize,
}

impl PerfBatch {
    pub(crate) fn new(client: PerfClient) -> Self {
        let defaults = QuerySpec::default();
        Self {
            client,
            groups: Vec::new(),
            format: defaults.format,
            chunk_size: defaults.chunk_size,
            concurrency: defaults.concurrency,
        }
    }

    /// Add a group built from a [`PerfQuery`].
    ///
    /// The closure receives an empty query. Return it after setting objects,
    /// counters, and the interval. [`PerfQuery::format`], [`PerfQuery::chunk_size`],
    /// and [`PerfQuery::concurrency`] do not affect the batch.
    pub fn group(mut self, build: impl FnOnce(PerfQuery) -> PerfQuery) -> Self {
        self.groups
            .push(build(PerfQuery::new(self.client.clone())).into_group());
        self
    }

    /// Add a group locked to one managed-object kind.
    ///
    /// [`TypedPerfQuery::metric`] accepts a counter only when its scope includes
    /// that kind. The closure receives an empty [`PerfQuery`]; call
    /// [`PerfQuery::typed_entity`] or [`PerfQuery::typed_entities`] and return
    /// the typed query. Format, chunk size, and concurrency set inside the
    /// closure are ignored.
    pub fn typed_group<K: EntityKind>(
        mut self,
        build: impl FnOnce(PerfQuery) -> TypedPerfQuery<K>,
    ) -> Self {
        self.groups
            .push(build(PerfQuery::new(self.client.clone())).into_group());
        self
    }

    /// Wire format for every group. [`WireFormat::Auto`] sends CSV.
    pub fn format(mut self, format: WireFormat) -> Self {
        self.format = format;
        self
    }

    /// Maximum metric ids in one `QueryPerf` call. The default is 256. Values below 1 become 1.
    ///
    /// Specs from every group are packed in group order, then object order.
    pub fn chunk_size(mut self, n: usize) -> Self {
        self.chunk_size = n.max(1);
        self
    }

    /// How many `QueryPerf` calls run at once. The default is 4. Values below 1 become 1.
    ///
    /// Availability lookups for `.all_available()` and `.groups()` use this limit too.
    pub fn concurrency(mut self, n: usize) -> Self {
        self.concurrency = n.max(1);
        self
    }

    /// Plan every group and run `QueryPerf`.
    ///
    /// An empty batch, or a group with no objects, returns
    /// [`crate::perf::PerfError::NoEntities`]. A group with no counters returns
    /// [`crate::perf::PerfError::NoMetrics`]. The result lists each object once,
    /// in the order groups and objects were added.
    pub async fn fetch(self) -> Result<PerfResult, PerfError> {
        execute_batch(
            &self.client,
            &self.groups,
            self.format,
            self.chunk_size,
            self.concurrency,
        )
        .await
    }
}

/// Inputs for one object in [`plan_intervals`].
#[doc(hidden)]
#[derive(Debug)]
pub struct IntervalCase {
    /// Object the table is deciding.
    pub entity: ManagedObjectReference,
    /// Provider summary for this object's kind.
    pub caps: crate::perf::registry::ProviderCaps,
    /// Exclusive start, already resolved from `since` / `since_ago`.
    pub start: Option<SystemTime>,
    /// Inclusive end.
    pub end: Option<SystemTime>,
    /// Incremental pulls omit `max_sample` and skip latest-N truncation.
    pub incremental: bool,
}

/// One object's interval-table result.
#[doc(hidden)]
#[derive(Debug)]
pub struct PlannedEntity {
    /// Object this decision is for.
    pub entity: ManagedObjectReference,
    /// Skip, or the `PerfQuerySpec` fields to send.
    pub decision: EntityDecision,
}

/// Skip reason or a concrete query.
#[doc(hidden)]
#[derive(Debug)]
pub enum EntityDecision {
    /// Interval table skipped this object before `QueryPerf`.
    Unsupported(UnsupportedReason),
    /// Fields to send for this object.
    Query(PlannedQuery),
}

/// Resolved wire bounds for one object.
#[doc(hidden)]
#[derive(Debug)]
pub struct PlannedQuery {
    /// `PerfQuerySpec.intervalId` (a sampling period, not `PerfInterval.key`).
    pub interval_id: i32,
    /// Exclusive start. `None` leaves the field unset.
    pub start: Option<SystemTime>,
    /// Inclusive end.
    pub end: Option<SystemTime>,
    /// Set only for realtime `latest(n)` that is not incremental.
    pub max_sample: Option<i32>,
    /// Newest samples to keep after a historical `latest(n)` decode.
    pub truncate_newest: Option<usize>,
    /// Realtime queries ignore [`QuerySpec::time_chunk`].
    pub realtime: bool,
    /// Warnings produced by the table (fallback, realtime window).
    pub warnings: Vec<PerfWarning>,
}

/// First-match interval table. `Err(NoSupportedEntities)` when every object is skipped.
/// `Err(InvalidInterval)` aborts before any object is queried.
#[doc(hidden)]
pub fn plan_intervals(
    cases: &[IntervalCase],
    intervals: &[PerfInterval],
    requested: Interval,
    latest: Option<i32>,
    now: SystemTime,
) -> Result<Vec<PlannedEntity>, PerfError> {
    let planned = plan_entities(cases, intervals, requested, latest, now)?;
    if planned
        .iter()
        .all(|item| matches!(item.decision, EntityDecision::Unsupported(_)))
    {
        Err(PerfError::NoSupportedEntities)
    } else {
        Ok(planned)
    }
}

fn plan_entities(
    cases: &[IntervalCase],
    intervals: &[PerfInterval],
    requested: Interval,
    latest: Option<i32>,
    now: SystemTime,
) -> Result<Vec<PlannedEntity>, PerfError> {
    let mut planned = Vec::with_capacity(cases.len());
    for case in cases {
        if let (Some(start), Some(end)) = (case.start, case.end) {
            if start > end {
                return Err(PerfError::InvalidTimeRange {
                    start: Timestamp(start),
                    end: Timestamp(end),
                });
            }
        }
        planned.push(PlannedEntity {
            entity: case.entity.clone(),
            decision: decide(case, intervals, requested, latest, now)?,
        });
    }
    Ok(planned)
}

fn decide(
    case: &IntervalCase,
    intervals: &[PerfInterval],
    requested: Interval,
    latest: Option<i32>,
    now: SystemTime,
) -> Result<EntityDecision, PerfError> {
    if !case.caps.current_supported && !case.caps.summary_supported {
        return Ok(EntityDecision::Unsupported(UnsupportedReason::NotAProvider));
    }
    match requested {
        Interval::Realtime => Ok(decide_realtime(case, latest, now)),
        Interval::Historical(period) => decide_historical(case, intervals, period, latest, now),
        Interval::Auto => decide_auto(case, intervals, latest, now),
    }
}

fn decide_realtime(case: &IntervalCase, latest: Option<i32>, now: SystemTime) -> EntityDecision {
    let Some(rate) = case.caps.refresh_rate.filter(|rate| *rate > 0) else {
        return EntityDecision::Unsupported(UnsupportedReason::RealtimeUnsupported);
    };
    if !case.caps.current_supported {
        return EntityDecision::Unsupported(UnsupportedReason::RealtimeUnsupported);
    }
    EntityDecision::Query(realtime_query(case, rate, latest, now))
}

fn realtime_query(
    case: &IntervalCase,
    rate: i32,
    latest: Option<i32>,
    now: SystemTime,
) -> PlannedQuery {
    let has_window = case.start.is_some() || case.end.is_some();
    let mut warnings = Vec::new();
    if let Some(start) = case.start {
        if older_than_hour(start, now) {
            warnings.push(PerfWarning::RealtimeWindowTooOld {
                requested_start: Timestamp(start),
            });
        }
    }
    // A realtime window without latest still sends the requested bounds and no max_sample.
    // latest-N without a window sends max_sample and leaves start and end unset.
    // Incremental pulls omit max_sample even when latest is set.
    let max_sample = if case.incremental || (has_window && latest.is_none()) {
        None
    } else {
        latest
    };
    let (start, end) = if has_window {
        (case.start, case.end)
    } else {
        (None, None)
    };
    PlannedQuery {
        interval_id: rate,
        start,
        end,
        max_sample,
        truncate_newest: None,
        realtime: true,
        warnings,
    }
}

fn decide_historical(
    case: &IntervalCase,
    intervals: &[PerfInterval],
    period: i32,
    latest: Option<i32>,
    now: SystemTime,
) -> Result<EntityDecision, PerfError> {
    let enabled = enabled_intervals(intervals);
    if !case.caps.summary_supported || enabled.is_empty() {
        return Ok(EntityDecision::Unsupported(
            UnsupportedReason::HistoricalUnsupported,
        ));
    }
    let Some((_, length)) = enabled
        .iter()
        .find(|(sampling, _)| *sampling == period)
        .copied()
    else {
        return Err(PerfError::InvalidInterval(period));
    };
    Ok(EntityDecision::Query(historical_query(
        case,
        period,
        length,
        latest,
        now,
        Vec::new(),
    )))
}

fn decide_auto(
    case: &IntervalCase,
    intervals: &[PerfInterval],
    latest: Option<i32>,
    now: SystemTime,
) -> Result<EntityDecision, PerfError> {
    let refresh = case.caps.refresh_rate.filter(|rate| *rate > 0);
    let enabled = enabled_intervals(intervals);
    let within_hour = case
        .start
        .map(|start| !older_than_hour(start, now))
        .unwrap_or(true);
    let realtime_ok = case.caps.current_supported && refresh.is_some();
    if realtime_ok && (within_hour || enabled.is_empty() || !case.caps.summary_supported) {
        return Ok(EntityDecision::Query(realtime_query(
            case,
            refresh.unwrap(),
            latest,
            now,
        )));
    }
    if case.caps.summary_supported && !enabled.is_empty() {
        let (period, length) = choose_period(&enabled, case.start, now);
        let mut warnings = vec![PerfWarning::AutoIntervalFallback {
            entity_kind: case.entity.r#type.clone(),
            interval_secs: period,
        }];
        let mut query = historical_query(case, period, length, latest, now, Vec::new());
        warnings.append(&mut query.warnings);
        query.warnings = warnings;
        return Ok(EntityDecision::Query(query));
    }
    if realtime_ok {
        Ok(EntityDecision::Query(realtime_query(
            case,
            refresh.unwrap(),
            latest,
            now,
        )))
    } else if case.caps.summary_supported {
        Ok(EntityDecision::Unsupported(
            UnsupportedReason::HistoricalUnsupported,
        ))
    } else {
        Ok(EntityDecision::Unsupported(
            UnsupportedReason::RealtimeUnsupported,
        ))
    }
}

fn historical_query(
    case: &IntervalCase,
    period: i32,
    length: i32,
    latest: Option<i32>,
    now: SystemTime,
    mut warnings: Vec<PerfWarning>,
) -> PlannedQuery {
    let has_window = case.start.is_some() || case.end.is_some();
    if let Some(start) = case.start.filter(|start| !covers(*start, now, length)) {
        warnings.push(PerfWarning::HistoricalWindowTooOld {
            requested_start: Timestamp(start),
            interval_secs: period,
            retention_secs: length,
        });
    }
    let (start, end, truncate) = if has_window {
        let truncate = if case.incremental {
            None
        } else {
            latest.map(|n| n as usize)
        };
        (case.start, case.end.or(Some(now)), truncate)
    } else if let Some(n) = latest {
        let cushion = (n as u64)
            .saturating_add(2)
            .saturating_mul(period.max(0) as u64);
        (
            Some(sub_duration(now, Duration::from_secs(cushion))),
            Some(now),
            if case.incremental {
                None
            } else {
                Some(n as usize)
            },
        )
    } else {
        (
            Some(sub_duration(now, Duration::from_secs(length.max(0) as u64))),
            Some(now),
            None,
        )
    };
    PlannedQuery {
        interval_id: period,
        start,
        end,
        max_sample: None,
        truncate_newest: truncate,
        realtime: false,
        warnings,
    }
}

fn enabled_intervals(intervals: &[PerfInterval]) -> Vec<(i32, i32)> {
    intervals
        .iter()
        .filter(|interval| interval.enabled)
        .map(|interval| (interval.sampling_period, interval.length))
        .collect()
}

fn choose_period(enabled: &[(i32, i32)], start: Option<SystemTime>, now: SystemTime) -> (i32, i32) {
    let finest = enabled
        .iter()
        .min_by_key(|(period, _)| *period)
        .copied()
        .unwrap_or((300, 86400));
    let Some(start) = start else {
        return finest;
    };
    let covering: Vec<(i32, i32)> = enabled
        .iter()
        .copied()
        .filter(|(_, length)| covers(start, now, *length))
        .collect();
    if covering.is_empty() {
        enabled
            .iter()
            .max_by_key(|(period, _)| *period)
            .copied()
            .unwrap_or(finest)
    } else {
        covering
            .into_iter()
            .min_by_key(|(period, _)| *period)
            .unwrap_or(finest)
    }
}

fn covers(start: SystemTime, now: SystemTime, length: i32) -> bool {
    match now.duration_since(start) {
        Ok(age) => age.as_secs() <= length.max(0) as u64,
        Err(_) => true,
    }
}

fn older_than_hour(start: SystemTime, now: SystemTime) -> bool {
    match now.duration_since(start) {
        Ok(age) => age.as_secs() > 3600,
        Err(_) => false,
    }
}

struct EntityWindow {
    start: Option<SystemTime>,
    end: Option<SystemTime>,
    incremental: bool,
    /// `fetch_since` drops latest truncation and max_sample.
    open: bool,
}

fn window_for(
    spec: &QuerySpec,
    entity: &ManagedObjectReference,
    now: SystemTime,
    over: &WindowOverride,
) -> Result<EntityWindow, PerfError> {
    match over {
        WindowOverride::Since(start) => Ok(EntityWindow {
            start: Some(*start),
            end: Some(now),
            incremental: true,
            open: true,
        }),
        WindowOverride::Incremental(marks) => {
            if let Some(mark) = marks.get(entity) {
                Ok(EntityWindow {
                    start: Some(mark.0),
                    end: Some(now),
                    incremental: true,
                    open: true,
                })
            } else {
                let mut window = window_for(spec, entity, now, &WindowOverride::FromSpec)?;
                window.incremental = false;
                window.open = false;
                Ok(window)
            }
        }
        WindowOverride::FromSpec => {
            let (start, end) = match spec.time {
                TimeBound::None => (None, None),
                TimeBound::Since(start) => (Some(start), Some(now)),
                TimeBound::Between { start, end } => (Some(start), Some(end)),
                TimeBound::SinceAgo(ago) => (Some(sub_duration(now, ago)), Some(now)),
            };
            Ok(EntityWindow {
                start,
                end,
                incremental: false,
                open: false,
            })
        }
    }
}

/// Smallest realtime window a composite `latest(n)` asks for, in seconds.
const COMPOSITE_MIN_CUSHION_SECS: u64 = 300;

fn sub_duration(now: SystemTime, ago: Duration) -> SystemTime {
    now.checked_sub(ago).unwrap_or(SystemTime::UNIX_EPOCH)
}

pub(crate) async fn execute(
    client: &PerfClient,
    spec: &QuerySpec,
    entities: &[ManagedObjectReference],
    window: WindowOverride,
    cache: Option<&Mutex<Option<CachedResolve>>>,
) -> Result<PerfResult, PerfError> {
    if entities.is_empty() {
        return Err(PerfError::NoEntities);
    }
    let now = client.now();
    let registry = client.registry().await?;
    let planned = plan_group(
        client,
        &registry,
        spec,
        entities,
        window,
        cache,
        spec.concurrency,
        now,
    )
    .await?;
    if !planned.queried && !planned.probe_failed {
        return Err(PerfError::NoSupportedEntities);
    }
    let format = choose_format(spec.format, client.transport(), spec.latest);
    finish(
        client,
        &registry,
        planned,
        spec.chunk_size,
        spec.concurrency,
        format,
        now,
    )
    .await
}

async fn execute_batch(
    client: &PerfClient,
    groups: &[BatchGroup],
    format: WireFormat,
    chunk_size: usize,
    concurrency: usize,
) -> Result<PerfResult, PerfError> {
    if groups.is_empty() || groups.iter().any(|group| group.entities.is_empty()) {
        return Err(PerfError::NoEntities);
    }
    for group in groups {
        spec_has_selector(&group.spec)?;
    }
    let now = client.now();
    let registry = client.registry().await?;
    let mut planned = GroupPlan::default();
    let mut seen = HashSet::new();
    for group in groups {
        let group_plan = plan_group(
            client,
            &registry,
            &group.spec,
            &group.entities,
            WindowOverride::FromSpec,
            None,
            concurrency,
            now,
        )
        .await?;
        planned.queried |= group_plan.queried;
        planned.probe_failed |= group_plan.probe_failed;
        planned.warnings.extend(group_plan.warnings);
        for entity in group_plan.entities {
            if seen.insert(entity.clone()) {
                planned.entities.push(entity);
            }
        }
        for (entity, status) in group_plan.preset {
            planned.preset.entry(entity).or_insert(status);
        }
        planned.work.extend(group_plan.work);
    }
    reject_mixed_intervals(&planned.work)?;
    if !planned.queried && !planned.probe_failed {
        return Err(PerfError::NoSupportedEntities);
    }
    finish(
        client,
        &registry,
        planned,
        chunk_size,
        concurrency,
        choose_batch_format(format),
        now,
    )
    .await
}

fn spec_has_selector(spec: &QuerySpec) -> Result<(), PerfError> {
    let available_mode = spec.all_available
        || (spec.metrics.is_empty() && spec.names.is_empty() && !spec.groups.is_empty());
    if !available_mode && spec.metrics.is_empty() && spec.names.is_empty() {
        return Err(PerfError::NoMetrics);
    }
    Ok(())
}

struct GroupPlan {
    entities: Vec<ManagedObjectReference>,
    work: Vec<EntityWork>,
    preset: IndexMap<ManagedObjectReference, EntityStatus>,
    warnings: Vec<PerfWarning>,
    queried: bool,
    probe_failed: bool,
}

impl Default for GroupPlan {
    fn default() -> Self {
        Self {
            entities: Vec::new(),
            work: Vec::new(),
            preset: IndexMap::new(),
            warnings: Vec::new(),
            queried: false,
            probe_failed: false,
        }
    }
}

async fn plan_group(
    client: &PerfClient,
    registry: &Arc<CounterRegistry>,
    spec: &QuerySpec,
    entities: &[ManagedObjectReference],
    window: WindowOverride,
    cache: Option<&Mutex<Option<CachedResolve>>>,
    concurrency: usize,
    now: SystemTime,
) -> Result<GroupPlan, PerfError> {
    let entities = dedupe_entities(entities);
    let entities = entities.as_slice();
    spec_has_selector(spec)?;
    let available_mode = spec.all_available
        || (spec.metrics.is_empty() && spec.names.is_empty() && !spec.groups.is_empty());
    let (shared_resolved, shared_ids, cached_warnings) = if available_mode {
        (Arc::from([]), Arc::from([]), Arc::from([]))
    } else {
        resolve_explicit(spec, registry, cache)?
    };
    let mut warnings: Vec<PerfWarning> = cached_warnings.to_vec();
    note_ignored_selectors(spec, registry, &mut warnings);
    let probe = client.probe(entities).await?;
    let mut cases = Vec::new();
    for entity in entities {
        if probe.failed.contains_key(entity) {
            continue;
        }
        let caps = probe.caps.get(&entity.r#type).copied().unwrap_or(
            crate::perf::registry::ProviderCaps {
                current_supported: false,
                summary_supported: false,
                refresh_rate: None,
            },
        );
        let entity_window = window_for(spec, entity, now, &window)?;
        cases.push(IntervalCase {
            entity: entity.clone(),
            caps,
            start: entity_window.start,
            end: entity_window.end,
            incremental: entity_window.incremental || entity_window.open,
        });
    }
    let mut open_entities = HashSet::new();
    if matches!(window, WindowOverride::Since(_)) {
        open_entities.extend(entities.iter().cloned());
    }
    if let WindowOverride::Incremental(marks) = &window {
        open_entities.extend(marks.keys().cloned());
    }
    let planned = plan_entities(
        &cases,
        registry.intervals(),
        spec.interval,
        spec.latest,
        now,
    )?;
    let queried = planned
        .iter()
        .any(|item| matches!(item.decision, EntityDecision::Query(_)));
    for item in &planned {
        if let EntityDecision::Query(query) = &item.decision {
            warnings.extend(query.warnings.clone());
        }
    }

    let mut available = if available_mode {
        let jobs = planned
            .iter()
            .filter_map(|item| match &item.decision {
                EntityDecision::Query(query) => Some((item.entity.clone(), query.interval_id)),
                EntityDecision::Unsupported(_) => None,
            })
            .collect();
        fetch_available(client, jobs, concurrency).await?
    } else {
        HashMap::new()
    };
    let mut work = Vec::new();
    let mut preset_status: IndexMap<ManagedObjectReference, EntityStatus> = IndexMap::new();
    for entity in entities {
        if let Some(err) = probe.failed.get(entity) {
            preset_status.insert(entity.clone(), EntityStatus::Failed(Arc::clone(err)));
        }
    }
    let mut warned_levels = HashSet::new();
    for item in &planned {
        match &item.decision {
            EntityDecision::Unsupported(reason) => {
                preset_status.insert(item.entity.clone(), EntityStatus::Unsupported(*reason));
            }
            EntityDecision::Query(query) => {
                let (wire_ids, resolved) = if available_mode {
                    match available
                        .remove(&item.entity)
                        .expect("queryable entity has an availability result")
                    {
                        Err(err) => {
                            preset_status
                                .insert(item.entity.clone(), EntityStatus::Failed(Arc::new(err)));
                            continue;
                        }
                        Ok(None) => {
                            preset_status
                                .insert(item.entity.clone(), EntityStatus::NoMetricsAvailable);
                            continue;
                        }
                        Ok(Some(ids)) => {
                            let ids = filter_groups(registry, ids, &spec.groups);
                            if ids.is_empty() {
                                preset_status
                                    .insert(item.entity.clone(), EntityStatus::NoMetricsAvailable);
                                continue;
                            }
                            (Arc::<[PerfMetricId]>::from(ids), None)
                        }
                    }
                } else {
                    (Arc::clone(&shared_ids), Some(Arc::clone(&shared_resolved)))
                };
                if !available_mode && warned_levels.insert(query.interval_id) {
                    level_warnings(
                        &shared_resolved,
                        registry.intervals(),
                        query.interval_id,
                        &mut warnings,
                    );
                }
                let open = open_entities.contains(&item.entity);
                work.push(EntityWork {
                    entity: item.entity.clone(),
                    wire_ids,
                    resolved,
                    query: query.clone_bounds(open),
                    time_chunk: spec.time_chunk,
                });
            }
        }
    }
    dedup_warnings(&mut warnings);
    Ok(GroupPlan {
        entities: entities.to_vec(),
        work,
        preset: preset_status,
        warnings,
        queried,
        probe_failed: !probe.failed.is_empty(),
    })
}

async fn finish(
    client: &PerfClient,
    registry: &Arc<CounterRegistry>,
    mut planned: GroupPlan,
    chunk_size: usize,
    concurrency: usize,
    format: &str,
    now: SystemTime,
) -> Result<PerfResult, PerfError> {
    for item in &planned.work {
        planned.preset.shift_remove(&item.entity);
    }
    let mut warnings = planned.warnings;
    if planned.work.is_empty() {
        dedup_warnings(&mut warnings);
        return Ok(assemble(
            &planned.entities,
            planned.preset,
            IndexMap::new(),
            registry,
            now,
            warnings,
        ));
    }
    let chunks = pack_work(&planned.work, chunk_size, format);
    let fetched = run_chunks(client, chunks, concurrency).await?;
    let mut series_by_entity = IndexMap::new();
    let mut payloads = fetched.payloads;
    // One object can have work from several batch groups. Group it once.
    let mut work_by_entity: IndexMap<&ManagedObjectReference, Vec<&EntityWork>> = IndexMap::new();
    for item in &planned.work {
        work_by_entity.entry(&item.entity).or_default().push(item);
    }
    for (entity, items) in work_by_entity {
        if !fetched.succeeded.contains(entity) {
            continue;
        }
        let raw = payloads.shift_remove(entity).unwrap_or_default();
        if raw.is_empty() {
            series_by_entity.insert(entity.clone(), None);
            continue;
        }
        let decoded = decode_metrics(
            raw,
            registry,
            std::slice::from_ref(entity),
            Some(items[0].query.interval_id),
            now,
        );
        let mut decoded = match decoded {
            Ok(decoded) => decoded,
            Err(err) if is_fatal_perf(&err) => return Err(err),
            Err(err) => {
                planned
                    .preset
                    .insert(entity.clone(), EntityStatus::Failed(Arc::new(err)));
                continue;
            }
        };
        warnings.extend(std::mem::take(&mut decoded.warnings));
        let mut series = decoded.into_entities().next();
        if let Some(series) = series.as_mut() {
            if let Some(n) = agreed_truncate(&items) {
                truncate_newest(series, n);
            }
            for item in &items {
                if let Some(metrics) = &item.resolved {
                    scope_warnings(series, metrics, &mut warnings);
                }
            }
        }
        series_by_entity.insert(entity.clone(), series);
    }
    for (entity, err) in fetched.failed {
        if fetched.succeeded.contains(&entity) {
            warnings.push(PerfWarning::PartialChunk {
                entity,
                message: err.to_string(),
            });
        } else {
            planned.preset.insert(entity, EntityStatus::Failed(err));
        }
    }
    dedup_warnings(&mut warnings);
    Ok(assemble(
        &planned.entities,
        planned.preset,
        series_by_entity,
        registry,
        now,
        warnings,
    ))
}

/// One object in several batch groups must resolve to one period. Its samples share
/// one grid and one `interval_secs`, so a second period cannot be represented.
fn reject_mixed_intervals(work: &[EntityWork]) -> Result<(), PerfError> {
    let mut period: HashMap<&ManagedObjectReference, i32> = HashMap::new();
    for item in work {
        let first = *period.entry(&item.entity).or_insert(item.query.interval_id);
        if first != item.query.interval_id {
            return Err(PerfError::MixedIntervals {
                entity: item.entity.clone(),
                first_secs: first,
                second_secs: item.query.interval_id,
            });
        }
    }
    Ok(())
}

/// `latest` applies to a shared object only when every group agrees.
fn agreed_truncate(items: &[&EntityWork]) -> Option<usize> {
    let mut limit = None;
    for item in items {
        match item.query.truncate_newest {
            Some(n) if limit.is_none() || limit == Some(n) => limit = Some(n),
            _ => return None,
        }
    }
    limit
}

async fn fetch_available(
    client: &PerfClient,
    jobs: Vec<(ManagedObjectReference, i32)>,
    concurrency: usize,
) -> Result<HashMap<ManagedObjectReference, Result<Option<Vec<PerfMetricId>>, PerfError>>, PerfError>
{
    let mut out = HashMap::with_capacity(jobs.len());
    if jobs.is_empty() {
        return Ok(out);
    }
    let sem = Arc::new(Semaphore::new(concurrency.max(1)));
    let mut set = JoinSet::new();
    for (entity, interval_id) in jobs {
        let client = client.clone();
        let sem = Arc::clone(&sem);
        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore open");
            let result = client.available_metrics(&entity, interval_id).await;
            (entity, result)
        });
    }
    while let Some(joined) = set.join_next().await {
        let (entity, result) = joined.map_err(|err| {
            PerfError::Client(client::Error::ParseError(format!(
                "availability query task failed: {err}"
            )))
        })?;
        match result {
            Err(err) if is_fatal_perf(&err) => {
                set.abort_all();
                return Err(err);
            }
            other => {
                out.insert(entity, other);
            }
        }
    }
    Ok(out)
}

fn is_fatal_perf(err: &PerfError) -> bool {
    match err {
        PerfError::Client(inner) => session::is_fatal(inner),
        _ => false,
    }
}

fn dedupe_entities(entities: &[ManagedObjectReference]) -> Vec<ManagedObjectReference> {
    let mut seen = HashSet::with_capacity(entities.len());
    let mut unique = Vec::with_capacity(entities.len());
    for entity in entities {
        if seen.insert(entity.clone()) {
            unique.push(entity.clone());
        }
    }
    unique
}

fn metric_ids(metrics: &[ResolvedMetric]) -> Arc<[PerfMetricId]> {
    Arc::from(owned_metric_ids(metrics))
}

fn owned_metric_ids(metrics: &[ResolvedMetric]) -> Vec<PerfMetricId> {
    metrics
        .iter()
        .map(|metric| PerfMetricId {
            counter_id: metric.def.id.0,
            instance: metric.instance.as_wire_str().to_string(),
        })
        .collect()
}

struct EntityWork {
    entity: ManagedObjectReference,
    wire_ids: Arc<[PerfMetricId]>,
    resolved: Option<Arc<[ResolvedMetric]>>,
    query: PlannedQuery,
    time_chunk: Option<Duration>,
}

impl PlannedQuery {
    fn clone_bounds(&self, open: bool) -> Self {
        Self {
            interval_id: self.interval_id,
            start: self.start,
            end: self.end,
            max_sample: if open { None } else { self.max_sample },
            truncate_newest: if open { None } else { self.truncate_newest },
            realtime: self.realtime,
            warnings: Vec::new(),
        }
    }
}

fn filter_groups(
    registry: &CounterRegistry,
    ids: Vec<PerfMetricId>,
    groups: &[String],
) -> Vec<PerfMetricId> {
    if groups.is_empty() {
        return ids;
    }
    ids.into_iter()
        .filter(|id| {
            registry
                .by_id(crate::perf::registry::CounterId(id.counter_id))
                .map(|def| groups.iter().any(|group| def.name.group.as_ref() == group))
                .unwrap_or(false)
        })
        .collect()
}

fn resolve_explicit(
    spec: &QuerySpec,
    registry: &Arc<CounterRegistry>,
    cache: Option<&Mutex<Option<CachedResolve>>>,
) -> Result<
    (
        Arc<[ResolvedMetric]>,
        Arc<[PerfMetricId]>,
        Arc<[PerfWarning]>,
    ),
    PerfError,
> {
    if let Some(cache) = cache {
        let guard = cache.lock().unwrap_or_else(|err| err.into_inner());
        if let Some(hit) = guard.as_ref() {
            if hit.generation == registry.generation() {
                return Ok((
                    Arc::clone(&hit.metrics),
                    Arc::clone(&hit.ids),
                    Arc::clone(&hit.warnings),
                ));
            }
        }
    }
    let (metrics, warnings) = resolve_lists(spec, registry)?;
    let metrics: Arc<[ResolvedMetric]> = Arc::from(metrics);
    let ids = metric_ids(&metrics);
    let warnings: Arc<[PerfWarning]> = Arc::from(warnings);
    if let Some(cache) = cache {
        *cache.lock().unwrap_or_else(|err| err.into_inner()) = Some(CachedResolve {
            generation: registry.generation(),
            metrics: Arc::clone(&metrics),
            ids: Arc::clone(&ids),
            warnings: Arc::clone(&warnings),
        });
    }
    Ok((metrics, ids, warnings))
}

/// `.all_available()` replaces named counters. `.groups()` applies only in that mode.
fn note_ignored_selectors(
    spec: &QuerySpec,
    registry: &CounterRegistry,
    warnings: &mut Vec<PerfWarning>,
) {
    let named = !spec.metrics.is_empty() || !spec.names.is_empty();
    if spec.all_available && named {
        warnings.push(PerfWarning::ExplicitMetricsIgnored);
        for erased in &spec.metrics {
            if registry
                .lookup_parts(erased.group, erased.name, erased.rollup, erased.stats)
                .is_none()
            {
                warnings.push(PerfWarning::CounterNotOnServer {
                    counter: format!(
                        "{}.{}.{}",
                        erased.group,
                        erased.name,
                        erased.rollup.as_str()
                    ),
                });
            }
        }
        for name in &spec.names {
            if registry.by_name(name).is_none() {
                warnings.push(PerfWarning::CounterNotOnServer {
                    counter: name.clone(),
                });
            }
        }
    }
    if !spec.groups.is_empty() && !spec.all_available && named {
        warnings.push(PerfWarning::GroupsIgnored);
    }
}

fn resolve_lists(
    spec: &QuerySpec,
    registry: &CounterRegistry,
) -> Result<(Vec<ResolvedMetric>, Vec<PerfWarning>), PerfError> {
    let mut metrics = Vec::new();
    let mut warnings = Vec::new();
    let mut unknown = Vec::new();
    let mut dimension = Vec::new();
    for erased in &spec.metrics {
        let found = registry
            .lookup_parts(erased.group, erased.name, erased.rollup, erased.stats)
            .cloned();
        match found {
            None => {
                let full = format!(
                    "{}.{}.{}",
                    erased.group,
                    erased.name,
                    erased.rollup.as_str()
                );
                unknown.push(full.clone());
                warnings.push(PerfWarning::CounterNotOnServer { counter: full });
            }
            Some(def) => push_resolved(
                erased,
                def,
                spec,
                &mut metrics,
                &mut warnings,
                &mut dimension,
            ),
        }
    }
    for name in &spec.names {
        match registry.by_name(name).cloned() {
            None => {
                unknown.push(name.clone());
                warnings.push(PerfWarning::CounterNotOnServer {
                    counter: name.clone(),
                });
            }
            Some(def) => {
                if !unit_is_scalable(&def.unit.0) {
                    warnings.push(PerfWarning::UnscalableUnit {
                        counter: def.name.full.to_string(),
                        unit: def.unit.as_str().to_string(),
                    });
                }
                metrics.push(ResolvedMetric {
                    def,
                    instance: spec.default_instance.clone(),
                    scope: MASK_ALL,
                });
            }
        }
    }
    if metrics.is_empty() {
        if let Some(counter) = unknown.first() {
            return Err(PerfError::UnknownCounter(counter.clone()));
        }
        if let Some((counter, unit)) = dimension.first() {
            return Err(PerfError::DimensionMismatch {
                counter: counter.clone(),
                unit: unit.clone(),
            });
        }
        return Err(PerfError::NoMetrics);
    }
    Ok((metrics, warnings))
}

fn push_resolved(
    erased: &ErasedMetric,
    def: Arc<CounterDef>,
    spec: &QuerySpec,
    metrics: &mut Vec<ResolvedMetric>,
    warnings: &mut Vec<PerfWarning>,
    dimension: &mut Vec<(String, String)>,
) {
    let full = def.name.full.to_string();
    if !(erased.accepts)(&def.unit.0) {
        warnings.push(PerfWarning::DimensionMismatch {
            counter: full.clone(),
            unit: def.unit.as_str().to_string(),
        });
        dimension.push((full, def.unit.as_str().to_string()));
        return;
    }
    if erased.unconstrained && !unit_is_scalable(&def.unit.0) {
        warnings.push(PerfWarning::UnscalableUnit {
            counter: full,
            unit: def.unit.as_str().to_string(),
        });
    }
    metrics.push(ResolvedMetric {
        def,
        instance: erased
            .instance
            .clone()
            .unwrap_or_else(|| spec.default_instance.clone()),
        scope: erased.scope,
    });
}

fn level_warnings(
    metrics: &[ResolvedMetric],
    intervals: &[PerfInterval],
    period: i32,
    warnings: &mut Vec<PerfWarning>,
) {
    let Some(level) = intervals
        .iter()
        .find(|interval| interval.enabled && interval.sampling_period == period)
        .and_then(|interval| interval.level)
    else {
        return;
    };
    for metric in metrics {
        let counter = metric.def.name.full.to_string();
        if let Some(required) = metric.def.level {
            if required > level {
                warnings.push(PerfWarning::LevelTooLow {
                    counter: counter.clone(),
                    required_level: required,
                    interval_level: level,
                });
            }
        }
        if !metric.instance.is_aggregate() {
            if let Some(required) = metric.def.per_device_level {
                if required > level {
                    warnings.push(PerfWarning::PerDeviceLevelTooLow {
                        counter,
                        required_level: required,
                        interval_level: level,
                    });
                }
            }
        }
    }
}

fn scope_warnings(
    series: &EntitySeries,
    metrics: &[ResolvedMetric],
    warnings: &mut Vec<PerfWarning>,
) {
    let bit = mo_type_bit(&series.entity.r#type);
    for metric in metrics {
        if metric.scope == MASK_ALL || metric.scope & bit != 0 {
            continue;
        }
        let present = series
            .series
            .iter()
            .any(|raw| raw.counter.id == metric.def.id);
        if !present {
            warnings.push(PerfWarning::ScopeMismatch {
                entity: series.entity.clone(),
                counter: metric.def.name.full.to_string(),
            });
        }
    }
}

/// [`WireFormat::Auto`] is CSV. Groups in one batch may disagree on `latest(1)`,
/// which is the only case where [`choose_format`] would send normal JSON.
fn choose_batch_format(format: WireFormat) -> &'static str {
    match format {
        WireFormat::Normal => PerfFormatEnum::Normal.as_str(),
        WireFormat::Csv | WireFormat::Auto => PerfFormatEnum::Csv.as_str(),
    }
}

fn choose_format(format: WireFormat, transport: Transport, latest: Option<i32>) -> &'static str {
    match format {
        WireFormat::Normal => PerfFormatEnum::Normal.as_str(),
        WireFormat::Csv => PerfFormatEnum::Csv.as_str(),
        WireFormat::Auto => {
            #[cfg(feature = "xml")]
            if transport == Transport::Soap {
                return PerfFormatEnum::Csv.as_str();
            }
            let _ = transport;
            if latest == Some(1) {
                PerfFormatEnum::Normal.as_str()
            } else {
                PerfFormatEnum::Csv.as_str()
            }
        }
    }
}

struct Chunk {
    specs: Vec<PerfQuerySpec>,
    members: Vec<ManagedObjectReference>,
}

fn pack_work(work: &[EntityWork], chunk_size: usize, format: &str) -> Vec<Chunk> {
    let mut slices = Vec::new();
    for item in work {
        for (start, end) in slices_for(&item.query, item.time_chunk) {
            slices.push(SliceWork {
                entity: item.entity.clone(),
                wire_ids: Arc::clone(&item.wire_ids),
                query: PlannedQuery {
                    start,
                    end,
                    ..item.query.clone_bounds(false)
                },
            });
        }
    }
    pack_slices(slices, chunk_size, format)
}

struct SliceWork {
    entity: ManagedObjectReference,
    wire_ids: Arc<[PerfMetricId]>,
    query: PlannedQuery,
}

fn slices_for(
    query: &PlannedQuery,
    time_chunk: Option<Duration>,
) -> Vec<(Option<SystemTime>, Option<SystemTime>)> {
    let Some(window) = time_chunk else {
        return vec![(query.start, query.end)];
    };
    if query.realtime || window.is_zero() {
        return vec![(query.start, query.end)];
    }
    let (Some(start), Some(end)) = (query.start, query.end) else {
        return vec![(query.start, query.end)];
    };
    let mut slices = Vec::new();
    let mut cursor = start;
    loop {
        let next = match cursor.checked_add(window) {
            Some(next) if next < end => next,
            _ => end,
        };
        slices.push((Some(cursor), Some(next)));
        if next >= end {
            break;
        }
        cursor = next;
    }
    slices
}

fn pack_slices(entities: Vec<SliceWork>, chunk_size: usize, format: &str) -> Vec<Chunk> {
    let chunk_size = chunk_size.max(1);
    let mut chunks = Vec::new();
    let mut open: Vec<PerfQuerySpec> = Vec::new();
    let mut open_members = Vec::new();
    let mut open_n = 0usize;
    let flush = |open: &mut Vec<PerfQuerySpec>,
                 members: &mut Vec<ManagedObjectReference>,
                 open_n: &mut usize,
                 chunks: &mut Vec<Chunk>| {
        if !open.is_empty() {
            chunks.push(Chunk {
                specs: std::mem::take(open),
                members: std::mem::take(members),
            });
            *open_n = 0;
        }
    };
    for entity in entities {
        let ids = entity.wire_ids.as_ref();
        if ids.is_empty() {
            continue;
        }
        if ids.len() > chunk_size {
            flush(&mut open, &mut open_members, &mut open_n, &mut chunks);
            let mut offset = 0;
            while ids.len() - offset > chunk_size {
                let end = offset + chunk_size;
                chunks.push(Chunk {
                    specs: vec![spec_for(&entity, &ids[offset..end], format)],
                    members: vec![entity.entity.clone()],
                });
                offset = end;
            }
            if offset < ids.len() {
                let rest = &ids[offset..];
                open_n = rest.len();
                open_members.push(entity.entity.clone());
                open.push(spec_for(&entity, rest, format));
            }
        } else if open_n + ids.len() <= chunk_size {
            open_n += ids.len();
            open_members.push(entity.entity.clone());
            open.push(spec_for(&entity, ids, format));
        } else {
            flush(&mut open, &mut open_members, &mut open_n, &mut chunks);
            open_n = ids.len();
            open_members.push(entity.entity.clone());
            open.push(spec_for(&entity, ids, format));
        }
    }
    flush(&mut open, &mut open_members, &mut open_n, &mut chunks);
    chunks
}

fn spec_for(work: &SliceWork, ids: &[PerfMetricId], format: &str) -> PerfQuerySpec {
    PerfQuerySpec {
        entity: work.entity.clone(),
        start_time: work.query.start.map(|ts| Timestamp(ts).to_rfc3339_utc()),
        end_time: work.query.end.map(|ts| Timestamp(ts).to_rfc3339_utc()),
        max_sample: work.query.max_sample,
        metric_id: Some(ids.iter().map(session::clone_metric_id).collect()),
        interval_id: Some(work.query.interval_id),
        format: Some(format.to_string()),
    }
}

struct FetchGather {
    succeeded: HashSet<ManagedObjectReference>,
    payloads: IndexMap<ManagedObjectReference, Vec<Box<dyn PerfEntityMetricBaseTrait>>>,
    failed: IndexMap<ManagedObjectReference, Arc<PerfError>>,
}

async fn run_chunks(
    client: &PerfClient,
    chunks: Vec<Chunk>,
    concurrency: usize,
) -> Result<FetchGather, PerfError> {
    if chunks.is_empty() {
        return Ok(FetchGather {
            succeeded: HashSet::new(),
            payloads: IndexMap::new(),
            failed: IndexMap::new(),
        });
    }
    let sem = Arc::new(Semaphore::new(concurrency.max(1)));
    let mut set = JoinSet::new();
    for (index, chunk) in chunks.into_iter().enumerate() {
        let manager = client.manager().clone();
        let sem = Arc::clone(&sem);
        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore open");
            let result = manager.query_perf(&chunk.specs).await;
            (index, chunk.members, result)
        });
    }
    let mut pending = Vec::new();
    while let Some(joined) = set.join_next().await {
        let (index, chunk_members, result) = joined.map_err(|err| {
            PerfError::Client(client::Error::ParseError(format!(
                "performance query task failed: {err}"
            )))
        })?;
        if let Err(err) = &result {
            if session::is_fatal(err) {
                set.abort_all();
                let Err(err) = result else {
                    unreachable!("fatal error")
                };
                return Err(err.into());
            }
        }
        pending.push((index, chunk_members, result));
    }
    pending.sort_by_key(|(index, _, _)| *index);
    let mut gather = FetchGather {
        succeeded: HashSet::new(),
        payloads: IndexMap::new(),
        failed: IndexMap::new(),
    };
    let mut chunk_errors: Vec<(usize, Arc<PerfError>)> = Vec::new();
    for (index, chunk_members, result) in pending {
        // A payload of an unknown subtype has no object to file it under. It fails
        // the chunk as a parse error.
        let result = result.and_then(|payload| {
            payload
                .unwrap_or_default()
                .into_iter()
                .map(|sample| match decode::entity_of(sample.as_ref()) {
                    Some(entity) => Ok((entity.clone(), sample)),
                    None => Err(client::Error::ParseError(decode::UNKNOWN_PAYLOAD.into())),
                })
                .collect::<Result<Vec<_>, _>>()
        });
        match result {
            Ok(samples) => {
                for member in &chunk_members {
                    gather.succeeded.insert(member.clone());
                }
                for (entity, sample) in samples {
                    gather.payloads.entry(entity).or_default().push(sample);
                }
            }
            Err(err) => {
                let err = Arc::new(PerfError::from(err));
                chunk_errors.push((index, Arc::clone(&err)));
                for member in &chunk_members {
                    gather
                        .failed
                        .entry(member.clone())
                        .or_insert_with(|| Arc::clone(&err));
                }
            }
        }
    }
    if gather.succeeded.is_empty() && !chunk_errors.is_empty() {
        chunk_errors.sort_by_key(|(index, _)| *index);
        return Err(Arc::unwrap_or_clone(chunk_errors.remove(0).1));
    }
    Ok(gather)
}

fn assemble(
    requested: &[ManagedObjectReference],
    preset: IndexMap<ManagedObjectReference, EntityStatus>,
    mut series: IndexMap<ManagedObjectReference, Option<EntitySeries>>,
    registry: &Arc<CounterRegistry>,
    now: SystemTime,
    warnings: Vec<PerfWarning>,
) -> PerfResult {
    let mut entities = IndexMap::new();
    for entity in requested {
        match series.shift_remove(entity) {
            Some(Some(decoded)) => {
                entities.insert(entity.clone(), decoded);
            }
            Some(None) | None => {
                let status = preset
                    .get(entity)
                    .cloned()
                    .unwrap_or(EntityStatus::NoSamples);
                entities.insert(
                    entity.clone(),
                    EntitySeries::new(
                        entity.clone(),
                        status,
                        Arc::from([]),
                        0,
                        Vec::new(),
                        Arc::clone(registry),
                    ),
                );
            }
        }
    }
    PerfResult::new(entities, Arc::clone(registry), now, warnings)
}

fn truncate_newest(series: &mut EntitySeries, n: usize) {
    if n == 0 || series.timestamps.len() <= n {
        return;
    }
    let drop_n = series.timestamps.len() - n;
    let timestamps: Arc<[Timestamp]> = series.timestamps[drop_n..].iter().copied().collect();
    for raw in &mut series.series {
        raw.values.drain(0..drop_n);
        raw.timestamps = Arc::clone(&timestamps);
    }
    series.timestamps = timestamps;
}

fn dedup_warnings(warnings: &mut Vec<PerfWarning>) {
    let mut seen = HashSet::with_capacity(warnings.len());
    let mut kept = Vec::with_capacity(warnings.len());
    for warning in warnings.drain(..) {
        if seen.insert(warning.clone()) {
            kept.push(warning);
        }
    }
    *warnings = kept;
}

async fn execute_composite(
    client: &PerfClient,
    spec: &QuerySpec,
    host: &ManagedObjectReference,
) -> Result<crate::perf::result::CompositePerfResult, PerfError> {
    if spec.metrics.is_empty() && spec.names.is_empty() {
        return Err(PerfError::NoMetrics);
    }
    let now = client.now();
    let registry = client.registry().await?;
    let (metrics, _ids, cached_warnings) = resolve_explicit(spec, &registry, None)?;
    let mut warnings: Vec<PerfWarning> = cached_warnings.to_vec();
    let mut probe = client.probe(std::slice::from_ref(host)).await?;
    if let Some(err) = probe.failed.shift_remove(host) {
        return Err(Arc::unwrap_or_clone(err));
    }
    let caps = probe
        .caps
        .get(&host.r#type)
        .copied()
        .ok_or(PerfError::NoSupportedEntities)?;
    let entity_window = window_for(spec, host, now, &WindowOverride::FromSpec)?;
    let planned = plan_intervals(
        &[IntervalCase {
            entity: host.clone(),
            caps,
            start: entity_window.start,
            end: entity_window.end,
            incremental: false,
        }],
        registry.intervals(),
        spec.interval,
        spec.latest,
        now,
    )?;
    let EntityDecision::Query(query) = &planned[0].decision else {
        return Err(PerfError::NoSupportedEntities);
    };
    warnings.extend(query.warnings.clone());
    level_warnings(
        &metrics,
        registry.intervals(),
        query.interval_id,
        &mut warnings,
    );
    // `max_sample` is invalid on QueryPerfComposite. Realtime latest-N without a window
    // bounds the start instead, so the server does not return the whole retained hour.
    // The floor keeps a few minutes of slack for skew between the local and server clocks.
    let start = match (query.start, spec.latest.filter(|n| *n > 0)) {
        (None, Some(n)) if query.realtime => {
            let cushion = (n as u64)
                .saturating_add(2)
                .saturating_mul(query.interval_id.max(0) as u64)
                .max(COMPOSITE_MIN_CUSHION_SECS);
            Some(sub_duration(now, Duration::from_secs(cushion)))
        }
        (start, _) => start,
    };
    let spec_wire = PerfQuerySpec {
        entity: host.clone(),
        start_time: start.map(|ts| Timestamp(ts).to_rfc3339_utc()),
        end_time: query.end.map(|ts| Timestamp(ts).to_rfc3339_utc()),
        max_sample: None,
        metric_id: Some(owned_metric_ids(&metrics)),
        interval_id: Some(query.interval_id),
        format: Some(PerfFormatEnum::Normal.as_str().to_string()),
    };
    let composite = client.manager().query_perf_composite(&spec_wire).await?;
    let mut decoded = decode::decode_composite(
        composite.entity,
        composite.child_entity.unwrap_or_default(),
        &registry,
        Some(query.interval_id),
        now,
    )?;
    warnings.extend(std::mem::take(&mut decoded.warnings));
    if let Some(n) = spec.latest.filter(|n| *n > 0) {
        let n = n as usize;
        if let Some(host) = decoded.host.as_mut() {
            truncate_newest(host, n);
        }
        for child in decoded.children.values_mut() {
            truncate_newest(child, n);
        }
    }
    dedup_warnings(&mut warnings);
    decoded.warnings = warnings;
    Ok(decoded)
}
