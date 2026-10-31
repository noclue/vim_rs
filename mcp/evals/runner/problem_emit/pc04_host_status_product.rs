use vim_rs::types::enums::ManagedEntityStatusEnum;

use crate::solve::Target;

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
        .map(|o| {
            serde_json::json!({
                "overall_status": status_str(&o.overall_status),
                "product_name": o.product_name,
            })
        })
        .collect();
    serde_json::to_string(&rows).expect("grade json")
}

fn status_str(s: &ManagedEntityStatusEnum) -> String {
    match s {
        ManagedEntityStatusEnum::Gray => "gray".to_string(),
        ManagedEntityStatusEnum::Green => "green".to_string(),
        ManagedEntityStatusEnum::Yellow => "yellow".to_string(),
        ManagedEntityStatusEnum::Red => "red".to_string(),
        ManagedEntityStatusEnum::Other_(v) => v.clone(),
    }
}
