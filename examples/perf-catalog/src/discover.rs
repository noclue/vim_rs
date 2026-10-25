//! Inventory samples used by the catalog dump and the probe.

use std::collections::BTreeMap;
use std::sync::Arc;

use anyhow::{Context, Result};
use vim_rs::core::client::Client;
use vim_rs::core::pc_retrieve::ObjectRetriever;
use vim_rs::mo::{ContainerView, ViewManager};
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::structs::ManagedObjectReference;
use vim_rs::vim_retrievable;

use crate::session::{clone_moref, moref_key, Session};

pub const SAMPLE_TYPES: &[&str] = &[
    "VirtualMachine",
    "HostSystem",
    "ComputeResource",
    "ClusterComputeResource",
    "ResourcePool",
    "VirtualApp",
    "Datastore",
    "StoragePod",
    "Datacenter",
];

vim_retrievable!(
    struct VmPower: VirtualMachine {
        power_state = "runtime.power_state",
    }
);

vim_retrievable!(
    struct VmQuick: VirtualMachine {
        name = "name",
        power_state = "runtime.power_state",
        overall_cpu_usage = "summary.quick_stats.overall_cpu_usage",
        max_cpu_usage = "summary.runtime.max_cpu_usage",
        guest_memory_usage = "summary.quick_stats.guest_memory_usage",
        host_memory_usage = "summary.quick_stats.host_memory_usage",
    }
);

#[derive(Debug, Clone)]
pub struct Sample {
    pub moref: ManagedObjectReference,
    pub power_state: Option<String>,
}

#[derive(Debug)]
pub struct Discovery {
    pub samples: BTreeMap<String, Vec<Sample>>,
    pub missing_types: Vec<String>,
    pub missing_samples: Vec<String>,
    pub vms: Vec<Sample>,
}

pub async fn discover(session: &Session) -> Result<Discovery> {
    let Some(view_moref) = session.client.service_content().view_manager.clone() else {
        let mut missing_types = SAMPLE_TYPES
            .iter()
            .map(|s| (*s).to_string())
            .collect::<Vec<_>>();
        missing_types.sort();
        return Ok(Discovery {
            samples: BTreeMap::new(),
            missing_types,
            missing_samples: vec!["VirtualMachine:poweredOff".into()],
            vms: Vec::new(),
        });
    };
    let views = ViewManager::new(session.client.clone(), &view_moref.value);
    let root = clone_moref(&session.client.service_content().root_folder);

    let mut samples: BTreeMap<String, Vec<Sample>> = BTreeMap::new();
    let mut missing_types = Vec::new();
    let mut missing_samples = Vec::new();

    for type_name in SAMPLE_TYPES {
        if *type_name == "VirtualMachine" {
            continue;
        }
        match list_exact(&session.client, &views, &root, type_name).await {
            Ok(morefs) => {
                if morefs.is_empty() {
                    missing_types.push((*type_name).to_string());
                    continue;
                }
                let chosen = prefer_moref(&morefs, session.mo_ref.as_ref());
                samples.insert(
                    (*type_name).to_string(),
                    vec![Sample {
                        moref: chosen,
                        power_state: None,
                    }],
                );
            }
            Err(err) => {
                log::warn!("listing {type_name}: {err:#}");
                missing_types.push((*type_name).to_string());
            }
        }
    }

    let vms = match load_vm_power(session).await {
        Ok(vms) => vms,
        Err(err) => {
            log::warn!("listing virtual machines: {err:#}");
            missing_types.push("VirtualMachine".into());
            Vec::new()
        }
    };
    if vms.is_empty() && !missing_types.iter().any(|t| t == "VirtualMachine") {
        missing_types.push("VirtualMachine".into());
    }
    let (vm_samples, vm_missing) = pick_vms(&vms, session.mo_ref.as_ref());
    missing_samples.extend(vm_missing);
    if !vm_samples.is_empty() {
        samples.insert("VirtualMachine".into(), vm_samples);
    }
    missing_types.sort();
    missing_samples.sort();
    Ok(Discovery {
        samples,
        missing_types,
        missing_samples,
        vms,
    })
}

