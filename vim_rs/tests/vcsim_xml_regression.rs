//! Regression tests for SOAP/XML from **govc vcsim**, which uses a non-standard
//! XML Schema Instance **prefix**: `xmlns:_XMLSchema-instance` and attributes
//! `_XMLSchema-instance:type` instead of the usual `xmlns:xsi` / `xsi:type`.
//!
//! The decoder treats any qualified attribute whose **local name** is `type` as
//! the schema-instance type discriminator (same as `xsi:type`).
//!
//! Run: `cargo test -p vim_rs --features xml vcsim_xml -- --nocapture`

#![cfg(feature = "xml")]

use vim_rs::types::boxed_types::ValueElements;
use vim_rs::types::enums::ManagedEntityStatusEnum;
use vim_rs::types::structs::{OptionDef, UpdateSet, VirtualMachineStorageSummary};
use vim_rs::types::vim_any::VimAny;
use vim_rs::xml::de::{from_xml, from_xml_with, DeserializeOptions};
use vim_rs::xml::soap::vim_response;

/// Captured `WaitForUpdatesEx` SOAP envelope from vcsim (small, committed to
/// the repository — covers the VM-list path).
const VCSIM_WAIT_FOR_UPDATES_EX: &str = include_str!("fixtures/vcsim.xml");

// ---------------------------------------------------------------------------
// Exploded fragments — same semantics as pieces inside `vcsim.xml`, for
// isolated expectations.
// ---------------------------------------------------------------------------

/// Minimal typed string leaf: **vcsim** naming (`_XMLSchema-instance` prefix).
const FRAG_VCSIM_TYPED_STRING_VAL: &str = r#"<val xmlns:_XMLSchema-instance="http://www.w3.org/2001/XMLSchema-instance" _XMLSchema-instance:type="xsd:string">DC0_H0_VM0</val>"#;

/// Same logical value as [`FRAG_VCSIM_TYPED_STRING_VAL`], **vCenter-style** `xsi:type`.
const FRAG_VCENTER_TYPED_STRING_VAL: &str = r#"<val xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="xsd:string">DC0_H0_VM0</val>"#;

/// Enum-typed leaf (ManagedEntityStatus) — vcsim prefix.
const FRAG_VCSIM_TYPED_ENUM_VAL: &str = r#"<val xmlns:_XMLSchema-instance="http://www.w3.org/2001/XMLSchema-instance" _XMLSchema-instance:type="ManagedEntityStatus">green</val>"#;

/// Same — vCenter `xsi` prefix.
const FRAG_VCENTER_TYPED_ENUM_VAL: &str = r#"<val xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="ManagedEntityStatus">green</val>"#;

/// Struct-typed `<val>` with child elements (abbreviated from the capture).
const FRAG_VCSIM_TYPED_STORAGE_SUMMARY_VAL: &str = r#"<val xmlns:_XMLSchema-instance="http://www.w3.org/2001/XMLSchema-instance" _XMLSchema-instance:type="VirtualMachineStorageSummary"><committed>234</committed><uncommitted>10737418240</uncommitted><unshared>202</unshared><timestamp>2026-04-14T20:42:10.38132+03:00</timestamp></val>"#;

const FRAG_VCENTER_TYPED_STORAGE_SUMMARY_VAL: &str = r#"<val xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="VirtualMachineStorageSummary"><committed>234</committed><uncommitted>10737418240</uncommitted><unshared>202</unshared><timestamp>2026-04-14T20:42:10.38132+03:00</timestamp></val>"#;

#[test]
fn vcsim_xml_full_fixture_wait_for_updates_ex_parses_as_update_set() {
    let update_set: UpdateSet =
        vim_response(VCSIM_WAIT_FOR_UPDATES_EX).expect("vcsim WaitForUpdatesEx → UpdateSet");
    assert_eq!(update_set.version, "-");
    let fs = update_set.filter_set.as_ref().expect("filter_set");
    assert_eq!(fs.len(), 1);
    let objects = fs[0].object_set.as_ref().expect("object_set");
    assert_eq!(objects.len(), 4);
}

