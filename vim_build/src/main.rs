mod printer;
mod vim_model;
mod generator;
pub mod rs_emitter;

use std::{path::Path, time::Instant};

use generator::emit_vim_bindings;

fn main() {
    let root_folder = Path::new("vim/src/");
    let vi_json_spec_path = Path::new("vim_build/data/vi_json_openapi_specification_v8_0_2_0.json");
    let start = Instant::now();
    emit_vim_bindings(vi_json_spec_path, root_folder).unwrap();
    println!("Total time in generation: {:?}", start.elapsed());
}

