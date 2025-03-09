use base64::Engine;

/// Serialize a binary value as a base64 string.
pub struct SerializeBinary<'a> {
    pub value: &'a Vec<u8>,
}

impl<'a> serde::Serialize for SerializeBinary<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.collect_str(&base64::display::Base64Display::new(
            self.value,
            &base64::engine::general_purpose::STANDARD,
        ))
    }
}

/// Deserialize a base64 string into a binary value.
pub struct DeserializeBinary {
    pub value: Vec<u8>,
}

impl<'de> serde::Deserialize<'de> for DeserializeBinary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let field = String::deserialize(deserializer)?;
        base64::engine::general_purpose::STANDARD
            .decode(field.as_bytes())
            .map(|d| DeserializeBinary { value: d })
            .map_err(serde::de::Error::custom)
    }
}

/// Adapter from `std::io::Write` to `std::fmt::Formatter`. This allows to use
/// `serde_json::to_writer_pretty` to implement `std::fmt::Debug` for a struct.
///
/// There does not seem to be such utility in Rust std. Perhaps the usual case
/// is to adapt `std::fmt::Formatter` to `std::io::Write` instead.
pub struct FmtWriter<'a, 'b> {
    pub formatter: &'a mut std::fmt::Formatter<'b>,
}

impl<'a, 'b> std::io::Write for FmtWriter<'a, 'b> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // Convert bytes to string and write to formatter
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.formatter
                    .write_str(s)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                Ok(buf.len())
            }
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_binary() {
        let value = vec![1, 2, 3, 4, 5];
        let ser = SerializeBinary { value: &value };
        let json = serde_json::to_string(&ser).unwrap();
        assert_eq!(json, "\"AQIDBAU=\"");
    }

    #[test]
    fn test_deserialize_binary() {
        let json = "\"AQIDBAU=\"";
        let de: DeserializeBinary = serde_json::from_str(json).unwrap();
        assert_eq!(de.value, vec![1, 2, 3, 4, 5]);
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(tag = "_typeName")]
    pub struct Dog {
        pub name: String,
        pub breed: String,
    }

    impl std::fmt::Debug for Dog {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut writer = FmtWriter { formatter: f };
            serde_json::to_writer_pretty(&mut writer, self).map_err(|_| std::fmt::Error)
        }
    }

    #[test]
    fn test_dog_debug_implementation() {
        let dog = Dog {
            name: String::from("Rex"),
            breed: String::from("Golden Retriever"),
        };

        // Format the dog using Debug trait
        let debug_output = format!("{:?}", dog);

        // Parse the output as JSON to verify it's valid
        let parsed: serde_json::Value =
            serde_json::from_str(&debug_output).expect("Debug output should be valid JSON");

        // Check the values are correctly serialized
        assert_eq!(parsed["_typeName"], "Dog");
        assert_eq!(parsed["name"], "Rex");
        assert_eq!(parsed["breed"], "Golden Retriever");

        // Verify the output has pretty formatting (contains newlines and indentation)
        assert!(debug_output.contains("\n"));
        assert!(debug_output.contains("  "));
    }
}
