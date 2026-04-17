use indexmap::IndexMap;
use std::ops::Index;
use std::sync::{Arc, RwLock, Mutex};
use std::time::{Duration, Instant};
use log::{debug, error, trace, warn};
use crate::core::client::VimClient;
use crate::core::error::{Error, Result};
use crate::core::pc_helpers::{self, BoxableError, Queriable};
use crate::mo::{PropertyCollector, PropertyFilter, ViewManager};
use crate::types::enums::ObjectUpdateKindEnum;
use crate::types::structs::{
    ManagedObjectReference, ObjectSpec, ObjectUpdate, PropertyFilterSpec, PropertyFilterUpdate,
    PropertySpec, WaitOptions,
};

/// A trait for objects that can be retrieved and continuously updated using the `PropertyCollector`
/// API.
pub trait Cacheable: Queriable + TryFrom<ObjectUpdate>
where
    Self::Error: BoxableError,
{
    /// The type of the object.
    fn apply_update(&mut self, update: ObjectUpdate) -> Result<()>;

    /// The ID of the object.
    fn id(&self) -> &ManagedObjectReference;
}

/// A trait for PropertyCollector caches used by the infrastructure to dispatch updates.
pub trait Cache {
    /// Property spec for the objects in this cache.
    fn prop_spec(&self) -> Result<PropertySpec>;

    /// Apply an update to the cache.
    fn process_update(&mut self, update: Vec<ObjectUpdate>) -> Result<()>;
}

/// A thread-safe proxy with read-write locking using Arc<RwLock<T>>
pub struct ReadWriteCacheProxy<T: Cache> {
    cache: Arc<RwLock<T>>,
}

impl<T: Cache> ReadWriteCacheProxy<T> {
    pub fn new(cache: Arc<RwLock<T>>) -> Self {
        Self { cache }
    }

    pub fn get_cache(&self) -> Arc<RwLock<T>> {
        self.cache.clone()
    }
}

impl<T: Cache> Cache for ReadWriteCacheProxy<T> {
    fn prop_spec(&self) -> Result<PropertySpec> {
        match self.cache.read() {
            Ok(guard) => guard.prop_spec(),
            Err(e) => {
                error!("Failed to acquire read lock: {}", e);
                return Err(Error::lock_poisoned(format!("Failed to acquire read lock: {}", e)));
            }
        }
    }

    fn process_update(&mut self, updates: Vec<ObjectUpdate>) -> Result<()> {
        match self.cache.write() {
            Ok(mut guard) => guard.process_update(updates),
            Err(e) => Err(Error::lock_poisoned(format!("Failed to acquire write lock: {}", e))),
        }
    }
}

/// Listener trait for receiving notifications about objects in an ObjectCache.
///
/// Implementors can react to objects being added, updated, or removed from the cache.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheAction {
    /// Keep the object in the cache.
    Keep,
    /// Evict the object from the cache after the callback returns. Eviction will trigger
    /// `ObjectCacheListener::on_remove` with ownership of the removed object.
    Evict,
}

pub trait ObjectCacheListener<T: Cacheable>: Send
where
    T::Error: BoxableError,
{
    /// Called when a new object is added to the cache.
    ///
    /// # Parameters
    /// * `obj` - Reference to the newly added object
    fn on_new(&mut self, obj: &T) -> CacheAction;

    /// Called when an existing object in the cache is updated.
    ///
    /// # Parameters
    /// * `obj` - Reference to the updated object
    fn on_update(&mut self, obj: &T) -> CacheAction;

    /// Called when an object is removed from the cache.
    ///
    /// # Parameters
    /// * `obj` - the object being removed
    fn on_remove(&mut self, obj: T);
}

