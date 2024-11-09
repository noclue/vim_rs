
use std::borrow::Borrow;

///! Codify the rust convention on names from https://github.com/rust-lang/rust/blob/0bb6fec1c9aa484a7ec987a9e8ffca2eb647b0b3/src/doc/style-guide/src/advice.md
///! Also apply safe rust naming conventions 
///! We use:
///! - check_keyword to check if the name is a keyword and make it safe
///! - convert_case to convert the name as needed

use convert_case::{Case, Casing};
use check_keyword::CheckKeyword;

use crate::vim_model::{Struct, VimModel, VimType};
use crate::rs_emitter::errors::{Result, Error};

pub fn to_type_name(name: &str) -> String {
    name.to_case(Case::Pascal).into_safe()
}

pub fn to_enum_variant(name: &str) -> String {
    name.to_case(Case::Pascal).into_safe()
}

pub fn to_field_name(name: &str) -> String {
    name.to_case(Case::Snake).into_safe()
}

pub fn getter_name(name: &str) -> String {
    format!("get_{}", name.to_case(Case::Snake))
}

pub fn any_into_name(name: &str) -> String {
    format!("any_into_{}", name.to_case(Case::Snake))
}


pub fn to_fn_name(name: &str) -> String {
    name.to_case(Case::Snake).into_safe()
}

/// Apply a wrapper to a type declaration. The wrapper is a function that takes a string and
/// returns a string. Bor example to add a Box<> to a type declaration use `box_type_declaration`.
type TypeWrapper = fn (&str) -> String;


pub struct TypeDefResolver<'a> {
    vim_model: &'a VimModel,
}

impl TypeDefResolver<'_> {
    pub fn new(vim_model: &VimModel) -> TypeDefResolver {
        TypeDefResolver { vim_model }
    }

    /// Convert a VimType to a Rust type declaration. Structure types are always boxed. To use
    /// borrow semantics instead of boxing use `to_rust_type_with_wrapper` with 
    /// `ref_type_declaration` instead.
    pub fn to_rust_type(&mut self, vim_type: &VimType) -> Result<String> {
        self.to_rust_type_with_wrapper(vim_type, box_type_declaration)
    }

    /// Convert a VimType to a Rust type declaration. Allowing to customize the wrapper of the top
    /// level reference types. By using `box_type_declaration` the top level structs are boxed. In
    /// case of method calls we want to use borrow semantics instead to avoid boxing. This change of
    /// Box to borrow is done by `ref_type_declaration`. The `type_wrapper` function is used only on
    /// the top level structure types.
    pub fn to_rust_type_with_wrapper(&mut self, vim_type: &VimType, type_wrapper: TypeWrapper) -> Result<String> {
        match &vim_type {
            VimType::Boolean => Ok("bool".to_string()),
            VimType::String => Ok("String".to_string()),
            VimType::Binary => Ok("Vec<u8>".to_string()),
            VimType::Int8 => Ok("i8".to_string()),
            VimType::Int16 => Ok("i16".to_string()),
            VimType::Int32 => Ok("i32".to_string()),
            VimType::Int64 => Ok("i64".to_string()),
            VimType::Float => Ok("f32".to_string()),
            VimType::Double => Ok("f64".to_string()),
            VimType::DateTime => Ok("String".to_string()),
            VimType::Array(nested_type) => Ok(format!("Vec<{}>", self.to_rust_type(nested_type)?)),
            VimType::Reference(ref_name) => Ok(self.get_ref_type_declaration(ref_name, type_wrapper)?),
        }
    }
    
    /// Generate the type declaration for a reference field - enum or struct.
    /// If enum return just the enum Pascal case name. If structure then return a Box<> 
    /// If the structure has children then we need dynamic trait reference.
    /// If the Struct has no children then we reference the Struct type.
    fn get_ref_type_declaration(&mut self, ref_name: &str, type_wrapper: TypeWrapper) -> Result<String> {
        // If we cannot find the struct this is programatic error
        let rust_name = to_type_name(ref_name);
        if ref_name == "Any" {
            return Ok("VimAny".to_string());
        }
        if let Some(struct_type) = self.vim_model.structs.get(ref_name) {
            let struct_ref = struct_type.borrow();
            if struct_ref.has_children() {
                Ok(type_wrapper(&format!("dyn {}Trait", rust_name)))
            } else {
                // TODO: Optimize further the boxing check for structs. For example structs that 
                // have structure fields weith only simple types do not need boxing.
                if self.needs_boxing(struct_ref.borrow()) {
                    Ok(type_wrapper(&rust_name))
                } else {
                    Ok(rust_name)
                }
            }
        } else if let Some(_) = self.vim_model.enums.get(ref_name) {
            Ok(rust_name)
        } else {
            Err(Error::TypeNotFound(ref_name.to_string()))
        }
    }

    /// Check if a struct type needs boxing. If all struct fields are primitive then we do not need
    /// to box the struct. If any field is a struct type then we need to box the struct. Array
    /// fields of struct types are also to be boxed.
    fn needs_boxing(&self, struct_ref: &Struct) -> bool {
        for (_, field) in &struct_ref.properties {
            if self.is_struct_type(&field.vim_type) {
                return true;
            }            
            match &field.vim_type {
                VimType::Array(nested_type) => {
                    if self.is_struct_type(nested_type) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    fn is_struct_type(&self, vim_type: &VimType) -> bool {
        match vim_type {
            VimType::Reference(ref_name) => {
                if let Some(struct_type) = self.vim_model.structs.get(ref_name) {
                    return struct_type.borrow().name != "Any";
                }
            }
            _ => {}
        }
        false
    }
    
}

// Add a Box<> to the type declaration
pub fn box_type_declaration(type_decl: &str) -> String {
    format!("Box<{}>", type_decl)
}

// Add reference to the type declaration
pub fn ref_type_declaration(type_decl: &str) -> String {
    format!("&{}", type_decl)
}