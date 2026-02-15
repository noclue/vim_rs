use super::vim_any::VimAny;

// ============================================================================
// Static Deserialization Framework
//
// This module contains all the reusable infrastructure for polymorphic
// deserialization that doesn't depend on specific types. Type-specific
// implementations go in the generated deserialize.rs.
// ============================================================================

// ============================================================================
// Type Registry Abstraction
// ============================================================================

/// Type information enum - encodes type category and associated functions.
/// Each variant carries only the data relevant to that type category.
pub enum TypeInfo {
    /// Object type - fields are spread across the JSON object
    Object {
        name: &'static str,
        builder_fn: fn() -> Box<dyn FieldsBuilder>,
    },
    /// Value type (primitives, arrays, polymorphic arrays) - uses wrapper format {"_typeName":"...", "_value":...}
    /// Provides both fast path (direct deserializer) and slow path (from Value) deserialization.
    Value {
        name: &'static str,
        /// Fast path: creates a ValueDeserializer for direct deserialization of _value
        make_deserializer: fn() -> Box<dyn ValueDeserializer>,
        /// Slow path: deserialize from buffered miniserde::json::Value
        from_value: fn(&miniserde::json::Value) -> miniserde::Result<super::boxed_types::ValueElements>,
    },
}

impl TypeInfo {
    /// Get the type name
    #[inline]
    pub fn name(&self) -> &'static str {
        match self {
            TypeInfo::Object { name, .. } => name,
            TypeInfo::Value { name, .. } => name,
        }
    }

    /// Returns true if this is an Object type
    #[inline]
    pub fn is_object(&self) -> bool {
        matches!(self, TypeInfo::Object { .. })
    }

    /// Create a FieldsBuilder (only valid for Object types)
    #[inline]
    pub fn create_builder(&self) -> Option<Box<dyn FieldsBuilder>> {
        match self {
            TypeInfo::Object { builder_fn, .. } => Some(builder_fn()),
            _ => None,
        }
    }

    /// Deserialize from buffered Value (slow path)
    #[inline]
    pub fn deserialize_from_value(
        &self,
        value: &miniserde::json::Value,
    ) -> miniserde::Result<super::boxed_types::ValueElements> {
        match self {
            TypeInfo::Value { from_value, .. } => from_value(value),
            _ => Err(miniserde::Error),
        }
    }

    /// Create a unified PolyBuilder (works for both Object and Value types)
    #[inline]
    pub fn create_poly_builder(&self) -> miniserde::Result<Box<dyn PolyBuilder>> {
        match self {
            TypeInfo::Object { builder_fn, .. } => {
                Ok(Box::new(ObjectPolyBuilder::new(builder_fn())))
            }
            TypeInfo::Value {
                make_deserializer, ..
            } => Ok(Box::new(ValuePolyBuilder::new(make_deserializer()))),
        }
    }
}

// ============================================================================
// Direct Value Deserializers (True Fast Path)
// ============================================================================

/// Trait for value deserializers - combines Visitor with finish capability
pub trait ValueDeserializer: miniserde::de::Visitor {
    /// Convert internal storage to ValueElements
    fn finish_value(&mut self) -> miniserde::Result<super::boxed_types::ValueElements>;
}

// ============================================================================
// Generic Delegating Deserializer
//
// Reuses miniserde's existing Deserialize implementations for all types.
// Just stores Option<T> and delegates all visitor methods to miniserde.
// ============================================================================

pub struct DelegatingDeserializer<T> {
    pub value: Option<T>,
    pub wrap: fn(T) -> miniserde::Result<super::boxed_types::ValueElements>,
}

impl<T> DelegatingDeserializer<T> {
    pub fn new(wrap: fn(T) -> miniserde::Result<super::boxed_types::ValueElements>) -> Self {
        Self { value: None, wrap }
    }
}

