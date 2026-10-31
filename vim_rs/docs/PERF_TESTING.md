# Testing performance code

`vim_rs::perf` has no mock of `PerfClient`. Two public seams cover the tests that do not need a live vCenter or ESXi host. Enable the `perf` feature on the `vim_rs` dependency the test compiles against. A dev-dependency is enough: Cargo turns the feature on for the whole crate.

```toml
[dev-dependencies]
vim_rs = { version = "0.7", features = ["perf"] }
tokio = { version = "1", features = ["macros", "rt"] }
miniserde = "0.1"
bytes = "1"
```

The scripted client used by this crate's own tests is `vim_rs/tests/support/perf_mock.rs`. It is not part of the published API.

## Reading samples

`PerfResult` has no public constructor. `CounterRegistry::from_catalog_json` and [`decode::decode_metrics`](https://docs.rs/vim_rs/latest/vim_rs/perf/decode/fn.decode_metrics.html) are the same decoder a fetch uses. A catalog with one counter is enough. The numeric `key` is whatever the test chooses. A generated constant such as `cpu::USAGE_AVERAGE` resolves by name against that catalog, then matches the series whose `counterId` is that key.

A raw value below `0` is a gap. `last()` skips trailing gaps. `last_slot()` reads only the newest timestamp, so a trailing gap is blank. `rows()` and `SampleWatermark::filter_new` skip gaps. `filter_new` advances its marks before it yields, so a second pass over the same result is empty.

```rust
use std::sync::Arc;

use vim_rs::perf::decode::decode_metrics;
use vim_rs::perf::metrics::cpu;
use vim_rs::perf::registry::CounterRegistry;
use vim_rs::perf::SampleWatermark;
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::structs::{
    ManagedObjectReference, PerfEntityMetric, PerfEntityMetricBase, PerfMetricId,
    PerfMetricIntSeries, PerfMetricSeries, PerfSampleInfo,
};

const CATALOG: &str = r#"{
  "schema": "vim_rs.perf-catalog/1",
  "intervals": [{
    "key": 1, "sampling_period": 300, "name": "Past day",
    "length": 86400, "level": 1, "enabled": true
  }],
  "counters": [{
    "key": 2, "group": "cpu", "name": "usage", "rollup": "average",
    "stats": "rate", "unit": "percent", "level": 1,
    "labels": { "name": "Usage" }
  }]
}"#;

fn vm() -> ManagedObjectReference {
    ManagedObjectReference {
        r#type: MoTypesEnum::VirtualMachine,
        value: "vm-1".into(),
    }
}

#[tokio::test]
async fn scaled_sample_and_watermark() {
    let registry = Arc::new(CounterRegistry::from_catalog_json(CATALOG).unwrap());
    let series = PerfMetricIntSeries {
        perf_metric_series_: PerfMetricSeries {
            id: PerfMetricId {
                counter_id: 2,
                instance: String::new(),
            },
        },
        // 10000 is 100.00%. The trailing -1 is a gap.
        value: Some(vec![10_000, -1]),
    };
    let metric = PerfEntityMetric {
        perf_entity_metric_base_: PerfEntityMetricBase { entity: vm() },
        sample_info: Some(vec![
            PerfSampleInfo {
                timestamp: "2026-10-24T12:00:00Z".into(),
                interval: 20,
            },
            PerfSampleInfo {
                timestamp: "2026-10-24T12:00:20Z".into(),
                interval: 20,
            },
        ]),
        value: Some(vec![Box::new(series)]),
    };
    let result = decode_metrics(
        vec![Box::new(metric)],
        &registry,
        &[vm()],
        Some(20),
        std::time::SystemTime::UNIX_EPOCH,
    )
    .unwrap();

    let cpu = result
        .entity(&vm())
        .unwrap()
        .series(cpu::USAGE_AVERAGE)
        .unwrap();
    assert_eq!(cpu.scaled().last().unwrap().value.0, 100.0);
    assert!(cpu.scaled().last_slot().is_none());

    let mut marks = SampleWatermark::new();
    let rows: Vec<_> = marks.filter_new(&result).collect();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].raw, 10_000);
    assert_eq!(marks.filter_new(&result).count(), 0);
}
```

`decode::decode_composite` does the same for a host payload plus the children the server returned. [`PerfClient::decode`](https://docs.rs/vim_rs/latest/vim_rs/perf/struct.PerfClient.html#method.decode) is the same function after the client has loaded its registry from the connection.

## Issuing a query

`PerfClient::new` takes `Arc<dyn VimClient>`. A test double implements that trait and answers `PerformanceManager`. `service_content().perf_manager` has to be set, or `new` returns `NoPerformanceManager`. `about.apiType` is `VirtualCenter` or `HostAgent`.

| Call | `VimClient` method | When |
|---|---|---|
| `perfCounter`, `historicalInterval` | `fetch_property_raw` | First `registry()` |
| `QueryPerfProviderSummary` | `invoke` | First use of each managed-object kind |
| `QueryPerf` | `invoke_optional` | Every fetch |
| `QueryAvailablePerfMetric` | `invoke_optional` | `.groups()` and `.all_available()` only |
| `QueryPerfComposite` | `invoke` | `composite()` only |

Property and method bodies are JSON. `Transport::Json` is the straightforward choice. `QueryAvailablePerfMetric` is not given the query window, and the client caches the list until `refresh_registry`. A test of `.groups()` returns the instances it wants that call to see. `QueryPerfComposite` never sends `maxSample`.

`miniserde::json::to_string` on the `params` argument is the `QueryPerf` body the client sent. Realtime `latest(1)` puts `"intervalId":20` and `"maxSample":1` in that body. `PerfClient::set_clock` pins `since_ago` and historical windows to a chosen instant. The method is hidden from rustdoc and is there for tests.

```rust
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use vim_rs::core::client::{self, PropertyValue, Transport, VimClient};
use vim_rs::perf::metrics::cpu;
use vim_rs::perf::{EntityStatus, Interval, PerfClient};
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::structs::{ManagedObjectReference, ServiceContent};

const CONTENT: &str = r#"{
  "_typeName": "ServiceContent",
  "rootFolder": {"_typeName":"ManagedObjectReference","type":"Folder","value":"root-1"},
  "propertyCollector": {"_typeName":"ManagedObjectReference","type":"PropertyCollector","value":"pc-1"},
  "viewManager": {"_typeName":"ManagedObjectReference","type":"ViewManager","value":"vmgr-1"},
  "perfManager": {"_typeName":"ManagedObjectReference","type":"PerformanceManager","value":"PerfMgr"},
  "about": {
    "_typeName": "AboutInfo",
    "name": "n", "fullName": "f", "vendor": "v", "version": "1", "build": "b",
    "osType": "o", "productLineId": "p", "apiType": "VirtualCenter", "apiVersion": "9.0.0.0"
  }
}"#;

const COUNTERS: &str = r#"[{
  "_typeName": "PerfCounterInfo", "key": 2,
  "nameInfo": {"_typeName":"ElementDescription","key":"usage","label":"Usage","summary":"Usage"},
  "groupInfo": {"_typeName":"ElementDescription","key":"cpu","label":"cpu","summary":"cpu"},
  "unitInfo": {"_typeName":"ElementDescription","key":"percent","label":"percent","summary":"percent"},
  "rollupType": "average", "statsType": "rate", "level": 1
}]"#;

const INTERVALS: &str = r#"[{
  "_typeName": "PerfInterval", "key": 1, "samplingPeriod": 300,
  "name": "Past day", "length": 86400, "level": 1, "enabled": true
}]"#;

const SUMMARY: &str = r#"{
  "_typeName": "PerfProviderSummary",
  "entity": {"_typeName":"ManagedObjectReference","type":"VirtualMachine","value":"vm-1"},
  "currentSupported": true, "summarySupported": true, "refreshRate": 20
}"#;

const SAMPLES: &str = r#"[{
  "_typeName": "PerfEntityMetric",
  "entity": {"_typeName":"ManagedObjectReference","type":"VirtualMachine","value":"vm-1"},
  "sampleInfo": [{
    "_typeName": "PerfSampleInfo", "timestamp": "2026-10-24T12:00:00Z", "interval": 20
  }],
  "value": [{
    "_typeName": "PerfMetricIntSeries",
    "id": {"_typeName":"PerfMetricId","counterId":2,"instance":""},
    "value": [10000]
  }]
}]"#;

struct ScriptedPerf {
    content: ServiceContent,
    queries: Mutex<Vec<String>>,
}

impl ScriptedPerf {
    fn new() -> Self {
        Self {
            content: miniserde::json::from_str(CONTENT).unwrap(),
            queries: Mutex::new(Vec::new()),
        }
    }
}

impl VimClient for ScriptedPerf {
    fn service_content(&self) -> &ServiceContent {
        &self.content
    }

    fn transport(&self) -> Transport {
        Transport::Json
    }

    fn api_release(&self) -> String {
        "9.1.0.0".into()
    }

    fn invoke<'a>(
        &'a self,
        _svc: &'a str,
        _mo_type: &'a str,
        _mo_id: &'a str,
        method_name: &'a str,
        _params: Option<&'a (dyn miniserde::Serialize + Send + Sync)>,
    ) -> client::BoxFuture<'a, client::Result<Bytes>> {
        match method_name {
            "QueryPerfProviderSummary" => Box::pin(async { Ok(Bytes::from(SUMMARY)) }),
            other => {
                let other = other.to_string();
                Box::pin(async move {
                    Err(client::Error::ParseError(format!("unhandled {other}")))
                })
            }
        }
    }

    fn invoke_optional<'a>(
        &'a self,
        _svc: &'a str,
        _mo_type: &'a str,
        _mo_id: &'a str,
        method_name: &'a str,
        params: Option<&'a (dyn miniserde::Serialize + Send + Sync)>,
    ) -> client::BoxFuture<'a, client::Result<Option<Bytes>>> {
        match method_name {
            "QueryPerf" => {
                let text = params.map(miniserde::json::to_string).unwrap_or_default();
                self.queries.lock().unwrap().push(text);
                Box::pin(async { Ok(Some(Bytes::from(SAMPLES))) })
            }
            other => {
                let other = other.to_string();
                Box::pin(async move {
                    Err(client::Error::ParseError(format!("unhandled {other}")))
                })
            }
        }
    }

    fn invoke_void<'a>(
        &'a self,
        _svc: &'a str,
        _mo_type: &'a str,
        _mo_id: &'a str,
        method_name: &'a str,
        _params: Option<&'a (dyn miniserde::Serialize + Send + Sync)>,
    ) -> client::BoxFuture<'a, client::Result<()>> {
        let method_name = method_name.to_string();
        Box::pin(async move {
            Err(client::Error::ParseError(format!("unhandled {method_name}")))
        })
    }

    fn fetch_property_raw<'a>(
        &'a self,
        _svc: &'a str,
        _mo_type: &'a str,
        _mo_id: &'a str,
        property: &'a str,
    ) -> client::BoxFuture<'a, client::Result<Option<PropertyValue>>> {
        let body = match property {
            "perfCounter" => COUNTERS,
            "historicalInterval" => INTERVALS,
            other => {
                let other = other.to_string();
                return Box::pin(async move {
                    Err(client::Error::ParseError(format!("unhandled {other}")))
                });
            }
        };
        Box::pin(async move { Ok(Some(PropertyValue::Json(Bytes::from(body)))) })
    }
}

fn vm() -> ManagedObjectReference {
    ManagedObjectReference {
        r#type: MoTypesEnum::VirtualMachine,
        value: "vm-1".into(),
    }
}

#[tokio::test]
async fn latest_sample_records_the_query() {
    let scripted = Arc::new(ScriptedPerf::new());
    let perf = PerfClient::new(scripted.clone()).unwrap();
    let result = perf
        .query()
        .entity(&vm())
        .metric(cpu::USAGE_AVERAGE)
        .interval(Interval::Realtime)
        .latest(1)
        .fetch()
        .await
        .unwrap();

    assert!(matches!(
        result.entity(&vm()).unwrap().status,
        EntityStatus::Ok
    ));
    let cpu = result
        .entity(&vm())
        .unwrap()
        .series(cpu::USAGE_AVERAGE)
        .unwrap();
    assert_eq!(cpu.scaled().last().unwrap().value.0, 100.0);

    let queries = scripted.queries.lock().unwrap();
    let body = queries.last().unwrap();
    assert!(body.contains("\"intervalId\":20"));
    assert!(body.contains("\"maxSample\":1"));
}
```
