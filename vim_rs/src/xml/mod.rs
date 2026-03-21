/// XML serialization and deserialization for miniserde types.
///
/// Follows serde-xml-rs conventions:
///   - Map keys starting with `@` are XML attributes
///   - Map key `#text` is text content of an element
///   - Namespace prefixes are part of the name (e.g., `soapenv:Body`)
///   - Sequences produce repeated child elements
///
/// This enables miniserde types to round-trip through XML without any
/// special derive macros — the same `Serialize`/`Deserialize` impls
/// that work with JSON work here, just with field-name conventions.
pub(crate) mod client;
pub mod soap;
pub mod ser;
pub mod de;


#[cfg(all(test, feature = "xml"))]
mod tests {
    use super::{de::from_xml, soap};
    use crate::types::boxed_types::ValueElements;
    use crate::types::enums::MoTypesEnum;
    use crate::types::struct_enum::StructType;
    use crate::types::structs::{Event, ManagedObjectReference, MethodFault, UpdateSet};
    use crate::types::vim_any::VimAny;
    use miniserde::json::{Number, Value};

    /// Minimal XML for empty ArrayOfX - the pattern that causes Property Explorer to fail.
    /// From vtui.log: changeSet with val xsi:type="ArrayOfCustomFieldDef" and no children.
    const EMPTY_ARRAY_VAL: &str = r#"<val xsi:type="ArrayOfCustomFieldDef"></val>"#;

    /// Minimal XML for empty ArrayOfX - the pattern that causes Property Explorer to fail.
    /// From vtui.log: changeSet with val xsi:type="ArrayOfCustomFieldDef" and no children.
    const EMPTY_ARRAY_VAL_WITH_NS: &str = r#"<val xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ArrayOfCustomFieldDef"></val>"#;


    /// Successful WaitForUpdatesEx response from vtui.log (VM list view).
    const WAIT_FOR_UPDATES_EX_SUCCESS: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<soapenv:Envelope xmlns:soapenc="http://schemas.xmlsoap.org/soap/encoding/"
 xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/"
 xmlns:xsd="http://www.w3.org/2001/XMLSchema"
 xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
<soapenv:Body>
<WaitForUpdatesExResponse xmlns="urn:vim25"><returnval><version>1</version><filterSet><filter type="PropertyFilter">session[521da002-3565-aed7-bfc9-a37573136c0e]52b39f40-3106-c12b-715c-bc09e1862cbf</filter><objectSet><kind>enter</kind><obj type="VirtualMachine">1</obj><changeSet><name>name</name><op>assign</op><val xsi:type="xsd:string">NFS server</val></changeSet><changeSet><name>overallStatus</name><op>assign</op><val xsi:type="ManagedEntityStatus">green</val></changeSet><changeSet><name>runtime.powerState</name><op>assign</op><val xsi:type="VirtualMachinePowerState">poweredOff</val></changeSet></objectSet></filterSet></returnval></WaitForUpdatesExResponse>
</soapenv:Body>
</soapenv:Envelope>"#;

