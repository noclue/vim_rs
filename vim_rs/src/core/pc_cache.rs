use indexmap::IndexMap;
use std::cell::RefCell;
use std::ops::Index;
use std::sync::{Arc, RwLock};
use log::{debug, error};
use std::rc::Rc;
use crate::core::client::Client;
use crate::core::pc_helpers;
use crate::core::pc_helpers::{BoxableError, Error, Queriable};
use crate::mo::{PropertyCollector, PropertyFilter, View, ViewManager};
use crate::types::enums::ObjectUpdateKindEnum;
use crate::types::structs::{ManagedObjectReference, ObjectSpec, ObjectUpdate, PropertyFilterSpec, PropertyFilterUpdate, PropertySpec, WaitOptions};

/// A trait for objects that can be retrieved and continuously updated using the `PropertyCollector`
/// API.
pub trait Cacheable: Queriable + TryFrom<ObjectUpdate>
where
    Self::Error: BoxableError
{
    /// The type of the object.
    fn apply_update(&mut self, update: ObjectUpdate) -> pc_helpers::Result<()>;

    /// The ID of the object.
    fn id(&self) -> &ManagedObjectReference;
}

/// A trait for PropertyCollector caches used by the infrastructure to dispatch updates.
pub trait Cache {
    /// Property spec for the objects in this cache.
    fn prop_spec(&self) -> pc_helpers::Result<PropertySpec>;

    /// Apply an update to the cache.
    fn process_update(&mut self, update: Vec<ObjectUpdate>) -> pc_helpers::Result<()>;
}

/// A proxy for a cache that is shared. This helps to use `Rc<RefCell<T>>` over the cache as it is
/// not possible to use both dynamic and static dispatch with `Rc<RefCell<T>>`. This proxy implements
/// the `Cache` trait and forwards calls to the underlying cache object while the cache itself
/// dispatches statically tp the wrapped in `Rc<RefCell<T>>`. Thus, the proxy allows for caches
/// holding different data types to be used in the same CacheManager.
pub struct SharedRefCacheProxy<T: Cache> {
    /// The cache object
    cache: Rc<RefCell<T>>,
}

impl<T: Cache> SharedRefCacheProxy<T> {
    /// Create a new SharedRefCacheProxy.
    pub fn new(cache: Rc<RefCell<T>>) -> Self {
        Self { cache }
    }

    /// Get the cache object.
    pub fn get_cache(&self) -> Rc<RefCell<T>> {
        self.cache.clone()
    }
}

impl<T: Cache> Cache for SharedRefCacheProxy<T> {
    fn prop_spec(&self) -> pc_helpers::Result<PropertySpec> {
        self.cache.borrow().prop_spec()
    }

    fn process_update(&mut self, updates: Vec<ObjectUpdate>) -> pc_helpers::Result<()> {
        self.cache.borrow_mut().process_update(updates)
    }
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
    fn prop_spec(&self) -> pc_helpers::Result<PropertySpec> {
        match self.cache.read() {
            Ok(guard) => guard.prop_spec(),
            Err(e) => {
                error!("Failed to acquire read lock: {}", e);
                return Err(Error::PoisonError(format!("Failed to acquire read lock: {}", e)));
            }
        }
    }

    fn process_update(&mut self, updates: Vec<ObjectUpdate>) -> pc_helpers::Result<()> {
        match self.cache.write() {
            Ok(mut guard) => guard.process_update(updates),
            Err(e) => Err(Error::PoisonError(format!("Failed to acquire write lock: {}", e))),
        }
    }
}

