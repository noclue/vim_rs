//! Build one performance query for these virtual machines. Include how much
//! electricity this virtual machine is using, and how long it sits waiting
//! for a turn on the processor even though the processor chart is not full.
//! Sample every 5 minutes across the past day.
//!
//! Interval::PAST_DAY. A typed metric or a group.name.rollup string are both
//! fine. Return the query. Do not fetch it.

use vim_rs::perf::{PerfClient, PerfQuery};

pub async fn solve(client: PerfClient, vm_ids: &[&str]) -> PerfQuery {
    let _ = vm_ids;
    client.query()
}
