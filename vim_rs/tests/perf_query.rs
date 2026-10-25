#![cfg(feature = "perf")]

//! Query packing, format, faults, availability, and clocks against `PerfMockClient`.

use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use vim_rs::core::client::{Error as ClientError, Transport, VimClientHandle};
use vim_rs::perf::{
    EntityStatus, HostOnly, Interval, Metric, PerfClient, PerfError, PerfWarning, Ratio, Rollup,
    Timestamp, UnsupportedReason,
};
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::struct_enum::StructType;
use vim_rs::types::structs::ManagedObjectReference;

#[path = "support/perf_mock.rs"]
mod perf_mock;

use perf_mock::{vcenter_counters, PerfMockClient};

fn vm(id: &str) -> ManagedObjectReference {
    ManagedObjectReference {
        r#type: MoTypesEnum::VirtualMachine,
        value: id.to_string(),
    }
}

fn datastore(id: &str) -> ManagedObjectReference {
    ManagedObjectReference {
        r#type: MoTypesEnum::Datastore,
        value: id.to_string(),
    }
}

fn perf(mock: &Arc<PerfMockClient>) -> PerfClient {
    let handle: VimClientHandle = mock.clone();
    PerfClient::new(handle).unwrap()
}

fn pin(perf: &PerfClient, mock: &PerfMockClient, now: SystemTime) {
    mock.set_clock(now);
    perf.set_clock(now);
}

fn ratio(
    group: &'static str,
    name: &'static str,
    rollup: Rollup,
) -> Metric<Ratio, vim_rs::perf::AnyEntity> {
    Metric::new(group, name, rollup, None, None, None, false)
}

#[tokio::test]
async fn missing_perf_manager_is_an_error() {
    let mock = PerfMockClient::without_perf_manager();
    let handle: VimClientHandle = mock;
    match PerfClient::new(handle) {
        Err(PerfError::NoPerformanceManager) => {}
        Err(other) => panic!("expected NoPerformanceManager, got {other}"),
        Ok(_) => panic!("expected NoPerformanceManager"),
    }
}

#[tokio::test]
async fn realtime_refresh_rate_is_cached_per_kind() {
    let mock = PerfMockClient::new();
    let perf = perf(&mock);
    let rate = perf.realtime_refresh_rate(&vm("vm-1")).await.unwrap();
    assert_eq!(rate, Duration::from_secs(20));
    let again = perf.realtime_refresh_rate(&vm("vm-2")).await.unwrap();
    assert_eq!(again, Duration::from_secs(20));
    assert_eq!(mock.summary_calls().len(), 1);

    mock.set_summary_provider("ds-1", false, false, None);
    let err = perf
        .realtime_refresh_rate(&datastore("ds-1"))
        .await
        .unwrap_err();
    assert!(matches!(err, PerfError::NoSupportedEntities));
}

#[tokio::test]
async fn packs_one_hundred_vms_into_two_calls() {
    let mock = PerfMockClient::new();
    let perf = perf(&mock);
    let vms: Vec<_> = (0..100).map(|i| vm(&format!("vm-{i}"))).collect();
    perf.query()
        .entities(&vms)
        .metrics_by_name(&[
            "cpu.usage.average",
            "cpu.usage.maximum",
            "cpu.ready.summation",
            "mem.usage.average",
        ])
        .interval(Interval::Realtime)
        .latest(2)
        .fetch()
        .await
        .unwrap();
    let mut counts: Vec<_> = mock.queries().iter().map(|call| call.len()).collect();
    counts.sort_unstable();
    assert_eq!(counts, vec![36, 64]);
}

#[tokio::test]
async fn one_object_splits_into_ceil_of_chunk() {
    let mock = PerfMockClient::new();
    let perf = perf(&mock);
    perf.query()
        .entity(&vm("vm-1"))
        .metrics_by_name(&[
            "cpu.usage.average",
            "cpu.usage.maximum",
            "cpu.ready.summation",
            "mem.usage.average",
            "mem.consumed.average",
        ])
        .interval(Interval::Realtime)
        .latest(1)
        .chunk_size(2)
        .fetch()
        .await
        .unwrap();
    let mut sizes: Vec<_> = mock
        .queries()
        .iter()
        .flat_map(|call| call.iter().map(|spec| spec.counter_ids.len()))
        .collect();
    sizes.sort_unstable();
    assert_eq!(sizes, vec![1, 2, 2]);
}

