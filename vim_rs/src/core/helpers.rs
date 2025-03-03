use base64::Engine;

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

pub struct DeserializeBinary {
    pub value: Vec<u8>,
}

impl<'de> serde::Deserialize<'de> for DeserializeBinary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let field = String::deserialize(deserializer)?;
        base64::engine::general_purpose::STANDARD.decode(field.as_bytes())
            .map(|d| DeserializeBinary { value: d })
            .map_err(serde::de::Error::custom)
    }
}