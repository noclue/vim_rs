use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Configuration of the compute resource; applies to both standalone hosts
/// and clusters.
pub trait ComputeResourceConfigInfoTrait : super::data_object_trait::DataObjectTrait {
    /// Swapfile placement policy for virtual machines within this compute
    /// resource.
    /// 
    /// Any policy except for "inherit" is a valid value for this
    /// property; the default is "vmDirectory". This setting will be honored
    /// for each virtual machine within the compute resource for which the
    /// following is true:
    /// - The virtual machine is executing on a host that has the
    ///   *perVmSwapFiles* capability.
    /// - The virtual machine configuration's
    ///   *swapPlacement* property is set
    ///   to "inherit".
    ///   
    /// See also *VirtualMachineConfigInfoSwapPlacementType_enum*.
    fn get_vm_swap_placement(&self) -> &str;
    /// Flag indicating whether or not the SPBM(Storage Policy Based Management)
    /// feature is enabled on this compute resource
    fn get_spbm_enabled(&self) -> Option<bool>;
    /// Key for Default Hardware Version used on this compute resource
    /// in the format of *VirtualMachineConfigOptionDescriptor.key*.
    /// 
    /// This field affects
    /// *VirtualMachineConfigOptionDescriptor.defaultConfigOption* returned
    /// by *ComputeResource.environmentBrowser* of this object and all its children
    /// with this field unset.
    fn get_default_hardware_version_key(&self) -> &Option<String>;
    /// Key for Maximum Hardware Version used on this compute resource
    /// in the format of *VirtualMachineConfigOptionDescriptor.key*.
    /// 
    /// This field affects
    /// *VirtualMachineConfigOptionDescriptor.defaultConfigOption* returned
    /// by *ComputeResource.environmentBrowser* of this object and all its children
    /// with this field unset.
    /// 
    /// ***Since:*** vSphere API Release 7.0.2.0
    fn get_maximum_hardware_version_key(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn ComputeResourceConfigInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ComputeResourceConfigInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ComputeResourceConfigInfoVisitor)
            }
        }

struct ComputeResourceConfigInfoVisitor;

impl<'de> de::Visitor<'de> for ComputeResourceConfigInfoVisitor {
    type Value = Box<dyn ComputeResourceConfigInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ComputeResourceConfigInfoTrait JSON object with a _typeName field")
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

impl ComputeResourceConfigInfoTrait for ComputeResourceConfigInfo {
    fn get_vm_swap_placement(&self) -> &str { &self.vm_swap_placement }
    fn get_spbm_enabled(&self) -> Option<bool> { self.spbm_enabled }
    fn get_default_hardware_version_key(&self) -> &Option<String> { &self.default_hardware_version_key }
    fn get_maximum_hardware_version_key(&self) -> &Option<String> { &self.maximum_hardware_version_key }
}
impl ComputeResourceConfigInfoTrait for ClusterConfigInfoEx {
    fn get_vm_swap_placement(&self) -> &str { &self.vm_swap_placement }
    fn get_spbm_enabled(&self) -> Option<bool> { self.spbm_enabled }
    fn get_default_hardware_version_key(&self) -> &Option<String> { &self.default_hardware_version_key }
    fn get_maximum_hardware_version_key(&self) -> &Option<String> { &self.maximum_hardware_version_key }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ComputeResourceConfigInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ComputeResourceConfigInfo => Some(from.as_any_ref().downcast_ref::<ComputeResourceConfigInfo>()?),
            StructType::ClusterConfigInfoEx => Some(from.as_any_ref().downcast_ref::<ClusterConfigInfoEx>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ComputeResourceConfigInfo => Ok(from.as_any_box().downcast::<ComputeResourceConfigInfo>()?),
            StructType::ClusterConfigInfoEx => Ok(from.as_any_box().downcast::<ClusterConfigInfoEx>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
