mod oas30;
mod vim_model;

use std::{borrow::Borrow, collections::HashSet, io::Read, path::Path};

use vim_model::{DataType::{Reference, Array}, Model, Struct};

fn load_openapi<P: AsRef<Path>>(path: P) -> oas30::OpenAPI {
    let mut file =
        std::fs::File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let openapi: oas30::OpenAPI = serde_json::from_str(&data).unwrap();
    openapi
}

fn main() {
    let vi_json_spec_path = Path::new("data/vi_json_openapi_specification_v8_0_2_0.json");
    let model = load_openapi(vi_json_spec_path);
    let vim_model = vim_model::load_vim_model(&model).unwrap();
    let loops = detect_loops(&vim_model);
    for loop_chain in loops {
        println!("Loop detected: {:?}", loop_chain);
    }
}

fn detect_loops(vim_model: &Model) -> Vec<Vec<(String, String)>> {
    let mut loops: Vec<Vec<(String, String)>> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();
    for (_ , struct_cell) in vim_model.structs.iter() {
        let mut stack: Vec<(String, String)> = Vec::new();
        let struct_ref = struct_cell.borrow();
        stack.push((struct_ref.name.clone(), String::new()));
        detect_loops_recursive(vim_model, struct_ref.borrow(), &mut stack, &mut visited, &mut loops);
    }
    loops
}


/// Depth first search to detect loops in the struct graph.
/// Iterates the fields of a struct and if a field is a struct type (with no
/// children) then it is added to the stack and the search continues. If the
/// a struct is found that is already in the stack then a loop is detected and
/// the loop is added to the loops vector.
fn detect_loops_recursive(vim_model: &Model, struct_ref: &Struct, stack: &mut Vec<(String, String)>, visited: &mut HashSet<String>, loops: &mut Vec<Vec<(String, String)>>) {
    for (_, field) in &struct_ref.fields {
        let ref_name = match &field.vim_type {
            Reference(ref_name) => {
                ref_name
            },
            Array(nested_type) => {
                if let Reference(ref_name) = nested_type.as_ref() {
                    ref_name
                } else {
                    continue;
                }
            },
            _ => {
                continue;
            }
        };
        let Some(struct_ref) = vim_model.structs.get(ref_name) else {
            continue;
        };
        if struct_ref.borrow().has_children() {
            continue;
        }
        if visited.contains(ref_name) {
            continue;
        }
        // Mutate the last frame to point to the current field in the current struct if we detect
        // a loop. The loop chain will be copied in the mutated state.
        stack.last_mut().unwrap().1 = field.name.clone();
        if stack.iter().any(|(name, _)| name == ref_name) {
            let loop_start = stack.iter().position(|(x, _)| x == ref_name).unwrap();
            // String implements Clone and we hope to_vec() will perform deep copy so further 
            // mutations of the stack do not affect the loop chain.
            let loop_chain = stack[loop_start..].to_vec();
            loops.push(loop_chain);
            continue;
        }
        let struct_ref = struct_ref.borrow();
        stack.push((ref_name.clone(), String::new()));
        detect_loops_recursive(vim_model, struct_ref.borrow(), stack, visited, loops);
        stack.pop();
    }
    visited.insert(struct_ref.name.clone());
}
