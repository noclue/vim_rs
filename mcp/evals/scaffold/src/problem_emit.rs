//! Formats retrieved objects as one JSON line.
//! Default: pc01 field layout.

use vim_rs::types::enums::VirtualMachinePowerStateEnum;

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
                "power_state": power_state_str(&o.power_state),
                "guest_host": o.guest_host,
            })
        })
        .collect();
    serde_json::to_string(&rows).expect("grade json")
}

fn power_state_str(s: &VirtualMachinePowerStateEnum) -> String {
    match s {
        VirtualMachinePowerStateEnum::PoweredOff => "poweredOff".to_string(),
        VirtualMachinePowerStateEnum::PoweredOn => "poweredOn".to_string(),
        VirtualMachinePowerStateEnum::Suspended => "suspended".to_string(),
        VirtualMachinePowerStateEnum::Other_(v) => v.clone(),
    }
}
