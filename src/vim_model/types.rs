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
    pub properties: IndexMap<String, Property>,
    pub parent: Option<String>,
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
pub struct Property {
    pub name: String,
    pub description: Option<String>,
    pub optional: bool,
    pub type_: VimType,
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
pub enum VimType {
    Boolean,
    String,
    Int32,
    Int64,
    Float,
    Double,
    DateTime,
    Array(Box<VimType>),
    Enum(String),
    Struct(String),
}

impl TryFrom<&RefOr<Schema>> for VimType {
    type Error = super::Error;
    fn try_from(schema: &RefOr<Schema>) -> Result<Self> {
        match schema {
            RefOr::Ref { reference } => Ok(VimType::Struct(
                reference_to_rust_name(reference)?.to_string(),
            )),
            RefOr::Val(inline_schema) => match inline_schema.as_ref() {
                Schema {
                    schema_type: Some(SchemaType::Boolean),
                    ..
                } => Ok(VimType::Boolean),
                Schema {
                    schema_type: Some(SchemaType::String),
                    format: Some(DataFormat::DateTime),
                    ..
                } => Ok(VimType::DateTime),
                Schema {
                    schema_type: Some(SchemaType::String),
                    ..
                } => Ok(VimType::String),
                Schema {
                    schema_type: Some(SchemaType::Number),
                    ..
                } => match &inline_schema.format {
                    Some(DataFormat::Int32) => Ok(VimType::Int32),
                    Some(DataFormat::Int64) => Ok(VimType::Int64),
                    Some(DataFormat::Float) => Ok(VimType::Float),
                    Some(DataFormat::Double) => Ok(VimType::Double),
                    Some(format) => Err(super::Error::UnsupportedFormat(format.to_string())),
                    None => Err(super::Error::MissingFormat(SchemaType::Number.to_string())),
                },
                Schema {
                    schema_type: Some(SchemaType::Integer),
                    ..
                } => match &inline_schema.format {
                    Some(DataFormat::Int32) => Ok(VimType::Int32),
                    Some(DataFormat::Int64) => Ok(VimType::Int64),
                    Some(format) => Err(super::Error::UnsupportedFormat(format.to_string())),
                    None => Err(super::Error::MissingFormat(SchemaType::Integer.to_string())),
                },
                Schema {
                    schema_type: Some(SchemaType::Array),
                    items: Some(items),
                    ..
                } => {
                    let array_type = VimType::try_from(items)?;
                    Ok(VimType::Array(Box::new(array_type)))
                }
                _ => Err(super::Error::UnsupportedType(format!(
                    "{:?}",
                    inline_schema
                ))),
            },
        }
    }
}

/// Represents a Vim RequestType.
/// For example:
/// ```yaml
/// AcquireGenericServiceTicketRequestType:
/// type: object
/// description: |2
///   The parameters of *SessionManager.AcquireGenericServiceTicket*.
/// properties:
///   spec:
///     description: |2
///       specification for the service request which will be
///       invoked with the ticket.
///     $ref: '#/components/schemas/SessionManagerServiceRequestSpec'
/// required:
///   - spec
/// ```
/// There is no parent field because RequestType is not a child of any other type.
#[derive(Debug, PartialEq)]
pub struct RequestType {
    pub name: String,
    pub description: Option<String>,
    pub properties: IndexMap<String, Property>,
}

/// Represents a Vim BoxType. This is a type that has single required property. No inherited
/// properties from all of except discriminator. Boxes have parent classes. No descendants.
///
/// Box types are grouped by their parent and their name. Thus we can emit a Rust enum that can be
/// processed by serde. For example:
/// ```rust
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
    pub description: String,
    pub property_name: String,
    pub property_type: VimType,
    pub parent: Option<String>,
}

/// Represents the VIM API data model build from OpenAPI model.
#[derive(Debug, PartialEq)]
pub struct VimModel {
    pub enums: IndexMap<String, Enum>,
    pub structs: IndexMap<String, Struct>,
    pub request_types: IndexMap<String, RequestType>,
    /// The BoxType classes grouped by their parent struct type and by property name.
    pub box_types_by_parent: IndexMap<String, IndexMap<String, BoxType>>,
}
