use std::any;
/// Casts one trait to another. For example:
///
/// Example: `let virtual_device = <dyn VirtualDeviceTrait>::from_ref(data_object_trait_ref)?`;
pub trait CastFrom<From: ?Sized> {
    /// Casts a reference to a trait object. If the cast fails, [`std::option::Option::None`] is
    /// returned.
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self>;
    /// Casts a boxed trait object to another trait object. If the cast fails, the original boxed 
    /// trait object is returned in [`std::result::Result::Err`].
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn any::Any>>;
}

/// Casts one trait to another. For example:
///
/// Example: `let virtual_device: &dyn VirtualDeviceTrait = data_object_trait_ref.into_ref()?`; 
pub trait CastInto<To: ?Sized> {
    /// Casts a reference to a trait object. If the cast fails, [`std::option::Option::None`] is
    /// returned.
    fn into_ref<'a>(self: &'a Self) -> Option<&'a To>;
    /// Casts a boxed trait object to another trait object. If the cast fails, the original boxed
    /// trait object is returned in [`std::result::Result::Err`].
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

