#![cfg(feature = "perf")]

//! Offline catalog registry tests. No vCenter connection.

use std::collections::HashMap;

use vim_rs::perf::entity::AnyEntity;
use vim_rs::perf::metric::{Metric, Rollup, StatsKind};
use vim_rs::perf::registry::CounterRegistry;
use vim_rs::perf::result::Timestamp;
use vim_rs::perf::units::Time;

const VCENTER: &str = include_str!("fixtures/perf/vcenter_catalog.json");
const ESXI: &str = include_str!("fixtures/perf/esxi_catalog.json");

fn vcenter() -> CounterRegistry {
    CounterRegistry::from_catalog_json(VCENTER).expect("vcenter catalog")
}

fn esxi() -> CounterRegistry {
    CounterRegistry::from_catalog_json(ESXI).expect("esxi catalog")
}

#[test]
fn catalogs_load_expected_counts() {
    let vc = vcenter();
    let host = esxi();
    assert_eq!(vc.len(), 732);
    assert_eq!(host.len(), 353);
    assert_eq!(vc.intervals().len(), 4);
    assert!(host.intervals().is_empty());
    assert!(host
        .iter()
        .all(|counter| counter.level.is_none() && counter.per_device_level.is_none()));
}

#[test]
fn vcenter_catalog_copies_are_byte_identical() {
    let build = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../vim_build/data/perf-catalog.json"
    ))
    .expect("generator catalog");
    let fixture = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/perf/vcenter_catalog.json"
    ))
    .expect("test catalog");
    assert_eq!(build, fixture);
}

#[test]
fn name_split_and_stats_preference() {
    let registry = vcenter();

    let contention = registry
        .by_name("cpu.capacity.contention.average")
        .expect("capacity.contention");
    assert_eq!(contention.id.0, 19);
    assert_eq!(contention.name.group.as_ref(), "cpu");
    assert_eq!(contention.name.name.as_ref(), "capacity.contention");
    assert_eq!(contention.name.rollup.as_ref(), "average");
    assert_eq!(contention.stats.as_str(), "rate");

    let zip_dotted = registry.by_name("mem.zip.saved.latest").expect("zip.saved");
    assert_eq!(zip_dotted.id.0, 389);
    let zip_camel = registry.by_name("mem.zipSaved.latest").expect("zipSaved");
    assert_eq!(zip_camel.id.0, 471);

    let scsi = registry
        .by_name("disk.scsiReservationCnflctsPct.average")
        .expect("scsi bare name prefers rate");
    assert_eq!(scsi.id.0, 358);
    assert_eq!(scsi.stats.as_str(), "rate");
    let scsi_absolute = registry
        .by_name_with_stats(
            "disk.scsiReservationCnflctsPct.average",
            StatsKind::Absolute,
        )
        .expect("scsi absolute");
    assert_eq!(scsi_absolute.id.0, 148);
    let scsi_metric = Metric::<Time, AnyEntity>::new(
        "disk",
        "scsiReservationCnflctsPct",
        Rollup::Average,
        Some(StatsKind::Absolute),
        None,
        None,
        false,
    );
    assert_eq!(registry.resolve(&scsi_metric).unwrap().id.0, 148);
    let scsi_rate = Metric::<Time, AnyEntity>::new(
        "disk",
        "scsiReservationCnflctsPct",
        Rollup::Average,
        Some(StatsKind::Rate),
        None,
        None,
        false,
    );
    assert_eq!(registry.resolve(&scsi_rate).unwrap().id.0, 358);

    let ready = registry.by_name("cpu.ready.summation").expect("ready");
    assert_eq!(ready.id.0, 12);
    assert_eq!(ready.stats.as_str(), "delta");

    let metric = Metric::<Time, AnyEntity>::new(
        "cpu",
        "ready",
        Rollup::Summation,
        None,
        Some(1),
        Some(3),
        true,
    );
    assert_eq!(registry.resolve(&metric).unwrap().id.0, 12);
}

#[test]
fn scsi_pair_is_the_only_duplicate_full_name() {
    let registry = vcenter();
    let mut kinds: HashMap<String, Vec<&str>> = HashMap::new();
    for counter in registry.iter() {
        kinds
            .entry(counter.name.full.to_string())
            .or_default()
            .push(counter.stats.as_str());
    }
    let multi: Vec<_> = kinds
        .iter()
        .filter(|(_, stats)| stats.len() > 1)
        .map(|(name, _)| name.as_str())
        .collect();
    assert_eq!(multi, ["disk.scsiReservationCnflctsPct.average"]);
}

