use std::borrow::Cow;

use miniserde::de::{Deserialize, Visitor};
use miniserde::{Error, Result};
use quick_xml::events::{BytesRef, BytesStart, Event};
use quick_xml::reader::Reader;

// ============================================================================
// xsi:type ↔ _typeName bridging
// ============================================================================

/// Strip XML namespace prefix from an xsi:type value to get a clean type name.
/// `"xsd:string"` → `"string"`, `"VirtualMachine"` → `"VirtualMachine"`.
fn xsi_type_to_type_name(xsi_val: &str) -> &str {
    match xsi_val.rsplit_once(':') {
        Some((_, local)) => local,
        None => xsi_val,
    }
}

/// True for `xmlns`, `xmlns:foo`, etc. — namespace declarations to skip.
fn is_xmlns_attr(name: &str) -> bool {
    name == "xmlns" || name.starts_with("xmlns:")
}

/// Scan attributes in one pass: extract the `xsi:type` value and count
/// non-xmlns attributes. Avoids allocating a filtered Vec.
fn find_xsi_type_info(attrs: &[(String, String)]) -> (Option<&str>, usize) {
    let mut xsi_val = None;
    let mut non_ns_count = 0;
    for (name, value) in attrs {
        if is_xmlns_attr(name) {
            continue;
        }
        non_ns_count += 1;
        if name == "xsi:type" {
            xsi_val = Some(value.as_str());
        }
    }
    (xsi_val, non_ns_count)
}

// ============================================================================
// Streaming deserialization: quick_xml events → miniserde Visitor (no tree)
// ============================================================================

