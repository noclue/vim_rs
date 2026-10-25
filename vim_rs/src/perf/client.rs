//! `PerfClient`: one `PerformanceManager`, shared catalogs and provider caps.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use indexmap::IndexMap;
use tokio::sync::watch;
use tokio::task::JoinSet;

use crate::core::client::{self, is_request_canceled_error, VimClientHandle};
use crate::mo::PerformanceManager;
use crate::perf::error::PerfError;
use crate::perf::registry::{CounterRegistry, ProviderCaps};
use crate::perf::result::PerfResult;
use crate::types::struct_enum::StructType;
use crate::types::structs::{ManagedObjectReference, PerfMetricId};
use crate::types::traits::PerfEntityMetricBaseTrait;

pub(crate) struct ProbeOutcome {
    pub caps: HashMap<MoTypesEnum, ProviderCaps>,
    /// Entities whose kind never produced a summary. Not cached.
    pub failed: IndexMap<ManagedObjectReference, Arc<PerfError>>,
}

use crate::types::enums::MoTypesEnum;

/// Session-scoped entry point for `PerformanceManager`.
#[derive(Clone)]
pub struct PerfClient {
    inner: Arc<Inner>,
}

struct Inner {
    client: VimClientHandle,
    manager: PerformanceManager,
    registry: tokio::sync::Mutex<Option<Arc<CounterRegistry>>>,
    caps: Mutex<HashMap<MoTypesEnum, ProviderCaps>>,
    /// In-flight provider probes, one leader per managed-object kind.
    inflight: Mutex<HashMap<MoTypesEnum, watch::Sender<ProbeNotice>>>,
    availability: Mutex<IndexMap<(ManagedObjectReference, i32), Vec<PerfMetricId>>>,
    clock: Mutex<Option<SystemTime>>,
}

impl PerfClient {
    /// Bind the connection’s `ServiceContent.perf_manager`. Does not load the catalog.
    pub fn new(client: VimClientHandle) -> Result<Self, PerfError> {
        let moref = client
            .service_content()
            .perf_manager
            .clone()
            .ok_or(PerfError::NoPerformanceManager)?;
        Ok(Self {
            inner: Arc::new(Inner {
                manager: PerformanceManager::new(Arc::clone(&client), &moref.value),
                client,
                registry: tokio::sync::Mutex::new(None),
                caps: Mutex::new(HashMap::new()),
                inflight: Mutex::new(HashMap::new()),
                availability: Mutex::new(IndexMap::new()),
                clock: Mutex::new(None),
            }),
        })
    }

    /// One-shot query builder.
    pub fn query(&self) -> super::query::PerfQuery {
        super::query::PerfQuery::new(self.clone())
    }

    /// Reusable query. [`super::query::PreparedQueryBuilder::build`] does not call the server.
    pub fn prepare(&self) -> super::query::PreparedQueryBuilder {
        super::query::PreparedQueryBuilder::new(self.clone())
    }

    /// Host-and-children query. The host type is checked when the query is fetched.
    pub fn composite(&self, host: &ManagedObjectReference) -> super::query::CompositeQuery {
        super::query::CompositeQuery::new(self.clone(), host.clone())
    }

    /// Load `perf_counter` and `historical_interval` once.
    pub async fn registry(&self) -> Result<Arc<CounterRegistry>, PerfError> {
        let mut guard = self.inner.registry.lock().await;
        if let Some(registry) = guard.as_ref() {
            return Ok(Arc::clone(registry));
        }
        let registry = Arc::new(self.load_registry().await?);
        *guard = Some(Arc::clone(&registry));
        Ok(registry)
    }

    /// Replace the cached catalog. The new registry has a new generation.
    pub async fn refresh_registry(&self) -> Result<Arc<CounterRegistry>, PerfError> {
        let mut guard = self.inner.registry.lock().await;
        let registry = Arc::new(self.load_registry().await?);
        *guard = Some(Arc::clone(&registry));
        Ok(registry)
    }

