

pub mod vec {
    use serde::{Deserialize, Deserializer, Serializer};
    use base64::{self, Engine};

    pub fn serialize<S: Serializer>(v: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&base64::display::Base64Display::new(
            v,
            &base64::engine::general_purpose::STANDARD
        ))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let base64 = String::deserialize(d)?;
        base64::engine::general_purpose::STANDARD.decode(base64.as_bytes())
            .map_err(|e| serde::de::Error::custom(e))
    }
}

pub mod option {
    use serde::{Deserialize, Deserializer, Serializer};
    use base64::{self, Engine};

    pub fn serialize<S: Serializer>(v: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error> {
        match v {
            Some(v) => serializer.collect_str(&base64::display::Base64Display::new(
                v,
                &base64::engine::general_purpose::STANDARD
            )),
            None => serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Vec<u8>>, D::Error> {
        let base64 = Option::<String>::deserialize(d)?;
        match base64 {
            Some(base64) => base64::engine::general_purpose::STANDARD.decode(base64.as_bytes())
                .map_err(|e| serde::de::Error::custom(e))
                .map(Some),
            None => Ok(None)
        }
    }
}