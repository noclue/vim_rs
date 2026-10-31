//! Dummy `ServiceContent` for offline mock grading.

use vim_rs::types::structs::ServiceContent;

pub fn dummy_service_content() -> ServiceContent {
    let json = r#"{
        "_typeName": "ServiceContent",
        "rootFolder": {"_typeName":"ManagedObjectReference","type":"Folder","value":"root-1"},
        "propertyCollector": {"_typeName":"ManagedObjectReference","type":"PropertyCollector","value":"pc-1"},
        "viewManager": {"_typeName":"ManagedObjectReference","type":"ViewManager","value":"vmgr-1"},
        "perfManager": {"_typeName":"ManagedObjectReference","type":"PerformanceManager","value":"PerfMgr"},
        "about": {
            "_typeName":"AboutInfo",
            "name":"eval",
            "fullName":"eval-harness",
            "vendor":"vim-rs",
            "version":"1",
            "build":"1",
            "osType":"linux",
            "productLineId":"eval",
            "apiType":"VirtualCenter",
            "apiVersion":"9.0.0.0"
        }
    }"#;
    miniserde::json::from_str(json).expect("dummy ServiceContent should deserialize")
}
