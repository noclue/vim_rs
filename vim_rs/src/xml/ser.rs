use miniserde::ser::{Fragment, Map as SerMap, Serialize};
use std::borrow::Cow;


// ============================================================================
// XML escaping (delegated to quick_xml)
// ============================================================================

fn escape_xml(s: &str) -> Cow<'_, str> {
    quick_xml::escape::escape(s)
}

/// Map VIM type names to their `xsi:type` attribute form.
/// XSD primitives get the `xsd:` prefix; VIM types pass through unchanged.
fn xml_type_attr_name(tn: &str) -> Cow<'_, str> {
    match tn {
        "string" | "boolean" | "byte" | "short" | "int" | "long" | "float" | "double"
        | "dateTime" | "anyURI" | "base64Binary" => Cow::Owned(format!("xsd:{tn}")),
        _ => Cow::Borrowed(tn),
    }
}

/// Derive the child element name for items inside an `ArrayOf*` value.
/// XSD array items use lowercase names; VIM struct arrays keep their casing.
fn array_element_name(array_type: &str) -> &str {
    match array_type {
        "ArrayOfString" => "string",
        "ArrayOfBoolean" => "boolean",
        "ArrayOfByte" => "byte",
        "ArrayOfShort" => "short",
        "ArrayOfInt" => "int",
        "ArrayOfLong" => "long",
        "ArrayOfFloat" => "float",
        "ArrayOfDouble" => "double",
        "ArrayOfDateTime" => "dateTime",
        "ArrayOfAnyType" => "anyType",
        "ArrayOfAnyURI" => "anyURI",
        "ArrayOfBase64Binary" => "base64Binary",
        other => other.strip_prefix("ArrayOf").unwrap_or("item"),
    }
}

// ============================================================================
// Streaming serialization: Serialize → XML string (no intermediate tree)
// ============================================================================

/// Serialize a miniserde type to an XML string.
///
/// XML is emitted directly as the Serialize impl produces Fragment data —
/// no intermediate tree is built. Memory usage is O(depth).
///
/// # Example
/// ```ignore
/// let xml = xml::to_xml(&dog, "Dog");
/// // <Dog><name>Rex</name><breed>Labrador</breed></Dog>
/// ```
pub fn to_xml<T: Serialize>(value: &T, root_name: &str) -> String {
    let mut out = String::new();
    stream_xml(value, root_name, &mut out);
    out
}

/// Like `to_xml` but includes the XML declaration.
pub fn to_xml_doc<T: Serialize>(value: &T, root_name: &str) -> String {
    let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    stream_xml(value, root_name, &mut out);
    out
}

/// Write a Serialize value as `<tag>...</tag>` directly to a string.
pub(crate) fn stream_xml(ser: &dyn Serialize, tag: &str, out: &mut String) {
    match ser.begin() {
        Fragment::Null => {
            out.push('<');
            out.push_str(tag);
            out.push_str("/>");
        }
        Fragment::Bool(b) => write_leaf(tag, if b { "true" } else { "false" }, out),
        Fragment::Str(s) => write_leaf(tag, &escape_xml(&s), out),
        Fragment::U64(n) => write_leaf(tag, &n.to_string(), out),
        Fragment::I64(n) => write_leaf(tag, &n.to_string(), out),
        Fragment::F64(n) => write_leaf(tag, &n.to_string(), out),
        Fragment::Seq(mut seq) => {
            while let Some(item) = seq.next() {
                stream_xml(item, tag, out);
            }
        }
        Fragment::Map(mut map) => {
            stream_xml_map(&mut *map, tag, out);
        }
    }
}

fn write_leaf(tag: &str, text: &str, out: &mut String) {
    out.push('<');
    out.push_str(tag);
    out.push('>');
    out.push_str(text);
    out.push_str("</");
    out.push_str(tag);
    out.push('>');
}

/// Write a scalar Serialize value as text (no tags). Used for attribute values
/// and `#text` content.
fn write_scalar(ser: &dyn Serialize, out: &mut String) {
    match ser.begin() {
        Fragment::Null => {}
        Fragment::Bool(b) => out.push_str(if b { "true" } else { "false" }),
        Fragment::Str(s) => out.push_str(&escape_xml(&s)),
        Fragment::U64(n) => out.push_str(&n.to_string()),
        Fragment::I64(n) => out.push_str(&n.to_string()),
        Fragment::F64(n) => out.push_str(&n.to_string()),
        Fragment::Seq(_) | Fragment::Map(_) => {}
    }
}

fn scalar_to_string(ser: &dyn Serialize) -> Option<String> {
    match ser.begin() {
        Fragment::Str(s) => Some(s.into_owned()),
        _ => None,
    }
}

