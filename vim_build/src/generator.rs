use std::{io::Read, path::Path, time::Instant};
use convert_case::{Case, Casing};
use super::printer::{self, Printer};
use super::vim_model;
use super::rs_emitter;
use rs_emitter::library::emit_library;

pub fn load_openapi<P: AsRef<Path>>(path: P) -> Result<openapi30::OpenAPI, Box<dyn std::error::Error>> {
    let mut file =
        std::fs::File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let openapi: openapi30::OpenAPI = serde_json::from_str(&data)?;
    Ok(openapi)
}

pub fn emit_vim_bindings(vi_json_spec_path: &Path, root_folder: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let start_load = Instant::now();
    let model = load_openapi(vi_json_spec_path)?;
    println!("Time to load OpenAPI: {:?}", start_load.elapsed());
    return generate_bindings(model, root_folder);
}

pub fn generate_bindings(model: openapi30::OpenAPI, root_folder: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let start_model_transform = Instant::now();
    let vim_model = vim_model::load_vim_model(&model)?;
    println!("Time to transform OpenAPI model: {:?}", start_model_transform.elapsed());

    let start_emit = Instant::now();
    emit_types(root_folder, &vim_model)?;
    println!("Time to emit types: {:?}", start_emit.elapsed());

    // let start_emit_mo = Instant::now();
    // emit_managed_objects(root_folder, &vim_model)?;
    // println!("Time to emit managed objects: {:?}", start_emit_mo.elapsed());
    Ok(())
}

fn emit_managed_objects(root_folder: &Path, vim_model: &vim_model::Model) -> Result<(), Box<dyn std::error::Error>> {
    let mo_folder = root_folder.join("mo");
    std::fs::create_dir_all(&mo_folder).expect("Could not create mo folder");

    let mut modules = Vec::new();
    for (mo_type, mo) in vim_model.managed_objects.iter() {
        if mo.methods.is_empty() {
            continue; // Skip managed objects without methods
        }
        let file_name = mo_type.to_case(Case::Snake);
        modules.push(file_name.clone());
        let file_path =  mo_folder.join(format!("{}.rs", file_name));
        let file = std::fs::File::create(&file_path).expect(&format!("Could not create {} file", file_path.display()));
        let mut printer = printer::FilePrinter::new(file, None, None);
        let mut emitter = rs_emitter::ManagedObjectEmitter::new(&mo, &mut printer, &vim_model);
        emitter.emit()?;
    }
    // Generate mod.rs
    let file = std::fs::File::create(mo_folder.join("mod.rs")).expect("Could not create mo/mod.rs file");
    let mut printer = printer::FilePrinter::new(file, None, None);
    emit_library(&modules, &mut printer)?;
    Ok(())
}

fn emit_types(root_folder: &Path, vim_model: &vim_model::Model) -> Result<(), Box<dyn std::error::Error>> {
    let types_folder = root_folder.join("types");
    std::fs::create_dir_all(&types_folder).expect("Could not create types folder");
    emit_enums(&types_folder, &vim_model)?;
    emit_structs(&types_folder, &vim_model)?;

    // Generate mod.rs
    let file = std::fs::File::create(types_folder.join("mod.rs")).expect("Could not create types/mod.rs file");
    let mut p = printer::FilePrinter::new(file, None, None);
    p.println("pub mod enums;")?;
    p.println("pub mod structs;")?;
    Ok(())
}

fn emit_enums(types_folder: &Path, vim_model: &vim_model::Model) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(types_folder.join("enums.rs")).expect("Could not create enums.rs file");
    let mut printer = printer::FilePrinter::new(file, None, None);
    rs_emitter::enums::emit_enums(&vim_model, &mut printer)?;
    Ok(())
}

fn emit_structs(root_folder: &Path, vim_model: &vim_model::Model) -> Result<(), Box<dyn std::error::Error>> {
    use crate::rs_emitter::TypesEmitter;
    let file = std::fs::File::create(root_folder.join("structs.rs")).expect("Could not create types.rs file");
    let mut printer = printer::FilePrinter::new(file, None, None);
    let mut emitter = TypesEmitter::new(vim_model, &mut printer);
    emitter.emit_data_types()?;
    Ok(())
}