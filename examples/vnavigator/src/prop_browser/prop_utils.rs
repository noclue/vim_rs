use indexmap::IndexMap;
use serde_json::Value;
use tui_tree_widget::TreeItem;
use std::sync::Arc;
use vim_rs::core::client::Client;
use vim_rs::types::structs::{DynamicProperty, ManagedObjectReference, ObjectSpec, PropertyFilterSpec, PropertySpec};
use vim_rs::mo::PropertyCollector;
use vim_rs::types::vim_any::VimAny;
use super::json_to_tree::property_to_tree_item;

pub async fn load_props(
    client: Arc<Client>,
    mo_ref: &ManagedObjectReference,
) -> anyhow::Result<Vec<DynamicProperty>> {
    let pc_moref = &client.service_content().property_collector.value;
    let pc = PropertyCollector::new(client.clone(), pc_moref);
    let s: &'static str = From::from(&mo_ref.r#type);
    let spec = PropertyFilterSpec {
        prop_set: vec![PropertySpec{
            r#type: s.to_string(),
            all: Some(true),
            path_set: None,
        }],
        object_set: vec![ObjectSpec{
            obj: mo_ref.clone(),
            skip: Some(false),
            select_set: None,
        }],
        report_missing_objects_in_results: None,
    };
    let Some(mut props) = pc.retrieve_properties(&[spec]).await? else {
        return Err(anyhow::anyhow!("No properties found"));
    };

    if props.is_empty() {
        return Err(anyhow::anyhow!("No properties found for object {:?}", mo_ref));
    }

    let obj = props.remove(0);

    let Some(prop_set) = obj.prop_set else {
        return Err(anyhow::anyhow!("No prop_set found for object {:?}", mo_ref));
    };

    Ok(prop_set)
}

pub fn props_to_map<'a>(prop_set: &'a Vec<DynamicProperty>) -> anyhow::Result<IndexMap<String, Value>> {
    let mut nodes = IndexMap::new();
    for property in prop_set {
        let name = property.name.clone();
        let value = &property.val;
        let json = to_json_value(value, &name)?;
        nodes.insert(name, json);
    }
    Ok(nodes)
}

pub fn to_json_value(value: &VimAny, name: &String) -> anyhow::Result<Value> {
    Ok(match value {
        VimAny::Value(value) => {
            let json_val = serde_json::to_value(value).map_err(|e| anyhow::anyhow!("Failed to convert value to JSON: {}", e))?;
            match json_val {
                Value::Object(mut obj) => {
                    let Some(value) = obj.remove("_value") else {
                        return Err(anyhow::anyhow!("Expected JSON object with _value field for property {}", name));
                    };
                    value
                }
                _ => {
                    return Err(anyhow::anyhow!("Expected JSON object for property '{}', got {:?}", name, json_val));
                }
            }
        }
        VimAny::Object(obj) => {
            serde_json::to_value(obj).map_err(|e| anyhow::anyhow!("Failed to convert property '{}' object to JSON: {}", name, e))?
        }
    })
}

pub fn map_to_tree(nodes: &IndexMap<String, Value>) -> Vec<TreeItem<'static, String>> {
    nodes.iter().map(|item| {
        let name = item.0.clone();
        let json = item.1;
        property_to_tree_item(name, json)
    }).collect()
}