pub async fn vm_quick(session: &Session, moref: &ManagedObjectReference) -> Result<VmQuick> {
    let retriever =
        ObjectRetriever::new(session.client.clone()).context("building property retriever")?;
    let found = retriever
        .retrieve_objects_from_list::<VmQuick>(std::slice::from_ref(moref))
        .await
        .context("retrieving VM quick stats")?;
    found
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("quick stats for {} returned no object", moref_key(moref)))
}

async fn load_vm_power(session: &Session) -> Result<Vec<Sample>> {
    let retriever =
        ObjectRetriever::new(session.client.clone()).context("building property retriever")?;
    let vms = retriever
        .retrieve_objects_from_container::<VmPower>(&session.client.service_content().root_folder)
        .await
        .context("retrieving virtual machine power states")?;
    Ok(vms
        .into_iter()
        .map(|vm| Sample {
            moref: vm.id,
            power_state: Some(vm.power_state.as_str().to_string()),
        })
        .collect())
}

fn pick_vms(
    vms: &[Sample],
    preferred: Option<&ManagedObjectReference>,
) -> (Vec<Sample>, Vec<String>) {
    let mut missing = Vec::new();
    let powered_on = choose_state(vms, "poweredOn", preferred);
    let powered_off = choose_state(vms, "poweredOff", preferred);
    if powered_on.is_none() {
        missing.push("VirtualMachine:poweredOn".into());
    }
    if powered_off.is_none() {
        missing.push("VirtualMachine:poweredOff".into());
    }
    let mut kept = Vec::new();
    if let Some(sample) = powered_on {
        kept.push(sample);
    }
    if let Some(sample) = powered_off {
        kept.push(sample);
    }
    if kept.is_empty() {
        if let Some(sample) = vms.first() {
            kept.push(sample.clone());
        }
    }
    (kept, missing)
}

fn choose_state(
    vms: &[Sample],
    state: &str,
    preferred: Option<&ManagedObjectReference>,
) -> Option<Sample> {
    let matching: Vec<&Sample> = vms
        .iter()
        .filter(|vm| vm.power_state.as_deref() == Some(state))
        .collect();
    if let Some(preferred) = preferred {
        if let Some(found) = matching.iter().find(|vm| same_moref(&vm.moref, preferred)) {
            return Some((*found).clone());
        }
    }
    matching.first().map(|sample| (*sample).clone())
}

fn prefer_moref(
    morefs: &[ManagedObjectReference],
    preferred: Option<&ManagedObjectReference>,
) -> ManagedObjectReference {
    if let Some(preferred) = preferred {
        if let Some(found) = morefs.iter().find(|moref| same_moref(moref, preferred)) {
            return clone_moref(found);
        }
    }
    clone_moref(&morefs[0])
}

fn same_moref(left: &ManagedObjectReference, right: &ManagedObjectReference) -> bool {
    left.value == right.value && left.r#type.as_str() == right.r#type.as_str()
}

async fn list_exact(
    client: &Arc<Client>,
    views: &ViewManager,
    root: &ManagedObjectReference,
    type_name: &str,
) -> Result<Vec<ManagedObjectReference>> {
    let type_string = type_name.to_string();
    let view_moref = views
        .create_container_view(root, Some(&[type_string]), true)
        .await
        .with_context(|| format!("CreateContainerView {type_name}"))?;
    let view = ContainerView::new(client.clone(), &view_moref.value);
    let listed = view.view().await;
    let _ = view.destroy_view().await;
    let listed = listed.with_context(|| format!("reading {type_name} view"))?;
    let wanted = MoTypesEnum::from_str(type_name);
    Ok(listed
        .unwrap_or_default()
        .into_iter()
        .filter(|moref| moref.r#type.as_str() == wanted.as_str())
        .collect())
}
