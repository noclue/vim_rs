mod generator;
mod printer;
pub mod rs_emitter;
mod vim_model;

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
    let root_folder = Path::new("../vim_rs/src/");
    let vi_json_spec_path = Path::new("data/vi_json_openapi_specification_v8_0_2_0.json");

    //generate_to_console(vi_json_spec_path).unwrap();
    let start = Instant::now();
    emit_vim_bindings(vi_json_spec_path, root_folder, Some(&PRUNED_TYPES)).unwrap();
    println!("Total time in generation: {:?}", start.elapsed());
}

// fn generate_to_console(vi_json_spec_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
//     let model = load_openapi(vi_json_spec_path)?;
//     let vim_model = vim_model::load_vim_model(&model)?;
//     let mut printer = printer::StdoutPrinter::new(None, None);
//     Ok(())
// }
