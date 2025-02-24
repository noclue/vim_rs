use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// An ImportSpec is used when importing VMs or vApps.
/// 
/// It can be built from scratch, or it can be generated from an OVF descriptor using the
/// service interface *OvfManager*.
/// 
/// This class is the abstract base for *VirtualMachineImportSpec* and
/// *VirtualAppImportSpec*. These three classes form a composite structure
/// that allows us to contain arbitrarily complex entitites in a single ImportSpec.
pub trait ImportSpecTrait : super::data_object_trait::DataObjectTrait {
    /// Configuration of sub-entities (virtual machine or vApp).
    /// 
    /// This is used for
    /// sub-entities of a vApp that could be a virtual machine or a vApp.
    fn get_entity_config(&self) -> &Option<VAppEntityConfigInfo>;
    /// The instantiation OST (see *OvfConsumer* ) to be consumed by OVF
    /// consumers.
    fn get_instantiation_ost(&self) -> &Option<OvfConsumerOstNode>;
}
impl<'s> serde::Serialize for dyn ImportSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ImportSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ImportSpecVisitor)
            }
        }

struct ImportSpecVisitor;

impl<'de> de::Visitor<'de> for ImportSpecVisitor {
    type Value = Box<dyn ImportSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ImportSpecTrait JSON object with a _typeName field")
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

impl ImportSpecTrait for ImportSpec {
    fn get_entity_config(&self) -> &Option<VAppEntityConfigInfo> { &self.entity_config }
    fn get_instantiation_ost(&self) -> &Option<OvfConsumerOstNode> { &self.instantiation_ost }
}
impl ImportSpecTrait for VirtualAppImportSpec {
    fn get_entity_config(&self) -> &Option<VAppEntityConfigInfo> { &self.entity_config }
    fn get_instantiation_ost(&self) -> &Option<OvfConsumerOstNode> { &self.instantiation_ost }
}
impl ImportSpecTrait for VirtualMachineImportSpec {
    fn get_entity_config(&self) -> &Option<VAppEntityConfigInfo> { &self.entity_config }
    fn get_instantiation_ost(&self) -> &Option<OvfConsumerOstNode> { &self.instantiation_ost }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ImportSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ImportSpec => Some(from.as_any_ref().downcast_ref::<ImportSpec>()?),
            StructType::VirtualAppImportSpec => Some(from.as_any_ref().downcast_ref::<VirtualAppImportSpec>()?),
            StructType::VirtualMachineImportSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineImportSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ImportSpec => Ok(from.as_any_box().downcast::<ImportSpec>()?),
            StructType::VirtualAppImportSpec => Ok(from.as_any_box().downcast::<VirtualAppImportSpec>()?),
            StructType::VirtualMachineImportSpec => Ok(from.as_any_box().downcast::<VirtualMachineImportSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
