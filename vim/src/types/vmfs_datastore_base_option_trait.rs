use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base class that describes a VMFS datastore provisioning option.
pub trait VmfsDatastoreBaseOptionTrait : super::data_object_trait::DataObjectTrait {
    /// The partition table layout that the disk will have if this
    /// provisioning option is selected.
    /// 
    /// In releases after vSphere API 5.0, vSphere Servers might not
    /// generate property collector update notifications for this property.
    /// To obtain the latest value of the property, you can use
    /// PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
    /// If you use the PropertyCollector.WaitForUpdatesEx method, specify
    /// an empty string for the version parameter.
    /// Since this property is on a DataObject, an update returned by WaitForUpdatesEx may
    /// contain values for this property when some other property on the DataObject changes.
    /// If this update is a result of a call to WaitForUpdatesEx with a non-empty
    /// version parameter, the value for this property may not be current.
    fn get_layout(&self) -> &HostDiskPartitionLayout;
    /// Indicates whether selecting this option will change the partition
    /// format type on the disk.
    /// 
    /// See also *HostDiskPartitionInfoPartitionFormat_enum*.
    fn get_partition_format_change(&self) -> Option<bool>;
}
impl<'s> serde::Serialize for dyn VmfsDatastoreBaseOptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VmfsDatastoreBaseOptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VmfsDatastoreBaseOptionVisitor)
            }
        }

struct VmfsDatastoreBaseOptionVisitor;

impl<'de> de::Visitor<'de> for VmfsDatastoreBaseOptionVisitor {
    type Value = Box<dyn VmfsDatastoreBaseOptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VmfsDatastoreBaseOptionTrait JSON object with a _typeName field")
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

impl VmfsDatastoreBaseOptionTrait for VmfsDatastoreBaseOption {
    fn get_layout(&self) -> &HostDiskPartitionLayout { &self.layout }
    fn get_partition_format_change(&self) -> Option<bool> { self.partition_format_change }
}
impl VmfsDatastoreBaseOptionTrait for VmfsDatastoreMultipleExtentOption {
    fn get_layout(&self) -> &HostDiskPartitionLayout { &self.layout }
    fn get_partition_format_change(&self) -> Option<bool> { self.partition_format_change }
}
impl VmfsDatastoreBaseOptionTrait for VmfsDatastoreSingleExtentOption {
    fn get_layout(&self) -> &HostDiskPartitionLayout { &self.layout }
    fn get_partition_format_change(&self) -> Option<bool> { self.partition_format_change }
}
impl VmfsDatastoreBaseOptionTrait for VmfsDatastoreAllExtentOption {
    fn get_layout(&self) -> &HostDiskPartitionLayout { &self.layout }
    fn get_partition_format_change(&self) -> Option<bool> { self.partition_format_change }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VmfsDatastoreBaseOptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmfsDatastoreBaseOption => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreBaseOption>()?),
            StructType::VmfsDatastoreMultipleExtentOption => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreMultipleExtentOption>()?),
            StructType::VmfsDatastoreSingleExtentOption => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreSingleExtentOption>()?),
            StructType::VmfsDatastoreAllExtentOption => Some(from.as_any_ref().downcast_ref::<VmfsDatastoreAllExtentOption>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VmfsDatastoreBaseOption => Ok(from.as_any_box().downcast::<VmfsDatastoreBaseOption>()?),
            StructType::VmfsDatastoreMultipleExtentOption => Ok(from.as_any_box().downcast::<VmfsDatastoreMultipleExtentOption>()?),
            StructType::VmfsDatastoreSingleExtentOption => Ok(from.as_any_box().downcast::<VmfsDatastoreSingleExtentOption>()?),
            StructType::VmfsDatastoreAllExtentOption => Ok(from.as_any_box().downcast::<VmfsDatastoreAllExtentOption>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
