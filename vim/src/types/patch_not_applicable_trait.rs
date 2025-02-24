use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This fault is thrown if a patch install fails because the patch is not
/// applicable to the host.
/// 
/// Typically, a subclass of this exception is
/// thrown, indicating a problem such as the patch is superseded, already
/// installed, or has dependencies missing, and so on.
pub trait PatchNotApplicableTrait : super::vim_fault_trait::VimFaultTrait {
    /// The ID of the patch that is not applicable to the host.
    fn get_patch_id(&self) -> &str;
}
impl<'s> serde::Serialize for dyn PatchNotApplicableTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn PatchNotApplicableTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(PatchNotApplicableVisitor)
            }
        }

struct PatchNotApplicableVisitor;

impl<'de> de::Visitor<'de> for PatchNotApplicableVisitor {
    type Value = Box<dyn PatchNotApplicableTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid PatchNotApplicableTrait JSON object with a _typeName field")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let deserializer = de::value::MapAccessDeserializer::new(&mut map);
        let any: VimAny = de::Deserialize::deserialize(deserializer)?;
        match any {
            VimAny::Object(obj) => Ok(CastFrom::from_box(obj)
                .map_err(|_| de::Error::custom("Internal error converting to trait type"))?),
            VimAny::Value(value) => Err(de::Error::custom(format!(
                "expected object not wrapped value: {:?}",
                value))),
        }
    }
}

impl PatchNotApplicableTrait for PatchNotApplicable {
    fn get_patch_id(&self) -> &str { &self.patch_id }
}
impl PatchNotApplicableTrait for PatchAlreadyInstalled {
    fn get_patch_id(&self) -> &str { &self.patch_id }
}
impl PatchNotApplicableTrait for PatchMissingDependencies {
    fn get_patch_id(&self) -> &str { &self.patch_id }
}
impl PatchNotApplicableTrait for PatchSuperseded {
    fn get_patch_id(&self) -> &str { &self.patch_id }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn PatchNotApplicableTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::PatchNotApplicable => Some(from.as_any_ref().downcast_ref::<PatchNotApplicable>()?),
            StructType::PatchAlreadyInstalled => Some(from.as_any_ref().downcast_ref::<PatchAlreadyInstalled>()?),
            StructType::PatchMissingDependencies => Some(from.as_any_ref().downcast_ref::<PatchMissingDependencies>()?),
            StructType::PatchSuperseded => Some(from.as_any_ref().downcast_ref::<PatchSuperseded>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::PatchNotApplicable => Ok(from.as_any_box().downcast::<PatchNotApplicable>()?),
            StructType::PatchAlreadyInstalled => Ok(from.as_any_box().downcast::<PatchAlreadyInstalled>()?),
            StructType::PatchMissingDependencies => Ok(from.as_any_box().downcast::<PatchMissingDependencies>()?),
            StructType::PatchSuperseded => Ok(from.as_any_box().downcast::<PatchSuperseded>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
