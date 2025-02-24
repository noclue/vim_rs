use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This event is the base data object type from which all events inherit.
/// 
/// All event
/// objects are data structures that describe events. While event data objects are data
/// structures that describe events, event data type documentation may describe what the
/// event records, rather than the data structure, itself.
pub trait EventTrait : super::data_object_trait::DataObjectTrait {
    /// The event ID.
    fn get_key(&self) -> i32;
    /// The parent or group ID.
    fn get_chain_id(&self) -> i32;
    /// The time the event was created.
    fn get_created_time(&self) -> &str;
    /// The user who caused the event.
    fn get_user_name(&self) -> &str;
    /// The Datacenter object of the event.
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument>;
    /// The ComputeResource object of the event.
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument>;
    /// The Host object of the event.
    fn get_host(&self) -> &Option<HostEventArgument>;
    /// The VirtualMachine object of the event.
    fn get_vm(&self) -> &Option<VmEventArgument>;
    /// The Datastore object of the event.
    fn get_ds(&self) -> &Option<DatastoreEventArgument>;
    /// The Network object of the event.
    fn get_net(&self) -> &Option<NetworkEventArgument>;
    /// The DistributedVirtualSwitch object of the event.
    fn get_dvs(&self) -> &Option<DvsEventArgument>;
    /// A formatted text message describing the event.
    /// 
    /// The message may be localized.
    fn get_full_formatted_message(&self) -> &Option<String>;
    /// The user entered tag to identify the operations and their side effects
    fn get_change_tag(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn EventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn EventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(EventVisitor)
            }
        }

struct EventVisitor;

