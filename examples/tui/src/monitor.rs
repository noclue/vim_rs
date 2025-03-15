use std::sync::Arc;
use vim_rs::core::client::Client;
use vim_rs::mo::{PropertyCollector, PropertyFilter, View, ViewManager};
use vim_rs::types::structs::{ManagedObjectReference, PropertySpec, PropertyFilterSpec, WaitOptions, ObjectSpec, TraversalSpec, PropertyFilterUpdate, ObjectUpdate};
use anyhow::{anyhow, Result};
use vim_rs::types::enums::MoTypesEnum;

pub struct Monitor {
    client: Arc<Client>,
    property_collector: PropertyCollector,
    view_moref: ManagedObjectReference,
    filter_moref: ManagedObjectReference,
    version: String,
}

impl Monitor {
    pub async fn new(client: Arc<Client>, root: ManagedObjectReference, props: PropertySpec) -> Result<Self> {
        let content = client.service_content();
        let property_collector = PropertyCollector::new(client.clone(), &content.property_collector.value);
        let Some(ref view_mgr_moref) = content.view_manager else {
            return Err(anyhow!("No view manager found."));
        };
        let view_manager = ViewManager::new(client.clone(), &view_mgr_moref.value);
        let view_moref = view_manager.create_container_view(
            &root,
            Some(&[props.r#type.clone()]),
            true,
        ).await?;

        let spec = PropertyFilterSpec {
            object_set: obj_spec_for_view(view_moref.clone()),
            prop_set: vec![props],
            report_missing_objects_in_results: Some(true),
        };
        let filter = property_collector.create_filter(&spec, false).await?;

        Ok(Self {
            client,
            property_collector,
            view_moref,
            filter_moref: filter,
            version: "".to_string(),
        })
    }

    pub async fn wait_updates(&mut self, delay_s: i32) -> Result<Option<Vec<ObjectUpdate>>>
    {
        let options = WaitOptions {
            max_wait_seconds: Some(delay_s),
            max_object_updates: Some(100), // TODO customizable???
        };
        let future = self.property_collector.wait_for_updates_ex(Some(&self.version), Some(&options));
        let result = future.await?;
        let Some(update_set) = result else {
            return Ok(None);
        };
        self.version = update_set.version.clone();

        let Some(filter_updates) = update_set.filter_set else {
            return Ok(None);
        };
        let filter_update = get_filter_update(filter_updates, &self.filter_moref);
        Ok(filter_update)
    }
}

impl Drop for Monitor {
    fn drop(&mut self) {
        let view_id = self.view_moref.value.clone();
        let filter_id = self.filter_moref.value.clone();
        let client = self.client.clone();
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async move {
                let view = View::new(client.clone(), &view_id);
                if let Err(e) = view.destroy_view().await {
                    println!("Error destroying view {}: {:?}", view_id, e);
                };
                let filter = PropertyFilter::new(client.clone(), &filter_id);
                if let Err(e) = filter.destroy_property_filter().await {
                    println!("Error destroying property filter {}: {:?}", filter_id, e);
                };
            })
        });
    }
}

/// Find the object updates that match the given filter.
fn get_filter_update(updates: Vec<PropertyFilterUpdate>, filter: &ManagedObjectReference) -> Option<Vec<ObjectUpdate>> {
    for update in updates {
        if update.filter == *filter {
            if let Some(update) = update.object_set {
                return Some(update);
            } else {
                return None;
            }
        }
    }
    None
}

type StaticStr = &'static str;

pub fn obj_spec_for_view(view_moref: ManagedObjectReference) -> Vec<ObjectSpec> {
    vec![ObjectSpec {
        obj: view_moref.clone(),
        skip: Some(false),
        select_set: Some(vec![Box::new(TraversalSpec {
            name: Some("traverseEntities".to_string()),
            r#type: StaticStr::from(MoTypesEnum::ContainerView).to_string(),
            path: "view".to_string(),
            skip: Some(false),
            select_set: None,
        })]),
    }]
}

