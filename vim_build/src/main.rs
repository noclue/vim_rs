mod generator;
mod printer;
pub mod rs_emitter;
mod vim_model;
mod json_emitter;

use generator::emit_vim_bindings;
use std::{path::Path, time::Instant};

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
static PRUNED_TYPES: [&str; 2] = ["MethodFault", "Event"];

fn main() {
    let root_folder = Path::new("../");
    let vi_json_spec_path = Path::new("data/vi_json_openapi_specification_v9_0_0_0_24798170.json");
    let mcp_output_dir = root_folder.join("mcp/data");  // Project root: vim_rs/mcp/data

    //generate_to_console(vi_json_spec_path).unwrap();
    let start = Instant::now();
    emit_vim_bindings(vi_json_spec_path, root_folder, Some(&PRUNED_TYPES)).unwrap();
    println!("Total time in Rust generation: {:?}", start.elapsed());

    // Generate MCP JSON data
    let mcp_start = Instant::now();
    match emit_mcp_data(vi_json_spec_path, &mcp_output_dir, Some(&PRUNED_TYPES)) {
        Ok(_) => println!("Total time in MCP JSON generation: {:?}", mcp_start.elapsed()),
        Err(e) => eprintln!("Error generating MCP JSON: {}", e),
    }
}

fn emit_mcp_data(
    vi_json_spec_path: &Path,
    output_dir: &Path,
    pruned_types: Option<&[&str]>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Load OpenAPI spec and transform to vim_model
    let openapi = generator::load_openapi(vi_json_spec_path)?;
    let model = vim_model::load_vim_model(&openapi, pruned_types)?;

    // Generate JSON files
    let pruned_types_slice = pruned_types.unwrap_or(&[]);
    json_emitter::emit_mcp_data(&model, output_dir, pruned_types_slice)?;

    Ok(())
}
