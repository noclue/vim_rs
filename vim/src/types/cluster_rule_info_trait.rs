use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *ClusterRuleInfo* data object is the base type for affinity
/// and anti-affinity rules.
/// 
/// The affinity and anti-affinity rules
/// are DRS (Distributed Resource Scheduling) rules that affect the placement
/// of virtual machines in a cluster. Hosts and virtual machines referenced
/// in a DRS rule must be in the same cluster.
/// 
/// Note: DRS rules are different than an individual host's CPU affinity rules
/// (*VirtualMachineAffinityInfo*).
/// 
/// The Server uses DRS rule objects to describe the current rule configuration
/// (*ClusterConfigInfoEx*.*ClusterConfigInfoEx.rule*).
/// Your client application uses rule objects to configure the affinity and
/// anti-affinity rules
/// (*ClusterConfigSpecEx*.*ClusterConfigSpecEx.rulesSpec*).
/// 
/// You can create the following types of rules:
/// - An affinity rule defines a set of virtual machines that should run
///   on the same host.
///   The *ClusterAffinityRuleSpec* object describes a rule that
///   identifies virtual machines, but does not identify any specific host.
/// - An anti-affinity rule defines a set of virtual machines that should run
///   on different hosts.
///   The *ClusterAntiAffinityRuleSpec* object describes a rule that
///   identifies virtual machines, but does not identify any specific host.
/// - A VM-Host rule defines affinity and anti-affinity relationships between
///   virtual machines and hosts.
///   The *ClusterVmHostRuleInfo* object describes a rule that identifies
///   a virtual machine group (*ClusterVmGroup*) and affinity and
///   anti-affinity host groups (*ClusterHostGroup*).
///   
/// Rule configuration is a dynamic process. When you create or modify a DRS rule,
/// the Server applies the rule to the cluster. If the existing cluster configuration
/// violates the rule, the Server attempts to correct the situation. If that is not
/// possible, the Server generates a fault and produces a log event.
/// DRS rules do not have precedence; all rules are applied equally.
/// DRS does not validate one rule against another. If you create conflicting
/// rules, the older rule takes precedence and DRS disables the newer rule.
/// 
/// Improperly used, DRS rules can fragment the cluster and inhibit the proper
/// functioning of DRS, HA, and DPM services. vSphere services never
/// take any actions that would result in the violation of mandatory DRS rules.
/// An operation that violates a mandatory rule would produce the following
/// consequences.
/// - DRS does not evacuate virtual machines to place a host in maintenance
///   mode.
/// - DRS does not place virtual machines for power-on or load balance virtual
///   machines.
/// - HA does not perform failovers.
/// - DPM does not optimize power management by placing hosts into standby
///   mode.
///   
/// To avoid these situations, exercise caution when creating more than one
/// mandatory rule, or consider using only optional rules. Make sure that
/// the number of hosts with which a virtual machine is related by affinity rule
/// is large enough that losing a host does not prevent the virtual machine
/// from running.
/// 
/// For manual and partially automated DRS clusters, the Server produces migration
/// recommendations to satisfy the DRS rules. You are not required to act on the
/// recommendations, but the Server maintains the recommendations until the rules
/// are satisfied.
pub trait ClusterRuleInfoTrait : super::data_object_trait::DataObjectTrait {
    /// Unique ID for rules.
    /// 
    /// When adding a new rule, do not specify this property.
    /// The Server will assign the key.
    fn get_key(&self) -> Option<i32>;
    /// Flag to indicate whether or not the rule is currently satisfied.
    fn get_status(&self) -> &Option<super::enums::ManagedEntityStatusEnum>;
    /// Flag to indicate whether or not the rule is enabled.
    /// 
    /// Set this property
    /// when you configure the rule. The default value is false (disabled).
    /// If there is a rule conflict, the Server can override the setting to disable
    /// a rule.
    fn get_enabled(&self) -> Option<bool>;
    /// Name of the rule.
    fn get_name(&self) -> &Option<String>;
    /// Flag to indicate whether compliance with this rule is mandatory or optional.
    /// 
    /// The default value is false (optional).
    /// - A mandatory rule will prevent a virtual machine from being powered on
    ///   or migrated to a host that does not satisfy the rule.
    /// - An optional rule specifies a preference. DRS takes an optional rule
    ///   into consideration when it places a virtual machine in the cluster.
    ///   DRS will act on an optional rule as long as it does not impact
    ///   the ability of the host to satisfy current CPU or memory requirements
    ///   for virtual machines on the system. (As long as the operation does not
    ///   cause any host to be more than 100% utilized.)
    fn get_mandatory(&self) -> Option<bool>;
    /// Flag to indicate whether the rule is created by the user or the system.
    fn get_user_created(&self) -> Option<bool>;
    /// Flag to indicate whether or not the placement of Virtual Machines is currently
    /// in compliance with this rule.
    /// 
    /// The Server does not currently use this property.
    fn get_in_compliance(&self) -> Option<bool>;
    /// UUID for the rule.
    /// 
    /// When adding a new rule, do not specify this
    /// property. The Server will assign the key.
    fn get_rule_uuid(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn ClusterRuleInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ClusterRuleInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ClusterRuleInfoVisitor)
            }
        }

