//! Sealed grade output for pc03. The navigation (array iteration, trait
//! downcast, MAC extraction) is the AGENT's job — implemented in
//! `solve::ethernet_macs`. This sealed emitter only calls it and shapes JSON,
//! so the multi-hop path-navigation skill is what actually gets graded.

use crate::solve::{ethernet_macs, Target};

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
        .map(|o| serde_json::json!({ "mac_addresses": ethernet_macs(o) }))
        .collect();
    serde_json::to_string(&rows).expect("grade json")
}
