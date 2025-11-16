pub mod json_emitter;

use anyhow::Result;
use std::path::Path;
use vim_build::{load_openapi, load_vim_model};

/// Types whose hierarchies are pruned from the model. The listed types will be generated. Their
/// descendants will not. This means that:
/// 1. struct types for the descendants will not be emitted
/// 2. traits for the listed types and their descendants will not be emitted
/// 3. additional 2 fields will be added to the listed types
///     - type_name_ : String - holding the discriminator of the deserialized types
///     - extra_fields_ : HashMap<String, serde_json::Value> - holding the extra fields that are not
///       part of the base type schema
/// 4. deserialization for the descendant types will be handled by the parent type Visitor. The
/// parent type Visitor will optionally accept the discriminator during creation to populate the
/// correct type_name_ for the descendant types.
pub const PRUNED_TYPES: [&str; 2] = ["MethodFault", "Event"];

/// Build API definitions from OpenAPI specification
pub fn build_api_definitions(vi_json_spec_path: &Path, output_dir: &Path) -> Result<()> {
    // Load OpenAPI spec and transform to vim_model
    let openapi = load_openapi(vi_json_spec_path)?;
    let model = load_vim_model(&openapi, Some(&PRUNED_TYPES))?;

    // Generate JSON files
    json_emitter::emit_mcp_data(&model, output_dir, &PRUNED_TYPES)
        .map_err(|e| anyhow::anyhow!("Failed to emit MCP data: {}", e))?;

    Ok(())
}

