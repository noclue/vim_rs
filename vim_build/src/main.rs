mod generator;
mod printer;
pub mod rs_emitter;
mod vim_model;

use generator::emit_vim_bindings;
use std::{path::Path, time::Instant};
// use crate::generator::load_openapi;
// use crate::printer::Printer;

fn main() {
    let root_folder = Path::new("../vim/src/");
    let vi_json_spec_path = Path::new("data/vi_json_openapi_specification_v8_0_2_0.json");

    //generate_to_console(vi_json_spec_path).unwrap();
    let start = Instant::now();
    emit_vim_bindings(vi_json_spec_path, root_folder).unwrap();
    println!("Total time in generation: {:?}", start.elapsed());
}

// fn generate_to_console(vi_json_spec_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
//     let model = load_openapi(vi_json_spec_path)?;
//     let vim_model = vim_model::load_vim_model(&model)?;
//     let mut printer = printer::StdoutPrinter::new(None, None);
//     Ok(())
// }