/// Delegates all visitor methods to miniserde's existing Deserialize implementations
impl<T: miniserde::Deserialize> miniserde::de::Visitor for DelegatingDeserializer<T> {
    fn null(&mut self) -> miniserde::Result<()> {
        miniserde::Deserialize::begin(&mut self.value).null()
    }
    fn boolean(&mut self, b: bool) -> miniserde::Result<()> {
        miniserde::Deserialize::begin(&mut self.value).boolean(b)
    }
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        miniserde::Deserialize::begin(&mut self.value).string(s)
    }
    fn negative(&mut self, n: i64) -> miniserde::Result<()> {
        miniserde::Deserialize::begin(&mut self.value).negative(n)
    }
    fn nonnegative(&mut self, n: u64) -> miniserde::Result<()> {
        miniserde::Deserialize::begin(&mut self.value).nonnegative(n)
    }
    fn float(&mut self, n: f64) -> miniserde::Result<()> {
        miniserde::Deserialize::begin(&mut self.value).float(n)
    }
    fn seq(&mut self) -> miniserde::Result<Box<dyn miniserde::de::Seq + '_>> {
        miniserde::Deserialize::begin(&mut self.value).seq()
    }
    fn map(&mut self) -> miniserde::Result<Box<dyn miniserde::de::Map + '_>> {
        miniserde::Deserialize::begin(&mut self.value).map()
    }
}

impl<T: miniserde::Deserialize> ValueDeserializer for DelegatingDeserializer<T> {
    fn finish_value(&mut self) -> miniserde::Result<super::boxed_types::ValueElements> {
        (self.wrap)(self.value.take().ok_or(miniserde::Error)?)
    }
}

// ============================================================================
// WrapValue trait - defines how each type wraps into ValueElements
// ============================================================================

pub trait WrapValue: miniserde::Deserialize + Sized + 'static {
    fn wrap(self) -> super::boxed_types::ValueElements;
}

// Generic functions for PHF entries - these become the function pointers
pub fn make_deser<T: WrapValue>() -> Box<dyn ValueDeserializer> {
    Box::new(DelegatingDeserializer::<T>::new(|v| Ok(v.wrap())))
}

pub fn from_val<T: WrapValue>(
    v: &miniserde::json::Value,
) -> miniserde::Result<super::boxed_types::ValueElements> {
    Ok(super::mini_helpers::from_value::<T>(v)?.wrap())
}

// ============================================================================
// Polymorphic array cast helper
// ============================================================================

/// Generic helper to cast VimObjectHolder vec to a target trait vec.
#[inline]
pub fn polymorphic_array_cast<T: ?Sized + 'static>(
    holders: Vec<VimObjectHolder>,
    wrap: fn(Vec<Box<T>>) -> super::boxed_types::ValueElements,
) -> miniserde::Result<super::boxed_types::ValueElements>
where
    T: super::convert::CastFrom<dyn super::vim_object_trait::VimObjectTrait>,
{
    let items: Vec<Box<T>> = holders
        .into_iter()
        .map(|h| {
            let vim_obj = h.out.ok_or(miniserde::Error)?;
            <T>::from_box(vim_obj).map_err(|_| miniserde::Error)
        })
        .collect::<miniserde::Result<Vec<_>>>()?;
    Ok(wrap(items))
}

// ============================================================================
// Trait for polymorphic field builders
// ============================================================================

/// Trait for field builders that can be used in polymorphic deserialization.
/// Extends Map to inherit the key() method - no need to implement it twice.
/// Implemented by each *Fields struct to enable dynamic dispatch.
///
/// Returns Box<dyn VimObjectTrait> - the universal base trait. Callers that need
/// a more specific trait must cast using CastFrom.
pub trait FieldsBuilder: miniserde::de::Map {
    /// Build the concrete type and box it as a VimObjectTrait.
    fn build_boxed(&mut self) -> miniserde::Result<Box<dyn super::vim_object_trait::VimObjectTrait>>;
}

// ============================================================================
// Unified Polymorphic Builder Trait
// ============================================================================

/// Unified trait for polymorphic builders - handles both objects and values.
/// This abstraction allows PolyCore to treat both cases uniformly.
pub trait PolyBuilder: miniserde::de::Map {
    /// Build and return as VimAny (Object or Value)
    fn finish_poly(&mut self) -> miniserde::Result<VimAny>;
}

