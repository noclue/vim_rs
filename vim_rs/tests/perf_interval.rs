#![cfg(feature = "perf")]

//! Interval decision table. No `QueryPerf` calls.

use std::time::{Duration, SystemTime};

use vim_rs::perf::error::{PerfError, PerfWarning, UnsupportedReason};
use vim_rs::perf::query::{plan_intervals, EntityDecision, Interval, IntervalCase};
use vim_rs::perf::registry::ProviderCaps;
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::structs::{ManagedObjectReference, PerfInterval};

fn moref(kind: MoTypesEnum, value: &str) -> ManagedObjectReference {
    ManagedObjectReference {
        r#type: kind,
        value: value.to_string(),
    }
}

fn caps(current: bool, summary: bool, refresh: Option<i32>) -> ProviderCaps {
    ProviderCaps {
        current_supported: current,
        summary_supported: summary,
        refresh_rate: refresh,
    }
}

fn vcenter_intervals() -> Vec<PerfInterval> {
    vec![
        interval(1, 300, 86400, Some(2)),
        interval(2, 1800, 604800, Some(1)),
        interval(3, 7200, 2592000, Some(1)),
        interval(4, 86400, 31536000, Some(1)),
    ]
}

fn interval(key: i32, sampling_period: i32, length: i32, level: Option<i32>) -> PerfInterval {
    PerfInterval {
        key,
        sampling_period,
        name: format!("period-{sampling_period}"),
        length,
        level,
        enabled: true,
    }
}

fn case(entity: ManagedObjectReference, provider: ProviderCaps) -> IntervalCase {
    IntervalCase {
        entity,
        caps: provider,
        start: None,
        end: None,
        incremental: false,
    }
}

fn planned_query(
    planned: &vim_rs::perf::query::PlannedEntity,
) -> &vim_rs::perf::query::PlannedQuery {
    match &planned.decision {
        EntityDecision::Query(query) => query,
        EntityDecision::Unsupported(reason) => panic!("expected a query, got {reason:?}"),
    }
}

#[test]
fn realtime_leaves_bounds_unset() {
    let now = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let vm = moref(MoTypesEnum::VirtualMachine, "vm-1");
    let planned = plan_intervals(
        &[case(vm, caps(true, true, Some(20)))],
        &vcenter_intervals(),
        Interval::Realtime,
        None,
        now,
    )
    .unwrap();
    let query = planned_query(&planned[0]);
    assert_eq!(query.interval_id, 20);
    assert!(query.start.is_none());
    assert!(query.end.is_none());
    assert!(query.max_sample.is_none());
    assert!(query.warnings.is_empty());
}

#[test]
fn realtime_start_older_than_an_hour_still_queries() {
    let now = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let mut item = case(
        moref(MoTypesEnum::VirtualMachine, "vm-1"),
        caps(true, true, Some(20)),
    );
    item.start = Some(now - Duration::from_secs(3601));
    let planned =
        plan_intervals(&[item], &vcenter_intervals(), Interval::Realtime, None, now).unwrap();
    let query = planned_query(&planned[0]);
    assert_eq!(query.interval_id, 20);
    assert!(query
        .warnings
        .iter()
        .any(|warning| matches!(warning, PerfWarning::RealtimeWindowTooOld { .. })));

    let mut exact = case(
        moref(MoTypesEnum::VirtualMachine, "vm-1"),
        caps(true, true, Some(20)),
    );
    exact.start = Some(now - Duration::from_secs(3600));
    let planned = plan_intervals(
        &[exact],
        &vcenter_intervals(),
        Interval::Realtime,
        None,
        now,
    )
    .unwrap();
    assert!(planned_query(&planned[0]).warnings.is_empty());
}

