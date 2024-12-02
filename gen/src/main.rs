mod oas30;
mod printer;
mod vim_model;
pub mod rs_emitter;

use std::{io::Read, path::Path, time::Instant};
use convert_case::{Case, Casing};
use rs_emitter::library::emit_library;

fn load_openapi<P: AsRef<Path>>(path: P) -> oas30::OpenAPI {
    let mut file =
        std::fs::File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let openapi: oas30::OpenAPI = serde_json::from_str(&data).unwrap();
    openapi
}

fn main() {
    let root_folder = Path::new("../vim/src/");
    let vi_json_spec_path = Path::new("data/vi_json_openapi_specification_v8_0_2_0.json");
    let start = Instant::now();
    emit_vim_bindings(vi_json_spec_path, root_folder);
    println!("Total time in generation: {:?}", start.elapsed());
}
     
fn emit_vim_bindings(vi_json_spec_path: &Path, root_folder: &Path) {
    let start_load = Instant::now();
    let model = load_openapi(vi_json_spec_path);
    println!("Time to load OpenAPI: {:?}", start_load.elapsed());
    let start_model_transform = Instant::now();
    let vim_model = vim_model::load_vim_model(&model).unwrap();
    println!("Time to transform OpenAPI model: {:?}", start_model_transform.elapsed());

    let start_emit = Instant::now();
    emit_types(root_folder, &vim_model);
    println!("Time to emit types: {:?}", start_emit.elapsed());

    let start_emit_mo = Instant::now();
    let modules = emit_managed_objects(root_folder, &vim_model);
    println!("Time to emit managed objects: {:?}", start_emit_mo.elapsed());

    // Generate lib.rs
    let file = std::fs::File::create(root_folder.join("lib.rs")).expect("Could not create lib.rs file");
    let mut printer = printer::FilePrinter::new(file, None, None);
    emit_library(&modules, &mut printer).unwrap();
}

fn emit_managed_objects(root_folder: &Path, vim_model: &vim_model::Model) -> Vec<String> {
    let mut modules = Vec::new();
    for (mo_type, mo) in vim_model.managed_objects.iter() {
        if mo.methods.is_empty() {
            continue; // Skip managed objects without methods
        }
        let file_name = mo_type.to_case(Case::Snake);
        modules.push(file_name.clone());
        let file_path =  root_folder.join(format!("{}.rs", file_name));
        let file = std::fs::File::create(&file_path).expect(&format!("Could not create {} file", file_path.display()));
        let mut printer = printer::FilePrinter::new(file, None, None);
        let mut emitter = rs_emitter::ManagedObjectEmitter::new(&mo, &mut printer, &vim_model);
        emitter.emit().unwrap();
    }
    modules
}

fn emit_types(root_folder: &Path, vim_model: &vim_model::Model) {
    use crate::rs_emitter::TypesEmitter;
    let file = std::fs::File::create(root_folder.join("types.rs")).expect("Could not create types.rs file");
    let mut printer = printer::FilePrinter::new(file, None, None);
    let mut emitter = TypesEmitter::new(vim_model, &mut printer);
    emitter.emit_data_types().unwrap();
}
