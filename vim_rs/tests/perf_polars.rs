//! Columnar export. Compiled only with `--features perf-polars`.

#![cfg(feature = "perf-polars")]

use std::sync::Arc;
use std::time::UNIX_EPOCH;

use vim_rs::core::client::{unmarshal_array, Transport};
use vim_rs::perf::decode::decode_metrics;
use vim_rs::perf::registry::CounterRegistry;
use vim_rs::types::traits::PerfEntityMetricBaseTrait;

fn registry() -> Arc<CounterRegistry> {
    Arc::new(
        CounterRegistry::from_catalog_json(
            r#"{
                "schema": "vim_rs.perf-catalog/1",
                "intervals": [],
                "counters": [
                    {"key": 2, "group": "cpu", "name": "usage", "rollup": "average", "stats": "rate", "unit": "percent", "level": 1, "per_device_level": 3, "labels": {"name": "Usage"}}
                ]
            }"#,
        )
        .unwrap(),
    )
}

#[test]
fn to_dataframe_drops_gaps_and_names_row_columns() {
    let json = r#"[
        {
            "_typeName": "PerfEntityMetric",
            "entity": {"_typeName": "ManagedObjectReference", "type": "VirtualMachine", "value": "vm-1"},
            "sampleInfo": [
                {"_typeName": "PerfSampleInfo", "timestamp": "2026-10-24T10:00:00Z", "interval": 20},
                {"_typeName": "PerfSampleInfo", "timestamp": "2026-10-24T10:00:20Z", "interval": 20}
            ],
            "value": [
                {"_typeName": "PerfMetricIntSeries", "id": {"_typeName": "PerfMetricId", "counterId": 2, "instance": ""}, "value": [5, -1]}
            ]
        }
    ]"#;
    let raw: Vec<Box<dyn PerfEntityMetricBaseTrait>> =
        unmarshal_array(Transport::Json, json.as_bytes()).unwrap();
    let result = decode_metrics(raw, &registry(), &[], None, UNIX_EPOCH).unwrap();
    let frame = result.to_dataframe().unwrap();
    assert_eq!(frame.height(), 1);
    let names: Vec<_> = frame
        .get_column_names()
        .iter()
        .map(|name| name.as_str().to_string())
        .collect();
    assert_eq!(
        names,
        [
            "entity",
            "counter",
            "instance",
            "timestamp",
            "interval",
            "raw",
            "scaled"
        ]
    );
}
