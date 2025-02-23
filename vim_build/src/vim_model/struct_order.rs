use std::cell::RefCell;
use std::collections::HashMap;
use indexmap::IndexMap;
use crate::vim_model::{Struct, Result, Error};

pub fn reorder_structs(structs: &mut IndexMap<String, RefCell<Struct>>) -> Result<IndexMap<String, RefCell<Struct>>> {
    let (ordered, last_child) = generate_last_child_map(structs, "Any");
    let mut structs = order_structs(structs, &ordered)?;
    populate_last_child(&mut structs, &last_child)?;
    Ok(structs)
}


fn populate_last_child(structs: &IndexMap<String, RefCell<Struct>>, last_child: &HashMap<String, String>) -> Result<()> {
    if structs.len() != last_child.len() {
        return Err(Error::InternalProcessingError("Last child map does not match struct count".to_string()));
    }
    for (name, struct_ref) in structs.iter() {
        let mut struct_ref = struct_ref.borrow_mut();
        let last_child = last_child.get(name).cloned();
        let Some(last_child) = last_child else {
            return Err(Error::InvalidReference(name.clone()));
        };
        struct_ref.last_child = last_child;
    }
    Ok(())
}

fn order_structs(structs: &mut IndexMap<String, RefCell<Struct>>, order: &[String]) -> Result<IndexMap<String, RefCell<Struct>>> {
    if structs.len() != order.len() {
        return Err(Error::InternalProcessingError("Order list length does not match the struct count".to_string()));
    }
    let mut ordered_structs = IndexMap::new();
    for name in order {
        let (key, value) = structs.swap_remove_entry(name).ok_or_else(|| Error::InvalidReference(name.clone()))?;
        ordered_structs.insert(key, value);
    }
    Ok(ordered_structs)
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