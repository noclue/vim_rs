use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *ApplyProfile* data object is the base class for all data objects
/// that define profile configuration data.
/// 
/// <code>ApplyProfile</code> defines ESX configuration data storage and it
/// supports recursive profile definition for the profile plug-in architecture.
pub trait ApplyProfileTrait : super::data_object_trait::DataObjectTrait {
    /// Indicates whether the profile is enabled.
    fn get_enabled(&self) -> bool;
    /// The list of policies comprising the profile.
    /// 
    /// A *ProfilePolicy*
    /// stores one or more configuration data values in a *PolicyOption*.
    /// The policy option is one of the configuration options from the
    /// *ProfilePolicyMetadata*.*ProfilePolicyMetadata.possibleOption*
    /// list.
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>>;
    /// Identifies the profile type.
    fn get_profile_type_name(&self) -> &Option<String>;
    /// Profile engine version.
    fn get_profile_version(&self) -> &Option<String>;
    /// List of subprofiles for this profile.
    /// 
    /// This list can change depending on which profile plug-ins are available in the system.
    /// Subprofiles can be nested to arbitrary depths to represent host capabilities.
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>>;
    /// Indicates whether this profile is marked as "favorite".
    fn get_favorite(&self) -> Option<bool>;
    /// Indicates whether this profile is marked as to-be-merged.
    fn get_to_be_merged(&self) -> Option<bool>;
    /// Indicates whether the selected array elements, with the current
    /// as one of them, replace the profile array in the target host
    /// profile.
    fn get_to_replace_with(&self) -> Option<bool>;
    /// Indicates whether this profile is marked as to-be-deleted.
    fn get_to_be_deleted(&self) -> Option<bool>;
    /// Indicates that the member variable <code>enabled</code> of this profile
    /// will be copied from source profile to target profiles at host profile
    /// composition.
    fn get_copy_enable_status(&self) -> Option<bool>;
    /// Indicates whether this profile will be displayed or not.
    fn get_hidden(&self) -> Option<bool>;
}
impl<'s> serde::Serialize for dyn ApplyProfileTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ApplyProfileTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ApplyProfileVisitor)
            }
        }

struct ApplyProfileVisitor;

