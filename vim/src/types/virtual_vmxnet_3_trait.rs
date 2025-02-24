use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The VirtualVmxnet3 data object type represents an instance
/// of the Vmxnet3 virtual Ethernet adapter attached to a virtual machine.
pub trait VirtualVmxnet3Trait : super::virtual_vmxnet_trait::VirtualVmxnetTrait {
    /// Indicates whether UPTv2(Uniform Pass-through version 2) compatibility is
    /// enabled on this network adapter.
    /// 
    /// UPTv2 is only available on Vmxnet3
    /// adapter. Clients can set this property enabled or disabled if ethernet
    /// virtual device is Vmxnet3. It requires the VM hardware version is
    /// compatible with ESXi version which has enabled smartnic feature.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    fn get_uptv_2_enabled(&self) -> Option<bool>;
}
impl<'s> serde::Serialize for dyn VirtualVmxnet3Trait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualVmxnet3Trait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualVmxnet3Visitor)
            }
        }

struct VirtualVmxnet3Visitor;

impl<'de> de::Visitor<'de> for VirtualVmxnet3Visitor {
    type Value = Box<dyn VirtualVmxnet3Trait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualVmxnet3Trait JSON object with a _typeName field")
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

impl VirtualVmxnet3Trait for VirtualVmxnet3 {
    fn get_uptv_2_enabled(&self) -> Option<bool> { self.uptv_2_enabled }
}
impl VirtualVmxnet3Trait for VirtualVmxnet3Vrdma {
    fn get_uptv_2_enabled(&self) -> Option<bool> { self.uptv_2_enabled }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualVmxnet3Trait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualVmxnet3 => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet3>()?),
            StructType::VirtualVmxnet3Vrdma => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet3Vrdma>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualVmxnet3 => Ok(from.as_any_box().downcast::<VirtualVmxnet3>()?),
            StructType::VirtualVmxnet3Vrdma => Ok(from.as_any_box().downcast::<VirtualVmxnet3Vrdma>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
