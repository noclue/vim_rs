// Binary serialization/deserialization is now handled by:
// - Serialization: base64::display::Base64Display in generated Serializer structs
// - Deserialization: crate::types::mini_helpers::Base64 wrapper in generated Fields structs
//
// The SerializeBinary/DeserializeBinary wrappers and FmtWriter are no longer needed.
// Pretty-print Debug is handled by crate::types::mini_helpers::write_pretty_json.