/// A cache for objects of type T. This is a simple in-memory cache for property collector result
/// objects that stores objects by their ID.
pub struct ObjectCache<T: Cacheable>
where
    T::Error: BoxableError,
{
    cache: IndexMap<String, T>,
    /// Optional listener for receiving notifications about objects in the cache.
    /// This is used to notify about new, updated, or removed objects.
    /// The listener is wrapped in a Mutex to allow for interior mutability in a thread-safe manner.
    listener: Option<Mutex<Box<dyn ObjectCacheListener<T>>>>,
}

impl<T: Cacheable> ObjectCache<T>
where
    T::Error: BoxableError,
{
    /// Create a new ObjectCache.
    pub fn new() -> Self {
        Self {
            cache: IndexMap::new(),
            listener: None,
        }
    }

    /// Create a new ObjectCache with a listener.
    pub fn new_with_listener(listener: Box<dyn ObjectCacheListener<T>>) -> Self {
        Self {
            cache: IndexMap::new(),
            listener: Some(Mutex::new(listener)),
        }
    }

    /// Get an object by its ID.
    pub fn get(&self, id: &str) -> Option<&T> {
        self.cache.get(id)
    }

    /// Borrowing iterator over the values in the cache.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.cache.values()
    }

    /// Return the number of objects in the cache.
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Returned true if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    fn notify_new(&self, obj: &T) -> CacheAction {
        if let Some(listener) = self.listener.as_ref() {
            if let Ok(mut guard) = listener.lock() {
                return guard.on_new(obj);
            } else {
                error!("Failed to acquire listener lock for on_new notification");
            }
        }
        CacheAction::Keep
    }

    fn notify_update(&self, obj: &T) -> CacheAction {
        if let Some(listener) = self.listener.as_ref() {
            if let Ok(mut guard) = listener.lock() {
                return guard.on_update(obj);
            } else {
                error!("Failed to acquire listener lock for on_update notification");
            }
        }
        CacheAction::Keep
    }

    fn notify_remove(&self, obj: T) {
        if let Some(listener) = self.listener.as_ref() {
            if let Ok(mut guard) = listener.lock() {
                guard.on_remove(obj);
            } else {
                error!("Failed to acquire listener lock for on_remove notification");
            }
        }
    }
}

impl<T: Cacheable> Index<usize> for ObjectCache<T>
where
    T::Error: BoxableError,
{
    type Output = T;

    /// Get an object by its index.
    fn index(&self, index: usize) -> &Self::Output {
        if let Some((_, value)) = self.cache.get_index(index) {
            value
        } else {
            panic!("Index out of bounds: {}", index)
        }
    }
}

impl<T: Cacheable> Index<&str> for ObjectCache<T>
where
    T::Error: BoxableError,
{
    type Output = T;

    /// Get an object by its ID.
    fn index(&self, key: &str) -> &Self::Output {
        match self.cache.get(key) {
            Some(value) => value,
            None => panic!("No entry found for key: {}", key),
        }
    }
}

impl<T: Cacheable> Index<String> for ObjectCache<T>
where
    T::Error: BoxableError,
{
    type Output = T;

    /// Get an object by its ID.
    fn index(&self, key: String) -> &Self::Output {
        self.index(key.as_str())
    }
}

impl<'a, T: Cacheable> IntoIterator for &'a ObjectCache<T>
where
    T::Error: BoxableError,
{
    type Item = &'a T;
    type IntoIter = indexmap::map::Values<'a, String, T>;
    /// Create an iterator over the values in the cache.
    fn into_iter(self) -> Self::IntoIter {
        self.cache.values()
    }
}