    async fn load_registry(&self) -> Result<CounterRegistry, PerfError> {
        let (counters, intervals) = tokio::try_join!(
            self.inner.manager.perf_counter(),
            self.inner.manager.historical_interval(),
        )?;
        Ok(CounterRegistry::from_wire(
            counters.unwrap_or_default(),
            intervals.unwrap_or_default(),
        ))
    }

    /// Refresh rate for `entity`. Errors when realtime is unsupported or the rate is missing.
    pub async fn realtime_refresh_rate(
        &self,
        entity: &ManagedObjectReference,
    ) -> Result<Duration, PerfError> {
        let caps = self.probe_one(entity).await?;
        match caps.refresh_rate {
            Some(rate) if caps.current_supported && rate > 0 => {
                Ok(Duration::from_secs(rate as u64))
            }
            _ => Err(PerfError::NoSupportedEntities),
        }
    }

    /// Generated stub, for calls this module does not wrap.
    pub fn raw(&self) -> &PerformanceManager {
        &self.inner.manager
    }

    /// Decode a `QueryPerf` payload with the cached registry. Omitted entities are not invented.
    pub async fn decode(
        &self,
        raw: Vec<Box<dyn PerfEntityMetricBaseTrait>>,
    ) -> Result<PerfResult, PerfError> {
        let registry = self.registry().await?;
        super::decode::decode_metrics(raw, &registry, &[], None, self.now())
    }

    /// Pin `since_ago` and the interval table to this instant. Tests share the value.
    #[doc(hidden)]
    pub fn set_clock(&self, now: SystemTime) {
        *self.clock_lock() = Some(now);
    }

    pub(crate) fn now(&self) -> SystemTime {
        self.clock_lock().unwrap_or_else(SystemTime::now)
    }

    pub(crate) fn transport(&self) -> client::Transport {
        self.inner.client.transport()
    }

    pub(crate) fn manager(&self) -> &PerformanceManager {
        &self.inner.manager
    }

