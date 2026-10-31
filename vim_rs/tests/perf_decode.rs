#![cfg(feature = "perf")]

//! Fixture decode, gap accessors, and unit display.

use std::sync::Arc;

use vim_rs::core::client::{unmarshal_array, Transport};
use vim_rs::perf::entity::AnyEntity;
use vim_rs::perf::error::{PerfError, PerfWarning};
use vim_rs::perf::metric::{Metric, Rollup, StatsKind};
use vim_rs::perf::query::Instance;
use vim_rs::perf::registry::{CounterId, CounterRegistry};
use vim_rs::perf::result::Timestamp;
use vim_rs::perf::units::{display_raw, AnyDimension, Bytes, Time};
use vim_rs::perf::{decode::decode_metrics, EntityStatus};
use vim_rs::types::enums::PerformanceManagerUnitEnum;
use vim_rs::types::traits::PerfEntityMetricBaseTrait;

const CSV: &str = include_str!("fixtures/perf/query_perf_csv.json");
const NORMAL: &str = include_str!("fixtures/perf/query_perf_normal.json");
const GAPS: &str = include_str!("fixtures/perf/query_perf_gaps.json");

fn registry() -> Arc<CounterRegistry> {
    Arc::new(
        CounterRegistry::from_catalog_json(
            r#"{
                "schema": "vim_rs.perf-catalog/1",
                "intervals": [],
                "counters": [
                    {"key": 2, "group": "cpu", "name": "usage", "rollup": "average", "stats": "rate", "unit": "percent", "level": 1, "per_device_level": 3, "labels": {"name": "Usage"}},
                    {"key": 6, "group": "cpu", "name": "usagemhz", "rollup": "average", "stats": "rate", "unit": "megaHertz", "level": 1, "per_device_level": 3, "labels": {"name": "Usage MHz"}},
                    {"key": 12, "group": "cpu", "name": "ready", "rollup": "summation", "stats": "delta", "unit": "millisecond", "level": 1, "per_device_level": 3, "labels": {"name": "Ready"}},
                    {"key": 24, "group": "mem", "name": "usage", "rollup": "average", "stats": "absolute", "unit": "percent", "level": 1, "per_device_level": 1, "labels": {"name": "Usage"}},
                    {"key": 98, "group": "mem", "name": "consumed", "rollup": "average", "stats": "absolute", "unit": "kiloBytes", "level": 1, "per_device_level": 1, "labels": {"name": "Consumed"}}
                ]
            }"#,
        )
        .unwrap(),
    )
}

fn load(json: &str) -> Vec<Box<dyn PerfEntityMetricBaseTrait>> {
    unmarshal_array(Transport::Json, json.as_bytes()).expect("fixture json")
}

