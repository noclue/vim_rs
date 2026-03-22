//! # Inventory path in one PropertyCollector round-trip (`MO_REF`)
//!
//! Resolves an inventory path using the same **batched** PropertyCollector pattern as govmomi
//! [`mo.Ancestors`](https://github.com/vmware/govmomi/blob/main/vim25/mo/ancestors.go):
//! traverse `ManagedEntity.parent` (named `traverseParent`, recursive) plus `VirtualMachine.parentVApp`
//! into that traversal, and collect `ManagedEntity` `name`/`parent` with `VirtualMachine` `parentVApp`
//! in a **single** `RetrievePropertiesEx` (plus continuation tokens if needed). Then walk the parent
//! chain in memory—**one** server round-trip instead of one per ancestor level.
//!
//! Path assembly matches govmomi [`internal.InventoryPath`](https://github.com/vmware/govmomi/blob/57d3dfb60c6e172824db4eae6c263c28b9053a12/internal/helpers.go#L35):
//! names are joined with `/` (leading `/`); entities with **no parent** (inventory root, e.g. ESXi’s top folder)
//! are omitted; the hidden vCenter root folder `Folder:group-d1` stops the walk when encountered.
//!
//! ## Environment
//!
//! Uses the shared `snippets::connect` helper: `VIM_SERVER`, `VIM_USERNAME`, `VIM_PASSWORD`, optional `VIM_PROTOCOL`.
//! **`MO_REF`** — managed object id as `Type:id` (split on the **first** `:` only). Example: `VirtualMachine:vm-42`.

use std::collections::HashMap;
use std::env;
use std::sync::Arc;

use anyhow::{Context, Result};
use log::info;
use snippets::connect;
use vim_rs::mo::PropertyCollector;
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::boxed_types::ValueElements;
use vim_rs::types::structs::{
    ManagedObjectReference, ObjectContent, ObjectSpec, PropertyFilterSpec, PropertySpec,
    RetrieveOptions, SelectionSpec, TraversalSpec,
};
use vim_rs::types::traits::SelectionSpecTrait;
use vim_rs::types::vim_any::VimAny;

// Parse `TYPE:ID` using only the first colon (IDs may contain `:`).
fn parse_mo_ref(raw: &str) -> Result<ManagedObjectReference> {
    let raw = raw.trim();
    let (type_part, value_part) = raw
        .split_once(':')
        .context("MO_REF must contain a ':' (e.g. 'VirtualMachine:vm-41')")?;
    let t = type_part.trim();
    let v = value_part.trim();
    anyhow::ensure!(!t.is_empty(), "MO_REF type segment (before ':') must not be empty");
    anyhow::ensure!(!v.is_empty(), "MO_REF id segment (after first ':') must not be empty");
    Ok(ManagedObjectReference {
        r#type: MoTypesEnum::from_str(t),
        value: v.to_string(),
    })
}

fn is_hidden_root_folder(mor: &ManagedObjectReference) -> bool {
    mor.r#type == MoTypesEnum::Folder && mor.value == "group-d1"
}

/// One managed object returned by the batched ancestor PropertyCollector retrieve.
#[derive(Clone, Debug)]
struct AncestorParsed {
    pub mor: ManagedObjectReference,
    pub name: String,
    /// After applying VirtualMachine `parentVApp` when `parent` is unset (govmomi `Ancestors`).
    pub parent_resolved: Option<ManagedObjectReference>,
}

/// Build path like govmomi `internal.InventoryPath`: skip entities with no parent (inventory root),
/// join names with `/`, leading `/`. `leaf_to_root` is ordered from the start object up toward the root.
fn inventory_path_govmomi_style(leaf_to_root: &[AncestorParsed]) -> String {
    let mut parts = Vec::new();
    for row in leaf_to_root.iter().rev() {
        if row.parent_resolved.is_none() {
            continue;
        }
        parts.push(row.name.as_str());
    }
    if parts.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", parts.join("/"))
    }
}

/// Same `ObjectSpec` / `PropSet` shape as govmomi `mo.Ancestors`.
fn ancestors_filter_spec(start: ManagedObjectReference) -> PropertyFilterSpec {
    let traverse_name = "traverseParent";
    let traverse_parent_ref: Box<dyn SelectionSpecTrait> = Box::new(SelectionSpec {
        name: Some(traverse_name.to_string()),
    });

    PropertyFilterSpec {
        object_set: vec![ObjectSpec {
            obj: start,
            skip: Some(false),
            select_set: Some(vec![
                Box::new(TraversalSpec {
                    selection_spec_: SelectionSpec {
                        name: Some(traverse_name.to_string()),
                    },
                    r#type: "ManagedEntity".to_string(),
                    path: "parent".to_string(),
                    skip: Some(false),
                    select_set: Some(vec![Box::new(SelectionSpec {
                        name: Some(traverse_name.to_string()),
                    })]),
                }),
                Box::new(TraversalSpec {
                    selection_spec_: SelectionSpec { name: None },
                    r#type: "VirtualMachine".to_string(),
                    path: "parentVApp".to_string(),
                    skip: Some(false),
                    select_set: Some(vec![traverse_parent_ref]),
                }),
            ]),
        }],
        prop_set: vec![
            PropertySpec {
                r#type: "ManagedEntity".to_string(),
                all: Some(false),
                path_set: Some(vec!["name".to_string(), "parent".to_string()]),
            },
            PropertySpec {
                r#type: "VirtualMachine".to_string(),
                all: Some(false),
                path_set: Some(vec!["parentVApp".to_string()]),
            },
        ],
        report_missing_objects_in_results: Some(true),
    }
}

