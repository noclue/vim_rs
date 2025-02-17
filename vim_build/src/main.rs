mod printer;
mod vim_model;
mod generator;
pub mod rs_emitter;

use std::{path::Path, time::Instant};
use std::cell::RefCell;
use std::collections::HashMap;
use indexmap::IndexMap;
use generator::emit_vim_bindings;
use crate::generator::load_openapi;
use crate::vim_model::Struct;

fn main() {
    let root_folder = Path::new("../vim/src/");
    let vi_json_spec_path = Path::new("data/vi_json_openapi_specification_v8_0_2_0.json");

    generate_type_enum(vi_json_spec_path).unwrap();
    // let start = Instant::now();
    // emit_vim_bindings(vi_json_spec_path, root_folder).unwrap();
    // println!("Total time in generation: {:?}", start.elapsed());
}


fn generate_type_enum(vi_json_spec_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let model = load_openapi(vi_json_spec_path)?;
    let vim_model = vim_model::load_vim_model(&model)?;
    let (ordered, last_child) = generate_last_child_map(&vim_model.structs, "Any");

    let mut indent = 0;
    let mut stack = Vec::new();
    for ref struct_name in ordered {
        print_indent(indent);
        println!("{}", struct_name);
        let last = last_child.get(struct_name).unwrap();
        if last != struct_name {
            indent += 1;
            stack.push((last.clone()));
        } else {
            while stack.last() == Some(&struct_name) {
                stack.pop();
                indent -= 1;
            }
        }
    }
    Ok(())
}

fn print_indent(indent: usize) {
    for _ in 0..indent {
        print!("  ");
    }
}


fn generate_order(structs: &IndexMap<String, RefCell<Struct>>) -> Vec<String> {
    let mut result = Vec::new();
    let mut stack = Vec::new();
    // Assuming the root is named "DataObject"
    stack.push("Any".to_string());

    while let Some(current_name) = stack.pop() {
        result.push(current_name.clone());
        let struct_ = structs.get(&current_name).expect("Node not found").borrow();
        // Iterate children in reverse to maintain original order when pushed onto the stack
        for child in struct_.children.iter().rev() {
            stack.push(child.clone());
        }
    }

    result
}


fn generate_last_child_map(
    structs: &IndexMap<String, RefCell<Struct>>,
    root_name: &str,
) -> (Vec<String>, HashMap<String, String>) {
    enum StackEntry {
        Discover(String),   // First encounter (pre-order visit)
        Finalize(String),   // Ready to record last child
    }

    let mut traversal_order = Vec::new();
    let mut last_child_map = HashMap::new();
    let mut stack = vec![StackEntry::Discover(root_name.to_string())];

    while let Some(entry) = stack.pop() {
        match entry {
            StackEntry::Discover(name) => {
                // Record discovery time (pre-order position)
                traversal_order.push(name.clone());

                // Push Finalize phase after children are processed
                stack.push(StackEntry::Finalize(name.clone()));

                // Push children in reverse order to maintain original order
                let struct_ref = structs.get(&name).expect("Node not found").borrow();
                for child in struct_ref.children.iter().rev() {
                    stack.push(StackEntry::Discover(child.clone()));
                }
            }
            StackEntry::Finalize(name) => {
                // The last element added after processing children is our last descendant
                let last_child = traversal_order.last().unwrap().clone();
                last_child_map.insert(name, last_child);
            }
        }
    }

    (traversal_order, last_child_map)
}