fn values_of(json: &str) -> Vec<(i32, String, Vec<i64>)> {
    let result = decode_metrics(
        load(json),
        &registry(),
        &[],
        None,
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap();
    let mut rows = Vec::new();
    for entity in result.entities() {
        assert!(matches!(entity.status, EntityStatus::Ok));
        for series in &entity.series {
            rows.push((
                series.counter.id.0,
                series.instance.as_wire_str().to_string(),
                series.values.clone(),
            ));
        }
    }
    rows
}

#[test]
fn duplicate_requested_entity_keeps_decoded_series() {
    let first = decode_metrics(
        load(NORMAL),
        &registry(),
        &[],
        None,
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap();
    let entity = first.entities().next().unwrap().entity.clone();
    let result = decode_metrics(
        load(NORMAL),
        &registry(),
        &[entity.clone(), entity.clone()],
        None,
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap();
    assert_eq!(result.entities().count(), 1);
    let decoded = result.entity(&entity).unwrap();
    assert!(matches!(decoded.status, EntityStatus::Ok));
    assert!(!decoded.series.is_empty());
}

#[test]
fn normal_and_csv_fixtures_match() {
    let mut csv = values_of(CSV);
    let mut normal = values_of(NORMAL);
    csv.sort_by_key(|(id, instance, _)| (*id, instance.clone()));
    normal.sort_by_key(|(id, instance, _)| (*id, instance.clone()));
    assert_eq!(csv, normal);
    assert_eq!(csv.len(), 4);
}

#[test]
fn gaps_last_and_window() {
    let result = decode_metrics(
        load(GAPS),
        &registry(),
        &[],
        None,
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap();
    let vm = result
        .entities()
        .find(|entity| entity.entity.value == "vm-1001")
        .unwrap();
    let mhz = vm
        .series
        .iter()
        .find(|series| series.counter.id == CounterId(6))
        .unwrap();
    assert_eq!(mhz.values, vec![420, 410, -1, -1]);
    assert_eq!(mhz.last().unwrap().value, 410);
    assert!(mhz.last_slot().is_none());
    let usage = vm
        .series
        .iter()
        .find(|series| series.counter.id == CounterId(2))
        .unwrap();
    assert_eq!(usage.values, vec![38, 41, -1, -1]);

    let vm2 = result
        .entities()
        .find(|entity| entity.entity.value == "vm-1002")
        .unwrap();
    let window = vm2.series[0].view::<Time>().window::<5>();
    assert_eq!(window, [None, None, Some(100), None, Some(300)]);
}

#[test]
fn ready_percent_aggregate_and_blanks() {
    let result = decode_metrics(
        load(GAPS),
        &registry(),
        &[],
        None,
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap();
    let vm = result
        .entities()
        .find(|entity| entity.entity.value == "vm-1001")
        .unwrap();
    let ready = Metric::<Time, AnyEntity>::new(
        "cpu",
        "ready",
        Rollup::Summation,
        Some(StatsKind::Delta),
        Some(1),
        Some(3),
        true,
    );
    let aggregate = vm.series(ready).unwrap();
    let percent = aggregate.ready_percent(2);
    assert_eq!(percent[0].unwrap().0, 2.5);
    assert!(percent[2].is_none());

    let device = vm
        .series_instance(ready, &Instance::Named("0".into()))
        .unwrap();
    assert!(device.ready_percent(2).iter().all(|slot| slot.is_none()));
    assert!(aggregate.ready_percent(0).iter().all(|slot| slot.is_none()));

    let mut blank_interval = vm
        .series
        .iter()
        .find(|series| series.counter.id == CounterId(12) && series.instance.is_aggregate())
        .unwrap()
        .clone();
    blank_interval.interval_secs = 0;
    assert!(blank_interval
        .view::<Time>()
        .ready_percent(2)
        .iter()
        .all(|slot| slot.is_none()));
}

#[test]
fn byte_scaling_and_unit_display() {
    assert_eq!(
        display_raw(4523, &PerformanceManagerUnitEnum::Percent),
        "45.23 %"
    );
    assert_eq!(
        Bytes::from_raw(1_048_576, &PerformanceManagerUnitEnum::KiloBytes)
            .unwrap()
            .0,
        1_073_741_824
    );
    assert_eq!(
        display_raw(1_048_576, &PerformanceManagerUnitEnum::KiloBytes),
        "1.00 GiB"
    );
    assert_eq!(
        Bytes::from_raw(16, &PerformanceManagerUnitEnum::Other_("gigaBytes".into()))
            .unwrap()
            .0,
        17_179_869_184
    );
    assert_eq!(
        display_raw(16, &PerformanceManagerUnitEnum::Other_("gigaBytes".into())),
        "16.00 GiB"
    );
    assert_eq!(
        display_raw(1, &PerformanceManagerUnitEnum::MegaBytes),
        "1.00 MiB"
    );
    assert_eq!(
        display_raw(1, &PerformanceManagerUnitEnum::TeraBytes),
        "1.00 TiB"
    );
    assert_eq!(
        display_raw(1, &PerformanceManagerUnitEnum::KiloBytesPerSecond),
        "1.00 KiB/s"
    );
    assert_eq!(
        display_raw(1, &PerformanceManagerUnitEnum::MegaBytesPerSecond),
        "1.00 MiB/s"
    );
    assert_eq!(
        display_raw(100, &PerformanceManagerUnitEnum::MegaHertz),
        "100 MHz"
    );
    assert_eq!(
        display_raw(500, &PerformanceManagerUnitEnum::Nanosecond),
        "500 ns"
    );
    assert_eq!(
        display_raw(1, &PerformanceManagerUnitEnum::Microsecond),
        "1.00 µs"
    );
    assert_eq!(
        display_raw(1, &PerformanceManagerUnitEnum::Millisecond),
        "1.00 ms"
    );
    assert_eq!(
        display_raw(2, &PerformanceManagerUnitEnum::Second),
        "2.00 s"
    );
    assert_eq!(display_raw(1, &PerformanceManagerUnitEnum::Watt), "1.0 W");
    assert_eq!(display_raw(1, &PerformanceManagerUnitEnum::Joule), "1.0 J");
    assert_eq!(
        display_raw(1, &PerformanceManagerUnitEnum::Celsius),
        "1.0 °C"
    );
    assert_eq!(display_raw(4, &PerformanceManagerUnitEnum::Number), "4");
    assert_eq!(
        display_raw(0, &PerformanceManagerUnitEnum::KiloBytes),
        "0 B"
    );
}

#[test]
fn union_grid_and_interval_substitution() {
    let json = r#"[
        {"_typeName":"PerfEntityMetricCSV","entity":{"_typeName":"ManagedObjectReference","type":"VirtualMachine","value":"vm-1"},
         "sampleInfoCSV":"20,2026-10-24T10:00:00Z,20,2026-10-24T10:00:20Z",
         "value":[{"_typeName":"PerfMetricSeriesCSV","id":{"_typeName":"PerfMetricId","counterId":2,"instance":""},"value":"1,2"}]},
        {"_typeName":"PerfEntityMetricCSV","entity":{"_typeName":"ManagedObjectReference","type":"VirtualMachine","value":"vm-1"},
         "sampleInfoCSV":"20,2026-10-24T10:00:20Z,20,2026-10-24T10:00:40Z",
         "value":[{"_typeName":"PerfMetricSeriesCSV","id":{"_typeName":"PerfMetricId","counterId":6,"instance":""},"value":"3,4"}]}
    ]"#;
    let result = decode_metrics(
        load(json),
        &registry(),
        &[],
        Some(20),
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap();
    let entity = result.entities().next().unwrap();
    assert_eq!(entity.timestamps.len(), 3);
    let usage = entity
        .series
        .iter()
        .find(|series| series.counter.id == CounterId(2))
        .unwrap();
    let mhz = entity
        .series
        .iter()
        .find(|series| series.counter.id == CounterId(6))
        .unwrap();
    assert_eq!(usage.values, vec![1, 2, -1]);
    assert_eq!(mhz.values, vec![-1, 3, 4]);
    assert!(Arc::ptr_eq(&usage.timestamps, &mhz.timestamps));

    let gaps = decode_metrics(
        load(GAPS),
        &registry(),
        &[],
        Some(20),
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap();
    assert!(gaps.warnings.iter().any(|warning| matches!(
        warning,
        PerfWarning::IntervalSubstituted {
            requested_secs: 20,
            actual_secs: 300
        }
    )));
    let substituted = gaps
        .warnings
        .iter()
        .filter(|warning| matches!(warning, PerfWarning::IntervalSubstituted { .. }))
        .count();
    assert_eq!(substituted, 1);
}

#[test]
fn empty_csv_token_is_a_gap() {
    let json = r#"[{
        "_typeName":"PerfEntityMetricCSV",
        "entity":{"_typeName":"ManagedObjectReference","type":"VirtualMachine","value":"vm-1"},
        "sampleInfoCSV":"20,2026-10-24T10:00:00Z,20,2026-10-24T10:00:20Z,20,2026-10-24T10:00:40Z",
        "value":[{"_typeName":"PerfMetricSeriesCSV","id":{"_typeName":"PerfMetricId","counterId":2,"instance":""},"value":"1,,3"}]
    }]"#;
    let result = decode_metrics(
        load(json),
        &registry(),
        &[],
        None,
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap();
    assert_eq!(
        result.entities().next().unwrap().series[0].values,
        vec![1, -1, 3]
    );
}

#[cfg(feature = "xml")]
#[test]
fn soap_fixture_matches_csv() {
    let soap: Vec<Box<dyn PerfEntityMetricBaseTrait>> =
        vim_rs::xml::soap::vim_response_list(include_str!("fixtures/perf/query_perf_soap.xml"))
            .expect("soap fixture");
    let result = decode_metrics(
        soap,
        &registry(),
        &[],
        None,
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap();
    let mut soap_rows = Vec::new();
    for entity in result.entities() {
        for series in &entity.series {
            soap_rows.push((series.counter.id.0, series.values.clone()));
        }
    }
    soap_rows.sort();
    let mut csv = values_of(CSV);
    csv.sort_by_key(|(id, _, _)| *id);
    let csv_rows: Vec<_> = csv
        .into_iter()
        .map(|(id, _, values)| (id, values))
        .collect();
    assert_eq!(soap_rows, csv_rows);
}

#[test]
fn omitted_requested_entity_is_no_samples() {
    let result = decode_metrics(
        load(CSV),
        &registry(),
        &[vim_rs::types::structs::ManagedObjectReference {
            r#type: vim_rs::types::enums::MoTypesEnum::VirtualMachine,
            value: "vm-missing".into(),
        }],
        None,
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap();
    assert!(matches!(
        result.entities().next().unwrap().status,
        EntityStatus::NoSamples
    ));
    let _ = Timestamp::parse_rfc3339("2026-10-24T10:48:20Z").unwrap();
}

fn usage_metric() -> Metric<AnyDimension, AnyEntity> {
    Metric::new(
        "cpu",
        "usage",
        Rollup::Average,
        Some(StatsKind::Rate),
        Some(1),
        Some(3),
        true,
    )
}

#[test]
fn single_payload_keeps_column_order_and_pads() {
    let json = r#"[
        {"_typeName":"PerfEntityMetricCSV","entity":{"_typeName":"ManagedObjectReference","type":"VirtualMachine","value":"vm-1"},
         "sampleInfoCSV":"20,2026-10-24T10:00:00Z,300,2026-10-24T10:00:20Z,300,2026-10-24T10:00:40Z",
         "value":[
            {"_typeName":"PerfMetricSeriesCSV","id":{"_typeName":"PerfMetricId","counterId":2,"instance":""},"value":"1,-1"},
            {"_typeName":"PerfMetricSeriesCSV","id":{"_typeName":"PerfMetricId","counterId":2,"instance":""},"value":"-1,4,9"},
            {"_typeName":"PerfMetricSeriesCSV","id":{"_typeName":"PerfMetricId","counterId":6,"instance":""},"value":"8,7"}
         ]}
    ]"#;
    let result = decode_metrics(
        load(json),
        &registry(),
        &[],
        Some(20),
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap();
    let entity = result.entities().next().unwrap();
    assert_eq!(entity.timestamps.len(), 3);
    let usage = entity
        .series
        .iter()
        .find(|series| series.counter.id == CounterId(2))
        .unwrap();
    let mhz = entity
        .series
        .iter()
        .find(|series| series.counter.id == CounterId(6))
        .unwrap();
    assert_eq!(usage.values, vec![1, 4, 9]);
    assert_eq!(mhz.values, vec![8, 7, -1]);
    assert!(Arc::ptr_eq(&usage.timestamps, &mhz.timestamps));
    assert_eq!(entity.interval_secs, 20);
    let substituted = result
        .warnings
        .iter()
        .filter(|warning| {
            matches!(
                warning,
                PerfWarning::IntervalSubstituted {
                    requested_secs: 20,
                    actual_secs: 300
                }
            )
        })
        .count();
    assert_eq!(substituted, 1);
}

#[test]
fn unsorted_payload_sorts_onto_the_grid() {
    let json = r#"[
        {"_typeName":"PerfEntityMetricCSV","entity":{"_typeName":"ManagedObjectReference","type":"VirtualMachine","value":"vm-1"},
         "sampleInfoCSV":"20,2026-10-24T10:00:20Z,20,2026-10-24T10:00:00Z",
         "value":[{"_typeName":"PerfMetricSeriesCSV","id":{"_typeName":"PerfMetricId","counterId":2,"instance":""},"value":"2,1"}]}
    ]"#;
    let result = decode_metrics(
        load(json),
        &registry(),
        &[],
        Some(20),
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap();
    let entity = result.entities().next().unwrap();
    assert_eq!(
        entity.timestamps.as_ref(),
        &[
            Timestamp::parse_rfc3339("2026-10-24T10:00:00Z").unwrap(),
            Timestamp::parse_rfc3339("2026-10-24T10:00:20Z").unwrap(),
        ]
    );
    assert_eq!(entity.series[0].values, vec![1, 2]);
}

#[test]
fn duplicate_timestamp_keeps_the_first_interval_and_merges_values() {
    let json = r#"[
        {"_typeName":"PerfEntityMetricCSV","entity":{"_typeName":"ManagedObjectReference","type":"VirtualMachine","value":"vm-1"},
         "sampleInfoCSV":"20,2026-10-24T10:00:00Z,99,2026-10-24T10:00:00Z",
         "value":[{"_typeName":"PerfMetricSeriesCSV","id":{"_typeName":"PerfMetricId","counterId":2,"instance":""},"value":"-1,7"}]}
    ]"#;
    let result = decode_metrics(
        load(json),
        &registry(),
        &[],
        Some(20),
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap();
    let entity = result.entities().next().unwrap();
    assert_eq!(entity.timestamps.len(), 1);
    assert_eq!(entity.interval_secs, 20);
    assert_eq!(entity.series[0].values, vec![7]);
}

#[test]
fn identical_payloads_still_merge() {
    let json = r#"[
        {"_typeName":"PerfEntityMetricCSV","entity":{"_typeName":"ManagedObjectReference","type":"VirtualMachine","value":"vm-1"},
         "sampleInfoCSV":"20,2026-10-24T10:00:00Z,20,2026-10-24T10:00:20Z",
         "value":[{"_typeName":"PerfMetricSeriesCSV","id":{"_typeName":"PerfMetricId","counterId":2,"instance":""},"value":"1,2"}]},
        {"_typeName":"PerfEntityMetricCSV","entity":{"_typeName":"ManagedObjectReference","type":"VirtualMachine","value":"vm-1"},
         "sampleInfoCSV":"20,2026-10-24T10:00:00Z,20,2026-10-24T10:00:20Z",
         "value":[{"_typeName":"PerfMetricSeriesCSV","id":{"_typeName":"PerfMetricId","counterId":2,"instance":""},"value":"-1,9"}]}
    ]"#;
    let result = decode_metrics(
        load(json),
        &registry(),
        &[],
        Some(20),
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap();
    let entity = result.entities().next().unwrap();
    assert_eq!(entity.series[0].values, vec![1, 9]);
}

#[test]
fn bad_timestamp_fails_the_decode() {
    let json = r#"[{
        "_typeName":"PerfEntityMetricCSV",
        "entity":{"_typeName":"ManagedObjectReference","type":"VirtualMachine","value":"vm-1"},
        "sampleInfoCSV":"20,not-a-timestamp",
        "value":[{"_typeName":"PerfMetricSeriesCSV","id":{"_typeName":"PerfMetricId","counterId":2,"instance":""},"value":"1"}]
    }]"#;
    let err = decode_metrics(
        load(json),
        &registry(),
        &[],
        None,
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap_err();
    assert!(matches!(err, PerfError::InvalidTimestamp(_)));
}

#[test]
fn series_lookup_follows_the_entity_registry() {
    let json = |counter: i32| {
        format!(
            r#"[{{
                "_typeName":"PerfEntityMetricCSV",
                "entity":{{"_typeName":"ManagedObjectReference","type":"VirtualMachine","value":"vm-1"}},
                "sampleInfoCSV":"20,2026-10-24T10:00:00Z,20,2026-10-24T10:00:20Z",
                "value":[
                    {{"_typeName":"PerfMetricSeriesCSV","id":{{"_typeName":"PerfMetricId","counterId":{counter},"instance":""}},"value":"4,5"}},
                    {{"_typeName":"PerfMetricSeriesCSV","id":{{"_typeName":"PerfMetricId","counterId":{counter},"instance":"vmnic0"}},"value":"6,7"}}
                ]
            }}]"#
        )
    };
    let other = Arc::new(
        CounterRegistry::from_catalog_json(
            r#"{
                "schema": "vim_rs.perf-catalog/1",
                "intervals": [],
                "counters": [
                    {"key": 9, "group": "cpu", "name": "usage", "rollup": "average", "stats": "rate", "unit": "percent", "level": 1, "per_device_level": 3, "labels": {"name": "Usage"}}
                ]
            }"#,
        )
        .unwrap(),
    );
    let first = decode_metrics(
        load(&json(2)),
        &registry(),
        &[],
        Some(20),
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap();
    let second = decode_metrics(
        load(&json(9)),
        &other,
        &[],
        Some(20),
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap();
    let metric = usage_metric();
    let left = first.entities().next().unwrap();
    let right = second.entities().next().unwrap();
    assert_eq!(left.series(metric).unwrap().raw(), &[4, 5]);
    assert_eq!(left.series(metric).unwrap().raw(), &[4, 5]);
    assert_eq!(
        left.series_instance(metric, &Instance::Named("vmnic0".into()))
            .unwrap()
            .raw(),
        &[6, 7]
    );
    assert_eq!(right.series(metric).unwrap().raw(), &[4, 5]);
    let mhz = Metric::<AnyDimension, AnyEntity>::new(
        "cpu",
        "usagemhz",
        Rollup::Average,
        Some(StatsKind::Rate),
        None,
        None,
        false,
    );
    assert!(left.series(mhz).is_none());
    assert!(left
        .series(Metric::custom("no", "such", Rollup::Average))
        .is_none());
    assert_eq!(
        left.devices(Metric::custom("no", "such", Rollup::Average))
            .count(),
        0
    );
    assert_eq!(
        left.devices(metric)
            .map(|series| series.raw().to_vec())
            .collect::<Vec<_>>(),
        vec![vec![6, 7]]
    );
    assert!(right
        .series(Metric::custom("no", "such", Rollup::Average))
        .is_none());
}

#[test]
fn sample_info_without_values_is_no_samples() {
    let json = r#"[{
        "_typeName":"PerfEntityMetricCSV",
        "entity":{"_typeName":"ManagedObjectReference","type":"VirtualMachine","value":"vm-1"},
        "sampleInfoCSV":"300,2026-10-24T10:00:00Z,300,2026-10-24T10:00:20Z",
        "value":[]
    }]"#;
    let result = decode_metrics(
        load(json),
        &registry(),
        &[],
        Some(20),
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap();
    let entity = result.entities().next().unwrap();
    assert!(matches!(entity.status, EntityStatus::NoSamples));
    assert!(entity.timestamps.is_empty());
    assert!(entity.series.is_empty());
    assert!(result.warnings.is_empty());
}

#[test]
fn empty_value_payload_does_not_extend_the_grid() {
    let json = r#"[
        {"_typeName":"PerfEntityMetricCSV","entity":{"_typeName":"ManagedObjectReference","type":"VirtualMachine","value":"vm-1"},
         "sampleInfoCSV":"20,2026-10-24T10:00:00Z,20,2026-10-24T10:00:20Z",
         "value":[{"_typeName":"PerfMetricSeriesCSV","id":{"_typeName":"PerfMetricId","counterId":2,"instance":""},"value":"1,2"}]},
        {"_typeName":"PerfEntityMetricCSV","entity":{"_typeName":"ManagedObjectReference","type":"VirtualMachine","value":"vm-1"},
         "sampleInfoCSV":"300,2026-10-24T10:00:40Z",
         "value":[]}
    ]"#;
    let result = decode_metrics(
        load(json),
        &registry(),
        &[],
        Some(20),
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap();
    let entity = result.entities().next().unwrap();
    assert!(matches!(entity.status, EntityStatus::Ok));
    assert_eq!(entity.timestamps.len(), 2);
    assert_eq!(entity.series[0].values, vec![1, 2]);
    assert!(result.warnings.is_empty());
}