struct ClusterRuleInfoVisitor;

impl<'de> de::Visitor<'de> for ClusterRuleInfoVisitor {
    type Value = Box<dyn ClusterRuleInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ClusterRuleInfoTrait JSON object with a _typeName field")
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

impl ClusterRuleInfoTrait for ClusterRuleInfo {
    fn get_key(&self) -> Option<i32> { self.key }
    fn get_status(&self) -> &Option<super::enums::ManagedEntityStatusEnum> { &self.status }
    fn get_enabled(&self) -> Option<bool> { self.enabled }
    fn get_name(&self) -> &Option<String> { &self.name }
    fn get_mandatory(&self) -> Option<bool> { self.mandatory }
    fn get_user_created(&self) -> Option<bool> { self.user_created }
    fn get_in_compliance(&self) -> Option<bool> { self.in_compliance }
    fn get_rule_uuid(&self) -> &Option<String> { &self.rule_uuid }
}
impl ClusterRuleInfoTrait for ClusterAffinityRuleSpec {
    fn get_key(&self) -> Option<i32> { self.key }
    fn get_status(&self) -> &Option<super::enums::ManagedEntityStatusEnum> { &self.status }
    fn get_enabled(&self) -> Option<bool> { self.enabled }
    fn get_name(&self) -> &Option<String> { &self.name }
    fn get_mandatory(&self) -> Option<bool> { self.mandatory }
    fn get_user_created(&self) -> Option<bool> { self.user_created }
    fn get_in_compliance(&self) -> Option<bool> { self.in_compliance }
    fn get_rule_uuid(&self) -> &Option<String> { &self.rule_uuid }
}
impl ClusterRuleInfoTrait for ClusterAntiAffinityRuleSpec {
    fn get_key(&self) -> Option<i32> { self.key }
    fn get_status(&self) -> &Option<super::enums::ManagedEntityStatusEnum> { &self.status }
    fn get_enabled(&self) -> Option<bool> { self.enabled }
    fn get_name(&self) -> &Option<String> { &self.name }
    fn get_mandatory(&self) -> Option<bool> { self.mandatory }
    fn get_user_created(&self) -> Option<bool> { self.user_created }
    fn get_in_compliance(&self) -> Option<bool> { self.in_compliance }
    fn get_rule_uuid(&self) -> &Option<String> { &self.rule_uuid }
}
impl ClusterRuleInfoTrait for ClusterDependencyRuleInfo {
    fn get_key(&self) -> Option<i32> { self.key }
    fn get_status(&self) -> &Option<super::enums::ManagedEntityStatusEnum> { &self.status }
    fn get_enabled(&self) -> Option<bool> { self.enabled }
    fn get_name(&self) -> &Option<String> { &self.name }
    fn get_mandatory(&self) -> Option<bool> { self.mandatory }
    fn get_user_created(&self) -> Option<bool> { self.user_created }
    fn get_in_compliance(&self) -> Option<bool> { self.in_compliance }
    fn get_rule_uuid(&self) -> &Option<String> { &self.rule_uuid }
}
impl ClusterRuleInfoTrait for ClusterVmHostRuleInfo {
    fn get_key(&self) -> Option<i32> { self.key }
    fn get_status(&self) -> &Option<super::enums::ManagedEntityStatusEnum> { &self.status }
    fn get_enabled(&self) -> Option<bool> { self.enabled }
    fn get_name(&self) -> &Option<String> { &self.name }
    fn get_mandatory(&self) -> Option<bool> { self.mandatory }
    fn get_user_created(&self) -> Option<bool> { self.user_created }
    fn get_in_compliance(&self) -> Option<bool> { self.in_compliance }
    fn get_rule_uuid(&self) -> &Option<String> { &self.rule_uuid }
}
impl ClusterRuleInfoTrait for VirtualDiskAntiAffinityRuleSpec {
    fn get_key(&self) -> Option<i32> { self.key }
    fn get_status(&self) -> &Option<super::enums::ManagedEntityStatusEnum> { &self.status }
    fn get_enabled(&self) -> Option<bool> { self.enabled }
    fn get_name(&self) -> &Option<String> { &self.name }
    fn get_mandatory(&self) -> Option<bool> { self.mandatory }
    fn get_user_created(&self) -> Option<bool> { self.user_created }
    fn get_in_compliance(&self) -> Option<bool> { self.in_compliance }
    fn get_rule_uuid(&self) -> &Option<String> { &self.rule_uuid }
}
impl ClusterRuleInfoTrait for VirtualDiskRuleSpec {
    fn get_key(&self) -> Option<i32> { self.key }
    fn get_status(&self) -> &Option<super::enums::ManagedEntityStatusEnum> { &self.status }
    fn get_enabled(&self) -> Option<bool> { self.enabled }
    fn get_name(&self) -> &Option<String> { &self.name }
    fn get_mandatory(&self) -> Option<bool> { self.mandatory }
    fn get_user_created(&self) -> Option<bool> { self.user_created }
    fn get_in_compliance(&self) -> Option<bool> { self.in_compliance }
    fn get_rule_uuid(&self) -> &Option<String> { &self.rule_uuid }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ClusterRuleInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterRuleInfo => Some(from.as_any_ref().downcast_ref::<ClusterRuleInfo>()?),
            StructType::ClusterAffinityRuleSpec => Some(from.as_any_ref().downcast_ref::<ClusterAffinityRuleSpec>()?),
            StructType::ClusterAntiAffinityRuleSpec => Some(from.as_any_ref().downcast_ref::<ClusterAntiAffinityRuleSpec>()?),
            StructType::ClusterDependencyRuleInfo => Some(from.as_any_ref().downcast_ref::<ClusterDependencyRuleInfo>()?),
            StructType::ClusterVmHostRuleInfo => Some(from.as_any_ref().downcast_ref::<ClusterVmHostRuleInfo>()?),
            StructType::VirtualDiskAntiAffinityRuleSpec => Some(from.as_any_ref().downcast_ref::<VirtualDiskAntiAffinityRuleSpec>()?),
            StructType::VirtualDiskRuleSpec => Some(from.as_any_ref().downcast_ref::<VirtualDiskRuleSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterRuleInfo => Ok(from.as_any_box().downcast::<ClusterRuleInfo>()?),
            StructType::ClusterAffinityRuleSpec => Ok(from.as_any_box().downcast::<ClusterAffinityRuleSpec>()?),
            StructType::ClusterAntiAffinityRuleSpec => Ok(from.as_any_box().downcast::<ClusterAntiAffinityRuleSpec>()?),
            StructType::ClusterDependencyRuleInfo => Ok(from.as_any_box().downcast::<ClusterDependencyRuleInfo>()?),
            StructType::ClusterVmHostRuleInfo => Ok(from.as_any_box().downcast::<ClusterVmHostRuleInfo>()?),
            StructType::VirtualDiskAntiAffinityRuleSpec => Ok(from.as_any_box().downcast::<VirtualDiskAntiAffinityRuleSpec>()?),
            StructType::VirtualDiskRuleSpec => Ok(from.as_any_box().downcast::<VirtualDiskRuleSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
