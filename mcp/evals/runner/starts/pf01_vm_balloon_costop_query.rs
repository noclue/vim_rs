//! Build one performance query for these virtual machines. Include the guest
//! memory the balloon driver has already reclaimed, and the time a
//! multi-processor virtual machine spends stalled because its processors were
//! not scheduled as a group. Sample every 5 minutes across the past day.
//!
//! Interval::PAST_DAY. A typed metric or a group.name.rollup string are both
//! fine. Return the query. Do not fetch it.

use vim_rs::perf::{PerfClient, PerfQuery};

pub async fn solve(client: PerfClient, vm_ids: &[&str]) -> PerfQuery {
    let _ = vm_ids;
    client.query()
}
