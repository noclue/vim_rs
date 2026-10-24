# Design notes

These documents explain why `vim_rs` is shaped the way it is. The how-to for calling the API is the [crate guide](../README.md). The project overview, including the EuroRust 2025 talk, is the [repository README](../../README.md).

| Document | What it covers |
|----------|----------------|
| [TYPE_SYSTEM.md](TYPE_SYSTEM.md) | Traits, enums, compositional inheritance, pruned `MethodFault` and `Event` types, and why `Clone` / `PartialEq` are omitted. |
| [CLIENT.md](CLIENT.md) | Public `Client` facade, private JSON and SOAP transports, `VimClient`, and `close` versus destructor logout. |
| [REPOSITORY.md](REPOSITORY.md) | Crate map, regenerating bindings, updating the OpenAPI spec, and where generated code lives. |
| [METADATA_DRIVEN_XML_DE.md](METADATA_DRIVEN_XML_DE.md) | Schema-guided XML deserializer that drives the same miniserde visitors as JSON. |
| [SKIPPED_TYPE_XML_DE.md](SKIPPED_TYPE_XML_DE.md) | How pruned `MethodFault` and `Event` payloads are recovered from XML, which has less type information than JSON. |
