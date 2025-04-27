use serde_json::Value;
use vim_rs::types::vim_any::VimAny;

pub fn to_json_value(value: &VimAny, name: &String) -> anyhow::Result<Value> {
    Ok(match value {
        VimAny::Value(value) => {
            let json_val = serde_json::to_value(value).map_err(|e| anyhow::anyhow!("Failed to convert value to JSON: {}", e))?;
            match json_val {
                Value::Object(mut obj) => {
                    let Some(value) = obj.remove("_value") else {
                        return Err(anyhow::anyhow!("Expected JSON object with _value field for property {}", name));
                    };
                    value
                }
                _ => {
                    return Err(anyhow::anyhow!("Expected JSON object for property '{}', got {:?}", name, json_val));
                }
            }
        }
        VimAny::Object(obj) => {
            serde_json::to_value(obj).map_err(|e| anyhow::anyhow!("Failed to convert property '{}' object to JSON: {}", name, e))?
        }
    })
}