impl<'de> de::Visitor<'de> for ApplyProfileVisitor {
    type Value = Box<dyn ApplyProfileTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ApplyProfileTrait JSON object with a _typeName field")
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

impl ApplyProfileTrait for ApplyProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for ProfileApplyProfileElement {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for ActiveDirectoryProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for AuthenticationProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for DateTimeProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for DvsProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for DvsVNicProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for DvsHostVNicProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for DvsServiceConsoleVNicProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for FirewallProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for FirewallProfileRulesetProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for HostApplyProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for HostMemoryProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for IpAddressProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for IpRouteProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for NasStorageProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for NetStackInstanceProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for NetworkPolicyProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for NetworkProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for NetworkProfileDnsConfigProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for NsxHostVNicProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for OpaqueSwitchProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for OptionProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for PermissionProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for PhysicalNicProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for PnicUplinkProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for PortGroupProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for HostPortGroupProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for ServiceConsolePortGroupProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for VmPortGroupProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for VirtualSwitchSelectionProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for VlanProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for SecurityProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for ServiceProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for StaticRouteProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for StorageProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for UserGroupProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for UserProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for VirtualSwitchProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for LinkProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl ApplyProfileTrait for NumPortsProfile {
    fn get_enabled(&self) -> bool { self.enabled }
    fn get_policy(&self) -> &Option<Vec<ProfilePolicy>> { &self.policy }
    fn get_profile_type_name(&self) -> &Option<String> { &self.profile_type_name }
    fn get_profile_version(&self) -> &Option<String> { &self.profile_version }
    fn get_property(&self) -> &Option<Vec<ProfileApplyProfileProperty>> { &self.property }
    fn get_favorite(&self) -> Option<bool> { self.favorite }
    fn get_to_be_merged(&self) -> Option<bool> { self.to_be_merged }
    fn get_to_replace_with(&self) -> Option<bool> { self.to_replace_with }
    fn get_to_be_deleted(&self) -> Option<bool> { self.to_be_deleted }
    fn get_copy_enable_status(&self) -> Option<bool> { self.copy_enable_status }
    fn get_hidden(&self) -> Option<bool> { self.hidden }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ApplyProfileTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ApplyProfile => Some(from.as_any_ref().downcast_ref::<ApplyProfile>()?),
            StructType::ProfileApplyProfileElement => Some(from.as_any_ref().downcast_ref::<ProfileApplyProfileElement>()?),
            StructType::ActiveDirectoryProfile => Some(from.as_any_ref().downcast_ref::<ActiveDirectoryProfile>()?),
            StructType::AuthenticationProfile => Some(from.as_any_ref().downcast_ref::<AuthenticationProfile>()?),
            StructType::DateTimeProfile => Some(from.as_any_ref().downcast_ref::<DateTimeProfile>()?),
            StructType::DvsProfile => Some(from.as_any_ref().downcast_ref::<DvsProfile>()?),
            StructType::DvsVNicProfile => Some(from.as_any_ref().downcast_ref::<DvsVNicProfile>()?),
            StructType::DvsHostVNicProfile => Some(from.as_any_ref().downcast_ref::<DvsHostVNicProfile>()?),
            StructType::DvsServiceConsoleVNicProfile => Some(from.as_any_ref().downcast_ref::<DvsServiceConsoleVNicProfile>()?),
            StructType::FirewallProfile => Some(from.as_any_ref().downcast_ref::<FirewallProfile>()?),
            StructType::FirewallProfileRulesetProfile => Some(from.as_any_ref().downcast_ref::<FirewallProfileRulesetProfile>()?),
            StructType::HostApplyProfile => Some(from.as_any_ref().downcast_ref::<HostApplyProfile>()?),
            StructType::HostMemoryProfile => Some(from.as_any_ref().downcast_ref::<HostMemoryProfile>()?),
            StructType::IpAddressProfile => Some(from.as_any_ref().downcast_ref::<IpAddressProfile>()?),
            StructType::IpRouteProfile => Some(from.as_any_ref().downcast_ref::<IpRouteProfile>()?),
            StructType::NasStorageProfile => Some(from.as_any_ref().downcast_ref::<NasStorageProfile>()?),
            StructType::NetStackInstanceProfile => Some(from.as_any_ref().downcast_ref::<NetStackInstanceProfile>()?),
            StructType::NetworkPolicyProfile => Some(from.as_any_ref().downcast_ref::<NetworkPolicyProfile>()?),
            StructType::NetworkProfile => Some(from.as_any_ref().downcast_ref::<NetworkProfile>()?),
            StructType::NetworkProfileDnsConfigProfile => Some(from.as_any_ref().downcast_ref::<NetworkProfileDnsConfigProfile>()?),
            StructType::NsxHostVNicProfile => Some(from.as_any_ref().downcast_ref::<NsxHostVNicProfile>()?),
            StructType::OpaqueSwitchProfile => Some(from.as_any_ref().downcast_ref::<OpaqueSwitchProfile>()?),
            StructType::OptionProfile => Some(from.as_any_ref().downcast_ref::<OptionProfile>()?),
            StructType::PermissionProfile => Some(from.as_any_ref().downcast_ref::<PermissionProfile>()?),
            StructType::PhysicalNicProfile => Some(from.as_any_ref().downcast_ref::<PhysicalNicProfile>()?),
            StructType::PnicUplinkProfile => Some(from.as_any_ref().downcast_ref::<PnicUplinkProfile>()?),
            StructType::PortGroupProfile => Some(from.as_any_ref().downcast_ref::<PortGroupProfile>()?),
            StructType::HostPortGroupProfile => Some(from.as_any_ref().downcast_ref::<HostPortGroupProfile>()?),
            StructType::ServiceConsolePortGroupProfile => Some(from.as_any_ref().downcast_ref::<ServiceConsolePortGroupProfile>()?),
            StructType::VmPortGroupProfile => Some(from.as_any_ref().downcast_ref::<VmPortGroupProfile>()?),
            StructType::VirtualSwitchSelectionProfile => Some(from.as_any_ref().downcast_ref::<VirtualSwitchSelectionProfile>()?),
            StructType::VlanProfile => Some(from.as_any_ref().downcast_ref::<VlanProfile>()?),
            StructType::SecurityProfile => Some(from.as_any_ref().downcast_ref::<SecurityProfile>()?),
            StructType::ServiceProfile => Some(from.as_any_ref().downcast_ref::<ServiceProfile>()?),
            StructType::StaticRouteProfile => Some(from.as_any_ref().downcast_ref::<StaticRouteProfile>()?),
            StructType::StorageProfile => Some(from.as_any_ref().downcast_ref::<StorageProfile>()?),
            StructType::UserGroupProfile => Some(from.as_any_ref().downcast_ref::<UserGroupProfile>()?),
            StructType::UserProfile => Some(from.as_any_ref().downcast_ref::<UserProfile>()?),
            StructType::VirtualSwitchProfile => Some(from.as_any_ref().downcast_ref::<VirtualSwitchProfile>()?),
            StructType::LinkProfile => Some(from.as_any_ref().downcast_ref::<LinkProfile>()?),
            StructType::NumPortsProfile => Some(from.as_any_ref().downcast_ref::<NumPortsProfile>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ApplyProfile => Ok(from.as_any_box().downcast::<ApplyProfile>()?),
            StructType::ProfileApplyProfileElement => Ok(from.as_any_box().downcast::<ProfileApplyProfileElement>()?),
            StructType::ActiveDirectoryProfile => Ok(from.as_any_box().downcast::<ActiveDirectoryProfile>()?),
            StructType::AuthenticationProfile => Ok(from.as_any_box().downcast::<AuthenticationProfile>()?),
            StructType::DateTimeProfile => Ok(from.as_any_box().downcast::<DateTimeProfile>()?),
            StructType::DvsProfile => Ok(from.as_any_box().downcast::<DvsProfile>()?),
            StructType::DvsVNicProfile => Ok(from.as_any_box().downcast::<DvsVNicProfile>()?),
            StructType::DvsHostVNicProfile => Ok(from.as_any_box().downcast::<DvsHostVNicProfile>()?),
            StructType::DvsServiceConsoleVNicProfile => Ok(from.as_any_box().downcast::<DvsServiceConsoleVNicProfile>()?),
            StructType::FirewallProfile => Ok(from.as_any_box().downcast::<FirewallProfile>()?),
            StructType::FirewallProfileRulesetProfile => Ok(from.as_any_box().downcast::<FirewallProfileRulesetProfile>()?),
            StructType::HostApplyProfile => Ok(from.as_any_box().downcast::<HostApplyProfile>()?),
            StructType::HostMemoryProfile => Ok(from.as_any_box().downcast::<HostMemoryProfile>()?),
            StructType::IpAddressProfile => Ok(from.as_any_box().downcast::<IpAddressProfile>()?),
            StructType::IpRouteProfile => Ok(from.as_any_box().downcast::<IpRouteProfile>()?),
            StructType::NasStorageProfile => Ok(from.as_any_box().downcast::<NasStorageProfile>()?),
            StructType::NetStackInstanceProfile => Ok(from.as_any_box().downcast::<NetStackInstanceProfile>()?),
            StructType::NetworkPolicyProfile => Ok(from.as_any_box().downcast::<NetworkPolicyProfile>()?),
            StructType::NetworkProfile => Ok(from.as_any_box().downcast::<NetworkProfile>()?),
            StructType::NetworkProfileDnsConfigProfile => Ok(from.as_any_box().downcast::<NetworkProfileDnsConfigProfile>()?),
            StructType::NsxHostVNicProfile => Ok(from.as_any_box().downcast::<NsxHostVNicProfile>()?),
            StructType::OpaqueSwitchProfile => Ok(from.as_any_box().downcast::<OpaqueSwitchProfile>()?),
            StructType::OptionProfile => Ok(from.as_any_box().downcast::<OptionProfile>()?),
            StructType::PermissionProfile => Ok(from.as_any_box().downcast::<PermissionProfile>()?),
            StructType::PhysicalNicProfile => Ok(from.as_any_box().downcast::<PhysicalNicProfile>()?),
            StructType::PnicUplinkProfile => Ok(from.as_any_box().downcast::<PnicUplinkProfile>()?),
            StructType::PortGroupProfile => Ok(from.as_any_box().downcast::<PortGroupProfile>()?),
            StructType::HostPortGroupProfile => Ok(from.as_any_box().downcast::<HostPortGroupProfile>()?),
            StructType::ServiceConsolePortGroupProfile => Ok(from.as_any_box().downcast::<ServiceConsolePortGroupProfile>()?),
            StructType::VmPortGroupProfile => Ok(from.as_any_box().downcast::<VmPortGroupProfile>()?),
            StructType::VirtualSwitchSelectionProfile => Ok(from.as_any_box().downcast::<VirtualSwitchSelectionProfile>()?),
            StructType::VlanProfile => Ok(from.as_any_box().downcast::<VlanProfile>()?),
            StructType::SecurityProfile => Ok(from.as_any_box().downcast::<SecurityProfile>()?),
            StructType::ServiceProfile => Ok(from.as_any_box().downcast::<ServiceProfile>()?),
            StructType::StaticRouteProfile => Ok(from.as_any_box().downcast::<StaticRouteProfile>()?),
            StructType::StorageProfile => Ok(from.as_any_box().downcast::<StorageProfile>()?),
            StructType::UserGroupProfile => Ok(from.as_any_box().downcast::<UserGroupProfile>()?),
            StructType::UserProfile => Ok(from.as_any_box().downcast::<UserProfile>()?),
            StructType::VirtualSwitchProfile => Ok(from.as_any_box().downcast::<VirtualSwitchProfile>()?),
            StructType::LinkProfile => Ok(from.as_any_box().downcast::<LinkProfile>()?),
            StructType::NumPortsProfile => Ok(from.as_any_box().downcast::<NumPortsProfile>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
