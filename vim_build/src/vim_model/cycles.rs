/// Logic in this module detect cycles in the data model and selects fields to be boxed to break the
/// cycle.
use std::{borrow::Borrow, collections::HashSet};
use std::ops::Deref;
use super::{DataType::Reference, EmitMode, Error, Model, Result, Struct};

/// Identify cycles in the data model, select field references where the cycles can be broken and
/// mark thge selected fields to be boxed to break the cycle.
pub fn mark_cycles(vim_model: &mut Model) -> Result<()> {
    let loops = detect_loops(vim_model);
    let boxed_fields = select_boxed_fields(vim_model, &loops);
    for field in boxed_fields {
        let struct_cell = vim_model
            .structs
            .get(&field.0)
            .ok_or_else(|| Error::InvalidReference(field.0.clone()))?;
        let mut struct_ref = struct_cell.borrow_mut();
        let field = struct_ref
            .fields
            .get_mut(&field.1)
            .ok_or_else(|| Error::InvalidReference(format!("{}::{}", field.0, field.1)))?;
        field.require_box = true;
    }
    Ok(())
}

/// Select one struct and field from each loop chain to be boxed so as to break the loop. We need
/// the results to be repeatable between runs. So we will select the first struct in alpabetical
/// order.
fn select_boxed_fields(_: &Model, loops: &Vec<Vec<(String, String)>>) -> Vec<(String, String)> {
    let mut boxed_fields: Vec<(String, String)> = Vec::new();
    for loop_chain in loops {
        let mut selected_field = loop_chain[0].clone();
        for field in loop_chain {
            if field.0 < selected_field.0 {
                selected_field = field.clone();
            }
        }
        boxed_fields.push(selected_field);
    }
    boxed_fields
}

fn detect_loops(vim_model: &Model) -> Vec<Vec<(String, String)>> {
    let mut loops: Vec<Vec<(String, String)>> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();
    for (_, struct_cell) in vim_model.structs.iter() {
        let mut stack: Vec<(String, String)> = Vec::new();
        let struct_ref = struct_cell.borrow();
        stack.push((struct_ref.name.clone(), String::new()));
        detect_loops_recursive(
            vim_model,
            struct_ref.borrow(),
            &mut stack,
            &mut visited,
            &mut loops,
        );
    }
    loops
}

/// Depth first search to detect loops in the struct graph.
/// Iterates the fields of a struct and if a field is a struct type (with no
/// children) then it is added to the stack and the search continues. If the
/// a struct is found that is already in the stack then a loop is detected and
/// the loop is added to the loops vector.
fn detect_loops_recursive(
    vim_model: &Model,
    struct_ref: &Struct,
    stack: &mut Vec<(String, String)>,
    visited: &mut HashSet<String>,
    loops: &mut Vec<Vec<(String, String)>>,
) {
    for (_, field) in &struct_ref.fields {
        let ref_name = match &field.vim_type {
            Reference(ref_name) => ref_name,
            // arrays are ok as Vec field in a struct is a pointer to the array and not the array
            // itself hence cyclic references from Vec do not violate Sized constraint.
            _ => {
                continue;
            }
        };
        let Some(mut struct_ref) = vim_model.structs.get(ref_name) else {
            continue;
        };
        if let EmitMode::Skip(ref pruned) = struct_ref.borrow().emit_mode {
            if let Some(pruned_ref) = vim_model.structs.get(pruned) {
                struct_ref = pruned_ref;
            } else {
                panic!("Invalid pruned type reference: {}", pruned);
            }
        }
        let struct_ref = struct_ref.borrow();
        if struct_ref.has_children() && struct_ref.emit_mode == EmitMode::Emit {
            continue;
        }
        if visited.contains(ref_name) {
            continue;
        }
        // Mutate the last frame to point to the current field in the current struct. If we detect
        // a loop the loop chain will be copied in the mutated state.
        stack.last_mut().unwrap().1 = field.name.clone();
        if stack.iter().any(|(name, _)| name == ref_name) {
            let loop_start = stack.iter().position(|(x, _)| x == ref_name).unwrap();
            // String implements Clone and we hope to_vec() will perform deep copy so further
            // mutations of the stack do not affect the loop chain.
            let loop_chain = stack[loop_start..].to_vec();
            loops.push(loop_chain);
            continue;
        }
        stack.push((ref_name.clone(), String::new()));
        detect_loops_recursive(vim_model, struct_ref.deref(), stack, visited, loops);
        stack.pop();
    }
    visited.insert(struct_ref.name.clone());
}
