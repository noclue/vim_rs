use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type describes the DNS configuration.
/// 
/// All IPv4 addresses, subnet addresses, and netmasks are specified using
/// dotted decimal notation. For example, "192.0.2.1".
/// IPv6 addresses are 128-bit addresses represented as
/// eight fields of up to four hexadecimal digits.
/// A colon separates each field (:). For example,
/// 2001:DB8:101::230:6eff:fe04:d9ff. The address can also consist of the
/// symbol '::' to represent multiple 16-bit groups of
/// contiguous 0's only once in an address as described in RFC 2373.
pub trait HostDnsConfigTrait : super::data_object_trait::DataObjectTrait {
    /// The flag to indicate whether or not DHCP (dynamic host control
    /// protocol) is used to determine DNS configuration automatically.
    fn get_dhcp(&self) -> bool;
    /// If DHCP is enabled, the DHCP DNS of the vmkernel nic will override
    /// the system's DNS.
    /// 
    /// This field applies to both IPv4 and IPv6 DNS settings
    /// if *ipv6VirtualNicDevice*
    /// is unset, otherwise it is applicable only for IPv4 setting.
    /// This field is ignored if DHCP is disabled by the
    /// *dhcp* property.
    fn get_virtual_nic_device(&self) -> &Option<String>;
    /// If DHCP is enabled, the IPv6 DHCP DNS of the vmkernel nic will override
    /// the system's IPv6 DNS.
    /// 
    /// This field is ignored if DHCP is disabled by the
    /// *dhcp* property.
    fn get_ipv_6_virtual_nic_device(&self) -> &Option<String>;
    /// The host name portion of DNS name.
    /// 
    /// For example, "esx01".
    /// 
    /// **Note**: When DHCP is not enabled, the property can be set
    /// explicitly. When DHCP is enabled, the property reflects the current
    /// DNS configuration, but cannot be set.
    /// The hostName can't have character '.' in it when set explicitly.
    fn get_host_name(&self) -> &str;
    /// The domain name portion of the DNS name.
    /// 
    /// For example, "vmware.com".
    /// 
    /// **Note**: When DHCP is not enabled, the property can be set
    /// explicitly. When DHCP is enabled, the property reflects the current
    /// DNS configuration, but cannot be set.
    fn get_domain_name(&self) -> &str;
    /// The IP addresses of the DNS servers, placed in order of preference.
    /// 
    /// **Note**: When DHCP is not enabled, the property can be set
    /// explicitly. When DHCP is enabled, the property reflects the current
    /// DNS configuration, but cannot be set.
    fn get_address(&self) -> &Option<Vec<String>>;
    /// The domain in which to search for hosts, placed in order of preference.
    /// 
    /// **Note**: When DHCP is not enabled, the property can be set
    /// explicitly. When DHCP is enabled, the property reflects the current
    /// DNS configuration, but cannot be set.
    fn get_search_domain(&self) -> &Option<Vec<String>>;
}
impl<'s> serde::Serialize for dyn HostDnsConfigTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostDnsConfigTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostDnsConfigVisitor)
            }
        }

struct HostDnsConfigVisitor;

impl<'de> de::Visitor<'de> for HostDnsConfigVisitor {
    type Value = Box<dyn HostDnsConfigTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostDnsConfigTrait JSON object with a _typeName field")
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

impl HostDnsConfigTrait for HostDnsConfig {
    fn get_dhcp(&self) -> bool { self.dhcp }
    fn get_virtual_nic_device(&self) -> &Option<String> { &self.virtual_nic_device }
    fn get_ipv_6_virtual_nic_device(&self) -> &Option<String> { &self.ipv_6_virtual_nic_device }
    fn get_host_name(&self) -> &str { &self.host_name }
    fn get_domain_name(&self) -> &str { &self.domain_name }
    fn get_address(&self) -> &Option<Vec<String>> { &self.address }
    fn get_search_domain(&self) -> &Option<Vec<String>> { &self.search_domain }
}
impl HostDnsConfigTrait for HostDnsConfigSpec {
    fn get_dhcp(&self) -> bool { self.dhcp }
    fn get_virtual_nic_device(&self) -> &Option<String> { &self.virtual_nic_device }
    fn get_ipv_6_virtual_nic_device(&self) -> &Option<String> { &self.ipv_6_virtual_nic_device }
    fn get_host_name(&self) -> &str { &self.host_name }
    fn get_domain_name(&self) -> &str { &self.domain_name }
    fn get_address(&self) -> &Option<Vec<String>> { &self.address }
    fn get_search_domain(&self) -> &Option<Vec<String>> { &self.search_domain }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostDnsConfigTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostDnsConfig => Some(from.as_any_ref().downcast_ref::<HostDnsConfig>()?),
            StructType::HostDnsConfigSpec => Some(from.as_any_ref().downcast_ref::<HostDnsConfigSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostDnsConfig => Ok(from.as_any_box().downcast::<HostDnsConfig>()?),
            StructType::HostDnsConfigSpec => Ok(from.as_any_box().downcast::<HostDnsConfigSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