#[test]
fn generations_are_unique() {
    let first = vcenter().generation();
    let second = esxi().generation();
    assert_ne!(first, second);
    assert!(first >= 1);
    assert!(second >= 1);
}

#[test]
fn timestamp_rfc3339_round_trip() {
    let parsed = Timestamp::parse_rfc3339("2026-10-24T10:48:20Z").unwrap();
    assert_eq!(parsed.to_rfc3339_utc(), "2026-10-24T10:48:20Z");
    let offset = Timestamp::parse_rfc3339("2026-10-24T13:48:20+03:00").unwrap();
    assert_eq!(offset, parsed);
    let fraction = Timestamp::parse_rfc3339("2026-10-24T10:48:20.5Z").unwrap();
    assert_eq!(fraction.to_rfc3339_utc(), "2026-10-24T10:48:20.500000000Z");
}

#[test]
fn duplicate_catalog_identity_is_an_error() {
    let json = r#"{
        "schema": "vim_rs.perf-catalog/1",
        "intervals": [],
        "counters": [
            {"key": 1, "group": "cpu", "name": "usage", "rollup": "average", "stats": "rate", "unit": "percent"},
            {"key": 2, "group": "cpu", "name": "usage", "rollup": "average", "stats": "rate", "unit": "percent"}
        ]
    }"#;
    assert!(CounterRegistry::from_catalog_json(json).is_err());
}

#[test]
fn bare_name_fails_when_rate_is_absent_and_two_kinds_exist() {
    let json = r#"{
        "schema": "vim_rs.perf-catalog/1",
        "intervals": [],
        "counters": [
            {"key": 1, "group": "cpu", "name": "usage", "rollup": "average", "stats": "absolute", "unit": "percent"},
            {"key": 2, "group": "cpu", "name": "usage", "rollup": "average", "stats": "delta", "unit": "percent"}
        ]
    }"#;
    let registry = CounterRegistry::from_catalog_json(json).unwrap();
    assert!(registry.by_name("cpu.usage.average").is_none());
    assert_eq!(
        registry
            .by_name_with_stats("cpu.usage.average", StatsKind::Delta)
            .unwrap()
            .id
            .0,
        2
    );
}

#[test]
fn stats_token_case_hits_the_same_counter() {
    let json = r#"{
        "schema": "vim_rs.perf-catalog/1",
        "intervals": [],
        "counters": [
            {"key": 7, "group": "cpu", "name": "usage", "rollup": "Average", "stats": "Rate", "unit": "percent"}
        ]
    }"#;
    let registry = CounterRegistry::from_catalog_json(json).unwrap();
    let metric = Metric::<Time, AnyEntity>::new(
        "cpu",
        "usage",
        Rollup::Average,
        Some(StatsKind::Rate),
        None,
        None,
        false,
    );
    assert_eq!(registry.resolve(&metric).unwrap().id.0, 7);
    assert_eq!(
        registry
            .by_name_with_stats("cpu.usage.average", StatsKind::Rate)
            .unwrap()
            .id
            .0,
        7
    );
    assert_eq!(registry.by_name("cpu.usage.AVERAGE").unwrap().id.0, 7);
}

#[test]
fn a_name_past_the_stack_buffer_resolves_the_same_way() {
    let name: &'static str = Box::leak("n".repeat(600).into_boxed_str());
    let json = format!(
        r#"{{
            "schema": "vim_rs.perf-catalog/1",
            "intervals": [],
            "counters": [
                {{"key": 4, "group": "cpu", "name": "{name}", "rollup": "average", "stats": "Rate", "unit": "percent"}}
            ]
        }}"#
    );
    let registry = CounterRegistry::from_catalog_json(&json).unwrap();
    let metric = Metric::<Time, AnyEntity>::new(
        "cpu",
        name,
        Rollup::Average,
        Some(StatsKind::Rate),
        None,
        None,
        false,
    );
    assert_eq!(registry.resolve(&metric).unwrap().id.0, 4);
    assert_eq!(
        registry
            .by_name_with_stats(&format!("cpu.{name}.average"), StatsKind::Rate)
            .unwrap()
            .id
            .0,
        4
    );
}