/// Listener trait for receiving notifications about objects in an ObjectCache.
///
/// Implementors can react to objects being added, updated, or removed from the cache.
pub trait ObjectCacheListener<T: Cacheable>
where
    T::Error: BoxableError
{
    /// Called when a new object is added to the cache.
    ///
    /// # Parameters
    /// * `obj` - Reference to the newly added object
    fn on_new(&mut self, obj: &T);

    /// Called when an existing object in the cache is updated.
    ///
    /// # Parameters
    /// * `obj` - Reference to the updated object
    fn on_update(&mut self, obj: &T);

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
    T::Error: BoxableError
{
    cache: IndexMap<String, T>,
    /// Optional listener for receiving notifications about objects in the cache.
    /// This is used to notify about new, updated, or removed objects.
    /// The listener is wrapped in a RefCell to allow for interior mutability.
    listener: Option<RefCell<Box<dyn ObjectCacheListener<T>>>>,
}

impl<T: Cacheable> ObjectCache<T>
where
    T::Error: BoxableError
{
    /// Create a new ObjectCache.
    pub fn new() -> Self {
        Self {
            cache: IndexMap::new(),
            listener: None,
        }
    }

    pub fn new_with_listener(listener: Box<dyn ObjectCacheListener<T>>) -> Self {
        Self {
            cache: IndexMap::new(),
            listener: Some(RefCell::new(listener)),
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

    // Add this method
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    // Optionally add this too
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    fn notify_new(&self, obj: &T) {
        if let Some(listener) = self.listener.as_ref() {
            listener.borrow_mut().on_new(obj);
        }
    }

    fn notify_update(&self, obj: &T) {
        if let Some(listener) = self.listener.as_ref() {
            listener.borrow_mut().on_update(obj);
        }
    }

    fn notify_remove(&self, obj: T) {
        if let Some(listener) = self.listener.as_ref() {
            listener.borrow_mut().on_remove(obj);
        }
    }

}

impl<T: Cacheable> Index<usize> for ObjectCache<T>
where
    T::Error: BoxableError
{
    type Output = T;

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
    T::Error: BoxableError
{
    type Output = T;

    fn index(&self, key: &str) -> &Self::Output {
        match self.cache.get(key) {
            Some(value) => value,
            None => panic!("No entry found for key: {}", key),
        }
    }
}

impl<T: Cacheable> Index<String> for ObjectCache<T>
where
    T::Error: BoxableError
{
    type Output = T;

    fn index(&self, key: String) -> &Self::Output {
        self.index(key.as_str())
    }
}

impl<'a, T: Cacheable> IntoIterator for &'a ObjectCache<T>
where
    T::Error: BoxableError
{
    type Item = &'a T;
    type IntoIter = indexmap::map::Values<'a, String, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.cache.values()
    }
}

impl<T: Cacheable> Cache for ObjectCache<T>
where
    T::Error: BoxableError
{
    /// Get the property spec for the objects in this cache.
    fn prop_spec(&self) -> pc_helpers::Result<PropertySpec> {
        Ok(T::prop_spec())
    }

    /// Process a PropertyCollector update.
    fn process_update(&mut self, updates: Vec<ObjectUpdate>) -> pc_helpers::Result<()> {
        for update in updates {
            let id = update.obj.value.clone();
            match update.kind {
                ObjectUpdateKindEnum::Enter | ObjectUpdateKindEnum::Modify => {
                    if let Some(obj) = self.cache.get_mut(&id) {
                        debug!("Updating '{}' object in cache", id);
                        obj.apply_update(update)?;

                        // Notify the listener about the update
                        if let Some(obj) = self.cache.get(&id) {
                            self.notify_update(obj);
                        } else {
                            error!("Failed to add object to cache");
                        }
                    } else {
                        // If the object is not in the cache, try to create it
                        match T::try_from(update) {
                            Ok(new_obj) => {
                                debug!("Adding '{}' object to cache", id);
                                self.cache.insert(id.clone(), new_obj);
                                // Notify the listener about the new object
                                if let Some(obj) = self.cache.get(&id) {
                                    self.notify_new(obj);
                                } else {
                                    error!("Failed to add object to cache");
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
    cache: Box<dyn Cache>,
    /// Optional view ID if add_container_cache is used
    view: Option<String>,
}

/// A manager for object caches. This is used to manage multiple caches and dispatch updates to
/// them. The CacheManager is responsible for creating the filters and dispatching updates to the
/// caches. The CacheManager is also responsible for cleaning up the filters and caches when
/// no longer needed.
///
/// Use the `destroy` method to clean up all caches and filters.
pub struct CacheManager {
    client: Arc<Client>,
    property_collector: PropertyCollector,
    view_manager: ViewManager,
    caches: std::collections::HashMap<String, CacheRecord>,
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
    pub fn new(client: Arc<Client>) -> pc_helpers::Result<Self> {
        let pc_mo_id = &client.service_content().property_collector.value;
        let property_collector = PropertyCollector::new(client.clone(), pc_mo_id);
        let Some(view_manager_moref) = &client.service_content().view_manager else {
            return Err(Error::InternalError("cannot find view_manager".to_string()));
        };
        let view_manager = ViewManager::new(client.clone(), &view_manager_moref.value);
        Ok(Self {
            client,
            property_collector,
            view_manager,
            caches: std::collections::HashMap::new(),
        })
    }

    /// Create a new CacheManager with an existing PropertyCollector. This allows to not use the
    /// default PropertyCollector, have different PropertyCollector instances and different
    /// CacheManager instances.
    pub fn new_with_property_collector(client: Arc<Client>, property_collector: PropertyCollector) -> pc_helpers::Result<Self> {
        let Some(view_manager_moref) = &client.service_content().view_manager else {
            return Err(Error::InternalError("cannot find view_manager".to_string()));
        };
        let view_manager = ViewManager::new(client.clone(), &view_manager_moref.value);
        Ok(Self {
            client,
            property_collector,
            view_manager,
            caches: std::collections::HashMap::new(),
        })
    }

    /// Create a new Monitor with the same PropertyCollector as the CacheManager.
    pub fn create_monitor(&self) -> pc_helpers::Result<Monitor> {
        Ok(Monitor::new_with_property_collector(self.property_collector.clone())?)
    }

    /// Add an object cache for a specific type of object in a given container like Folder, Datacenter, etc.
    pub async fn add_container_cache(&mut self, cache: Box<dyn Cache>, container: &ManagedObjectReference) -> pc_helpers::Result<ManagedObjectReference> {
        let view = self.view_manager.create_container_view(container,
                                                           Some(&[cache.prop_spec()?.r#type.clone()]),
                                                           true,
        ).await?;

        let res= self.add_cache(cache, pc_helpers::obj_spec_for_view(view.clone())).await;
        if let Ok(ref filter) = res {
            if let Some(record) = self.caches.get_mut(&filter.value) {
                record.view = Some(view.value.clone());
            }
        };
        res
    }



    /// Add a cache for a specific type of object. This creates a filter on the server to update
    /// the cache. The filter is created with the given object set.
    pub async fn add_cache(&mut self, cache: Box<dyn Cache>, object_set: Vec<ObjectSpec>) -> pc_helpers::Result<ManagedObjectReference> {
        let filter_spec = PropertyFilterSpec {
            object_set,
            prop_set: vec![cache.prop_spec()?],
            report_missing_objects_in_results: None,
        };

        let filter = self.property_collector.create_filter(&filter_spec, false).await?;

        self.caches.insert(filter.value.clone(), CacheRecord{
            cache,
            view: None,
        });
        Ok(filter)
    }

    /// Apply updates to the caches. This is used to dispatch updates to the caches.
    pub fn apply_updates(&mut self, updates: Vec<PropertyFilterUpdate>) -> pc_helpers::Result<()> {
        for update in updates {
            let filter_id = update.filter.value.clone();
            if let Some(cache_rec) = self.caches.get_mut(&filter_id) {
                if let Some(object_update) = update.object_set {
                    cache_rec.cache.process_update(object_update)?;
                } else {
                    debug!("No object updates found for filter '{}'", filter_id);
                }
            } else {
                error!("Update received for unknown filter '{}'", filter_id);
            }
        }
        Ok(())
    }

    /// Remove a cache by its ID. This is used to clean up caches that are no longer needed.
    pub async fn remove_cache(&mut self, filter: &ManagedObjectReference) -> pc_helpers::Result<()> {
        if let Some(cache_rec) = self.caches.remove(&filter.value) {
            self.dispose_filter(&filter.value, &cache_rec).await;
        }
        Ok(())
    }

    /// Remove all caches. This is used to clean up all caches that are no longer needed.
    pub async fn destroy(&mut self) -> pc_helpers::Result<()> {
        for (filter_id, cache_rec) in self.caches.iter() {
            self.dispose_filter(&filter_id, &cache_rec).await;
        }
        self.caches.clear();
        Ok(())
    }

    /// Dispose of a filter and as needed the associated view.
    async fn dispose_filter(&self, filter_id: &str, cache_rec: &CacheRecord) {
        let filter = PropertyFilter::new(self.client.clone(), &filter_id);
        if let Err(e) = filter.destroy_property_filter().await {
            error!("Error destroying property filter {}: {:?}", filter_id, e);
        };
        if let Some(ref view_id) = cache_rec.view {
            let view = View::new(self.client.clone(), view_id);
            if let Err(e) = view.destroy_view().await {
                error!("Error destroying view {}: {:?}", view_id, e);
            }
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
    fn new_with_property_collector(property_collector: PropertyCollector) -> pc_helpers::Result<Self> {
        Ok(Self {
            property_collector,
            version: "".to_string(),
        })
    }

    /// Waits for updates from the PropertyCollector. This is used to get updates from the server.
    /// It sends a long poll HTTP request to the server.
    ///
    /// MAX_OBJECT_UPDATES_PER_CALL is used to limit the number of updates returned in a single call.
    ///
    /// **Parameters**
    /// * `delay_s` - The maximum time to wait for updates in seconds. This is used to limit the
    /// time the server waits for updates. If no updates are received within this time, the server
    /// will return an empty response. This should not exceed several 10s of seconds as to avoid TCP
    /// idle timeouts in network equipment.
    pub async fn wait_updates(&mut self, delay_s: i32) -> pc_helpers::Result<Option<Vec<PropertyFilterUpdate>>>
    {
        let options = WaitOptions {
            max_wait_seconds: Some(delay_s),
            max_object_updates: Some(MAX_OBJECT_UPDATES_PER_CALL),
        };
        let result = self.property_collector.wait_for_updates_ex(Some(&self.version), Some(&options)).await?;
        let Some(update_set) = result else {
            return Ok(None);
        };
        self.version = update_set.version.clone();

        Ok(update_set.filter_set)
    }
}