use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::HostVsanInternalSystemCmmdsQuery;
use crate::types::structs::HostVsanInternalSystemDeleteVsanObjectsResult;
use crate::types::structs::HostVsanInternalSystemVsanObjectOperationResult;
use crate::types::structs::HostVsanInternalSystemVsanPhysicalDiskDiagnosticsResult;
use crate::types::structs::VsanNewPolicyBatch;
use crate::types::structs::VsanPolicyChangeBatch;
use crate::types::structs::VsanPolicySatisfiability;
/// The VsanInternalSystem exposes low level access to CMMDS, as well as draft
/// versions of VSAN object and disk management APIs that are subject to change
/// in future releases.
/// 
/// No compatibility is guaranteed on any of the APIs,
/// including their prototype, behavior or result encoding.
pub struct HostVsanInternalSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl HostVsanInternalSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
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
        let path = format!("/HostVsanInternalSystem/{moId}/AbdicateDomOwnership", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
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
    pub async fn can_provision_objects(&self, npbs: &[VsanNewPolicyBatch], ignore_satisfiability: Option<bool>) -> Result<Vec<VsanPolicySatisfiability>> {
        let input = CanProvisionObjectsRequestType {npbs, ignore_satisfiability, };
        let path = format!("/HostVsanInternalSystem/{moId}/CanProvisionObjects", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn delete_vsan_objects(&self, uuids: &[String], force: Option<bool>) -> Result<Vec<HostVsanInternalSystemDeleteVsanObjectsResult>> {
        let input = DeleteVsanObjectsRequestType {uuids, force, };
        let path = format!("/HostVsanInternalSystem/{moId}/DeleteVsanObjects", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
        let path = format!("/HostVsanInternalSystem/{moId}/GetVsanObjExtAttrs", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn query_cmmds(&self, queries: &[HostVsanInternalSystemCmmdsQuery]) -> Result<String> {
        let input = QueryCmmdsRequestType {queries, };
        let path = format!("/HostVsanInternalSystem/{moId}/QueryCmmds", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
        let path = format!("/HostVsanInternalSystem/{moId}/QueryObjectsOnPhysicalVsanDisk", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
        let path = format!("/HostVsanInternalSystem/{moId}/QueryPhysicalVsanDisks", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
        let path = format!("/HostVsanInternalSystem/{moId}/QuerySyncingVsanObjects", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
        let path = format!("/HostVsanInternalSystem/{moId}/QueryVsanObjectUuidsByFilter", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
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
        let path = format!("/HostVsanInternalSystem/{moId}/QueryVsanObjects", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
        let path = format!("/HostVsanInternalSystem/{moId}/QueryVsanStatistics", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn reconfiguration_satisfiable(&self, pcbs: &[VsanPolicyChangeBatch], ignore_satisfiability: Option<bool>) -> Result<Vec<VsanPolicySatisfiability>> {
        let input = ReconfigurationSatisfiableRequestType {pcbs, ignore_satisfiability, };
        let path = format!("/HostVsanInternalSystem/{moId}/ReconfigurationSatisfiable", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
        let path = format!("/HostVsanInternalSystem/{moId}/ReconfigureDomObject", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
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
    pub async fn run_vsan_physical_disk_diagnostics(&self, disks: Option<&[String]>) -> Result<Vec<HostVsanInternalSystemVsanPhysicalDiskDiagnosticsResult>> {
        let input = RunVsanPhysicalDiskDiagnosticsRequestType {disks, };
        let path = format!("/HostVsanInternalSystem/{moId}/RunVsanPhysicalDiskDiagnostics", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
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
    pub async fn upgrade_vsan_objects(&self, uuids: &[String], new_version: i32) -> Result<Option<Vec<HostVsanInternalSystemVsanObjectOperationResult>>> {
        let input = UpgradeVsanObjectsRequestType {uuids, new_version, };
        let path = format!("/HostVsanInternalSystem/{moId}/UpgradeVsanObjects", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AbdicateDomOwnershipRequestType<'a> {
    uuids: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CanProvisionObjectsRequestType<'a> {
    npbs: &'a [VsanNewPolicyBatch],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ignoreSatisfiability")]
    ignore_satisfiability: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DeleteVsanObjectsRequestType<'a> {
    uuids: &'a [String],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct GetVsanObjExtAttrsRequestType<'a> {
    uuids: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryCmmdsRequestType<'a> {
    queries: &'a [HostVsanInternalSystemCmmdsQuery],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryObjectsOnPhysicalVsanDiskRequestType<'a> {
    disks: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryPhysicalVsanDisksRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    props: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QuerySyncingVsanObjectsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    uuids: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryVsanObjectUuidsByFilterRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    uuids: Option<&'a [String]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    version: Option<i32>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryVsanObjectsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    uuids: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct QueryVsanStatisticsRequestType<'a> {
    labels: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReconfigurationSatisfiableRequestType<'a> {
    pcbs: &'a [VsanPolicyChangeBatch],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ignoreSatisfiability")]
    ignore_satisfiability: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReconfigureDomObjectRequestType<'a> {
    uuid: &'a str,
    policy: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RunVsanPhysicalDiskDiagnosticsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    disks: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpgradeVsanObjectsRequestType<'a> {
    uuids: &'a [String],
    #[serde(rename = "newVersion")]
    new_version: i32,
}
