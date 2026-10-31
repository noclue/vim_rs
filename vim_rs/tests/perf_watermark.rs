#![cfg(feature = "perf")]

//! Row export and incremental watermarks.

use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, UNIX_EPOCH};

use vim_rs::core::client::{unmarshal_array, Transport, VimClientHandle};
use vim_rs::perf::decode::decode_metrics;
use vim_rs::perf::registry::CounterRegistry;
use vim_rs::perf::watermark::SampleWatermark;
use vim_rs::perf::PerfClient;
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::structs::ManagedObjectReference;
use vim_rs::types::traits::PerfEntityMetricBaseTrait;

#[path = "support/perf_mock.rs"]
mod perf_mock;

use perf_mock::PerfMockClient;

const GAPS: &str = include_str!("fixtures/perf/query_perf_gaps.json");

fn registry() -> Arc<CounterRegistry> {
    Arc::new(
        CounterRegistry::from_catalog_json(
            r#"{
                "schema": "vim_rs.perf-catalog/1",
                "intervals": [],
                "counters": [
                    {"key": 2, "group": "cpu", "name": "usage", "rollup": "average", "stats": "rate", "unit": "percent", "level": 1, "per_device_level": 3, "labels": {"name": "Usage"}},
                    {"key": 6, "group": "cpu", "name": "usagemhz", "rollup": "average", "stats": "rate", "unit": "megaHertz", "level": 1, "per_device_level": 3, "labels": {"name": "Usage MHz"}}
                ]
            }"#,
        )
        .unwrap(),
    )
}

fn vm(id: &str) -> ManagedObjectReference {
    ManagedObjectReference {
        r#type: MoTypesEnum::VirtualMachine,
        value: id.into(),
    }
}

fn uneven() -> vim_rs::perf::PerfResult {
    let json = r#"[
        {
            "_typeName": "PerfEntityMetric",
            "entity": {"_typeName": "ManagedObjectReference", "type": "VirtualMachine", "value": "vm-1"},
            "sampleInfo": [
                {"_typeName": "PerfSampleInfo", "timestamp": "2026-10-24T10:00:00Z", "interval": 20},
                {"_typeName": "PerfSampleInfo", "timestamp": "2026-10-24T10:00:20Z", "interval": 20}
            ],
            "value": [
                {"_typeName": "PerfMetricIntSeries", "id": {"_typeName": "PerfMetricId", "counterId": 2, "instance": ""}, "value": [5, -1]},
                {"_typeName": "PerfMetricIntSeries", "id": {"_typeName": "PerfMetricId", "counterId": 6, "instance": ""}, "value": [7, 9]}
            ]
        }
    ]"#;
    let raw: Vec<Box<dyn PerfEntityMetricBaseTrait>> =
        unmarshal_array(Transport::Json, json.as_bytes()).unwrap();
    decode_metrics(raw, &registry(), &[], None, UNIX_EPOCH).unwrap()
}

/// A later poll where `cpu.usage` (counter 2) is gone and only counter 6 reports.
fn only_usagemhz() -> vim_rs::perf::PerfResult {
    let json = r#"[
        {
            "_typeName": "PerfEntityMetric",
            "entity": {"_typeName": "ManagedObjectReference", "type": "VirtualMachine", "value": "vm-1"},
            "sampleInfo": [
                {"_typeName": "PerfSampleInfo", "timestamp": "2026-10-24T10:00:40Z", "interval": 20}
            ],
            "value": [
                {"_typeName": "PerfMetricIntSeries", "id": {"_typeName": "PerfMetricId", "counterId": 6, "instance": ""}, "value": [11]}
            ]
        }
    ]"#;
    let raw: Vec<Box<dyn PerfEntityMetricBaseTrait>> =
        unmarshal_array(Transport::Json, json.as_bytes()).unwrap();
    decode_metrics(raw, &registry(), &[], None, UNIX_EPOCH).unwrap()
}

#[test]
fn series_that_stops_reporting_does_not_pin_high_water() {
    let mut marks = SampleWatermark::new();
    let first = uneven();
    marks.filter_new(&first).for_each(drop);
    let stale = marks.entity_high_water(&vm("vm-1")).unwrap();

    let second = only_usagemhz();
    assert_eq!(marks.filter_new(&second).count(), 1);
    let advanced = marks.entity_high_water(&vm("vm-1")).unwrap();
    assert!(advanced > stale);
    assert_eq!(advanced, second.rows().next().unwrap().ts);
}

