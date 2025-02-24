use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// vApp related configuration of a VM.
pub trait VmConfigSpecTrait : super::data_object_trait::DataObjectTrait {
    /// Information about the product.
    /// 
    /// Reconfigure privilege: VApp.ApplicationConfig
    fn get_product(&self) -> &Option<Vec<VAppProductSpec>>;
    /// List of properties.
    /// 
    /// Adding and editing properties requires various privileges depending on which fields
    /// are affected. See *VAppPropertyInfo* for details.
    /// 
    /// Deleting properties requires the privilege VApp.ApplicationConfig.
    fn get_property(&self) -> &Option<Vec<VAppPropertySpec>>;
    /// IP assignment policy and DHCP support configuration.
    /// 
    /// Reconfigure privilege: See *VAppIPAssignmentInfo*
    fn get_ip_assignment(&self) -> &Option<VAppIpAssignmentInfo>;
    /// End User Liceses Agreements.
    /// 
    /// If this list is set, it replaces all exiting licenses. An empty list will not
    /// make any changes to installed licenses. A list with a single element {""} will
    /// remove all licenses and leave an empty list.
    /// 
    /// Reconfigure privilege: VApp.ApplicationConfig
    fn get_eula(&self) -> &Option<Vec<String>>;
    /// List of uninterpreted OVF meta-data sections.
    /// 
    /// Reconfigure privilege: VApp.ApplicationConfig
    fn get_ovf_section(&self) -> &Option<Vec<VAppOvfSectionSpec>>;
    /// List the transports to use for properties.
    /// 
    /// Supported values are: iso and
    /// com.vmware.guestInfo.
    /// 
    /// If this list is set, it replaces all exiting entries. An empty list will not make
    /// any changes. A list with a single element {""} will clear the list of transports.
    /// 
    /// Reconfigure privilege: VApp.ApplicationConfig
    fn get_ovf_environment_transport(&self) -> &Option<Vec<String>>;
    /// If this is on a VirtualMachine object, it specifies whether the VM needs an
    /// initial boot before the deployment is complete.
    /// 
    /// If this is on a vApp object,
    /// it indicates than one or more VMs needs an initial reboot. This flag is
    /// automatically reset once the reboot has happened.
    /// 
    /// Reconfigure privilege: VApp.ApplicationConfig
    fn get_install_boot_required(&self) -> Option<bool>;
    /// Specifies the delay in seconds to wait for the VM to power off after the initial
    /// boot (used only if installBootRequired is true).
    /// 
    /// A value of 0 means wait forever.
    /// 
    /// Reconfigure privilege: VApp.ApplicationConfig
    fn get_install_boot_stop_delay(&self) -> Option<i32>;
}
impl<'s> serde::Serialize for dyn VmConfigSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VmConfigSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VmConfigSpecVisitor)
            }
        }

struct VmConfigSpecVisitor;

impl<'de> de::Visitor<'de> for VmConfigSpecVisitor {
    type Value = Box<dyn VmConfigSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VmConfigSpecTrait JSON object with a _typeName field")
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

impl VmConfigSpecTrait for VmConfigSpec {
    fn get_product(&self) -> &Option<Vec<VAppProductSpec>> { &self.product }
    fn get_property(&self) -> &Option<Vec<VAppPropertySpec>> { &self.property }
    fn get_ip_assignment(&self) -> &Option<VAppIpAssignmentInfo> { &self.ip_assignment }
    fn get_eula(&self) -> &Option<Vec<String>> { &self.eula }
    fn get_ovf_section(&self) -> &Option<Vec<VAppOvfSectionSpec>> { &self.ovf_section }
    fn get_ovf_environment_transport(&self) -> &Option<Vec<String>> { &self.ovf_environment_transport }
    fn get_install_boot_required(&self) -> Option<bool> { self.install_boot_required }
    fn get_install_boot_stop_delay(&self) -> Option<i32> { self.install_boot_stop_delay }
}
impl VmConfigSpecTrait for VAppConfigSpec {
    fn get_product(&self) -> &Option<Vec<VAppProductSpec>> { &self.product }
    fn get_property(&self) -> &Option<Vec<VAppPropertySpec>> { &self.property }
    fn get_ip_assignment(&self) -> &Option<VAppIpAssignmentInfo> { &self.ip_assignment }
    fn get_eula(&self) -> &Option<Vec<String>> { &self.eula }
    fn get_ovf_section(&self) -> &Option<Vec<VAppOvfSectionSpec>> { &self.ovf_section }
    fn get_ovf_environment_transport(&self) -> &Option<Vec<String>> { &self.ovf_environment_transport }
    fn get_install_boot_required(&self) -> Option<bool> { self.install_boot_required }
    fn get_install_boot_stop_delay(&self) -> Option<i32> { self.install_boot_stop_delay }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VmConfigSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmConfigSpec => Some(from.as_any_ref().downcast_ref::<VmConfigSpec>()?),
            StructType::VAppConfigSpec => Some(from.as_any_ref().downcast_ref::<VAppConfigSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmConfigSpec => Ok(from.as_any_box().downcast::<VmConfigSpec>()?),
            StructType::VAppConfigSpec => Ok(from.as_any_box().downcast::<VAppConfigSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
