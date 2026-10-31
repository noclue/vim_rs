#![cfg(feature = "perf")]

//! Generated catalog constants against the sanitized vCenter catalog.
//!
//! `metrics.rs` names counters by `group.name.rollup` only. Counter ids differ
//! between endpoints, so ids are pinned against the fixture in a few named checks.

use std::sync::Arc;

use vim_rs::mo::{ClusterComputeResource, Datastore, VirtualMachine};
use vim_rs::perf::entity::{EntityScope, MASK_ALL};
use vim_rs::perf::metric::StatsKind;
use vim_rs::perf::metrics::{cpu, disk, mem};
use vim_rs::perf::registry::CounterRegistry;
use vim_rs::perf::typed::{Admits, Entity};
use vim_rs::perf::units::Dimension;
use vim_rs::perf::{Metric, PerfError};
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::structs::ManagedObjectReference;

const METRICS_RS: &str = include_str!("../src/perf/metrics.rs");
const CATALOG: &str = include_str!("fixtures/perf/vcenter_catalog.json");

fn scope_of<D: Dimension, S: EntityScope>(_: Metric<D, S>) -> u8 {
    S::MASK
}

#[test]
fn named_constants_resolve_and_exclude_some_kind() {
    let registry = CounterRegistry::from_catalog_json(CATALOG).unwrap();
    assert_eq!(registry.len(), 732);
    let usage = registry.resolve(&cpu::USAGE_AVERAGE).unwrap();
    assert_eq!(usage.id.0, 2);
    assert_eq!(scope_of(cpu::USAGE_AVERAGE), 0x07);
    assert_ne!(scope_of(cpu::USAGE_AVERAGE), MASK_ALL);
    assert_eq!(registry.resolve(&cpu::USAGE_MAXIMUM).unwrap().id.0, 4);
    assert_eq!(scope_of(cpu::USAGE_MAXIMUM), scope_of(cpu::USAGE_AVERAGE));
    assert_eq!(registry.resolve(&mem::SWAP_IN_AVERAGE).unwrap().id.0, 78);
    assert_eq!(registry.resolve(&mem::SWAPIN_AVERAGE).unwrap().id.0, 456);
    assert_eq!(registry.resolve(&mem::ZIP_SAVED_LATEST).unwrap().id.0, 389);
    assert_eq!(registry.resolve(&mem::ZIPSAVED_LATEST).unwrap().id.0, 471);
    assert_eq!(
        registry
            .by_name_with_stats("disk.scsiReservationCnflctsPct.average", StatsKind::Rate)
            .unwrap()
            .id
            .0,
        358
    );
    assert_eq!(
        disk::SCSI_RESERVATION_CNFLCTS_PCT_AVERAGE.full_name(),
        "disk.scsiReservationCnflctsPct.average"
    );
    assert_eq!(
        registry
            .resolve(&disk::SCSI_RESERVATION_CNFLCTS_PCT_AVERAGE_ABSOLUTE)
            .unwrap()
            .id
            .0,
        148
    );
}

#[test]
fn every_generated_constant_matches_the_catalog_and_is_not_any_entity() {
    let registry = Arc::new(CounterRegistry::from_catalog_json(CATALOG).unwrap());
    let consts = parse_consts(METRICS_RS);
    assert_eq!(consts.len(), 732);
    for item in consts {
        let stats = match item.stats.as_str() {
            "Rate" => StatsKind::Rate,
            "Absolute" => StatsKind::Absolute,
            "Delta" => StatsKind::Delta,
            other => panic!("unknown stats {other}"),
        };
        let found = registry
            .by_name_with_stats(&item.full, stats)
            .unwrap_or_else(|| {
                panic!("missing catalog counter {}", item.full);
            });
        assert_eq!(&*found.name.full, item.full);
        assert_ne!(item.mask, MASK_ALL, "{}", item.full);
        assert_ne!(item.mask, 0, "{}", item.full);
    }
}

fn admits_cluster<D: Dimension, S: EntityScope + Admits<ClusterComputeResource>>(_: Metric<D, S>) {}