/// Minimal reproducer for the culprit found by differential bisection of a
/// real `vcsim` `HostSystem` `WaitForUpdatesEx` response:
///
/// `vcsim` serializes `HostConfigInfo.optionDef` as an `OptionDef` element
/// that lacks the `<optionType>` child, e.g.
///
/// ```xml
/// <optionDef>
///   <label></label>
///   <summary></summary>
///   <key>RUN.container</key>
/// </optionDef>
/// ```
///
/// Real vCenter/ESX always include `<optionType>…</optionType>`.
/// `OptionDef::build()` requires the `optionType` holder to be present, so
/// strict mode fails. Tolerant mode drops the value instead of propagating
/// the error.
#[test]
fn vcsim_option_def_without_option_type_strict_vs_tolerant() {
    let xml =
        r#"<optionDef><label></label><summary></summary><key>RUN.container</key></optionDef>"#;

    // Strict: errors out because required optionType is missing.
    let strict: Result<OptionDef, _> = from_xml(xml);
    assert!(strict.is_err(), "strict from_xml must reject missing optionType");

    // Tolerant at the *same* scope as the element: the build() error is
    // swallowed, which means the output slot stays `None` and from_xml
    // surfaces that as its own Err (the Option sentinel).
    //
    // This is the key semantic observation: tolerance drops the *element*,
    // but a top-level deserialize still needs *something* to return. The
    // effect is meaningful only when the failing element is nested inside
    // a parent (Vec or Option<...>) that can absorb the drop — which is
    // precisely the HostConfigInfo.option_def situation.
    let tolerant_top: Result<OptionDef, _> =
        from_xml_with(xml, DeserializeOptions { tolerate_build_errors: true });
    assert!(
        tolerant_top.is_err(),
        "tolerant mode at the root still has no value to return"
    );
}

/// Sibling sanity check: the same `OptionDef` *with* `<optionType/>` does
/// parse today in strict mode. Keeps the minimum-correct shape on record so
/// any future codegen change to `OptionDef` is noticed immediately.
#[test]
fn vcsim_option_def_with_empty_option_type_deserializes() {
    let xml = r#"<optionDef><label></label><summary></summary><key>RUN.container</key><optionType/></optionDef>"#;
    let od: OptionDef = from_xml(xml).expect("OptionDef with <optionType/> must deserialize");
    assert_eq!(od.key, "RUN.container");
}

#[test]
fn vcsim_xml_minimal_typed_string_val_ok_vim_any() {
    let v: VimAny = from_xml(FRAG_VCSIM_TYPED_STRING_VAL).expect("vcsim xsd:string val");
    match v {
        VimAny::Value(ValueElements::PrimitiveString(s)) => assert_eq!(s, "DC0_H0_VM0"),
        other => panic!("expected VimAny::Value(PrimitiveString), got {:?}", other),
    }
}

#[test]
fn vcsim_xml_equivalent_xsi_typed_string_val_ok_vim_any() {
    let v: VimAny = from_xml(FRAG_VCENTER_TYPED_STRING_VAL).expect("xsi:type xsd:string");
    match v {
        VimAny::Value(ValueElements::PrimitiveString(s)) => assert_eq!(s, "DC0_H0_VM0"),
        other => panic!("expected VimAny::Value(PrimitiveString), got {:?}", other),
    }
}

#[test]
fn vcsim_xml_minimal_typed_enum_val_ok_vim_any() {
    let v: VimAny = from_xml(FRAG_VCSIM_TYPED_ENUM_VAL).expect("vcsim ManagedEntityStatus");
    match v {
        VimAny::Value(ValueElements::ManagedEntityStatus(e)) => {
            assert_eq!(e, ManagedEntityStatusEnum::Green);
        }
        other => panic!("unexpected {:?}", other),
    }
}

#[test]
fn vcsim_xml_equivalent_xsi_typed_enum_val_ok_vim_any() {
    let v: VimAny = from_xml(FRAG_VCENTER_TYPED_ENUM_VAL).expect("xsi ManagedEntityStatus");
    match v {
        VimAny::Value(ValueElements::ManagedEntityStatus(e)) => {
            assert_eq!(e, ManagedEntityStatusEnum::Green);
        }
        other => panic!("unexpected {:?}", other),
    }
}

#[test]
fn vcsim_xml_struct_val_storage_summary_ok_with_vcsim_prefix() {
    let v: VimAny =
        from_xml(FRAG_VCSIM_TYPED_STORAGE_SUMMARY_VAL).expect("VirtualMachineStorageSummary");
    match &v {
        VimAny::Object(o) => {
            let s: Option<&VirtualMachineStorageSummary> = o.as_any_ref().downcast_ref();
            let s = s.expect("VirtualMachineStorageSummary object");
            assert_eq!(s.committed, 234);
        }
        other => panic!("expected VimAny::Object(VirtualMachineStorageSummary), got {:?}", other),
    }
}

#[test]
fn vcsim_xml_struct_val_storage_summary_ok_with_vcenter_xsi() {
    let v: VimAny =
        from_xml(FRAG_VCENTER_TYPED_STORAGE_SUMMARY_VAL).expect("VirtualMachineStorageSummary");
    match &v {
        VimAny::Object(o) => {
            let s: Option<&VirtualMachineStorageSummary> = o.as_any_ref().downcast_ref();
            let s = s.expect("VirtualMachineStorageSummary object");
            assert_eq!(s.committed, 234);
        }
        other => panic!("expected VimAny::Object(VirtualMachineStorageSummary), got {:?}", other),
    }
}
