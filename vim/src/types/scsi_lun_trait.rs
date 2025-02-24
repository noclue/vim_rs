use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The *ScsiLun* data object describes a SCSI logical unit.
/// 
/// A SCSI logical unit is a host device that an ESX Server or virtual machine
/// can use for I/O operations.
/// 
/// An ESX Server creates SCSI logical unit objects to represent
/// devices in the host configuration. (See the definition of
/// *ScsiLunType_enum* for a list of the supported device types.)
/// The vSphere API uses one of two object types to represent a SCSI
/// logical unit, depending on the device type.
/// - Disks containing file system volumes or parts of volumes for hosts
///   or raw disks for virtual machines. To represent disks, the ESX Server
///   creates a *HostScsiDisk* object, which inherits properties from
///   the *ScsiLun* base class.
/// - Other SCSI devices, for example SCSI passthrough devices
///   for virtual machines. To represent one of these devices,
///   the ESX Server creates a *ScsiLun* object.
///   
/// When the Server creates a *HostScsiDisk* or *ScsiLun* object,
/// it specifies a valid device name and type:
/// - *HostDevice.deviceName* - A string representing the name of the device
///   that is meaningful to the host. The following are some examples of
///   device names.  
///   &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>/dev/cdrom</code>  
///   &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>/vmkdev/vmhba0:0:1:0</code>  
///   &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>PhysicalDrive0</code>
/// - *HostDevice.deviceType* - A string describing the type of device.
///   The following are some examples of device types.  
///   &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>scsi-cdrom</code>  
///   &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>scsi-tape</code>  
///   &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>scsi-disk</code>  
///   &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>scsi-processor</code>  
///   &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>scsi-unknown</code>
pub trait ScsiLunTrait : super::host_device_trait::HostDeviceTrait {
    /// Linkable identifier
    fn get_key(&self) -> &Option<String>;
    /// Universally unique identifier for the LUN used to identify ScsiLun across
    /// multiple servers.
    /// 
    /// This identifier can be used to identify analogous objects in other views
    /// such as *HostMultipathInfoLogicalUnit* and *HostScsiTopologyLun*.
    /// 
    /// See also *HostMultipathInfoLogicalUnit*, *HostScsiTopologyLun*.
    fn get_uuid(&self) -> &str;
    /// List of descriptors that can be used to identify the LUN object.
    /// 
    /// The
    /// uuid will also appear as a descriptor.
    /// 
    /// The id field in the descriptor is a string that can be used to correlate
    /// the ScsiLun across multiple servers. A ScsiLun may have multiple
    /// descriptors. The choice and order of these descriptors may be different
    /// on different servers.
    /// 
    /// Not all descriptors are suitable for correlation. Some descriptors are
    /// only sufficient to identify the ScsiLun within a single host. Each
    /// descriptor contains a quality property that indicates whether or not
    /// the descriptor is suitable for correlation.
    fn get_descriptor(&self) -> &Option<Vec<ScsiLunDescriptor>>;
    /// Canonical name of the SCSI logical unit.
    /// 
    /// Disk partition or extent identifiers refer to this name when
    /// referring to a disk. Use this property to correlate a partition
    /// or extent to a specific SCSI disk.
    /// 
    /// See also *HostScsiDiskPartition.diskName*.
    fn get_canonical_name(&self) -> &Option<String>;
    /// User configurable display name of the SCSI logical unit.
    /// 
    /// A default
    /// display name will be used if available. If the display name is not
    /// supported, it will be unset. The display name does not have to be
    /// unique but it is recommended that it be unique.
    fn get_display_name(&self) -> &Option<String>;
    /// The type of SCSI device.
    /// 
    /// Must be one of the values of
    /// *ScsiLunType_enum*.
    fn get_lun_type(&self) -> &str;
    /// The vendor of the SCSI device.
    fn get_vendor(&self) -> &Option<String>;
    /// The model number of the SCSI device.
    fn get_model(&self) -> &Option<String>;
    /// The revision of the SCSI device.
    fn get_revision(&self) -> &Option<String>;
    /// The SCSI level of the SCSI device.
    fn get_scsi_level(&self) -> Option<i32>;
    /// The serial number of the SCSI device.
    /// 
    /// For a device that is SCSI-3 compliant, this property is derived
    /// from page 80h of the Vital Product Data (VPD), as defined by the
    /// SCSI-3 Primary Commands (SPC-3) spec. Not all SCSI-3 compliant
    /// devices provide this information. For devices that are not
    /// SCSI-3 compliant, this property is not defined.
    fn get_serial_number(&self) -> &Option<String>;
    /// The durable name of the SCSI device.
    /// 
    /// For a SCSI-3 compliant device this property is derived from the
    /// payloads of pages 80h and 83h of the Vital Product Data (VPD) as
    /// defined by the T10 and SMI standards. For devices that do not provide
    /// this information, this property is not defined.
    fn get_durable_name(&self) -> &Option<ScsiLunDurableName>;
    /// Alternate durable names.
    /// 
    /// Records all available durable names derived from page 80h of the Vital
    /// Product Data (VPD) and the Identification Vital Product Data (VPD) page
    /// 83h as defined by the SCSI-3 Primary Commands. For devices that are not
    /// SCSI-3 compliant this property is not defined.
    fn get_alternate_name(&self) -> &Option<Vec<ScsiLunDurableName>>;
    /// Standard Inquiry payload.
    /// 
    /// For a SCSI-3 compliant device this property is derived from the
    /// standard inquiry data. For devices that are not SCSI-3 compliant this
    /// property is not defined.
    fn get_standard_inquiry(&self) -> &Option<Vec<i8>>;
    /// The queue depth of SCSI device.
    fn get_queue_depth(&self) -> Option<i32>;
    /// The operational states of the LUN.
    /// 
    /// When more than one item is present in the array, the first state
    /// should be considered the primary state. For example, a LUN may
    /// be "ok" and "degraded" indicating I/O is still possible to the LUN, but
    /// it is operating in a degraded mode.
    /// 
    /// See also *ScsiLunState_enum*.
    fn get_operational_state(&self) -> &Vec<String>;
    /// Capabilities of SCSI device.
    fn get_capabilities(&self) -> &Option<ScsiLunCapabilities>;
    /// vStorage hardware acceleration support status.
    /// 
    /// This property
    /// represents storage acceleration provided by the SCSI logical unit.
    /// See *ScsiLunVStorageSupportStatus_enum* for valid values.
    /// 
    /// If a storage device supports hardware acceleration,
    /// the ESX host can offload specific virtual machine management
    /// operations to the storage device. With hardware assistance,
    /// the host performs storage operations faster and consumes
    /// less CPU, memory, and storage fabric bandwidth.
    /// 
    /// For vSphere 4.0 or earlier hosts, this value will be unset.
    fn get_v_storage_support(&self) -> &Option<String>;
    /// Indicates that this SCSI LUN is protocol endpoint.
    /// 
    /// This
    /// property will be populated if and only if host supports
    /// VirtualVolume based Datastore. Check the host capability
    /// *HostCapability.virtualVolumeDatastoreSupported*.
    /// See *HostProtocolEndpoint*.
    fn get_protocol_endpoint(&self) -> Option<bool>;
    /// Indicates the state of a perennially reserved flag for a LUN.
    /// 
    /// If
    /// set for Raw Device Mapped (RDM) LUNs, the host startup or LUN rescan
    /// take comparatively shorter duration than when it is unset.
    fn get_perennially_reserved(&self) -> Option<bool>;
    /// Indicates if LUN has the prequisite properties to enable Clustered Vmdk
    /// feature once formatted into VMFS Datastore.
    fn get_clustered_vmdk_supported(&self) -> Option<bool>;
    /// Indicates the current device protocol.
    /// 
    /// Application protocol for a device which is set based on input
    /// from vmkctl storage control plane. Must be one of the values of
    /// *DeviceProtocol_enum*.
    /// 
    /// ***Since:*** vSphere API Release 8.0.1.0
    fn get_application_protocol(&self) -> &Option<String>;
    /// Indicates whether namespace is dispersed.
    /// 
    /// Set to true when the namespace of LUN is dispersed.
    /// 
    /// ***Since:*** vSphere API Release 8.0.1.0
    fn get_dispersed_ns(&self) -> Option<bool>;
}
impl<'s> serde::Serialize for dyn ScsiLunTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ScsiLunTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ScsiLunVisitor)
            }
        }

