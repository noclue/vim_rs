use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// VM Configuration.
pub trait VmConfigInfoTrait : super::data_object_trait::DataObjectTrait {
    /// Information about the package content.
    fn get_product(&self) -> &Option<Vec<VAppProductInfo>>;
    /// List of properties
    fn get_property(&self) -> &Option<Vec<VAppPropertyInfo>>;
    /// IP assignment policy and DHCP support configuration.
    fn get_ip_assignment(&self) -> &VAppIpAssignmentInfo;
    /// End User Liceses Agreements.
    fn get_eula(&self) -> &Option<Vec<String>>;
    /// List of uninterpreted OVF meta-data sections.
    fn get_ovf_section(&self) -> &Option<Vec<VAppOvfSectionInfo>>;
    /// List the transports to use for properties.
    /// 
    /// Supported values are: iso and
    /// com.vmware.guestInfo.
    fn get_ovf_environment_transport(&self) -> &Option<Vec<String>>;
    /// Specifies whether the VM needs an initial boot before the deployment is complete.
    /// 
    /// Not relevant for vApps. This means that the value is always false when reading the
    /// configuration and is ignored when setting the configuration.
    /// 
    /// If a vApp requires an install boot (because one of its VMs does), this is visible
    /// on the *VirtualAppSummary.installBootRequired* field of the vApp.
    fn get_install_boot_required(&self) -> bool;
    /// Specifies the delay in seconds to wait for the VM to power off after the initial
    /// boot (used only if installBootRequired is true).
    /// 
    /// A value of 0 means wait forever.
    /// 
    /// Not relevant for vApps. This means that the value is always false when reading the
    /// configuration and is ignored when setting the configuration.
    fn get_install_boot_stop_delay(&self) -> i32;
}
impl<'s> serde::Serialize for dyn VmConfigInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VmConfigInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VmConfigInfoVisitor)
            }
        }

struct VmConfigInfoVisitor;

impl<'de> de::Visitor<'de> for VmConfigInfoVisitor {
    type Value = Box<dyn VmConfigInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VmConfigInfoTrait JSON object with a _typeName field")
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

impl VmConfigInfoTrait for VmConfigInfo {
    fn get_product(&self) -> &Option<Vec<VAppProductInfo>> { &self.product }
    fn get_property(&self) -> &Option<Vec<VAppPropertyInfo>> { &self.property }
    fn get_ip_assignment(&self) -> &VAppIpAssignmentInfo { &self.ip_assignment }
    fn get_eula(&self) -> &Option<Vec<String>> { &self.eula }
    fn get_ovf_section(&self) -> &Option<Vec<VAppOvfSectionInfo>> { &self.ovf_section }
    fn get_ovf_environment_transport(&self) -> &Option<Vec<String>> { &self.ovf_environment_transport }
    fn get_install_boot_required(&self) -> bool { self.install_boot_required }
    fn get_install_boot_stop_delay(&self) -> i32 { self.install_boot_stop_delay }
}
impl VmConfigInfoTrait for VAppConfigInfo {
    fn get_product(&self) -> &Option<Vec<VAppProductInfo>> { &self.product }
    fn get_property(&self) -> &Option<Vec<VAppPropertyInfo>> { &self.property }
    fn get_ip_assignment(&self) -> &VAppIpAssignmentInfo { &self.ip_assignment }
    fn get_eula(&self) -> &Option<Vec<String>> { &self.eula }
    fn get_ovf_section(&self) -> &Option<Vec<VAppOvfSectionInfo>> { &self.ovf_section }
    fn get_ovf_environment_transport(&self) -> &Option<Vec<String>> { &self.ovf_environment_transport }
    fn get_install_boot_required(&self) -> bool { self.install_boot_required }
    fn get_install_boot_stop_delay(&self) -> i32 { self.install_boot_stop_delay }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VmConfigInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmConfigInfo => Some(from.as_any_ref().downcast_ref::<VmConfigInfo>()?),
            StructType::VAppConfigInfo => Some(from.as_any_ref().downcast_ref::<VAppConfigInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmConfigInfo => Ok(from.as_any_box().downcast::<VmConfigInfo>()?),
            StructType::VAppConfigInfo => Ok(from.as_any_box().downcast::<VAppConfigInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
