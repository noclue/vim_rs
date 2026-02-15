// Polymorphic serialization is handled via VimObjectTrait: miniserde::Serialize supertrait.
// Each concrete type implements miniserde::Serialize and vtable dispatch handles the rest.