#[tokio::test]
async fn auto_format_is_csv_for_json_and_normal_for_latest_one() {
    let mock = PerfMockClient::new();
    mock.set_transport(Transport::Json);
    let perf = perf(&mock);
    let entity = vm("vm-1");
    perf.query()
        .entity(&entity)
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(2)
        .fetch()
        .await
        .unwrap();
    perf.query()
        .entity(&entity)
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    let calls = mock.queries();
    assert_eq!(calls[0][0].format.as_deref(), Some("csv"));
    assert_eq!(calls[1][0].format.as_deref(), Some("normal"));
    assert_eq!(calls[1][0].max_sample, Some(1));
}

#[cfg(feature = "xml")]
#[tokio::test]
async fn auto_format_on_soap_is_csv() {
    let mock = PerfMockClient::new();
    mock.set_transport(Transport::Soap);
    let perf = perf(&mock);
    perf.query()
        .entity(&vm("vm-1"))
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    assert_eq!(mock.queries()[0][0].format.as_deref(), Some("csv"));
}

#[tokio::test]
async fn missing_counter_warns_when_another_resolves() {
    let mock = PerfMockClient::new();
    let perf = perf(&mock);
    let result = perf
        .query()
        .entity(&vm("vm-1"))
        .metric_by_name("cpu.usage.average")
        .metric_by_name("no.such.counter")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    assert!(matches!(
        result.entity(&vm("vm-1")).unwrap().status,
        EntityStatus::Ok
    ));
    assert!(result.warnings.iter().any(|warning| matches!(
        warning,
        PerfWarning::CounterNotOnServer { counter } if counter == "no.such.counter"
    )));
    let ids = &mock.queries()[0][0].counter_ids;
    assert_eq!(ids, &vec![2]);
}

#[tokio::test]
async fn unknown_counter_fails_when_nothing_resolves() {
    let mock = PerfMockClient::new();
    let perf = perf(&mock);
    let err = perf
        .query()
        .entity(&vm("vm-1"))
        .metric_by_name("no.such.counter")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap_err();
    assert!(matches!(err, PerfError::UnknownCounter(name) if name == "no.such.counter"));
    assert!(mock.queries().is_empty());
}

#[tokio::test]
async fn dimension_mismatch_warns_or_fails_the_fetch() {
    let mock = PerfMockClient::new();
    let perf = perf(&mock);
    let err = perf
        .query()
        .entity(&vm("vm-1"))
        .metric(ratio("mem", "consumed", Rollup::Average))
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap_err();
    assert!(matches!(
        err,
        PerfError::DimensionMismatch { ref counter, ref unit }
            if counter == "mem.consumed.average" && unit == "kiloBytes"
    ));

    let result = perf
        .query()
        .entity(&vm("vm-1"))
        .metric(ratio("mem", "consumed", Rollup::Average))
        .metric(ratio("cpu", "usage", Rollup::Average))
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    assert!(result.warnings.iter().any(|warning| matches!(
        warning,
        PerfWarning::DimensionMismatch { counter, unit }
            if counter == "mem.consumed.average" && unit == "kiloBytes"
    )));
    assert_eq!(mock.queries()[0][0].counter_ids, vec![2]);
}

#[tokio::test]
async fn esxi_counter_without_a_level_does_not_warn() {
    let mock = PerfMockClient::new();
    mock.clear_counter_levels();
    mock.set_intervals(Vec::new());
    let perf = perf(&mock);
    let result = perf
        .query()
        .entity(&vm("vm-1"))
        .metric_by_name("disk.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    assert!(result
        .warnings
        .iter()
        .all(|warning| !matches!(warning, PerfWarning::LevelTooLow { .. })));
    assert!(mock.queries()[0][0].counter_ids.contains(&100));
}

