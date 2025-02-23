pub mod generator;
pub mod printer;
pub mod rs_emitter;
pub mod vim_model;

use std::path::Path;
use generator::{generate_bindings, load_openapi};

pub struct Generation {
    vim_spec: Option<String>,
    root_folder: String,
}

pub fn configurator(root_folder: String) -> Generation {
    Generation {
        vim_spec: None,
        root_folder: root_folder,
    }
}

fn get_default_api() -> Result<openapi30::OpenAPI, Box<dyn std::error::Error>> {
    let openapi = include_str!("../data/vi_json_openapi_specification_v8_0_2_0.json");
    let spec: openapi30::OpenAPI = serde_json::from_str(openapi)?;
    Ok(spec)
}

impl Generation {
    pub fn vim_spec(&mut self, vim_spec: String) -> &mut Self {
        self.vim_spec = Some(vim_spec);
        self
    }



    pub fn generate(&self) -> Result<(), Box<dyn std::error::Error>> {
        let model = if let Some(vim_spec) = self.vim_spec.as_ref() {
            load_openapi(vim_spec)?
        } else {
            get_default_api()?
        };
        Ok(generate_bindings(model, Path::new(&self.root_folder))?)
    }
}

