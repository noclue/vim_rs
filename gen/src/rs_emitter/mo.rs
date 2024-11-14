use std::cell::RefCell;
use std::collections::HashSet;

use serde_json::error;

use crate::vim_model::ManagedObject;
use crate::vim_model::Method;
use crate::vim_model::Struct;
use crate::vim_model::VimModel;
use crate::printer::Printer;
use crate::vim_model::VimType;
use super::common::emit_doc;
use super::errors::{Result, Error};
use super::ref_type_declaration;
use super::to_field_name;
use super::to_fn_name;
use super::to_type_name;
use super::TypeDefResolver;


pub struct ManagedObjectEmitter <'a>{
    mo: &'a ManagedObject,
    vim_model: &'a VimModel,
    printer: &'a mut dyn Printer,
    tdf: TypeDefResolver<'a>,
}

impl <'a> ManagedObjectEmitter <'a> {
    pub fn new(mo: &'a ManagedObject, printer: &'a mut dyn Printer, vim_model: &'a VimModel) -> ManagedObjectEmitter<'a> {
        ManagedObjectEmitter {
            mo,
            vim_model,
            printer,
            tdf: TypeDefResolver::new(vim_model),
        }
    }

    pub fn emit(&mut self) -> Result<()> {
        self.emit_imports()?;
        self.emit_struct()?;
        self.emit_impl()?;
        // self.emit_footer()?;
        Ok(())
    }

    fn emit_imports(&mut self) -> Result<()> {
        self.printer.println("use std::sync::Arc;")?;
        self.printer.println("use url_template::substitute;")?;
        self.printer.println("use vim_client::{VimClient, Result};")?;
        let imported_types = self.get_imported_types()?;
        for type_name in &imported_types {
            self.printer.println(&format!("use types::{type_name};"))?;
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
            for (_, param) in &request_type.borrow().properties {
                self.accumulate_type_reference(&param.vim_type, &mut types)?;
            }
        }
        Ok(Vec::from_iter(types))
    }


    fn accumulate_type_reference(&self, output: &VimType, types: &mut HashSet<String>) -> Result<()> {
        match output {
            VimType::Reference(ref_name) => {
                types.insert(self.resolve_import_type(ref_name)?);
            }
            VimType::Array(arr_type) => {
                if let VimType::Reference(ref_name) = arr_type.as_ref() {
                    types.insert(self.resolve_import_type(ref_name)?);
                }
            }
            _ => {}
        };
        Ok(())
    }
    
    fn resolve_import_type(&self, ref_name: &str) -> Result<String> {
        if ref_name == "Any" {
            return Ok("VimAny".to_string());
        }
        let rust_name = to_type_name(ref_name);
        if let Some(_) = self.vim_model.structs.get(ref_name) {
            Ok(rust_name)
        } else if let Some(_) = self.vim_model.enums.get(ref_name) {
            Ok(rust_name)
        } else {
            Err(Error::TypeNotFound(ref_name.to_string()))
        }
    }

    fn emit_struct(&mut self) -> Result<()> {
        emit_doc(self.printer, &self.mo.description)?;
        let struct_name = to_type_name(&self.mo.name);
        self.printer.println(&format!("pub struct {} {{", struct_name))?;
        self.printer.indent();
        self.printer.println("client: Arc<VimClient>,")?;
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
        self.printer.println("pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {")?;
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
        emit_doc(self.printer, &method.description)?;
        self.emit_param_docs(method)?;

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
        let request_type  = get_request_type(method, self.vim_model)?;
        let method_name = to_fn_name(&method.name);
        self.printer.print_indent()?;
        self.printer.print(&format!("pub async fn {}(&self", method_name))?;

        if let Some(request_type) = request_type {
            for (param_name, param) in &request_type.borrow().properties {
                self.printer.print(&format!(", {}: {}", param_name, self.tdf.to_rust_type_with_wrapper(&param.vim_type, ref_type_declaration)?))?;
            }
        }
        match &method.output {
            Some(output) => {
                self.printer.print(&format!(") -> Result<{}> {{", self.tdf.to_rust_type(output)?))?;
            }
            None => {
                self.printer.print(") -> Result<()> {")?;
            }
            
        }
        self.printer.newline()?;

        
        // for (param_name, param) in &method.pa {
        //     self.printer.print(&format!(", {}: {}", param_name, self.resolve_param_type(param.vim_type)?))?;
        // }


        // self.printer.indent();
        // self.emit_method_args(method)?;
        // self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }

    fn emit_param_docs(&mut self, method: &Method) -> Result<()> {
        let request_type = get_request_type(method, self.vim_model)?;
        let Some(request_type) = request_type else {
            return Ok(());
        };
        self.printer.println("///")?;
        self.printer.println("/// ## Parameters:")?;
        for (param_name, param) in &request_type.borrow().properties {
            let param_name = to_field_name(param_name);
            self.printer.println("///")?;
            self.printer.println(&format!("/// ### {param_name}"))?;
            match &param.description {
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

}

fn get_request_type<'a>(method: &Method, vim_model: &'a VimModel) -> Result<Option<&'a RefCell<Struct>>> {
    // Input type is a synthetic struct referece or none. We do not have array input type.
    let Some(VimType::Reference(input)) = &method.input else {
        return Ok(None);
    };
    let request_type = vim_model.request_types.get(input);
    let Some(request_type) = request_type else {
        return Err(Error::TypeNotFound(input.to_string()));
    };
    Ok(Some(request_type))
}