#[tokio::test]
async fn level_too_low_still_sends_the_metric() {
    let mock = PerfMockClient::new();
    let perf = perf(&mock);
    let result = perf
        .query()
        .entity(&vm("vm-1"))
        .metric_by_name("disk.usage.average")
        .interval(Interval::PAST_DAY)
        .fetch()
        .await
        .unwrap();
    assert!(result.warnings.iter().any(|warning| matches!(
        warning,
        PerfWarning::LevelTooLow { counter, required_level: 4, interval_level: 2 }
            if counter == "disk.usage.average"
    )));
    assert!(mock.queries()[0][0].counter_ids.contains(&100));
}

#[tokio::test]
async fn out_of_scope_counter_without_a_series_warns_and_stays_ok() {
    let mock = PerfMockClient::new();
    mock.omit_counter(6);
    let perf = perf(&mock);
    let entity = vm("vm-1");
    let scoped =
        Metric::<Ratio, HostOnly>::new("cpu", "usage", Rollup::Maximum, None, None, None, false);
    let result = perf
        .query()
        .entity(&entity)
        .metric(ratio("cpu", "usage", Rollup::Average))
        .metric(scoped)
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    assert!(matches!(
        result.entity(&entity).unwrap().status,
        EntityStatus::Ok
    ));
    assert!(result.warnings.iter().any(|warning| matches!(
        warning,
        PerfWarning::ScopeMismatch { counter, .. } if counter == "cpu.usage.maximum"
    )));
}

#[tokio::test]
async fn unknown_other_unit_records_unscalable_unit() {
    let mock = PerfMockClient::new();
    let perf = perf(&mock);
    let result = perf
        .query()
        .entity(&vm("vm-1"))
        .metric(Metric::custom("weird", "thing", Rollup::Average))
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    assert!(result.warnings.iter().any(|warning| matches!(
        warning,
        PerfWarning::UnscalableUnit { counter, unit }
            if counter == "weird.thing.average" && unit == "furlongs"
    )));
    assert_eq!(mock.queries()[0][0].counter_ids, vec![200]);
}

#[tokio::test]
async fn not_authenticated_aborts() {
    let mock = PerfMockClient::new();
    mock.fail_fatal(StructType::NotAuthenticated);
    let perf = perf(&mock);
    let err = perf
        .query()
        .entity(&vm("vm-1"))
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap_err();
    match err {
        PerfError::Client(ClientError::MethodFault(fault)) => {
            assert_eq!(fault.type_, Some(StructType::NotAuthenticated));
        }
        other => panic!("expected NotAuthenticated, got {other}"),
    }
}

#[tokio::test]
async fn parse_error_fails_the_chunk() {
    let mock = PerfMockClient::new();
    mock.fail_parse("broken payload");
    let perf = perf(&mock);
    let err = perf
        .query()
        .entity(&vm("vm-1"))
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap_err();
    assert!(err.to_string().contains("broken payload"));
}

#[tokio::test]
async fn transport_error_aborts() {
    let mock = PerfMockClient::new();
    mock.fail_transport();
    let perf = perf(&mock);
    let err = perf
        .query()
        .entity(&vm("vm-1"))
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap_err();
    assert!(matches!(
        err,
        PerfError::Client(ClientError::MissingOrInvalidSessionKey)
    ));
}

#[tokio::test]
async fn object_scoped_fault_fails_one_chunk() {
    let mock = PerfMockClient::new();
    mock.fail_entity("vm-perm", StructType::NoPermission);
    mock.fail_entity("vm-arg", StructType::InvalidArgument);
    let perf = perf(&mock);
    let perm = vm("vm-perm");
    let arg = vm("vm-arg");
    let ok = vm("vm-ok");
    let result = perf
        .query()
        .entities(&[perm.clone(), arg.clone(), ok.clone()])
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .chunk_size(1)
        .fetch()
        .await
        .unwrap();
    assert!(matches!(
        result.entity(&perm).unwrap().status,
        EntityStatus::Failed(_)
    ));
    assert!(matches!(
        result.entity(&arg).unwrap().status,
        EntityStatus::Failed(_)
    ));
    assert!(matches!(
        result.entity(&ok).unwrap().status,
        EntityStatus::Ok
    ));
}

