// Generate phf to look up field metadata for the vim_updatable macro
// The generated map is 2 level deep, first level is the class name, second level is the field name
// Class name will be managed object name of inventory data types or vim struct name types
// Field name is the field name in the class.
//
// Example:
// ```
// static CLASS_FIELDS: phf::Map<&'static str, phf::Map<&'static str, NodeData>> = phf_map! {
//     "VirtualMachine" => phf_map! {
//         "name" => NodeData {
//             type_decl: "String",
//             type_name: "",
//             is_optional: false,
//             processing_type: FieldProcessingType::Enum("PrimitiveString"),
//             path_segment: "name",
//         },
//         "runtime" => NodeData {
//             type_decl: "vim_rs::types::structs::VirtualMachineRuntimeInfo",
//             type_name: "VirtualMachineRuntimeInfo",
//             is_optional: false,
//             processing_type: FieldProcessingType::Struct,
//             path_segment: "runtime",
//         },
//         "summary" => NodeData {
//             type_decl: "vim_rs::types::structs::VirtualMachineSummary",
//             type_name: "VirtualMachineSummary",
//             is_optional: false,
//             processing_type: FieldProcessingType::Struct,
//             path_segment: "summary",
//         },
//         "config" => NodeData {
//             type_decl: "vim_rs::types::structs::VirtualMachineConfigInfo",
//             type_name: "VirtualMachineConfigInfo",
//             is_optional: true,
//             processing_type: FieldProcessingType::Struct,
//             path_segment: "config",
//         },
//     },
// };
// ```

use std::collections::HashSet;
use indexmap::{IndexMap, IndexSet};
use crate::printer::Printer;
use crate::rs_emitter::Error::InternalError;
use crate::rs_emitter::{to_field_name, TypeDefResolver};
use crate::vim_model::{DataType, EmitMode, HttpMethod, Model};
use super::errors::Result;

static INVENTORY_TYPES: &[&str] = &[
    "Folder",
    "VirtualMachine",
    "HostSystem",
    "Datacenter",
    "ClusterComputeResource",
    "ResourcePool",
    "Datastore",
    "Network",
    "DistributedVirtualSwitch",
    "DistributedVirtualPortgroup",
    "VmwareDistributedVirtualSwitch",
    "StoragePod",
    "ComputeResource",
    "VirtualApp",
    // Add other useful types here
    "Task",
];

#[derive(Clone, Debug)]
struct FieldData {
    data_type: DataType,
    is_optional: bool,
    description: Option<String>,
}

pub struct FieldDataEmitter<'a> {
    vim_model: &'a Model,
    printer: &'a mut dyn Printer,
    tdf: TypeDefResolver<'a>,
}

impl<'a> FieldDataEmitter<'a> {
    pub fn new(vim_model: &'a Model, printer: &'a mut dyn Printer) -> Self {
        FieldDataEmitter {
            vim_model,
            printer,
            tdf: TypeDefResolver::new_with_root_package(vim_model, "vim_rs::types".to_string()),
        }
    }
    pub fn emit_field_data(&mut self) -> Result<()> {
        let mut field_data: IndexMap<String, IndexMap<String, FieldData>> = IndexMap::new();

        // Extract managed properties from the managed objects
        self.extract_managed_props(&mut field_data)?;

        // Extract field data from structs forming transitive closure of data types
        self.add_referenced_structs(&mut field_data)?;
        
        self.emit_imports()?;
        
        self.emit_field_maps(&field_data)?;
        
        self.emit_resolve_function()?;
        
        Ok(())
    }
    
    fn emit_imports(&mut self) -> Result<()> {
        self.printer.println("use std::string::ToString;")?;
        self.printer.println("use crate::resolver::{Result, FieldProcessingType, HierarchyError, NodeData};")?;
        self.printer.newline()?;
        Ok(())
    }
    
    fn emit_resolve_function(&mut self) -> Result<()> {
        self.printer.println("pub(crate) fn lookup_field_data(class: &str, field: &str) -> Result<&'static NodeData> {
    CLASS_FIELDS
        .get(class)
        .and_then(|fields| fields.get(field))
        .ok_or_else(|| {
            if CLASS_FIELDS.contains_key(class) {
                HierarchyError::UnknownField(class.to_string(), field.to_string())
            } else {
                HierarchyError::UnsupportedObjectType(class.to_string())
            }
        })
}
#[allow(dead_code)]
pub(crate) fn get_type_fields(class: &str) -> Result<&'static phf::Map<&'static str, NodeData>> {
    CLASS_FIELDS
        .get(class)
        .ok_or_else(|| HierarchyError::UnsupportedObjectType(class.to_string()))
}")?;
        Ok(())
    }
    
