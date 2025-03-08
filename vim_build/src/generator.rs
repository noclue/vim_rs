use super::printer::{self, FilePrinter, Printer};
use super::rs_emitter;
use super::vim_model;
use crate::rs_emitter::deser::DeserializationGenerator;
use crate::rs_emitter::trait_emitter::TraitEmitter;
use convert_case::{Case, Casing};
use rs_emitter::library::emit_library;
use std::path::PathBuf;
use std::{io::Read, path::Path, time::Instant};
use openapi30::OpenAPI;
use crate::vim_model::{EmitMode, Model};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("OpenAPI parse error: {0}")]
    Parse(#[from] openapi30::Error),
    #[error("Printer error: {0}")]
    Print(#[from] printer::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Emit error: {0}")]
    Emit(#[from] rs_emitter::errors::Error),
    #[error("VimModelError: {0}")]
    VimModel(#[from] vim_model::Error),
}

type Result<T> = std::result::Result<T, Error>;

pub fn load_openapi<P: AsRef<Path>>(path: P) -> Result<openapi30::OpenAPI> {
    let mut file = std::fs::File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let openapi: openapi30::OpenAPI = serde_json::from_str(&data)?;
    Ok(openapi)
}

pub fn emit_vim_bindings(vi_json_spec_path: &Path, root_folder: &Path, pruned_types: Option<&[&str]>) -> Result<()> {
    let start_load = Instant::now();
    let model = load_openapi(vi_json_spec_path)?;
    println!("Time to load OpenAPI: {:?}", start_load.elapsed());
    let vim_model = transform_model(&model, pruned_types)?;
    generate_bindings(vim_model, root_folder)
}

pub fn generate_bindings(vim_model: Model, root_folder: &Path) -> Result<()> {
    let start_emit = Instant::now();
    emit_types(root_folder, &vim_model)?;
    println!("Time to emit types: {:?}", start_emit.elapsed());

    let start_emit_mo = Instant::now();
    emit_managed_objects(root_folder, &vim_model)?;
    println!(
        "Time to emit managed objects: {:?}",
        start_emit_mo.elapsed()
    );
    Ok(())
}

fn transform_model(model: &OpenAPI, pruned_types: Option<&[&str]>) -> Result<Model> {
    let start_model_transform = Instant::now();
    let vim_model = vim_model::load_vim_model(model, pruned_types)?;
    println!(
        "Time to transform OpenAPI model: {:?}",
        start_model_transform.elapsed()
    );
    Ok(vim_model)
}

fn emit_managed_objects(root_folder: &Path, vim_model: &vim_model::Model) -> Result<()> {
    let mo_folder = root_folder.join("mo");
    std::fs::create_dir_all(&mo_folder).expect("Could not create mo folder");

    let mut modules = Vec::new();
    for (mo_type, mo) in vim_model.managed_objects.iter() {
        if mo.methods.is_empty() {
            continue; // Skip managed objects without methods
        }
        let file_name = mo_type.to_case(Case::Snake);
        modules.push(file_name.clone());
        let file_path = mo_folder.join(format!("{}.rs", file_name));
        let file = std::fs::File::create(&file_path)
            .unwrap_or_else(|_| panic!("Could not create {} file", file_path.display()));
        let mut printer = printer::FilePrinter::new(file, None, None);
        let mut emitter = rs_emitter::ManagedObjectEmitter::new(mo, &mut printer, vim_model);
        emitter.emit()?;
    }
    // Generate mod.rs
    let file =
        std::fs::File::create(mo_folder.join("mod.rs")).expect("Could not create mo/mod.rs file");
    let mut printer = printer::FilePrinter::new(file, None, None);
    emit_library(&modules, &mut printer)?;
    Ok(())
}

fn emit_types(root_folder: &Path, vim_model: &vim_model::Model) -> Result<()> {
    let types_folder = root_folder.join("types");
    std::fs::create_dir_all(&types_folder).expect("Could not create types folder");

    emit_ser(&types_folder, vim_model)?;
    emit_de(&types_folder, vim_model)?;
    emit_struct_enum(&types_folder, vim_model)?;
    emit_boxed_types(&types_folder, vim_model)?;

    emit_enums(&types_folder, vim_model)?;
    emit_structs(&types_folder, vim_model)?;

    // Emit traits
    delete_trait_files(&types_folder)?;
    emit_vim_object_trait(&types_folder, vim_model)?;
    emit_inheritable_traits(&types_folder, vim_model)?;

    emit_mod_rs(&types_folder)?;
    Ok(())
}

// Delete *_trait.rs files
fn delete_trait_files(types_folder: &Path) -> Result<()> {
    let files = std::fs::read_dir(types_folder)?;
    for file in files {
        let file = file?;
        let file_name = file.file_name();
        let file_name = file_name.to_string_lossy();
        if file_name.ends_with("_trait.rs") {
            std::fs::remove_file(file.path())?;
        }
    }
    Ok(())
}

fn emit_vim_object_trait(root_folder: &Path, vim_model: &vim_model::Model) -> Result<()> {
    let file = std::fs::File::create(root_folder.join("vim_object_trait.rs"))?;
    let mut printer = printer::FilePrinter::new(file, None, None);
    rs_emitter::vim_object::generate_vim_object_trait(vim_model, &mut printer)?;
    Ok(())
}

fn emit_inheritable_traits(root_folder: &Path, vim_model: &vim_model::Model) -> Result<()> {
    let file_name = "traits.rs";
    let mut printer = printer_for_file(root_folder.join(file_name))?;
    TraitEmitter::emit_imports(&mut printer)?;
    for (struct_name, struct_ref) in &vim_model.structs {
        let struct_ref = struct_ref.borrow();
        if struct_name == rs_emitter::structs::ANY || struct_ref.children.is_empty() {
            continue;
        }
        if struct_ref.emit_mode != EmitMode::Emit {
            continue;
        }

        let mut trait_emitter = TraitEmitter::new(struct_name.clone(), vim_model, &mut printer);

        trait_emitter.emit_trait()?
    }
    Ok(())
}

fn emit_boxed_types(types_folder: &Path, vim_model: &vim_model::Model) -> Result<()> {
    let mut printer = printer_for_file(types_folder.join("boxed_types.rs"))?;
    let mut types_emitter =
        rs_emitter::boxed_types::BoxedTypesEmitter::new(vim_model, &mut printer);
    types_emitter.emit_boxed_types()?;

    Ok(())
}

fn emit_struct_enum(types_folder: &Path, vim_model: &vim_model::Model) -> Result<()> {
    let file = std::fs::File::create(types_folder.join("struct_enum.rs"))
        .expect("Could not create struct_enum.rs file");
    let mut printer = printer::FilePrinter::new(file, None, None);
    rs_emitter::struct_enum::generate_struct_enum(vim_model, &mut printer)?;
    Ok(())
}

fn emit_ser(types_folder: &Path, vim_model: &vim_model::Model) -> Result<()> {
    let file = std::fs::File::create(types_folder.join("dyn_serialize.rs"))
        .expect("Could not create dyn_serialize.rs file");
    let mut printer = printer::FilePrinter::new(file, None, None);
    rs_emitter::ser::generate_serialize_polymorphic_enum(vim_model, &mut printer)?;
    Ok(())
}

fn emit_de(types_folder: &Path, vim_model: &vim_model::Model) -> Result<()> {
    let mut printer = printer_for_file(types_folder.join("deserialize.rs"))?;
    printer.println("use std::fmt;")?;
    printer.println("use serde::Deserializer;")?;
    printer.println("use serde::de;")?;
    printer.println("use super::boxed_types::ValueElements;")?;
    printer.println("use super::structs::*;")?;
    printer.println("use super::vim_any::VimAny;")?;

    let mut gen = DeserializationGenerator::new(vim_model, &mut printer);
    gen.generate_deserialization()?;
    Ok(())
}

fn emit_enums(types_folder: &Path, vim_model: &vim_model::Model) -> Result<()> {
    let file = std::fs::File::create(types_folder.join("enums.rs"))
        .expect("Could not create enums.rs file");
    let mut printer = printer::FilePrinter::new(file, None, None);
    rs_emitter::enums::emit_enums(vim_model, &mut printer)?;
    Ok(())
}

fn emit_structs(root_folder: &Path, vim_model: &vim_model::Model) -> Result<()> {
    use crate::rs_emitter::TypesEmitter;
    let file = std::fs::File::create(root_folder.join("structs.rs"))
        .expect("Could not create structs file");
    let mut printer = printer::FilePrinter::new(file, None, None);
    let mut emitter = TypesEmitter::new(vim_model, &mut printer);
    emitter.emit_data_types()?;
    Ok(())
}

fn emit_mod_rs(types_folder: &std::path::Path) -> Result<()> {
    let mut p = printer_for_file(types_folder.join("mod.rs"))?;
    p.println("pub mod enums;")?;
    p.println("pub mod structs;")?;
    p.println("pub mod traits;")?;
    p.println("pub mod dyn_serialize;")?;
    p.println("pub mod deserialize;")?;
    p.println("pub mod struct_enum;")?;
    p.println("pub mod boxed_types;")?;
    p.println("pub mod vim_any;")?;
    p.println("pub mod as_any;")?;
    p.println("pub mod convert;")?;
    p.println("pub mod vim_object_trait;")?;
    p.newline()?;
    Ok(())
}

fn printer_for_file(file_path: PathBuf) -> Result<FilePrinter> {
    let file = std::fs::File::create(file_path)?;
    Ok(printer::FilePrinter::new(file, None, None))
}