fn parse_dynamic_parent(val: &VimAny) -> Result<Option<ManagedObjectReference>> {
    match val {
        VimAny::Object(obj) => {
            // `as_any_ref` on `&Box<dyn VimObjectTrait>` uses `Any` for the box, not the pointee;
            // dispatch through `dyn VimObjectTrait` (same idea as `vim_retrievable!`'s
            // `as_any_box().downcast()`).
            let mor = obj
                .as_ref()
                .as_any_ref()
                .downcast_ref::<ManagedObjectReference>()
                .context("expected ManagedObjectReference for parent / parentVApp")?;
            Ok(Some(mor.clone()))
        }
        VimAny::Value(ValueElements::ArrayOfManagedObjectReference(refs)) if refs.len() == 1 => {
            Ok(Some(refs[0].clone()))
        }
        VimAny::Value(_) => Ok(None),
    }
}

fn parse_object_content(oc: &ObjectContent) -> Result<AncestorParsed> {
    let mor = oc.obj.clone();
    let mut name: Option<String> = None;
    let mut parent: Option<ManagedObjectReference> = None;
    let mut parent_vapp: Option<ManagedObjectReference> = None;

    let props = oc.prop_set.as_ref().context(format!(
        "no prop_set for {}:{}",
        mor.r#type.as_str(),
        mor.value
    ))?;
    for p in props {
        match p.name.as_str() {
            "name" => {
                if let VimAny::Value(ValueElements::PrimitiveString(s)) = &p.val {
                    name = Some(s.clone());
                }
            }
            "parent" => {
                parent = parse_dynamic_parent(&p.val)?;
            }
            "parentVApp" => {
                parent_vapp = parse_dynamic_parent(&p.val)?;
            }
            _ => {}
        }
    }

    let name = name.unwrap_or_default();
    let mut parent_resolved = parent;
    if parent_resolved.is_none() {
        parent_resolved = parent_vapp;
    }

    Ok(AncestorParsed {
        mor,
        name,
        parent_resolved,
    })
}

async fn retrieve_ancestors_contents(
    client: &Arc<vim_rs::core::client::Client>,
    start: ManagedObjectReference,
) -> Result<Vec<ObjectContent>> {
    let spec_set = vec![ancestors_filter_spec(start)];
    let options = RetrieveOptions {
        max_objects: Some(256),
    };
    let pc_id = client.service_content().property_collector.value.clone();
    let pc = PropertyCollector::new(client.clone(), &pc_id);

    let mut collected = Vec::new();
    let mut res = pc
        .retrieve_properties_ex(&spec_set, &options)
        .await?
        .context("RetrievePropertiesEx returned None")?;

    loop {
        collected.extend(std::mem::take(&mut res.objects));
        let Some(token) = res.token else {
            break;
        };
        res = pc.continue_retrieve_properties_ex(&token).await?;
    }

    Ok(collected)
}

fn build_map(contents: Vec<ObjectContent>) -> Result<HashMap<ManagedObjectReference, AncestorParsed>> {
    let mut map = HashMap::with_capacity(contents.len());
    for oc in contents {
        let row = parse_object_content(&oc)?;
        map.insert(row.mor.clone(), row);
    }
    Ok(map)
}

/// Walk from `start` toward the root using resolved parents (stops at hidden root or missing parent in the map).
fn leaf_to_root_chain(
    map: &HashMap<ManagedObjectReference, AncestorParsed>,
    start: &ManagedObjectReference,
) -> Result<Vec<AncestorParsed>> {
    let mut chain = Vec::new();
    let mut cur = start.clone();
    loop {
        let row = map
            .get(&cur)
            .with_context(|| {
                format!(
                    "batched retrieve did not include {}:{} (cannot complete ancestry)",
                    cur.r#type.as_str(),
                    cur.value
                )
            })?
            .clone();
        let next_parent = row.parent_resolved.clone();
        chain.push(row);
        let Some(p) = next_parent else {
            break;
        };
        if is_hidden_root_folder(&p) {
            break;
        }
        if !map.contains_key(&p) {
            break;
        }
        cur = p;
    }
    Ok(chain)
}

async fn inventory_path_batch(
    client: Arc<vim_rs::core::client::Client>,
    start: ManagedObjectReference,
) -> Result<String> {
    let contents = retrieve_ancestors_contents(&client, start.clone()).await?;
    let map = build_map(contents)?;
    let chain = leaf_to_root_chain(&map, &start)?;
    Ok(inventory_path_govmomi_style(&chain))
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let mo_ref_str = env::var("MO_REF").context(
        "MO_REF env var not set (e.g. 'VirtualMachine:vm-41' — split on the first ':' only)",
    )?;
    let start = parse_mo_ref(&mo_ref_str)?;

    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;
    info!("Connected to {}", client.service_content().about.full_name);

    let path = inventory_path_batch(client, start.clone()).await?;
    info!(
        "Inventory path of {}:{}: {}",
        start.r#type.as_str(),
        start.value,
        path
    );
    println!("{}", path);
    Ok(())
}
