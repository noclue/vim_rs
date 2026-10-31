use vim_rs::core::client::VimClient;
use vim_rs::perf::PerfClient;
use vim_rs::types::structs::ManagedObjectReference;

pub async fn drive(
    client: std::sync::Arc<dyn VimClient>,
    _mock: &crate::mock::MockVimClient,
    morefs: &[ManagedObjectReference],
) -> String {
    let perf = PerfClient::new(client).expect("perf");
    let ids: Vec<&str> = morefs.iter().map(|moref| moref.value.as_str()).collect();
    let _query = crate::solve::solve(perf, &ids).await;
    "[]".to_string()
}
