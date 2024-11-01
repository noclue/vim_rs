// Generator for Rust data models from vim

use std::borrow::Borrow;

use crate::vim_model::VimModel;

use super::super::printer::Printer;

use super::names::*;
use super::super::vim_model::*;

const ANY: &str = "Any";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Type not found error: {0}")]
    TypeNotFound(String),
    #[error("Printer error: {0}")]
    PrinterError(#[from] super::super::printer::Error),
}

// Result is a type alias for handling errors.
pub type Result<T> = std::result::Result<T, Error>;

pub struct TypesEmitter<'a> {
    vim_model: &'a VimModel,
    printer: &'a mut dyn Printer,
}


impl<'a> TypesEmitter<'a> {
    pub fn new(vim_model: &'a VimModel, printer: &'a mut dyn Printer) -> Self {
        TypesEmitter { vim_model, printer }
    }

    pub fn emit_data_types(&mut self) -> Result<()> {
        self.emit_use_statements()?;
        self.emit_vim_object()?;
        self.emit_vimany()?;
        self.emit_enums()?;
        self.emit_structs()?;
        self.emit_boxed_types()?;
        self.emit_value_deserializers()?;
        self.emit_object_deserializers()?;
        self.emit_any_deserialization()?;

        Ok(())
    }
    fn emit_use_statements(&mut self) -> Result<()> {
        self.printer.println("use std::{any, fmt};")?;
        self.printer.println("use serde::de;")?;
        self.printer.println("use erased_serde::serialize_trait_object;")?;
        self.printer.newline()?;
        Ok(())
    }
    fn emit_vim_object(&mut self) -> Result<()> {
        self.printer.println(r#"/// Casts trait object to Any. This is the first step in casting between trait objects.
/// 
/// See the `AnyInto` struct for the second step.
pub trait AsAny {
    /// Cast a reference to a trait object.
    fn as_any_ref<'a>(&'a self) -> &'a dyn any::Any;

    /// Cast to a boxed reference to a trait object.
    fn as_any_box(self: Box<Self>) -> Box<dyn any::Any>;

    /// Get the underlying type identifier.
    fn type_id(&self) -> std::any::TypeId {
        self.as_any_ref().type_id()
    }
}


impl<T> AsAny for T
where
    T: Sized + 'static,
{
    fn as_any_ref<'a>(&'a self) -> &'a dyn any::Any {
        self
    }

    fn as_any_box(self: Box<Self>) -> Box<dyn any::Any> {
        self
    }
}


impl AsAny for dyn any::Any {
    fn as_any_ref<'a>(&'a self) -> &'a dyn any::Any {
        self
    }

    fn as_any_box(self: Box<Self>) -> Box<dyn any::Any> {
        self
    }
}

/// Casts one trait to another using type methods. For example:
/// ```
/// let data_object: &dyn DataObjectTrait = &VirtualDevice {
///     unit_number: Some(1),
///     controller_key: Some(2),
///     key: 3,
///     numa_node: Some(4),
/// };
/// let virtual_device: &dyn VirtualDeviceTrait = data_object.into_ref().unwrap();
/// 
/// ```
pub trait CastFrom<From: ?Sized> {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self>;
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn any::Any>>;
}


pub trait CastInto<To: ?Sized> {
    fn into_ref<'a>(self: &'a Self) -> Option<&'a To>;
    fn into_box(self: Box<Self>) -> Result<Box<To>, Box<dyn any::Any>>;
}

impl<To: CastFrom<T> + ?Sized, T: ?Sized + 'static> CastInto<To> for T {
    fn into_ref<'a>(self: &'a Self) -> Option<&'a To> {
        CastFrom::from_ref(self)
    }

    fn into_box(self: Box<Self>) -> Result<Box<To>, Box<dyn any::Any + 'static>> {
        CastFrom::from_box(self)
    }
}

/// Holder for cast function pointers. We have one of those for each struct implementing a trait.
/// Thus casting between traits is implemented. First we cast to `dyn any::Any` using the `AsAny`
/// functionality, then we look up the `AnyInto` instance and use it to cast to the target trait.
/// 
/// The functions in `AnyInto`` typically downcast the `dyn any::Any` to the specific struct type, and
/// then it is converted by the compiler to a fat pointer of the trait type.
pub struct AnyInto<To> 
    where To: ?Sized {
    pub to_ref: fn(&dyn any::Any) -> Option<&To>,
    pub to_box: fn(Box<dyn any::Any>) -> Result<Box<To>, Box<dyn any::Any>>,
}

