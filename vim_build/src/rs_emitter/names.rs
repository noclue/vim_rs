use std::borrow::Borrow;

// Codify the rust convention on names from https://github.com/rust-lang/rust/blob/0bb6fec1c9aa484a7ec987a9e8ffca2eb647b0b3/src/doc/style-guide/src/advice.md
// Also apply safe rust naming conventions
// We use:
// - check_keyword to check if the name is a keyword and make it safe
// - convert_case to convert the name as needed

use check_keyword::CheckKeyword;
use convert_case::{Case, Casing};

use crate::rs_emitter::errors::{Error, Result};
use crate::vim_model::{DataType, Field, Model, Struct};

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

pub fn to_module_name(name: &str) -> String {
    name.to_case(Case::Snake).into_safe()
}

/// Convert a struct reference to a Rust type declaration. This function type allows for
/// customizing the reference type declaration. We havve case for field and parameter declarations.
type StructRefRenderer = Box<dyn Fn(&Struct, &Model) -> String>;

pub struct TypeDefResolver<'a> {
    vim_model: &'a Model,
    root_package: String,
}

impl TypeDefResolver<'_> {
    pub fn new(vim_model: &Model) -> TypeDefResolver {
        TypeDefResolver {
            vim_model,
            root_package: "super".to_string(),
        }
    }

    pub fn new_with_root_package(vim_model: &Model, root_package: String) -> TypeDefResolver {
        TypeDefResolver {
            vim_model,
            root_package,
        }
    }

    pub fn field_type(&self, field: &Field) -> Result<String> {
        let mut field_type = self.to_rust_field_type(&field.vim_type)?;
        if field.require_box {
            field_type = box_type_declaration(&field_type);
        }
        if field.optional {
            field_type = format!("Option<{field_type}>");
        }
        Ok(field_type)
    }

    /// Convert a VimType to a Rust field type declaration. Structure types are always boxed. To
    /// use borrow semantics instead of boxing use `to_rust_param_type`
    pub fn to_rust_field_type(&self, vim_type: &DataType) -> Result<String> {
        self.to_rust_type_with_wrapper(vim_type, field_reference(self.root_package.clone()))
    }

    /// Convert a VimType to a Rust param type declaration. Structs and strings are borrowed.
    /// Arrays are borrowed slices.
    pub fn to_rust_param_type(&self, field: &Field, lifecycle: Option<String>) -> Result<String> {
        let mut decl = match &field.vim_type {
            DataType::String => ref_type_declaration("str", lifecycle.clone()),
            DataType::DateTime => ref_type_declaration("str", lifecycle.clone()),
            DataType::Array(nested_type) => ref_type_declaration(
                &format!("[{}]", self.to_rust_field_type(nested_type)?),
                lifecycle.clone(),
            ),
            _ => self.to_rust_type_with_wrapper(
                &field.vim_type,
                param_reference(lifecycle.clone(), self.root_package.clone()),
            )?,
        };
        if field.optional {
            decl = format!("Option<{}>", decl);
        }
        Ok(decl)
    }

    /// Convert a VimType to a Rust type declaration. Allowing to customize the wrapper of the top
    /// level reference types. By using `box_type_declaration` the top level structs are boxed. In
    /// case of method calls we want to use borrow semantics instead to avoid boxing. This change of
    /// Box to borrow is done by `ref_type_declaration`. The `type_wrapper` function is used only on
    /// the top level structure types.
    fn to_rust_type_with_wrapper(
        &self,
        vim_type: &DataType,
        type_wrapper: StructRefRenderer,
    ) -> Result<String> {
        match &vim_type {
            DataType::Boolean => Ok("bool".to_string()),
            DataType::String => Ok("String".to_string()),
            DataType::Binary => Ok("Vec<u8>".to_string()),
            DataType::Int8 => Ok("i8".to_string()),
            DataType::Int16 => Ok("i16".to_string()),
            DataType::Int32 => Ok("i32".to_string()),
            DataType::Int64 => Ok("i64".to_string()),
            DataType::Float => Ok("f32".to_string()),
            DataType::Double => Ok("f64".to_string()),
            DataType::DateTime => Ok("String".to_string()),
            DataType::Array(nested_type) => {
                Ok(format!("Vec<{}>", self.to_rust_field_type(nested_type)?))
            }
            DataType::Reference(ref_name) => {
                Ok(self.get_ref_type_declaration(ref_name, type_wrapper)?)
            }
        }
    }

    /// Generate the type declaration for a reference field - enum or struct.
    /// If VIM::Any then return VimAny.
    /// If enum return just the enum Pascal case name.
    /// If structure then return a Box<> or & reference to the structure type.
    /// If the structure has children then we need dynamic trait reference.
    /// If the Struct has no children then we reference the Struct type.
    /// If we cannot match the name to a struct or enum this is programatic error
    fn get_ref_type_declaration(
        &self,
        ref_name: &str,
        render_struct_ref: StructRefRenderer,
    ) -> Result<String> {
        if ref_name == "Any" {
            return Ok("VimAny".to_string());
        }
        if let Some(struct_type) = self.vim_model.structs.get(ref_name) {
            let struct_ref = struct_type.borrow();
            let struct_ref: &Struct = struct_ref.borrow();
            Ok(render_struct_ref(struct_ref, self.vim_model))
        } else if self.vim_model.enums.contains_key(ref_name) {
            Ok(format!(
                "{}::enums::{}",
                self.root_package,
                to_type_name(ref_name)
            ))
        } else {
            Err(Error::TypeNotFound(ref_name.to_string()))
        }
    }
}

fn field_reference(root_package: String) -> StructRefRenderer {
    Box::new(move |struct_ref: &Struct, _: &Model| -> String {
        let type_name = to_type_name(&struct_ref.name);
        if struct_ref.has_children() {
            box_type_declaration(&format!("dyn {root_package}::traits::{type_name}Trait"))
        } else {
            type_name
        }
    })
}

fn param_reference(lifecycle: Option<String>, root_package: String) -> StructRefRenderer {
    Box::new(move |struct_ref: &Struct, _: &Model| -> String {
        let rust_name = to_type_name(&struct_ref.name);
        if struct_ref.has_children() {
            ref_type_declaration(
                &format!("dyn {root_package}::traits::{rust_name}Trait"),
                lifecycle.clone(),
            )
        } else {
            ref_type_declaration(&rust_name, lifecycle.clone())
        }
    })
}

// Add a Box<> to the type declaration
fn box_type_declaration(type_decl: &str) -> String {
    format!("Box<{}>", type_decl)
}

// Add reference to the type declaration
fn ref_type_declaration(type_decl: &str, lifecycle: Option<String>) -> String {
    match lifecycle {
        Some(lc) => format!("&'{} {}", lc, type_decl),
        None => format!("&{}", type_decl),
    }
}
