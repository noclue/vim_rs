use base64::Engine;
use miniserde::de::Visitor;
use miniserde::json::{Number, Value};
use miniserde::{make_place, Deserialize, Error, Result};

/// Replay a captured Value to a Visitor.
/// This effectively "deserializes" the Value into whatever the Visitor expects.
pub fn replay_value_to_visitor(value: &Value, visitor: &mut dyn Visitor) -> Result<()> {
    match value {
        Value::Null => visitor.null(),
        Value::Bool(b) => visitor.boolean(*b),
        Value::String(s) => visitor.string(s),
        Value::Number(n) => match n {
            Number::U64(n) => visitor.nonnegative(*n),
            Number::I64(n) => visitor.negative(*n),
            Number::F64(n) => visitor.float(*n),
        },
        Value::Array(arr) => {
            let mut seq = visitor.seq()?;
            for item in arr.iter() {
                let element_visitor = seq.element()?;
                replay_value_to_visitor(item, element_visitor)?;
            }
            seq.finish()
        }
        Value::Object(obj) => {
            let mut map = visitor.map()?;
            for (k, v) in obj.iter() {
                let field_visitor = map.key(k)?;
                replay_value_to_visitor(v, field_visitor)?;
            }
            map.finish()
        }
    }
}

/// Deserialize a type from a `miniserde::json::Value`.
/// This is the miniserde equivalent of `serde_json::from_value`.
///
/// # Example
/// ```ignore
/// let value: miniserde::json::Value = /* captured from extra_fields_ */;
/// let mor: ManagedObjectReference = from_value(&value)?;
/// ```
pub fn from_value<T: Deserialize>(value: &Value) -> Result<T> {
    let mut out: Option<T> = None;
    let visitor = T::begin(&mut out);
    replay_value_to_visitor(value, visitor)?;
    out.ok_or(Error)
}

/// Wrapper for base64 encoded data to simplify deserialization.
pub struct Base64(pub Vec<u8>);

make_place!(Place);

impl Deserialize for Base64 {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
        Place::new(out)
    }
}

impl Visitor for Place<Base64> {
    fn string(&mut self, s: &str) -> Result<()> {
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(s)
            .map_err(|_| Error)?;
        self.out = Some(Base64(bytes));
        Ok(())
    }
}

/// Write a JSON string to a formatter with basic pretty-printing.
/// This is used by Debug implementations since miniserde lacks `to_string_pretty`.
pub fn write_pretty_json(
    f: &mut std::fmt::Formatter<'_>,
    json: &str,
) -> std::fmt::Result {
    let mut indent = 0usize;
    let mut in_string = false;
    let mut escape_next = false;
    let bytes = json.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        let ch = bytes[i] as char;

        if escape_next {
            f.write_str(&json[i..i + 1])?;
            escape_next = false;
            i += 1;
            continue;
        }

        if in_string {
            f.write_str(&json[i..i + 1])?;
            if ch == '\\' {
                escape_next = true;
            } else if ch == '"' {
                in_string = false;
            }
            i += 1;
            continue;
        }

        match ch {
            '"' => {
                in_string = true;
                f.write_str("\"")?;
            }
            '{' | '[' => {
                indent += 1;
                f.write_str(&json[i..i + 1])?;
                // Check if next non-whitespace is closing bracket
                let next_meaningful = bytes[i + 1..].iter().position(|b| !b.is_ascii_whitespace());
                if let Some(pos) = next_meaningful {
                    let next_ch = bytes[i + 1 + pos] as char;
                    if next_ch == '}' || next_ch == ']' {
                        // Empty object/array -- don't add newline
                    } else {
                        f.write_str("\n")?;
                        write_indent(f, indent)?;
                    }
                }
            }
            '}' | ']' => {
                indent = indent.saturating_sub(1);
                f.write_str("\n")?;
                write_indent(f, indent)?;
                f.write_str(&json[i..i + 1])?;
            }
            ',' => {
                f.write_str(",\n")?;
                write_indent(f, indent)?;
            }
            ':' => {
                f.write_str(": ")?;
            }
            c if c.is_ascii_whitespace() => {
                // skip existing whitespace
            }
            _ => {
                f.write_str(&json[i..i + 1])?;
            }
        }
        i += 1;
    }
    Ok(())
}

fn write_indent(f: &mut std::fmt::Formatter<'_>, level: usize) -> std::fmt::Result {
    for _ in 0..level {
        f.write_str("  ")?;
    }
    Ok(())
}
