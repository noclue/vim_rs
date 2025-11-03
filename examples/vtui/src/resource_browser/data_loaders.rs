use vim_rs::core::pc_cache::{CacheManager, Cacheable, ObjectCache, SharedRefCacheProxy};
use std::rc::Rc;
use std::cell::RefCell;
use vim_rs::types::structs::{ManagedObjectReference, ObjectSpec, TraversalSpec};
use vim_rs::core::pc_helpers::BoxableError;
use ratatui::widgets::Row;
use crate::resource_browser::indexed_cache::IndexedCache;
use crate::resource_browser::tabular_data::{TableDataSource, TabularData};

pub(crate) async fn load_from_container<T: TabularData + Cacheable + Send + Sync + 'static>(
    cache_mgr: Rc<RefCell<CacheManager>>,
    container: &ManagedObjectReference,
) -> anyhow::Result<(Box<dyn TableDataSource>, ManagedObjectReference)>
where
    <T as TryFrom<vim_rs::types::structs::ObjectUpdate>>::Error: BoxableError,
    for<'a> Row<'static>: From<&'a T>,
{
    let cache = Arc::new(RwLock::new(ObjectCache::<T>::new()));
    let filter = cache_mgr
        .borrow_mut()
        .add_container_cache(
            Box::new(ReadWriteCacheProxy::new(cache.clone())),
            container
        )
        .await?;
    let indexed_cache = IndexedCache::new(cache.clone());
    Ok((Box::new(indexed_cache), filter))
}

type StaticStr = &'static str;

pub(crate) async fn load_from_property<T: TabularData + Cacheable + Send + Sync + 'static>(
    cache_mgr: Rc<RefCell<CacheManager>>,
    object: &ManagedObjectReference,
    property: &str,
) -> anyhow::Result<(Box<dyn TableDataSource>, ManagedObjectReference)>
where
    <T as TryFrom<vim_rs::types::structs::ObjectUpdate>>::Error: BoxableError,
    for<'a> Row<'static>: From<&'a T>,
{
    let object_specs = vec![ObjectSpec {
        obj: object.clone(),
        skip: Some(false),
        select_set: Some(vec![Box::new(TraversalSpec {
            selection_spec_: SelectionSpec {
                name: Some("expandProperty".to_string()),
            },
            r#type: StaticStr::from(object.r#type.clone()).to_string(),
            path: property.to_string(),
            skip: Some(false),
            select_set: None,
        })])],
    }];
    let cache = Rc::new(RefCell::new(ObjectCache::<T>::new()));
    let filter = cache_mgr
        .borrow_mut()
        .add_cache(
            Box::new(ReadWriteCacheProxy::new(cache.clone())),
            object_specs
        )
        .await?;
    let indexed_cache = IndexedCache::new(cache.clone());
    Ok((Box::new(indexed_cache), filter))
}

pub(crate) async fn load_from_list<T: TabularData + Cacheable + Send + Sync + 'static>(
    cache_mgr: Rc<RefCell<CacheManager>>,
    objects: &[ManagedObjectReference],
) -> anyhow::Result<(Box<dyn TableDataSource>, ManagedObjectReference)>
where
    <T as TryFrom<vim_rs::types::structs::ObjectUpdate>>::Error: BoxableError,
    for<'a> Row<'static>: From<&'a T>,
{
    let cache = Arc::new(RwLock::new(ObjectCache::<T>::new()));
    let filter = cache_mgr
        .borrow_mut()
        .add_list_cache(Box::new(ReadWriteCacheProxy::new(cache.clone())), objects)
        .await?;
    let indexed_cache = IndexedCache::new(cache.clone());
    Ok((Box::new(indexed_cache), filter))
}
