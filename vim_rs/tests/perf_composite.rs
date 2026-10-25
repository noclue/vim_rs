#![cfg(feature = "perf")]

//! Composite host-and-children queries.

use std::sync::Arc;

use vim_rs::core::client::{unmarshal, Transport, VimClientHandle};
use vim_rs::perf::decode::decode_composite;
use vim_rs::perf::registry::CounterRegistry;
use vim_rs::perf::{EntityStatus, PerfClient, PerfError};
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::structs::{ManagedObjectReference, PerfCompositeMetric};

#[path = "support/perf_mock.rs"]
mod perf_mock;

use perf_mock::PerfMockClient;

const COMPOSITE: &str = include_str!("fixtures/perf/query_composite.json");

fn registry() -> Arc<CounterRegistry> {
    Arc::new(
        CounterRegistry::from_catalog_json(
            r#"{
                "schema": "vim_rs.perf-catalog/1",
                "intervals": [{"key": 1, "sampling_period": 20, "name": "Realtime", "length": 3600, "level": 1, "enabled": true}],
                "counters": [
                    {"key": 2, "group": "cpu", "name": "usage", "rollup": "average", "stats": "rate", "unit": "percent", "level": 1, "per_device_level": 3, "labels": {"name": "Usage"}}
                ]
            }"#,
        )
        .unwrap(),
    )
}

#[test]
fn composite_fixture_decodes_host_and_two_children() {
    let metric: PerfCompositeMetric = unmarshal(Transport::Json, COMPOSITE.as_bytes()).unwrap();
    let decoded = decode_composite(
        metric.entity,
        metric.child_entity.unwrap_or_default(),
        &registry(),
        Some(20),
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap();
    let host = decoded.host.expect("host payload");
    assert_eq!(host.entity.value, "host-1001");
    assert!(matches!(host.status, EntityStatus::Ok));
    assert_eq!(host.series[0].values, vec![10]);
    assert_eq!(decoded.children.len(), 2);
    let values: Vec<_> = decoded
        .children
        .values()
        .map(|child| child.series[0].values[0])
        .collect();
    assert_eq!(values, vec![20, 30]);
}

#[tokio::test]
async fn composite_rejects_a_non_host_before_the_call() {
    let mock = PerfMockClient::new();
    let handle: VimClientHandle = mock.clone();
    let perf = PerfClient::new(handle).unwrap();
    let vm = ManagedObjectReference {
        r#type: MoTypesEnum::VirtualMachine,
        value: "vm-1".into(),
    };
    let err = perf
        .composite(&vm)
        .metric_by_name("cpu.usage.average")
        .interval(vim_rs::perf::Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap_err();
    assert!(matches!(err, PerfError::EntityKindMismatch { .. }));
    assert!(mock.composites().is_empty());
}

#[tokio::test]
async fn composite_sends_normal_format_without_max_sample() {
    let mock = PerfMockClient::new();
    let handle: VimClientHandle = mock.clone();
    let perf = PerfClient::new(handle).unwrap();
    let host = ManagedObjectReference {
        r#type: MoTypesEnum::HostSystem,
        value: "host-1".into(),
    };
    perf.composite(&host)
        .metric_by_name("cpu.usage.average")
        .interval(vim_rs::perf::Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    let spec = &mock.composites()[0];
    assert_eq!(spec.format.as_deref(), Some("normal"));
    assert!(spec.max_sample.is_none());
    assert_eq!(spec.entity, "host-1");
}

#[tokio::test]
async fn composite_latest_truncates_without_sending_max_sample() {
    let mock = PerfMockClient::new();
    mock.set_composite_sample_count(3);
    let handle: VimClientHandle = mock.clone();
    let perf = PerfClient::new(handle).unwrap();
    let host = ManagedObjectReference {
        r#type: MoTypesEnum::HostSystem,
        value: "host-1".into(),
    };
    let result = perf
        .composite(&host)
        .metric_by_name("cpu.usage.average")
        .interval(vim_rs::perf::Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();
    let spec = &mock.composites()[0];
    assert!(spec.max_sample.is_none());
    let host_series = result.host.expect("host payload");
    assert_eq!(host_series.series[0].values, vec![3]);
    let child = result.children.values().next().expect("child");
    assert_eq!(child.series[0].values, vec![3]);
}
