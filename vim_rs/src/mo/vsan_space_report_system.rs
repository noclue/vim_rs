use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object type provides the service interface to report the
/// vSAN cluster space usage information including the space overview, the
/// space usage breakdown to various vSAN object types and the vSAN data
/// efficiency info after enabling vSAN deduplication.  
/// The ManagedEntity can be accessed with MOID of 'vsan-cluster-space-report-system',
/// through vSAN service at vCenter server side.
#[derive(Clone)]
pub struct VsanSpaceReportSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanSpaceReportSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Query the vSAN space usage for the given vSAN object entities which
    /// contain a group of objects like the virtual machine with virtual disks
    /// backed by vSAN objects.
    /// 
    /// Note it can query maximum 100 entities' space
    /// usage in one call for performance consideration.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// The vSAN cluster
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### query_spec
    /// The vSAN space usage query spec to specify the entity
    /// types like virtual machine and the entity UUIDs like
    /// the managed object ID for virtual machines.
    ///
    /// ## Returns:
    ///
    /// The vSAN space usage for the vSAN entity
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_entity_space_usage(&self, cluster: &crate::types::structs::ManagedObjectReference, query_spec: &crate::types::structs::VsanSpaceQuerySpec) -> Result<Option<Vec<crate::types::structs::VsanEntitySpaceUsage>>> {
        let input = VsanQueryEntitySpaceUsageRequestType {cluster, query_spec, };
        let path = format!("/vsan/VsanSpaceReportSystem/{moId}/VsanQueryEntitySpaceUsage", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::VsanEntitySpaceUsage>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Query the vSAN space usage including the space usage overview
    /// and the space usage breakdown according to vSAN object type.
    /// 
    /// This API will take less than one or tens seconds to return according
    /// to the number of node and object in the vSAN cluster. The API can also
    /// be used to query vSAN datastore physical capacity and what-if capacity
    /// under specified storage policies.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// vSAN cluster
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### storage_policies
    /// Storage policies specified to calculate what-if
    /// capacity of the cluster vSAN datastore. Default
    /// value is None
    ///
    /// ### whatif_capacity_only
    /// Flag indicates if the API is called only
    /// for getting vSAN datastore physical capacity
    /// and what-if capacity. Default value is False,
    /// which returns the vSAN space usage including
    /// the space usage overview and the space usage
    /// breakdown according to vSAN object type. When
    /// it is True, the API only returns the vSAN
    /// datastore physical capacity and what-if capacity
    /// under given storage policies.
    ///
    /// ## Returns:
    ///
    /// the vSAN space usage and vSAN datastore capacities.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_query_space_usage(&self, cluster: &crate::types::structs::ManagedObjectReference, storage_policies: Option<&[Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>, whatif_capacity_only: Option<bool>) -> Result<crate::types::structs::VsanSpaceUsage> {
        let input = VsanQuerySpaceUsageRequestType {cluster, storage_policies, whatif_capacity_only, };
        let path = format!("/vsan/VsanSpaceReportSystem/{moId}/VsanQuerySpaceUsage", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::VsanSpaceUsage = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Query the space usage including the space usage overview and the space usage
    /// breakdown based on different datastore types.
    /// 
    /// The API can also be used to
    /// query physical capacity.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// vSAN cluster
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### query_spec
    /// The QueryVsanManagedStorageSpaceUsageSpec to specify more
    /// detailed query specification.
    ///
    /// ## Returns:
    ///
    /// the space usage aggregated by different datastore type.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: Exception for invalid input arguments, for example,
    /// if the querySpec contains invalid datastore type.
    pub async fn query_vsan_managed_storage_space_usage(&self, cluster: &crate::types::structs::ManagedObjectReference, query_spec: &crate::types::structs::QueryVsanManagedStorageSpaceUsageSpec) -> Result<Vec<crate::types::structs::VsanSpaceUsageWithDatastoreType>> {
        let input = QueryVsanManagedStorageSpaceUsageRequestType {cluster, query_spec, };
        let path = format!("/vsan/VsanSpaceReportSystem/{moId}/QueryVsanManagedStorageSpaceUsage", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: Vec<crate::types::structs::VsanSpaceUsageWithDatastoreType> = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct VsanQueryEntitySpaceUsageRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    query_spec: &'a crate::types::structs::VsanSpaceQuerySpec,
}

impl<'a> miniserde::Serialize for VsanQueryEntitySpaceUsageRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQueryEntitySpaceUsageRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQueryEntitySpaceUsageRequestTypeSer<'b, 'a> {
    data: &'b VsanQueryEntitySpaceUsageRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanQueryEntitySpaceUsageRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQueryEntitySpaceUsageRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("querySpec"), &self.data.query_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanQuerySpaceUsageRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    storage_policies: Option<&'a [Box<dyn crate::types::traits::VirtualMachineProfileSpecTrait>]>,
    whatif_capacity_only: Option<bool>,
}

impl<'a> miniserde::Serialize for VsanQuerySpaceUsageRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanQuerySpaceUsageRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanQuerySpaceUsageRequestTypeSer<'b, 'a> {
    data: &'b VsanQuerySpaceUsageRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanQuerySpaceUsageRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanQuerySpaceUsageRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.storage_policies else { continue; };
                    return Some((std::borrow::Cow::Borrowed("storagePolicies"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.whatif_capacity_only else { continue; };
                    return Some((std::borrow::Cow::Borrowed("whatifCapacityOnly"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryVsanManagedStorageSpaceUsageRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    query_spec: &'a crate::types::structs::QueryVsanManagedStorageSpaceUsageSpec,
}

impl<'a> miniserde::Serialize for QueryVsanManagedStorageSpaceUsageRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryVsanManagedStorageSpaceUsageRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryVsanManagedStorageSpaceUsageRequestTypeSer<'b, 'a> {
    data: &'b QueryVsanManagedStorageSpaceUsageRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for QueryVsanManagedStorageSpaceUsageRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryVsanManagedStorageSpaceUsageRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("querySpec"), &self.data.query_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
