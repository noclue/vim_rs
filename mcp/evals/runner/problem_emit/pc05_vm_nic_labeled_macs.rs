//! Sealed grade output for pc05. The navigation — trait downcast, MAC on the
//! subtype, and label from the BASE VirtualDevice.deviceInfo via the supertrait
//! (itself a DescriptionTrait) — is the AGENT's job in `solve::labeled_macs`.
//! This sealed emitter only calls it and shapes JSON.

use crate::solve::{labeled_macs, Target};

pub async fn drive(
    client: std::sync::Arc<dyn vim_rs::core::client::VimClient>,
    _mock: &crate::mock::MockVimClient,
    morefs: &[vim_rs::types::structs::ManagedObjectReference],
) -> String {
    let retriever = vim_rs::core::pc_retrieve::ObjectRetriever::new(client).expect("ObjectRetriever");
    let objs: Vec<Target> = retriever
        .retrieve_objects_from_list(morefs)
        .await
        .expect("retrieve_objects_from_list");
    emit_json(&objs)
}

pub fn emit_json(objs: &[Target]) -> String {
    let rows: Vec<serde_json::Value> = objs
        .iter()
        .map(|o| serde_json::json!({ "labeled_macs": labeled_macs(o) }))
        .collect();
    serde_json::to_string(&rows).expect("grade json")
}
