// Generator for Rust data models from vim

use std::borrow::Borrow;

use crate::vim_model::Model;

use super::super::printer::Printer;

use super::common::emit_description;
use super::deser::DeserializationGenerator;
use super::names::*;
use super::super::vim_model::*;
use super::errors::{Result, Error};

pub(crate) const ANY: &str = "Any";
pub(crate) const DATA_OBJECT: &str = "DataObject";

pub struct TypesEmitter<'a> {
    vim_model: &'a Model,
    printer: &'a mut dyn Printer,
    tdf: TypeDefResolver<'a>,
}


impl<'a> TypesEmitter<'a> {
    pub fn new(vim_model: &'a Model, printer: &'a mut dyn Printer) -> Self {
        TypesEmitter { vim_model, printer, tdf: TypeDefResolver::new(vim_model) }
    }

    pub fn emit_data_types(&mut self) -> Result<()> {
        self.emit_use_statements()?;
        // self.emit_vim_object()?;
        // self.emit_vimany()?;
        self.emit_structs()?;

        Ok(())
    }
    fn emit_use_statements(&mut self) -> Result<()> {
        //self.printer.println("use super::enums::*;")?;
        self.printer.println("use super::vim_any::VimAny;")?;
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

pub trait VimObjectTrait: AsAny  {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait;
}


impl<T> VimObjectTrait for T where T: AsAny  {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
}
}"#)?;
        Ok(())
    }

    fn emit_vimany(&mut self) -> Result<()> {
        self.printer.println("#[derive(Debug, serde::Serialize, serde::Deserialize)]")?;
        self.printer.println("#[serde(untagged)]")?;
        self.printer.println(r#"pub enum VimAny {
    Object(Box<DataObject>),
    Value(ValueElements),
}
"#)?;
        Ok(())
    }
    
    fn emit_structs(&mut self) -> Result<()> {
        for (name, vim_type_cell) in &self.vim_model.structs {
            let struct_type = vim_type_cell.borrow();
            if struct_type.name == "Any" {
                continue;
            }
            self.emit_struct_type(name, &struct_type)?;
            // self.emit_trait_type(name, &struct_type)?;
            // self.emit_inherited_traits(name)?;
        }
        Ok(())
    }

    /// Emit boxed value types from Vim like ArrayOfInt, ArrayOfString, Boolean etc.
    pub(crate) fn emit_boxed_types(&mut self) -> Result<()> {
        self.printer.println("use super::vim_any::VimAny;")?;
        self.printer.println("use super::structs::*;")?;
        //self.printer.println("use super::enums::*;")?;
        self.printer.newline()?;
        self.printer.println("#[derive(Debug, serde::Deserialize, serde::Serialize)]")?;
        self.printer.println("#[serde(tag = \"_typeName\", content = \"_value\")]")?;
        self.printer.println("pub enum ValueElements {")?;
        self.printer.indent();
        for (_, box_type) in &self.vim_model.any_value_types {
            {
                let this = &mut *self;
                let doc_string: &Option<String> = &box_type.description;
                emit_description(this.printer, doc_string)
            }?;
            let name = box_type.discriminator_value.as_ref().unwrap_or(&box_type.name);
            let type_name = to_type_name(&box_type.name);
            if &type_name != name {
                self.printer.println(&format!("#[serde(rename = \"{}\")]", name))?;
            }
            let rust_type = self.tdf.to_rust_field_type(&box_type.property_type)?;
            self.printer.println(&format!("{type_name}({rust_type}),"))?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }

    fn emit_struct_type(&mut self, name: &str, vim_type: &Struct) -> Result<()> {
        {
            let this = &mut *self;
            let doc_string: &Option<String> = &vim_type.description;
            emit_description(this.printer, doc_string)
        }?;
        let struct_name = to_type_name(name);
        let discriminator = vim_type.discriminator_value.clone().unwrap_or(name.to_string());
        if name == "ManagedObjectReference" {
            // Add Clone, PartialEq in addtion for ManagedObjectReference
            self.printer.println("#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]")?;
        } else {
            self.printer.println("#[derive(Debug, serde::Deserialize, serde::Serialize)]")?;
        }
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
        if vim_type.fields.is_empty() { return Ok(()); } // skip the comment if there are no fields
        self.printer.println(&format!("// Fields of {}", vim_type.name))?;
        for (_, property) in &vim_type.fields {
            self.emit_struct_field(property)?;
        }
        Ok(())
    }
    
    fn emit_struct_field(&mut self, field: &Field) -> Result<()> {
        {
            let this = &mut *self;
            let doc_string: &Option<String> = &field.description;
            emit_description(this.printer, doc_string)
        }?;
        let field_name = to_field_name(&field.name);
        let field_type = self.tdf.field_type(field)?;
        if field.optional {
            self.printer.println(&format!("#[serde(default, skip_serializing_if = \"Option::is_none\")]"))?;
        }
        if field_name != field.name {
            self.printer.println(&format!(r#"#[serde(rename = "{}")]"#, field.name))?;
        }
        if field.vim_type == DataType::Binary {
            if field.optional {
                self.printer.println(r#"#[serde(with = "crate::core::base64::option")]"#)?;
            } else {
                self.printer.println(r#"#[serde(with = "crate::core::base64::vec")]"#)?;
            }
        }
        self.printer.println(&format!("pub {field_name}: {field_type},"))?;
        Ok(())
    }
    
    // To allow for polymorphic fields every structure type that has descendants will have a trait
    // alternative that will be passed as dynamic reference. This trait will be implemented for
    // all of the structure type descendants. The trait will provide access to the struct type fields
    // and will extend the VimObjectTrait as to allow up and down casts.
    fn emit_trait_type(&mut self, name: &str, vim_type: &Struct) -> Result<()> {
        if !vim_type.has_children() { return Ok(()); }
        if ANY == name { return Ok(()); } // Skip the Any type
        let struct_name = to_type_name(name);
        let Some(ref parent_trait) = vim_type.parent else {
            return Ok(()); // or error?
        };
        let base_trait = to_type_name(if ANY == parent_trait {
            "VimObject"
        } else {
            parent_trait
        });
        {
            let this = &mut *self;
            let doc_string: &Option<String> = &vim_type.description;
            emit_description(this.printer, doc_string)
        }?;
        self.printer.println(&format!("pub trait {}Trait : {}Trait {{", struct_name, base_trait))?;
        self.printer.indent();
        if DATA_OBJECT == name {
            self.printer.println("/// Retrieve the serialization type name")?;
            self.printer.println("fn type_name_(&self) -> &'static str;")?;
        }
        for (prop_name, property) in &vim_type.fields {
            self.emit_trait_field(prop_name, property)?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        // self.emit_any_into_trait(name)?;
//         self.printer.println(&format!(r#"impl<'s> serde::Serialize for dyn {struct_name}Trait + 's {{
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {{
//         self.as_vim_object_ref().serialize(serializer)
//     }}
// }}"#))?;
//         self.emit_trait_deserialization(name)?;
        Ok(())
    }
    
    fn emit_trait_field(&mut self, prop_name: &str, property: &Field) -> Result<()> {
        let this = &mut *self;
        let doc_string: &Option<String> = &property.description;
        emit_description(this.printer, doc_string)?;
        let field_name = getter_name(&prop_name);
        let field_type = self.getter_return_type(property)?;
        self.printer.println(&format!("fn {field_name}(&self) -> {field_type};"))?;
        Ok(())
    }

    fn emit_any_into_trait(&mut self, name: &str) -> Result<()> {
        let fn_name = any_into_name(name);
        let type_name = to_type_name(name);
        self.printer.println(&format!(r#"fn {fn_name}(from: std::any::TypeId) -> Option<&'static AnyInto<dyn {type_name}Trait>> {{
    static TYPE_MAP: OnceLock<std::collections::HashMap<std::any::TypeId, AnyInto<dyn {type_name}Trait>>> = OnceLock::new();
    
    TYPE_MAP.get_or_init(|| {{
        let mut map: std::collections::HashMap<std::any::TypeId, AnyInto<dyn {type_name}Trait>> = std::collections::HashMap::new();"#))?;
        // TODO Populate the map with the AnyInto instances by walking the struct hierarchy and adding all child types
        self.emit_any_into_map_entries(name)?;
        self.printer.println(r#"map
    }).get(&from)
}"#)?;
        self.printer.println(&format!(r#"impl<From: AsAny + ?Sized + 'static> CastFrom<From> for dyn {type_name}Trait {{
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
    
    fn getter_return_type(&mut self, property: &Field) -> Result<String> {
        let mut field_type = self.tdf.field_type(property)?;
        if get_by_ref(&property.vim_type) {
            field_type = format!("&{field_type}");
        }
        if "&String" == field_type {
            field_type = "&str".to_string();
        }
        Ok(field_type)
    }
    
    fn emit_inherited_traits(&mut self, type_name: &String) -> Result<()> {
        let mut data_type = self.vim_model.structs.get(type_name).ok_or_else(|| Error::TypeNotFound(type_name.clone()))?.borrow();
        if data_type.has_children() {
            self.emit_trait_implementation(&data_type, type_name)?;
        }
        let mut parent_opt = data_type.parent.as_ref();
        while let Some(parent_name) = parent_opt {
            if ANY == parent_name { break; }
            data_type = self.vim_model.structs.get(parent_name).ok_or_else(|| Error::TypeNotFound(parent_name.clone()))?.borrow();
            self.emit_trait_implementation(&data_type, type_name)?;
            parent_opt = data_type.parent.as_ref();
        }

        Ok(())
    }
    
    
    
    /// Emits implementation of a structure type trait for a given structure. The trait should belong to
    /// the same structure or an ancestor
    fn emit_trait_implementation(&mut self, trait_type: &Struct, type_name: &String) -> Result<()> {
        let base_name = to_type_name(&trait_type.name);
        let struct_name = &to_type_name(&type_name);
        self.printer.println(&format!("impl {}Trait for {} {{", base_name, struct_name))?;
        self.printer.indent();
        if DATA_OBJECT == trait_type.name {
            self.printer.println("fn type_name_(&self) -> &'static str {")?;
            self.printer.indent();
            self.printer.println(&format!("\"{}\"", type_name))?;
            self.printer.dedent();
            self.printer.println("}")?;
        }
        for (prop_name, property) in &trait_type.fields {
            self.emit_field_getter(prop_name, property)?;
        }
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }
    
    fn emit_field_getter(&mut self, prop_name: &str, property: &Field) -> Result<()> {
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

}




/// Checks if type is to be returned as value copy or reference. Integer and float types are good to
/// copy. Structures, strings and arrays go by immutable reference
pub fn get_by_ref(vim_type: &DataType) -> bool {
    match &vim_type {
        DataType::Reference(_) => true,
        DataType::Array(_) => true,
        DataType::Binary => true,
        DataType::String => true,
        DataType::DateTime => true, // Uses string
        _ => false,
    }
}




