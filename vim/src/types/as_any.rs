use std::any;

/// Casts trait object to Any. This is the first step in casting between trait objects.
/// The second step is to downcast Any to the actual data type inside trait specific code.
/// See the implementations of `CastFrom` for an example.  
pub trait AsAny {
    /// Cast a reference to a trait object.
    fn as_any_ref<'a>(&'a self) -> &'a dyn any::Any;

    /// Cast to a boxed reference to a trait object.
    fn as_any_box(self: Box<Self>) -> Box<dyn any::Any>;

    /// Get the underlying type identifier.
    fn type_id(&self) -> any::TypeId {
        self.as_any_ref().type_id()
    }
}

/// A trait to cast a trait object to Any. This is the first step in casting between traits
impl AsAny for dyn any::Any {
    fn as_any_ref<'a>(&'a self) -> &'a dyn any::Any {
        self
    }

    fn as_any_box(self: Box<Self>) -> Box<dyn any::Any> {
        self
    }
}

/// A trait to cast a trait object to Any. This is the first step in casting between traits
/// objects.
impl<T> AsAny for T
where
    T: Sized + 'static,
{
    /// Cast a reference to Any trait.
    fn as_any_ref<'a>(&'a self) -> &'a dyn any::Any {
        self
    }

    /// Cast to a boxed reference to Any trait.
    fn as_any_box(self: Box<Self>) -> Box<dyn any::Any> {
        self
    }
}