    pub(crate) async fn probe(
        &self,
        entities: &[ManagedObjectReference],
    ) -> Result<ProbeOutcome, PerfError> {
        let mut order: Vec<MoTypesEnum> = Vec::new();
        for entity in entities {
            if !order.contains(&entity.r#type) {
                order.push(entity.r#type.clone());
            }
        }
        let mut resolved: HashMap<MoTypesEnum, KindProbe> = HashMap::new();
        let mut set = JoinSet::new();
        for kind in &order {
            if let Some(caps) = self.caps_lock().get(kind).copied() {
                resolved.insert(kind.clone(), KindProbe::Ready(caps));
                continue;
            }
            let of_kind: Vec<_> = entities
                .iter()
                .filter(|entity| &entity.r#type == kind)
                .cloned()
                .collect();
            let client = self.clone();
            let kind = kind.clone();
            set.spawn(async move {
                let outcome = client.probe_kind(kind.clone(), of_kind).await;
                (kind, outcome)
            });
        }
        while let Some(joined) = set.join_next().await {
            let (kind, outcome) = joined.map_err(|err| {
                PerfError::Client(client::Error::ParseError(format!(
                    "provider summary task failed: {err}"
                )))
            })?;
            match outcome {
                Ok(probe) => {
                    resolved.insert(kind, probe);
                }
                Err(err) => {
                    set.abort_all();
                    return Err(err);
                }
            }
        }
        let mut failed = IndexMap::new();
        for kind in &order {
            if let Some(KindProbe::Failed(errors)) = resolved.remove(kind) {
                for (entity, err) in errors {
                    failed.insert(entity, err);
                }
            }
        }
        if !entities.is_empty() && failed.len() == entities.len() {
            let (_, err) = failed.swap_remove_index(0).expect("failure");
            return Err(Arc::unwrap_or_clone(err));
        }
        let caps = self.caps_lock().clone();
        Ok(ProbeOutcome { caps, failed })
    }

    /// One kind. A second caller for the same kind waits for the first probe.
    async fn probe_kind(
        &self,
        kind: MoTypesEnum,
        entities: Vec<ManagedObjectReference>,
    ) -> Result<KindProbe, PerfError> {
        loop {
            if let Some(caps) = self.caps_lock().get(&kind).copied() {
                return Ok(KindProbe::Ready(caps));
            }
            let role = {
                let mut inflight = self.inflight_lock();
                if let Some(caps) = self.caps_lock().get(&kind).copied() {
                    return Ok(KindProbe::Ready(caps));
                }
                if let Some(tx) = inflight.get(&kind) {
                    ProbeRole::Wait(tx.subscribe())
                } else {
                    let (tx, _rx) = watch::channel(ProbeNotice::Pending);
                    inflight.insert(kind.clone(), tx);
                    ProbeRole::Lead
                }
            };
            match role {
                ProbeRole::Wait(mut rx) => loop {
                    let notice = rx.borrow().clone();
                    match notice {
                        ProbeNotice::Pending => {
                            if rx.changed().await.is_err() {
                                break;
                            }
                        }
                        ProbeNotice::Ready(caps) => return Ok(KindProbe::Ready(caps)),
                        ProbeNotice::Fatal | ProbeNotice::Miss => break,
                    }
                },
                ProbeRole::Lead => {
                    let mut guard = InflightGuard {
                        client: self.clone(),
                        kind: kind.clone(),
                        done: false,
                    };
                    let outcome = scan_provider(self, &entities).await;
                    match &outcome {
                        Ok(KindProbe::Ready(caps)) => {
                            self.caps_lock().insert(kind.clone(), *caps);
                            self.finish_probe(&kind, ProbeNotice::Ready(*caps), &mut guard);
                        }
                        Ok(KindProbe::Failed(_)) => {
                            self.finish_probe(&kind, ProbeNotice::Miss, &mut guard);
                        }
                        Err(_) => {
                            self.finish_probe(&kind, ProbeNotice::Fatal, &mut guard);
                        }
                    }
                    return outcome;
                }
            }
        }
    }

    fn finish_probe(&self, kind: &MoTypesEnum, notice: ProbeNotice, guard: &mut InflightGuard) {
        let mut inflight = self.inflight_lock();
        if let Some(tx) = inflight.remove(kind) {
            let _ = tx.send(notice);
        }
        guard.done = true;
    }

    /// Full availability list for `(entity, interval)`. `None` when the server returned nothing;
    /// that result is not cached.
    pub(crate) async fn available_metrics(
        &self,
        entity: &ManagedObjectReference,
        interval_id: i32,
    ) -> Result<Option<Vec<PerfMetricId>>, PerfError> {
        let key = (entity.clone(), interval_id);
        if let Some(hit) = self.avail_lock().get(&key) {
            return Ok(Some(hit.iter().map(clone_metric_id).collect()));
        }
        let listed = self
            .inner
            .manager
            .query_available_perf_metric(entity, None, None, Some(interval_id))
            .await?
            .unwrap_or_default();
        if listed.is_empty() {
            return Ok(None);
        }
        let mut cache = self.avail_lock();
        if !cache.contains_key(&key) {
            if cache.len() >= 1024 {
                cache.shift_remove_index(0);
            }
            cache.insert(key, listed.iter().map(clone_metric_id).collect());
        }
        Ok(Some(listed))
    }

    async fn probe_one(&self, entity: &ManagedObjectReference) -> Result<ProviderCaps, PerfError> {
        if let Some(caps) = self.caps_lock().get(&entity.r#type).copied() {
            return Ok(caps);
        }
        let outcome = self.probe(std::slice::from_ref(entity)).await?;
        outcome
            .caps
            .get(&entity.r#type)
            .copied()
            .ok_or(PerfError::NoSupportedEntities)
    }

    fn clock_lock(&self) -> std::sync::MutexGuard<'_, Option<SystemTime>> {
        self.inner
            .clock
            .lock()
            .unwrap_or_else(|err| err.into_inner())
    }

    fn caps_lock(&self) -> std::sync::MutexGuard<'_, HashMap<MoTypesEnum, ProviderCaps>> {
        self.inner
            .caps
            .lock()
            .unwrap_or_else(|err| err.into_inner())
    }

    fn inflight_lock(
        &self,
    ) -> std::sync::MutexGuard<'_, HashMap<MoTypesEnum, watch::Sender<ProbeNotice>>> {
        self.inner
            .inflight
            .lock()
            .unwrap_or_else(|err| err.into_inner())
    }

