use std::time::SystemTime;

use serde_json::json;
use vim_rs::core::client::VimClient;
use vim_rs::perf::PerfClient;
use vim_rs::types::structs::ManagedObjectReference;

pub async fn drive(
    client: std::sync::Arc<dyn VimClient>,
    mock: &crate::mock::MockVimClient,
    morefs: &[ManagedObjectReference],
) -> String {
    let perf = PerfClient::new(client).expect("perf client");
    let ids: Vec<&str> = morefs.iter().map(|moref| moref.value.as_str()).collect();
    let query = crate::solve::solve(perf, &ids).await;
    let _ = query.fetch().await;
    summarize(&mock.recorded_queries())
}

fn summarize(recorded: &[crate::mock::RecordedQuery]) -> String {
    let mut counters: Vec<i32> = recorded
        .iter()
        .flat_map(|query| query.metrics.iter().map(|(id, _)| *id))
        .collect();
    counters.sort_unstable();
    counters.dedup();
    let mut entities: Vec<&str> = recorded.iter().map(|query| query.entity.as_str()).collect();
    entities.sort_unstable();
    entities.dedup();
    let interval = recorded.first().and_then(|query| query.interval_id);
    let window = recorded
        .first()
        .and_then(|query| span_secs(query.start, query.end));
    let consistent = recorded.iter().all(|query| {
        let mut ids: Vec<i32> = query.metrics.iter().map(|(id, _)| *id).collect();
        ids.sort_unstable();
        ids.dedup();
        ids == counters
            && query.interval_id == interval
            && span_secs(query.start, query.end) == window
            && query.entity_type == "VirtualMachine"
    });
    if !consistent {
        return json!({
            "counters": [],
            "entities": [],
            "interval": null,
            "window_secs": null
        })
        .to_string();
    }
    json!({
        "counters": counters,
        "entities": entities,
        "interval": interval,
        "window_secs": window
    })
    .to_string()
}

fn span_secs(start: Option<SystemTime>, end: Option<SystemTime>) -> Option<u64> {
    let (start, end) = (start?, end?);
    end.duration_since(start).ok().map(|span| span.as_secs())
}