#[tokio::test]
async fn dropping_fetch_cancels_the_call() {
    let mock = PerfMockClient::new();
    let perf = perf(&mock);
    let gate = mock.hang_query_perf();
    let fetch = perf
        .query()
        .entity(&vm("vm-1"))
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch();
    {
        let mut fetch = std::pin::pin!(fetch);
        let mut started = false;
        for _ in 0..200 {
            tokio::select! {
                biased;
                _ = &mut fetch => panic!("hung fetch finished"),
                _ = tokio::task::yield_now() => {
                    if gate.started.load(Ordering::SeqCst) {
                        started = true;
                        break;
                    }
                }
            }
        }
        assert!(started, "QueryPerf was not entered");
    }
    for _ in 0..50 {
        tokio::task::yield_now().await;
    }
    assert!(
        gate.cancelled.load(Ordering::SeqCst),
        "in-flight QueryPerf was not cancelled"
    );
}

#[tokio::test]
async fn every_local_failure_returns_the_first_chunk() {
    let mock = PerfMockClient::new();
    mock.fail_all_local();
    let perf = perf(&mock);
    let err = perf
        .query()
        .entities(&[vm("vm-a"), vm("vm-b")])
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .chunk_size(1)
        .fetch()
        .await
        .unwrap_err();
    let text = err.to_string();
    assert!(text.contains("local:vm-a"), "{text}");
    assert!(!text.contains("local:vm-b"), "{text}");
    match err {
        PerfError::Client(ClientError::ParseError(message)) => {
            assert_eq!(message, "local:vm-a");
        }
        other => panic!("expected the original parse error, got {other}"),
    }
}

#[tokio::test]
async fn split_entity_stays_ok_when_one_chunk_succeeds() {
    let mock = PerfMockClient::new();
    mock.fail_batches_containing(2, "first-half");
    mock.echo_gaps();
    let perf = perf(&mock);
    let entity = vm("vm-1");
    let result = perf
        .query()
        .entity(&entity)
        .metrics_by_name(&[
            "cpu.usage.average",
            "cpu.usage.maximum",
            "cpu.ready.summation",
            "mem.usage.average",
        ])
        .interval(Interval::Realtime)
        .latest(1)
        .chunk_size(2)
        .fetch()
        .await
        .unwrap();
    let series = result.entity(&entity).unwrap();
    assert!(matches!(series.status, EntityStatus::Ok));
    assert!(series.series.iter().any(|raw| raw.values.contains(&-1)));
}

#[tokio::test]
async fn split_entity_fails_only_when_every_chunk_fails() {
    let mock = PerfMockClient::new();
    mock.fail_entity("vm-a", StructType::NoPermission);
    let perf = perf(&mock);
    let failed = vm("vm-a");
    let ok = vm("vm-b");
    let result = perf
        .query()
        .entities(&[failed.clone(), ok.clone()])
        .metrics_by_name(&[
            "cpu.usage.average",
            "cpu.usage.maximum",
            "cpu.ready.summation",
            "mem.usage.average",
        ])
        .interval(Interval::Realtime)
        .latest(1)
        .chunk_size(2)
        .fetch()
        .await
        .unwrap();
    assert!(matches!(
        result.entity(&failed).unwrap().status,
        EntityStatus::Failed(_)
    ));
    assert!(matches!(
        result.entity(&ok).unwrap().status,
        EntityStatus::Ok
    ));
    assert!(result
        .warnings
        .iter()
        .all(|warning| !matches!(warning, PerfWarning::PartialChunk { .. })));
}

#[tokio::test]
async fn one_failed_chunk_keeps_the_entity_and_warns() {
    let mock = PerfMockClient::new();
    mock.fail_batches_containing(12, "slice failed");
    let perf = perf(&mock);
    let entity = vm("vm-1");
    let result = perf
        .query()
        .entity(&entity)
        .metrics_by_name(&[
            "cpu.usage.average",
            "cpu.usage.maximum",
            "cpu.ready.summation",
            "mem.usage.average",
        ])
        .interval(Interval::Realtime)
        .latest(1)
        .chunk_size(2)
        .fetch()
        .await
        .unwrap();
    let series = result.entity(&entity).unwrap();
    assert!(matches!(series.status, EntityStatus::Ok));
    assert!(!series.series.is_empty());
    assert!(result.warnings.iter().any(|warning| matches!(
        warning,
        PerfWarning::PartialChunk { entity: warned, message }
            if warned == &entity && message.contains("slice failed")
    )));
}