impl<T: Cacheable> Cache for ObjectCache<T>
where
    T::Error: BoxableError,
{
    /// Get the property spec for the objects in this cache.
    fn prop_spec(&self) -> Result<PropertySpec> {
        Ok(T::prop_spec())
    }

    /// Process a PropertyCollector update.
    fn process_update(&mut self, updates: Vec<ObjectUpdate>) -> Result<()> {
        for update in updates {
            let id = update.obj.value.clone();
            match update.kind {
                ObjectUpdateKindEnum::Enter | ObjectUpdateKindEnum::Modify => {
                    if let Some(obj) = self.cache.get_mut(&id) {
                        debug!("Updating '{}' object in cache", id);
                        obj.apply_update(update)?;

                        // Notify the listener about the update
                        let action = if let Some(obj) = self.cache.get(&id) {
                            self.notify_update(obj)
                        } else {
                            error!("Failed to find object in cache after update");
                            CacheAction::Keep
                        };

                        if action == CacheAction::Evict {
                            if let Some(obj) = self.cache.shift_remove(&id) {
                                debug!("Evicting '{}' object from cache (listener requested)", id);
                                self.notify_remove(obj);
                            }
                        }
                    } else {
                        // If the object is not in the cache, try to create it
                        match T::try_from(update) {
                            Ok(new_obj) => {
                                debug!("Adding '{}' object to cache", id);
                                self.cache.insert(id.clone(), new_obj);
                                // Notify the listener about the new object
                                let action = if let Some(obj) = self.cache.get(&id) {
                                    self.notify_new(obj)
                                } else {
                                    error!("Failed to find object in cache after insert");
                                    CacheAction::Keep
                                };

                                if action == CacheAction::Evict {
                                    if let Some(obj) = self.cache.shift_remove(&id) {
                                        debug!("Evicting '{}' object from cache (listener requested)", id);
                                        self.notify_remove(obj);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to create object from update: {}", e);
                            }
                        }
                    }
                }
                ObjectUpdateKindEnum::Leave => {
                    debug!("Object {} left", id);
                    // Remove the object from the cache
                    if let Some(obj) = self.cache.shift_remove(&id) {
                        debug!("Removing '{}' object from cache", id);
                        self.notify_remove(obj);
                    } else {
                        debug!("Object to be removed {} not found in cache", id);
                    }
                }
                _ => {
                    debug!("Unknown update kind: {:?}", update.kind);
                }
            }
        }
        Ok(())
    }
}

/// A record for a cache object. This is used to store the cache object and its associated view ID.
struct CacheRecord {
    /// The cache object
    ///
    /// The cache must be `Send + Sync` because `CacheManager` is used from async tasks
    /// that may be moved between threads.
    cache: Box<dyn Cache + Send + Sync>,
    /// Optional view ID if add_container_cache is used
    view: Option<ManagedObjectReference>,
}

/// A manager for object caches. This is used to manage multiple caches and dispatch updates to
/// them. The CacheManager is responsible for creating the filters and dispatching updates to the
/// caches. The CacheManager is also responsible for cleaning up the filters and caches when
/// no longer needed.
///
/// Use the `destroy` method to clean up all caches and filters.
pub struct CacheManager {
    client: Arc<dyn VimClient>,
    property_collector: PropertyCollector,
    view_manager: ViewManager,
    caches: std::collections::HashMap<String, CacheRecord>,
    /// When `true`, every filter topology change (`add_cache`, `remove_cache`,
    /// `destroy`) issues a best-effort `CancelWaitForUpdates` on the session's
    /// `PropertyCollector` to wake any long-polling [`Monitor::wait_updates`]
    /// so the new filter set takes effect on the next iteration.
    ///
    /// Real vCenter/ESX deliver the initial enter set for a newly added filter
    /// into an already-running wait automatically. The `govmomi` simulator
    /// (`vcsim`) does not — its wait loop only reacts to CRUD events on managed
    /// objects, so without a wake an app that adds filters at runtime appears
    /// frozen until the current `WaitForUpdatesEx` times out.
    ///
    /// Defaults to `false`. See [`CacheManager::set_cancel_wait_on_filter_change`].
    cancel_wait_on_filter_change: bool,
}

/// A CacheManager is used to manage multiple caches and dispatch updates to them. Each cache has a
/// an associated filter.  The CacheManager is responsible for creating the filters and
/// dispatching updates to the caches. The CacheManager is also responsible for cleaning up
/// the filters and caches when they are no longer needed using the `destroy` method.
impl CacheManager {
    /// Create a new CacheManager with the default PropertyCollector. This is used to manage
    /// multiple caches and dispatch updates to them. The default PropertyCollector is used
    /// to create filters for the caches. Only one CacheManager can work correctly with given
    /// PropertyCollector including the default one.
    pub fn new(client: Arc<dyn VimClient>) -> Result<Self> {
        let pc_mo_id = &client.service_content().property_collector.value;
        let property_collector = PropertyCollector::new(client.clone(), pc_mo_id);
        let Some(view_manager_moref) = &client.service_content().view_manager else {
            return Err(Error::internal("cannot find view_manager".to_string()));
        };
        let view_manager = ViewManager::new(client.clone(), &view_manager_moref.value);
        Ok(Self {
            client,
            property_collector,
            view_manager,
            caches: std::collections::HashMap::new(),
            cancel_wait_on_filter_change: false,
        })
    }

    /// Create a new CacheManager with an existing PropertyCollector. This allows to not use the
    /// default PropertyCollector, have different PropertyCollector instances and different
    /// CacheManager instances.
    pub fn new_with_property_collector(
        client: Arc<dyn VimClient>,
        property_collector: PropertyCollector,
    ) -> Result<Self> {
        let Some(view_manager_moref) = &client.service_content().view_manager else {
            return Err(Error::internal("cannot find view_manager".to_string()));
        };
        let view_manager = ViewManager::new(client.clone(), &view_manager_moref.value);
        Ok(Self {
            client,
            property_collector,
            view_manager,
            caches: std::collections::HashMap::new(),
            cancel_wait_on_filter_change: false,
        })
    }

    /// Enable or disable the automatic wake-up of in-flight `WaitForUpdatesEx`
    /// calls when filter topology changes.
    ///
    /// When enabled (the default), `add_cache`, `remove_cache`, and `destroy`
    /// issue a best-effort `CancelWaitForUpdates` on the session's
    /// `PropertyCollector` after mutating the filter set. Combined with
    /// [`Monitor::wait_updates`]' built-in cancel recovery, this makes new
    /// filters take effect immediately on `vcsim` (which otherwise ignores
    /// filter changes for the rest of the in-flight long-poll).
    ///
    /// Disabling it restores the pre-0.4.x behavior: filters only become
    /// visible on the next naturally-scheduled wait iteration. Most callers
    /// should leave this enabled; disable it only if another task on the same
    /// session intentionally relies on `cancel_wait_for_updates` semantics.
    pub fn set_cancel_wait_on_filter_change(&mut self, enabled: bool) {
        self.cancel_wait_on_filter_change = enabled;
    }

    /// Best-effort wake-up of any in-flight `WaitForUpdatesEx` on the session
    /// `PropertyCollector`. No-op when the policy is disabled. Errors are
    /// logged and ignored: a failed cancel only delays visibility of the new
    /// filter set until the next wait iteration, it does not compromise
    /// correctness.
    async fn wake_in_flight_wait(&self) {
        if !self.cancel_wait_on_filter_change {
            return;
        }
        trace!("CacheManager::wake_in_flight_wait: canceling in-flight wait");
        if let Err(e) = self.property_collector.cancel_wait_for_updates().await {
            debug!(
                "CacheManager::wake_in_flight_wait: cancel_wait_for_updates after filter mutation failed (ignored): {}",
                e
            );
        }
    }

    /// Create a new Monitor with the same PropertyCollector as the CacheManager.
    pub fn create_monitor(&self) -> Result<Monitor> {
        Ok(Monitor::new_with_property_collector(self.property_collector.clone())?)
    }

    /// Add an object cache for a specific type of object in a given container like Folder, Datacenter, etc.
    pub async fn add_container_cache(
        &mut self,
        cache: Box<dyn Cache + Send + Sync>,
        container: &ManagedObjectReference,
    ) -> Result<ManagedObjectReference> {
        let view = self.view_manager.create_container_view(container,
                                                           Some(&[cache.prop_spec()?.r#type.clone()]),
                                                           true,
        ).await?;

        let res = self
            .add_cache(cache, pc_helpers::obj_spec_for_view(view.clone()))
            .await;
        if let Ok(ref filter) = res {
            if let Some(record) = self.caches.get_mut(&filter.value) {
                record.view = Some(view.clone());
            }
        };
        res
    }

    pub async fn add_list_cache(
        &mut self,
        cache: Box<dyn Cache + Send + Sync>,
        obj: &[crate::types::structs::ManagedObjectReference],
    ) -> Result<ManagedObjectReference> {
        let view = self.view_manager.create_list_view(Some(obj)).await?;

        let res = self
            .add_cache(cache, pc_helpers::obj_spec_for_view(view.clone()))
            .await;
        if let Ok(ref filter) = res {
            if let Some(record) = self.caches.get_mut(&filter.value) {
                record.view = Some(view.clone());
            }
        };
        res
    }

    /// Add a cache for a specific type of object. This creates a filter on the server to update
    /// the cache. The filter is created with the given object set.
    pub async fn add_cache(
        &mut self,
        cache: Box<dyn Cache + Send + Sync>,
        object_set: Vec<ObjectSpec>,
    ) -> Result<ManagedObjectReference> {
        let prop_spec = cache.prop_spec()?;
        trace!(
            "CacheManager::add_cache creating filter for type={} all={:?} path_set={:?} object_count={}",
            prop_spec.r#type,
            prop_spec.all,
            prop_spec.path_set,
            object_set.len()
        );
        let filter_spec = PropertyFilterSpec {
            object_set,
            prop_set: vec![prop_spec],
            report_missing_objects_in_results: None,
        };

        let filter = self
            .property_collector
            .create_filter(&filter_spec, false)
            .await?;

        debug!(
            "CacheManager::add_cache registered filter={} cache_count_before_insert={}",
            filter.value,
            self.caches.len()
        );
        self.caches
            .insert(filter.value.clone(), CacheRecord { cache, view: None });
        self.wake_in_flight_wait().await;
        Ok(filter)
    }

    /// Apply updates to the caches. This is used to dispatch updates to the caches.
    pub fn apply_updates(&mut self, updates: Vec<PropertyFilterUpdate>) -> Result<()> {
        for update in updates {
            let filter_id = update.filter.value.clone();
            if let Some(cache_rec) = self.caches.get_mut(&filter_id) {
                if let Some(object_update) = update.object_set {
                    let object_count = object_update.len();
                    let object_ids: Vec<String> =
                        object_update.iter().map(|obj| obj.obj.value.clone()).collect();
                    debug!(
                        "CacheManager::apply_updates dispatching filter={} object_count={} object_ids={:?}",
                        filter_id,
                        object_count,
                        object_ids
                    );
                    cache_rec.cache.process_update(object_update)?;
                } else {
                    debug!("No object updates found for filter '{}'", filter_id);
                }
            } else {
                warn!("Update received for unknown filter '{}'", filter_id);
            }
        }
        Ok(())
    }

    /// Remove a cache by its ID. This is used to clean up caches that are no longer needed.
    pub async fn remove_cache(&mut self, filter: &ManagedObjectReference) -> Result<()> {
        if let Some(cache_rec) = self.caches.remove(&filter.value) {
            self.dispose_filter(&filter.value, &cache_rec).await;
            self.wake_in_flight_wait().await;
        }
        Ok(())
    }

    /// Remove all caches. This is used to clean up all caches that are no longer needed.
    pub async fn destroy(&mut self) -> Result<()> {
        let had_caches = !self.caches.is_empty();
        for (filter_id, cache_rec) in self.caches.iter() {
            self.dispose_filter(&filter_id, &cache_rec).await;
        }
        self.caches.clear();
        if had_caches {
            self.wake_in_flight_wait().await;
        }
        Ok(())
    }

    /// Dispose of a filter and as needed the associated view.
    async fn dispose_filter(&self, filter_id: &str, cache_rec: &CacheRecord) {
        let filter = PropertyFilter::new(self.client.clone(), &filter_id);
        if let Err(e) = filter.destroy_property_filter().await {
            error!("Error destroying property filter {}: {:?}", filter_id, e);
        };
        if let Some(ref view_moref) = cache_rec.view {
            if let Err(e) = self.client
                    .invoke_void("", view_moref.r#type.as_str(), &view_moref.value, "DestroyView", None)
                    .await {
                error!("Error destroying view {}:{}: {:?}", view_moref.r#type.as_str(), view_moref.value, e);
            };
        }
    }
}

/// Utility for calling the PropertyCollector::WaitForUpdates API successively. It keeps track of
/// the version token. It can be used in an event loop to wait for updates from the vCenter server.
/// The output of the wait_updates is meant for use with the CacheManager::apply_updates method.
pub struct Monitor {
    property_collector: PropertyCollector,
    version: String,
}

const MAX_OBJECT_UPDATES_PER_CALL: i32 = 256;

impl Monitor {
    /// Create a new Monitor with an existing PropertyCollector. This allows to not use the
    /// default PropertyCollector, have different PropertyCollector instances and different
    /// CacheManager instances.
    fn new_with_property_collector(property_collector: PropertyCollector) -> Result<Self> {
        Ok(Self {
            property_collector,
            version: "".to_string(),
        })
    }

    /// Cancels an in-flight [`PropertyCollector::wait_for_updates_ex`] on the same session.
    ///
    /// The corresponding [`Monitor::wait_updates`] call completes with [`crate::core::client::Error::MethodFault`]
    /// (`RequestCanceled`). This method uses the same [`PropertyCollector`] handle as
    /// [`Monitor::wait_updates`] and is safe to call from another task while a wait is running.
    pub async fn cancel_wait(&self) -> crate::core::client::Result<()> {
        self.property_collector.cancel_wait_for_updates().await
    }

    /// Resets the client-side collector version token to the initial empty string (`""`).
    ///
    /// Some simulators require this before the first `WaitForUpdatesEx` after filter topology
    /// changes; real ESXi/vCenter often do not.
    pub fn reset_version(&mut self) {
        self.version.clear();
    }

    /// Waits for updates from the PropertyCollector. This is used to get updates from the server.
    /// It sends a long poll HTTP request to the server.
    ///
    /// MAX_OBJECT_UPDATES_PER_CALL is used to limit the number of updates returned in a single call.
    ///
    /// **Parameters**
    /// * `delay_s` - The maximum total time to wait for updates in seconds, including any internal
    /// retries triggered by absorbed `RequestCanceled` faults. If `delay_s <= 0`, the server is
    /// instructed to return after one update calculation (single-shot) and no deadline tracking
    /// is performed. Typical values are a few tens of seconds so as not to exceed TCP idle
    /// timeouts in network equipment.
    ///
    /// # Cancellation recovery
    ///
    /// When a companion task (such as [`CacheManager`] with its default
    /// [`CacheManager::set_cancel_wait_on_filter_change`] policy, or an explicit
    /// [`Monitor::cancel_wait`] call) cancels an in-flight wait, the server returns
    /// `RequestCanceled`. This method absorbs up to [`Monitor::MAX_ABSORBED_CANCELS`] such
    /// faults within a single call and transparently re-issues the wait with a freshly reset
    /// version token. The reset is required for `vcsim` (which only emits the initial enter
    /// set for newly registered filters via the `Version=""` snapshot path) and is harmless
    /// on real vCenter/ESX. Callers relying on externally-visible `RequestCanceled` semantics
    /// should trigger shutdown by dropping the channel that consumes updates rather than via
    /// cancel.
    ///
    /// # Deadline shrinking
    ///
    /// When `delay_s > 0`, each internal retry receives only the time budget that remains
    /// until the original deadline, so the total wall-clock time of a single
    /// `wait_updates(delay_s)` call never materially exceeds `delay_s` even in the presence
    /// of repeated absorbed cancels. Returning `Ok(None)` signals natural timeout
    /// (either because the server returned no updates or because the deadline was reached
    /// before another attempt could complete usefully).
    pub async fn wait_updates(&mut self, delay_s: i32) -> Result<Option<Vec<PropertyFilterUpdate>>>
    {
        let started = Instant::now();
        let total_budget = if delay_s > 0 {
            Some(Duration::from_secs(delay_s as u64))
        } else {
            None
        };

        for absorbed in 0..=Self::MAX_ABSORBED_CANCELS {
            let attempt_delay_s = match total_budget {
                Some(budget) => {
                    let elapsed = started.elapsed();
                    if elapsed >= budget {
                        trace!(
                            "Monitor::wait_updates budget exhausted after {} absorbed cancel(s); returning Ok(None)",
                            absorbed
                        );
                        return Ok(None);
                    }
                    let remaining = budget - elapsed;
                    let secs = remaining.as_secs();
                    if secs == 0 {
                        // Less than one second left. Don't bother issuing a sub-second
                        // long-poll; the caller will re-enter on the next loop tick.
                        return Ok(None);
                    }
                    secs.min(i32::MAX as u64) as i32
                }
                None => delay_s,
            };

            let options = WaitOptions {
                max_wait_seconds: Some(attempt_delay_s),
                max_object_updates: Some(MAX_OBJECT_UPDATES_PER_CALL),
            };
            trace!(
                "Monitor::wait_updates sending wait with version={:?} delay_s={} absorbed_cancels={}",
                self.version,
                attempt_delay_s,
                absorbed
            );

            match self
                .property_collector
                .wait_for_updates_ex(Some(&self.version), Some(&options))
                .await
            {
                Err(e) if crate::core::client::is_request_canceled_error(&e) => {
                    // Either CacheManager woke us up because the filter set changed, or the
                    // app explicitly called `cancel_wait`. In both cases we force the next
                    // iteration through the server's full-snapshot path so newly registered
                    // filters report their initial enter set (required for vcsim, harmless
                    // on real vCenter/ESX where the enter set would have been delivered
                    // into the running wait anyway).
                    trace!(
                        "Monitor::wait_updates absorbed RequestCanceled, resetting version and re-arming"
                    );
                    self.version.clear();
                    continue;
                }
                Err(e) => return Err(e.into()),
                Ok(None) => {
                    trace!(
                        "Monitor::wait_updates returned no update set for version={:?}",
                        self.version
                    );
                    return Ok(None);
                }
                Ok(Some(update_set)) => {
                    let filter_ids: Vec<String> = update_set
                        .filter_set
                        .as_ref()
                        .map(|set| set.iter().map(|u| u.filter.value.clone()).collect())
                        .unwrap_or_default();
                    debug!(
                        "Monitor::wait_updates received version={:?} previous_version={:?} filter_ids={:?}",
                        update_set.version, self.version, filter_ids
                    );
                    self.version = update_set.version.clone();
                    return Ok(update_set.filter_set);
                }
            }
        }

        Err(Error::internal(format!(
            "Monitor::wait_updates absorbed more than {} consecutive RequestCanceled faults",
            Self::MAX_ABSORBED_CANCELS
        )))
    }

    /// Upper bound on consecutive `RequestCanceled` faults absorbed by a single
    /// [`Monitor::wait_updates`] call before surfacing an internal error. Prevents
    /// runaway spin if some misbehaving task keeps issuing cancels.
    const MAX_ABSORBED_CANCELS: usize = 16;
}
