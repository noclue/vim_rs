use std::borrow::Borrow;
use std::cell::RefCell;

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
            .println("use crate::core::client::{VimClient, Result};")?;
        Ok(())
    }

    fn emit_mo_struct(&mut self) -> Result<()> {
        emit_description(self.printer, &self.mo.description)?;
        let struct_name = to_type_name(&self.mo.name);
        self.printer.println("#[derive(Clone)]")?;
        self.printer
            .println(&format!("pub struct {} {{", struct_name))?;
        self.printer.indent();
        self.printer.println("client: Arc<dyn VimClient>,")?;
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
            .println("pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {")?;
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


        // TODO We should use &self.mo.name and &method.name to obtain the mo type and method/property name.
        // Problem is for properties method.name is capitalized because of the "getProperty" convention and so it is incorrect.
        // Thus we resort to path parsing

        // Paths have the form "/{moType}/{moId}/{MethodOrProp}"for vim 
        // or "/{Service}/{moType}/{moId}/{MethodOrProp}" for other services -
        // eam, pbm, sms, vslm, vsan.
        let path_parts: Vec<&str> = method.path.trim_start_matches('/').split('/').collect();
        let (service_name, mo_type_name, method_or_prop) = match path_parts.as_slice() {
            // "/{MoType}/{moId}/MethodOrProp" (vim): no service prefix
            [mo_type_name, "{moId}", method_or_prop] => ("", *mo_type_name, *method_or_prop),
            // "/{Service}/{MoType}/{moId}/MethodOrProp" (eam/pbm/sms/vslm/vsan)
            [service, mo_type_name, "{moId}", method_or_prop] => (*service, *mo_type_name, *method_or_prop),
            _ => ("", "", ""),
        };

        match method.http_method {
            HttpMethod::Get => {
                // Property access via fetch_property_raw + extract_property (JSON bytes or SOAP VimAny)
                match &method.output {
                    Some(_) => {
                        let res_type = self.tdf.to_rust_field_type(method.output.as_ref().unwrap())?;
                        self.printer.println(&format!(
                            "let pv_opt = self.client.fetch_property_raw(\"{service_name}\", \"{mo_type_name}\", &self.mo_id, \"{method_or_prop}\").await?;"
                        ))?;
                        if method.optional_response {
                            self.printer.println("match pv_opt {")?;
                            self.printer.indent();
                            self.printer.println(
                                "Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),",
                            )?;
                            self.printer.println("None => Ok(None),")?;
                            self.printer.dedent();
                            self.printer.println("}")?;
                        } else {
                            self.printer.println(&format!(
                                "let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError(\"property {method_or_prop} was empty\".to_string()))?;"
                            ))?;
                            self.printer.println(&format!(
                                "let result: {res_type} = crate::core::client::extract_property(pv)?;"
                            ))?;
                            self.printer.println("Ok(result)")?;
                        }
                    }
                    None => {
                        // Void GET is unusual but handle gracefully
                        self.printer.println(&format!(
                            "let _ = self.client.fetch_property_raw(\"\", \"{mo_type_name}\", &self.mo_id, \"{method_or_prop}\").await?;"
                        ))?;
                        self.printer.println("Ok(())")?;
                    }
                }
            }
            HttpMethod::Post => {
                let params_arg = if request_type.is_some() {
                    "Some(&input)"
                } else {
                    "None"
                };
                match &method.output {
                    Some(_) => {
                        let res_type = self
                            .tdf
                            .to_rust_field_type(method.output.as_ref().unwrap())?;
                        let output_dt = method.output.as_ref().unwrap();
                        if method.optional_response {
                            self.printer.println(&format!(
                                "let bytes_opt = self.client.invoke_optional(\"{service_name}\", \"{mo_type_name}\", &self.mo_id, \"{method_or_prop}\", {params_arg}).await?;"
                            ))?;
                            self.printer.println("match bytes_opt {")?;
                            self.printer.indent();
                            let decode_b =
                                mo_soap_response_unmarshal_expr(output_dt, "b");
                            self.printer.println(&format!(
                                "Some(ref b) => Ok(Some({decode_b}?)),"
                            ))?;
                            self.printer.println("None => Ok(None),")?;
                            self.printer.dedent();
                            self.printer.println("}")?;
                        } else {
                            self.printer.println(&format!(
                                "let bytes = self.client.invoke(\"{service_name}\", \"{mo_type_name}\", &self.mo_id, \"{method_or_prop}\", {params_arg}).await?;"
                            ))?;
                            let decode_bytes =
                                mo_soap_response_unmarshal_expr(output_dt, "&bytes");
                            self.printer.println(&format!(
                                "let result: {res_type} = {decode_bytes}?;"
                            ))?;
                            self.printer.println("Ok(result)")?;
                        }
                    }
                    None => {
                        self.printer.println(&format!(
                            "self.client.invoke_void(\"{service_name}\", \"{mo_type_name}\", &self.mo_id, \"{method_or_prop}\", {params_arg}).await"
                        ))?;
                    }
                }
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
            // Some return type descriptions read "OK" and are not helpful.
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
        let struct_ref = request_type.borrow();
        let struct_name = struct_ref.rust_name();
        let discriminator = struct_ref
            .discriminator_value
            .clone()
            .unwrap_or(struct_ref.name.to_string());
        let has_lifetime = self.needs_lifetime(request_type.borrow().borrow());
        let lt = if has_lifetime { "<'a>" } else { "" };
        let ser_name = format!("{struct_name}Ser");

        // Emit struct definition
        self.printer
            .println(&format!("struct {struct_name}{lt} {{"))?;
        self.printer.indent();
        for (_, field) in &struct_ref.fields {
            let field_name = field.rust_name();
            self.printer.println(&format!(
                "{}: {},",
                field_name,
                self.tdf.to_rust_param_type(field, Some("a".to_string()))?
            ))?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // Collect fields
        let fields: Vec<_> = struct_ref
            .fields
            .iter()
            .map(|(_, f)| (f.rust_name(), f.name.clone(), f.optional))
            .collect();
        let has_optional = fields.iter().any(|(_, _, opt)| *opt);

        // Emit miniserde::Serialize impl
        self.printer.println(&format!(
            "impl{lt} miniserde::Serialize for {struct_name}{lt} {{"
        ))?;
        self.printer.indent();
        self.printer
            .println("fn begin(&self) -> miniserde::ser::Fragment<'_> {")?;
        self.printer.indent();
        self.printer.println(&format!(
            "miniserde::ser::Fragment::Map(Box::new({ser_name} {{ data: self, seq: 0 }}))"
        ))?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // Serializer struct
        let ser_lt = if has_lifetime { "<'b, 'a>" } else { "<'b>" };
        self.printer
            .println(&format!("struct {ser_name}{ser_lt} {{"))?;
        self.printer.indent();
        self.printer
            .println(&format!("data: &'b {struct_name}{lt},"))?;
        self.printer.println("seq: usize,")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.newline()?;

        // Map impl
        let (map_impl_generics, map_impl_lt) = if has_lifetime {
            ("<'b, 'a>", "<'b, 'a>")
        } else {
            ("<'b>", "<'b>")
        };
        self.printer.println(&format!(
            "impl{map_impl_generics} miniserde::ser::Map for {ser_name}{map_impl_lt} {{"
        ))?;
        self.printer.indent();
        self.printer.println(
            "fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {"
        )?;
        self.printer.indent();

        if has_optional {
            self.printer.println("loop {")?;
            self.printer.indent();
        }

        self.printer.println("let seq = self.seq;")?;
        self.printer.println("self.seq += 1;")?;
        self.printer.println("match seq {")?;
        self.printer.indent();

        // seq 0: _typeName
        self.printer.println(&format!(
            "0 => return Some((std::borrow::Cow::Borrowed(\"_typeName\"), &\"{discriminator}\")),"
        ))?;

        // Remaining fields
        for (i, (field_name, ser_name_str, optional)) in fields.iter().enumerate() {
            let seq_num = i + 1;
            if *optional {
                self.printer.println(&format!("{seq_num} => {{"))?;
                self.printer.indent();
                self.printer.println(&format!(
                    "let Some(ref val) = self.data.{field_name} else {{ continue; }};"
                ))?;
                self.printer.println(&format!(
                    "return Some((std::borrow::Cow::Borrowed(\"{ser_name_str}\"), val as &dyn miniserde::Serialize));"
                ))?;
                self.printer.dedent();
                self.printer.println("}")?;
            } else {
                self.printer.println(&format!(
                    "{seq_num} => return Some((std::borrow::Cow::Borrowed(\"{ser_name_str}\"), &self.data.{field_name} as &dyn miniserde::Serialize)),"
                ))?;
            }
        }

        self.printer.println("_ => return None,")?;
        self.printer.dedent();
        self.printer.println("}")?;

        if has_optional {
            self.printer.dedent();
            self.printer.println("}")?;
        }

        self.printer.dedent();
        self.printer.println("}")?;
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

/// SOAP **POST** bodies that deserialize to `Vec<Event>` or `Vec<TaskInfo>` often use repeated
/// sibling `<returnval>` elements (not a single wrapper). Emit `unmarshal_array` for array return
/// types; scalars use `unmarshal`. **GET** properties use `extract_property` instead (not this helper).
fn mo_soap_response_unmarshal_expr(output: &DataType, bytes_arg: &str) -> String {
    match output {
        DataType::Array(_) => {
            return format!(
                "crate::core::client::unmarshal_array(self.client.transport(), {bytes_arg})"
            );
        }
        _ => {}
    }
    format!(
        "crate::core::client::unmarshal(self.client.transport(), {bytes_arg})"
    )
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
