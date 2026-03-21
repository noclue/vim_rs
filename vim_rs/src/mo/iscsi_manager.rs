use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object provides interfaces for mapping VMkernel NIC to
/// iSCSI Host Bus Adapter.
#[derive(Clone)]
pub struct IscsiManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl IscsiManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Bind a Virtual NIC to be used for an iSCSI adapter
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### i_scsi_hba_name
    /// iSCSI adapter name for which the Virtual NIC to
    /// be added.
    ///
    /// ### vnic_device
    /// Virtual NIC that is to be bound to the iSCSI HBA
    ///
    /// ## Errors:
    ///
    /// ***IscsiFaultVnicAlreadyBound***: The given Virtual NIC is already bound to the HBA.
    /// 
    /// ***IscsiFaultVnicHasNoUplinks***: The given Virtual NIC has no physical uplinks.
    /// 
    /// ***IscsiFaultVnicHasMultipleUplinks***: The given Virtual NIC has multiple uplinks.
    /// 
    /// ***IscsiFaultVnicHasWrongUplink***: The given Virtual NIC has the wrong uplink and
    /// it can't be used for iSCSI multi-pathing.
    /// 
    /// ***IscsiFaultVnicNotFound***: The given Virtual NIC is not present on the system.
    /// 
    /// ***IscsiFaultInvalidVnic***: The given Virtual NIC is not valid for the HBA.
    /// 
    /// ***PlatformConfigFault***: For platform error that occurs during the operation.
    /// 
    /// ***IscsiFault***: For any problem that is not handled with a more specific fault.
    /// 
    /// ***NotFound***: If the given HBA is not found
    pub async fn bind_vnic(&self, i_scsi_hba_name: &str, vnic_device: &str) -> Result<()> {
        let input = BindVnicRequestType {i_scsi_hba_name, vnic_device, };
        self.client.invoke_void("", "IscsiManager", &self.mo_id, "BindVnic", Some(&input)).await
    }
    /// Query the list of Virtual NICs that are bound to a given iSCSI HBA.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### i_scsi_hba_name
    /// iSCSI adapter name for which the method to be
    /// applied.
    ///
    /// ## Returns:
    ///
    /// An array of *IscsiPortInfo* containing detailed
    /// information on the list of Virtual NICs bound to the adapter
    ///
    /// ## Errors:
    ///
    /// ***IscsiFault***: For any problem that is not handled with a more specific fault.
    /// 
    /// ***NotFound***: If the given HBA is not found
    pub async fn query_bound_vnics(&self, i_scsi_hba_name: &str) -> Result<Option<Vec<crate::types::structs::IscsiPortInfo>>> {
        let input = QueryBoundVnicsRequestType {i_scsi_hba_name, };
        let bytes_opt = self.client.invoke_optional("", "IscsiManager", &self.mo_id, "QueryBoundVnics", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Query the candidate Virtual NICs and Physical NICs that can be used
    /// for Port-Binding.
    /// 
    /// For dependent offload adapters, the Virtual NIC should be attached
    /// to the physical NIC associated with the hardware function.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### i_scsi_hba_name
    /// iSCSI Adapter name for which the method to be
    /// applied.
    ///
    /// ## Returns:
    ///
    /// Array of *IscsiPortInfo* containing detailed
    /// information on list of eligible Virtual NICs that can be bound
    /// to the adapter. This list will also include details on the
    /// eligible Physical NICs that are not associated with any
    /// Virtual NICs.
    ///
    /// ## Errors:
    ///
    /// ***IscsiFault***: For any problem that is not handled with a more specific fault.
    /// 
    /// ***NotFound***: If the given HBA is not found
    pub async fn query_candidate_nics(&self, i_scsi_hba_name: &str) -> Result<Option<Vec<crate::types::structs::IscsiPortInfo>>> {
        let input = QueryCandidateNicsRequestType {i_scsi_hba_name, };
        let bytes_opt = self.client.invoke_optional("", "IscsiManager", &self.mo_id, "QueryCandidateNics", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Query the dependency table for a migration operation of a given Physical
    /// NIC.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### pnic_device
    /// List of Physical NICs to be migrated
    ///
    /// ## Returns:
    ///
    /// Dependency table, as described in *IscsiMigrationDependency*,
    /// providing the user of all the Virtual NIC and iSCSI resources
    /// affected.
    pub async fn query_migration_dependencies(&self, pnic_device: &[String]) -> Result<crate::types::structs::IscsiMigrationDependency> {
        let input = QueryMigrationDependenciesRequestType {pnic_device, };
        let bytes = self.client.invoke("", "IscsiManager", &self.mo_id, "QueryMigrationDependencies", Some(&input)).await?;
        let result: crate::types::structs::IscsiMigrationDependency = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Query if Physical NIC device is used for iSCSI.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### pnic_device
    /// Physical NIC device name to check the status for
    ///
    /// ## Returns:
    ///
    /// A status object, *IscsiStatus*, indicating
    /// whether Physical NIC is used by iSCSI or not.
    /// - Empty *IscsiStatus* (i.e reason unset)
    ///   if Physical NIC device is not used.
    /// - Fault code *IscsiFaultPnicInUse* if
    ///   Physical NIC is being used.
    ///
    /// ## Errors:
    ///
    /// ***IscsiFault***: For any problem that is not handled with a more specific fault.
    pub async fn query_pnic_status(&self, pnic_device: &str) -> Result<crate::types::structs::IscsiStatus> {
        let input = QueryPnicStatusRequestType {pnic_device, };
        let bytes = self.client.invoke("", "IscsiManager", &self.mo_id, "QueryPnicStatus", Some(&input)).await?;
        let result: crate::types::structs::IscsiStatus = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Query the status of Virtual NIC association with the iSCSI.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### vnic_device
    /// Virtual NIC device to check the status for
    ///
    /// ## Returns:
    ///
    /// A status object *IscsiStatus*, containing
    /// list of the fault codes, providing the user with information as to
    /// whether Virtual NIC is used by iSCSI and list of compliance check
    /// failure codes if any. The returned *IscsiStatus*
    /// object will have an array of *MethodFault* objects providing
    /// following information:
    /// - Empty *IscsiStatus* (i.e reason unset)
    ///   if Virtual NIC device is not used.
    /// - Fault code *IscsiFaultVnicInUse* if Virtual
    ///   NIC is being used by iSCSI.
    /// - This will be followed with list of fault codes
    ///   corresponding to the compliance check failures.
    ///
    /// ## Errors:
    ///
    /// ***IscsiFault***: For any problem that is not handled with a more specific fault.
    pub async fn query_vnic_status(&self, vnic_device: &str) -> Result<crate::types::structs::IscsiStatus> {
        let input = QueryVnicStatusRequestType {vnic_device, };
        let bytes = self.client.invoke("", "IscsiManager", &self.mo_id, "QueryVnicStatus", Some(&input)).await?;
        let result: crate::types::structs::IscsiStatus = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Unbind Virtual NIC binding from an iSCSI adapter.
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### i_scsi_hba_name
    /// iSCSI adapter name for which the Virtual NIC to
    /// be removed.
    ///
    /// ### vnic_device
    /// Virtual NIC that is to be removed from the iSCSI HBA
    ///
    /// ### force
    /// -
    ///
    /// ## Errors:
    ///
    /// ***IscsiFaultVnicNotBound***: The given Virtual NIC is not bound to the adapter
    /// 
    /// ***IscsiFaultVnicHasActivePaths***: The given Virtual NIC is associated with "active" paths
    /// to the storage.
    /// 
    /// ***IscsiFaultVnicIsLastPath***: The given Virtual NIC is associated with "only" paths
    /// to the storage.
    /// 
    /// ***PlatformConfigFault***: For platform error that occurs during the operation.
    /// 
    /// ***IscsiFault***: For any problem that is not handled with a more specific fault.
    /// 
    /// ***NotFound***: If the given HBA is not found
    pub async fn unbind_vnic(&self, i_scsi_hba_name: &str, vnic_device: &str, force: bool) -> Result<()> {
        let input = UnbindVnicRequestType {i_scsi_hba_name, vnic_device, force, };
        self.client.invoke_void("", "IscsiManager", &self.mo_id, "UnbindVnic", Some(&input)).await
    }
}
struct BindVnicRequestType<'a> {
    i_scsi_hba_name: &'a str,
    vnic_device: &'a str,
}

impl<'a> miniserde::Serialize for BindVnicRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(BindVnicRequestTypeSer { data: self, seq: 0 }))
    }
}

struct BindVnicRequestTypeSer<'b, 'a> {
    data: &'b BindVnicRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for BindVnicRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"BindVnicRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("iScsiHbaName"), &self.data.i_scsi_hba_name as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("vnicDevice"), &self.data.vnic_device as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryBoundVnicsRequestType<'a> {
    i_scsi_hba_name: &'a str,
}

impl<'a> miniserde::Serialize for QueryBoundVnicsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryBoundVnicsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryBoundVnicsRequestTypeSer<'b, 'a> {
    data: &'b QueryBoundVnicsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryBoundVnicsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryBoundVnicsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("iScsiHbaName"), &self.data.i_scsi_hba_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryCandidateNicsRequestType<'a> {
    i_scsi_hba_name: &'a str,
}

impl<'a> miniserde::Serialize for QueryCandidateNicsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryCandidateNicsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryCandidateNicsRequestTypeSer<'b, 'a> {
    data: &'b QueryCandidateNicsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryCandidateNicsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryCandidateNicsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("iScsiHbaName"), &self.data.i_scsi_hba_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryMigrationDependenciesRequestType<'a> {
    pnic_device: &'a [String],
}

impl<'a> miniserde::Serialize for QueryMigrationDependenciesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryMigrationDependenciesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryMigrationDependenciesRequestTypeSer<'b, 'a> {
    data: &'b QueryMigrationDependenciesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryMigrationDependenciesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryMigrationDependenciesRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("pnicDevice"), &self.data.pnic_device as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryPnicStatusRequestType<'a> {
    pnic_device: &'a str,
}

impl<'a> miniserde::Serialize for QueryPnicStatusRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryPnicStatusRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryPnicStatusRequestTypeSer<'b, 'a> {
    data: &'b QueryPnicStatusRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryPnicStatusRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryPnicStatusRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("pnicDevice"), &self.data.pnic_device as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct QueryVnicStatusRequestType<'a> {
    vnic_device: &'a str,
}

impl<'a> miniserde::Serialize for QueryVnicStatusRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryVnicStatusRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryVnicStatusRequestTypeSer<'b, 'a> {
    data: &'b QueryVnicStatusRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryVnicStatusRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryVnicStatusRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("vnicDevice"), &self.data.vnic_device as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UnbindVnicRequestType<'a> {
    i_scsi_hba_name: &'a str,
    vnic_device: &'a str,
    force: bool,
}

impl<'a> miniserde::Serialize for UnbindVnicRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UnbindVnicRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UnbindVnicRequestTypeSer<'b, 'a> {
    data: &'b UnbindVnicRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UnbindVnicRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UnbindVnicRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("iScsiHbaName"), &self.data.i_scsi_hba_name as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("vnicDevice"), &self.data.vnic_device as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("force"), &self.data.force as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