impl<'de> de::Visitor<'de> for EventVisitor {
    type Value = Box<dyn EventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid EventTrait JSON object with a _typeName field")
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

impl EventTrait for Event {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AlarmEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AlarmAcknowledgedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AlarmActionTriggeredEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AlarmClearedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AlarmCreatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AlarmEmailCompletedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AlarmEmailFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AlarmReconfiguredEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AlarmRemovedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AlarmScriptCompleteEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AlarmScriptFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AlarmSnmpCompletedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AlarmSnmpFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AlarmStatusChangedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AuthorizationEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for PermissionEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for PermissionAddedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for PermissionRemovedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for PermissionUpdatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for RoleEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for RoleAddedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for RoleRemovedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for RoleUpdatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ClusterEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ClusterComplianceCheckedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ClusterCreatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ClusterDestroyedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ClusterOvercommittedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostOvercommittedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ClusterReconfiguredEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ClusterStatusChangedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostStatusChangedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DasAdmissionControlDisabledEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DasAdmissionControlEnabledEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DasAgentFoundEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DasAgentUnavailableEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DasClusterIsolatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DasDisabledEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DasEnabledEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DasHostFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DasHostIsolatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DrsDisabledEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DrsEnabledEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DrsInvocationFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DrsRecoveredFromFailureEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for FailoverLevelRestored {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostMonitoringStateChangedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for InsufficientFailoverResourcesEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmHealthMonitoringStateChangedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for CustomFieldEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for CustomFieldDefEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for CustomFieldDefAddedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for CustomFieldDefRemovedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for CustomFieldDefRenamedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for CustomFieldValueChangedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvPortgroupEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvPortgroupCreatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvPortgroupDestroyedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvPortgroupReconfiguredEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvPortgroupRenamedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvpgImportEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvpgRestoreEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DatacenterEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DatacenterCreatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DatacenterRenamedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DatastoreEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DatastoreCapacityIncreasedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DatastoreDestroyedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DatastoreDuplicatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DatastoreFileEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DatastoreFileCopiedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DatastoreFileDeletedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DatastoreFileMovedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DatastoreIormReconfiguredEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DatastoreRenamedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for NonViWorkloadDetectedOnDatastoreEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsCreatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsDestroyedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsHostBackInSyncEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsHostJoinedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsHostLeftEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsHostStatusUpdated {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsHostWentOutOfSyncEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsImportEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsMergedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsPortBlockedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsPortConnectedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsPortCreatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsPortDeletedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsPortDisconnectedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsPortEnteredPassthruEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsPortExitedPassthruEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsPortJoinPortgroupEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsPortLeavePortgroupEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsPortLinkDownEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsPortLinkUpEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsPortReconfiguredEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsPortRuntimeChangeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsPortUnblockedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsPortVendorSpecificStateChangeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsReconfiguredEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsRenamedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsRestoreEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsUpgradeAvailableEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsUpgradeInProgressEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsUpgradeRejectedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsUpgradedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostLocalPortCreatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for OutOfSyncDvsHost {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for RecoveryEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for RollbackEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmVnicPoolReservationViolationClearEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmVnicPoolReservationViolationRaiseEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for EventEx {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for GeneralEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ExtendedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for GeneralHostErrorEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for GeneralHostInfoEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for GeneralHostWarningEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for GeneralUserEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for GeneralVmErrorEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for GeneralVmInfoEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for GeneralVmWarningEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HealthStatusChangedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AccountCreatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AccountRemovedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AccountUpdatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AdminPasswordNotChangedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for CanceledHostOperationEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DatastoreDiscoveredEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DatastorePrincipalConfigured {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DatastoreRemovedOnHostEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DatastoreRenamedOnHostEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DrsResourceConfigureFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DrsResourceConfigureSyncedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DuplicateIpDetectedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DvsHealthStatusChangeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for MtuMatchEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for MtuMismatchEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for TeamingMatchEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for TeamingMisMatchEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for UplinkPortMtuNotSupportEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for UplinkPortMtuSupportEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for UplinkPortVlanTrunkedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for UplinkPortVlanUntrunkedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for EnteredMaintenanceModeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for EnteredStandbyModeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DrsEnteredStandbyModeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for EnteringMaintenanceModeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for EnteringStandbyModeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DrsEnteringStandbyModeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ExitMaintenanceModeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ExitStandbyModeFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DrsExitStandbyModeFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ExitedStandbyModeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DrsExitedStandbyModeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ExitingStandbyModeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DrsExitingStandbyModeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for GhostDvsProxySwitchDetectedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for GhostDvsProxySwitchRemovedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostAddFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostAddedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostAdminDisableEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostAdminEnableEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostCnxFailedAccountFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostCnxFailedAlreadyManagedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostCnxFailedBadCcagentEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostCnxFailedBadUsernameEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostCnxFailedBadVersionEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostCnxFailedCcagentUpgradeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostCnxFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostCnxFailedNetworkErrorEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostCnxFailedNoAccessEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostCnxFailedNoConnectionEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostCnxFailedNoLicenseEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostCnxFailedNotFoundEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostCnxFailedTimeoutEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostComplianceCheckedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostCompliantEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostConfigAppliedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostConnectedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostConnectionLostEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostDasDisabledEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostDasDisablingEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostDasEnabledEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostDasEnablingEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostDasErrorEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostDasEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostExtraNetworksEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostIsolationIpPingFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostMissingNetworksEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostNoAvailableNetworksEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostNoHaEnabledPortGroupsEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostNoRedundantManagementNetworkEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostNotInClusterEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostPrimaryAgentNotShortNameEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostShortNameInconsistentEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostDasOkEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostDisconnectedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostEnableAdminFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostGetShortNameFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostInAuditModeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostIpChangedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostIpInconsistentEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostIpToShortNameFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostNonCompliantEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostProfileAppliedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostReconnectionFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostRemovedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostShortNameToIpFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostShutdownEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostSpecificationChangedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostSpecificationRequireEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostSpecificationUpdateEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostSubSpecificationDeleteEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostSubSpecificationUpdateEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostSyncFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostUpgradeFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostUserWorldSwapNotEnabledEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostVnicConnectedToCustomizedDvPortEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostWwnChangedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostWwnConflictEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for LocalDatastoreCreatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for LocalTsmEnabledEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for NasDatastoreCreatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for NoDatastoresConfiguredEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for RemoteTsmEnabledEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for TimedOutHostOperationEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for UpdatedAgentBeingRestartedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for UserAssignedToGroup {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for UserPasswordChanged {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for UserUnassignedFromGroup {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmfsDatastoreCreatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmfsDatastoreExpandedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmfsDatastoreExtendedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VcAgentUninstallFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VcAgentUninstalledEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VcAgentUpgradeFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VcAgentUpgradedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VimAccountPasswordChangedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for IScsiBootFailureEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostInventoryUnreadableEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for LicenseEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AllVirtualMachinesLicensedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostInventoryFullEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for HostLicenseExpiredEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for IncorrectHostInformationEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for InvalidEditionEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for LicenseNonComplianceEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for LicenseRestrictedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for LicenseServerAvailableEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for LicenseServerUnavailableEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for NoLicenseEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ServerLicenseExpiredEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for UnlicensedVirtualMachinesEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for UnlicensedVirtualMachinesFoundEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VMotionLicenseExpiredEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for LicenseExpiredEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for LockerMisconfiguredEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for LockerReconfiguredEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for NetworkRollbackEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ProfileEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ProfileAssociatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ProfileChangedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ProfileCreatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ProfileDissociatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ProfileReferenceHostChangedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ProfileRemovedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ResourcePoolEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ResourcePoolCreatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ResourcePoolDestroyedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ResourcePoolMovedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ResourcePoolReconfiguredEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ResourceViolatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ScheduledTaskEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ScheduledTaskCompletedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ScheduledTaskCreatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ScheduledTaskEmailCompletedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ScheduledTaskEmailFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ScheduledTaskFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ScheduledTaskReconfiguredEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ScheduledTaskRemovedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ScheduledTaskStartedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for SessionEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for AlreadyAuthenticatedSessionEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for BadUsernameSessionEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for GlobalMessageChangedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for NoAccessUserEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ServerStartedSessionEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for SessionTerminatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for UserLoginSessionEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for UserLogoutSessionEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for TaskEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for TaskTimeoutEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for TemplateUpgradeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for TemplateBeingUpgradedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for TemplateUpgradeFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for TemplateUpgradedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for UpgradeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for ErrorUpgradeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for InfoUpgradeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for UserUpgradeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for WarningUpgradeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for CustomizationEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for CustomizationFailed {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for CustomizationLinuxIdentityFailed {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for CustomizationNetworkSetupFailed {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for CustomizationSysprepFailed {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for CustomizationUnknownFailure {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for CustomizationStartedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for CustomizationSucceeded {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DrsRuleComplianceEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DrsRuleViolationEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DrsSoftRuleViolationEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for MigrationEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for MigrationErrorEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for MigrationHostErrorEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for MigrationHostWarningEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for MigrationResourceErrorEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for MigrationResourceWarningEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for MigrationWarningEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for NoMaintenanceModeDrsRecommendationForVm {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for NotEnoughResourcesToStartVmEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmAcquiredMksTicketEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmAcquiredTicketEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmAutoRenameEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmBeingCreatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmBeingDeployedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmBeingHotMigratedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmBeingMigratedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmCloneEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmBeingClonedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmBeingClonedNoFolderEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmCloneFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmClonedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmConfigMissingEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmConnectedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmCreatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmDasBeingResetEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmDasBeingResetWithScreenshotEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmDasResetFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmDasUpdateErrorEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmDasUpdateOkEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmDateRolledBackEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmDeployFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmDeployedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmDisconnectedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmDiscoveredEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmDiskFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmEmigratingEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmEndRecordingEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmEndReplayingEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmFailedMigrateEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmFailedRelayoutEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmFailedRelayoutOnVmfs2DatastoreEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmFailedStartingSecondaryEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmFailedToPowerOffEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmFailedToPowerOnEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmFailedToRebootGuestEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmFailedToResetEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmFailedToShutdownGuestEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmFailedToStandbyGuestEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmFailedToSuspendEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmFailedUpdatingSecondaryConfig {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmFailoverFailed {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmFaultToleranceStateChangedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmFaultToleranceTurnedOffEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmFaultToleranceVmTerminatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmGuestOsCrashedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmGuestRebootEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmGuestShutdownEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmGuestStandbyEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmInstanceUuidAssignedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmInstanceUuidChangedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmInstanceUuidConflictEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmMacAssignedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmMacChangedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmMacConflictEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmMaxFtRestartCountReached {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmMaxRestartCountReached {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmMessageErrorEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmMessageEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmMessageWarningEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmMigratedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DrsVmMigratedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmNoCompatibleHostForSecondaryEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmNoNetworkAccessEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmOrphanedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmPoweredOffEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmPowerOffOnIsolationEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmShutdownOnIsolationEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmPoweredOnEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for DrsVmPoweredOnEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmRestartedOnAlternateHostEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmPoweringOnWithCustomizedDvPortEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmPrimaryFailoverEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmReconfiguredEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmRegisteredEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmRelayoutSuccessfulEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmRelayoutUpToDateEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmReloadFromPathEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmReloadFromPathFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmRelocateSpecEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmBeingRelocatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmRelocateFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmRelocatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmRemoteConsoleConnectedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmRemoteConsoleDisconnectedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmRemovedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmRenamedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmRequirementsExceedCurrentEvcModeEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmResettingEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmResourcePoolMovedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmResourceReallocatedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmResumingEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmSecondaryAddedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmSecondaryDisabledBySystemEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmSecondaryDisabledEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmSecondaryEnabledEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmSecondaryStartedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmStartRecordingEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmStartReplayingEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmStartingEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmUnsupportedStartingEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmStartingSecondaryEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmStaticMacConflictEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmStoppingEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmSuspendedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmSuspendingEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmTimedoutStartingSecondaryEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmUpgradeCompleteEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmUpgradeFailedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmUpgradingEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmUuidAssignedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmUuidChangedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmUuidConflictEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmWwnAssignedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmWwnChangedEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl EventTrait for VmWwnConflictEvent {
    fn get_key(&self) -> i32 { self.key }
    fn get_chain_id(&self) -> i32 { self.chain_id }
    fn get_created_time(&self) -> &str { &self.created_time }
    fn get_user_name(&self) -> &str { &self.user_name }
    fn get_datacenter(&self) -> &Option<DatacenterEventArgument> { &self.datacenter }
    fn get_compute_resource(&self) -> &Option<ComputeResourceEventArgument> { &self.compute_resource }
    fn get_host(&self) -> &Option<HostEventArgument> { &self.host }
    fn get_vm(&self) -> &Option<VmEventArgument> { &self.vm }
    fn get_ds(&self) -> &Option<DatastoreEventArgument> { &self.ds }
    fn get_net(&self) -> &Option<NetworkEventArgument> { &self.net }
    fn get_dvs(&self) -> &Option<DvsEventArgument> { &self.dvs }
    fn get_full_formatted_message(&self) -> &Option<String> { &self.full_formatted_message }
    fn get_change_tag(&self) -> &Option<String> { &self.change_tag }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn EventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::Event => Some(from.as_any_ref().downcast_ref::<Event>()?),
            StructType::AlarmEvent => Some(from.as_any_ref().downcast_ref::<AlarmEvent>()?),
            StructType::AlarmAcknowledgedEvent => Some(from.as_any_ref().downcast_ref::<AlarmAcknowledgedEvent>()?),
            StructType::AlarmActionTriggeredEvent => Some(from.as_any_ref().downcast_ref::<AlarmActionTriggeredEvent>()?),
            StructType::AlarmClearedEvent => Some(from.as_any_ref().downcast_ref::<AlarmClearedEvent>()?),
            StructType::AlarmCreatedEvent => Some(from.as_any_ref().downcast_ref::<AlarmCreatedEvent>()?),
            StructType::AlarmEmailCompletedEvent => Some(from.as_any_ref().downcast_ref::<AlarmEmailCompletedEvent>()?),
            StructType::AlarmEmailFailedEvent => Some(from.as_any_ref().downcast_ref::<AlarmEmailFailedEvent>()?),
            StructType::AlarmReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<AlarmReconfiguredEvent>()?),
            StructType::AlarmRemovedEvent => Some(from.as_any_ref().downcast_ref::<AlarmRemovedEvent>()?),
            StructType::AlarmScriptCompleteEvent => Some(from.as_any_ref().downcast_ref::<AlarmScriptCompleteEvent>()?),
            StructType::AlarmScriptFailedEvent => Some(from.as_any_ref().downcast_ref::<AlarmScriptFailedEvent>()?),
            StructType::AlarmSnmpCompletedEvent => Some(from.as_any_ref().downcast_ref::<AlarmSnmpCompletedEvent>()?),
            StructType::AlarmSnmpFailedEvent => Some(from.as_any_ref().downcast_ref::<AlarmSnmpFailedEvent>()?),
            StructType::AlarmStatusChangedEvent => Some(from.as_any_ref().downcast_ref::<AlarmStatusChangedEvent>()?),
            StructType::AuthorizationEvent => Some(from.as_any_ref().downcast_ref::<AuthorizationEvent>()?),
            StructType::PermissionEvent => Some(from.as_any_ref().downcast_ref::<PermissionEvent>()?),
            StructType::PermissionAddedEvent => Some(from.as_any_ref().downcast_ref::<PermissionAddedEvent>()?),
            StructType::PermissionRemovedEvent => Some(from.as_any_ref().downcast_ref::<PermissionRemovedEvent>()?),
            StructType::PermissionUpdatedEvent => Some(from.as_any_ref().downcast_ref::<PermissionUpdatedEvent>()?),
            StructType::RoleEvent => Some(from.as_any_ref().downcast_ref::<RoleEvent>()?),
            StructType::RoleAddedEvent => Some(from.as_any_ref().downcast_ref::<RoleAddedEvent>()?),
            StructType::RoleRemovedEvent => Some(from.as_any_ref().downcast_ref::<RoleRemovedEvent>()?),
            StructType::RoleUpdatedEvent => Some(from.as_any_ref().downcast_ref::<RoleUpdatedEvent>()?),
            StructType::ClusterEvent => Some(from.as_any_ref().downcast_ref::<ClusterEvent>()?),
            StructType::ClusterComplianceCheckedEvent => Some(from.as_any_ref().downcast_ref::<ClusterComplianceCheckedEvent>()?),
            StructType::ClusterCreatedEvent => Some(from.as_any_ref().downcast_ref::<ClusterCreatedEvent>()?),
            StructType::ClusterDestroyedEvent => Some(from.as_any_ref().downcast_ref::<ClusterDestroyedEvent>()?),
            StructType::ClusterOvercommittedEvent => Some(from.as_any_ref().downcast_ref::<ClusterOvercommittedEvent>()?),
            StructType::HostOvercommittedEvent => Some(from.as_any_ref().downcast_ref::<HostOvercommittedEvent>()?),
            StructType::ClusterReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<ClusterReconfiguredEvent>()?),
            StructType::ClusterStatusChangedEvent => Some(from.as_any_ref().downcast_ref::<ClusterStatusChangedEvent>()?),
            StructType::HostStatusChangedEvent => Some(from.as_any_ref().downcast_ref::<HostStatusChangedEvent>()?),
            StructType::DasAdmissionControlDisabledEvent => Some(from.as_any_ref().downcast_ref::<DasAdmissionControlDisabledEvent>()?),
            StructType::DasAdmissionControlEnabledEvent => Some(from.as_any_ref().downcast_ref::<DasAdmissionControlEnabledEvent>()?),
            StructType::DasAgentFoundEvent => Some(from.as_any_ref().downcast_ref::<DasAgentFoundEvent>()?),
            StructType::DasAgentUnavailableEvent => Some(from.as_any_ref().downcast_ref::<DasAgentUnavailableEvent>()?),
            StructType::DasClusterIsolatedEvent => Some(from.as_any_ref().downcast_ref::<DasClusterIsolatedEvent>()?),
            StructType::DasDisabledEvent => Some(from.as_any_ref().downcast_ref::<DasDisabledEvent>()?),
            StructType::DasEnabledEvent => Some(from.as_any_ref().downcast_ref::<DasEnabledEvent>()?),
            StructType::DasHostFailedEvent => Some(from.as_any_ref().downcast_ref::<DasHostFailedEvent>()?),
            StructType::DasHostIsolatedEvent => Some(from.as_any_ref().downcast_ref::<DasHostIsolatedEvent>()?),
            StructType::DrsDisabledEvent => Some(from.as_any_ref().downcast_ref::<DrsDisabledEvent>()?),
            StructType::DrsEnabledEvent => Some(from.as_any_ref().downcast_ref::<DrsEnabledEvent>()?),
            StructType::DrsInvocationFailedEvent => Some(from.as_any_ref().downcast_ref::<DrsInvocationFailedEvent>()?),
            StructType::DrsRecoveredFromFailureEvent => Some(from.as_any_ref().downcast_ref::<DrsRecoveredFromFailureEvent>()?),
            StructType::FailoverLevelRestored => Some(from.as_any_ref().downcast_ref::<FailoverLevelRestored>()?),
            StructType::HostMonitoringStateChangedEvent => Some(from.as_any_ref().downcast_ref::<HostMonitoringStateChangedEvent>()?),
            StructType::InsufficientFailoverResourcesEvent => Some(from.as_any_ref().downcast_ref::<InsufficientFailoverResourcesEvent>()?),
            StructType::VmHealthMonitoringStateChangedEvent => Some(from.as_any_ref().downcast_ref::<VmHealthMonitoringStateChangedEvent>()?),
            StructType::CustomFieldEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldEvent>()?),
            StructType::CustomFieldDefEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldDefEvent>()?),
            StructType::CustomFieldDefAddedEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldDefAddedEvent>()?),
            StructType::CustomFieldDefRemovedEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldDefRemovedEvent>()?),
            StructType::CustomFieldDefRenamedEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldDefRenamedEvent>()?),
            StructType::CustomFieldValueChangedEvent => Some(from.as_any_ref().downcast_ref::<CustomFieldValueChangedEvent>()?),
            StructType::DvPortgroupEvent => Some(from.as_any_ref().downcast_ref::<DvPortgroupEvent>()?),
            StructType::DvPortgroupCreatedEvent => Some(from.as_any_ref().downcast_ref::<DvPortgroupCreatedEvent>()?),
            StructType::DvPortgroupDestroyedEvent => Some(from.as_any_ref().downcast_ref::<DvPortgroupDestroyedEvent>()?),
            StructType::DvPortgroupReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<DvPortgroupReconfiguredEvent>()?),
            StructType::DvPortgroupRenamedEvent => Some(from.as_any_ref().downcast_ref::<DvPortgroupRenamedEvent>()?),
            StructType::DvpgImportEvent => Some(from.as_any_ref().downcast_ref::<DvpgImportEvent>()?),
            StructType::DvpgRestoreEvent => Some(from.as_any_ref().downcast_ref::<DvpgRestoreEvent>()?),
            StructType::DatacenterEvent => Some(from.as_any_ref().downcast_ref::<DatacenterEvent>()?),
            StructType::DatacenterCreatedEvent => Some(from.as_any_ref().downcast_ref::<DatacenterCreatedEvent>()?),
            StructType::DatacenterRenamedEvent => Some(from.as_any_ref().downcast_ref::<DatacenterRenamedEvent>()?),
            StructType::DatastoreEvent => Some(from.as_any_ref().downcast_ref::<DatastoreEvent>()?),
            StructType::DatastoreCapacityIncreasedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreCapacityIncreasedEvent>()?),
            StructType::DatastoreDestroyedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreDestroyedEvent>()?),
            StructType::DatastoreDuplicatedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreDuplicatedEvent>()?),
            StructType::DatastoreFileEvent => Some(from.as_any_ref().downcast_ref::<DatastoreFileEvent>()?),
            StructType::DatastoreFileCopiedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreFileCopiedEvent>()?),
            StructType::DatastoreFileDeletedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreFileDeletedEvent>()?),
            StructType::DatastoreFileMovedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreFileMovedEvent>()?),
            StructType::DatastoreIormReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<DatastoreIormReconfiguredEvent>()?),
            StructType::DatastoreRenamedEvent => Some(from.as_any_ref().downcast_ref::<DatastoreRenamedEvent>()?),
            StructType::NonViWorkloadDetectedOnDatastoreEvent => Some(from.as_any_ref().downcast_ref::<NonViWorkloadDetectedOnDatastoreEvent>()?),
            StructType::DvsEvent => Some(from.as_any_ref().downcast_ref::<DvsEvent>()?),
            StructType::DvsCreatedEvent => Some(from.as_any_ref().downcast_ref::<DvsCreatedEvent>()?),
            StructType::DvsDestroyedEvent => Some(from.as_any_ref().downcast_ref::<DvsDestroyedEvent>()?),
            StructType::DvsHostBackInSyncEvent => Some(from.as_any_ref().downcast_ref::<DvsHostBackInSyncEvent>()?),
            StructType::DvsHostJoinedEvent => Some(from.as_any_ref().downcast_ref::<DvsHostJoinedEvent>()?),
            StructType::DvsHostLeftEvent => Some(from.as_any_ref().downcast_ref::<DvsHostLeftEvent>()?),
            StructType::DvsHostStatusUpdated => Some(from.as_any_ref().downcast_ref::<DvsHostStatusUpdated>()?),
            StructType::DvsHostWentOutOfSyncEvent => Some(from.as_any_ref().downcast_ref::<DvsHostWentOutOfSyncEvent>()?),
            StructType::DvsImportEvent => Some(from.as_any_ref().downcast_ref::<DvsImportEvent>()?),
            StructType::DvsMergedEvent => Some(from.as_any_ref().downcast_ref::<DvsMergedEvent>()?),
            StructType::DvsPortBlockedEvent => Some(from.as_any_ref().downcast_ref::<DvsPortBlockedEvent>()?),
            StructType::DvsPortConnectedEvent => Some(from.as_any_ref().downcast_ref::<DvsPortConnectedEvent>()?),
            StructType::DvsPortCreatedEvent => Some(from.as_any_ref().downcast_ref::<DvsPortCreatedEvent>()?),
            StructType::DvsPortDeletedEvent => Some(from.as_any_ref().downcast_ref::<DvsPortDeletedEvent>()?),
            StructType::DvsPortDisconnectedEvent => Some(from.as_any_ref().downcast_ref::<DvsPortDisconnectedEvent>()?),
            StructType::DvsPortEnteredPassthruEvent => Some(from.as_any_ref().downcast_ref::<DvsPortEnteredPassthruEvent>()?),
            StructType::DvsPortExitedPassthruEvent => Some(from.as_any_ref().downcast_ref::<DvsPortExitedPassthruEvent>()?),
            StructType::DvsPortJoinPortgroupEvent => Some(from.as_any_ref().downcast_ref::<DvsPortJoinPortgroupEvent>()?),
            StructType::DvsPortLeavePortgroupEvent => Some(from.as_any_ref().downcast_ref::<DvsPortLeavePortgroupEvent>()?),
            StructType::DvsPortLinkDownEvent => Some(from.as_any_ref().downcast_ref::<DvsPortLinkDownEvent>()?),
            StructType::DvsPortLinkUpEvent => Some(from.as_any_ref().downcast_ref::<DvsPortLinkUpEvent>()?),
            StructType::DvsPortReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<DvsPortReconfiguredEvent>()?),
            StructType::DvsPortRuntimeChangeEvent => Some(from.as_any_ref().downcast_ref::<DvsPortRuntimeChangeEvent>()?),
            StructType::DvsPortUnblockedEvent => Some(from.as_any_ref().downcast_ref::<DvsPortUnblockedEvent>()?),
            StructType::DvsPortVendorSpecificStateChangeEvent => Some(from.as_any_ref().downcast_ref::<DvsPortVendorSpecificStateChangeEvent>()?),
            StructType::DvsReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<DvsReconfiguredEvent>()?),
            StructType::DvsRenamedEvent => Some(from.as_any_ref().downcast_ref::<DvsRenamedEvent>()?),
            StructType::DvsRestoreEvent => Some(from.as_any_ref().downcast_ref::<DvsRestoreEvent>()?),
            StructType::DvsUpgradeAvailableEvent => Some(from.as_any_ref().downcast_ref::<DvsUpgradeAvailableEvent>()?),
            StructType::DvsUpgradeInProgressEvent => Some(from.as_any_ref().downcast_ref::<DvsUpgradeInProgressEvent>()?),
            StructType::DvsUpgradeRejectedEvent => Some(from.as_any_ref().downcast_ref::<DvsUpgradeRejectedEvent>()?),
            StructType::DvsUpgradedEvent => Some(from.as_any_ref().downcast_ref::<DvsUpgradedEvent>()?),
            StructType::HostLocalPortCreatedEvent => Some(from.as_any_ref().downcast_ref::<HostLocalPortCreatedEvent>()?),
            StructType::OutOfSyncDvsHost => Some(from.as_any_ref().downcast_ref::<OutOfSyncDvsHost>()?),
            StructType::RecoveryEvent => Some(from.as_any_ref().downcast_ref::<RecoveryEvent>()?),
            StructType::RollbackEvent => Some(from.as_any_ref().downcast_ref::<RollbackEvent>()?),
            StructType::VmVnicPoolReservationViolationClearEvent => Some(from.as_any_ref().downcast_ref::<VmVnicPoolReservationViolationClearEvent>()?),
            StructType::VmVnicPoolReservationViolationRaiseEvent => Some(from.as_any_ref().downcast_ref::<VmVnicPoolReservationViolationRaiseEvent>()?),
            StructType::EventEx => Some(from.as_any_ref().downcast_ref::<EventEx>()?),
            StructType::GeneralEvent => Some(from.as_any_ref().downcast_ref::<GeneralEvent>()?),
            StructType::ExtendedEvent => Some(from.as_any_ref().downcast_ref::<ExtendedEvent>()?),
            StructType::GeneralHostErrorEvent => Some(from.as_any_ref().downcast_ref::<GeneralHostErrorEvent>()?),
            StructType::GeneralHostInfoEvent => Some(from.as_any_ref().downcast_ref::<GeneralHostInfoEvent>()?),
            StructType::GeneralHostWarningEvent => Some(from.as_any_ref().downcast_ref::<GeneralHostWarningEvent>()?),
            StructType::GeneralUserEvent => Some(from.as_any_ref().downcast_ref::<GeneralUserEvent>()?),
            StructType::GeneralVmErrorEvent => Some(from.as_any_ref().downcast_ref::<GeneralVmErrorEvent>()?),
            StructType::GeneralVmInfoEvent => Some(from.as_any_ref().downcast_ref::<GeneralVmInfoEvent>()?),
            StructType::GeneralVmWarningEvent => Some(from.as_any_ref().downcast_ref::<GeneralVmWarningEvent>()?),
            StructType::HealthStatusChangedEvent => Some(from.as_any_ref().downcast_ref::<HealthStatusChangedEvent>()?),
            StructType::HostEvent => Some(from.as_any_ref().downcast_ref::<HostEvent>()?),
            StructType::AccountCreatedEvent => Some(from.as_any_ref().downcast_ref::<AccountCreatedEvent>()?),
            StructType::AccountRemovedEvent => Some(from.as_any_ref().downcast_ref::<AccountRemovedEvent>()?),
            StructType::AccountUpdatedEvent => Some(from.as_any_ref().downcast_ref::<AccountUpdatedEvent>()?),
            StructType::AdminPasswordNotChangedEvent => Some(from.as_any_ref().downcast_ref::<AdminPasswordNotChangedEvent>()?),
            StructType::CanceledHostOperationEvent => Some(from.as_any_ref().downcast_ref::<CanceledHostOperationEvent>()?),
            StructType::DatastoreDiscoveredEvent => Some(from.as_any_ref().downcast_ref::<DatastoreDiscoveredEvent>()?),
            StructType::DatastorePrincipalConfigured => Some(from.as_any_ref().downcast_ref::<DatastorePrincipalConfigured>()?),
            StructType::DatastoreRemovedOnHostEvent => Some(from.as_any_ref().downcast_ref::<DatastoreRemovedOnHostEvent>()?),
            StructType::DatastoreRenamedOnHostEvent => Some(from.as_any_ref().downcast_ref::<DatastoreRenamedOnHostEvent>()?),
            StructType::DrsResourceConfigureFailedEvent => Some(from.as_any_ref().downcast_ref::<DrsResourceConfigureFailedEvent>()?),
            StructType::DrsResourceConfigureSyncedEvent => Some(from.as_any_ref().downcast_ref::<DrsResourceConfigureSyncedEvent>()?),
            StructType::DuplicateIpDetectedEvent => Some(from.as_any_ref().downcast_ref::<DuplicateIpDetectedEvent>()?),
            StructType::DvsHealthStatusChangeEvent => Some(from.as_any_ref().downcast_ref::<DvsHealthStatusChangeEvent>()?),
            StructType::MtuMatchEvent => Some(from.as_any_ref().downcast_ref::<MtuMatchEvent>()?),
            StructType::MtuMismatchEvent => Some(from.as_any_ref().downcast_ref::<MtuMismatchEvent>()?),
            StructType::TeamingMatchEvent => Some(from.as_any_ref().downcast_ref::<TeamingMatchEvent>()?),
            StructType::TeamingMisMatchEvent => Some(from.as_any_ref().downcast_ref::<TeamingMisMatchEvent>()?),
            StructType::UplinkPortMtuNotSupportEvent => Some(from.as_any_ref().downcast_ref::<UplinkPortMtuNotSupportEvent>()?),
            StructType::UplinkPortMtuSupportEvent => Some(from.as_any_ref().downcast_ref::<UplinkPortMtuSupportEvent>()?),
            StructType::UplinkPortVlanTrunkedEvent => Some(from.as_any_ref().downcast_ref::<UplinkPortVlanTrunkedEvent>()?),
            StructType::UplinkPortVlanUntrunkedEvent => Some(from.as_any_ref().downcast_ref::<UplinkPortVlanUntrunkedEvent>()?),
            StructType::EnteredMaintenanceModeEvent => Some(from.as_any_ref().downcast_ref::<EnteredMaintenanceModeEvent>()?),
            StructType::EnteredStandbyModeEvent => Some(from.as_any_ref().downcast_ref::<EnteredStandbyModeEvent>()?),
            StructType::DrsEnteredStandbyModeEvent => Some(from.as_any_ref().downcast_ref::<DrsEnteredStandbyModeEvent>()?),
            StructType::EnteringMaintenanceModeEvent => Some(from.as_any_ref().downcast_ref::<EnteringMaintenanceModeEvent>()?),
            StructType::EnteringStandbyModeEvent => Some(from.as_any_ref().downcast_ref::<EnteringStandbyModeEvent>()?),
            StructType::DrsEnteringStandbyModeEvent => Some(from.as_any_ref().downcast_ref::<DrsEnteringStandbyModeEvent>()?),
            StructType::ExitMaintenanceModeEvent => Some(from.as_any_ref().downcast_ref::<ExitMaintenanceModeEvent>()?),
            StructType::ExitStandbyModeFailedEvent => Some(from.as_any_ref().downcast_ref::<ExitStandbyModeFailedEvent>()?),
            StructType::DrsExitStandbyModeFailedEvent => Some(from.as_any_ref().downcast_ref::<DrsExitStandbyModeFailedEvent>()?),
            StructType::ExitedStandbyModeEvent => Some(from.as_any_ref().downcast_ref::<ExitedStandbyModeEvent>()?),
            StructType::DrsExitedStandbyModeEvent => Some(from.as_any_ref().downcast_ref::<DrsExitedStandbyModeEvent>()?),
            StructType::ExitingStandbyModeEvent => Some(from.as_any_ref().downcast_ref::<ExitingStandbyModeEvent>()?),
            StructType::DrsExitingStandbyModeEvent => Some(from.as_any_ref().downcast_ref::<DrsExitingStandbyModeEvent>()?),
            StructType::GhostDvsProxySwitchDetectedEvent => Some(from.as_any_ref().downcast_ref::<GhostDvsProxySwitchDetectedEvent>()?),
            StructType::GhostDvsProxySwitchRemovedEvent => Some(from.as_any_ref().downcast_ref::<GhostDvsProxySwitchRemovedEvent>()?),
            StructType::HostAddFailedEvent => Some(from.as_any_ref().downcast_ref::<HostAddFailedEvent>()?),
            StructType::HostAddedEvent => Some(from.as_any_ref().downcast_ref::<HostAddedEvent>()?),
            StructType::HostAdminDisableEvent => Some(from.as_any_ref().downcast_ref::<HostAdminDisableEvent>()?),
            StructType::HostAdminEnableEvent => Some(from.as_any_ref().downcast_ref::<HostAdminEnableEvent>()?),
            StructType::HostCnxFailedAccountFailedEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedAccountFailedEvent>()?),
            StructType::HostCnxFailedAlreadyManagedEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedAlreadyManagedEvent>()?),
            StructType::HostCnxFailedBadCcagentEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedBadCcagentEvent>()?),
            StructType::HostCnxFailedBadUsernameEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedBadUsernameEvent>()?),
            StructType::HostCnxFailedBadVersionEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedBadVersionEvent>()?),
            StructType::HostCnxFailedCcagentUpgradeEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedCcagentUpgradeEvent>()?),
            StructType::HostCnxFailedEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedEvent>()?),
            StructType::HostCnxFailedNetworkErrorEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedNetworkErrorEvent>()?),
            StructType::HostCnxFailedNoAccessEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedNoAccessEvent>()?),
            StructType::HostCnxFailedNoConnectionEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedNoConnectionEvent>()?),
            StructType::HostCnxFailedNoLicenseEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedNoLicenseEvent>()?),
            StructType::HostCnxFailedNotFoundEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedNotFoundEvent>()?),
            StructType::HostCnxFailedTimeoutEvent => Some(from.as_any_ref().downcast_ref::<HostCnxFailedTimeoutEvent>()?),
            StructType::HostComplianceCheckedEvent => Some(from.as_any_ref().downcast_ref::<HostComplianceCheckedEvent>()?),
            StructType::HostCompliantEvent => Some(from.as_any_ref().downcast_ref::<HostCompliantEvent>()?),
            StructType::HostConfigAppliedEvent => Some(from.as_any_ref().downcast_ref::<HostConfigAppliedEvent>()?),
            StructType::HostConnectedEvent => Some(from.as_any_ref().downcast_ref::<HostConnectedEvent>()?),
            StructType::HostConnectionLostEvent => Some(from.as_any_ref().downcast_ref::<HostConnectionLostEvent>()?),
            StructType::HostDasDisabledEvent => Some(from.as_any_ref().downcast_ref::<HostDasDisabledEvent>()?),
            StructType::HostDasDisablingEvent => Some(from.as_any_ref().downcast_ref::<HostDasDisablingEvent>()?),
            StructType::HostDasEnabledEvent => Some(from.as_any_ref().downcast_ref::<HostDasEnabledEvent>()?),
            StructType::HostDasEnablingEvent => Some(from.as_any_ref().downcast_ref::<HostDasEnablingEvent>()?),
            StructType::HostDasErrorEvent => Some(from.as_any_ref().downcast_ref::<HostDasErrorEvent>()?),
            StructType::HostDasEvent => Some(from.as_any_ref().downcast_ref::<HostDasEvent>()?),
            StructType::HostExtraNetworksEvent => Some(from.as_any_ref().downcast_ref::<HostExtraNetworksEvent>()?),
            StructType::HostIsolationIpPingFailedEvent => Some(from.as_any_ref().downcast_ref::<HostIsolationIpPingFailedEvent>()?),
            StructType::HostMissingNetworksEvent => Some(from.as_any_ref().downcast_ref::<HostMissingNetworksEvent>()?),
            StructType::HostNoAvailableNetworksEvent => Some(from.as_any_ref().downcast_ref::<HostNoAvailableNetworksEvent>()?),
            StructType::HostNoHaEnabledPortGroupsEvent => Some(from.as_any_ref().downcast_ref::<HostNoHaEnabledPortGroupsEvent>()?),
            StructType::HostNoRedundantManagementNetworkEvent => Some(from.as_any_ref().downcast_ref::<HostNoRedundantManagementNetworkEvent>()?),
            StructType::HostNotInClusterEvent => Some(from.as_any_ref().downcast_ref::<HostNotInClusterEvent>()?),
            StructType::HostPrimaryAgentNotShortNameEvent => Some(from.as_any_ref().downcast_ref::<HostPrimaryAgentNotShortNameEvent>()?),
            StructType::HostShortNameInconsistentEvent => Some(from.as_any_ref().downcast_ref::<HostShortNameInconsistentEvent>()?),
            StructType::HostDasOkEvent => Some(from.as_any_ref().downcast_ref::<HostDasOkEvent>()?),
            StructType::HostDisconnectedEvent => Some(from.as_any_ref().downcast_ref::<HostDisconnectedEvent>()?),
            StructType::HostEnableAdminFailedEvent => Some(from.as_any_ref().downcast_ref::<HostEnableAdminFailedEvent>()?),
            StructType::HostGetShortNameFailedEvent => Some(from.as_any_ref().downcast_ref::<HostGetShortNameFailedEvent>()?),
            StructType::HostInAuditModeEvent => Some(from.as_any_ref().downcast_ref::<HostInAuditModeEvent>()?),
            StructType::HostIpChangedEvent => Some(from.as_any_ref().downcast_ref::<HostIpChangedEvent>()?),
            StructType::HostIpInconsistentEvent => Some(from.as_any_ref().downcast_ref::<HostIpInconsistentEvent>()?),
            StructType::HostIpToShortNameFailedEvent => Some(from.as_any_ref().downcast_ref::<HostIpToShortNameFailedEvent>()?),
            StructType::HostNonCompliantEvent => Some(from.as_any_ref().downcast_ref::<HostNonCompliantEvent>()?),
            StructType::HostProfileAppliedEvent => Some(from.as_any_ref().downcast_ref::<HostProfileAppliedEvent>()?),
            StructType::HostReconnectionFailedEvent => Some(from.as_any_ref().downcast_ref::<HostReconnectionFailedEvent>()?),
            StructType::HostRemovedEvent => Some(from.as_any_ref().downcast_ref::<HostRemovedEvent>()?),
            StructType::HostShortNameToIpFailedEvent => Some(from.as_any_ref().downcast_ref::<HostShortNameToIpFailedEvent>()?),
            StructType::HostShutdownEvent => Some(from.as_any_ref().downcast_ref::<HostShutdownEvent>()?),
            StructType::HostSpecificationChangedEvent => Some(from.as_any_ref().downcast_ref::<HostSpecificationChangedEvent>()?),
            StructType::HostSpecificationRequireEvent => Some(from.as_any_ref().downcast_ref::<HostSpecificationRequireEvent>()?),
            StructType::HostSpecificationUpdateEvent => Some(from.as_any_ref().downcast_ref::<HostSpecificationUpdateEvent>()?),
            StructType::HostSubSpecificationDeleteEvent => Some(from.as_any_ref().downcast_ref::<HostSubSpecificationDeleteEvent>()?),
            StructType::HostSubSpecificationUpdateEvent => Some(from.as_any_ref().downcast_ref::<HostSubSpecificationUpdateEvent>()?),
            StructType::HostSyncFailedEvent => Some(from.as_any_ref().downcast_ref::<HostSyncFailedEvent>()?),
            StructType::HostUpgradeFailedEvent => Some(from.as_any_ref().downcast_ref::<HostUpgradeFailedEvent>()?),
            StructType::HostUserWorldSwapNotEnabledEvent => Some(from.as_any_ref().downcast_ref::<HostUserWorldSwapNotEnabledEvent>()?),
            StructType::HostVnicConnectedToCustomizedDvPortEvent => Some(from.as_any_ref().downcast_ref::<HostVnicConnectedToCustomizedDvPortEvent>()?),
            StructType::HostWwnChangedEvent => Some(from.as_any_ref().downcast_ref::<HostWwnChangedEvent>()?),
            StructType::HostWwnConflictEvent => Some(from.as_any_ref().downcast_ref::<HostWwnConflictEvent>()?),
            StructType::LocalDatastoreCreatedEvent => Some(from.as_any_ref().downcast_ref::<LocalDatastoreCreatedEvent>()?),
            StructType::LocalTsmEnabledEvent => Some(from.as_any_ref().downcast_ref::<LocalTsmEnabledEvent>()?),
            StructType::NasDatastoreCreatedEvent => Some(from.as_any_ref().downcast_ref::<NasDatastoreCreatedEvent>()?),
            StructType::NoDatastoresConfiguredEvent => Some(from.as_any_ref().downcast_ref::<NoDatastoresConfiguredEvent>()?),
            StructType::RemoteTsmEnabledEvent => Some(from.as_any_ref().downcast_ref::<RemoteTsmEnabledEvent>()?),
            StructType::TimedOutHostOperationEvent => Some(from.as_any_ref().downcast_ref::<TimedOutHostOperationEvent>()?),
            StructType::UpdatedAgentBeingRestartedEvent => Some(from.as_any_ref().downcast_ref::<UpdatedAgentBeingRestartedEvent>()?),
            StructType::UserAssignedToGroup => Some(from.as_any_ref().downcast_ref::<UserAssignedToGroup>()?),
            StructType::UserPasswordChanged => Some(from.as_any_ref().downcast_ref::<UserPasswordChanged>()?),
            StructType::UserUnassignedFromGroup => Some(from.as_any_ref().downcast_ref::<UserUnassignedFromGroup>()?),
            StructType::VmfsDatastoreCreatedEvent => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreCreatedEvent>()?),
            StructType::VmfsDatastoreExpandedEvent => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreExpandedEvent>()?),
            StructType::VmfsDatastoreExtendedEvent => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreExtendedEvent>()?),
            StructType::VcAgentUninstallFailedEvent => Some(from.as_any_ref().downcast_ref::<VcAgentUninstallFailedEvent>()?),
            StructType::VcAgentUninstalledEvent => Some(from.as_any_ref().downcast_ref::<VcAgentUninstalledEvent>()?),
            StructType::VcAgentUpgradeFailedEvent => Some(from.as_any_ref().downcast_ref::<VcAgentUpgradeFailedEvent>()?),
            StructType::VcAgentUpgradedEvent => Some(from.as_any_ref().downcast_ref::<VcAgentUpgradedEvent>()?),
            StructType::VimAccountPasswordChangedEvent => Some(from.as_any_ref().downcast_ref::<VimAccountPasswordChangedEvent>()?),
            StructType::IScsiBootFailureEvent => Some(from.as_any_ref().downcast_ref::<IScsiBootFailureEvent>()?),
            StructType::HostInventoryUnreadableEvent => Some(from.as_any_ref().downcast_ref::<HostInventoryUnreadableEvent>()?),
            StructType::LicenseEvent => Some(from.as_any_ref().downcast_ref::<LicenseEvent>()?),
            StructType::AllVirtualMachinesLicensedEvent => Some(from.as_any_ref().downcast_ref::<AllVirtualMachinesLicensedEvent>()?),
            StructType::HostInventoryFullEvent => Some(from.as_any_ref().downcast_ref::<HostInventoryFullEvent>()?),
            StructType::HostLicenseExpiredEvent => Some(from.as_any_ref().downcast_ref::<HostLicenseExpiredEvent>()?),
            StructType::IncorrectHostInformationEvent => Some(from.as_any_ref().downcast_ref::<IncorrectHostInformationEvent>()?),
            StructType::InvalidEditionEvent => Some(from.as_any_ref().downcast_ref::<InvalidEditionEvent>()?),
            StructType::LicenseNonComplianceEvent => Some(from.as_any_ref().downcast_ref::<LicenseNonComplianceEvent>()?),
            StructType::LicenseRestrictedEvent => Some(from.as_any_ref().downcast_ref::<LicenseRestrictedEvent>()?),
            StructType::LicenseServerAvailableEvent => Some(from.as_any_ref().downcast_ref::<LicenseServerAvailableEvent>()?),
            StructType::LicenseServerUnavailableEvent => Some(from.as_any_ref().downcast_ref::<LicenseServerUnavailableEvent>()?),
            StructType::NoLicenseEvent => Some(from.as_any_ref().downcast_ref::<NoLicenseEvent>()?),
            StructType::ServerLicenseExpiredEvent => Some(from.as_any_ref().downcast_ref::<ServerLicenseExpiredEvent>()?),
            StructType::UnlicensedVirtualMachinesEvent => Some(from.as_any_ref().downcast_ref::<UnlicensedVirtualMachinesEvent>()?),
            StructType::UnlicensedVirtualMachinesFoundEvent => Some(from.as_any_ref().downcast_ref::<UnlicensedVirtualMachinesFoundEvent>()?),
            StructType::VMotionLicenseExpiredEvent => Some(from.as_any_ref().downcast_ref::<VMotionLicenseExpiredEvent>()?),
            StructType::LicenseExpiredEvent => Some(from.as_any_ref().downcast_ref::<LicenseExpiredEvent>()?),
            StructType::LockerMisconfiguredEvent => Some(from.as_any_ref().downcast_ref::<LockerMisconfiguredEvent>()?),
            StructType::LockerReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<LockerReconfiguredEvent>()?),
            StructType::NetworkRollbackEvent => Some(from.as_any_ref().downcast_ref::<NetworkRollbackEvent>()?),
            StructType::ProfileEvent => Some(from.as_any_ref().downcast_ref::<ProfileEvent>()?),
            StructType::ProfileAssociatedEvent => Some(from.as_any_ref().downcast_ref::<ProfileAssociatedEvent>()?),
            StructType::ProfileChangedEvent => Some(from.as_any_ref().downcast_ref::<ProfileChangedEvent>()?),
            StructType::ProfileCreatedEvent => Some(from.as_any_ref().downcast_ref::<ProfileCreatedEvent>()?),
            StructType::ProfileDissociatedEvent => Some(from.as_any_ref().downcast_ref::<ProfileDissociatedEvent>()?),
            StructType::ProfileReferenceHostChangedEvent => Some(from.as_any_ref().downcast_ref::<ProfileReferenceHostChangedEvent>()?),
            StructType::ProfileRemovedEvent => Some(from.as_any_ref().downcast_ref::<ProfileRemovedEvent>()?),
            StructType::ResourcePoolEvent => Some(from.as_any_ref().downcast_ref::<ResourcePoolEvent>()?),
            StructType::ResourcePoolCreatedEvent => Some(from.as_any_ref().downcast_ref::<ResourcePoolCreatedEvent>()?),
            StructType::ResourcePoolDestroyedEvent => Some(from.as_any_ref().downcast_ref::<ResourcePoolDestroyedEvent>()?),
            StructType::ResourcePoolMovedEvent => Some(from.as_any_ref().downcast_ref::<ResourcePoolMovedEvent>()?),
            StructType::ResourcePoolReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<ResourcePoolReconfiguredEvent>()?),
            StructType::ResourceViolatedEvent => Some(from.as_any_ref().downcast_ref::<ResourceViolatedEvent>()?),
            StructType::ScheduledTaskEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskEvent>()?),
            StructType::ScheduledTaskCompletedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskCompletedEvent>()?),
            StructType::ScheduledTaskCreatedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskCreatedEvent>()?),
            StructType::ScheduledTaskEmailCompletedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskEmailCompletedEvent>()?),
            StructType::ScheduledTaskEmailFailedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskEmailFailedEvent>()?),
            StructType::ScheduledTaskFailedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskFailedEvent>()?),
            StructType::ScheduledTaskReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskReconfiguredEvent>()?),
            StructType::ScheduledTaskRemovedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskRemovedEvent>()?),
            StructType::ScheduledTaskStartedEvent => Some(from.as_any_ref().downcast_ref::<ScheduledTaskStartedEvent>()?),
            StructType::SessionEvent => Some(from.as_any_ref().downcast_ref::<SessionEvent>()?),
            StructType::AlreadyAuthenticatedSessionEvent => Some(from.as_any_ref().downcast_ref::<AlreadyAuthenticatedSessionEvent>()?),
            StructType::BadUsernameSessionEvent => Some(from.as_any_ref().downcast_ref::<BadUsernameSessionEvent>()?),
            StructType::GlobalMessageChangedEvent => Some(from.as_any_ref().downcast_ref::<GlobalMessageChangedEvent>()?),
            StructType::NoAccessUserEvent => Some(from.as_any_ref().downcast_ref::<NoAccessUserEvent>()?),
            StructType::ServerStartedSessionEvent => Some(from.as_any_ref().downcast_ref::<ServerStartedSessionEvent>()?),
            StructType::SessionTerminatedEvent => Some(from.as_any_ref().downcast_ref::<SessionTerminatedEvent>()?),
            StructType::UserLoginSessionEvent => Some(from.as_any_ref().downcast_ref::<UserLoginSessionEvent>()?),
            StructType::UserLogoutSessionEvent => Some(from.as_any_ref().downcast_ref::<UserLogoutSessionEvent>()?),
            StructType::TaskEvent => Some(from.as_any_ref().downcast_ref::<TaskEvent>()?),
            StructType::TaskTimeoutEvent => Some(from.as_any_ref().downcast_ref::<TaskTimeoutEvent>()?),
            StructType::TemplateUpgradeEvent => Some(from.as_any_ref().downcast_ref::<TemplateUpgradeEvent>()?),
            StructType::TemplateBeingUpgradedEvent => Some(from.as_any_ref().downcast_ref::<TemplateBeingUpgradedEvent>()?),
            StructType::TemplateUpgradeFailedEvent => Some(from.as_any_ref().downcast_ref::<TemplateUpgradeFailedEvent>()?),
            StructType::TemplateUpgradedEvent => Some(from.as_any_ref().downcast_ref::<TemplateUpgradedEvent>()?),
            StructType::UpgradeEvent => Some(from.as_any_ref().downcast_ref::<UpgradeEvent>()?),
            StructType::ErrorUpgradeEvent => Some(from.as_any_ref().downcast_ref::<ErrorUpgradeEvent>()?),
            StructType::InfoUpgradeEvent => Some(from.as_any_ref().downcast_ref::<InfoUpgradeEvent>()?),
            StructType::UserUpgradeEvent => Some(from.as_any_ref().downcast_ref::<UserUpgradeEvent>()?),
            StructType::WarningUpgradeEvent => Some(from.as_any_ref().downcast_ref::<WarningUpgradeEvent>()?),
            StructType::VmEvent => Some(from.as_any_ref().downcast_ref::<VmEvent>()?),
            StructType::CustomizationEvent => Some(from.as_any_ref().downcast_ref::<CustomizationEvent>()?),
            StructType::CustomizationFailed => Some(from.as_any_ref().downcast_ref::<CustomizationFailed>()?),
            StructType::CustomizationLinuxIdentityFailed => Some(from.as_any_ref().downcast_ref::<CustomizationLinuxIdentityFailed>()?),
            StructType::CustomizationNetworkSetupFailed => Some(from.as_any_ref().downcast_ref::<CustomizationNetworkSetupFailed>()?),
            StructType::CustomizationSysprepFailed => Some(from.as_any_ref().downcast_ref::<CustomizationSysprepFailed>()?),
            StructType::CustomizationUnknownFailure => Some(from.as_any_ref().downcast_ref::<CustomizationUnknownFailure>()?),
            StructType::CustomizationStartedEvent => Some(from.as_any_ref().downcast_ref::<CustomizationStartedEvent>()?),
            StructType::CustomizationSucceeded => Some(from.as_any_ref().downcast_ref::<CustomizationSucceeded>()?),
            StructType::DrsRuleComplianceEvent => Some(from.as_any_ref().downcast_ref::<DrsRuleComplianceEvent>()?),
            StructType::DrsRuleViolationEvent => Some(from.as_any_ref().downcast_ref::<DrsRuleViolationEvent>()?),
            StructType::DrsSoftRuleViolationEvent => Some(from.as_any_ref().downcast_ref::<DrsSoftRuleViolationEvent>()?),
            StructType::MigrationEvent => Some(from.as_any_ref().downcast_ref::<MigrationEvent>()?),
            StructType::MigrationErrorEvent => Some(from.as_any_ref().downcast_ref::<MigrationErrorEvent>()?),
            StructType::MigrationHostErrorEvent => Some(from.as_any_ref().downcast_ref::<MigrationHostErrorEvent>()?),
            StructType::MigrationHostWarningEvent => Some(from.as_any_ref().downcast_ref::<MigrationHostWarningEvent>()?),
            StructType::MigrationResourceErrorEvent => Some(from.as_any_ref().downcast_ref::<MigrationResourceErrorEvent>()?),
            StructType::MigrationResourceWarningEvent => Some(from.as_any_ref().downcast_ref::<MigrationResourceWarningEvent>()?),
            StructType::MigrationWarningEvent => Some(from.as_any_ref().downcast_ref::<MigrationWarningEvent>()?),
            StructType::NoMaintenanceModeDrsRecommendationForVm => Some(from.as_any_ref().downcast_ref::<NoMaintenanceModeDrsRecommendationForVm>()?),
            StructType::NotEnoughResourcesToStartVmEvent => Some(from.as_any_ref().downcast_ref::<NotEnoughResourcesToStartVmEvent>()?),
            StructType::VmAcquiredMksTicketEvent => Some(from.as_any_ref().downcast_ref::<VmAcquiredMksTicketEvent>()?),
            StructType::VmAcquiredTicketEvent => Some(from.as_any_ref().downcast_ref::<VmAcquiredTicketEvent>()?),
            StructType::VmAutoRenameEvent => Some(from.as_any_ref().downcast_ref::<VmAutoRenameEvent>()?),
            StructType::VmBeingCreatedEvent => Some(from.as_any_ref().downcast_ref::<VmBeingCreatedEvent>()?),
            StructType::VmBeingDeployedEvent => Some(from.as_any_ref().downcast_ref::<VmBeingDeployedEvent>()?),
            StructType::VmBeingHotMigratedEvent => Some(from.as_any_ref().downcast_ref::<VmBeingHotMigratedEvent>()?),
            StructType::VmBeingMigratedEvent => Some(from.as_any_ref().downcast_ref::<VmBeingMigratedEvent>()?),
            StructType::VmCloneEvent => Some(from.as_any_ref().downcast_ref::<VmCloneEvent>()?),
            StructType::VmBeingClonedEvent => Some(from.as_any_ref().downcast_ref::<VmBeingClonedEvent>()?),
            StructType::VmBeingClonedNoFolderEvent => Some(from.as_any_ref().downcast_ref::<VmBeingClonedNoFolderEvent>()?),
            StructType::VmCloneFailedEvent => Some(from.as_any_ref().downcast_ref::<VmCloneFailedEvent>()?),
            StructType::VmClonedEvent => Some(from.as_any_ref().downcast_ref::<VmClonedEvent>()?),
            StructType::VmConfigMissingEvent => Some(from.as_any_ref().downcast_ref::<VmConfigMissingEvent>()?),
            StructType::VmConnectedEvent => Some(from.as_any_ref().downcast_ref::<VmConnectedEvent>()?),
            StructType::VmCreatedEvent => Some(from.as_any_ref().downcast_ref::<VmCreatedEvent>()?),
            StructType::VmDasBeingResetEvent => Some(from.as_any_ref().downcast_ref::<VmDasBeingResetEvent>()?),
            StructType::VmDasBeingResetWithScreenshotEvent => Some(from.as_any_ref().downcast_ref::<VmDasBeingResetWithScreenshotEvent>()?),
            StructType::VmDasResetFailedEvent => Some(from.as_any_ref().downcast_ref::<VmDasResetFailedEvent>()?),
            StructType::VmDasUpdateErrorEvent => Some(from.as_any_ref().downcast_ref::<VmDasUpdateErrorEvent>()?),
            StructType::VmDasUpdateOkEvent => Some(from.as_any_ref().downcast_ref::<VmDasUpdateOkEvent>()?),
            StructType::VmDateRolledBackEvent => Some(from.as_any_ref().downcast_ref::<VmDateRolledBackEvent>()?),
            StructType::VmDeployFailedEvent => Some(from.as_any_ref().downcast_ref::<VmDeployFailedEvent>()?),
            StructType::VmDeployedEvent => Some(from.as_any_ref().downcast_ref::<VmDeployedEvent>()?),
            StructType::VmDisconnectedEvent => Some(from.as_any_ref().downcast_ref::<VmDisconnectedEvent>()?),
            StructType::VmDiscoveredEvent => Some(from.as_any_ref().downcast_ref::<VmDiscoveredEvent>()?),
            StructType::VmDiskFailedEvent => Some(from.as_any_ref().downcast_ref::<VmDiskFailedEvent>()?),
            StructType::VmEmigratingEvent => Some(from.as_any_ref().downcast_ref::<VmEmigratingEvent>()?),
            StructType::VmEndRecordingEvent => Some(from.as_any_ref().downcast_ref::<VmEndRecordingEvent>()?),
            StructType::VmEndReplayingEvent => Some(from.as_any_ref().downcast_ref::<VmEndReplayingEvent>()?),
            StructType::VmFailedMigrateEvent => Some(from.as_any_ref().downcast_ref::<VmFailedMigrateEvent>()?),
            StructType::VmFailedRelayoutEvent => Some(from.as_any_ref().downcast_ref::<VmFailedRelayoutEvent>()?),
            StructType::VmFailedRelayoutOnVmfs2DatastoreEvent => Some(from.as_any_ref().downcast_ref::<VmFailedRelayoutOnVmfs2DatastoreEvent>()?),
            StructType::VmFailedStartingSecondaryEvent => Some(from.as_any_ref().downcast_ref::<VmFailedStartingSecondaryEvent>()?),
            StructType::VmFailedToPowerOffEvent => Some(from.as_any_ref().downcast_ref::<VmFailedToPowerOffEvent>()?),
            StructType::VmFailedToPowerOnEvent => Some(from.as_any_ref().downcast_ref::<VmFailedToPowerOnEvent>()?),
            StructType::VmFailedToRebootGuestEvent => Some(from.as_any_ref().downcast_ref::<VmFailedToRebootGuestEvent>()?),
            StructType::VmFailedToResetEvent => Some(from.as_any_ref().downcast_ref::<VmFailedToResetEvent>()?),
            StructType::VmFailedToShutdownGuestEvent => Some(from.as_any_ref().downcast_ref::<VmFailedToShutdownGuestEvent>()?),
            StructType::VmFailedToStandbyGuestEvent => Some(from.as_any_ref().downcast_ref::<VmFailedToStandbyGuestEvent>()?),
            StructType::VmFailedToSuspendEvent => Some(from.as_any_ref().downcast_ref::<VmFailedToSuspendEvent>()?),
            StructType::VmFailedUpdatingSecondaryConfig => Some(from.as_any_ref().downcast_ref::<VmFailedUpdatingSecondaryConfig>()?),
            StructType::VmFailoverFailed => Some(from.as_any_ref().downcast_ref::<VmFailoverFailed>()?),
            StructType::VmFaultToleranceStateChangedEvent => Some(from.as_any_ref().downcast_ref::<VmFaultToleranceStateChangedEvent>()?),
            StructType::VmFaultToleranceTurnedOffEvent => Some(from.as_any_ref().downcast_ref::<VmFaultToleranceTurnedOffEvent>()?),
            StructType::VmFaultToleranceVmTerminatedEvent => Some(from.as_any_ref().downcast_ref::<VmFaultToleranceVmTerminatedEvent>()?),
            StructType::VmGuestOsCrashedEvent => Some(from.as_any_ref().downcast_ref::<VmGuestOsCrashedEvent>()?),
            StructType::VmGuestRebootEvent => Some(from.as_any_ref().downcast_ref::<VmGuestRebootEvent>()?),
            StructType::VmGuestShutdownEvent => Some(from.as_any_ref().downcast_ref::<VmGuestShutdownEvent>()?),
            StructType::VmGuestStandbyEvent => Some(from.as_any_ref().downcast_ref::<VmGuestStandbyEvent>()?),
            StructType::VmInstanceUuidAssignedEvent => Some(from.as_any_ref().downcast_ref::<VmInstanceUuidAssignedEvent>()?),
            StructType::VmInstanceUuidChangedEvent => Some(from.as_any_ref().downcast_ref::<VmInstanceUuidChangedEvent>()?),
            StructType::VmInstanceUuidConflictEvent => Some(from.as_any_ref().downcast_ref::<VmInstanceUuidConflictEvent>()?),
            StructType::VmMacAssignedEvent => Some(from.as_any_ref().downcast_ref::<VmMacAssignedEvent>()?),
            StructType::VmMacChangedEvent => Some(from.as_any_ref().downcast_ref::<VmMacChangedEvent>()?),
            StructType::VmMacConflictEvent => Some(from.as_any_ref().downcast_ref::<VmMacConflictEvent>()?),
            StructType::VmMaxFtRestartCountReached => Some(from.as_any_ref().downcast_ref::<VmMaxFtRestartCountReached>()?),
            StructType::VmMaxRestartCountReached => Some(from.as_any_ref().downcast_ref::<VmMaxRestartCountReached>()?),
            StructType::VmMessageErrorEvent => Some(from.as_any_ref().downcast_ref::<VmMessageErrorEvent>()?),
            StructType::VmMessageEvent => Some(from.as_any_ref().downcast_ref::<VmMessageEvent>()?),
            StructType::VmMessageWarningEvent => Some(from.as_any_ref().downcast_ref::<VmMessageWarningEvent>()?),
            StructType::VmMigratedEvent => Some(from.as_any_ref().downcast_ref::<VmMigratedEvent>()?),
            StructType::DrsVmMigratedEvent => Some(from.as_any_ref().downcast_ref::<DrsVmMigratedEvent>()?),
            StructType::VmNoCompatibleHostForSecondaryEvent => Some(from.as_any_ref().downcast_ref::<VmNoCompatibleHostForSecondaryEvent>()?),
            StructType::VmNoNetworkAccessEvent => Some(from.as_any_ref().downcast_ref::<VmNoNetworkAccessEvent>()?),
            StructType::VmOrphanedEvent => Some(from.as_any_ref().downcast_ref::<VmOrphanedEvent>()?),
            StructType::VmPoweredOffEvent => Some(from.as_any_ref().downcast_ref::<VmPoweredOffEvent>()?),
            StructType::VmPowerOffOnIsolationEvent => Some(from.as_any_ref().downcast_ref::<VmPowerOffOnIsolationEvent>()?),
            StructType::VmShutdownOnIsolationEvent => Some(from.as_any_ref().downcast_ref::<VmShutdownOnIsolationEvent>()?),
            StructType::VmPoweredOnEvent => Some(from.as_any_ref().downcast_ref::<VmPoweredOnEvent>()?),
            StructType::DrsVmPoweredOnEvent => Some(from.as_any_ref().downcast_ref::<DrsVmPoweredOnEvent>()?),
            StructType::VmRestartedOnAlternateHostEvent => Some(from.as_any_ref().downcast_ref::<VmRestartedOnAlternateHostEvent>()?),
            StructType::VmPoweringOnWithCustomizedDvPortEvent => Some(from.as_any_ref().downcast_ref::<VmPoweringOnWithCustomizedDvPortEvent>()?),
            StructType::VmPrimaryFailoverEvent => Some(from.as_any_ref().downcast_ref::<VmPrimaryFailoverEvent>()?),
            StructType::VmReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<VmReconfiguredEvent>()?),
            StructType::VmRegisteredEvent => Some(from.as_any_ref().downcast_ref::<VmRegisteredEvent>()?),
            StructType::VmRelayoutSuccessfulEvent => Some(from.as_any_ref().downcast_ref::<VmRelayoutSuccessfulEvent>()?),
            StructType::VmRelayoutUpToDateEvent => Some(from.as_any_ref().downcast_ref::<VmRelayoutUpToDateEvent>()?),
            StructType::VmReloadFromPathEvent => Some(from.as_any_ref().downcast_ref::<VmReloadFromPathEvent>()?),
            StructType::VmReloadFromPathFailedEvent => Some(from.as_any_ref().downcast_ref::<VmReloadFromPathFailedEvent>()?),
            StructType::VmRelocateSpecEvent => Some(from.as_any_ref().downcast_ref::<VmRelocateSpecEvent>()?),
            StructType::VmBeingRelocatedEvent => Some(from.as_any_ref().downcast_ref::<VmBeingRelocatedEvent>()?),
            StructType::VmRelocateFailedEvent => Some(from.as_any_ref().downcast_ref::<VmRelocateFailedEvent>()?),
            StructType::VmRelocatedEvent => Some(from.as_any_ref().downcast_ref::<VmRelocatedEvent>()?),
            StructType::VmRemoteConsoleConnectedEvent => Some(from.as_any_ref().downcast_ref::<VmRemoteConsoleConnectedEvent>()?),
            StructType::VmRemoteConsoleDisconnectedEvent => Some(from.as_any_ref().downcast_ref::<VmRemoteConsoleDisconnectedEvent>()?),
            StructType::VmRemovedEvent => Some(from.as_any_ref().downcast_ref::<VmRemovedEvent>()?),
            StructType::VmRenamedEvent => Some(from.as_any_ref().downcast_ref::<VmRenamedEvent>()?),
            StructType::VmRequirementsExceedCurrentEvcModeEvent => Some(from.as_any_ref().downcast_ref::<VmRequirementsExceedCurrentEvcModeEvent>()?),
            StructType::VmResettingEvent => Some(from.as_any_ref().downcast_ref::<VmResettingEvent>()?),
            StructType::VmResourcePoolMovedEvent => Some(from.as_any_ref().downcast_ref::<VmResourcePoolMovedEvent>()?),
            StructType::VmResourceReallocatedEvent => Some(from.as_any_ref().downcast_ref::<VmResourceReallocatedEvent>()?),
            StructType::VmResumingEvent => Some(from.as_any_ref().downcast_ref::<VmResumingEvent>()?),
            StructType::VmSecondaryAddedEvent => Some(from.as_any_ref().downcast_ref::<VmSecondaryAddedEvent>()?),
            StructType::VmSecondaryDisabledBySystemEvent => Some(from.as_any_ref().downcast_ref::<VmSecondaryDisabledBySystemEvent>()?),
            StructType::VmSecondaryDisabledEvent => Some(from.as_any_ref().downcast_ref::<VmSecondaryDisabledEvent>()?),
            StructType::VmSecondaryEnabledEvent => Some(from.as_any_ref().downcast_ref::<VmSecondaryEnabledEvent>()?),
            StructType::VmSecondaryStartedEvent => Some(from.as_any_ref().downcast_ref::<VmSecondaryStartedEvent>()?),
            StructType::VmStartRecordingEvent => Some(from.as_any_ref().downcast_ref::<VmStartRecordingEvent>()?),
            StructType::VmStartReplayingEvent => Some(from.as_any_ref().downcast_ref::<VmStartReplayingEvent>()?),
            StructType::VmStartingEvent => Some(from.as_any_ref().downcast_ref::<VmStartingEvent>()?),
            StructType::VmUnsupportedStartingEvent => Some(from.as_any_ref().downcast_ref::<VmUnsupportedStartingEvent>()?),
            StructType::VmStartingSecondaryEvent => Some(from.as_any_ref().downcast_ref::<VmStartingSecondaryEvent>()?),
            StructType::VmStaticMacConflictEvent => Some(from.as_any_ref().downcast_ref::<VmStaticMacConflictEvent>()?),
            StructType::VmStoppingEvent => Some(from.as_any_ref().downcast_ref::<VmStoppingEvent>()?),
            StructType::VmSuspendedEvent => Some(from.as_any_ref().downcast_ref::<VmSuspendedEvent>()?),
            StructType::VmSuspendingEvent => Some(from.as_any_ref().downcast_ref::<VmSuspendingEvent>()?),
            StructType::VmTimedoutStartingSecondaryEvent => Some(from.as_any_ref().downcast_ref::<VmTimedoutStartingSecondaryEvent>()?),
            StructType::VmUpgradeCompleteEvent => Some(from.as_any_ref().downcast_ref::<VmUpgradeCompleteEvent>()?),
            StructType::VmUpgradeFailedEvent => Some(from.as_any_ref().downcast_ref::<VmUpgradeFailedEvent>()?),
            StructType::VmUpgradingEvent => Some(from.as_any_ref().downcast_ref::<VmUpgradingEvent>()?),
            StructType::VmUuidAssignedEvent => Some(from.as_any_ref().downcast_ref::<VmUuidAssignedEvent>()?),
            StructType::VmUuidChangedEvent => Some(from.as_any_ref().downcast_ref::<VmUuidChangedEvent>()?),
            StructType::VmUuidConflictEvent => Some(from.as_any_ref().downcast_ref::<VmUuidConflictEvent>()?),
            StructType::VmWwnAssignedEvent => Some(from.as_any_ref().downcast_ref::<VmWwnAssignedEvent>()?),
            StructType::VmWwnChangedEvent => Some(from.as_any_ref().downcast_ref::<VmWwnChangedEvent>()?),
            StructType::VmWwnConflictEvent => Some(from.as_any_ref().downcast_ref::<VmWwnConflictEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::Event => Ok(from.as_any_box().downcast::<Event>()?),
            StructType::AlarmEvent => Ok(from.as_any_box().downcast::<AlarmEvent>()?),
            StructType::AlarmAcknowledgedEvent => Ok(from.as_any_box().downcast::<AlarmAcknowledgedEvent>()?),
            StructType::AlarmActionTriggeredEvent => Ok(from.as_any_box().downcast::<AlarmActionTriggeredEvent>()?),
            StructType::AlarmClearedEvent => Ok(from.as_any_box().downcast::<AlarmClearedEvent>()?),
            StructType::AlarmCreatedEvent => Ok(from.as_any_box().downcast::<AlarmCreatedEvent>()?),
            StructType::AlarmEmailCompletedEvent => Ok(from.as_any_box().downcast::<AlarmEmailCompletedEvent>()?),
            StructType::AlarmEmailFailedEvent => Ok(from.as_any_box().downcast::<AlarmEmailFailedEvent>()?),
            StructType::AlarmReconfiguredEvent => Ok(from.as_any_box().downcast::<AlarmReconfiguredEvent>()?),
            StructType::AlarmRemovedEvent => Ok(from.as_any_box().downcast::<AlarmRemovedEvent>()?),
            StructType::AlarmScriptCompleteEvent => Ok(from.as_any_box().downcast::<AlarmScriptCompleteEvent>()?),
            StructType::AlarmScriptFailedEvent => Ok(from.as_any_box().downcast::<AlarmScriptFailedEvent>()?),
            StructType::AlarmSnmpCompletedEvent => Ok(from.as_any_box().downcast::<AlarmSnmpCompletedEvent>()?),
            StructType::AlarmSnmpFailedEvent => Ok(from.as_any_box().downcast::<AlarmSnmpFailedEvent>()?),
            StructType::AlarmStatusChangedEvent => Ok(from.as_any_box().downcast::<AlarmStatusChangedEvent>()?),
            StructType::AuthorizationEvent => Ok(from.as_any_box().downcast::<AuthorizationEvent>()?),
            StructType::PermissionEvent => Ok(from.as_any_box().downcast::<PermissionEvent>()?),
            StructType::PermissionAddedEvent => Ok(from.as_any_box().downcast::<PermissionAddedEvent>()?),
            StructType::PermissionRemovedEvent => Ok(from.as_any_box().downcast::<PermissionRemovedEvent>()?),
            StructType::PermissionUpdatedEvent => Ok(from.as_any_box().downcast::<PermissionUpdatedEvent>()?),
            StructType::RoleEvent => Ok(from.as_any_box().downcast::<RoleEvent>()?),
            StructType::RoleAddedEvent => Ok(from.as_any_box().downcast::<RoleAddedEvent>()?),
            StructType::RoleRemovedEvent => Ok(from.as_any_box().downcast::<RoleRemovedEvent>()?),
            StructType::RoleUpdatedEvent => Ok(from.as_any_box().downcast::<RoleUpdatedEvent>()?),
            StructType::ClusterEvent => Ok(from.as_any_box().downcast::<ClusterEvent>()?),
            StructType::ClusterComplianceCheckedEvent => Ok(from.as_any_box().downcast::<ClusterComplianceCheckedEvent>()?),
            StructType::ClusterCreatedEvent => Ok(from.as_any_box().downcast::<ClusterCreatedEvent>()?),
            StructType::ClusterDestroyedEvent => Ok(from.as_any_box().downcast::<ClusterDestroyedEvent>()?),
            StructType::ClusterOvercommittedEvent => Ok(from.as_any_box().downcast::<ClusterOvercommittedEvent>()?),
            StructType::HostOvercommittedEvent => Ok(from.as_any_box().downcast::<HostOvercommittedEvent>()?),
            StructType::ClusterReconfiguredEvent => Ok(from.as_any_box().downcast::<ClusterReconfiguredEvent>()?),
            StructType::ClusterStatusChangedEvent => Ok(from.as_any_box().downcast::<ClusterStatusChangedEvent>()?),
            StructType::HostStatusChangedEvent => Ok(from.as_any_box().downcast::<HostStatusChangedEvent>()?),
            StructType::DasAdmissionControlDisabledEvent => Ok(from.as_any_box().downcast::<DasAdmissionControlDisabledEvent>()?),
            StructType::DasAdmissionControlEnabledEvent => Ok(from.as_any_box().downcast::<DasAdmissionControlEnabledEvent>()?),
            StructType::DasAgentFoundEvent => Ok(from.as_any_box().downcast::<DasAgentFoundEvent>()?),
            StructType::DasAgentUnavailableEvent => Ok(from.as_any_box().downcast::<DasAgentUnavailableEvent>()?),
            StructType::DasClusterIsolatedEvent => Ok(from.as_any_box().downcast::<DasClusterIsolatedEvent>()?),
            StructType::DasDisabledEvent => Ok(from.as_any_box().downcast::<DasDisabledEvent>()?),
            StructType::DasEnabledEvent => Ok(from.as_any_box().downcast::<DasEnabledEvent>()?),
            StructType::DasHostFailedEvent => Ok(from.as_any_box().downcast::<DasHostFailedEvent>()?),
            StructType::DasHostIsolatedEvent => Ok(from.as_any_box().downcast::<DasHostIsolatedEvent>()?),
            StructType::DrsDisabledEvent => Ok(from.as_any_box().downcast::<DrsDisabledEvent>()?),
            StructType::DrsEnabledEvent => Ok(from.as_any_box().downcast::<DrsEnabledEvent>()?),
            StructType::DrsInvocationFailedEvent => Ok(from.as_any_box().downcast::<DrsInvocationFailedEvent>()?),
            StructType::DrsRecoveredFromFailureEvent => Ok(from.as_any_box().downcast::<DrsRecoveredFromFailureEvent>()?),
            StructType::FailoverLevelRestored => Ok(from.as_any_box().downcast::<FailoverLevelRestored>()?),
            StructType::HostMonitoringStateChangedEvent => Ok(from.as_any_box().downcast::<HostMonitoringStateChangedEvent>()?),
            StructType::InsufficientFailoverResourcesEvent => Ok(from.as_any_box().downcast::<InsufficientFailoverResourcesEvent>()?),
            StructType::VmHealthMonitoringStateChangedEvent => Ok(from.as_any_box().downcast::<VmHealthMonitoringStateChangedEvent>()?),
            StructType::CustomFieldEvent => Ok(from.as_any_box().downcast::<CustomFieldEvent>()?),
            StructType::CustomFieldDefEvent => Ok(from.as_any_box().downcast::<CustomFieldDefEvent>()?),
            StructType::CustomFieldDefAddedEvent => Ok(from.as_any_box().downcast::<CustomFieldDefAddedEvent>()?),
            StructType::CustomFieldDefRemovedEvent => Ok(from.as_any_box().downcast::<CustomFieldDefRemovedEvent>()?),
            StructType::CustomFieldDefRenamedEvent => Ok(from.as_any_box().downcast::<CustomFieldDefRenamedEvent>()?),
            StructType::CustomFieldValueChangedEvent => Ok(from.as_any_box().downcast::<CustomFieldValueChangedEvent>()?),
            StructType::DvPortgroupEvent => Ok(from.as_any_box().downcast::<DvPortgroupEvent>()?),
            StructType::DvPortgroupCreatedEvent => Ok(from.as_any_box().downcast::<DvPortgroupCreatedEvent>()?),
            StructType::DvPortgroupDestroyedEvent => Ok(from.as_any_box().downcast::<DvPortgroupDestroyedEvent>()?),
            StructType::DvPortgroupReconfiguredEvent => Ok(from.as_any_box().downcast::<DvPortgroupReconfiguredEvent>()?),
            StructType::DvPortgroupRenamedEvent => Ok(from.as_any_box().downcast::<DvPortgroupRenamedEvent>()?),
            StructType::DvpgImportEvent => Ok(from.as_any_box().downcast::<DvpgImportEvent>()?),
            StructType::DvpgRestoreEvent => Ok(from.as_any_box().downcast::<DvpgRestoreEvent>()?),
            StructType::DatacenterEvent => Ok(from.as_any_box().downcast::<DatacenterEvent>()?),
            StructType::DatacenterCreatedEvent => Ok(from.as_any_box().downcast::<DatacenterCreatedEvent>()?),
            StructType::DatacenterRenamedEvent => Ok(from.as_any_box().downcast::<DatacenterRenamedEvent>()?),
            StructType::DatastoreEvent => Ok(from.as_any_box().downcast::<DatastoreEvent>()?),
            StructType::DatastoreCapacityIncreasedEvent => Ok(from.as_any_box().downcast::<DatastoreCapacityIncreasedEvent>()?),
            StructType::DatastoreDestroyedEvent => Ok(from.as_any_box().downcast::<DatastoreDestroyedEvent>()?),
            StructType::DatastoreDuplicatedEvent => Ok(from.as_any_box().downcast::<DatastoreDuplicatedEvent>()?),
            StructType::DatastoreFileEvent => Ok(from.as_any_box().downcast::<DatastoreFileEvent>()?),
            StructType::DatastoreFileCopiedEvent => Ok(from.as_any_box().downcast::<DatastoreFileCopiedEvent>()?),
            StructType::DatastoreFileDeletedEvent => Ok(from.as_any_box().downcast::<DatastoreFileDeletedEvent>()?),
            StructType::DatastoreFileMovedEvent => Ok(from.as_any_box().downcast::<DatastoreFileMovedEvent>()?),
            StructType::DatastoreIormReconfiguredEvent => Ok(from.as_any_box().downcast::<DatastoreIormReconfiguredEvent>()?),
            StructType::DatastoreRenamedEvent => Ok(from.as_any_box().downcast::<DatastoreRenamedEvent>()?),
            StructType::NonViWorkloadDetectedOnDatastoreEvent => Ok(from.as_any_box().downcast::<NonViWorkloadDetectedOnDatastoreEvent>()?),
            StructType::DvsEvent => Ok(from.as_any_box().downcast::<DvsEvent>()?),
            StructType::DvsCreatedEvent => Ok(from.as_any_box().downcast::<DvsCreatedEvent>()?),
            StructType::DvsDestroyedEvent => Ok(from.as_any_box().downcast::<DvsDestroyedEvent>()?),
            StructType::DvsHostBackInSyncEvent => Ok(from.as_any_box().downcast::<DvsHostBackInSyncEvent>()?),
            StructType::DvsHostJoinedEvent => Ok(from.as_any_box().downcast::<DvsHostJoinedEvent>()?),
            StructType::DvsHostLeftEvent => Ok(from.as_any_box().downcast::<DvsHostLeftEvent>()?),
            StructType::DvsHostStatusUpdated => Ok(from.as_any_box().downcast::<DvsHostStatusUpdated>()?),
            StructType::DvsHostWentOutOfSyncEvent => Ok(from.as_any_box().downcast::<DvsHostWentOutOfSyncEvent>()?),
            StructType::DvsImportEvent => Ok(from.as_any_box().downcast::<DvsImportEvent>()?),
            StructType::DvsMergedEvent => Ok(from.as_any_box().downcast::<DvsMergedEvent>()?),
            StructType::DvsPortBlockedEvent => Ok(from.as_any_box().downcast::<DvsPortBlockedEvent>()?),
            StructType::DvsPortConnectedEvent => Ok(from.as_any_box().downcast::<DvsPortConnectedEvent>()?),
            StructType::DvsPortCreatedEvent => Ok(from.as_any_box().downcast::<DvsPortCreatedEvent>()?),
            StructType::DvsPortDeletedEvent => Ok(from.as_any_box().downcast::<DvsPortDeletedEvent>()?),
            StructType::DvsPortDisconnectedEvent => Ok(from.as_any_box().downcast::<DvsPortDisconnectedEvent>()?),
            StructType::DvsPortEnteredPassthruEvent => Ok(from.as_any_box().downcast::<DvsPortEnteredPassthruEvent>()?),
            StructType::DvsPortExitedPassthruEvent => Ok(from.as_any_box().downcast::<DvsPortExitedPassthruEvent>()?),
            StructType::DvsPortJoinPortgroupEvent => Ok(from.as_any_box().downcast::<DvsPortJoinPortgroupEvent>()?),
            StructType::DvsPortLeavePortgroupEvent => Ok(from.as_any_box().downcast::<DvsPortLeavePortgroupEvent>()?),
            StructType::DvsPortLinkDownEvent => Ok(from.as_any_box().downcast::<DvsPortLinkDownEvent>()?),
            StructType::DvsPortLinkUpEvent => Ok(from.as_any_box().downcast::<DvsPortLinkUpEvent>()?),
            StructType::DvsPortReconfiguredEvent => Ok(from.as_any_box().downcast::<DvsPortReconfiguredEvent>()?),
            StructType::DvsPortRuntimeChangeEvent => Ok(from.as_any_box().downcast::<DvsPortRuntimeChangeEvent>()?),
            StructType::DvsPortUnblockedEvent => Ok(from.as_any_box().downcast::<DvsPortUnblockedEvent>()?),
            StructType::DvsPortVendorSpecificStateChangeEvent => Ok(from.as_any_box().downcast::<DvsPortVendorSpecificStateChangeEvent>()?),
            StructType::DvsReconfiguredEvent => Ok(from.as_any_box().downcast::<DvsReconfiguredEvent>()?),
            StructType::DvsRenamedEvent => Ok(from.as_any_box().downcast::<DvsRenamedEvent>()?),
            StructType::DvsRestoreEvent => Ok(from.as_any_box().downcast::<DvsRestoreEvent>()?),
            StructType::DvsUpgradeAvailableEvent => Ok(from.as_any_box().downcast::<DvsUpgradeAvailableEvent>()?),
            StructType::DvsUpgradeInProgressEvent => Ok(from.as_any_box().downcast::<DvsUpgradeInProgressEvent>()?),
            StructType::DvsUpgradeRejectedEvent => Ok(from.as_any_box().downcast::<DvsUpgradeRejectedEvent>()?),
            StructType::DvsUpgradedEvent => Ok(from.as_any_box().downcast::<DvsUpgradedEvent>()?),
            StructType::HostLocalPortCreatedEvent => Ok(from.as_any_box().downcast::<HostLocalPortCreatedEvent>()?),
            StructType::OutOfSyncDvsHost => Ok(from.as_any_box().downcast::<OutOfSyncDvsHost>()?),
            StructType::RecoveryEvent => Ok(from.as_any_box().downcast::<RecoveryEvent>()?),
            StructType::RollbackEvent => Ok(from.as_any_box().downcast::<RollbackEvent>()?),
            StructType::VmVnicPoolReservationViolationClearEvent => Ok(from.as_any_box().downcast::<VmVnicPoolReservationViolationClearEvent>()?),
            StructType::VmVnicPoolReservationViolationRaiseEvent => Ok(from.as_any_box().downcast::<VmVnicPoolReservationViolationRaiseEvent>()?),
            StructType::EventEx => Ok(from.as_any_box().downcast::<EventEx>()?),
            StructType::GeneralEvent => Ok(from.as_any_box().downcast::<GeneralEvent>()?),
            StructType::ExtendedEvent => Ok(from.as_any_box().downcast::<ExtendedEvent>()?),
            StructType::GeneralHostErrorEvent => Ok(from.as_any_box().downcast::<GeneralHostErrorEvent>()?),
            StructType::GeneralHostInfoEvent => Ok(from.as_any_box().downcast::<GeneralHostInfoEvent>()?),
            StructType::GeneralHostWarningEvent => Ok(from.as_any_box().downcast::<GeneralHostWarningEvent>()?),
            StructType::GeneralUserEvent => Ok(from.as_any_box().downcast::<GeneralUserEvent>()?),
            StructType::GeneralVmErrorEvent => Ok(from.as_any_box().downcast::<GeneralVmErrorEvent>()?),
            StructType::GeneralVmInfoEvent => Ok(from.as_any_box().downcast::<GeneralVmInfoEvent>()?),
            StructType::GeneralVmWarningEvent => Ok(from.as_any_box().downcast::<GeneralVmWarningEvent>()?),
            StructType::HealthStatusChangedEvent => Ok(from.as_any_box().downcast::<HealthStatusChangedEvent>()?),
            StructType::HostEvent => Ok(from.as_any_box().downcast::<HostEvent>()?),
            StructType::AccountCreatedEvent => Ok(from.as_any_box().downcast::<AccountCreatedEvent>()?),
            StructType::AccountRemovedEvent => Ok(from.as_any_box().downcast::<AccountRemovedEvent>()?),
            StructType::AccountUpdatedEvent => Ok(from.as_any_box().downcast::<AccountUpdatedEvent>()?),
            StructType::AdminPasswordNotChangedEvent => Ok(from.as_any_box().downcast::<AdminPasswordNotChangedEvent>()?),
            StructType::CanceledHostOperationEvent => Ok(from.as_any_box().downcast::<CanceledHostOperationEvent>()?),
            StructType::DatastoreDiscoveredEvent => Ok(from.as_any_box().downcast::<DatastoreDiscoveredEvent>()?),
            StructType::DatastorePrincipalConfigured => Ok(from.as_any_box().downcast::<DatastorePrincipalConfigured>()?),
            StructType::DatastoreRemovedOnHostEvent => Ok(from.as_any_box().downcast::<DatastoreRemovedOnHostEvent>()?),
            StructType::DatastoreRenamedOnHostEvent => Ok(from.as_any_box().downcast::<DatastoreRenamedOnHostEvent>()?),
            StructType::DrsResourceConfigureFailedEvent => Ok(from.as_any_box().downcast::<DrsResourceConfigureFailedEvent>()?),
            StructType::DrsResourceConfigureSyncedEvent => Ok(from.as_any_box().downcast::<DrsResourceConfigureSyncedEvent>()?),
            StructType::DuplicateIpDetectedEvent => Ok(from.as_any_box().downcast::<DuplicateIpDetectedEvent>()?),
            StructType::DvsHealthStatusChangeEvent => Ok(from.as_any_box().downcast::<DvsHealthStatusChangeEvent>()?),
            StructType::MtuMatchEvent => Ok(from.as_any_box().downcast::<MtuMatchEvent>()?),
            StructType::MtuMismatchEvent => Ok(from.as_any_box().downcast::<MtuMismatchEvent>()?),
            StructType::TeamingMatchEvent => Ok(from.as_any_box().downcast::<TeamingMatchEvent>()?),
            StructType::TeamingMisMatchEvent => Ok(from.as_any_box().downcast::<TeamingMisMatchEvent>()?),
            StructType::UplinkPortMtuNotSupportEvent => Ok(from.as_any_box().downcast::<UplinkPortMtuNotSupportEvent>()?),
            StructType::UplinkPortMtuSupportEvent => Ok(from.as_any_box().downcast::<UplinkPortMtuSupportEvent>()?),
            StructType::UplinkPortVlanTrunkedEvent => Ok(from.as_any_box().downcast::<UplinkPortVlanTrunkedEvent>()?),
            StructType::UplinkPortVlanUntrunkedEvent => Ok(from.as_any_box().downcast::<UplinkPortVlanUntrunkedEvent>()?),
            StructType::EnteredMaintenanceModeEvent => Ok(from.as_any_box().downcast::<EnteredMaintenanceModeEvent>()?),
            StructType::EnteredStandbyModeEvent => Ok(from.as_any_box().downcast::<EnteredStandbyModeEvent>()?),
            StructType::DrsEnteredStandbyModeEvent => Ok(from.as_any_box().downcast::<DrsEnteredStandbyModeEvent>()?),
            StructType::EnteringMaintenanceModeEvent => Ok(from.as_any_box().downcast::<EnteringMaintenanceModeEvent>()?),
            StructType::EnteringStandbyModeEvent => Ok(from.as_any_box().downcast::<EnteringStandbyModeEvent>()?),
            StructType::DrsEnteringStandbyModeEvent => Ok(from.as_any_box().downcast::<DrsEnteringStandbyModeEvent>()?),
            StructType::ExitMaintenanceModeEvent => Ok(from.as_any_box().downcast::<ExitMaintenanceModeEvent>()?),
            StructType::ExitStandbyModeFailedEvent => Ok(from.as_any_box().downcast::<ExitStandbyModeFailedEvent>()?),
            StructType::DrsExitStandbyModeFailedEvent => Ok(from.as_any_box().downcast::<DrsExitStandbyModeFailedEvent>()?),
            StructType::ExitedStandbyModeEvent => Ok(from.as_any_box().downcast::<ExitedStandbyModeEvent>()?),
            StructType::DrsExitedStandbyModeEvent => Ok(from.as_any_box().downcast::<DrsExitedStandbyModeEvent>()?),
            StructType::ExitingStandbyModeEvent => Ok(from.as_any_box().downcast::<ExitingStandbyModeEvent>()?),
            StructType::DrsExitingStandbyModeEvent => Ok(from.as_any_box().downcast::<DrsExitingStandbyModeEvent>()?),
            StructType::GhostDvsProxySwitchDetectedEvent => Ok(from.as_any_box().downcast::<GhostDvsProxySwitchDetectedEvent>()?),
            StructType::GhostDvsProxySwitchRemovedEvent => Ok(from.as_any_box().downcast::<GhostDvsProxySwitchRemovedEvent>()?),
            StructType::HostAddFailedEvent => Ok(from.as_any_box().downcast::<HostAddFailedEvent>()?),
            StructType::HostAddedEvent => Ok(from.as_any_box().downcast::<HostAddedEvent>()?),
            StructType::HostAdminDisableEvent => Ok(from.as_any_box().downcast::<HostAdminDisableEvent>()?),
            StructType::HostAdminEnableEvent => Ok(from.as_any_box().downcast::<HostAdminEnableEvent>()?),
            StructType::HostCnxFailedAccountFailedEvent => Ok(from.as_any_box().downcast::<HostCnxFailedAccountFailedEvent>()?),
            StructType::HostCnxFailedAlreadyManagedEvent => Ok(from.as_any_box().downcast::<HostCnxFailedAlreadyManagedEvent>()?),
            StructType::HostCnxFailedBadCcagentEvent => Ok(from.as_any_box().downcast::<HostCnxFailedBadCcagentEvent>()?),
            StructType::HostCnxFailedBadUsernameEvent => Ok(from.as_any_box().downcast::<HostCnxFailedBadUsernameEvent>()?),
            StructType::HostCnxFailedBadVersionEvent => Ok(from.as_any_box().downcast::<HostCnxFailedBadVersionEvent>()?),
            StructType::HostCnxFailedCcagentUpgradeEvent => Ok(from.as_any_box().downcast::<HostCnxFailedCcagentUpgradeEvent>()?),
            StructType::HostCnxFailedEvent => Ok(from.as_any_box().downcast::<HostCnxFailedEvent>()?),
            StructType::HostCnxFailedNetworkErrorEvent => Ok(from.as_any_box().downcast::<HostCnxFailedNetworkErrorEvent>()?),
            StructType::HostCnxFailedNoAccessEvent => Ok(from.as_any_box().downcast::<HostCnxFailedNoAccessEvent>()?),
            StructType::HostCnxFailedNoConnectionEvent => Ok(from.as_any_box().downcast::<HostCnxFailedNoConnectionEvent>()?),
            StructType::HostCnxFailedNoLicenseEvent => Ok(from.as_any_box().downcast::<HostCnxFailedNoLicenseEvent>()?),
            StructType::HostCnxFailedNotFoundEvent => Ok(from.as_any_box().downcast::<HostCnxFailedNotFoundEvent>()?),
            StructType::HostCnxFailedTimeoutEvent => Ok(from.as_any_box().downcast::<HostCnxFailedTimeoutEvent>()?),
            StructType::HostComplianceCheckedEvent => Ok(from.as_any_box().downcast::<HostComplianceCheckedEvent>()?),
            StructType::HostCompliantEvent => Ok(from.as_any_box().downcast::<HostCompliantEvent>()?),
            StructType::HostConfigAppliedEvent => Ok(from.as_any_box().downcast::<HostConfigAppliedEvent>()?),
            StructType::HostConnectedEvent => Ok(from.as_any_box().downcast::<HostConnectedEvent>()?),
            StructType::HostConnectionLostEvent => Ok(from.as_any_box().downcast::<HostConnectionLostEvent>()?),
            StructType::HostDasDisabledEvent => Ok(from.as_any_box().downcast::<HostDasDisabledEvent>()?),
            StructType::HostDasDisablingEvent => Ok(from.as_any_box().downcast::<HostDasDisablingEvent>()?),
            StructType::HostDasEnabledEvent => Ok(from.as_any_box().downcast::<HostDasEnabledEvent>()?),
            StructType::HostDasEnablingEvent => Ok(from.as_any_box().downcast::<HostDasEnablingEvent>()?),
            StructType::HostDasErrorEvent => Ok(from.as_any_box().downcast::<HostDasErrorEvent>()?),
            StructType::HostDasEvent => Ok(from.as_any_box().downcast::<HostDasEvent>()?),
            StructType::HostExtraNetworksEvent => Ok(from.as_any_box().downcast::<HostExtraNetworksEvent>()?),
            StructType::HostIsolationIpPingFailedEvent => Ok(from.as_any_box().downcast::<HostIsolationIpPingFailedEvent>()?),
            StructType::HostMissingNetworksEvent => Ok(from.as_any_box().downcast::<HostMissingNetworksEvent>()?),
            StructType::HostNoAvailableNetworksEvent => Ok(from.as_any_box().downcast::<HostNoAvailableNetworksEvent>()?),
            StructType::HostNoHaEnabledPortGroupsEvent => Ok(from.as_any_box().downcast::<HostNoHaEnabledPortGroupsEvent>()?),
            StructType::HostNoRedundantManagementNetworkEvent => Ok(from.as_any_box().downcast::<HostNoRedundantManagementNetworkEvent>()?),
            StructType::HostNotInClusterEvent => Ok(from.as_any_box().downcast::<HostNotInClusterEvent>()?),
            StructType::HostPrimaryAgentNotShortNameEvent => Ok(from.as_any_box().downcast::<HostPrimaryAgentNotShortNameEvent>()?),
            StructType::HostShortNameInconsistentEvent => Ok(from.as_any_box().downcast::<HostShortNameInconsistentEvent>()?),
            StructType::HostDasOkEvent => Ok(from.as_any_box().downcast::<HostDasOkEvent>()?),
            StructType::HostDisconnectedEvent => Ok(from.as_any_box().downcast::<HostDisconnectedEvent>()?),
            StructType::HostEnableAdminFailedEvent => Ok(from.as_any_box().downcast::<HostEnableAdminFailedEvent>()?),
            StructType::HostGetShortNameFailedEvent => Ok(from.as_any_box().downcast::<HostGetShortNameFailedEvent>()?),
            StructType::HostInAuditModeEvent => Ok(from.as_any_box().downcast::<HostInAuditModeEvent>()?),
            StructType::HostIpChangedEvent => Ok(from.as_any_box().downcast::<HostIpChangedEvent>()?),
            StructType::HostIpInconsistentEvent => Ok(from.as_any_box().downcast::<HostIpInconsistentEvent>()?),
            StructType::HostIpToShortNameFailedEvent => Ok(from.as_any_box().downcast::<HostIpToShortNameFailedEvent>()?),
            StructType::HostNonCompliantEvent => Ok(from.as_any_box().downcast::<HostNonCompliantEvent>()?),
            StructType::HostProfileAppliedEvent => Ok(from.as_any_box().downcast::<HostProfileAppliedEvent>()?),
            StructType::HostReconnectionFailedEvent => Ok(from.as_any_box().downcast::<HostReconnectionFailedEvent>()?),
            StructType::HostRemovedEvent => Ok(from.as_any_box().downcast::<HostRemovedEvent>()?),
            StructType::HostShortNameToIpFailedEvent => Ok(from.as_any_box().downcast::<HostShortNameToIpFailedEvent>()?),
            StructType::HostShutdownEvent => Ok(from.as_any_box().downcast::<HostShutdownEvent>()?),
            StructType::HostSpecificationChangedEvent => Ok(from.as_any_box().downcast::<HostSpecificationChangedEvent>()?),
            StructType::HostSpecificationRequireEvent => Ok(from.as_any_box().downcast::<HostSpecificationRequireEvent>()?),
            StructType::HostSpecificationUpdateEvent => Ok(from.as_any_box().downcast::<HostSpecificationUpdateEvent>()?),
            StructType::HostSubSpecificationDeleteEvent => Ok(from.as_any_box().downcast::<HostSubSpecificationDeleteEvent>()?),
            StructType::HostSubSpecificationUpdateEvent => Ok(from.as_any_box().downcast::<HostSubSpecificationUpdateEvent>()?),
            StructType::HostSyncFailedEvent => Ok(from.as_any_box().downcast::<HostSyncFailedEvent>()?),
            StructType::HostUpgradeFailedEvent => Ok(from.as_any_box().downcast::<HostUpgradeFailedEvent>()?),
            StructType::HostUserWorldSwapNotEnabledEvent => Ok(from.as_any_box().downcast::<HostUserWorldSwapNotEnabledEvent>()?),
            StructType::HostVnicConnectedToCustomizedDvPortEvent => Ok(from.as_any_box().downcast::<HostVnicConnectedToCustomizedDvPortEvent>()?),
            StructType::HostWwnChangedEvent => Ok(from.as_any_box().downcast::<HostWwnChangedEvent>()?),
            StructType::HostWwnConflictEvent => Ok(from.as_any_box().downcast::<HostWwnConflictEvent>()?),
            StructType::LocalDatastoreCreatedEvent => Ok(from.as_any_box().downcast::<LocalDatastoreCreatedEvent>()?),
            StructType::LocalTsmEnabledEvent => Ok(from.as_any_box().downcast::<LocalTsmEnabledEvent>()?),
            StructType::NasDatastoreCreatedEvent => Ok(from.as_any_box().downcast::<NasDatastoreCreatedEvent>()?),
            StructType::NoDatastoresConfiguredEvent => Ok(from.as_any_box().downcast::<NoDatastoresConfiguredEvent>()?),
            StructType::RemoteTsmEnabledEvent => Ok(from.as_any_box().downcast::<RemoteTsmEnabledEvent>()?),
            StructType::TimedOutHostOperationEvent => Ok(from.as_any_box().downcast::<TimedOutHostOperationEvent>()?),
            StructType::UpdatedAgentBeingRestartedEvent => Ok(from.as_any_box().downcast::<UpdatedAgentBeingRestartedEvent>()?),
            StructType::UserAssignedToGroup => Ok(from.as_any_box().downcast::<UserAssignedToGroup>()?),
            StructType::UserPasswordChanged => Ok(from.as_any_box().downcast::<UserPasswordChanged>()?),
            StructType::UserUnassignedFromGroup => Ok(from.as_any_box().downcast::<UserUnassignedFromGroup>()?),
            StructType::VmfsDatastoreCreatedEvent => Ok(from.as_any_box().downcast::<VmfsDatastoreCreatedEvent>()?),
            StructType::VmfsDatastoreExpandedEvent => Ok(from.as_any_box().downcast::<VmfsDatastoreExpandedEvent>()?),
            StructType::VmfsDatastoreExtendedEvent => Ok(from.as_any_box().downcast::<VmfsDatastoreExtendedEvent>()?),
            StructType::VcAgentUninstallFailedEvent => Ok(from.as_any_box().downcast::<VcAgentUninstallFailedEvent>()?),
            StructType::VcAgentUninstalledEvent => Ok(from.as_any_box().downcast::<VcAgentUninstalledEvent>()?),
            StructType::VcAgentUpgradeFailedEvent => Ok(from.as_any_box().downcast::<VcAgentUpgradeFailedEvent>()?),
            StructType::VcAgentUpgradedEvent => Ok(from.as_any_box().downcast::<VcAgentUpgradedEvent>()?),
            StructType::VimAccountPasswordChangedEvent => Ok(from.as_any_box().downcast::<VimAccountPasswordChangedEvent>()?),
            StructType::IScsiBootFailureEvent => Ok(from.as_any_box().downcast::<IScsiBootFailureEvent>()?),
            StructType::HostInventoryUnreadableEvent => Ok(from.as_any_box().downcast::<HostInventoryUnreadableEvent>()?),
            StructType::LicenseEvent => Ok(from.as_any_box().downcast::<LicenseEvent>()?),
            StructType::AllVirtualMachinesLicensedEvent => Ok(from.as_any_box().downcast::<AllVirtualMachinesLicensedEvent>()?),
            StructType::HostInventoryFullEvent => Ok(from.as_any_box().downcast::<HostInventoryFullEvent>()?),
            StructType::HostLicenseExpiredEvent => Ok(from.as_any_box().downcast::<HostLicenseExpiredEvent>()?),
            StructType::IncorrectHostInformationEvent => Ok(from.as_any_box().downcast::<IncorrectHostInformationEvent>()?),
            StructType::InvalidEditionEvent => Ok(from.as_any_box().downcast::<InvalidEditionEvent>()?),
            StructType::LicenseNonComplianceEvent => Ok(from.as_any_box().downcast::<LicenseNonComplianceEvent>()?),
            StructType::LicenseRestrictedEvent => Ok(from.as_any_box().downcast::<LicenseRestrictedEvent>()?),
            StructType::LicenseServerAvailableEvent => Ok(from.as_any_box().downcast::<LicenseServerAvailableEvent>()?),
            StructType::LicenseServerUnavailableEvent => Ok(from.as_any_box().downcast::<LicenseServerUnavailableEvent>()?),
            StructType::NoLicenseEvent => Ok(from.as_any_box().downcast::<NoLicenseEvent>()?),
            StructType::ServerLicenseExpiredEvent => Ok(from.as_any_box().downcast::<ServerLicenseExpiredEvent>()?),
            StructType::UnlicensedVirtualMachinesEvent => Ok(from.as_any_box().downcast::<UnlicensedVirtualMachinesEvent>()?),
            StructType::UnlicensedVirtualMachinesFoundEvent => Ok(from.as_any_box().downcast::<UnlicensedVirtualMachinesFoundEvent>()?),
            StructType::VMotionLicenseExpiredEvent => Ok(from.as_any_box().downcast::<VMotionLicenseExpiredEvent>()?),
            StructType::LicenseExpiredEvent => Ok(from.as_any_box().downcast::<LicenseExpiredEvent>()?),
            StructType::LockerMisconfiguredEvent => Ok(from.as_any_box().downcast::<LockerMisconfiguredEvent>()?),
            StructType::LockerReconfiguredEvent => Ok(from.as_any_box().downcast::<LockerReconfiguredEvent>()?),
            StructType::NetworkRollbackEvent => Ok(from.as_any_box().downcast::<NetworkRollbackEvent>()?),
            StructType::ProfileEvent => Ok(from.as_any_box().downcast::<ProfileEvent>()?),
            StructType::ProfileAssociatedEvent => Ok(from.as_any_box().downcast::<ProfileAssociatedEvent>()?),
            StructType::ProfileChangedEvent => Ok(from.as_any_box().downcast::<ProfileChangedEvent>()?),
            StructType::ProfileCreatedEvent => Ok(from.as_any_box().downcast::<ProfileCreatedEvent>()?),
            StructType::ProfileDissociatedEvent => Ok(from.as_any_box().downcast::<ProfileDissociatedEvent>()?),
            StructType::ProfileReferenceHostChangedEvent => Ok(from.as_any_box().downcast::<ProfileReferenceHostChangedEvent>()?),
            StructType::ProfileRemovedEvent => Ok(from.as_any_box().downcast::<ProfileRemovedEvent>()?),
            StructType::ResourcePoolEvent => Ok(from.as_any_box().downcast::<ResourcePoolEvent>()?),
            StructType::ResourcePoolCreatedEvent => Ok(from.as_any_box().downcast::<ResourcePoolCreatedEvent>()?),
            StructType::ResourcePoolDestroyedEvent => Ok(from.as_any_box().downcast::<ResourcePoolDestroyedEvent>()?),
            StructType::ResourcePoolMovedEvent => Ok(from.as_any_box().downcast::<ResourcePoolMovedEvent>()?),
            StructType::ResourcePoolReconfiguredEvent => Ok(from.as_any_box().downcast::<ResourcePoolReconfiguredEvent>()?),
            StructType::ResourceViolatedEvent => Ok(from.as_any_box().downcast::<ResourceViolatedEvent>()?),
            StructType::ScheduledTaskEvent => Ok(from.as_any_box().downcast::<ScheduledTaskEvent>()?),
            StructType::ScheduledTaskCompletedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskCompletedEvent>()?),
            StructType::ScheduledTaskCreatedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskCreatedEvent>()?),
            StructType::ScheduledTaskEmailCompletedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskEmailCompletedEvent>()?),
            StructType::ScheduledTaskEmailFailedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskEmailFailedEvent>()?),
            StructType::ScheduledTaskFailedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskFailedEvent>()?),
            StructType::ScheduledTaskReconfiguredEvent => Ok(from.as_any_box().downcast::<ScheduledTaskReconfiguredEvent>()?),
            StructType::ScheduledTaskRemovedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskRemovedEvent>()?),
            StructType::ScheduledTaskStartedEvent => Ok(from.as_any_box().downcast::<ScheduledTaskStartedEvent>()?),
            StructType::SessionEvent => Ok(from.as_any_box().downcast::<SessionEvent>()?),
            StructType::AlreadyAuthenticatedSessionEvent => Ok(from.as_any_box().downcast::<AlreadyAuthenticatedSessionEvent>()?),
            StructType::BadUsernameSessionEvent => Ok(from.as_any_box().downcast::<BadUsernameSessionEvent>()?),
            StructType::GlobalMessageChangedEvent => Ok(from.as_any_box().downcast::<GlobalMessageChangedEvent>()?),
            StructType::NoAccessUserEvent => Ok(from.as_any_box().downcast::<NoAccessUserEvent>()?),
            StructType::ServerStartedSessionEvent => Ok(from.as_any_box().downcast::<ServerStartedSessionEvent>()?),
            StructType::SessionTerminatedEvent => Ok(from.as_any_box().downcast::<SessionTerminatedEvent>()?),
            StructType::UserLoginSessionEvent => Ok(from.as_any_box().downcast::<UserLoginSessionEvent>()?),
            StructType::UserLogoutSessionEvent => Ok(from.as_any_box().downcast::<UserLogoutSessionEvent>()?),
            StructType::TaskEvent => Ok(from.as_any_box().downcast::<TaskEvent>()?),
            StructType::TaskTimeoutEvent => Ok(from.as_any_box().downcast::<TaskTimeoutEvent>()?),
            StructType::TemplateUpgradeEvent => Ok(from.as_any_box().downcast::<TemplateUpgradeEvent>()?),
            StructType::TemplateBeingUpgradedEvent => Ok(from.as_any_box().downcast::<TemplateBeingUpgradedEvent>()?),
            StructType::TemplateUpgradeFailedEvent => Ok(from.as_any_box().downcast::<TemplateUpgradeFailedEvent>()?),
            StructType::TemplateUpgradedEvent => Ok(from.as_any_box().downcast::<TemplateUpgradedEvent>()?),
            StructType::UpgradeEvent => Ok(from.as_any_box().downcast::<UpgradeEvent>()?),
            StructType::ErrorUpgradeEvent => Ok(from.as_any_box().downcast::<ErrorUpgradeEvent>()?),
            StructType::InfoUpgradeEvent => Ok(from.as_any_box().downcast::<InfoUpgradeEvent>()?),
            StructType::UserUpgradeEvent => Ok(from.as_any_box().downcast::<UserUpgradeEvent>()?),
            StructType::WarningUpgradeEvent => Ok(from.as_any_box().downcast::<WarningUpgradeEvent>()?),
            StructType::VmEvent => Ok(from.as_any_box().downcast::<VmEvent>()?),
            StructType::CustomizationEvent => Ok(from.as_any_box().downcast::<CustomizationEvent>()?),
            StructType::CustomizationFailed => Ok(from.as_any_box().downcast::<CustomizationFailed>()?),
            StructType::CustomizationLinuxIdentityFailed => Ok(from.as_any_box().downcast::<CustomizationLinuxIdentityFailed>()?),
            StructType::CustomizationNetworkSetupFailed => Ok(from.as_any_box().downcast::<CustomizationNetworkSetupFailed>()?),
            StructType::CustomizationSysprepFailed => Ok(from.as_any_box().downcast::<CustomizationSysprepFailed>()?),
            StructType::CustomizationUnknownFailure => Ok(from.as_any_box().downcast::<CustomizationUnknownFailure>()?),
            StructType::CustomizationStartedEvent => Ok(from.as_any_box().downcast::<CustomizationStartedEvent>()?),
            StructType::CustomizationSucceeded => Ok(from.as_any_box().downcast::<CustomizationSucceeded>()?),
            StructType::DrsRuleComplianceEvent => Ok(from.as_any_box().downcast::<DrsRuleComplianceEvent>()?),
            StructType::DrsRuleViolationEvent => Ok(from.as_any_box().downcast::<DrsRuleViolationEvent>()?),
            StructType::DrsSoftRuleViolationEvent => Ok(from.as_any_box().downcast::<DrsSoftRuleViolationEvent>()?),
            StructType::MigrationEvent => Ok(from.as_any_box().downcast::<MigrationEvent>()?),
            StructType::MigrationErrorEvent => Ok(from.as_any_box().downcast::<MigrationErrorEvent>()?),
            StructType::MigrationHostErrorEvent => Ok(from.as_any_box().downcast::<MigrationHostErrorEvent>()?),
            StructType::MigrationHostWarningEvent => Ok(from.as_any_box().downcast::<MigrationHostWarningEvent>()?),
            StructType::MigrationResourceErrorEvent => Ok(from.as_any_box().downcast::<MigrationResourceErrorEvent>()?),
            StructType::MigrationResourceWarningEvent => Ok(from.as_any_box().downcast::<MigrationResourceWarningEvent>()?),
            StructType::MigrationWarningEvent => Ok(from.as_any_box().downcast::<MigrationWarningEvent>()?),
            StructType::NoMaintenanceModeDrsRecommendationForVm => Ok(from.as_any_box().downcast::<NoMaintenanceModeDrsRecommendationForVm>()?),
            StructType::NotEnoughResourcesToStartVmEvent => Ok(from.as_any_box().downcast::<NotEnoughResourcesToStartVmEvent>()?),
            StructType::VmAcquiredMksTicketEvent => Ok(from.as_any_box().downcast::<VmAcquiredMksTicketEvent>()?),
            StructType::VmAcquiredTicketEvent => Ok(from.as_any_box().downcast::<VmAcquiredTicketEvent>()?),
            StructType::VmAutoRenameEvent => Ok(from.as_any_box().downcast::<VmAutoRenameEvent>()?),
            StructType::VmBeingCreatedEvent => Ok(from.as_any_box().downcast::<VmBeingCreatedEvent>()?),
            StructType::VmBeingDeployedEvent => Ok(from.as_any_box().downcast::<VmBeingDeployedEvent>()?),
            StructType::VmBeingHotMigratedEvent => Ok(from.as_any_box().downcast::<VmBeingHotMigratedEvent>()?),
            StructType::VmBeingMigratedEvent => Ok(from.as_any_box().downcast::<VmBeingMigratedEvent>()?),
            StructType::VmCloneEvent => Ok(from.as_any_box().downcast::<VmCloneEvent>()?),
            StructType::VmBeingClonedEvent => Ok(from.as_any_box().downcast::<VmBeingClonedEvent>()?),
            StructType::VmBeingClonedNoFolderEvent => Ok(from.as_any_box().downcast::<VmBeingClonedNoFolderEvent>()?),
            StructType::VmCloneFailedEvent => Ok(from.as_any_box().downcast::<VmCloneFailedEvent>()?),
            StructType::VmClonedEvent => Ok(from.as_any_box().downcast::<VmClonedEvent>()?),
            StructType::VmConfigMissingEvent => Ok(from.as_any_box().downcast::<VmConfigMissingEvent>()?),
            StructType::VmConnectedEvent => Ok(from.as_any_box().downcast::<VmConnectedEvent>()?),
            StructType::VmCreatedEvent => Ok(from.as_any_box().downcast::<VmCreatedEvent>()?),
            StructType::VmDasBeingResetEvent => Ok(from.as_any_box().downcast::<VmDasBeingResetEvent>()?),
            StructType::VmDasBeingResetWithScreenshotEvent => Ok(from.as_any_box().downcast::<VmDasBeingResetWithScreenshotEvent>()?),
            StructType::VmDasResetFailedEvent => Ok(from.as_any_box().downcast::<VmDasResetFailedEvent>()?),
            StructType::VmDasUpdateErrorEvent => Ok(from.as_any_box().downcast::<VmDasUpdateErrorEvent>()?),
            StructType::VmDasUpdateOkEvent => Ok(from.as_any_box().downcast::<VmDasUpdateOkEvent>()?),
            StructType::VmDateRolledBackEvent => Ok(from.as_any_box().downcast::<VmDateRolledBackEvent>()?),
            StructType::VmDeployFailedEvent => Ok(from.as_any_box().downcast::<VmDeployFailedEvent>()?),
            StructType::VmDeployedEvent => Ok(from.as_any_box().downcast::<VmDeployedEvent>()?),
            StructType::VmDisconnectedEvent => Ok(from.as_any_box().downcast::<VmDisconnectedEvent>()?),
            StructType::VmDiscoveredEvent => Ok(from.as_any_box().downcast::<VmDiscoveredEvent>()?),
            StructType::VmDiskFailedEvent => Ok(from.as_any_box().downcast::<VmDiskFailedEvent>()?),
            StructType::VmEmigratingEvent => Ok(from.as_any_box().downcast::<VmEmigratingEvent>()?),
            StructType::VmEndRecordingEvent => Ok(from.as_any_box().downcast::<VmEndRecordingEvent>()?),
            StructType::VmEndReplayingEvent => Ok(from.as_any_box().downcast::<VmEndReplayingEvent>()?),
            StructType::VmFailedMigrateEvent => Ok(from.as_any_box().downcast::<VmFailedMigrateEvent>()?),
            StructType::VmFailedRelayoutEvent => Ok(from.as_any_box().downcast::<VmFailedRelayoutEvent>()?),
            StructType::VmFailedRelayoutOnVmfs2DatastoreEvent => Ok(from.as_any_box().downcast::<VmFailedRelayoutOnVmfs2DatastoreEvent>()?),
            StructType::VmFailedStartingSecondaryEvent => Ok(from.as_any_box().downcast::<VmFailedStartingSecondaryEvent>()?),
            StructType::VmFailedToPowerOffEvent => Ok(from.as_any_box().downcast::<VmFailedToPowerOffEvent>()?),
            StructType::VmFailedToPowerOnEvent => Ok(from.as_any_box().downcast::<VmFailedToPowerOnEvent>()?),
            StructType::VmFailedToRebootGuestEvent => Ok(from.as_any_box().downcast::<VmFailedToRebootGuestEvent>()?),
            StructType::VmFailedToResetEvent => Ok(from.as_any_box().downcast::<VmFailedToResetEvent>()?),
            StructType::VmFailedToShutdownGuestEvent => Ok(from.as_any_box().downcast::<VmFailedToShutdownGuestEvent>()?),
            StructType::VmFailedToStandbyGuestEvent => Ok(from.as_any_box().downcast::<VmFailedToStandbyGuestEvent>()?),
            StructType::VmFailedToSuspendEvent => Ok(from.as_any_box().downcast::<VmFailedToSuspendEvent>()?),
            StructType::VmFailedUpdatingSecondaryConfig => Ok(from.as_any_box().downcast::<VmFailedUpdatingSecondaryConfig>()?),
            StructType::VmFailoverFailed => Ok(from.as_any_box().downcast::<VmFailoverFailed>()?),
            StructType::VmFaultToleranceStateChangedEvent => Ok(from.as_any_box().downcast::<VmFaultToleranceStateChangedEvent>()?),
            StructType::VmFaultToleranceTurnedOffEvent => Ok(from.as_any_box().downcast::<VmFaultToleranceTurnedOffEvent>()?),
            StructType::VmFaultToleranceVmTerminatedEvent => Ok(from.as_any_box().downcast::<VmFaultToleranceVmTerminatedEvent>()?),
            StructType::VmGuestOsCrashedEvent => Ok(from.as_any_box().downcast::<VmGuestOsCrashedEvent>()?),
            StructType::VmGuestRebootEvent => Ok(from.as_any_box().downcast::<VmGuestRebootEvent>()?),
            StructType::VmGuestShutdownEvent => Ok(from.as_any_box().downcast::<VmGuestShutdownEvent>()?),
            StructType::VmGuestStandbyEvent => Ok(from.as_any_box().downcast::<VmGuestStandbyEvent>()?),
            StructType::VmInstanceUuidAssignedEvent => Ok(from.as_any_box().downcast::<VmInstanceUuidAssignedEvent>()?),
            StructType::VmInstanceUuidChangedEvent => Ok(from.as_any_box().downcast::<VmInstanceUuidChangedEvent>()?),
            StructType::VmInstanceUuidConflictEvent => Ok(from.as_any_box().downcast::<VmInstanceUuidConflictEvent>()?),
            StructType::VmMacAssignedEvent => Ok(from.as_any_box().downcast::<VmMacAssignedEvent>()?),
            StructType::VmMacChangedEvent => Ok(from.as_any_box().downcast::<VmMacChangedEvent>()?),
            StructType::VmMacConflictEvent => Ok(from.as_any_box().downcast::<VmMacConflictEvent>()?),
            StructType::VmMaxFtRestartCountReached => Ok(from.as_any_box().downcast::<VmMaxFtRestartCountReached>()?),
            StructType::VmMaxRestartCountReached => Ok(from.as_any_box().downcast::<VmMaxRestartCountReached>()?),
            StructType::VmMessageErrorEvent => Ok(from.as_any_box().downcast::<VmMessageErrorEvent>()?),
            StructType::VmMessageEvent => Ok(from.as_any_box().downcast::<VmMessageEvent>()?),
            StructType::VmMessageWarningEvent => Ok(from.as_any_box().downcast::<VmMessageWarningEvent>()?),
            StructType::VmMigratedEvent => Ok(from.as_any_box().downcast::<VmMigratedEvent>()?),
            StructType::DrsVmMigratedEvent => Ok(from.as_any_box().downcast::<DrsVmMigratedEvent>()?),
            StructType::VmNoCompatibleHostForSecondaryEvent => Ok(from.as_any_box().downcast::<VmNoCompatibleHostForSecondaryEvent>()?),
            StructType::VmNoNetworkAccessEvent => Ok(from.as_any_box().downcast::<VmNoNetworkAccessEvent>()?),
            StructType::VmOrphanedEvent => Ok(from.as_any_box().downcast::<VmOrphanedEvent>()?),
            StructType::VmPoweredOffEvent => Ok(from.as_any_box().downcast::<VmPoweredOffEvent>()?),
            StructType::VmPowerOffOnIsolationEvent => Ok(from.as_any_box().downcast::<VmPowerOffOnIsolationEvent>()?),
            StructType::VmShutdownOnIsolationEvent => Ok(from.as_any_box().downcast::<VmShutdownOnIsolationEvent>()?),
            StructType::VmPoweredOnEvent => Ok(from.as_any_box().downcast::<VmPoweredOnEvent>()?),
            StructType::DrsVmPoweredOnEvent => Ok(from.as_any_box().downcast::<DrsVmPoweredOnEvent>()?),
            StructType::VmRestartedOnAlternateHostEvent => Ok(from.as_any_box().downcast::<VmRestartedOnAlternateHostEvent>()?),
            StructType::VmPoweringOnWithCustomizedDvPortEvent => Ok(from.as_any_box().downcast::<VmPoweringOnWithCustomizedDvPortEvent>()?),
            StructType::VmPrimaryFailoverEvent => Ok(from.as_any_box().downcast::<VmPrimaryFailoverEvent>()?),
            StructType::VmReconfiguredEvent => Ok(from.as_any_box().downcast::<VmReconfiguredEvent>()?),
            StructType::VmRegisteredEvent => Ok(from.as_any_box().downcast::<VmRegisteredEvent>()?),
            StructType::VmRelayoutSuccessfulEvent => Ok(from.as_any_box().downcast::<VmRelayoutSuccessfulEvent>()?),
            StructType::VmRelayoutUpToDateEvent => Ok(from.as_any_box().downcast::<VmRelayoutUpToDateEvent>()?),
            StructType::VmReloadFromPathEvent => Ok(from.as_any_box().downcast::<VmReloadFromPathEvent>()?),
            StructType::VmReloadFromPathFailedEvent => Ok(from.as_any_box().downcast::<VmReloadFromPathFailedEvent>()?),
            StructType::VmRelocateSpecEvent => Ok(from.as_any_box().downcast::<VmRelocateSpecEvent>()?),
            StructType::VmBeingRelocatedEvent => Ok(from.as_any_box().downcast::<VmBeingRelocatedEvent>()?),
            StructType::VmRelocateFailedEvent => Ok(from.as_any_box().downcast::<VmRelocateFailedEvent>()?),
            StructType::VmRelocatedEvent => Ok(from.as_any_box().downcast::<VmRelocatedEvent>()?),
            StructType::VmRemoteConsoleConnectedEvent => Ok(from.as_any_box().downcast::<VmRemoteConsoleConnectedEvent>()?),
            StructType::VmRemoteConsoleDisconnectedEvent => Ok(from.as_any_box().downcast::<VmRemoteConsoleDisconnectedEvent>()?),
            StructType::VmRemovedEvent => Ok(from.as_any_box().downcast::<VmRemovedEvent>()?),
            StructType::VmRenamedEvent => Ok(from.as_any_box().downcast::<VmRenamedEvent>()?),
            StructType::VmRequirementsExceedCurrentEvcModeEvent => Ok(from.as_any_box().downcast::<VmRequirementsExceedCurrentEvcModeEvent>()?),
            StructType::VmResettingEvent => Ok(from.as_any_box().downcast::<VmResettingEvent>()?),
            StructType::VmResourcePoolMovedEvent => Ok(from.as_any_box().downcast::<VmResourcePoolMovedEvent>()?),
            StructType::VmResourceReallocatedEvent => Ok(from.as_any_box().downcast::<VmResourceReallocatedEvent>()?),
            StructType::VmResumingEvent => Ok(from.as_any_box().downcast::<VmResumingEvent>()?),
            StructType::VmSecondaryAddedEvent => Ok(from.as_any_box().downcast::<VmSecondaryAddedEvent>()?),
            StructType::VmSecondaryDisabledBySystemEvent => Ok(from.as_any_box().downcast::<VmSecondaryDisabledBySystemEvent>()?),
            StructType::VmSecondaryDisabledEvent => Ok(from.as_any_box().downcast::<VmSecondaryDisabledEvent>()?),
            StructType::VmSecondaryEnabledEvent => Ok(from.as_any_box().downcast::<VmSecondaryEnabledEvent>()?),
            StructType::VmSecondaryStartedEvent => Ok(from.as_any_box().downcast::<VmSecondaryStartedEvent>()?),
            StructType::VmStartRecordingEvent => Ok(from.as_any_box().downcast::<VmStartRecordingEvent>()?),
            StructType::VmStartReplayingEvent => Ok(from.as_any_box().downcast::<VmStartReplayingEvent>()?),
            StructType::VmStartingEvent => Ok(from.as_any_box().downcast::<VmStartingEvent>()?),
            StructType::VmUnsupportedStartingEvent => Ok(from.as_any_box().downcast::<VmUnsupportedStartingEvent>()?),
            StructType::VmStartingSecondaryEvent => Ok(from.as_any_box().downcast::<VmStartingSecondaryEvent>()?),
            StructType::VmStaticMacConflictEvent => Ok(from.as_any_box().downcast::<VmStaticMacConflictEvent>()?),
            StructType::VmStoppingEvent => Ok(from.as_any_box().downcast::<VmStoppingEvent>()?),
            StructType::VmSuspendedEvent => Ok(from.as_any_box().downcast::<VmSuspendedEvent>()?),
            StructType::VmSuspendingEvent => Ok(from.as_any_box().downcast::<VmSuspendingEvent>()?),
            StructType::VmTimedoutStartingSecondaryEvent => Ok(from.as_any_box().downcast::<VmTimedoutStartingSecondaryEvent>()?),
            StructType::VmUpgradeCompleteEvent => Ok(from.as_any_box().downcast::<VmUpgradeCompleteEvent>()?),
            StructType::VmUpgradeFailedEvent => Ok(from.as_any_box().downcast::<VmUpgradeFailedEvent>()?),
            StructType::VmUpgradingEvent => Ok(from.as_any_box().downcast::<VmUpgradingEvent>()?),
            StructType::VmUuidAssignedEvent => Ok(from.as_any_box().downcast::<VmUuidAssignedEvent>()?),
            StructType::VmUuidChangedEvent => Ok(from.as_any_box().downcast::<VmUuidChangedEvent>()?),
            StructType::VmUuidConflictEvent => Ok(from.as_any_box().downcast::<VmUuidConflictEvent>()?),
            StructType::VmWwnAssignedEvent => Ok(from.as_any_box().downcast::<VmWwnAssignedEvent>()?),
            StructType::VmWwnChangedEvent => Ok(from.as_any_box().downcast::<VmWwnChangedEvent>()?),
            StructType::VmWwnConflictEvent => Ok(from.as_any_box().downcast::<VmWwnConflictEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
