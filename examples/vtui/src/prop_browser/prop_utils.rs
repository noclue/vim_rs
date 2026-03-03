use miniserde::json::Value;
use vim_rs::types::vim_any::VimAny;

pub fn to_json_value(value: &VimAny, name: &str) -> anyhow::Result<Value> {
    Ok(match value {
        VimAny::Value(val) => {
            let json_str = miniserde::json::to_string(val);
            let json_val: Value =
                miniserde::json::from_str(&json_str).map_err(|e| anyhow::anyhow!("Failed to parse value JSON: {}", e))?;
            match json_val {
                Value::Object(mut obj) => {
                    let value = obj
                        .remove("_value")
                        .ok_or_else(|| anyhow::anyhow!("Expected JSON object with _value field for property {}", name))?;
                    value
                }
                _ => {
                    return Err(anyhow::anyhow!(
                        "Expected JSON object for property '{}', got {:?}",
                        name,
                        json_val
                    ));
                }
            }
        }
        VimAny::Object(obj) => {
            let json_str = miniserde::json::to_string(obj.as_ref());
            miniserde::json::from_str(&json_str).map_err(|e| {
                anyhow::anyhow!("Failed to convert property '{}' object to JSON: {}", name, e)
            })?
        }
    })
}