#[test]
fn past_day_uses_retention_and_omits_max_sample() {
    let now = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let planned = plan_intervals(
        &[case(
            moref(MoTypesEnum::VirtualMachine, "vm-1"),
            caps(true, true, Some(20)),
        )],
        &vcenter_intervals(),
        Interval::PAST_DAY,
        None,
        now,
    )
    .unwrap();
    let query = planned_query(&planned[0]);
    assert_eq!(query.interval_id, 300);
    assert_eq!(query.start, Some(now - Duration::from_secs(86400)));
    assert_eq!(query.end, Some(now));
    assert!(query.max_sample.is_none());
}

#[test]
fn historical_latest_uses_two_period_cushion() {
    let now = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let planned = plan_intervals(
        &[case(
            moref(MoTypesEnum::VirtualMachine, "vm-1"),
            caps(true, true, Some(20)),
        )],
        &vcenter_intervals(),
        Interval::PAST_DAY,
        Some(5),
        now,
    )
    .unwrap();
    let query = planned_query(&planned[0]);
    assert_eq!(query.start, Some(now - Duration::from_secs(7 * 300)));
    assert_eq!(query.end, Some(now));
    assert!(query.max_sample.is_none());
    assert_eq!(query.truncate_newest, Some(5));
}

#[test]
fn unknown_period_is_invalid_interval() {
    let now = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let err = plan_intervals(
        &[case(
            moref(MoTypesEnum::VirtualMachine, "vm-1"),
            caps(true, true, Some(20)),
        )],
        &vcenter_intervals(),
        Interval::Historical(999),
        None,
        now,
    )
    .unwrap_err();
    assert!(matches!(err, PerfError::InvalidInterval(999)));
}

#[test]
fn empty_interval_list_is_historical_unsupported() {
    let now = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let err = plan_intervals(
        &[case(
            moref(MoTypesEnum::VirtualMachine, "vm-1"),
            caps(true, false, Some(20)),
        )],
        &[],
        Interval::PAST_DAY,
        None,
        now,
    )
    .unwrap_err();
    assert!(matches!(err, PerfError::NoSupportedEntities));
}

#[test]
fn all_skipped_entities_are_no_supported_entities() {
    let now = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let err = plan_intervals(
        &[case(
            moref(MoTypesEnum::Datastore, "datastore-1"),
            caps(false, false, None),
        )],
        &vcenter_intervals(),
        Interval::Realtime,
        None,
        now,
    )
    .unwrap_err();
    assert!(matches!(err, PerfError::NoSupportedEntities));
}

#[test]
fn automatic_vm_is_realtime_and_cluster_falls_back() {
    let now = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let planned = plan_intervals(
        &[
            case(
                moref(MoTypesEnum::VirtualMachine, "vm-1"),
                caps(true, true, Some(20)),
            ),
            case(
                moref(MoTypesEnum::ClusterComputeResource, "domain-c1"),
                caps(false, true, None),
            ),
        ],
        &vcenter_intervals(),
        Interval::Auto,
        None,
        now,
    )
    .unwrap();
    assert_eq!(planned_query(&planned[0]).interval_id, 20);
    assert!(planned_query(&planned[0]).realtime);
    assert!(planned_query(&planned[0]).warnings.is_empty());
    assert_eq!(planned_query(&planned[1]).interval_id, 300);
    assert!(planned_query(&planned[1])
        .warnings
        .iter()
        .any(|warning| matches!(
            warning,
            PerfWarning::AutoIntervalFallback {
                interval_secs: 300,
                entity_kind: MoTypesEnum::ClusterComputeResource
            }
        )));
    assert!(matches!(
        plan_intervals(
            &[case(
                moref(MoTypesEnum::VirtualMachine, "vm-1"),
                caps(false, false, None)
            )],
            &[],
            Interval::Realtime,
            None,
            now,
        )
        .unwrap_err(),
        PerfError::NoSupportedEntities
    ));
    let historical = plan_intervals(
        &[case(
            moref(MoTypesEnum::HostSystem, "host-1"),
            caps(false, true, None),
        )],
        &vcenter_intervals(),
        Interval::Realtime,
        None,
        now,
    )
    .unwrap_err();
    assert!(matches!(historical, PerfError::NoSupportedEntities));
    let _ = UnsupportedReason::HistoricalUnsupported;
}