/// Resolve `Event::GeneralRef` payload (text between `&` and `;`).
/// Covers predefined entities and numeric character references `&#...;` / `&#x...;`.
fn resolve_entity(r: &BytesRef<'_>) -> Result<Cow<'static, str>> {
    let raw = r.as_ref();
    match raw {
        b"amp" => Ok(Cow::Borrowed("&")),
        b"lt" => Ok(Cow::Borrowed("<")),
        b"gt" => Ok(Cow::Borrowed(">")),
        b"quot" => Ok(Cow::Borrowed("\"")),
        b"apos" => Ok(Cow::Borrowed("'")),
        _ if raw.first() == Some(&b'#') => {
            let ch = parse_numeric_char_ref(&raw[1..])?;
            Ok(Cow::Owned(ch.to_string()))
        }
        _ => Err(Error),
    }
}

/// Parse XML numeric char ref body after `#` (e.g. `65`, `x41`).
fn parse_numeric_char_ref(body: &[u8]) -> Result<char> {
    if body.is_empty() {
        return Err(Error);
    }
    let code = if let Some(hex) = body
        .strip_prefix(b"x")
        .or_else(|| body.strip_prefix(b"X"))
    {
        if hex.is_empty() {
            return Err(Error);
        }
        let s = std::str::from_utf8(hex).map_err(|_| Error)?;
        u32::from_str_radix(s, 16).map_err(|_| Error)?
    } else {
        let s = std::str::from_utf8(body).map_err(|_| Error)?;
        if !s.bytes().all(|b| b.is_ascii_digit()) {
            return Err(Error);
        }
        s.parse::<u32>().map_err(|_| Error)?
    };
    char::from_u32(code).ok_or(Error)
}

fn extract_attrs(start: &BytesStart<'_>) -> Result<Vec<(String, String)>> {
    start
        .attributes()
        .map(|a| {
            let a = a.map_err(|_| Error)?;
            let name = std::str::from_utf8(a.key.as_ref())
                .map_err(|_| Error)?
                .to_string();
            let value = a.unescape_value().map_err(|_| Error)?.into_owned();
            Ok((name, value))
        })
        .collect()
}

pub(crate) fn start_name(start: &BytesStart<'_>) -> Result<String> {
    std::str::from_utf8(start.name().as_ref())
        .map_err(|_| Error)
        .map(String::from)
}

/// Deserialize an XML string into any miniserde Deserialize type.
///
/// Streams quick_xml events directly into the miniserde Visitor without
/// building an intermediate tree. Memory usage is O(depth) not O(document).
///
/// # Example
/// ```ignore
/// let dog: Dog = xml::from_xml("<Dog><name>Rex</name><breed>Lab</breed></Dog>").unwrap();
/// ```
pub fn from_xml<T: Deserialize>(xml: &str) -> Result<T> {
    let mut out = None;
    let visitor = T::begin(&mut out);
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(false);
    stream_root(&mut reader, visitor)?;
    out.ok_or(Error)
}

/// Find the root element and drive the visitor from it.
fn stream_root(reader: &mut Reader<&[u8]>, visitor: &mut dyn Visitor) -> Result<()> {
    loop {
        match reader.read_event().map_err(|_| Error)? {
            Event::Start(e) => return stream_drive(reader, &e, visitor),
            Event::Empty(e) => return drive_empty(&e, visitor),
            Event::Eof => return Err(Error),
            _ => continue,
        }
    }
}

// ============================================================================
// Value delivery helpers
// ============================================================================

/// Try to deliver an XML text value to a Visitor by attempting each typed method.
///
/// XML values are always strings. This function tries the Visitor's methods
/// in order: string → unsigned int → signed int → float → bool → null.
/// Only the matching type's Visitor implementation accepts.
fn deliver_text(text: &str, visitor: &mut dyn Visitor) -> Result<()> {
    let trimmed = text.trim();

    if visitor.string(trimmed).is_ok() {
        return Ok(());
    }

    if let Ok(n) = trimmed.parse::<u64>() {
        if visitor.nonnegative(n).is_ok() {
            return Ok(());
        }
    }

    if let Ok(n) = trimmed.parse::<i64>() {
        if visitor.negative(n).is_ok() {
            return Ok(());
        }
    }

    if let Ok(n) = trimmed.parse::<f64>() {
        if visitor.float(n).is_ok() {
            return Ok(());
        }
    }

    match trimmed {
        "true" => {
            if visitor.boolean(true).is_ok() {
                return Ok(());
            }
        }
        "false" => {
            if visitor.boolean(false).is_ok() {
                return Ok(());
            }
        }
        _ => {}
    }

    if trimmed.is_empty() {
        if visitor.null().is_ok() {
            return Ok(());
        }
    }

    Err(Error)
}

/// Deliver a leaf element (no attrs, no children) to a visitor.
///
/// For non-empty text, delegates to `deliver_text`.  For empty text, tries
/// `string("") → null() → map()+finish()` to cover String fields,
/// `Option<T>`, and all-optional structs respectively.
///
/// The `try_empty_map` call is factored out so the `Box<dyn Map>` borrow is
/// fully released before the caller resumes (borrow-checker requirement).
fn deliver_leaf(text: &str, visitor: &mut dyn Visitor) -> Result<()> {
    if !text.trim().is_empty() {
        return deliver_text(text, visitor);
    }
    if visitor.string("").is_ok() {
        return Ok(());
    }
    if visitor.null().is_ok() {
        return Ok(());
    }
    try_empty_map(visitor)
}

fn try_empty_map(visitor: &mut dyn Visitor) -> Result<()> {
    let mut map = visitor.map()?;
    map.finish()
}

/// Try to deliver a typed leaf value via the map() interface.
///
/// Emits `_typeName` and `#text` keys into the visitor's map and calls
/// `finish()`. Returns `Ok(true)` on success, `Ok(false)` when the visitor
/// rejects `map()` (the caller should fall back to plain text delivery).
///
/// The function boundary ensures the mutable borrow of `visitor` from
/// `visitor.map()` is fully released before the caller can use `visitor` again.
fn typed_leaf_via_map(visitor: &mut dyn Visitor, type_name: &str, text: &str) -> Result<bool> {
    let mut map = match visitor.map() {
        Ok(m) => m,
        Err(_) => return Ok(false),
    };
    deliver_text(type_name, map.key("_typeName")?)?;
    if !text.trim().is_empty() {
        deliver_text(text, map.key("#text")?)?;
    } else {
        // Empty text — could be either:
        //   (a) xsd:string with value "" → string("") accepted via #text
        //   (b) ArrayOf* with no children → initialise _value as empty seq
        let text_visitor = map.key("#text")?;
        if text_visitor.string("").is_err() {
            try_deliver_empty_value(&mut *map)?;
        }
    }
    map.finish()?;
    Ok(true)
}

/// When a typed element has no text content, attempt to deliver `_value` as
/// an empty sequence. Handles `ArrayOf*` types whose `ValuePolyBuilder`
/// wraps a `DelegatingDeserializer<Vec<T>>`: calling `seq()` + `finish()`
/// initialises the inner `Vec` to `Some(vec![])`.
///
/// Factored into its own function so the `Box<dyn Seq>` borrow is fully
/// released before the caller continues (borrow-checker requirement).
fn try_deliver_empty_value(map: &mut dyn miniserde::de::Map) -> Result<()> {
    let visitor = map.key("_value")?;
    if let Ok(mut seq) = visitor.seq() {
        seq.finish()?;
    }
    Ok(())
}

// ============================================================================
// Text accumulation
// ============================================================================

/// Accumulate text content from the event stream into `text`, stopping when
/// a child element Start/Empty or the parent's End tag is encountered.
/// Returns the first non-text child event as a lookahead, or None at End.
fn accumulate_text(
    reader: &mut Reader<&[u8]>,
    text: &mut String,
) -> Result<Option<(BytesStart<'static>, bool)>> {
    loop {
        match reader.read_event().map_err(|_| Error)? {
            Event::Text(e) => text.push_str(&e.decode().map_err(|_| Error)?),
            Event::GeneralRef(r) => text.push_str(&resolve_entity(&r)?),
            Event::CData(e) => text.push_str(std::str::from_utf8(&e).map_err(|_| Error)?),
            Event::Start(e) => return Ok(Some((e.into_owned(), false))),
            Event::Empty(e) => return Ok(Some((e.into_owned(), true))),
            Event::End(_) => return Ok(None),
            Event::Eof => return Err(Error),
            _ => continue,
        }
    }
}

// ============================================================================
// Element drivers
// ============================================================================

/// Emit attributes into an open map visitor: `xsi:type` → `_typeName` first,
/// then remaining non-xmlns attributes as `@name` keys.
fn emit_attrs_to_map(
    map: &mut dyn miniserde::de::Map,
    attrs: &[(String, String)],
) -> Result<()> {
    if let Some(xsi) = attrs
        .iter()
        .find_map(|(n, v)| (n == "xsi:type").then_some(v.as_str()))
    {
        deliver_text(xsi_type_to_type_name(xsi), map.key("_typeName")?)?;
    }
    for (name, value) in attrs {
        if name == "xsi:type" || is_xmlns_attr(name) {
            continue;
        }
        deliver_text(value, map.key(&format!("@{}", name))?)?;
    }
    Ok(())
}

/// Drive a miniserde Visitor directly from the quick_xml event stream.
///
/// The reader must be positioned right after the Start tag described by `start`.
/// On return the reader has consumed through the matching End tag.
pub(crate) fn stream_drive(
    reader: &mut Reader<&[u8]>,
    start: &BytesStart<'_>,
    visitor: &mut dyn Visitor,
) -> Result<()> {
    let attrs = extract_attrs(start)?;
    let mut text = String::new();
    let first_child = accumulate_text(reader, &mut text)?;

    if first_child.is_none() && attrs.is_empty() {
        return deliver_leaf(&text, visitor);
    }

    // Typed leaf: only `xsi:type` attribute (plus optional xmlns*), no children.
    // Try the map() route first (needed for VimAny / PolyCore which must see
    // `_typeName` before the value). If map() is rejected, fall back to plain
    // text delivery (e.g. a plain String inside Vec<String>).
    //
    // The `typed_leaf_via_map` function boundary releases the mutable borrow
    // before we fall through — the borrow checker cannot see that
    // `Box<dyn Map + '_>` is absent in the Err case.
    let (xsi_val, non_ns_count) = find_xsi_type_info(&attrs);
    if non_ns_count == 1 && xsi_val.is_some() && first_child.is_none() {
        let type_name = xsi_type_to_type_name(xsi_val.unwrap());
        if typed_leaf_via_map(visitor, type_name, &text)? {
            return Ok(());
        }
        return deliver_leaf(&text, visitor);
    }

    // Complex element — open as map, emit attrs, text, and children.
    let mut map = visitor.map()?;
    emit_attrs_to_map(&mut *map, &attrs)?;
    if !text.trim().is_empty() {
        deliver_text(&text, map.key("#text")?)?;
    }
    if let Some((child_start, is_empty)) = first_child {
        stream_children(reader, &mut *map, child_start, is_empty)?;
    }
    map.finish()
}

/// Drive a miniserde Visitor from a self-closing element (e.g. `<options/>`).
fn drive_empty(start: &BytesStart<'_>, visitor: &mut dyn Visitor) -> Result<()> {
    let attrs = extract_attrs(start)?;

    if attrs.is_empty() {
        return deliver_leaf("", visitor);
    }

    let (xsi_val, non_ns_count) = find_xsi_type_info(&attrs);
    if non_ns_count == 1 && xsi_val.is_some() {
        let type_name = xsi_type_to_type_name(xsi_val.unwrap());
        if typed_leaf_via_map(visitor, type_name, "")? {
            return Ok(());
        }
        return deliver_leaf("", visitor);
    }

    let mut map = visitor.map()?;
    emit_attrs_to_map(&mut *map, &attrs)?;
    map.finish()
}

/// Dispatch to the correct driver based on whether the element is self-closing.
fn drive_element(
    reader: &mut Reader<&[u8]>,
    start: &BytesStart<'_>,
    is_empty: bool,
    visitor: &mut dyn Visitor,
) -> Result<()> {
    if is_empty {
        drive_empty(start, visitor)
    } else {
        stream_drive(reader, start, visitor)
    }
}

// ============================================================================
// Child element streaming with sequence detection
// ============================================================================

/// Process child elements streaming, grouping adjacent same-named elements
/// into sequences via one-element lookahead.
fn stream_children(
    reader: &mut Reader<&[u8]>,
    map: &mut dyn miniserde::de::Map,
    first_start: BytesStart<'static>,
    first_is_empty: bool,
) -> Result<()> {
    let mut pending = Some((first_start, first_is_empty));

    while let Some((start, is_empty)) = pending.take() {
        let name = start_name(&start)?;
        let visitor = map.key(&name)?;
        pending = try_seq_or_single(reader, visitor, &name, &start, is_empty)?;
    }

    Ok(())
}

/// Try seq() on the visitor and, if accepted, stream all same-named siblings
/// into it. Returns `Ok((true, lookahead))` if seq was used, or
/// `Ok((false, None))` if the visitor rejected seq (caller should deliver as
/// a single element instead).
///
/// The function boundary ensures the `Box<dyn Seq>` from `seq()` is dropped
/// before returning, releasing the mutable borrow on visitor.
fn try_stream_as_seq(
    reader: &mut Reader<&[u8]>,
    visitor: &mut dyn Visitor,
    name: &str,
    start: &BytesStart<'_>,
    is_empty: bool,
) -> Result<(bool, Option<(BytesStart<'static>, bool)>)> {
    let mut seq = match visitor.seq() {
        Ok(seq) => seq,
        Err(_) => return Ok((false, None)),
    };

    drive_element(reader, start, is_empty, seq.element()?)?;

    loop {
        match read_next_child(reader)? {
            Some((next_start, next_empty)) => {
                if start_name(&next_start)? == name {
                    drive_element(reader, &next_start, next_empty, seq.element()?)?;
                } else {
                    seq.finish()?;
                    return Ok((true, Some((next_start, next_empty))));
                }
            }
            None => {
                seq.finish()?;
                return Ok((true, None));
            }
        }
    }
}

/// Try to deliver element(s) as a sequence. If seq() is rejected by the
/// visitor, deliver as a single element instead.
///
/// Returns the next pending child (lookahead from reading past same-named
/// siblings), or None when the parent's End tag was reached.
fn try_seq_or_single(
    reader: &mut Reader<&[u8]>,
    visitor: &mut dyn Visitor,
    name: &str,
    start: &BytesStart<'_>,
    is_empty: bool,
) -> Result<Option<(BytesStart<'static>, bool)>> {
    let (was_seq, pending) = try_stream_as_seq(reader, visitor, name, start, is_empty)?;
    if was_seq {
        return Ok(pending);
    }

    drive_element(reader, start, is_empty, visitor)?;
    Ok(read_next_child(reader)?)
}

/// Read the next child element from the stream, skipping inter-element
/// whitespace. Returns None when the parent's End tag is reached.
fn read_next_child(reader: &mut Reader<&[u8]>) -> Result<Option<(BytesStart<'static>, bool)>> {
    loop {
        match reader.read_event().map_err(|_| Error)? {
            Event::Start(e) => return Ok(Some((e.into_owned(), false))),
            Event::Empty(e) => return Ok(Some((e.into_owned(), true))),
            Event::End(_) => return Ok(None),
            Event::Eof => return Err(Error),
            _ => continue,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::boxed_types::ValueElements;
    use crate::types::enums::{MoTypesEnum, PropertyChangeOpEnum};
    use crate::types::mini_helpers::Base64;
    use crate::types::structs::{HooksHookListSpec, ManagedObjectReference, PropertyChange, UpdateSet, VirtualMachineVirtualNumaInfo};
    use crate::types::traits::OptionValueTrait;
    use crate::types::vim_any::VimAny;

    use super::*;

    #[test]
    fn test_string_from_xml() {
        let xml = "<child>text</child>";
        let value: String = from_xml(xml).unwrap();
        assert_eq!(value, "text");
    }

    #[test]
    fn test_empty_string_from_xml() {
        let xml = "<child></child>";
        let value: String = from_xml(xml).unwrap();
        assert_eq!(value.len(), 0);
    }

    #[test]
    fn test_empty_string_from_xml_self_closing() {
        let xml = "<child/>";
        let value: String = from_xml(xml).unwrap();
        assert_eq!(value.len(), 0);
    }

    #[test]
    fn test_from_xml_boolean() {
        let xml = "<child>true</child>";
        let value: bool = from_xml(xml).unwrap();
        assert_eq!(value, true);
    }
    
    #[test]
    fn test_from_xml_string() {
        let xml = "<child>text</child>";
        let value: String = from_xml(xml).unwrap();
        assert_eq!(value, "text");
    }
    
    #[test]
    fn test_from_xml_int8() {
        let xml = "<child>127</child>";
        let value: i8 = from_xml(xml).unwrap();
        assert_eq!(value, 127);
    }
    
    #[test]
    fn test_from_xml_int16() {
        let xml = "<child>32767</child>";
        let value: i16 = from_xml(xml).unwrap();
        assert_eq!(value, 32767);
    }
    
    #[test]
    fn test_from_xml_int32() {
        let xml = "<child>2147483647</child>";
        let value: i32 = from_xml(xml).unwrap();
        assert_eq!(value, 2147483647);
    }
    
    #[test]
    fn test_from_xml_int64() {
        let xml = "<child>9223372036854775807</child>";
        let value: i64 = from_xml(xml).unwrap();
        assert_eq!(value, 9223372036854775807);
    }
    
    #[test]
    fn test_from_xml_float() {
        let xml = "<child>3.14</child>";
        let value: f32 = from_xml(xml).unwrap();
        assert_eq!(value, 3.14);
    }
    
    #[test]
    fn test_from_xml_double() {
        let xml = "<child>2.718281828</child>";
        let value: f64 = from_xml(xml).unwrap();
        assert_eq!(value, 2.718281828);
    }
    
    #[test]
    fn test_from_xml_datetime() {
        let xml = "<child>2024-01-15T10:30:00Z</child>";
        let value: String = from_xml(xml).unwrap();
        assert_eq!(value, "2024-01-15T10:30:00Z");
    }
    
    #[test]
    fn test_from_xml_binary() {
        let xml = "<child>YWJjMTIz</child>";
        let value: Base64 = from_xml(xml).unwrap();
        assert_eq!(value.0, b"abc123");
    }

    #[test]
    fn test_basic_struct(){
        let xml = r#"<numaInfo>
                            <autoCoresPerNumaNode>true</autoCoresPerNumaNode>
                            <vnumaOnCpuHotaddExposed>false</vnumaOnCpuHotaddExposed>
                        </numaInfo>"#;
        let value: VirtualMachineVirtualNumaInfo = from_xml(xml).unwrap();
        assert_eq!(value.auto_cores_per_numa_node, Some(true));
        assert_eq!(value.vnuma_on_cpu_hotadd_exposed, Some(false));
        assert_eq!(value.cores_per_numa_node, None);
    }

    #[test]
    fn test_struct_no_fields(){
        let xml = r#"<numaInfo></numaInfo>"#;
        let value: VirtualMachineVirtualNumaInfo = from_xml(xml).unwrap();
        assert_eq!(value.auto_cores_per_numa_node, None);
        assert_eq!(value.vnuma_on_cpu_hotadd_exposed, None);
        assert_eq!(value.cores_per_numa_node, None);
    }

    #[test]
    fn test_struct_self_closing_empty(){
        let xml = r#"<numaInfo/>"#;
        let value: VirtualMachineVirtualNumaInfo = from_xml(xml).unwrap();
        assert_eq!(value.auto_cores_per_numa_node, None);
        assert_eq!(value.vnuma_on_cpu_hotadd_exposed, None);
        assert_eq!(value.cores_per_numa_node, None);
    }

    #[test]
    fn test_extra_config_parse() {
        let xml = r#"<extraConfig>
                        <key>tools.guest.desktop.autolock</key>
                        <value xsi:type="xsd:string">TRUE</value>
                    </extraConfig>"#;
        let result: Box<dyn OptionValueTrait> = from_xml(&xml).unwrap();
        assert_eq!(&result.key, "tools.guest.desktop.autolock");
        // if !matches!(result.value, Some(VimAny::Value(ValueElements::PrimitiveString("TRUE".to_string())))) {
        //     panic!("expected VimAny::Value(PrimitiveString(\"TRUE\"))");
        // }
    }

    #[test]
    fn test_extra_config_parse_empty_string() {
        let xml = r#"<extraConfig>
            <key>scsi0:0.redo</key>
            <value xsi:type="xsd:string"></value>
        </extraConfig>"#;
        let result: Box<dyn OptionValueTrait> = from_xml(&xml).unwrap();
        assert_eq!(&result.key, "scsi0:0.redo");
        //assert_eq!(result.value, Some("".to_string()));
    }

    #[test]
    fn test_empty_array_update_set() {
        let xml = r#"<updateSet>
            <version>2</version>
        </updateSet>"#;
        let result: UpdateSet = from_xml(&xml).unwrap();

        assert_eq!(result.version, "2");
        assert!(result.truncated.is_none());
        assert!(result.filter_set.is_none());
    }
    
    #[test]
    fn test_mor_deserialize() {
        let xml = r#"<val type="VirtualMachine">5</val>"#;
        let result: ManagedObjectReference = from_xml(&xml).unwrap();
        assert_eq!(result.r#type, MoTypesEnum::VirtualMachine);
        assert_eq!(result.value, "5");
    }

    #[test]
    fn test_struct_discriminator() {
        let xml = r#"<changeSet>
                                <name>availableField</name>
                                <op>assign</op>
                                <val xsi:type="ArrayOfCustomFieldDef"></val>
                            </changeSet>"#;
        let value: PropertyChange = from_xml(xml).unwrap();
        assert_eq!(value.name, "availableField");
        assert_eq!(value.op, PropertyChangeOpEnum::Assign);

        assert!(value.val.is_some());
        let VimAny::Value(ValueElements::ArrayOfCustomFieldDef(arr)) = value.val.unwrap() else {
            panic!("expected ArrayOfCustomFieldDef");
        };
        assert!(arr.is_empty());
    }

    #[test]
    fn test_struct_discriminator_self_closing() {
        let xml = r#"<changeSet>
                                <name>availableField</name>
                                <op>assign</op>
                                <val xsi:type="ArrayOfCustomFieldDef"/>
                            </changeSet>"#;
        let value: PropertyChange = from_xml(xml).unwrap();
        assert_eq!(value.name, "availableField");
        assert_eq!(value.op, PropertyChangeOpEnum::Assign);

        assert!(value.val.is_some());
        let VimAny::Value(ValueElements::ArrayOfCustomFieldDef(arr)) = value.val.unwrap() else {
            panic!("expected ArrayOfCustomFieldDef");
        };
        assert!(arr.is_empty());
    }

    #[test]
    fn test_arrays() {
        let xml = r#"<hooks>
                            <solutions>test1</solutions>
                            <solutions>test2</solutions>
                            <hosts type="HostSystem">test3</hosts>
                            <hosts type="HostSystem">test4</hosts>
                        </hooks>"#;
        let value: HooksHookListSpec = from_xml(&xml).unwrap();
        assert_eq!(value.solutions.unwrap().len(), 2);
        assert_eq!(value.hosts.as_ref().unwrap().len(), 2);
        assert_eq!(value.hosts.as_ref().unwrap()[0].r#type, MoTypesEnum::HostSystem);
        assert_eq!(value.hosts.as_ref().unwrap()[0].value, "test3");
        assert_eq!(value.hosts.as_ref().unwrap()[1].r#type, MoTypesEnum::HostSystem);
        assert_eq!(value.hosts.as_ref().unwrap()[1].value, "test4");
    }

    /// Illustrate the interesting behavior of the miniserde deserializer when
    /// encountering array elements in mixed order. This is not really valid XML.
    #[test]
    fn test_arrays_mixed_order() {
        let xml = r#"<hooks>
                            <solutions>test1</solutions>
                            <hosts type="HostSystem">test3</hosts>
                            <solutions>test2</solutions>
                        </hooks>"#;
        let value: HooksHookListSpec = from_xml(&xml).unwrap();
        assert_eq!(value.solutions.as_ref().unwrap().len(), 1);
        assert_eq!(value.solutions.as_ref().unwrap()[0], "test2");
        assert_eq!(value.hosts.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_arrays_empty() {
        let xml = r#"<hooks>
                            <solutions>test1</solutions>
                            <solutions>test2</solutions>
                        </hooks>"#;
        let value: HooksHookListSpec = from_xml(&xml).unwrap();
        assert_eq!(value.solutions.unwrap().len(), 2);
        assert!(value.hosts.is_none());
    }

    // --- XML entity decoding (text nodes + GeneralRef path) ---

    #[test]
    fn test_entity_amp_in_element_text() {
        let v: String = from_xml(r#"<child>a&amp;b</child>"#).unwrap();
        assert_eq!(v, "a&b");
    }

    #[test]
    fn test_entity_lt_gt_in_element_text() {
        let v: String = from_xml(r#"<child>&lt;tag&gt;</child>"#).unwrap();
        assert_eq!(v, "<tag>");
    }

    #[test]
    fn test_entity_quot_apos_in_element_text() {
        let v: String = from_xml(r#"<child>&quot;x&quot; &apos;y&apos;</child>"#).unwrap();
        assert_eq!(v, "\"x\" 'y'");
    }

    #[test]
    fn test_entity_multiple_in_sequence() {
        let v: String = from_xml(r#"<child>&amp;&lt;&amp;</child>"#).unwrap();
        assert_eq!(v, "&<&");
    }

    #[test]
    fn test_entity_mixed_with_plain_text() {
        let v: String = from_xml(r#"<child>pre&amp;suf</child>"#).unwrap();
        assert_eq!(v, "pre&suf");
    }

    /// CDATA must not treat `&amp;` as an entity — literal characters only.
    #[test]
    fn test_cdata_preserves_ampersand_sequence() {
        let v: String = from_xml(r#"<child><![CDATA[a&b<c>&amp;]]></child>"#).unwrap();
        assert_eq!(v, "a&b<c>&amp;");
    }

    /// Numeric / hex char references: quick-xml typically resolves these in `Text::decode`
    /// (not via `GeneralRef`). Document the expected decoded string.
    #[test]
    fn test_char_ref_decimal_in_element_text() {
        let v: String = from_xml("<child>&#65;BC</child>").unwrap();
        assert_eq!(v, "ABC");
    }

    #[test]
    fn test_char_ref_hex_in_element_text() {
        let v: String = from_xml("<child>&#x26;copy;</child>").unwrap();
        assert_eq!(v, "&copy;");
    }

    /// Unknown named entity: must not deserialize successfully.
    #[test]
    fn test_unknown_named_entity_fails() {
        let err = from_xml::<String>(r#"<child>&notarealxmlentity;</child>"#);
        assert!(err.is_err(), "expected unknown entity to fail, got {:?}", err);
    }

    #[test]
    fn test_char_ref_out_of_range_fails() {
        assert!(
            from_xml::<String>("<child>&#x110000;</child>").is_err(),
            "codepoint above U+10FFFF must fail"
        );
    }

    #[test]
    fn test_char_ref_empty_fails() {
        assert!(from_xml::<String>("<child>&#;</child>").is_err());
    }

    #[test]
    fn test_char_ref_surrogate_codepoint_fails() {
        assert!(
            from_xml::<String>("<child>&#xD800;</child>").is_err(),
            "lone surrogate must be rejected"
        );
    }

    /// Attribute values are unescaped by quick-xml separately from text accumulation.
    #[test]
    fn test_entity_in_attribute_value_for_moref() {
        let v: ManagedObjectReference =
            from_xml(r#"<mor type="VirtualMachine">vm&amp;1</mor>"#).unwrap();
        assert_eq!(v.r#type, MoTypesEnum::VirtualMachine);
        assert_eq!(v.value, "vm&1");
    }
}