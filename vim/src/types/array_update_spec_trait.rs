use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// An ArrayUpdateSpec data object type is a common superclass
/// for supporting incremental updates to arrays.
/// 
/// The common code pattern is:
/// 
///          class MyTypeSpec extrends ArrayUpdateSpec {
///                MyTypeInfo info;
///          }
/// The ArrayUpdateSpec contains the following:
/// - **operation**: the type of operation being performed.
/// - **removeKey**: In the case of a remove operation, the
///   key value that identifies the array to be removed.
pub trait ArrayUpdateSpecTrait : super::data_object_trait::DataObjectTrait {
    /// The type of operation being performed on the specified virtual device.
    fn get_operation(&self) -> &super::enums::ArrayUpdateOperationEnum;
    /// Key for the element to be removed.
    /// 
    /// Only used if the operation
    /// is "remove".
    fn get_remove_key(&self) -> &Option<VimAny>;
}
impl<'s> serde::Serialize for dyn ArrayUpdateSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ArrayUpdateSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ArrayUpdateSpecVisitor)
            }
        }

struct ArrayUpdateSpecVisitor;

impl<'de> de::Visitor<'de> for ArrayUpdateSpecVisitor {
    type Value = Box<dyn ArrayUpdateSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ArrayUpdateSpecTrait JSON object with a _typeName field")
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

impl ArrayUpdateSpecTrait for ArrayUpdateSpec {
    fn get_operation(&self) -> &super::enums::ArrayUpdateOperationEnum { &self.operation }
    fn get_remove_key(&self) -> &Option<VimAny> { &self.remove_key }
}
impl ArrayUpdateSpecTrait for ClusterDasVmConfigSpec {
    fn get_operation(&self) -> &super::enums::ArrayUpdateOperationEnum { &self.operation }
    fn get_remove_key(&self) -> &Option<VimAny> { &self.remove_key }
}
impl ArrayUpdateSpecTrait for ClusterDatastoreUpdateSpec {
    fn get_operation(&self) -> &super::enums::ArrayUpdateOperationEnum { &self.operation }
    fn get_remove_key(&self) -> &Option<VimAny> { &self.remove_key }
}
impl ArrayUpdateSpecTrait for ClusterDpmHostConfigSpec {
    fn get_operation(&self) -> &super::enums::ArrayUpdateOperationEnum { &self.operation }
    fn get_remove_key(&self) -> &Option<VimAny> { &self.remove_key }
}
impl ArrayUpdateSpecTrait for ClusterDrsVmConfigSpec {
    fn get_operation(&self) -> &super::enums::ArrayUpdateOperationEnum { &self.operation }
    fn get_remove_key(&self) -> &Option<VimAny> { &self.remove_key }
}
impl ArrayUpdateSpecTrait for ClusterGroupSpec {
    fn get_operation(&self) -> &super::enums::ArrayUpdateOperationEnum { &self.operation }
    fn get_remove_key(&self) -> &Option<VimAny> { &self.remove_key }
}
impl ArrayUpdateSpecTrait for ClusterPreemptibleVmPairSpec {
    fn get_operation(&self) -> &super::enums::ArrayUpdateOperationEnum { &self.operation }
    fn get_remove_key(&self) -> &Option<VimAny> { &self.remove_key }
}
impl ArrayUpdateSpecTrait for ClusterRuleSpec {
    fn get_operation(&self) -> &super::enums::ArrayUpdateOperationEnum { &self.operation }
    fn get_remove_key(&self) -> &Option<VimAny> { &self.remove_key }
}
impl ArrayUpdateSpecTrait for ClusterTagCategoryUpdateSpec {
    fn get_operation(&self) -> &super::enums::ArrayUpdateOperationEnum { &self.operation }
    fn get_remove_key(&self) -> &Option<VimAny> { &self.remove_key }
}
impl ArrayUpdateSpecTrait for ClusterVmOrchestrationSpec {
    fn get_operation(&self) -> &super::enums::ArrayUpdateOperationEnum { &self.operation }
    fn get_remove_key(&self) -> &Option<VimAny> { &self.remove_key }
}
impl ArrayUpdateSpecTrait for StorageDrsOptionSpec {
    fn get_operation(&self) -> &super::enums::ArrayUpdateOperationEnum { &self.operation }
    fn get_remove_key(&self) -> &Option<VimAny> { &self.remove_key }
}
impl ArrayUpdateSpecTrait for StorageDrsVmConfigSpec {
    fn get_operation(&self) -> &super::enums::ArrayUpdateOperationEnum { &self.operation }
    fn get_remove_key(&self) -> &Option<VimAny> { &self.remove_key }
}
impl ArrayUpdateSpecTrait for VAppOvfSectionSpec {
    fn get_operation(&self) -> &super::enums::ArrayUpdateOperationEnum { &self.operation }
    fn get_remove_key(&self) -> &Option<VimAny> { &self.remove_key }
}
impl ArrayUpdateSpecTrait for VAppProductSpec {
    fn get_operation(&self) -> &super::enums::ArrayUpdateOperationEnum { &self.operation }
    fn get_remove_key(&self) -> &Option<VimAny> { &self.remove_key }
}
impl ArrayUpdateSpecTrait for VAppPropertySpec {
    fn get_operation(&self) -> &super::enums::ArrayUpdateOperationEnum { &self.operation }
    fn get_remove_key(&self) -> &Option<VimAny> { &self.remove_key }
}
impl ArrayUpdateSpecTrait for VirtualMachineCpuIdInfoSpec {
    fn get_operation(&self) -> &super::enums::ArrayUpdateOperationEnum { &self.operation }
    fn get_remove_key(&self) -> &Option<VimAny> { &self.remove_key }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ArrayUpdateSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ArrayUpdateSpec => Some(from.as_any_ref().downcast_ref::<ArrayUpdateSpec>()?),
            StructType::ClusterDasVmConfigSpec => Some(from.as_any_ref().downcast_ref::<ClusterDasVmConfigSpec>()?),
            StructType::ClusterDatastoreUpdateSpec => Some(from.as_any_ref().downcast_ref::<ClusterDatastoreUpdateSpec>()?),
            StructType::ClusterDpmHostConfigSpec => Some(from.as_any_ref().downcast_ref::<ClusterDpmHostConfigSpec>()?),
            StructType::ClusterDrsVmConfigSpec => Some(from.as_any_ref().downcast_ref::<ClusterDrsVmConfigSpec>()?),
            StructType::ClusterGroupSpec => Some(from.as_any_ref().downcast_ref::<ClusterGroupSpec>()?),
            StructType::ClusterPreemptibleVmPairSpec => Some(from.as_any_ref().downcast_ref::<ClusterPreemptibleVmPairSpec>()?),
            StructType::ClusterRuleSpec => Some(from.as_any_ref().downcast_ref::<ClusterRuleSpec>()?),
            StructType::ClusterTagCategoryUpdateSpec => Some(from.as_any_ref().downcast_ref::<ClusterTagCategoryUpdateSpec>()?),
            StructType::ClusterVmOrchestrationSpec => Some(from.as_any_ref().downcast_ref::<ClusterVmOrchestrationSpec>()?),
            StructType::StorageDrsOptionSpec => Some(from.as_any_ref().downcast_ref::<StorageDrsOptionSpec>()?),
            StructType::StorageDrsVmConfigSpec => Some(from.as_any_ref().downcast_ref::<StorageDrsVmConfigSpec>()?),
            StructType::VAppOvfSectionSpec => Some(from.as_any_ref().downcast_ref::<VAppOvfSectionSpec>()?),
            StructType::VAppProductSpec => Some(from.as_any_ref().downcast_ref::<VAppProductSpec>()?),
            StructType::VAppPropertySpec => Some(from.as_any_ref().downcast_ref::<VAppPropertySpec>()?),
            StructType::VirtualMachineCpuIdInfoSpec => Some(from.as_any_ref().downcast_ref::<VirtualMachineCpuIdInfoSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ArrayUpdateSpec => Ok(from.as_any_box().downcast::<ArrayUpdateSpec>()?),
            StructType::ClusterDasVmConfigSpec => Ok(from.as_any_box().downcast::<ClusterDasVmConfigSpec>()?),
            StructType::ClusterDatastoreUpdateSpec => Ok(from.as_any_box().downcast::<ClusterDatastoreUpdateSpec>()?),
            StructType::ClusterDpmHostConfigSpec => Ok(from.as_any_box().downcast::<ClusterDpmHostConfigSpec>()?),
            StructType::ClusterDrsVmConfigSpec => Ok(from.as_any_box().downcast::<ClusterDrsVmConfigSpec>()?),
            StructType::ClusterGroupSpec => Ok(from.as_any_box().downcast::<ClusterGroupSpec>()?),
            StructType::ClusterPreemptibleVmPairSpec => Ok(from.as_any_box().downcast::<ClusterPreemptibleVmPairSpec>()?),
            StructType::ClusterRuleSpec => Ok(from.as_any_box().downcast::<ClusterRuleSpec>()?),
            StructType::ClusterTagCategoryUpdateSpec => Ok(from.as_any_box().downcast::<ClusterTagCategoryUpdateSpec>()?),
            StructType::ClusterVmOrchestrationSpec => Ok(from.as_any_box().downcast::<ClusterVmOrchestrationSpec>()?),
            StructType::StorageDrsOptionSpec => Ok(from.as_any_box().downcast::<StorageDrsOptionSpec>()?),
            StructType::StorageDrsVmConfigSpec => Ok(from.as_any_box().downcast::<StorageDrsVmConfigSpec>()?),
            StructType::VAppOvfSectionSpec => Ok(from.as_any_box().downcast::<VAppOvfSectionSpec>()?),
            StructType::VAppProductSpec => Ok(from.as_any_box().downcast::<VAppProductSpec>()?),
            StructType::VAppPropertySpec => Ok(from.as_any_box().downcast::<VAppPropertySpec>()?),
            StructType::VirtualMachineCpuIdInfoSpec => Ok(from.as_any_box().downcast::<VirtualMachineCpuIdInfoSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