pub trait VimObjectTrait: AsAny + std::fmt::Debug + erased_serde::Serialize {}

serialize_trait_object!(VimObjectTrait);

impl<T> VimObjectTrait for T where T: AsAny + std::fmt::Debug + erased_serde::Serialize {}"#)?;
        Ok(())
    }

    fn emit_vimany(&mut self) -> Result<()> {
        self.printer.println("#[derive(Debug, serde::Serialize)]")?;
        self.printer.println("#[serde(untagged)]")?;
        self.printer.println(r#"pub enum VimAny {
    Object(Box<dyn VimObjectTrait>),
    Value(ValueElements),
}
"#)?;
        Ok(())
    }

    fn emit_enums(&mut self) -> Result<()> {
        for (_, vim_enum) in &self.vim_model.enums {
            self.emit_doc(&vim_enum.description)?;
    
            let enum_name = to_type_name(&vim_enum.name); 
    
            self.printer.println("#[derive(Debug, serde::Deserialize, serde::Serialize)]")?;
            self.printer.println(&format!("pub enum {} {{", enum_name))?;
            self.printer.indent();
            for value in &vim_enum.variants {
                let variant = to_enum_variant(&value);
                if value != &variant {
                    self.printer.println(&format!("#[serde(rename = \"{}\")]", value))?;
                }                
                self.printer.println(&format!("{},", variant))?;
            }
            // Make enums open i.e. handle unknown values possibly from future API servers
            self.printer.println("/// This variant handles values not known at compile time.")?;
            self.printer.println("#[serde(untagged)]")?;
            self.printer.println("Other_(String),")?;
            self.printer.dedent();
            self.printer.println("}")?;
        }
        Ok(())
    }
    
    fn emit_structs(&mut self) -> Result<()> {
        for (name, vim_type_cell) in &self.vim_model.structs {
            let struct_type = vim_type_cell.borrow();
            if struct_type.name == "Any" {
                continue;
            }
            self.emit_struct_type(name, &struct_type)?;
            self.emit_trait_type(name, &struct_type)?;
            self.emit_inherited_traits(name)?;
        }
        Ok(())
    }

    /// Emit boxed value types from Vim like ArrayOfInt, ArrayOfString, Boolean etc.
    fn emit_boxed_types(&mut self) -> Result<()> {
        self.printer.println("#[derive(Debug, serde::Deserialize, serde::Serialize)]")?;
        self.printer.println("#[serde(tag = \"_typeName\", content = \"_value\")]")?;
        self.printer.println("pub enum ValueElements {")?;
        self.printer.indent();
        for (_, box_type) in &self.vim_model.any_value_types {
            self.emit_doc(&box_type.description)?;
            let name = box_type.discriminator_value.as_ref().unwrap_or(&box_type.name);
            let type_name = to_type_name(&box_type.name);
            if &type_name != name {
                self.printer.println(&format!("#[serde(rename = \"{}\")]", name))?;
            }
            let rust_type = self.to_rust_type(&box_type.property_type)?;
            self.printer.println(&format!("{type_name}({rust_type}),"))?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }

    fn emit_doc(&mut self, doc_string: &Option<String>) -> Result<()> {
        Ok(if let Some(doc) = doc_string {
            for line in doc.trim().split('\n') {
                self.printer.println(&format!("/// {}", line))?;
            }
        })
    }
    
    fn emit_struct_type(&mut self, name: &str, vim_type: &Struct) -> Result<()> {
        self.emit_doc(&vim_type.description)?;
        let struct_name = to_type_name(name);
        let discriminator = vim_type.discriminator_value.clone().unwrap_or(name.to_string()); 
        self.printer.println("#[derive(Debug, serde::Deserialize, serde::Serialize)]")?;
        if struct_name == discriminator {
            self.printer.println(r#"#[serde(tag="_typeName")]"#)?;
        } else {
            self.printer.println(&format!(r#"#[serde(rename = "{discriminator}", tag = "_typeName")]"#))?;
        }
        self.printer.println(&format!("pub struct {struct_name} {{"))?;
        self.printer.indent();
        self.emit_struct_all_fields(vim_type)?;
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }
    
    fn emit_struct_all_fields(&mut self, vim_type: &Struct) -> Result<()> {
        if let Some(parent) = vim_type.parent.as_ref() {
            if parent != "Any" { // WE do not need to emit fields for the Any type
                let parent_model_ref = self.vim_model.structs.get(parent).ok_or_else(|| Error::TypeNotFound(parent.clone()))?.borrow();
                let parent_model: &Struct = parent_model_ref.borrow();
                self.emit_struct_all_fields( parent_model)?;
            }
        }
        self.emit_struct_fields(vim_type)
    }
    fn emit_struct_fields(&mut self, vim_type: &Struct) -> Result<()> {
        if vim_type.properties.is_empty() { return Ok(()); } // skip the comment if there are no fields
        self.printer.println(&format!("// Fields of {}", vim_type.name))?;
        for (prop_name, property) in &vim_type.properties {
            self.emit_struct_field(prop_name, property)?;
        }
        Ok(())
    }
    
    fn emit_struct_field(&mut self, prop_name: &str, property: &Property) -> Result<()> {
        self.emit_doc(&property.description)?;
        let field_name = to_field_name(&prop_name);
        let mut field_type = self.to_rust_type(&property.vim_type)?;
        if property.optional {
            field_type = format!("Option<{field_type}>", field_type = field_type);
            self.printer.println(&format!("#[serde(default, skip_serializing_if = \"Option::is_none\")]"))?;
        }
        if field_name != prop_name {
            self.printer.println(&format!(r#"#[serde(rename = "{prop_name}")]"#))?;
        }
        if property.vim_type == VimType::Binary {
            if property.optional {
                self.printer.println(r#"#[serde(with = "super::base64::option")]"#)?;
            } else {
                self.printer.println(r#"#[serde(with = "super::base64::vec")]"#)?;
            }
        }
        self.printer.println(&format!("pub {field_name}: {field_type},"))?;
        Ok(())
    }
    
    fn to_rust_type(&mut self, vim_type: &VimType) -> Result<String> {
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
            VimType::Reference(ref_name) => Ok(self.get_ref_type_declaration(ref_name)?),
        }
    }
    
    /// Generate the type declaration for a reference field - enum or struct.
    /// If enum return just the enum Pascal case name. If structure then return a Box<> 
    /// If the structure has children then we need dynamic trait reference.
    /// If the Struct has no children then we reference the Struct type.
    fn get_ref_type_declaration(&mut self, ref_name: &str) -> Result<String> {
        // If we cannot find the struct this is programatic error
        let rust_name = to_type_name(ref_name);
        if ref_name == "Any" {
            return Ok("VimAny".to_string());
        }
        if let Some(struct_type) = self.vim_model.structs.get(ref_name) {
            // TODO: Add Box only for structures and when there is an issue. One case is recursive types
            // that cannot be sized and thus a struct cannot be compiled. MethodFault contains
            // Option<MethodFault> and without Box it cannot be compiled. However for very many types
            // there may not be need for a Box. For example when all members are primitive it is safe to
            // not box a struct.
            let struct_ref = struct_type.borrow();
            if struct_ref.has_children() {
                Ok(format!("Box<dyn {}Trait>", rust_name))
            } else {
                Ok(format!("Box<{}>", rust_name))
            }
        } else if let Some(_) = self.vim_model.enums.get(ref_name) {
            Ok(rust_name)
        } else {
            Err(Error::TypeNotFound(ref_name.to_string()))
        }
    }
    
    // To allow for polymorphic fields every structure type that has descendants will have a trait
    // alternative that will be passed as dynamic reference. This trait will be implemented for
    // all of the structure type descendants. The trait will provide access to the struct type fields
    // and will extend the VimObjectTrait as to allow up and down casts.
    fn emit_trait_type(&mut self, name: &str, vim_type: &Struct) -> Result<()> {
        if !vim_type.has_children() { return Ok(()); }
        if ANY == vim_type.name { return Ok(()); } // Skip the Any type
        let struct_name = to_type_name(name);
        let Some(ref parent_trait) = vim_type.parent else {
            return Ok(()); // or error?
        };
        let base_trait = to_type_name(if ANY == parent_trait {
            "VimObject"
        } else {
            parent_trait
        });
        self.emit_doc(&vim_type.description)?;
        self.printer.println(&format!("pub trait {}Trait : {}Trait {{", struct_name, base_trait))?;
        self.printer.indent();
        for (prop_name, property) in &vim_type.properties {
            self.emit_trait_field(prop_name, property)?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        self.emit_any_into_trait(name)?;
        self.printer.println(&format!("serialize_trait_object!({}Trait);", struct_name))?;
        self.emit_trait_deserialization(name)?;
        Ok(())
    }
    
    fn emit_trait_field(&mut self, prop_name: &str, property: &Property) -> Result<()> {
        self.emit_doc(&property.description)?;
        let field_name = getter_name(&prop_name);
        let field_type = self.getter_return_type(property)?;
        self.printer.println(&format!("fn {field_name}(&self) -> {field_type};"))?;
        Ok(())
    }

    fn emit_any_into_trait(&mut self, name: &str) -> Result<()> {
        let fn_name = any_into_name(name);
        let type_name = to_type_name(name);
        self.printer.println(&format!(r#"pub fn {fn_name}(from: std::any::TypeId) -> Option<&'static AnyInto<dyn {type_name}Trait>> {{
    static mut TYPE_MAP: Option<std::collections::HashMap<std::any::TypeId, AnyInto<dyn {type_name}Trait>>> = None;
    static RUN_ONCE: std::sync::Once = std::sync::Once::new();
    RUN_ONCE.call_once(|| {{
        let mut map: std::collections::HashMap<std::any::TypeId, AnyInto<dyn {type_name}Trait>> = std::collections::HashMap::new();"#))?;
        // TODO Populate the map with the AnyInto instances by walking the struct hierarchy and adding all child types
        self.emit_any_into_map_entries(name)?;
        self.printer.println(r#"        unsafe {
            TYPE_MAP = Some(map);
        }
    });
    unsafe {
        TYPE_MAP.as_ref().unwrap().get(&from)
    }
}"#)?;
        self.printer.print(&format!(r#"impl<From: AsAny + ?Sized + 'static> CastFrom<From> for dyn {type_name}Trait {{
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {{
        let into = {fn_name}(from.type_id())?;
        (into.to_ref)(from.as_any_ref())
    }}

    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn any::Any + 'static>> {{
        let Some(into) = {fn_name}(from.as_ref().type_id()) else {{
            return Err(from.as_any_box());
        }};
        (into.to_box)(from.as_any_box())
    }}
}}"#))?;
        Ok(())
    }

    fn emit_any_into_map_entries(&mut self, name: &str) -> Result<()> {
        let type_name = to_type_name(name);
        self.printer.println(&format!(r#"        map.insert(std::any::TypeId::of::<{type_name}>(), AnyInto {{
            to_ref: |value| {{ Some(value.downcast_ref::<{type_name}>()?) }},
            to_box: |value| {{ Ok(value.downcast::<{type_name}>()?) }},
        }});"#))?;

        let Some(struct_type) = self.vim_model.structs.get(name) else {
            return Err(Error::TypeNotFound(name.to_string()));
        };
        for child_name in &struct_type.borrow().children {
            self.emit_any_into_map_entries(child_name)?;
        };
        Ok(())
    }
    
    fn getter_return_type(&mut self, property: &Property) -> Result<String> {
        let mut field_type = self.to_rust_type(&property.vim_type)?;
        if property.optional {
            field_type = format!("Option<{field_type}>");
        }
        if get_by_ref(&property.vim_type) {
            field_type = format!("&{field_type}");
        }
        if "&String" == field_type {
            field_type = "&str".to_string();
        }
        Ok(field_type)
    }
    
    fn emit_inherited_traits(&mut self, type_name: &String) -> Result<()> {
        let struct_name = &to_type_name(&type_name);
        let mut data_type = self.vim_model.structs.get(type_name).ok_or_else(|| Error::TypeNotFound(type_name.clone()))?.borrow();
        if data_type.has_children() {
            self.emit_trait_implementation(&data_type, struct_name)?;
        }
        let mut parent_opt = data_type.parent.as_ref();
        while let Some(parent_name) = parent_opt {
            if ANY == parent_name { break; }
            data_type = self.vim_model.structs.get(parent_name).ok_or_else(|| Error::TypeNotFound(parent_name.clone()))?.borrow();
            self.emit_trait_implementation(&data_type, struct_name)?;
            parent_opt = data_type.parent.as_ref();
        }

        Ok(())
    }
    
    
    
    /// Emits implementation of a structure type trat for a given structure. The trait should belong to
    /// the same structure or an ancestor
    fn emit_trait_implementation(&mut self, trait_type: &Struct, struct_name: &String) -> Result<()> {
        let base_name = to_type_name(&trait_type.name);
        self.printer.println(&format!("impl {}Trait for {} {{", base_name, struct_name))?;
        self.printer.indent();
        for (prop_name, property) in &trait_type.properties {
            self.emit_field_getter(prop_name, property)?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }
    
    fn emit_field_getter(&mut self, prop_name: &str, property: &Property) -> Result<()> {
        let getter_name = getter_name(&prop_name);
        let mut field_access = format!("self.{}",to_field_name(&prop_name));
        let field_type = self.getter_return_type(property)?;
        if get_by_ref(&property.vim_type) {
            field_access = format!("&{field_access}");
        }
        self.printer.println(&format!("fn {getter_name}(&self) -> {field_type} {{ {field_access} }}"))?;
        Ok(())
    }

    fn emit_trait_deserialization(&mut self, type_name: &str) -> Result<()> {
        let trait_name = format!("{}Trait", to_type_name(type_name));
        self.printer.println(&format!(r#"impl<'de> de::Deserialize<'de> for Box<dyn {trait_name}> {{
        fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {{
            deserializer.deserialize_map({trait_name}Visitor)
        }}
    }}
    
    struct {trait_name}Visitor;
    
    impl<'de> de::Visitor<'de> for {trait_name}Visitor {{
        type Value = Box<dyn {trait_name}>;
    
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {{
            formatter.write_str("a valid {trait_name} JSON object with a _typeName field")
    }}
    
        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: de::MapAccess<'de>,
        {{
            let deserializer = de::value::MapAccessDeserializer::new(&mut map);
            let any: VimAny = de::Deserialize::deserialize(deserializer)?;
            match any {{
                VimAny::Object(obj) => Ok(CastFrom::from_box(obj).map_err(|_| de::Error::custom("Internal error converting to trait type"))?),
                VimAny::Value(value) => {{
                    Err(de::Error::custom(format!("expected object not wrapped value: {{:?}}", value)))
                }}
            }}
        }}
    }}"#))?;
        Ok(())
    }

    fn emit_value_deserializers(&mut self) -> Result<()> {

        self.printer.println("type _ValueDeserializer = fn(v: &serde_json::value::RawValue) -> Result<VimAny, serde_json::Error>;")?;
        self.printer.println("static mut VALUE_DESERIALIZER_MAP: Option<std::collections::HashMap<&str, Box<_ValueDeserializer>>> = None;")?;
        self.printer.println("static INITIALIZE_VALUE_DESERIALIZERS: std::sync::Once = std::sync::Once::new();")?;
        
        self.printer.println("fn get_value_deserializer(type_name: &str) -> Option<&'static Box<_ValueDeserializer>> {")?;
        self.printer.indent();
        self.printer.println("INITIALIZE_VALUE_DESERIALIZERS.call_once(|| {")?;
        self.printer.indent();
        self.printer.println("let mut value_deserializers: std::collections::HashMap<&str, Box<_ValueDeserializer>> = std::collections::HashMap::new();")?;
        for (type_name, box_type) in &self.vim_model.any_value_types {
            if type_name == ANY {
                continue;
            }
            let enum_name = to_type_name(&box_type.name);
            let value_type = self.to_rust_type(&box_type.property_type)?;
            let discriminator = box_type.discriminator_value.as_ref().unwrap_or(type_name);
            self.printer.println(&format!(r#"value_deserializers.insert("{discriminator}", Box::new(|v| {{"#))?;
            self.printer.indent();
            self.printer.println(&format!("let value: {value_type} = de::Deserialize::deserialize(v)"))?;
            self.printer.indent();
            self.printer.println(".map_err(de::Error::custom)?;")?;
            self.printer.dedent();
            self.printer.println(&format!("return Ok(VimAny::Value(ValueElements::{enum_name}(value)));"))?;
            self.printer.dedent();
            self.printer.println("}));")?;
    
        }
        self.printer.println("unsafe { VALUE_DESERIALIZER_MAP = Some(value_deserializers); }")?;
        self.printer.dedent();
        self.printer.println("});")?;
        self.printer.println("let map = unsafe { VALUE_DESERIALIZER_MAP.as_ref().unwrap() };")?;
        self.printer.println("map.get(type_name)")?;
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }
    
    fn emit_object_deserializers(&mut self) -> Result<()> {
        self.printer.println("type AnyDeserializer<'a> = de::value::MapDeserializer<'a, std::vec::IntoIter<(String, &'a serde_json::value::RawValue)>, serde_json::Error>;")?;
        self.printer.println("type ObjectDeserializer = fn(AnyDeserializer) -> Result<VimAny, serde_json::Error>;")?;
        self.printer.println("static mut OBJECT_DESERIALIZER_MAP: Option<std::collections::HashMap<&str, Box<ObjectDeserializer>>> = None;")?;
        self.printer.println("static INITIALIZE_OBJECT_DESERIALIZERS: std::sync::Once = std::sync::Once::new();")?;
        self.printer.println("fn get_object_deserializer(type_name: &str) -> Option<&'static Box<ObjectDeserializer>> {")?;
        self.printer.indent();
        self.printer.println("INITIALIZE_OBJECT_DESERIALIZERS.call_once(|| {")?;
        self.printer.indent();
        self.printer.println("let mut object_deserializers: std::collections::HashMap<&str, Box<ObjectDeserializer>> = std::collections::HashMap::new();")?;
    
        for (discriminator, data_type) in &self.vim_model.structs {
            if discriminator == ANY { continue; }
            if !data_type.borrow().has_parent() { continue; }; // RequestTypes do not have parent and are not via inheritance
            let data_type = to_type_name(discriminator);
            self.printer.println(&format!(r#"object_deserializers.insert("{discriminator}", Box::new(|ds: AnyDeserializer| -> Result<VimAny, serde_json::Error> {{"#))?;
            self.printer.indent();
            self.printer.println(&format!(r#"let obj: {data_type} = de::Deserialize::deserialize(ds).map_err(de::Error::custom)?;"#))?;
            self.printer.println("Ok(VimAny::Object(Box::new(obj)))")?;
            self.printer.dedent();
            self.printer.println("}));")?;
        };
        self.printer.println("unsafe { OBJECT_DESERIALIZER_MAP = Some(object_deserializers); }")?;
        self.printer.dedent();
        self.printer.println("});")?;
        self.printer.println("let map = unsafe { OBJECT_DESERIALIZER_MAP.as_ref().unwrap() };")?;
        self.printer.println("map.get(type_name)")?;
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }
    
    fn emit_any_deserialization(&mut self) -> Result<()> {
        self.printer.println(r#"
    impl<'de> de::Deserialize<'de> for VimAny {
        fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_map(VimAnyVisitor)
        }
    }
    
    struct VimAnyVisitor;
    
    impl<'de> de::Visitor<'de> for VimAnyVisitor {
        type Value = VimAny;
    
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("vim JSON object with _typeName field discrimnator")
        }
    
        fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut type_name = None;
            let mut map_data: Vec<(String, &serde_json::value::RawValue)> = Vec::new();
            while let Some(key) = map.next_key::<String>()? {
                if key == "_typeName" {
                    if type_name.is_some() {
                        return Err(de::Error::duplicate_field("_typeName"));
                    }
                    let value: String = map.next_value()?;
                    type_name = Some(value);
                } else {
                    let value: &serde_json::value::RawValue = map.next_value()?;
                    map_data.push((key, value));
                }
            }
            let type_name: String = type_name.ok_or_else(|| de::Error::missing_field("_typeName"))?;
    
            // Process value elements
            if map_data.len() == 1 && map_data[0].0 == "_value" {
                let v: &serde_json::value::RawValue = map_data.get(0).ok_or_else(|| de::Error::missing_field("_value"))?.1;
                if let Some(value_deserializer) = get_value_deserializer(&type_name) {
                    return value_deserializer(v).map_err(de::Error::custom);
                }
            }
            let ds = de::value::MapDeserializer::new(map_data.into_iter());
            let deser = get_object_deserializer(&type_name)
                .ok_or_else(|| de::Error::custom(format!("unknown variant `{type_name}`")))?;
            deser(ds).map_err(de::Error::custom)
        }
    }"#)?;
        Ok(())
    }
    



}




/// Checks if type is to be returned as value copy or reference. Integer and float types are good to
/// copy. Structures, strings and arrays go by immutable reference
fn get_by_ref(vim_type: &VimType) -> bool {
    match &vim_type {
        VimType::Reference(_) => true,
        VimType::Array(_) => true,
        VimType::Binary => true,
        VimType::String => true,
        VimType::DateTime => true, // Uses string
        _ => false,
    }
}




