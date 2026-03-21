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
        match key {
            // `_typeName` is already consumed by PolyCore before ValuePolyBuilder is created.
            // `_value` is the JSON wrapper key; `#text` is the XML text-content key.
            // Any other child-element key (e.g. `"string"` in `ArrayOfString`) is also
            // forwarded so the underlying deserializer can handle sequences via `seq()`.
            "_typeName" => Ok(<dyn miniserde::de::Visitor>::ignore()),
            _ => Ok(self.deserializer.as_mut()),
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
    pub default_type_name: Option<&'static str>,
}

// Note: Deserialize and Visitor impls for VimObjectHolder are in the generated
// deserialize.rs because they need the Place type from the make_place! macro.

// ============================================================================
// Shared Polymorphic State Machine
// ============================================================================

/// Polymorphic builder mode - shared between VimObjectHolder and VimAny builders.
///
/// Two-state machine: `Initial` until the type is resolved, then `Direct`.
/// Type resolution happens via:
///   1. `_typeName` key (JSON `_typeName` field or XML `xsi:type` attribute)
///   2. `default_type_name` (statically known from the schema, e.g. `deviceInfo`
///      is always `Description` unless overridden by `xsi:type`)
///
/// If neither is available when a non-`_typeName` key arrives, deserialization
/// fails. This matches both protocols: vCenter JSON always emits `_typeName`
/// first, and ESXi XML has `xsi:type` as an attribute (delivered before children).
pub enum PolyMode {
    Initial,
    Direct { builder: Box<dyn PolyBuilder> },
}

/// Core polymorphic state machine. Handles key() logic for both builders.
pub struct PolyCore {
    pub mode: PolyMode,
    pub type_name: Option<String>,
    pub default_type_name: Option<&'static str>,
}

impl PolyCore {
    pub fn new() -> Self {
        Self {
            mode: PolyMode::Initial,
            type_name: None,
            default_type_name: None,
        }
    }

    pub fn with_default(default_type_name: &'static str) -> Self {
        Self {
            mode: PolyMode::Initial,
            type_name: None,
            default_type_name: Some(default_type_name),
        }
    }

    /// Transition from Initial to Direct using a resolved type name.
    fn resolve<F>(&mut self, name: &str, lookup_type: &F) -> miniserde::Result<()>
    where
        F: Fn(&str) -> Option<&'static TypeInfo>,
    {
        let type_info = lookup_type(name).ok_or(miniserde::Error)?;
        self.mode = PolyMode::Direct {
            builder: type_info.create_poly_builder()?,
        };
        Ok(())
    }

    /// Handle a key. Returns the visitor for the value.
    pub fn key<F>(
        &mut self,
        key: &str,
        lookup_type: F,
    ) -> miniserde::Result<&mut dyn miniserde::de::Visitor>
    where
        F: Fn(&str) -> Option<&'static TypeInfo>,
    {
        // Ensure we're in Direct mode before delegating
        if matches!(self.mode, PolyMode::Initial) {
            if let Some(type_name) = self.type_name.take() {
                // _typeName was captured on a previous key() call — resolve it now
                self.resolve(&type_name, &lookup_type)?;
            } else if key == "_typeName" {
                // _typeName is the current key — capture its value, stay Initial
                return Ok(miniserde::de::Deserialize::begin(&mut self.type_name));
            } else if let Some(default) = self.default_type_name {
                // Non-_typeName key, no captured type — use schema default
                self.resolve(default, &lookup_type)?;
            } else {
                return Err(miniserde::Error);
            }
        }

        match &mut self.mode {
            PolyMode::Direct { builder } => builder.key(key),
            PolyMode::Initial => unreachable!(),
        }
    }

    /// Finish: resolve type if needed, then finalize the builder.
    pub fn finish<F>(&mut self, lookup_type: F) -> miniserde::Result<VimAny>
    where
        F: Fn(&str) -> Option<&'static TypeInfo>,
    {
        // If _typeName was the only key (e.g. `{"_typeName":"Foo"}`), resolve now
        if matches!(&self.mode, PolyMode::Initial) {
            if let Some(type_name) = self.type_name.take() {
                self.resolve(&type_name, &lookup_type)?;
            } else if let Some(default) = self.default_type_name {
                self.resolve(default, &lookup_type)?;
            }
        }

        match &mut self.mode {
            PolyMode::Initial => Err(miniserde::Error),
            PolyMode::Direct { builder } => builder.finish_poly(),
        }
    }
}

pub struct VimObjectHolderBuilder<'a> {
    pub core: PolyCore,
    pub __out: &'a mut Option<VimObjectHolder>,
}

impl<'a> VimObjectHolderBuilder<'a> {
    pub fn new(out: &'a mut Option<VimObjectHolder>) -> Self {
        let default = out.as_ref().and_then(|h| h.default_type_name);
        Self {
            core: if let Some(dt) = default {
                PolyCore::with_default(dt)
            } else {
                PolyCore::new()
            },
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
