use std::cell::RefCell;

use check_keyword::CheckKeyword;
use convert_case::{Case, Casing};
use indexmap::IndexMap;

use super::*;
use crate::oas30::*;

/// Represents a Vim Enum model. This is a set of string values.
/// For example:
/// ```yaml
/// ManagedEntityStatus_enum:
/// type: string
/// description: |2
///   The Status enumeration defines a general "health" value for a managed entity.
///   Possible values:
///   - `gray`: The status is unknown.
///   - `green`: The entity is OK.
///   - `yellow`: The entity might have a problem.
///   - `red`: The entity definitely has a problem.
/// enum:
///   - gray
///   - green
///   - yellow
///   - red
/// ```
#[derive(Debug, PartialEq)]
pub struct Enum {
    pub name: String,
    pub description: Option<String>,
    pub variants: Vec<String>,
    pub discriminator_value: Option<String>,
}

/// Represents a Vim Struct model. All of these except the `Any` type have a parent.
/// For example:
/// ```yaml
/// VirtualVmxnet3Vrdma:
/// type: object
/// description: |2
///   The VirtualVmxnet3Vrdma data object type represents an instance of the
///   VRDMA virtual Remote Direct Memory Access adapter attached to a virtual
///   machine.
/// properties:
///   deviceProtocol:
///     description: |2
///       VRDMA Device protocol.
///   
///       See
///       *VirtualVmxnet3VrdmaOptionDeviceProtocols_enum* for more information.
///     type: string
/// allOf:
///   - $ref: '#/components/schemas/VirtualVmxnet3'
/// ```
#[derive(Debug, PartialEq)]
pub struct Struct {
    pub name: String,
    pub description: Option<String>,
    pub fields: IndexMap<String, Field>,
    pub parent: Option<String>,
    pub discriminator_value: Option<String>,
    pub children: Vec<String>,
}

impl Struct {
    pub fn rust_name(&self) -> String {
        self.name.to_case(Case::Pascal).into_safe()
    }

    pub fn field_name(&self) -> String {
        self.name.to_case(Case::Snake).into_safe()
    }


    pub fn discriminator(&self) -> String {
        self.name.clone()
    }

    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

/// Represents a Vim Property model.
/// For example:
/// ```yaml
///  deviceProtocol:
///    description: |2
///     VRDMA Device protocol.
///
///     See
///    *VirtualVmxnet3VrdmaOptionDeviceProtocols_enum* for more information.
///   type: string
/// ```
#[derive(Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub description: Option<String>,
    pub optional: bool,
    pub vim_type: DataType,
    /// Indicator that the field needs to be rendered as pointer type to break a cyclic reference
    /// and thus keep the struct type Sized.
    pub require_box: bool,
}

impl Field {
    pub fn rust_name(&self) -> String {
        self.name.to_case(Case::Snake).into_safe()
    }
}

/// Represents a Vim PropertyType model.
/// For example:
/// ```yaml
/// type: string
/// ```
/// or
/// ```yaml
/// type: number
/// format: float
/// ```
/// or
/// ```yaml
/// type: integer
/// format: int64
/// ```
#[derive(Debug, PartialEq)]
pub enum DataType {
    Boolean,
    String,
    Int8,
    Int16,
    Int32,
    Int64,
    Float,
    Double,
    DateTime,
    Binary,
    Array(Box<DataType>),
    Reference(String),
}

impl TryFrom<&RefOr<Schema>> for DataType {
    type Error = super::Error;
    fn try_from(schema: &RefOr<Schema>) -> Result<Self> {
        match schema {
            RefOr::Ref { reference, ..} => Ok(DataType::Reference(
                reference_to_schema_name(reference)?.to_string(),
            )),
            RefOr::Val(inline_schema) => match inline_schema.as_ref() {
                Schema {
                    schema_type: Some(SchemaType::Boolean),
                    ..
                } => Ok(DataType::Boolean),
                Schema {
                    schema_type: Some(SchemaType::String),
                    format: Some(DataFormat::DateTime),
                    ..
                } => Ok(DataType::DateTime),
                Schema {
                    schema_type: Some(SchemaType::String),
                    format: Some(DataFormat::Byte),
                    ..
                } => Ok(DataType::Binary),
                Schema {
                    schema_type: Some(SchemaType::String),
                    ..
                } => Ok(DataType::String),
                Schema {
                    schema_type: Some(SchemaType::Number),
                    ..
                } => match &inline_schema.format {
                    Some(DataFormat::Int32) => Ok(DataType::Int32),
                    Some(DataFormat::Int64) => Ok(DataType::Int64),
                    Some(DataFormat::Float) => Ok(DataType::Float),
                    Some(DataFormat::Double) => Ok(DataType::Double),
                    Some(format) => Err(super::Error::UnsupportedFormat(format.to_string())),
                    None => Err(super::Error::MissingFormat(SchemaType::Number.to_string())),
                },
                Schema {
                    schema_type: Some(SchemaType::Integer),
                    ..
                } => match &inline_schema.as_ref() {
                    Schema {
                        format: Some(DataFormat::Int32),
                        ..
                    } => Ok(DataType::Int32),
                    Schema {
                        format: Some(DataFormat::Int64),
                        ..
                    } => Ok(DataType::Int64),
                    Schema {
                        minimum: Some(-128.0),
                        maximum: Some(127.0),
                        ..
                    } => Ok(DataType::Int8),
                    Schema {
                        minimum: Some(-32768.0),
                        maximum: Some(32767.0),
                        ..
                    } => Ok(DataType::Int16),
                    _ => Err(super::Error::UnsupportedFormat(SchemaType::Integer.to_string())),
                },
                Schema {
                    schema_type: Some(SchemaType::Array),
                    items: Some(items),
                    ..
                } => {
                    let array_type = DataType::try_from(items)?;
                    Ok(DataType::Array(Box::new(array_type)))
                }
                _ => Err(super::Error::UnsupportedType(format!(
                    "{:?}",
                    inline_schema
                ))),
            },
        }
    }
}