/// Stream a Map as XML. Decides rendering mode from the first key:
///
/// - `_typeName: "ManagedObjectReference"` → compact `<tag type="...">value</tag>`
/// - `_typeName: "Other"` → `<tag xsi:type="Other">children...</tag>`
/// - No `_typeName` → `<tag>children...</tag>`
///
/// Special handling for `_value` key (used by ValueElements for boxed
/// primitives/arrays): scalars become text content, Seq items become
/// repeated child elements named after the array's item type.
///
/// Relies on vim_rs convention: `_typeName` is always the first Map entry.
fn stream_xml_map(map: &mut dyn SerMap, tag: &str, out: &mut String) {
    let tag_start = out.len();
    out.push('<');
    out.push_str(tag);

    let mut is_mor = false;
    let mut mor_attr = String::new();
    let mut mor_text = String::new();
    let mut has_body = false;
    let mut captured_type_name: Option<String> = None;

    while let Some((key, value)) = map.next() {
        if &*key == "_typeName" {
            if let Some(tn) = scalar_to_string(value) {
                if tn == "ManagedObjectReference" {
                    is_mor = true;
                    out.truncate(tag_start);
                } else {
                    let attr_name = xml_type_attr_name(&tn);
                    out.push_str(" xsi:type=\"");
                    out.push_str(&escape_xml(&attr_name));
                    out.push('"');
                    captured_type_name = Some(tn);
                }
            }
        } else if is_mor {
            match &*key {
                "type" => write_scalar(value, &mut mor_attr),
                "value" => write_scalar(value, &mut mor_text),
                _ => {}
            }
        } else if &*key == "_value" {
            stream_value_content(value, &captured_type_name, tag, &mut has_body, out);
        } else if let Some(attr_name) = key.strip_prefix('@') {
            out.push(' ');
            out.push_str(attr_name);
            out.push_str("=\"");
            write_scalar(value, out);
            out.push('"');
        } else if &*key == "#text" {
            if !has_body {
                out.push('>');
                has_body = true;
            }
            write_scalar(value, out);
        } else {
            if !has_body {
                out.push('>');
                has_body = true;
            }
            stream_xml(value, &key, out);
        }
    }

    if is_mor {
        out.push('<');
        out.push_str(tag);
        if !mor_attr.is_empty() {
            out.push_str(" type=\"");
            out.push_str(&mor_attr);
            out.push('"');
        }
        if !mor_text.is_empty() {
            out.push('>');
            out.push_str(&mor_text);
            out.push_str("</");
            out.push_str(tag);
            out.push('>');
        } else {
            out.push_str("/>");
        }
    } else if has_body {
        out.push_str("</");
        out.push_str(tag);
        out.push('>');
    } else {
        out.push_str("/>");
    }
}

/// Handle the `_value` key from ValueElements serialization.
/// Scalars are emitted as text content; Seq items become repeated child
/// elements whose name is derived from the captured `_typeName`.
fn stream_value_content(
    value: &dyn Serialize,
    captured_type_name: &Option<String>,
    tag: &str,
    has_body: &mut bool,
    out: &mut String,
) {
    match value.begin() {
        Fragment::Null => {}
        Fragment::Bool(b) => {
            if !*has_body {
                out.push('>');
                *has_body = true;
            }
            out.push_str(if b { "true" } else { "false" });
        }
        Fragment::Str(s) => {
            if !*has_body {
                out.push('>');
                *has_body = true;
            }
            out.push_str(&escape_xml(&s));
        }
        Fragment::U64(n) => {
            if !*has_body {
                out.push('>');
                *has_body = true;
            }
            out.push_str(&n.to_string());
        }
        Fragment::I64(n) => {
            if !*has_body {
                out.push('>');
                *has_body = true;
            }
            out.push_str(&n.to_string());
        }
        Fragment::F64(n) => {
            if !*has_body {
                out.push('>');
                *has_body = true;
            }
            out.push_str(&n.to_string());
        }
        Fragment::Seq(mut seq) => {
            if !*has_body {
                out.push('>');
                *has_body = true;
            }
            let elem_name = captured_type_name
                .as_deref()
                .map(array_element_name)
                .unwrap_or("item");
            while let Some(item) = seq.next() {
                stream_xml(item, elem_name, out);
            }
        }
        Fragment::Map(mut map) => {
            if !*has_body {
                out.push('>');
                *has_body = true;
            }
            let elem_name = captured_type_name
                .as_deref()
                .map(array_element_name)
                .unwrap_or(tag);
            stream_xml_map(&mut *map, elem_name, out);
        }
    }
}
