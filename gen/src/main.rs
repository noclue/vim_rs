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
    use crate::rs_emitter::RsEmitter;
    let model = load_openapi();
    let vim_model = vim_model::load_vim_model(&model).unwrap();
    let file = std::fs::File::create("../vim/src/vim.rs").expect("Could not create file");
    let mut printer = printer::FilePrinter::new(file, None, None);
    let mut emitter = RsEmitter::new(&vim_model, &mut printer);
    emitter.emit_data_types().unwrap();
}
