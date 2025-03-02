use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashSet;

use super::common::emit_description;
use super::errors::{Error, Result};
use super::TypeDefResolver;
use super::{to_fn_name, to_type_name};
use crate::printer::Printer;
use crate::vim_model::DataType;
use crate::vim_model::HttpMethod;
use crate::vim_model::ManagedObject;
use crate::vim_model::Method;
use crate::vim_model::Model;
use crate::vim_model::Struct;

pub struct ManagedObjectEmitter<'a> {
    mo: &'a ManagedObject,
    vim_model: &'a Model,
    printer: &'a mut dyn Printer,
    tdf: TypeDefResolver<'a>,
}

impl<'a> ManagedObjectEmitter<'a> {
    pub fn new(
        mo: &'a ManagedObject,
        printer: &'a mut dyn Printer,
        vim_model: &'a Model,
    ) -> ManagedObjectEmitter<'a> {
        ManagedObjectEmitter {
            mo,
            vim_model,
            printer,
            tdf: TypeDefResolver::new_with_root_package(vim_model, "crate::types".to_string()),
        }
    }

    pub fn emit(&mut self) -> Result<()> {
        self.emit_imports()?;
        self.emit_mo_struct()?;
        self.emit_impl()?;
        self.emit_request_types()?;
        // self.emit_footer()?;
        Ok(())
    }

    fn emit_imports(&mut self) -> Result<()> {
        self.printer.println("use std::sync::Arc;")?;
        self.printer
            .println("use crate::core::client::{Client, Result};")?;
        let imported_types = self.get_imported_types()?;
        for type_name in &imported_types {
            self.printer
                .println(&format!("use crate::types::{type_name};"))?;
        }
        Ok(())
    }

    /// Check for structs referenced to by method return types and members of the *RequestType type.
    fn get_imported_types(&self) -> Result<Vec<String>> {
        let mut types: HashSet<String> = HashSet::new();
        for method in self.mo.methods.iter() {
            if let Some(output) = &method.output {
                self.accumulate_type_reference(output, &mut types)?;
            }
            let Some(request_type) = get_request_type(method, self.vim_model)? else {
                continue;
            };
            for (_, field) in &request_type.borrow().fields {
                self.accumulate_type_reference(&field.vim_type, &mut types)?;
            }
        }
        let mut types_vec = Vec::from_iter(types);
        types_vec.sort();
        Ok(types_vec)
    }

    fn accumulate_type_reference(
        &self,
        output: &DataType,
        types: &mut HashSet<String>,
    ) -> Result<()> {
        match output {
            DataType::Reference(ref_name) => {
                if let Some(type_name) = self.resolve_import_type(ref_name)? {
                    types.insert(type_name);
                }
            }
            DataType::Array(arr_type) => {
                if let DataType::Reference(ref_name) = arr_type.as_ref() {
                    if let Some(type_name) = self.resolve_import_type(ref_name)? {
                        types.insert(type_name);
                    }
                }
            }
            _ => {}
        };
        Ok(())
    }

    fn resolve_import_type(&self, ref_name: &str) -> Result<Option<String>> {
        if ref_name == "Any" {
            return Ok(Some("vim_any::VimAny".to_string()));
        }
        let rust_name = to_type_name(ref_name);
        if let Some(struct_ref) = self.vim_model.structs.get(ref_name) {
            if struct_ref.borrow().has_children() {
                Ok(None)
            } else {
                Ok(Some(format!("structs::{rust_name}")))
            }
        } else if self.vim_model.enums.contains_key(ref_name) {
            Ok(None)
        } else {
            Err(Error::TypeNotFound(ref_name.to_string()))
        }
    }

    fn emit_mo_struct(&mut self) -> Result<()> {
        emit_description(self.printer, &self.mo.description)?;
        let struct_name = to_type_name(&self.mo.name);
        self.printer
            .println(&format!("pub struct {} {{", struct_name))?;
        self.printer.indent();
        self.printer.println("client: Arc<Client>,")?;
        self.printer.println("mo_id: String,")?;
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }

    fn emit_impl(&mut self) -> Result<()> {
        let struct_name = to_type_name(&self.mo.name);
        self.printer.println(&format!("impl {} {{", struct_name))?;
        self.printer.indent();
        self.emit_new()?;
        self.emit_methods()?;
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }

    fn emit_new(&mut self) -> Result<()> {
        self.printer
            .println("pub fn new(client: Arc<Client>, mo_id: &str) -> Self {")?;
        self.printer.indent();
        self.printer.println("Self {")?;
        self.printer.indent();
        self.printer.println("client,")?;
        self.printer.println("mo_id: mo_id.to_string(),")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }

    fn emit_methods(&mut self) -> Result<()> {
        for method in self.mo.methods.iter() {
            self.emit_method(method)?;
        }
        Ok(())
    }

    fn emit_method(&mut self, method: &Method) -> Result<()> {
        self.emit_docstring(method)?;

        let request_type = get_request_type(method, self.vim_model)?;
        let method_name = to_fn_name(&method.name);
        self.printer.print_indent()?;
        self.printer
            .print(&format!("pub async fn {}(&self", method_name))?;

        if let Some(request_type) = request_type {
            for (_, field) in &request_type.borrow().fields {
                self.printer.print(&format!(
                    ", {}: {}",
                    field.rust_name(),
                    self.tdf.to_rust_param_type(field, None)?
                ))?;
            }
        }
        match &method.output {
            Some(output) => {
                let res_type = self.tdf.to_rust_field_type(output)?;
                if method.optional_response {
                    self.printer
                        .print(&format!(") -> Result<Option<{}>> {{", res_type))?;
                } else {
                    self.printer
                        .print(&format!(") -> Result<{}> {{", res_type))?;
                }
            }
            None => {
                self.printer.print(") -> Result<()> {")?;
            }
        }
        self.printer.indent();
        self.printer.newline()?;
        if let Some(request_type) = request_type {
            self.printer.print_indent()?;
            self.printer.print(&format!(
                "let input = {} {{",
                request_type.borrow().rust_name()
            ))?;
            for (_, field) in &request_type.borrow().fields {
                self.printer.print(&format!("{}, ", field.rust_name()))?;
            }
            self.printer.print("};")?;
            self.printer.newline()?;
        }

        self.printer.println(&format!(
            r#"let path = format!("{}", moId = &self.mo_id);"#,
            method.path
        ))?;

        match method.http_method {
            HttpMethod::Get => {
                self.printer
                    .println("let req = self.client.get_request(&path);")?;
            }
            HttpMethod::Post => {
                if request_type.is_some() {
                    self.printer
                        .println("let req = self.client.post_request(&path, &input);")?;
                } else {
                    self.printer
                        .println("let req = self.client.post_bare(&path);")?;
                }
            }
        }

        match &method.output {
            Some(_) => {
                if method.optional_response {
                    self.printer
                        .println("Ok(self.client.execute_option(req).await?)")?;
                } else {
                    self.printer
                        .println("Ok(self.client.execute(req).await?)")?;
                }
            }
            None => {
                self.printer
                    .println("Ok(self.client.execute_void(req).await?)")?;
            }
        }
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }

    fn emit_docstring(&mut self, method: &Method) -> Result<()> {
        emit_description(self.printer, &method.description)?;
        self.emit_param_docs(method)?;
        self.emit_returns_doc(method)?;
        self.emit_error_documentation(method)?;
        Ok(())
    }

    fn emit_error_documentation(&mut self, method: &Method) -> Result<()> {
        if let Some(error_description) = &method.error_description {
            if error_description.len() > 5 {
                self.printer.println("///")?;
                self.printer.println("/// ## Errors:")?;
                self.printer.println("///")?;
                for line in error_description.trim().split('\n') {
                    self.printer.println(&format!("/// {}", line))?;
                }
            }
        }
        Ok(())
    }

    fn emit_returns_doc(&mut self, method: &Method) -> Result<()> {
        if let Some(output_description) = &method.output_description {
            // Some return type descrptions read "OK" and are not helpful.
            if method.output.is_some() && output_description.len() > 5 {
                self.printer.println("///")?;
                self.printer.println("/// ## Returns:")?;
                self.printer.println("///")?;
                for line in output_description.trim().split('\n') {
                    self.printer.println(&format!("/// {}", line))?;
                }
            }
        }
        Ok(())
    }

    fn emit_param_docs(&mut self, method: &Method) -> Result<()> {
        let request_type = get_request_type(method, self.vim_model)?;
        let Some(request_type) = request_type else {
            return Ok(());
        };
        self.printer.println("///")?;
        self.printer.println("/// ## Parameters:")?;
        for (_, field) in &request_type.borrow().fields {
            let field_name = field.rust_name();
            self.printer.println("///")?;
            self.printer.println(&format!("/// ### {field_name}"))?;
            match &field.description {
                Some(desc) => {
                    for line in desc.trim().split('\n') {
                        self.printer.println(&format!("/// {}", line))?;
                    }
                }
                None => {
                    self.printer.println("/// -")?;
                }
            }
        }

        Ok(())
    }

    fn emit_request_types(&mut self) -> Result<()> {
        for method in self.mo.methods.iter() {
            let request_type = get_request_type(method, self.vim_model)?;
            let Some(request_type) = request_type else {
                continue;
            };
            self.emit_request_type(request_type)?;
        }
        Ok(())
    }

    fn emit_request_type(&mut self, request_type: &RefCell<Struct>) -> Result<()> {
        self.printer.println("#[derive(serde::Serialize)]")?;
        let struct_ref = request_type.borrow();
        let struct_name = struct_ref.rust_name();
        let discriminator = struct_ref
            .discriminator_value
            .clone()
            .unwrap_or(struct_ref.name.to_string());
        if struct_name == discriminator {
            self.printer.println(r#"#[serde(tag="_typeName")]"#)?;
        } else {
            self.printer.println(&format!(
                r#"#[serde(rename = "{discriminator}", tag = "_typeName")]"#
            ))?;
        }
        if self.needs_lifetime(request_type.borrow().borrow()) {
            self.printer
                .println(&format!("struct {}<'a> {{", struct_name))?;
        } else {
            self.printer
                .println(&format!("struct {} {{", struct_name))?;
        }
        self.printer.indent();
        for (_, field) in &request_type.borrow().fields {
            let field_name = field.rust_name();
            if field.optional {
                self.printer
                    .println("#[serde(default, skip_serializing_if = \"Option::is_none\")]")?;
            }
            if field_name != field.name {
                self.printer
                    .println(&format!(r#"#[serde(rename = "{}")]"#, field.name))?;
            }
            self.printer.println(&format!(
                "{}: {},",
                field_name,
                self.tdf.to_rust_param_type(field, Some("a".to_string()))?
            ))?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }

    fn needs_lifetime(&self, struct_ref: &Struct) -> bool {
        for (_, field) in &struct_ref.fields {
            match &field.vim_type {
                DataType::String => return true,
                DataType::DateTime => return true,
                DataType::Array(_) => return true,
                DataType::Reference(ref_name) => {
                    if self.vim_model.structs.get(ref_name).is_some() {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
}

fn get_request_type<'a>(
    method: &Method,
    vim_model: &'a Model,
) -> Result<Option<&'a RefCell<Struct>>> {
    // Input type is a synthetic struct referece or none. We do not have array input type.
    let Some(DataType::Reference(input)) = &method.input else {
        return Ok(None);
    };
    let request_type = vim_model.request_types.get(input);
    let Some(request_type) = request_type else {
        return Err(Error::TypeNotFound(input.to_string()));
    };
    Ok(Some(request_type))
}
