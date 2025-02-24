use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The VirtualDeviceSpec data object type encapsulates change
/// specifications for an individual virtual device.
/// 
/// The virtual
/// device being added or modified must be fully specified.
pub trait VirtualDeviceConfigSpecTrait : super::data_object_trait::DataObjectTrait {
    /// Type of operation being performed on the specified virtual device.
    /// 
    /// If no operation is specified, the spec. is ignored.
    fn get_operation(&self) -> &Option<super::enums::VirtualDeviceConfigSpecOperationEnum>;
    /// Type of operation being performed on the backing
    /// of the specified virtual device.
    /// 
    /// If no file operation is specified in the VirtualDeviceSpec,
    /// then any backing filenames in the
    /// *VirtualDevice*
    /// must refer to files that already exist.
    /// The "replace" and "delete" values for this property are only
    /// applicable to virtual disk backing files.
    fn get_file_operation(&self) -> &Option<super::enums::VirtualDeviceConfigSpecFileOperationEnum>;
    /// Device specification, with all necessary properties set.
    fn get_device(&self) -> &Box<dyn super::virtual_device_trait::VirtualDeviceTrait>;
    /// Virtual Device Profile requirement.
    /// 
    /// Profiles are solution specifics.
    /// Storage Profile Based Management(SPBM) is a vSphere server extension.
    /// The API users who want to provision VMs using Storage Profiles, need to
    /// interact with SPBM service.
    /// This is an optional parameter and if user doesn't specify profile,
    /// the default behavior will apply.
    fn get_profile(&self) -> &Option<Vec<Box<dyn super::virtual_machine_profile_spec_trait::VirtualMachineProfileSpecTrait>>>;
    /// BackingInfo configuration options.
    /// 
    /// Each BackingSpec corresponds to a BackingInfo object. The member
    /// *VirtualDeviceConfigSpec.backing* refers to the
    /// *VirtualDeviceConfigSpec.device*.*VirtualDevice.backing*.
    fn get_backing(&self) -> &Option<VirtualDeviceConfigSpecBackingSpec>;
    /// List of independent filters *VirtualMachineIndependentFilterSpec*
    /// to configure on the virtual device.
    /// 
    /// ***Since:*** vSphere API Release 7.0.2.1
    fn get_filter_spec(&self) -> &Option<Vec<Box<dyn super::virtual_machine_base_independent_filter_spec_trait::VirtualMachineBaseIndependentFilterSpecTrait>>>;
    /// The change mode of the device.
    /// 
    /// The values of the mode will be one of *VirtualDeviceConfigSpecChangeMode_enum* enumerations.
    /// On unset, default to 'fail'.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    fn get_change_mode(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn VirtualDeviceConfigSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualDeviceConfigSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualDeviceConfigSpecVisitor)
            }
        }

struct VirtualDeviceConfigSpecVisitor;

impl<'de> de::Visitor<'de> for VirtualDeviceConfigSpecVisitor {
    type Value = Box<dyn VirtualDeviceConfigSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualDeviceConfigSpecTrait JSON object with a _typeName field")
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

impl VirtualDeviceConfigSpecTrait for VirtualDeviceConfigSpec {
    fn get_operation(&self) -> &Option<super::enums::VirtualDeviceConfigSpecOperationEnum> { &self.operation }
    fn get_file_operation(&self) -> &Option<super::enums::VirtualDeviceConfigSpecFileOperationEnum> { &self.file_operation }
    fn get_device(&self) -> &Box<dyn super::virtual_device_trait::VirtualDeviceTrait> { &self.device }
    fn get_profile(&self) -> &Option<Vec<Box<dyn super::virtual_machine_profile_spec_trait::VirtualMachineProfileSpecTrait>>> { &self.profile }
    fn get_backing(&self) -> &Option<VirtualDeviceConfigSpecBackingSpec> { &self.backing }
    fn get_filter_spec(&self) -> &Option<Vec<Box<dyn super::virtual_machine_base_independent_filter_spec_trait::VirtualMachineBaseIndependentFilterSpecTrait>>> { &self.filter_spec }
    fn get_change_mode(&self) -> &Option<String> { &self.change_mode }
}
impl VirtualDeviceConfigSpecTrait for VirtualDiskConfigSpec {
    fn get_operation(&self) -> &Option<super::enums::VirtualDeviceConfigSpecOperationEnum> { &self.operation }
    fn get_file_operation(&self) -> &Option<super::enums::VirtualDeviceConfigSpecFileOperationEnum> { &self.file_operation }
    fn get_device(&self) -> &Box<dyn super::virtual_device_trait::VirtualDeviceTrait> { &self.device }
    fn get_profile(&self) -> &Option<Vec<Box<dyn super::virtual_machine_profile_spec_trait::VirtualMachineProfileSpecTrait>>> { &self.profile }
    fn get_backing(&self) -> &Option<VirtualDeviceConfigSpecBackingSpec> { &self.backing }
    fn get_filter_spec(&self) -> &Option<Vec<Box<dyn super::virtual_machine_base_independent_filter_spec_trait::VirtualMachineBaseIndependentFilterSpecTrait>>> { &self.filter_spec }
    fn get_change_mode(&self) -> &Option<String> { &self.change_mode }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualDeviceConfigSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceConfigSpec => Some(from.as_any_ref().downcast_ref::<VirtualDeviceConfigSpec>()?),
            StructType::VirtualDiskConfigSpec => Some(from.as_any_ref().downcast_ref::<VirtualDiskConfigSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceConfigSpec => Ok(from.as_any_box().downcast::<VirtualDeviceConfigSpec>()?),
            StructType::VirtualDiskConfigSpec => Ok(from.as_any_box().downcast::<VirtualDiskConfigSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