struct ScsiLunVisitor;

impl<'de> de::Visitor<'de> for ScsiLunVisitor {
    type Value = Box<dyn ScsiLunTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ScsiLunTrait JSON object with a _typeName field")
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

impl ScsiLunTrait for ScsiLun {
    fn get_key(&self) -> &Option<String> { &self.key }
    fn get_uuid(&self) -> &str { &self.uuid }
    fn get_descriptor(&self) -> &Option<Vec<ScsiLunDescriptor>> { &self.descriptor }
    fn get_canonical_name(&self) -> &Option<String> { &self.canonical_name }
    fn get_display_name(&self) -> &Option<String> { &self.display_name }
    fn get_lun_type(&self) -> &str { &self.lun_type }
    fn get_vendor(&self) -> &Option<String> { &self.vendor }
    fn get_model(&self) -> &Option<String> { &self.model }
    fn get_revision(&self) -> &Option<String> { &self.revision }
    fn get_scsi_level(&self) -> Option<i32> { self.scsi_level }
    fn get_serial_number(&self) -> &Option<String> { &self.serial_number }
    fn get_durable_name(&self) -> &Option<ScsiLunDurableName> { &self.durable_name }
    fn get_alternate_name(&self) -> &Option<Vec<ScsiLunDurableName>> { &self.alternate_name }
    fn get_standard_inquiry(&self) -> &Option<Vec<i8>> { &self.standard_inquiry }
    fn get_queue_depth(&self) -> Option<i32> { self.queue_depth }
    fn get_operational_state(&self) -> &Vec<String> { &self.operational_state }
    fn get_capabilities(&self) -> &Option<ScsiLunCapabilities> { &self.capabilities }
    fn get_v_storage_support(&self) -> &Option<String> { &self.v_storage_support }
    fn get_protocol_endpoint(&self) -> Option<bool> { self.protocol_endpoint }
    fn get_perennially_reserved(&self) -> Option<bool> { self.perennially_reserved }
    fn get_clustered_vmdk_supported(&self) -> Option<bool> { self.clustered_vmdk_supported }
    fn get_application_protocol(&self) -> &Option<String> { &self.application_protocol }
    fn get_dispersed_ns(&self) -> Option<bool> { self.dispersed_ns }
}
impl ScsiLunTrait for HostScsiDisk {
    fn get_key(&self) -> &Option<String> { &self.key }
    fn get_uuid(&self) -> &str { &self.uuid }
    fn get_descriptor(&self) -> &Option<Vec<ScsiLunDescriptor>> { &self.descriptor }
    fn get_canonical_name(&self) -> &Option<String> { &self.canonical_name }
    fn get_display_name(&self) -> &Option<String> { &self.display_name }
    fn get_lun_type(&self) -> &str { &self.lun_type }
    fn get_vendor(&self) -> &Option<String> { &self.vendor }
    fn get_model(&self) -> &Option<String> { &self.model }
    fn get_revision(&self) -> &Option<String> { &self.revision }
    fn get_scsi_level(&self) -> Option<i32> { self.scsi_level }
    fn get_serial_number(&self) -> &Option<String> { &self.serial_number }
    fn get_durable_name(&self) -> &Option<ScsiLunDurableName> { &self.durable_name }
    fn get_alternate_name(&self) -> &Option<Vec<ScsiLunDurableName>> { &self.alternate_name }
    fn get_standard_inquiry(&self) -> &Option<Vec<i8>> { &self.standard_inquiry }
    fn get_queue_depth(&self) -> Option<i32> { self.queue_depth }
    fn get_operational_state(&self) -> &Vec<String> { &self.operational_state }
    fn get_capabilities(&self) -> &Option<ScsiLunCapabilities> { &self.capabilities }
    fn get_v_storage_support(&self) -> &Option<String> { &self.v_storage_support }
    fn get_protocol_endpoint(&self) -> Option<bool> { self.protocol_endpoint }
    fn get_perennially_reserved(&self) -> Option<bool> { self.perennially_reserved }
    fn get_clustered_vmdk_supported(&self) -> Option<bool> { self.clustered_vmdk_supported }
    fn get_application_protocol(&self) -> &Option<String> { &self.application_protocol }
    fn get_dispersed_ns(&self) -> Option<bool> { self.dispersed_ns }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ScsiLunTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ScsiLun => Some(from.as_any_ref().downcast_ref::<ScsiLun>()?),
            StructType::HostScsiDisk => Some(from.as_any_ref().downcast_ref::<HostScsiDisk>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ScsiLun => Ok(from.as_any_box().downcast::<ScsiLun>()?),
            StructType::HostScsiDisk => Ok(from.as_any_box().downcast::<HostScsiDisk>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