    fn emit_field_maps(&mut self, field_data: &IndexMap<String, IndexMap<String, FieldData>>) -> Result<()> {
        // Emit static map using phf_codegen
        let mut structs = phf_codegen::Map::new();
        for (class_name, fields) in field_data.iter() {
            let mut field_details = phf_codegen::Map::new();
            for (field_name, field) in fields {
                let field_decl = self.tdf.to_rust_field_type(&field.data_type)?;
                let is_optional = if field.is_optional { "true" } else { "false" };
                let type_name = match &field.data_type {
                    DataType::Reference(type_name) if self.vim_model.structs.contains_key(type_name) => {
                        format!("{}", type_name)
                    },
                    _ => "".to_string()
                };
                let processing_type = match &field.data_type {
                    DataType::Reference(type_name) if self.vim_model.structs.contains_key(type_name) => {
                        let Some(struct_ref) = self.vim_model.structs.get(type_name) else {
                            return Err(InternalError("Struct lookup failed!".to_string()));
                        };
                        let struct_ref = struct_ref.borrow();
                        if struct_ref.has_children()  && (struct_ref.emit_mode == EmitMode::Emit) {
                            "Trait".to_string()
                        } else {
                            "Struct".to_string()
                        }
                    },
                    _ => self.lookup_enum_processing_type(&field.data_type)?,
                };
                field_details.entry(to_field_name(field_name), &format!(
                    "NodeData {{ type_decl: \"{}\", type_name: \"{}\", is_optional: {}, processing_type: FieldProcessingType::{}, path_segment: \"{}\", doc: {} }},",
                    field_decl, type_name, is_optional, processing_type, field_name, esc(&field.description)));
            }
            structs.entry(class_name, &format!("{}", field_details.build()));
        }
        self.printer.println(&format!("static CLASS_FIELDS: phf::Map<&'static str, phf::Map<&'static str, NodeData>> = {};", structs.build()))?;
        Ok(())
    }
    
    fn lookup_enum_processing_type(&self, data_type: &DataType) -> Result<String> {
        for (typename, def) in &self.vim_model.any_value_types {
            if def.property_type == *data_type {
                return Ok(format!("Enum(\"{}\")", typename));
            }
        }
        Err(InternalError(format!("Enum type not found for: '{:?}'!", data_type)))
    }

    fn extract_managed_props(&mut self, field_data: &mut IndexMap<String, IndexMap<String, FieldData>>) -> Result<()> {
        for mo in INVENTORY_TYPES {
            let Some(mo_def) = self.vim_model.managed_objects.get(*mo) else {
                return Err(super::errors::Error::TypeNotFound(mo.to_string()));
            };
            let mut fields: IndexMap<String, FieldData> = IndexMap::new();
            for method in &mo_def.methods {
                if method.http_method != HttpMethod::Get {
                    continue;
                }
                let property_name = last_segment(&method.path)?;
                let Some(data_type) = method.output.clone() else {
                    return Err(InternalError(format!("Cannot find output type for method: {}::{}", mo, method.name)));
                };
                let field_data = FieldData {
                    data_type: data_type.clone(),
                    is_optional: method.optional_response,
                    description: method.description.clone(),
                };
                fields.insert(property_name.clone(), field_data);
            }
            if fields.is_empty() {
                continue;
            }
            field_data.insert(mo.to_string(), fields);
        }
        Ok(())
    }

    fn add_referenced_structs(&self, field_data: &mut IndexMap<String, IndexMap<String, FieldData>>) -> Result<()> {
        let mut pending = IndexSet::new();
        let mut processed = HashSet::new();

        // Initial set from field_data - using the FieldData.data_type
        for (_, fields) in field_data.iter() {
            for (_, field) in fields.iter() {
                self.extract_struct_references(&field.data_type, &mut pending)?;
            }
        }

        // Process until no more new structs are found
        while let Some(current) = pending.pop() {
            // Skip if already processed
            if !processed.insert(current.clone()) {
                continue;
            }

            // Get all fields of this struct
            if self.vim_model.structs.contains_key(&current) {
                // If this struct is not yet in field_data, add it
                if !field_data.contains_key(&current) {

                    // Get inheritance chain
                    let inheritance_chain = self.vim_model.inheritance_chain(&current.to_string())?;
                    
                    // Create container for struct field data
                    let mut fields = IndexMap::new();

                    
                    // Add fields from inheritance chain (current type + parents)
                    for struct_cell in inheritance_chain {
                        let struct_def = struct_cell.borrow();
                        // Add own fields
                        for (field_name, field) in &struct_def.fields {
                            fields.insert(field_name.clone(), FieldData {
                                data_type: field.vim_type.clone(),
                                is_optional: field.optional,
                                description: field.description.clone(),
                            });

                            // Check if field references another struct
                            self.extract_struct_references(&field.vim_type, &mut pending)?;
                        }
                    }

                    field_data.insert(current.clone(), fields);
                }

            }
        }

        Ok(())
    }

    fn extract_struct_references(
        &self,
        data_type: &DataType,
        pending: &mut IndexSet<String>
    ) -> Result<()> {
        const MOREF: &str = "ManagedObjectReference";
        match data_type {
            DataType::Reference(type_name) => {
                // Add to pending if it's a struct (not an enum)
                if self.vim_model.structs.contains_key(type_name) && type_name != MOREF {
                    pending.insert(type_name.clone());
                }
            },
            // Not recursively resolving array types as requested
            _ => {}
        }
        Ok(())
    }
}

fn last_segment(path: &str) -> Result<String> {
    path.split('/').last().map(String::from)
        .ok_or(InternalError(format!("Cannot extract property name from path: {}", path)))
}

// Convert Option<String> to properly escaped string that can be assigned to an 
// Option<&str> in generated code.
fn esc(val: &Option<String>) -> String {
    match val {
        Some(v) => format!("Some(r#####\"{}\"#####)", v),
        None => "None".to_string(),
    }
}

