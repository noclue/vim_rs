use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base fault that occurs when an operation conflicts with a resource configuration
/// policy.
/// 
/// For example, this fault occurs if a power-on operation reserves more memory
/// than is allocated to a resource pool.
pub trait InsufficientResourcesFaultTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn InsufficientResourcesFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn InsufficientResourcesFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(InsufficientResourcesFaultVisitor)
            }
        }

struct InsufficientResourcesFaultVisitor;

impl<'de> de::Visitor<'de> for InsufficientResourcesFaultVisitor {
    type Value = Box<dyn InsufficientResourcesFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid InsufficientResourcesFaultTrait JSON object with a _typeName field")
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

impl InsufficientResourcesFaultTrait for InsufficientResourcesFault {
}
impl InsufficientResourcesFaultTrait for InsufficientAgentVmsDeployed {
}
impl InsufficientResourcesFaultTrait for InsufficientCpuResourcesFault {
}
impl InsufficientResourcesFaultTrait for InsufficientFailoverResourcesFault {
}
impl InsufficientResourcesFaultTrait for InsufficientGraphicsResourcesFault {
}
impl InsufficientResourcesFaultTrait for InsufficientHostCapacityFault {
}
impl InsufficientResourcesFaultTrait for InsufficientHostCpuCapacityFault {
}
impl InsufficientResourcesFaultTrait for InsufficientHostMemoryCapacityFault {
}
impl InsufficientResourcesFaultTrait for InsufficientPerCpuCapacity {
}
impl InsufficientResourcesFaultTrait for InsufficientMemoryResourcesFault {
}
impl InsufficientResourcesFaultTrait for InsufficientNetworkCapacity {
}
impl InsufficientResourcesFaultTrait for InsufficientNetworkResourcePoolCapacity {
}
impl InsufficientResourcesFaultTrait for InsufficientStandbyResource {
}
impl InsufficientResourcesFaultTrait for InsufficientStandbyCpuResource {
}
impl InsufficientResourcesFaultTrait for InsufficientStandbyMemoryResource {
}
impl InsufficientResourcesFaultTrait for InsufficientStorageSpace {
}
impl InsufficientResourcesFaultTrait for InsufficientVFlashResourcesFault {
}
impl InsufficientResourcesFaultTrait for InvalidResourcePoolStructureFault {
}
impl InsufficientResourcesFaultTrait for NumVirtualCpusExceedsLimit {
}
impl InsufficientResourcesFaultTrait for VmFaultToleranceTooManyFtVcpusOnHost {
}
impl InsufficientResourcesFaultTrait for VmFaultToleranceTooManyVMsOnHost {
}
impl InsufficientResourcesFaultTrait for VmSmpFaultToleranceTooManyVMsOnHost {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn InsufficientResourcesFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::InsufficientResourcesFault => Some(from.as_any_ref().downcast_ref::<InsufficientResourcesFault>()?),
            StructType::InsufficientAgentVmsDeployed => Some(from.as_any_ref().downcast_ref::<InsufficientAgentVmsDeployed>()?),
            StructType::InsufficientCpuResourcesFault => Some(from.as_any_ref().downcast_ref::<InsufficientCpuResourcesFault>()?),
            StructType::InsufficientFailoverResourcesFault => Some(from.as_any_ref().downcast_ref::<InsufficientFailoverResourcesFault>()?),
            StructType::InsufficientGraphicsResourcesFault => Some(from.as_any_ref().downcast_ref::<InsufficientGraphicsResourcesFault>()?),
            StructType::InsufficientHostCapacityFault => Some(from.as_any_ref().downcast_ref::<InsufficientHostCapacityFault>()?),
            StructType::InsufficientHostCpuCapacityFault => Some(from.as_any_ref().downcast_ref::<InsufficientHostCpuCapacityFault>()?),
            StructType::InsufficientHostMemoryCapacityFault => Some(from.as_any_ref().downcast_ref::<InsufficientHostMemoryCapacityFault>()?),
            StructType::InsufficientPerCpuCapacity => Some(from.as_any_ref().downcast_ref::<InsufficientPerCpuCapacity>()?),
            StructType::InsufficientMemoryResourcesFault => Some(from.as_any_ref().downcast_ref::<InsufficientMemoryResourcesFault>()?),
            StructType::InsufficientNetworkCapacity => Some(from.as_any_ref().downcast_ref::<InsufficientNetworkCapacity>()?),
            StructType::InsufficientNetworkResourcePoolCapacity => Some(from.as_any_ref().downcast_ref::<InsufficientNetworkResourcePoolCapacity>()?),
            StructType::InsufficientStandbyResource => Some(from.as_any_ref().downcast_ref::<InsufficientStandbyResource>()?),
            StructType::InsufficientStandbyCpuResource => Some(from.as_any_ref().downcast_ref::<InsufficientStandbyCpuResource>()?),
            StructType::InsufficientStandbyMemoryResource => Some(from.as_any_ref().downcast_ref::<InsufficientStandbyMemoryResource>()?),
            StructType::InsufficientStorageSpace => Some(from.as_any_ref().downcast_ref::<InsufficientStorageSpace>()?),
            StructType::InsufficientVFlashResourcesFault => Some(from.as_any_ref().downcast_ref::<InsufficientVFlashResourcesFault>()?),
            StructType::InvalidResourcePoolStructureFault => Some(from.as_any_ref().downcast_ref::<InvalidResourcePoolStructureFault>()?),
            StructType::NumVirtualCpusExceedsLimit => Some(from.as_any_ref().downcast_ref::<NumVirtualCpusExceedsLimit>()?),
            StructType::VmFaultToleranceTooManyFtVcpusOnHost => Some(from.as_any_ref().downcast_ref::<VmFaultToleranceTooManyFtVcpusOnHost>()?),
            StructType::VmFaultToleranceTooManyVMsOnHost => Some(from.as_any_ref().downcast_ref::<VmFaultToleranceTooManyVMsOnHost>()?),
            StructType::VmSmpFaultToleranceTooManyVMsOnHost => Some(from.as_any_ref().downcast_ref::<VmSmpFaultToleranceTooManyVMsOnHost>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::InsufficientResourcesFault => Ok(from.as_any_box().downcast::<InsufficientResourcesFault>()?),
            StructType::InsufficientAgentVmsDeployed => Ok(from.as_any_box().downcast::<InsufficientAgentVmsDeployed>()?),
            StructType::InsufficientCpuResourcesFault => Ok(from.as_any_box().downcast::<InsufficientCpuResourcesFault>()?),
            StructType::InsufficientFailoverResourcesFault => Ok(from.as_any_box().downcast::<InsufficientFailoverResourcesFault>()?),
            StructType::InsufficientGraphicsResourcesFault => Ok(from.as_any_box().downcast::<InsufficientGraphicsResourcesFault>()?),
            StructType::InsufficientHostCapacityFault => Ok(from.as_any_box().downcast::<InsufficientHostCapacityFault>()?),
            StructType::InsufficientHostCpuCapacityFault => Ok(from.as_any_box().downcast::<InsufficientHostCpuCapacityFault>()?),
            StructType::InsufficientHostMemoryCapacityFault => Ok(from.as_any_box().downcast::<InsufficientHostMemoryCapacityFault>()?),
            StructType::InsufficientPerCpuCapacity => Ok(from.as_any_box().downcast::<InsufficientPerCpuCapacity>()?),
            StructType::InsufficientMemoryResourcesFault => Ok(from.as_any_box().downcast::<InsufficientMemoryResourcesFault>()?),
            StructType::InsufficientNetworkCapacity => Ok(from.as_any_box().downcast::<InsufficientNetworkCapacity>()?),
            StructType::InsufficientNetworkResourcePoolCapacity => Ok(from.as_any_box().downcast::<InsufficientNetworkResourcePoolCapacity>()?),
            StructType::InsufficientStandbyResource => Ok(from.as_any_box().downcast::<InsufficientStandbyResource>()?),
            StructType::InsufficientStandbyCpuResource => Ok(from.as_any_box().downcast::<InsufficientStandbyCpuResource>()?),
            StructType::InsufficientStandbyMemoryResource => Ok(from.as_any_box().downcast::<InsufficientStandbyMemoryResource>()?),
            StructType::InsufficientStorageSpace => Ok(from.as_any_box().downcast::<InsufficientStorageSpace>()?),
            StructType::InsufficientVFlashResourcesFault => Ok(from.as_any_box().downcast::<InsufficientVFlashResourcesFault>()?),
            StructType::InvalidResourcePoolStructureFault => Ok(from.as_any_box().downcast::<InvalidResourcePoolStructureFault>()?),
            StructType::NumVirtualCpusExceedsLimit => Ok(from.as_any_box().downcast::<NumVirtualCpusExceedsLimit>()?),
            StructType::VmFaultToleranceTooManyFtVcpusOnHost => Ok(from.as_any_box().downcast::<VmFaultToleranceTooManyFtVcpusOnHost>()?),
            StructType::VmFaultToleranceTooManyVMsOnHost => Ok(from.as_any_box().downcast::<VmFaultToleranceTooManyVMsOnHost>()?),
            StructType::VmSmpFaultToleranceTooManyVMsOnHost => Ok(from.as_any_box().downcast::<VmSmpFaultToleranceTooManyVMsOnHost>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