/// Adapter that wraps FieldsBuilder to implement PolyBuilder for objects
pub struct ObjectPolyBuilder {
    delegate: Box<dyn FieldsBuilder>,
}

impl ObjectPolyBuilder {
    pub fn new(delegate: Box<dyn FieldsBuilder>) -> Self {
        Self { delegate }
    }
}

impl miniserde::de::Map for ObjectPolyBuilder {
    fn key(&mut self, key: &str) -> miniserde::Result<&mut dyn miniserde::de::Visitor> {
        if key == "_typeName" {
            // Ignore _typeName in object mode - we already know the type
            Ok(<dyn miniserde::de::Visitor>::ignore())
        } else {
            self.delegate.key(key)
        }
    }

    fn finish(&mut self) -> miniserde::Result<()> {
        self.delegate.finish()
    }
}

impl PolyBuilder for ObjectPolyBuilder {
    fn finish_poly(&mut self) -> miniserde::Result<VimAny> {
        Ok(VimAny::Object(self.delegate.build_boxed()?))
    }
}

/// Adapter that wraps ValueDeserializer to implement PolyBuilder for values
pub struct ValuePolyBuilder {
    deserializer: Box<dyn ValueDeserializer>,
}

impl ValuePolyBuilder {
    pub fn new(deserializer: Box<dyn ValueDeserializer>) -> Self {
        Self { deserializer }
    }
}

impl miniserde::de::Map for ValuePolyBuilder {
    fn key(&mut self, key: &str) -> miniserde::Result<&mut dyn miniserde::de::Visitor> {
        if key == "_value" {
            Ok(self.deserializer.as_mut())
        } else {
            // Ignore other fields (like _typeName)
            Ok(<dyn miniserde::de::Visitor>::ignore())
        }
    }

    fn finish(&mut self) -> miniserde::Result<()> {
        Ok(())
    }
}

impl PolyBuilder for ValuePolyBuilder {
    fn finish_poly(&mut self) -> miniserde::Result<VimAny> {
        Ok(VimAny::Value(self.deserializer.finish_value()?))
    }
}

// ============================================================================
// Universal holder for polymorphic deserialization
// ============================================================================

/// Universal holder for polymorphic deserialization of objects.
/// Holds Box<dyn VimObjectTrait> - can deserialize ANY object type in the hierarchy.
/// Callers cast to their required trait after deserialization.
pub struct VimObjectHolder {
    pub out: Option<Box<dyn super::vim_object_trait::VimObjectTrait>>,
}

// Note: Deserialize and Visitor impls for VimObjectHolder are in the generated
// deserialize.rs because they need the Place type from the make_place! macro.

// ============================================================================
// Shared Polymorphic State Machine
// ============================================================================

/// Polymorphic builder mode - shared between VimObjectHolder and VimAny builders
pub enum PolyMode {
    /// Initial state - haven't seen any fields yet
    Initial,
    /// Fast path: _typeName was first, using unified PolyBuilder (handles both objects and values)
    Direct { builder: Box<dyn PolyBuilder> },
    /// Slow path: _typeName was not first, buffering all fields
    Buffered {
        buffer: miniserde::json::Object,
        current_key: Option<String>,
        current_value: Option<miniserde::json::Value>,
    },
}

/// Core polymorphic state machine. Handles key() logic for both builders.
pub struct PolyCore {
    pub mode: PolyMode,
    pub type_name: Option<String>,
}

impl PolyCore {
    pub fn new() -> Self {
        Self {
            mode: PolyMode::Initial,
            type_name: None,
        }
    }

