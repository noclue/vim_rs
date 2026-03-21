use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The VsanInternalSystem exposes low level access to CMMDS, as well as draft
/// versions of VSAN object and disk management APIs that are subject to change
/// in future releases.
/// 
/// No compatibility is guaranteed on any of the APIs,
/// including their prototype, behavior or result encoding.
#[derive(Clone)]
pub struct HostVsanInternalSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostVsanInternalSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Abdicate ownership of DOM objects.
    /// 
    /// The objects must be currently owned
    /// by this host. Which host has ownership of an object at a given point in
    /// time can be queried from QueryVsanObjects() or QueryCmmds() APIs.
    /// Abidcating ownership tears down DOM owner in-memory state. Hosts in the
    /// cluster will then compete to become the new owner of the object, similar
    /// to a host failure event. There is a short interuption of IO flow while
    /// the owner re-election is going on, but it is transparent to any consumers
    /// of the object.
    /// This API is meant as a troubleshooting and debugging tool. It is internal
    /// at this point and can be used to resolve issues where DOM owner gets
    /// "stuck".
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### uuids
    /// List of VSAN/DOM object UUIDs.
    ///
    /// ## Returns:
    ///
    /// List of UUIDs successfully abdicated.
    pub async fn abdicate_dom_ownership(&self, uuids: &[String]) -> Result<Option<Vec<String>>> {
        let input = AbdicateDomOwnershipRequestType {uuids, };
        let bytes_opt = self.client.invoke_optional("", "HostVsanInternalSystem", &self.mo_id, "AbdicateDomOwnership", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Determine if given objects can be provisioned.
    /// 
    /// Determines if the objects
    /// of the given size can be provisioned with the given policies.
    /// The API is intended to answer the question: can these objects be
    /// provisioned with the given policy using the current cluster topology
    /// (#hosts and #disks) and does NOT take into account free space on the
    /// disk, size of disks, etc. If the objects cannot be provisioned,
    /// the API returns the reason for not being able to satisfy the policy.
    /// If the objects can be provisioned, the API returns the cost of
    /// provisioning objects with this policy. Please note: This API ignores
    /// forceProvisioning.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### npbs
    /// List of NewPolicyBatch structure with sizes and policies.
    ///
    /// ### ignore_satisfiability
    /// Optionally populate PolicyCost even though
    /// object cannot be provisioned in the current cluster topology.
    ///
    /// ## Returns:
    ///
    /// List of PolicySatisfiability objects, one for each specified
    /// size.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn can_provision_objects(&self, npbs: &[crate::types::structs::VsanNewPolicyBatch], ignore_satisfiability: Option<bool>) -> Result<Vec<crate::types::structs::VsanPolicySatisfiability>> {
        let input = CanProvisionObjectsRequestType {npbs, ignore_satisfiability, };
        let bytes = self.client.invoke("", "HostVsanInternalSystem", &self.mo_id, "CanProvisionObjects", Some(&input)).await?;
        let result: Vec<crate::types::structs::VsanPolicySatisfiability> = crate::core::client::unmarshal_array(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Delete VSAN objects.
    /// 
    /// This API is internal and intended for troubleshooting/debugging only.
    /// WARNING: This API can be slow because we do IOs to all the objects.
    /// This API can be used to delete VSAN objects. DOM won't allow access to
    /// objects which have lost quorum. Such objects can be deleted with the
    /// optional "force" flag. These objects may however re-appear with quorum
    /// if the absent components come back (network partition gets resolved,
    /// etc.)
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### uuids
    /// List of object UUIDs to be deleted.
    ///
    /// ### force
    /// Optional force delete.
    ///
    /// ## Returns:
    ///
    /// List of DeleteVsanObjectsResult.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn delete_vsan_objects(&self, uuids: &[String], force: Option<bool>) -> Result<Vec<crate::types::structs::HostVsanInternalSystemDeleteVsanObjectsResult>> {
        let input = DeleteVsanObjectsRequestType {uuids, force, };
        let bytes = self.client.invoke("", "HostVsanInternalSystem", &self.mo_id, "DeleteVsanObjects", Some(&input)).await?;
        let result: Vec<crate::types::structs::HostVsanInternalSystemDeleteVsanObjectsResult> = crate::core::client::unmarshal_array(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Get VSAN object extended attributes.
    /// 
    /// This API is internal and intended for troubleshooting/debugging
    /// situations in the field. WARNING: This API can be slow because we
    /// do IOs (reads) to all the objects. This API can be used to get
    /// extended attributes of any object in the VSAN cluster from any host
    /// provided the object is accessible from that host. In case of an error,
    /// we return a dict with key "Error" for that object.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### uuids
    /// List of object UUIDs.
    ///
    /// ## Returns:
    ///
    /// JSON string with the extended attributes.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn get_vsan_obj_ext_attrs(&self, uuids: &[String]) -> Result<String> {
        let input = GetVsanObjExtAttrsRequestType {uuids, };
        let bytes = self.client.invoke("", "HostVsanInternalSystem", &self.mo_id, "GetVsanObjExtAttrs", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Query CMMDS directly.
    /// 
    /// The list of given queries is executed and all
    /// results are returned in a flat list. No attempt is made to de-dupe
    /// results in the case of overlapping query results.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### queries
    /// List of CMMDS query specs.
    ///
    /// ## Returns:
    ///
    /// JSON string with the results
    pub async fn query_cmmds(&self, queries: &[crate::types::structs::HostVsanInternalSystemCmmdsQuery]) -> Result<String> {
        let input = QueryCmmdsRequestType {queries, };
        let bytes = self.client.invoke("", "HostVsanInternalSystem", &self.mo_id, "QueryCmmds", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Query DOM objects on a given set of physical disks.
    /// 
    /// Finds all DOM objects
    /// that have at least one component on the given physical disks. In order to
    /// make this API efficient, the output of this API contains the found
    /// DOM\_OBJECT, and referenced LSOM\_OBJECT and DISK entries.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### disks
    /// List of VSAN disk UUIDs.
    ///
    /// ## Returns:
    ///
    /// JSON string with the results
    pub async fn query_objects_on_physical_vsan_disk(&self, disks: &[String]) -> Result<String> {
        let input = QueryObjectsOnPhysicalVsanDiskRequestType {disks, };
        let bytes = self.client.invoke("", "HostVsanInternalSystem", &self.mo_id, "QueryObjectsOnPhysicalVsanDisk", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Query statistics about physical VSAN disks.
    /// 
    /// Using the props parameter the
    /// caller can control which properties are returned. Requesting only the
    /// required properties is encouraged to reduce server load, response time
    /// and client load.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### props
    /// List of properties to gather. Not specifying a list will
    /// fetch all properties.
    ///
    /// ## Returns:
    ///
    /// JSON string with the results
    pub async fn query_physical_vsan_disks(&self, props: Option<&[String]>) -> Result<String> {
        let input = QueryPhysicalVsanDisksRequestType {props, };
        let bytes = self.client.invoke("", "HostVsanInternalSystem", &self.mo_id, "QueryPhysicalVsanDisks", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Query information about VSAN DOM objects that are currently syncing data.
    /// 
    /// Instead of returning all objects, only such objects are returned that
    /// are currently resyncing any stale components or syncing fresh replicas.
    /// The API returns the same output format as queryVsanObjects(). It
    /// retrieves information about syncing all objects, or retricts the
    /// search for syncing objects to the UUID list provided. In order to make
    /// this API efficient, the output of this API contains the found
    /// DOM\_OBJECT, and referenced LSOM\_OBJECT and DISK entries.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### uuids
    /// List of VSAN/DOM object UUIDs to restrict search to.
    ///
    /// ## Returns:
    ///
    /// JSON string with the results
    pub async fn query_syncing_vsan_objects(&self, uuids: Option<&[String]>) -> Result<String> {
        let input = QuerySyncingVsanObjectsRequestType {uuids, };
        let bytes = self.client.invoke("", "HostVsanInternalSystem", &self.mo_id, "QuerySyncingVsanObjects", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Query VSAN object UUIDs by filtering conditions.
    /// 
    /// The API queries CMMDS by given filtering conditions (initially only for
    /// object version) and return object UUID in an array with limited elements
    /// count.
    /// If caller specified the inputs objects UUID, then only these objects will
    /// be checked for the filtering conditions, and return ones which satisfy
    /// the filtering condition. In this case, the 'limit' parameter will be
    /// ignored.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### uuids
    /// Objects UUID will be checked against the filtering
    /// conditions.
    ///
    /// ### limit
    /// To limit the size of the result set.
    ///
    /// ### version
    /// Filtering condition 1: object version.
    ///
    /// ## Returns:
    ///
    /// String array of object uuids which satisfy the filtering
    /// conditions.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: for any unexpected failures.
    pub async fn query_vsan_object_uuids_by_filter(&self, uuids: Option<&[String]>, limit: Option<i32>, version: Option<i32>) -> Result<Option<Vec<String>>> {
        let input = QueryVsanObjectUuidsByFilterRequestType {uuids, limit, version, };
        let bytes_opt = self.client.invoke_optional("", "HostVsanInternalSystem", &self.mo_id, "QueryVsanObjectUuidsByFilter", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Query information about VSAN DOM objects.
    /// 
    /// Retrieves information about the
    /// given set of DOM object UUIDs. In order to make this API efficient, the
    /// output of this API contains the found DOM\_OBJECT, and referenced
    /// LSOM\_OBJECT and DISK entries.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### uuids
    /// List of VSAN/DOM object UUIDs.
    ///
    /// ## Returns:
    ///
    /// JSON string with the results
    pub async fn query_vsan_objects(&self, uuids: Option<&[String]>) -> Result<String> {
        let input = QueryVsanObjectsRequestType {uuids, };
        let bytes = self.client.invoke("", "HostVsanInternalSystem", &self.mo_id, "QueryVsanObjects", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Query VSAN system statistics.
    /// 
    /// This is a low level API that gathers low
    /// level statistic counters from the system. The details of the counters
    /// remain undocumented and unsupported at this point, and this API remains
    /// internal.
    /// The data for this API call mostly comes from VSI, but also other tools
    /// like memstats.
    /// The caller can control which counters are being retrieved by providing
    /// a list of labels. The following labels are current supported:
    /// \- TBD
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### labels
    /// List of labels of counters to retrieve.
    ///
    /// ## Returns:
    ///
    /// JSON string with the results
    pub async fn query_vsan_statistics(&self, labels: &[String]) -> Result<String> {
        let input = QueryVsanStatisticsRequestType {labels, };
        let bytes = self.client.invoke("", "HostVsanInternalSystem", &self.mo_id, "QueryVsanStatistics", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Determine if the given objects can be reconfigured with the given
    /// policies.
    /// 
    /// The what-if determination only takes into account
    /// the total number of hosts and total number
    /// of disks per host. The API is intended to answer the question: is
    /// this reconfiguration possible using the current cluster topology
    /// (#hosts and #disks) and does NOT take into account free space on the
    /// disk, size of disks, etc. If policy is not satisfiable, the API returns
    /// the reason for not being able to satisfy the policy. If the policy is
    /// satisfiable, the API returns the cost of provisioning objects with the
    /// new policy. This cost can be combined with current available free disk
    /// space to compute if a particular operation is expected to succeed
    /// or fail. Please note: This API ignores forceProvisioning.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### pcbs
    /// List of PolicyChangeBatch structure with uuids and policies.
    ///
    /// ### ignore_satisfiability
    /// Optionally populate PolicyCost even though
    /// object cannot be reconfigured in the current cluster topology.
    ///
    /// ## Returns:
    ///
    /// List of PolicySatisfiability objects, one for each specified
    /// UUID.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn reconfiguration_satisfiable(&self, pcbs: &[crate::types::structs::VsanPolicyChangeBatch], ignore_satisfiability: Option<bool>) -> Result<Vec<crate::types::structs::VsanPolicySatisfiability>> {
        let input = ReconfigurationSatisfiableRequestType {pcbs, ignore_satisfiability, };
        let bytes = self.client.invoke("", "HostVsanInternalSystem", &self.mo_id, "ReconfigurationSatisfiable", Some(&input)).await?;
        let result: Vec<crate::types::structs::VsanPolicySatisfiability> = crate::core::client::unmarshal_array(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Reconfigure DOM object.
    /// 
    /// Typically we expect VM centric APIs to be used
    /// for setting storage policies, i.e. to use ReconfigVM() to change the
    /// policy/profile of a namespace directory or virtual disk. This is a low
    /// level API to reconfigure any object known by UUID.
    /// This API is internal and intended for troubleshooting/debugging
    /// situations in the field.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### uuid
    /// DOM object UUID.
    ///
    /// ### policy
    /// VSAN expression formatted policy string.
    pub async fn reconfigure_dom_object(&self, uuid: &str, policy: &str) -> Result<()> {
        let input = ReconfigureDomObjectRequestType {uuid, policy, };
        self.client.invoke_void("", "HostVsanInternalSystem", &self.mo_id, "ReconfigureDomObject", Some(&input)).await
    }
    /// Runs diagnostics on VSAN physical disks.
    /// 
    /// This method takes an active
    /// approach and creates a minimal and temporary object on each physical
    /// MD disk consumed by VSAN across the entire VSAN cluster. The temporary
    /// objects are deleted right away upon completion of creation. The result
    /// returns a list of all checked MDs, indicating wheather or not there was
    /// a problem creating an object on that MD at the given point in time.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### disks
    /// List of VSAN disk UUIDs. If specified restricts the
    /// diagnostics run to VSAN disks present in the provided list.
    ///
    /// ## Returns:
    ///
    /// A list of result structures. One per checked disk.
    pub async fn run_vsan_physical_disk_diagnostics(&self, disks: Option<&[String]>) -> Result<Vec<crate::types::structs::HostVsanInternalSystemVsanPhysicalDiskDiagnosticsResult>> {
        let input = RunVsanPhysicalDiskDiagnosticsRequestType {disks, };
        let bytes = self.client.invoke("", "HostVsanInternalSystem", &self.mo_id, "RunVsanPhysicalDiskDiagnostics", Some(&input)).await?;
        let result: Vec<crate::types::structs::HostVsanInternalSystemVsanPhysicalDiskDiagnosticsResult> = crate::core::client::unmarshal_array(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Upgrade VSAN objects version.
    /// 
    /// Upgrade a set of objects' version to new one in batch mode.
    /// API caller should limit the size of the inputs array, and suggested
    /// array size is 500 ~ 1000 initially. (The API will give more realistic
    /// suggestion after more experiments, then will apply hard limits in future)
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### uuids
    /// The array of objects' UUID which will be upgraded.
    ///
    /// ### new_version
    /// The new version will be applied to objects.
    ///
    /// ## Returns:
    ///
    /// Array of VsanObjectOperationResult, the array only contains
    /// result for failed objects, such as invalid or not existing UUID,
    /// upgrade failure, etc.
    ///
    /// ## Errors:
    ///
    /// ***VsanFault***: for any unexpected failures.
    pub async fn upgrade_vsan_objects(&self, uuids: &[String], new_version: i32) -> Result<Option<Vec<crate::types::structs::HostVsanInternalSystemVsanObjectOperationResult>>> {
        let input = UpgradeVsanObjectsRequestType {uuids, new_version, };
        let bytes_opt = self.client.invoke_optional("", "HostVsanInternalSystem", &self.mo_id, "UpgradeVsanObjects", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
}
struct AbdicateDomOwnershipRequestType<'a> {
    uuids: &'a [String],
}

impl<'a> miniserde::Serialize for AbdicateDomOwnershipRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AbdicateDomOwnershipRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AbdicateDomOwnershipRequestTypeSer<'b, 'a> {
    data: &'b AbdicateDomOwnershipRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AbdicateDomOwnershipRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AbdicateDomOwnershipRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("uuids"), &self.data.uuids as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CanProvisionObjectsRequestType<'a> {
    npbs: &'a [crate::types::structs::VsanNewPolicyBatch],
    ignore_satisfiability: Option<bool>,
}

impl<'a> miniserde::Serialize for CanProvisionObjectsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CanProvisionObjectsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CanProvisionObjectsRequestTypeSer<'b, 'a> {
    data: &'b CanProvisionObjectsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CanProvisionObjectsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CanProvisionObjectsRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("npbs"), &self.data.npbs as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.ignore_satisfiability else { continue; };
                    return Some((std::borrow::Cow::Borrowed("ignoreSatisfiability"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct DeleteVsanObjectsRequestType<'a> {
    uuids: &'a [String],
    force: Option<bool>,
}

impl<'a> miniserde::Serialize for DeleteVsanObjectsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DeleteVsanObjectsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DeleteVsanObjectsRequestTypeSer<'b, 'a> {
    data: &'b DeleteVsanObjectsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DeleteVsanObjectsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DeleteVsanObjectsRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("uuids"), &self.data.uuids as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.force else { continue; };
                    return Some((std::borrow::Cow::Borrowed("force"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct GetVsanObjExtAttrsRequestType<'a> {
    uuids: &'a [String],
}

impl<'a> miniserde::Serialize for GetVsanObjExtAttrsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(GetVsanObjExtAttrsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct GetVsanObjExtAttrsRequestTypeSer<'b, 'a> {
    data: &'b GetVsanObjExtAttrsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for GetVsanObjExtAttrsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"GetVsanObjExtAttrsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("uuids"), &self.data.uuids as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryCmmdsRequestType<'a> {
    queries: &'a [crate::types::structs::HostVsanInternalSystemCmmdsQuery],
}

impl<'a> miniserde::Serialize for QueryCmmdsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryCmmdsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryCmmdsRequestTypeSer<'b, 'a> {
    data: &'b QueryCmmdsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryCmmdsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryCmmdsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("queries"), &self.data.queries as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryObjectsOnPhysicalVsanDiskRequestType<'a> {
    disks: &'a [String],
}

impl<'a> miniserde::Serialize for QueryObjectsOnPhysicalVsanDiskRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryObjectsOnPhysicalVsanDiskRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryObjectsOnPhysicalVsanDiskRequestTypeSer<'b, 'a> {
    data: &'b QueryObjectsOnPhysicalVsanDiskRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryObjectsOnPhysicalVsanDiskRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryObjectsOnPhysicalVsanDiskRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("disks"), &self.data.disks as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryPhysicalVsanDisksRequestType<'a> {
    props: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for QueryPhysicalVsanDisksRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryPhysicalVsanDisksRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryPhysicalVsanDisksRequestTypeSer<'b, 'a> {
    data: &'b QueryPhysicalVsanDisksRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryPhysicalVsanDisksRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryPhysicalVsanDisksRequestType")),
                1 => {
                    let Some(ref val) = self.data.props else { continue; };
                    return Some((std::borrow::Cow::Borrowed("props"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QuerySyncingVsanObjectsRequestType<'a> {
    uuids: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for QuerySyncingVsanObjectsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QuerySyncingVsanObjectsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QuerySyncingVsanObjectsRequestTypeSer<'b, 'a> {
    data: &'b QuerySyncingVsanObjectsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QuerySyncingVsanObjectsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QuerySyncingVsanObjectsRequestType")),
                1 => {
                    let Some(ref val) = self.data.uuids else { continue; };
                    return Some((std::borrow::Cow::Borrowed("uuids"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryVsanObjectUuidsByFilterRequestType<'a> {
    uuids: Option<&'a [String]>,
    limit: Option<i32>,
    version: Option<i32>,
}

impl<'a> miniserde::Serialize for QueryVsanObjectUuidsByFilterRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryVsanObjectUuidsByFilterRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryVsanObjectUuidsByFilterRequestTypeSer<'b, 'a> {
    data: &'b QueryVsanObjectUuidsByFilterRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryVsanObjectUuidsByFilterRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryVsanObjectUuidsByFilterRequestType")),
                1 => {
                    let Some(ref val) = self.data.uuids else { continue; };
                    return Some((std::borrow::Cow::Borrowed("uuids"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.limit else { continue; };
                    return Some((std::borrow::Cow::Borrowed("limit"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.version else { continue; };
                    return Some((std::borrow::Cow::Borrowed("version"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryVsanObjectsRequestType<'a> {
    uuids: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for QueryVsanObjectsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryVsanObjectsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryVsanObjectsRequestTypeSer<'b, 'a> {
    data: &'b QueryVsanObjectsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryVsanObjectsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryVsanObjectsRequestType")),
                1 => {
                    let Some(ref val) = self.data.uuids else { continue; };
                    return Some((std::borrow::Cow::Borrowed("uuids"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryVsanStatisticsRequestType<'a> {
    labels: &'a [String],
}

impl<'a> miniserde::Serialize for QueryVsanStatisticsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryVsanStatisticsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryVsanStatisticsRequestTypeSer<'b, 'a> {
    data: &'b QueryVsanStatisticsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryVsanStatisticsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryVsanStatisticsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("labels"), &self.data.labels as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ReconfigurationSatisfiableRequestType<'a> {
    pcbs: &'a [crate::types::structs::VsanPolicyChangeBatch],
    ignore_satisfiability: Option<bool>,
}

impl<'a> miniserde::Serialize for ReconfigurationSatisfiableRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReconfigurationSatisfiableRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReconfigurationSatisfiableRequestTypeSer<'b, 'a> {
    data: &'b ReconfigurationSatisfiableRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ReconfigurationSatisfiableRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReconfigurationSatisfiableRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("pcbs"), &self.data.pcbs as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.ignore_satisfiability else { continue; };
                    return Some((std::borrow::Cow::Borrowed("ignoreSatisfiability"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct ReconfigureDomObjectRequestType<'a> {
    uuid: &'a str,
    policy: &'a str,
}

impl<'a> miniserde::Serialize for ReconfigureDomObjectRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReconfigureDomObjectRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReconfigureDomObjectRequestTypeSer<'b, 'a> {
    data: &'b ReconfigureDomObjectRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ReconfigureDomObjectRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReconfigureDomObjectRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("uuid"), &self.data.uuid as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("policy"), &self.data.policy as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RunVsanPhysicalDiskDiagnosticsRequestType<'a> {
    disks: Option<&'a [String]>,
}

impl<'a> miniserde::Serialize for RunVsanPhysicalDiskDiagnosticsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RunVsanPhysicalDiskDiagnosticsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RunVsanPhysicalDiskDiagnosticsRequestTypeSer<'b, 'a> {
    data: &'b RunVsanPhysicalDiskDiagnosticsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RunVsanPhysicalDiskDiagnosticsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RunVsanPhysicalDiskDiagnosticsRequestType")),
                1 => {
                    let Some(ref val) = self.data.disks else { continue; };
                    return Some((std::borrow::Cow::Borrowed("disks"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct UpgradeVsanObjectsRequestType<'a> {
    uuids: &'a [String],
    new_version: i32,
}

impl<'a> miniserde::Serialize for UpgradeVsanObjectsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpgradeVsanObjectsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpgradeVsanObjectsRequestTypeSer<'b, 'a> {
    data: &'b UpgradeVsanObjectsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpgradeVsanObjectsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpgradeVsanObjectsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("uuids"), &self.data.uuids as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("newVersion"), &self.data.new_version as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