#[tokio::test]
async fn availability_fault_fails_one_entity_and_queries_the_rest() {
    let mock = PerfMockClient::new();
    mock.fail_available("vm-1", StructType::ManagedObjectNotFound);
    let perf = perf(&mock);
    let missing = vm("vm-1");
    let live = vm("vm-2");
    let result = perf
        .query()
        .entities(&[missing.clone(), live.clone()])
        .all_available()
        .interval(Interval::Realtime)
        .latest(1)
        .concurrency(2)
        .fetch()
        .await
        .unwrap();
    assert!(matches!(
        result.entity(&missing).unwrap().status,
        EntityStatus::Failed(_)
    ));
    assert!(matches!(
        result.entity(&live).unwrap().status,
        EntityStatus::Ok
    ));
    let calls = mock.available_calls();
    assert!(calls.iter().any(|entity| entity == "vm-1"));
    assert!(calls.iter().any(|entity| entity == "vm-2"));
    let queried: Vec<_> = mock
        .queries()
        .into_iter()
        .flatten()
        .map(|spec| spec.entity)
        .collect();
    assert_eq!(queried, vec!["vm-2".to_string()]);
}

#[tokio::test]
async fn availability_not_authenticated_aborts_the_fetch() {
    let mock = PerfMockClient::new();
    mock.fail_available("vm-1", StructType::NotAuthenticated);
    let perf = perf(&mock);
    let err = perf
        .query()
        .entities(&[vm("vm-1"), vm("vm-2")])
        .all_available()
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap_err();
    match err {
        PerfError::Client(ClientError::MethodFault(fault)) => {
            assert_eq!(fault.type_, Some(StructType::NotAuthenticated));
        }
        other => panic!("expected NotAuthenticated, got {other}"),
    }
}

#[tokio::test]
async fn provider_not_found_retries_the_next_entity_and_caches() {
    let mock = PerfMockClient::new();
    mock.set_summary_fault("vm-1", StructType::ManagedObjectNotFound);
    let perf = perf(&mock);
    let first = vm("vm-1");
    let second = vm("vm-2");
    let result = perf
        .query()
        .entities(&[first.clone(), second.clone()])
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    assert!(matches!(
        result.entity(&first).unwrap().status,
        EntityStatus::Ok
    ));
    assert!(matches!(
        result.entity(&second).unwrap().status,
        EntityStatus::Ok
    ));
    assert_eq!(
        mock.summary_calls(),
        vec!["vm-1".to_string(), "vm-2".to_string()]
    );

    perf.query()
        .entity(&vm("vm-3"))
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    assert_eq!(mock.summary_calls().len(), 2);
}

#[tokio::test]
async fn esxi_vm_and_datastore_follow_provider_caps() {
    let mock = PerfMockClient::new();
    mock.set_intervals(Vec::new());
    mock.set_summary_provider("vm-1", true, false, Some(20));
    mock.set_summary_fault("ds-1", StructType::InvalidArgument);
    let perf = perf(&mock);
    let vm = vm("vm-1");
    let ds = datastore("ds-1");

    let auto = perf
        .query()
        .entities(&[vm.clone(), ds.clone()])
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Auto)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    assert!(matches!(auto.entity(&vm).unwrap().status, EntityStatus::Ok));
    assert!(matches!(
        auto.entity(&ds).unwrap().status,
        EntityStatus::Unsupported(UnsupportedReason::NotAProvider)
    ));

    let realtime = perf
        .query()
        .entities(&[vm.clone(), ds.clone()])
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    assert!(matches!(
        realtime.entity(&vm).unwrap().status,
        EntityStatus::Ok
    ));
    assert!(matches!(
        realtime.entity(&ds).unwrap().status,
        EntityStatus::Unsupported(UnsupportedReason::NotAProvider)
    ));

    let err = perf
        .query()
        .entities(&[vm, ds])
        .metric_by_name("cpu.usage.average")
        .interval(Interval::PAST_DAY)
        .fetch()
        .await
        .unwrap_err();
    assert!(matches!(err, PerfError::NoSupportedEntities));
}