#[test]
fn rows_and_filter_new_skip_gaps_and_marks_update_eagerly() {
    let result = uneven();
    assert!(result.rows().all(|row| row.raw >= 0));
    assert_eq!(result.rows().count(), 3);

    let mut marks = SampleWatermark::new();
    let iter = marks.filter_new(&result);
    drop(iter);
    let entity = vm("vm-1");
    let high = marks.entity_high_water(&entity).unwrap();
    assert_eq!(high.to_rfc3339_utc(), "2026-10-24T10:00:00Z");
    assert_eq!(
        marks.min_high_water(&[entity.clone(), vm("vm-2")]),
        Some(high)
    );

    let mut active = HashSet::new();
    active.insert(vm("vm-2"));
    marks.retain_entities(&active);
    assert!(marks.entity_high_water(&entity).is_none());
}

#[test]
fn gaps_fixture_rows_skip_negative_raw_values() {
    let raw: Vec<Box<dyn PerfEntityMetricBaseTrait>> =
        unmarshal_array(Transport::Json, GAPS.as_bytes()).unwrap();
    let registry = Arc::new(
        CounterRegistry::from_catalog_json(
            r#"{
                "schema": "vim_rs.perf-catalog/1",
                "intervals": [],
                "counters": [
                    {"key": 2, "group": "cpu", "name": "usage", "rollup": "average", "stats": "rate", "unit": "percent", "level": 1, "per_device_level": 3, "labels": {"name": "Usage"}},
                    {"key": 6, "group": "cpu", "name": "usagemhz", "rollup": "average", "stats": "rate", "unit": "megaHertz", "level": 1, "per_device_level": 3, "labels": {"name": "Usage"}},
                    {"key": 12, "group": "cpu", "name": "ready", "rollup": "summation", "stats": "delta", "unit": "millisecond", "level": 1, "per_device_level": 3, "labels": {"name": "Ready"}}
                ]
            }"#,
        )
        .unwrap(),
    );
    let result = decode_metrics(raw, &registry, &[], None, UNIX_EPOCH).unwrap();
    assert!(result.rows().all(|row| row.raw >= 0));
    assert!(result.rows().next().unwrap().scaled().is_some());
}

#[tokio::test]
async fn incremental_start_omits_max_sample_only_for_marked_entities() {
    let mock = PerfMockClient::new();
    let handle: VimClientHandle = mock.clone();
    let perf = PerfClient::new(handle).unwrap();
    let now = UNIX_EPOCH + Duration::from_secs(1_800_000_000);
    mock.set_clock(now);
    perf.set_clock(now);
    let marked = vm("vm-mark");
    let fresh = vm("vm-fresh");
    let prepared = perf
        .prepare()
        .metric_by_name("cpu.usage.average")
        .interval(vim_rs::perf::Interval::Realtime)
        .latest(1)
        .build();
    let first = prepared.fetch(std::slice::from_ref(&marked)).await.unwrap();
    let mut watermark = SampleWatermark::new();
    let _ = watermark.filter_new(&first);
    prepared
        .fetch_incremental(&[marked.clone(), fresh.clone()], &watermark)
        .await
        .unwrap();
    assert_eq!(mock.queries().len(), 2);
    assert_eq!(mock.queries()[1].len(), 2);
    let specs: Vec<_> = mock.queries().into_iter().flatten().collect();
    let marked_spec = specs
        .iter()
        .find(|spec| spec.entity == "vm-mark" && spec.start.is_some())
        .unwrap();
    let fresh_spec = specs.iter().find(|spec| spec.entity == "vm-fresh").unwrap();
    assert!(marked_spec.max_sample.is_none());
    assert!(fresh_spec.start.is_none());
    assert_eq!(fresh_spec.max_sample, Some(1));
    let again = watermark.entity_high_water(&marked);
    prepared
        .fetch_incremental(&[marked.clone()], &watermark)
        .await
        .unwrap();
    assert_eq!(watermark.entity_high_water(&marked), again);
}
