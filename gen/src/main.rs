mod oas30;
mod printer;
mod vim_model;
pub mod rs_emitter;

use std::io::Read;

fn load_openapi() -> oas30::OpenAPI {
    let mut file =
        std::fs::File::open("data/vi_json_openapi_specification_v8_0_2_0.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let openapi: oas30::OpenAPI = serde_json::from_str(&data).unwrap();
    openapi
}

fn main() {
    use crate::rs_emitter::TypesEmitter;
    let model = load_openapi();
    let root_folder = std::path::Path::new("../vim/src/");
    let vim_model = vim_model::load_vim_model(&model).unwrap();
    let file = std::fs::File::create(root_folder.join("types.rs")).expect("Could not create types.rs file");
    let mut printer = printer::FilePrinter::new(file, None, None);
    let mut emitter = TypesEmitter::new(&vim_model, &mut printer);
    emitter.emit_data_types().unwrap();

    for (mo_type, mo) in vim_model.managed_objects.iter() {
        let file_name =  root_folder.join(format!("{}.rs", mo_type));
        let file = std::fs::File::create(&file_name).expect(&format!("Could not create {} file", file_name.display()));
        let mut printer = printer::FilePrinter::new(file, None, None);
        let mut emitter = rs_emitter::ManagedObjectEmitter::new(&mo, &mut printer, &vim_model);
        emitter.emit().unwrap();
    }
}