#[tokio::test]
async fn omitted_entity_is_no_samples() {
    let mock = PerfMockClient::new();
    mock.omit_entity("vm-2");
    let perf = perf(&mock);
    let first = vm("vm-1");
    let second = vm("vm-2");
    let result = perf
        .query()
        .entities(&[first.clone(), second.clone()])
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    assert!(matches!(
        result.entity(&first).unwrap().status,
        EntityStatus::Ok
    ));
    assert!(matches!(
        result.entity(&second).unwrap().status,
        EntityStatus::NoSamples
    ));
}

#[tokio::test]
async fn since_ago_reads_the_clock_at_each_fetch() {
    let mock = PerfMockClient::new();
    let perf = perf(&mock);
    let prepared = perf
        .prepare()
        .metric_by_name("cpu.usage.average")
        .interval(Interval::PAST_DAY)
        .since_ago(Duration::from_secs(60))
        .build();
    assert!(mock.queries().is_empty());
    let t1 = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let t2 = t1 + Duration::from_secs(3600);
    pin(&perf, &mock, t1);
    prepared.fetch(&[vm("vm-1")]).await.unwrap();
    pin(&perf, &mock, t2);
    prepared.fetch(&[vm("vm-1")]).await.unwrap();
    let starts: Vec<_> = mock
        .queries()
        .iter()
        .flat_map(|call| call.iter().filter_map(|spec| spec.start.clone()))
        .collect();
    assert_eq!(starts.len(), 2);
    let a = Timestamp::parse_rfc3339(&starts[0]).unwrap();
    let b = Timestamp::parse_rfc3339(&starts[1]).unwrap();
    let delta =
        b.0.duration_since(a.0)
            .unwrap_or_else(|_| a.0.duration_since(b.0).unwrap());
    assert_eq!(delta, Duration::from_secs(3600));
    assert_eq!(mock.clock(), t2);
    assert!(mock.composites().is_empty());
}

#[tokio::test]
async fn time_chunk_boundary_is_neither_dropped_nor_duplicated() {
    let mock = PerfMockClient::new();
    let perf = perf(&mock);
    let start = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let boundary = start + Duration::from_secs(100);
    let end = start + Duration::from_secs(200);
    let entity = vm("vm-1");
    let result = perf
        .query()
        .entity(&entity)
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Historical(300))
        .between(start, end)
        .time_chunk(Duration::from_secs(100))
        .fetch()
        .await
        .unwrap();
    assert_eq!(mock.queries().len(), 1);
    let mut specs: Vec<_> = mock.queries().into_iter().flatten().collect();
    specs.sort_by(|left, right| left.start.cmp(&right.start));
    assert_eq!(specs.len(), 2);
    let boundary_text = Timestamp(boundary).to_rfc3339_utc();
    assert_eq!(specs[0].end.as_deref(), Some(boundary_text.as_str()));
    assert_eq!(specs[1].start.as_deref(), Some(boundary_text.as_str()));
    let series = result.entity(&entity).unwrap();
    let boundary_ts = Timestamp::parse_rfc3339(&boundary_text).unwrap();
    assert_eq!(
        series
            .timestamps
            .iter()
            .filter(|ts| **ts == boundary_ts)
            .count(),
        1
    );
    assert_eq!(series.timestamps.len(), 2);
}

#[tokio::test]
async fn empty_availability_is_not_cached() {
    let mock = PerfMockClient::new();
    mock.set_available_empty(true);
    let perf = perf(&mock);
    let entity = vm("vm-1");
    for _ in 0..2 {
        let result = perf
            .query()
            .entity(&entity)
            .all_available()
            .interval(Interval::Realtime)
            .latest(1)
            .fetch()
            .await
            .unwrap();
        assert!(matches!(
            result.entity(&entity).unwrap().status,
            EntityStatus::NoMetricsAvailable
        ));
    }
    assert_eq!(mock.available_calls().len(), 2);
    assert!(mock.queries().is_empty());
}