    fn avail_lock(
        &self,
    ) -> std::sync::MutexGuard<'_, IndexMap<(ManagedObjectReference, i32), Vec<PerfMetricId>>> {
        self.inner
            .availability
            .lock()
            .unwrap_or_else(|err| err.into_inner())
    }
}

pub(crate) fn is_fatal(err: &client::Error) -> bool {
    if is_request_canceled_error(err) {
        return true;
    }
    match err {
        client::Error::MethodFault(fault) => matches!(
            fault.type_,
            Some(StructType::NotAuthenticated) | Some(StructType::InvalidLogin)
        ),
        client::Error::ParseError(_) => false,
        _ => true,
    }
}

fn is_invalid_argument(err: &client::Error) -> bool {
    matches!(
        err,
        client::Error::MethodFault(fault) if fault.type_ == Some(StructType::InvalidArgument)
    )
}

pub(crate) fn clone_metric_id(id: &PerfMetricId) -> PerfMetricId {
    PerfMetricId {
        counter_id: id.counter_id,
        instance: id.instance.clone(),
    }
}

#[derive(Clone)]
enum ProbeNotice {
    Pending,
    Ready(ProviderCaps),
    /// The leader hit a session-fatal fault. Waiters probe again and keep that fault.
    Fatal,
    /// The leader cached nothing. Waiters may try their own entities.
    Miss,
}

enum ProbeRole {
    Lead,
    Wait(watch::Receiver<ProbeNotice>),
}

enum KindProbe {
    Ready(ProviderCaps),
    Failed(IndexMap<ManagedObjectReference, Arc<PerfError>>),
}

struct InflightGuard {
    client: PerfClient,
    kind: MoTypesEnum,
    done: bool,
}

impl Drop for InflightGuard {
    fn drop(&mut self) {
        if self.done {
            return;
        }
        let mut inflight = self.client.inflight_lock();
        if let Some(tx) = inflight.remove(&self.kind) {
            let _ = tx.send(ProbeNotice::Miss);
        }
    }
}

async fn scan_provider(
    client: &PerfClient,
    entities: &[ManagedObjectReference],
) -> Result<KindProbe, PerfError> {
    let mut failed = IndexMap::new();
    for entity in entities {
        match client
            .inner
            .manager
            .query_perf_provider_summary(entity)
            .await
        {
            Ok(summary) => {
                return Ok(KindProbe::Ready(ProviderCaps {
                    current_supported: summary.current_supported,
                    summary_supported: summary.summary_supported,
                    refresh_rate: summary.refresh_rate,
                }));
            }
            Err(err) if is_invalid_argument(&err) => {
                return Ok(KindProbe::Ready(ProviderCaps {
                    current_supported: false,
                    summary_supported: false,
                    refresh_rate: None,
                }));
            }
            Err(err) if is_fatal(&err) => return Err(err.into()),
            Err(err) => {
                failed.insert(entity.clone(), Arc::new(PerfError::from(err)));
            }
        }
    }
    Ok(KindProbe::Failed(failed))
}