#[test]
fn cluster_services_counters_are_cluster_scoped() {
    use vim_rs::perf::metrics::by_entity::cluster_compute_resource::cluster_services;
    // Compiles only if a typed cluster query accepts these constants.
    admits_cluster(cluster_services::CLUSTER_DRS_SCORE_LATEST);
    admits_cluster(cluster_services::EFFECTIVECPU_AVERAGE);
    admits_cluster(cluster_services::EFFECTIVEMEM_AVERAGE);
    admits_cluster(cluster_services::FAILOVER_LATEST);
    assert_eq!(scope_of(cluster_services::EFFECTIVECPU_AVERAGE), 0x04);
}

#[test]
fn datastore_listing_has_no_cpu_or_memory() {
    let by_entity = METRICS_RS.split_once("pub mod by_entity").unwrap().1;
    let datastore = by_entity
        .split_once("\n    pub mod datastore {\n")
        .unwrap()
        .1;
    let body = datastore.split_once("\n    }\n").unwrap().0;
    assert!(!body.contains("pub mod cpu"));
    assert!(!body.contains("pub mod mem"));
    assert!(body.contains("pub mod disk"));
}

#[test]
fn typed_entity_rejects_the_wrong_managed_object_type() {
    let vm = ManagedObjectReference {
        r#type: MoTypesEnum::VirtualMachine,
        value: "vm-1".into(),
    };
    let wrapped = Entity::<VirtualMachine>::try_from(vm).unwrap();
    let err = Entity::<Datastore>::try_from(wrapped.into_moref()).unwrap_err();
    assert!(matches!(
        err,
        PerfError::EntityKindMismatch {
            expected: MoTypesEnum::Datastore,
            actual: MoTypesEnum::VirtualMachine
        }
    ));
}

struct Parsed {
    full: String,
    stats: String,
    mask: u8,
}

fn parse_consts(text: &str) -> Vec<Parsed> {
    let mut out = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    let mut index = 0;
    while index < lines.len() {
        let line = lines[index].trim();
        // Name line: /// `group.name.rollup` · unit · endpoints
        if let Some(rest) = line
            .strip_prefix("/// `")
            .filter(|rest| rest.contains("` · "))
        {
            let full = rest.split('`').next().unwrap_or("").to_string();
            let mut scope = String::new();
            let mut stats = String::new();
            for follow in lines.iter().skip(index + 1).take(12) {
                let follow = follow.trim();
                if let Some(params) = follow.strip_prefix("pub const ") {
                    if let Some(ty) = params.split("Metric<").nth(1) {
                        scope = ty
                            .split('>')
                            .next()
                            .unwrap_or("")
                            .split(',')
                            .nth(1)
                            .unwrap_or("")
                            .trim()
                            .to_string();
                    }
                }
                if let Some(kind) = follow.strip_prefix("Some(StatsKind::") {
                    stats = kind.trim_end_matches([',', ')']).to_string();
                }
            }
            if !stats.is_empty() {
                out.push(Parsed {
                    full,
                    stats,
                    mask: scope_mask(&scope),
                });
            }
            index += 1;
            continue;
        }
        if line.starts_with("pub mod by_entity") {
            break;
        }
        index += 1;
    }
    out
}

fn scope_mask(scope: &str) -> u8 {
    match scope {
        "VmOnly" => 0x01,
        "HostOnly" => 0x02,
        "VmOrHost" => 0x03,
        "ClusterOnly" => 0x04,
        "HostCluster" => 0x06,
        "VmHostCluster" => 0x07,
        "VmOrPool" => 0x09,
        "ClusterPool" => 0x0C,
        "ComputeScope" => 0x0F,
        "DatastoreOnly" => 0x10,
        "DatastoreVm" => 0x11,
        "DatastoreVmHost" => 0x13,
        "ClusterDc" => 0x24,
        other => scope_flags_mask(other),
    }
}

/// `Scope<true, false, ...>` in flag order: VM, host, cluster, pool, datastore, datacenter.
fn scope_flags_mask(scope: &str) -> u8 {
    let Some(inner) = scope
        .strip_prefix("crate::perf::entity::Scope<")
        .and_then(|rest| rest.strip_suffix('>'))
    else {
        return 0x3F;
    };
    let bits = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20];
    inner.split(',').zip(bits).fold(0, |mask, (flag, bit)| {
        if flag.trim() == "true" {
            mask | bit
        } else {
            mask
        }
    })
}