#[tokio::test]
async fn availability_cache_evicts_the_oldest_key() {
    let mock = PerfMockClient::new();
    let perf = perf(&mock);
    let first: Vec<_> = (0..1024).map(|i| vm(&format!("vm-{i}"))).collect();
    perf.query()
        .entities(&first)
        .all_available()
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    perf.query()
        .entity(&vm("vm-1024"))
        .all_available()
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    perf.query()
        .entity(&vm("vm-0"))
        .all_available()
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    let calls = mock.available_calls();
    assert_eq!(
        calls
            .iter()
            .filter(|entity| entity.as_str() == "vm-0")
            .count(),
        2
    );
    assert_eq!(
        calls
            .iter()
            .filter(|entity| entity.as_str() == "vm-1")
            .count(),
        1
    );
}

#[tokio::test]
async fn refresh_registry_makes_the_next_prepared_fetch_resolve_again() {
    let mock = PerfMockClient::new();
    let perf = perf(&mock);
    let prepared = perf
        .prepare()
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .build();
    prepared.fetch(&[vm("vm-1")]).await.unwrap();
    let generation = perf.registry().await.unwrap().generation();
    let mut counters = vcenter_counters();
    counters.retain(|counter| counter.key == 98);
    mock.set_counters(counters);
    let refreshed = perf.refresh_registry().await.unwrap().generation();
    assert_ne!(generation, refreshed);
    let err = prepared.fetch(&[vm("vm-1")]).await.unwrap_err();
    assert!(matches!(err, PerfError::UnknownCounter(name) if name == "cpu.usage.average"));
}

#[tokio::test]
async fn duplicate_entities_are_queried_once() {
    let mock = PerfMockClient::new();
    let perf = perf(&mock);
    let entity = vm("vm-1");
    let result = perf
        .query()
        .entities(&[entity.clone(), entity.clone()])
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    assert!(matches!(
        result.entity(&entity).unwrap().status,
        EntityStatus::Ok
    ));
    assert_eq!(result.entities().count(), 1);
    assert_eq!(mock.queries().len(), 1);
    assert_eq!(mock.queries()[0].len(), 1);

    perf.query()
        .entities(&[entity.clone(), entity])
        .all_available()
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    assert_eq!(mock.available_calls().len(), 1);
}

#[tokio::test]
async fn malformed_sample_fails_only_that_entity() {
    let mock = PerfMockClient::new();
    mock.bad_sample("vm-bad");
    let perf = perf(&mock);
    let bad = vm("vm-bad");
    let ok = vm("vm-ok");
    let result = perf
        .query()
        .entities(&[bad.clone(), ok.clone()])
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    assert!(matches!(
        result.entity(&bad).unwrap().status,
        EntityStatus::Failed(_)
    ));
    assert!(matches!(
        result.entity(&ok).unwrap().status,
        EntityStatus::Ok
    ));
}

#[tokio::test]
async fn mixed_interval_and_window_share_one_call() {
    let mock = PerfMockClient::new();
    mock.set_summary_provider("vm-1", true, true, Some(20));
    mock.set_summary_provider("cluster-1", false, true, None);
    let perf = perf(&mock);
    let vm = vm("vm-1");
    let cluster = ManagedObjectReference {
        r#type: MoTypesEnum::ClusterComputeResource,
        value: "cluster-1".into(),
    };
    let result = perf
        .query()
        .entities(&[vm.clone(), cluster.clone()])
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Auto)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    assert!(matches!(
        result.entity(&vm).unwrap().status,
        EntityStatus::Ok
    ));
    assert!(matches!(
        result.entity(&cluster).unwrap().status,
        EntityStatus::Ok
    ));
    let calls = mock.queries();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].len(), 2);
    let mut intervals: Vec<_> = calls[0]
        .iter()
        .filter_map(|spec| spec.interval_id)
        .collect();
    intervals.sort_unstable();
    assert_eq!(intervals, vec![20, 300]);
}

