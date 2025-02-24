use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Deprecated as of vSphere API 6.5 use
/// *FeatureRequirementsNotMet*.
/// 
/// The host is not compatible with the CPU feature requirements of the
/// virtual machine, for a particular CPUID register.
/// 
/// A subclass of this fault
/// may be used to express the incompatibilities in a more easily
/// understandable format.
pub trait CpuIncompatibleTrait : super::virtual_hardware_compatibility_issue_trait::VirtualHardwareCompatibilityIssueTrait {
    /// The CpuIdInfo level where a problem was detected.
    /// 
    /// Other levels may
    /// also have problems.
    fn get_level(&self) -> i32;
    /// The CpuIdInfo register where a problem was detected.
    /// 
    /// Other registers
    /// may also have problems. Possible register names are eax, ebx, ecx, or edx.
    fn get_register_name(&self) -> &str;
    /// The contents of the register on the target host, in CpuIdInfo register
    /// format.
    /// 
    /// The '-' character indicates an unknown value.
    fn get_register_bits(&self) -> &Option<String>;
    /// The desired values for the register's bits.
    /// 
    /// The 'x' character indicates
    /// don't-care.
    fn get_desired_bits(&self) -> &Option<String>;
    /// The host that is not compatible with the requirements.
    /// 
    /// Refers instance of *HostSystem*.
    fn get_host(&self) -> &Option<ManagedObjectReference>;
}
impl<'s> serde::Serialize for dyn CpuIncompatibleTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn CpuIncompatibleTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(CpuIncompatibleVisitor)
            }
        }

struct CpuIncompatibleVisitor;

impl<'de> de::Visitor<'de> for CpuIncompatibleVisitor {
    type Value = Box<dyn CpuIncompatibleTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid CpuIncompatibleTrait JSON object with a _typeName field")
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

impl CpuIncompatibleTrait for CpuIncompatible {
    fn get_level(&self) -> i32 { self.level }
    fn get_register_name(&self) -> &str { &self.register_name }
    fn get_register_bits(&self) -> &Option<String> { &self.register_bits }
    fn get_desired_bits(&self) -> &Option<String> { &self.desired_bits }
    fn get_host(&self) -> &Option<ManagedObjectReference> { &self.host }
}
impl CpuIncompatibleTrait for CpuCompatibilityUnknown {
    fn get_level(&self) -> i32 { self.level }
    fn get_register_name(&self) -> &str { &self.register_name }
    fn get_register_bits(&self) -> &Option<String> { &self.register_bits }
    fn get_desired_bits(&self) -> &Option<String> { &self.desired_bits }
    fn get_host(&self) -> &Option<ManagedObjectReference> { &self.host }
}
impl CpuIncompatibleTrait for CpuIncompatible1Ecx {
    fn get_level(&self) -> i32 { self.level }
    fn get_register_name(&self) -> &str { &self.register_name }
    fn get_register_bits(&self) -> &Option<String> { &self.register_bits }
    fn get_desired_bits(&self) -> &Option<String> { &self.desired_bits }
    fn get_host(&self) -> &Option<ManagedObjectReference> { &self.host }
}
impl CpuIncompatibleTrait for CpuIncompatible81Edx {
    fn get_level(&self) -> i32 { self.level }
    fn get_register_name(&self) -> &str { &self.register_name }
    fn get_register_bits(&self) -> &Option<String> { &self.register_bits }
    fn get_desired_bits(&self) -> &Option<String> { &self.desired_bits }
    fn get_host(&self) -> &Option<ManagedObjectReference> { &self.host }
}
impl CpuIncompatibleTrait for FaultToleranceCpuIncompatible {
    fn get_level(&self) -> i32 { self.level }
    fn get_register_name(&self) -> &str { &self.register_name }
    fn get_register_bits(&self) -> &Option<String> { &self.register_bits }
    fn get_desired_bits(&self) -> &Option<String> { &self.desired_bits }
    fn get_host(&self) -> &Option<ManagedObjectReference> { &self.host }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn CpuIncompatibleTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::CpuIncompatible => Some(from.as_any_ref().downcast_ref::<CpuIncompatible>()?),
            StructType::CpuCompatibilityUnknown => Some(from.as_any_ref().downcast_ref::<CpuCompatibilityUnknown>()?),
            StructType::CpuIncompatible1Ecx => Some(from.as_any_ref().downcast_ref::<CpuIncompatible1Ecx>()?),
            StructType::CpuIncompatible81Edx => Some(from.as_any_ref().downcast_ref::<CpuIncompatible81Edx>()?),
            StructType::FaultToleranceCpuIncompatible => Some(from.as_any_ref().downcast_ref::<FaultToleranceCpuIncompatible>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::CpuIncompatible => Ok(from.as_any_box().downcast::<CpuIncompatible>()?),
            StructType::CpuCompatibilityUnknown => Ok(from.as_any_box().downcast::<CpuCompatibilityUnknown>()?),
            StructType::CpuIncompatible1Ecx => Ok(from.as_any_box().downcast::<CpuIncompatible1Ecx>()?),
            StructType::CpuIncompatible81Edx => Ok(from.as_any_box().downcast::<CpuIncompatible81Edx>()?),
            StructType::FaultToleranceCpuIncompatible => Ok(from.as_any_box().downcast::<FaultToleranceCpuIncompatible>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
