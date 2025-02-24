use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Changes to apply to the compute resource configuration.
pub trait ComputeResourceConfigSpecTrait : super::data_object_trait::DataObjectTrait {
    /// New setting for the swapfile placement policy.
    /// 
    /// Any change to this
    /// policy will affect virtual machines that subsequently power on or
    /// resume from a suspended state in this compute resource, or that
    /// migrate to a host in this compute resource while powered on; virtual
    /// machines that are currently powered on in this compute resource will
    /// not yet be affected.
    /// 
    /// See also *VirtualMachineConfigInfoSwapPlacementType_enum*.
    fn get_vm_swap_placement(&self) -> &Option<String>;
    /// Flag indicating whether or not the SPBM(Storage Policy Based Management)
    /// feature is enabled on this compute resource
    fn get_spbm_enabled(&self) -> Option<bool>;
    /// Key for Default Hardware Version to be used on this compute resource
    /// in the format of *VirtualMachineConfigOptionDescriptor.key*.
    /// 
    /// Setting this field affects
    /// *VirtualMachineConfigOptionDescriptor.defaultConfigOption* returned
    /// by *ComputeResource.environmentBrowser* of this object and all its children
    /// with this field unset.
    fn get_default_hardware_version_key(&self) -> &Option<String>;
    /// Desired software spec for the set of physical compute resources.
    /// 
    /// This
    /// parameter is only supported in vim.Folder#createClusterEx operation.
    fn get_desired_software_spec(&self) -> &Option<DesiredSoftwareSpec>;
    /// Key for Maximum Hardware Version to be used on this compute resource
    /// in the format of *VirtualMachineConfigOptionDescriptor.key*.
    /// 
    /// Setting this field affects
    /// *VirtualMachineConfigOptionDescriptor.defaultConfigOption* returned
    /// by *ComputeResource.environmentBrowser* of this object and all its children
    /// with this field unset.
    /// 
    /// ***Since:*** vSphere API Release 7.0.2.0
    fn get_maximum_hardware_version_key(&self) -> &Option<String>;
    /// Flag indicating whether or not the vLCM (vSphere Lifecycle Manager)
    /// Config Manager feature is enabled on this compute resource.
    /// 
    /// If the
    /// flag is not set, the Config Manager feature will be disabled by
    /// default. This parameter is only supported in *Folder.CreateClusterEx*
    /// operation.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.1
    fn get_enable_config_manager(&self) -> Option<bool>;
}
impl<'s> serde::Serialize for dyn ComputeResourceConfigSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ComputeResourceConfigSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ComputeResourceConfigSpecVisitor)
            }
        }

struct ComputeResourceConfigSpecVisitor;

impl<'de> de::Visitor<'de> for ComputeResourceConfigSpecVisitor {
    type Value = Box<dyn ComputeResourceConfigSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ComputeResourceConfigSpecTrait JSON object with a _typeName field")
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

impl ComputeResourceConfigSpecTrait for ComputeResourceConfigSpec {
    fn get_vm_swap_placement(&self) -> &Option<String> { &self.vm_swap_placement }
    fn get_spbm_enabled(&self) -> Option<bool> { self.spbm_enabled }
    fn get_default_hardware_version_key(&self) -> &Option<String> { &self.default_hardware_version_key }
    fn get_desired_software_spec(&self) -> &Option<DesiredSoftwareSpec> { &self.desired_software_spec }
    fn get_maximum_hardware_version_key(&self) -> &Option<String> { &self.maximum_hardware_version_key }
    fn get_enable_config_manager(&self) -> Option<bool> { self.enable_config_manager }
}
impl ComputeResourceConfigSpecTrait for ClusterConfigSpecEx {
    fn get_vm_swap_placement(&self) -> &Option<String> { &self.vm_swap_placement }
    fn get_spbm_enabled(&self) -> Option<bool> { self.spbm_enabled }
    fn get_default_hardware_version_key(&self) -> &Option<String> { &self.default_hardware_version_key }
    fn get_desired_software_spec(&self) -> &Option<DesiredSoftwareSpec> { &self.desired_software_spec }
    fn get_maximum_hardware_version_key(&self) -> &Option<String> { &self.maximum_hardware_version_key }
    fn get_enable_config_manager(&self) -> Option<bool> { self.enable_config_manager }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ComputeResourceConfigSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ComputeResourceConfigSpec => Some(from.as_any_ref().downcast_ref::<ComputeResourceConfigSpec>()?),
            StructType::ClusterConfigSpecEx => Some(from.as_any_ref().downcast_ref::<ClusterConfigSpecEx>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ComputeResourceConfigSpec => Ok(from.as_any_box().downcast::<ComputeResourceConfigSpec>()?),
            StructType::ClusterConfigSpecEx => Ok(from.as_any_box().downcast::<ClusterConfigSpecEx>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