#[tokio::test]
async fn distinct_kinds_probe_in_parallel_and_both_stay_cached() {
    let mock = PerfMockClient::new();
    let gate = mock.hang_summaries();
    let perf = perf(&mock);
    let vm_fetch = perf
        .query()
        .entity(&vm("vm-1"))
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch();
    let ds_fetch = perf
        .query()
        .entity(&datastore("ds-1"))
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch();
    let mut vm_fetch = std::pin::pin!(vm_fetch);
    let mut ds_fetch = std::pin::pin!(ds_fetch);
    let mut started = false;
    for _ in 0..400 {
        tokio::select! {
            biased;
            _ = &mut vm_fetch => panic!("vm fetch finished while summaries were held"),
            _ = &mut ds_fetch => panic!("datastore fetch finished while summaries were held"),
            _ = tokio::task::yield_now() => {
                if gate.started.load(Ordering::SeqCst) >= 2 {
                    started = true;
                    break;
                }
            }
        }
    }
    assert!(started, "the two kinds did not probe at the same time");
    gate.release.store(true, Ordering::SeqCst);
    vm_fetch.await.unwrap();
    ds_fetch.await.unwrap();
    assert_eq!(mock.summary_calls().len(), 2);
    perf.query()
        .entity(&vm("vm-2"))
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    perf.query()
        .entity(&datastore("ds-2"))
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    assert_eq!(mock.summary_calls().len(), 2);
}

#[tokio::test]
async fn concurrent_probes_of_one_kind_share_one_summary() {
    let mock = PerfMockClient::new();
    let gate = mock.hang_summaries();
    let perf = perf(&mock);
    let first = perf
        .query()
        .entity(&vm("vm-1"))
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch();
    let second = perf
        .query()
        .entity(&vm("vm-2"))
        .metric_by_name("cpu.usage.average")
        .interval(Interval::Realtime)
        .latest(1)
        .fetch();
    let mut first = std::pin::pin!(first);
    let mut second = std::pin::pin!(second);
    let mut entered = false;
    for _ in 0..400 {
        tokio::select! {
            biased;
            _ = &mut first => panic!("fetch finished while the summary was held"),
            _ = &mut second => panic!("fetch finished while the summary was held"),
            _ = tokio::task::yield_now() => {
                if gate.started.load(Ordering::SeqCst) >= 1 {
                    entered = true;
                    break;
                }
            }
        }
    }
    assert!(entered, "no provider summary started");
    for _ in 0..40 {
        tokio::task::yield_now().await;
    }
    assert_eq!(gate.started.load(Ordering::SeqCst), 1);
    gate.release.store(true, Ordering::SeqCst);
    first.await.unwrap();
    second.await.unwrap();
    assert_eq!(mock.summary_calls().len(), 1);
}

#[tokio::test]
async fn all_available_warns_that_named_metrics_were_ignored() {
    let mock = PerfMockClient::new();
    let perf = perf(&mock);
    let result = perf
        .query()
        .entity(&vm("vm-1"))
        .metric_by_name("not.a.counter")
        .all_available()
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    assert!(matches!(
        result.entity(&vm("vm-1")).unwrap().status,
        EntityStatus::Ok
    ));
    assert!(result
        .warnings
        .iter()
        .any(|warning| matches!(warning, PerfWarning::ExplicitMetricsIgnored)));
    assert!(result.warnings.iter().any(|warning| matches!(
        warning,
        PerfWarning::CounterNotOnServer { counter } if counter == "not.a.counter"
    )));
    assert!(!mock.queries().is_empty());
}

#[tokio::test]
async fn groups_with_an_explicit_metric_are_not_applied() {
    let mock = PerfMockClient::new();
    let perf = perf(&mock);
    let result = perf
        .query()
        .entity(&vm("vm-1"))
        .metric_by_name("cpu.usage.average")
        .groups(&["mem"])
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    assert!(result
        .warnings
        .iter()
        .any(|warning| matches!(warning, PerfWarning::GroupsIgnored)));
    assert!(mock.queries()[0][0].counter_ids.contains(&2));
}