    /// Handle a key. Returns the visitor for the value.
    /// Requires the lookup_type function from generated code.
    pub fn key<F>(
        &mut self,
        key: &str,
        lookup_type: F,
    ) -> miniserde::Result<&mut dyn miniserde::de::Visitor>
    where
        F: Fn(&str) -> Option<&'static TypeInfo>,
    {
        // Check if we need to transition from Initial after capturing _typeName
        if matches!(&self.mode, PolyMode::Initial) {
            if let Some(type_name) = &self.type_name {
                let type_info = lookup_type(type_name).ok_or(miniserde::Error)?;
                let builder = type_info.create_poly_builder()?;
                self.mode = PolyMode::Direct { builder };
            }
        }

        // Handle Initial mode (first field)
        if matches!(&self.mode, PolyMode::Initial) {
            if key == "_typeName" {
                // Fast path: _typeName is first
                return Ok(miniserde::de::Deserialize::begin(&mut self.type_name));
            } else {
                // Slow path: First field is not _typeName, switch to buffering
                self.mode = PolyMode::Buffered {
                    buffer: miniserde::json::Object::new(),
                    current_key: Some(key.to_owned()),
                    current_value: None,
                };
            }
        }

        match &mut self.mode {
            PolyMode::Initial => unreachable!(),
            PolyMode::Direct { builder } => {
                // Delegate to the unified PolyBuilder (handles both objects and values)
                builder.key(key)
            }
            PolyMode::Buffered {
                buffer,
                current_key,
                current_value,
            } => {
                // Shift previous field to buffer
                if let (Some(k), Some(v)) = (current_key.take(), current_value.take()) {
                    buffer.insert(k, v);
                }
                *current_key = Some(key.to_owned());
                Ok(miniserde::de::Deserialize::begin(current_value))
            }
        }
    }

    /// Finalize and ensure transition from Initial if needed
    pub fn prepare_finish<F>(&mut self, lookup_type: F) -> miniserde::Result<()>
    where
        F: Fn(&str) -> Option<&'static TypeInfo>,
    {
        if matches!(&self.mode, PolyMode::Initial) {
            if let Some(type_name) = &self.type_name {
                let type_info = lookup_type(type_name).ok_or(miniserde::Error)?;
                let builder = type_info.create_poly_builder()?;
                self.mode = PolyMode::Direct { builder };
            }
        }
        Ok(())
    }

    /// Finish for VimAny mode (objects or values)
    pub fn finish<F>(&mut self, lookup_type: F) -> miniserde::Result<VimAny>
    where
        F: Fn(&str) -> Option<&'static TypeInfo>,
    {
        self.prepare_finish(&lookup_type)?;

        match &mut self.mode {
            PolyMode::Initial => Err(miniserde::Error),
            PolyMode::Direct { builder } => {
                // Unified finish via PolyBuilder
                builder.finish_poly()
            }
            PolyMode::Buffered {
                buffer,
                current_key,
                current_value,
            } => {
                // Shift last field
                if let (Some(k), Some(v)) = (current_key.take(), current_value.take()) {
                    buffer.insert(k, v);
                }

                let type_name = buffer
                    .get("_typeName")
                    .and_then(|v| match v {
                        miniserde::json::Value::String(s) => Some(s.as_str()),
                        _ => None,
                    })
                    .ok_or(miniserde::Error)?;

                let type_info = lookup_type(type_name).ok_or(miniserde::Error)?;

                // Create unified builder and replay buffered fields to it
                let mut builder = type_info.create_poly_builder()?;
                for (key, value) in buffer.iter() {
                    if key == "_typeName" {
                        continue;
                    }
                    let visitor = builder.key(key)?;
                    super::mini_helpers::replay_value_to_visitor(value, visitor)?;
                }
                builder.finish_poly()
            }
        }
    }
}

pub struct VimObjectHolderBuilder<'a> {
    pub core: PolyCore,
    pub __out: &'a mut Option<VimObjectHolder>,
}

impl<'a> VimObjectHolderBuilder<'a> {
    pub fn new(out: &'a mut Option<VimObjectHolder>) -> Self {
        Self {
            core: PolyCore::new(),
            __out: out,
        }
    }
}

// Note: The Map implementation for VimObjectHolderBuilder must be in the generated file
// because it needs access to lookup_type.

pub struct VimAnyBuilder<'a> {
    pub core: PolyCore,
    pub __out: &'a mut Option<VimAny>,
}

impl<'a> VimAnyBuilder<'a> {
    pub fn new(out: &'a mut Option<VimAny>) -> Self {
        Self {
            core: PolyCore::new(),
            __out: out,
        }
    }
}

// Note: The Map implementation for VimAnyBuilder must be in the generated file
// because it needs access to lookup_type.