/// Represents a Vim BoxType. This is a type that has single required property. No inherited
/// properties from all of except discriminator. Boxes have parent classes. No descendants.
/// 
/// Box types are grouped by their parent and their name. Thus we can emit a Rust enum that can be
/// processed by serde. For example:
/// ```test
/// #[derive(Serialize, Deserialize)]
/// #[derive(Debug, PartialEq, Serialize, Deserialize)]
/// #[serde(tag = "_typeName", content = "_value")]
/// pub enum ValueElements {
///     PrimitiveBoolean(bool),
///     ArrayOfString(Vec<String>),
///     ArrayOfInt(Vec<i64>),
/// }
/// ```
/// `_typeName` is the discriminator. `_value` is the actual value. The enum variants are named
/// after the type name and the type reflects the schema type.
///
/// In vim all box types extend from `Any` and have `_value`` property. So we need not store the
/// parent class or property name. We only need a list of box types with their type, name and
/// description.
///
/// For example:
/// ```yaml
///     PrimitiveBoolean:
/// type: object
/// description: |2
///   A boxed Boolean primitive. To be used in *Any* placeholders.
/// properties:
///   _value:
///     type: boolean
/// required:
///   - _value
/// allOf:
///   - $ref: '#/components/schemas/Any'
/// ```
#[derive(Debug, PartialEq)]
pub struct BoxType {
    pub name: String,
    pub description: Option<String>,
    pub property_type: DataType,
    pub discriminator_value: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct ManagedObject {
    pub name: String,
    pub description: Option<String>,
    pub methods: Vec<Method>,
}

#[derive(Debug, PartialEq)]
pub struct Method {
    pub name: String,
    pub description: Option<String>,
    pub path: String,
    pub http_method: HttpMethod,
    pub input: Option<DataType>,
    pub output: Option<DataType>,
    pub output_description: Option<String>,
    pub error_description: Option<String>,
    pub optional_response: bool,
}

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
}

/// Represents the VIM API data model build from OpenAPI model.
#[derive(Debug, PartialEq)]
pub struct Model {
    pub enums: IndexMap<String, Enum>,
    pub structs: IndexMap<String, RefCell<Struct>>,
    pub request_types: IndexMap<String, RefCell<Struct>>,
    pub any_value_types: IndexMap<String, BoxType>,
    pub managed_objects: IndexMap<String, ManagedObject>,
}

impl Model {
    pub fn is_struct_type(&self, vim_type: &DataType) -> bool {
        match vim_type {
            DataType::Reference(ref_name) => {
                if let Some(struct_type) = self.structs.get(ref_name) {
                    return struct_type.borrow().name != "Any";
                }
            }
            _ => {}
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_try_from() {
        let schema = Schema {
            schema_type: Some(SchemaType::String),
            ..Default::default()
        };
        let typ = DataType::try_from(&RefOr::Val(Box::new(schema))).unwrap();
        assert_eq!(typ, DataType::String);
    }

    #[test]
    fn test_datetime_try_from() {
        let schema = Schema {
            schema_type: Some(SchemaType::String),
            format: Some(DataFormat::DateTime),
            ..Default::default()
        };
        let typ = DataType::try_from(&RefOr::Val(Box::new(schema))).unwrap();
        assert_eq!(typ, DataType::DateTime);
    }

    #[test]
    fn test_string_array_try_from() {
        let schema = Schema {
            schema_type: Some(SchemaType::Array),
            items: Some(RefOr::Val(Box::new(Schema {
                schema_type: Some(SchemaType::String),
                ..Default::default()
            }))),
            ..Default::default()
        };
        let typ = DataType::try_from(&RefOr::Val(Box::new(schema))).unwrap();
        assert_eq!(typ, DataType::Array(Box::new(DataType::String)));
    }

    #[test]
    fn test_unsafe_property_name() {
        let prop = Field {
            name: "Crate".to_string(),
            description: None,
            optional: false,
            vim_type: DataType::String,
            require_box: false,
        };
        assert_eq!(prop.rust_name(), "crate_");
    }

    #[test]
    fn test_mixed_struct_name() {
        let str = Struct {
            name: "StructCrate_Enum".to_string(),
            description: None,
            fields: IndexMap::new(),
            parent: None,
            discriminator_value: None,
            children: vec![],
        };
        assert_eq!(str.rust_name(), "StructCrateEnum");
    }
}