    #[test]
    fn test_mor_deserialize() {
        let moref: ManagedObjectReference =
            from_xml(r#"<val type="VirtualMachine">5-envmgr</val>"#)
                .expect("ManagedObjectReference deserialize failed.");
        assert_eq!(moref.r#type, MoTypesEnum::VirtualMachine);
        assert_eq!(moref.value, "5-envmgr");
    }

    #[test]
    fn test_vim_any_from_moref() {
        let vim_any: VimAny =
            // Easy mode - no buffering, just set the mor flags
            //from_xml(r#"<val xsi:type="ManagedObjectReference" type="Folder">ha-folder-vm</val>"#)
            // This is the real test that is hard as it requires buffering
            from_xml(r#"<val type="Folder" xsi:type="ManagedObjectReference">ha-folder-vm</val>"#)
                .expect("ManagedObjectReference deserialize failed.");
        match &vim_any {
            VimAny::Object(v) => {
                let moref: Option<&ManagedObjectReference> = v.as_any_ref().downcast_ref();
                assert!(moref.is_some());
                assert_eq!(moref.unwrap().r#type, MoTypesEnum::Folder);
                assert_eq!(moref.unwrap().value, "ha-folder-vm");
            }
            other => panic!("expected ManagedObjectReference, got {:?}", other),
        }
    }

    /// Minimal test: empty ArrayOfCustomFieldDef must deserialize to VimAny::Value(ArrayOfCustomFieldDef(vec![])).
    #[test]
    fn test_empty_array_val_deserialize() {
        let vim_any: VimAny =
            from_xml(EMPTY_ARRAY_VAL).expect("empty ArrayOfCustomFieldDef should deserialize");
        match &vim_any {
            VimAny::Value(ValueElements::ArrayOfCustomFieldDef(v)) => assert!(v.is_empty()),
            other => panic!("expected ArrayOfCustomFieldDef, got {:?}", other),
        }
    }

    #[test]
    fn test_empty_array_val_with_ns_deserialize() {
        let vim_any: VimAny = from_xml(EMPTY_ARRAY_VAL_WITH_NS)
            .expect("empty ArrayOfCustomFieldDef should deserialize");
        match &vim_any {
            VimAny::Value(ValueElements::ArrayOfCustomFieldDef(v)) => assert!(v.is_empty()),
            other => panic!("expected ArrayOfCustomFieldDef, got {:?}", other),
        }
    }

    #[test]
    fn test_wait_for_updates_ex_success_deserialize() {
        // VM list view response - parses successfully.
        let update_set: UpdateSet = soap::vim_response(WAIT_FOR_UPDATES_EX_SUCCESS).unwrap();
        assert_eq!(update_set.version, "1");
        let filter_set = update_set.filter_set.as_ref().unwrap();
        assert_eq!(filter_set.len(), 1);
        assert_eq!(
            filter_set[0].filter.value,
            "session[521da002-3565-aed7-bfc9-a37573136c0e]52b39f40-3106-c12b-715c-bc09e1862cbf"
        );
    }

    /// Minimal UpdateSet with only the empty-array changeSet.
    const WAIT_FOR_UPDATES_EX_EMPTY_ARRAY_ONLY: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<soapenv:Envelope xmlns:soapenc="http://schemas.xmlsoap.org/soap/encoding/"
 xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/"
 xmlns:xsd="http://www.w3.org/2001/XMLSchema"
 xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
<soapenv:Body>
<WaitForUpdatesExResponse xmlns="urn:vim25"><returnval><version>2</version><filterSet><filter type="PropertyFilter">x</filter><objectSet><kind>enter</kind><obj type="VirtualMachine">7</obj><changeSet><name>availableField</name><op>assign</op><val xsi:type="ArrayOfCustomFieldDef"></val></changeSet></objectSet></filterSet></returnval></WaitForUpdatesExResponse>
</soapenv:Body>
</soapenv:Envelope>"#;

    #[test]
    fn test_wait_for_updates_ex_empty_array_only() {
        // Just the empty ArrayOfCustomFieldDef changeSet in full UpdateSet structure.
        let result: Result<UpdateSet, _> = soap::vim_response(WAIT_FOR_UPDATES_EX_EMPTY_ARRAY_ONLY);
        let update_set =
            result.expect("UpdateSet with only empty-array changeSet should deserialize");
        assert_eq!(update_set.version, "2");
    }

    // --- Property Explorer case breakdown: run with `cargo test --features xml property_explorer -- --ignored` ---

    /// Case 1: changeSet with no val element (alarmActionsEnabled).
    const PROP_EXPLORER_ALARM_ONLY: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<soapenv:Envelope xmlns:soapenc="http://schemas.xmlsoap.org/soap/encoding/"
 xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/"
 xmlns:xsd="http://www.w3.org/2001/XMLSchema"
 xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
<soapenv:Body>
<WaitForUpdatesExResponse xmlns="urn:vim25"><returnval><version>2</version><filterSet><filter type="PropertyFilter">x</filter><objectSet><kind>enter</kind><obj type="VirtualMachine">7</obj><changeSet><name>alarmActionsEnabled</name><op>assign</op></changeSet></objectSet></filterSet></returnval></WaitForUpdatesExResponse>
</soapenv:Body>
</soapenv:Envelope>"#;

    /// Case 2: empty ArrayOfCustomFieldDef (availableField).
    const PROP_EXPLORER_EMPTY_ARRAY_ONLY: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<soapenv:Envelope xmlns:soapenc="http://schemas.xmlsoap.org/soap/encoding/"
 xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/"
 xmlns:xsd="http://www.w3.org/2001/XMLSchema"
 xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
<soapenv:Body>
<WaitForUpdatesExResponse xmlns="urn:vim25"><returnval><version>2</version><filterSet><filter type="PropertyFilter">x</filter><objectSet><kind>enter</kind><obj type="VirtualMachine">7</obj><changeSet><name>availableField</name><op>assign</op><val xsi:type="ArrayOfCustomFieldDef"></val></changeSet></objectSet></filterSet></returnval></WaitForUpdatesExResponse>
</soapenv:Body>
</soapenv:Envelope>"#;

    /// Case 3: alarmActionsEnabled + availableField (no val + empty array).
    const PROP_EXPLORER_ALARM_AND_EMPTY_ARRAY: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<soapenv:Envelope xmlns:soapenc="http://schemas.xmlsoap.org/soap/encoding/"
 xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/"
 xmlns:xsd="http://www.w3.org/2001/XMLSchema"
 xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
<soapenv:Body>
<WaitForUpdatesExResponse xmlns="urn:vim25"><returnval><version>2</version><filterSet><filter type="PropertyFilter">x</filter><objectSet><kind>enter</kind><obj type="VirtualMachine">7</obj><changeSet><name>alarmActionsEnabled</name><op>assign</op></changeSet><changeSet><name>availableField</name><op>assign</op><val xsi:type="ArrayOfCustomFieldDef"></val></changeSet></objectSet></filterSet></returnval></WaitForUpdatesExResponse>
</soapenv:Body>
</soapenv:Envelope>"#;

    #[test]
    fn test_property_explorer_case1_alarm_only() {
        let result: Result<UpdateSet, _> = soap::vim_response(PROP_EXPLORER_ALARM_ONLY);
        result.expect("changeSet with no val should deserialize");
    }

    #[test]
    fn test_property_explorer_case2_empty_array_only() {
        let result: Result<UpdateSet, _> = soap::vim_response(PROP_EXPLORER_EMPTY_ARRAY_ONLY);
        result.expect("empty ArrayOfCustomFieldDef changeSet should deserialize");
    }

    #[test]
    fn test_property_explorer_case4_alarm_and_empty_array() {
        let result: Result<UpdateSet, _> = soap::vim_response(PROP_EXPLORER_ALARM_AND_EMPTY_ARRAY);
        result.expect("alarm + empty array changeSets should deserialize");
    }

    /// Pruned `MethodFault` / `Event` XML: typed extra fields (registry + `ApiTypedValueVisitor`).
    #[test]
    fn test_xml_vapp_property_fault_simple() {
        let xml = r#"<fault xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="VAppPropertyFault">
        <faultMessage>
            <key>config.product.version</key>
            <message>Product Version: 1.0.0</message>
        </faultMessage>
        <id>config.product.version</id>
        <category>string</category>
        <label>Product Version</label>
        <type>string</type>
        <value>1.0.0</value>
    </fault>"#;

        let fault: MethodFault = from_xml(xml).unwrap();
        assert_eq!(fault.type_, Some(StructType::VAppPropertyFault));
        assert!(fault.fault_message.is_some());
        match fault.extra_fields_.get("id") {
            Some(Value::String(s)) => assert_eq!(s, "config.product.version"),
            o => panic!("id: {:?}", o),
        }
        match fault.extra_fields_.get("label") {
            Some(Value::String(s)) => assert_eq!(s, "Product Version"),
            o => panic!("label: {:?}", o),
        }
        match fault.extra_fields_.get("value") {
            Some(Value::String(s)) => assert_eq!(s, "1.0.0"),
            o => panic!("value: {:?}", o),
        }
    }

    #[test]
    fn test_xml_vapp_property_fault_with_args() {
        let xml = r#"<fault xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="VAppPropertyFault">
        <faultMessage>
            <key>config.product.version</key>
            <arg xsi:type="KeyAnyValue">
                <key>config.product.version</key>
                <value xsi:type="xsd:string">1.0.0</value>
            </arg>
            <message>Product Version: 1.0.0</message>
        </faultMessage>
        <id>config.product.version</id>
        <category>string</category>
        <label>Product Version</label>
        <type>string</type>
        <value>1.0.0</value>
    </fault>"#;

        let fault: MethodFault = from_xml(xml).unwrap();
        assert_eq!(fault.type_, Some(StructType::VAppPropertyFault));
        assert!(fault.fault_message.is_some());
        assert!(fault.extra_fields_.get("label").is_some());
    }

    #[test]
    fn test_xml_event_ex_extra_fields() {
        let xml = r#"<event xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="EventEx">
        <key>12345</key>
        <chainId>100</chainId>
        <createdTime>2024-06-15T10:30:00Z</createdTime>
        <userName>admin</userName>
        <eventTypeId>com.vmware.example.event</eventTypeId>
        <severity>info</severity>
        <message>Something happened</message>
    </event>"#;

        let event: Event = from_xml(xml).unwrap();
        assert_eq!(event.type_, Some(StructType::EventEx));
        assert_eq!(event.key, 12345);
        assert_eq!(event.user_name, "admin");
        match event.extra_fields_.get("eventTypeId") {
            Some(Value::String(s)) => assert_eq!(s, "com.vmware.example.event"),
            o => panic!("eventTypeId: {:?}", o),
        }
        match event.extra_fields_.get("severity") {
            Some(Value::String(s)) => assert_eq!(s, "info"),
            o => panic!("severity: {:?}", o),
        }
    }

    #[test]
    fn test_xml_fault_numeric_extra_field() {
        let xml = r#"<fault xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ReadOnlyDisksWithLegacyDestination">
        <roDiskCount>2</roDiskCount>
        <timeoutDanger>false</timeoutDanger>
    </fault>"#;

        let fault: MethodFault = from_xml(xml).unwrap();
        assert_eq!(
            fault.type_,
            Some(StructType::ReadOnlyDisksWithLegacyDestination)
        );
        match fault.extra_fields_.get("roDiskCount") {
            Some(Value::Number(Number::I64(n))) => assert_eq!(*n, 2),
            o => panic!("roDiskCount: {:?}", o),
        }
        match fault.extra_fields_.get("timeoutDanger") {
            Some(Value::Bool(b)) => assert!(!b),
            o => panic!("timeoutDanger: {:?}", o),
        }
    }

    #[test]
    fn test_xml_event_arguments_array() {
        let xml = r#"<event xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="EventEx">
        <key>1</key>
        <chainId>1</chainId>
        <createdTime>2024-01-01T00:00:00Z</createdTime>
        <userName>system</userName>
        <eventTypeId>test.event</eventTypeId>
        <message>m</message>
        <arguments xsi:type="KeyAnyValue">
            <key>arg1</key>
            <value xsi:type="xsd:string">val1</value>
        </arguments>
        <arguments xsi:type="KeyAnyValue">
            <key>arg2</key>
            <value xsi:type="xsd:string">val2</value>
        </arguments>
    </event>"#;

        let event: Event = from_xml(xml).unwrap();
        let args = event.extra_fields_.get("arguments");
        assert!(args.is_some());
        if let Some(Value::Array(arr)) = args {
            assert_eq!(arr.len(), 2);
        } else {
            panic!("arguments should be Value::Array, got {:?}", args);
        }
    }

    #[test]
    fn test_xml_single_arguments_still_array() {
        let xml = r#"<event xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="EventEx">
        <key>1</key>
        <chainId>1</chainId>
        <createdTime>2024-01-01T00:00:00Z</createdTime>
        <userName>system</userName>
        <eventTypeId>test.event</eventTypeId>
        <message>m</message>
        <arguments xsi:type="KeyAnyValue">
            <key>only_arg</key>
            <value xsi:type="xsd:string">only_val</value>
        </arguments>
    </event>"#;

        let event: Event = from_xml(xml).unwrap();
        let args = event.extra_fields_.get("arguments");
        if let Some(Value::Array(arr)) = args {
            assert_eq!(arr.len(), 1);
        } else {
            panic!("single arguments element should still be Array, got {:?}", args);
        }
    }

}
