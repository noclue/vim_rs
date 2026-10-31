//! Load the contract in solve.rs and print one JSON line.

mod mock;
mod problem_emit;
mod solve;

use std::sync::Arc;

use bytes::Bytes;
use tokio::sync::mpsc;
use vim_rs::core::client::VimClient;
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::structs::ManagedObjectReference;

use mock::service_content::dummy_service_content;
use mock::MockVimClient;

fn vm_morefs_from_env() -> Vec<ManagedObjectReference> {
    let ids = std::env::var("VM_IDS").unwrap_or_else(|_| "vm-101".to_string());
    ids.split(',')
        .filter(|id| !id.trim().is_empty())
        .map(|id| ManagedObjectReference {
            r#type: MoTypesEnum::VirtualMachine,
            value: id.trim().to_string(),
        })
        .collect()
}

fn host_morefs_from_env() -> Vec<ManagedObjectReference> {
    let ids = std::env::var("VM_IDS").unwrap_or_else(|_| "host-201".to_string());
    ids.split(',')
        .filter(|id| !id.trim().is_empty())
        .map(|id| ManagedObjectReference {
            r#type: MoTypesEnum::HostSystem,
            value: id.trim().to_string(),
        })
        .collect()
}

/// `ENTITY_REFS=VirtualMachine:vm-1,ClusterComputeResource:domain-c1`.
fn mixed_morefs_from_env() -> Vec<ManagedObjectReference> {
    let raw = std::env::var("ENTITY_REFS").unwrap_or_default();
    raw.split(',')
        .filter(|item| !item.trim().is_empty())
        .map(|item| {
            let item = item.trim();
            let (kind, value) = item.split_once(':').unwrap_or(("VirtualMachine", item));
            ManagedObjectReference {
                r#type: MoTypesEnum::from_str(kind),
                value: value.to_string(),
            }
        })
        .collect()
}

#[tokio::main]
async fn main() {
    let fixture_path = std::env::var("FIXTURE").expect("FIXTURE env var required");
    let fixture_bytes = Bytes::from(std::fs::read(&fixture_path).expect("read fixture file"));

    let (_tx, rx) = mpsc::unbounded_channel();
    let mock = Arc::new(MockVimClient::new(dummy_service_content(), rx));
    mock.set_retrieve_fixture(fixture_bytes);

    let client: Arc<dyn VimClient> = mock.clone();
    let mo_type = std::env::var("MO_TYPE").unwrap_or_else(|_| "VirtualMachine".to_string());
    let morefs = if mo_type == "HostSystem" {
        host_morefs_from_env()
    } else if mo_type == "Mixed" {
        mixed_morefs_from_env()
    } else {
        vm_morefs_from_env()
    };

    let line = problem_emit::drive(client, mock.as_ref(), &morefs).await;
    println!("{line}");
}
