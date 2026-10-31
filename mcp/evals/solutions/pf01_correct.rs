use std::time::Duration;

use vim_rs::perf::metrics::cpu::COSTOP_SUMMATION;
use vim_rs::perf::metrics::mem::VMMEMCTL_AVERAGE;
use vim_rs::perf::{Interval, PerfClient, PerfQuery};
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::structs::ManagedObjectReference;

pub async fn solve(client: PerfClient, vm_ids: &[&str]) -> PerfQuery {
    let entities: Vec<ManagedObjectReference> = vm_ids
        .iter()
        .map(|id| ManagedObjectReference {
            r#type: MoTypesEnum::VirtualMachine,
            value: (*id).to_string(),
        })
        .collect();
    client
        .query()
        .interval(Interval::PAST_DAY)
        .since_ago(Duration::from_secs(24 * 60 * 60))
        .metric(VMMEMCTL_AVERAGE)
        .metric(COSTOP_SUMMATION)
        .entities(&entities)
}
