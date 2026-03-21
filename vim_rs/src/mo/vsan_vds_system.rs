use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// vSAN optimized methods for performing VDS related operations, especially
/// migrations from VSS to VDS.
/// 
/// In every vCenter server there is a singleton instance of this class
/// with the Managed Object ID of 'vsan-vds-system'.
#[derive(Clone)]
pub struct VsanVdsSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanVdsSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Rollback the change committed by the
    /// *VsanVdsSystem.VsanVdsMigrateVss*
    /// 
    /// ***Required privileges:*** Host.Config.Storage
    ///
    /// ## Parameters:
    ///
    /// ### task
    /// The task associated with the change committed by
    /// the VsanVdsMigrateVss
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Returns:
    ///
    /// Always return True
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: Task not found
    /// 
    /// ***VsanFault***: Any unexpected runtime error.
    pub async fn vsan_rollback_vds_to_vss(&self, task: &crate::types::structs::ManagedObjectReference) -> Result<bool> {
        let input = VsanRollbackVdsToVssRequestType {task, };
        let bytes = self.client.invoke("vsan", "VsanVdsSystem", &self.mo_id, "VsanRollbackVdsToVss", Some(&input)).await?;
        let result: bool = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Computes a migration plan to convert the VSS instances per host in
    /// the cluster to a newly created VDS.
    /// 
    /// The VDS preserves all settings
    /// of the VSS, and all consumers (vmknics, VM vNICs, pNICs) are
    /// considered as part of the migration. The VSS vSwitch and portgroup
    /// instances across the hosts are matched to each other by common name.
    /// It is expected that hosts have a uniform configuration, and issues
    /// will be raised in the migration plan if deviations are found. Any
    /// deviations will still be carried over, but may require renaming of
    /// portgroups and additional VDS portgroups to resolve the conflicts.
    /// The migration plan is defined to be safe, i.e., the effective data
    /// path for all impacted VMs, vmknics and so on are going to be
    /// using the same settings as before the migration. The only change
    /// will be in any additional pNICs added as part of the migration.  
    /// Note: This API can also be used to create a new VDS without any
    /// portgroups, but with the physical NICs attached, and with good
    /// settings for vSAN, by simply passing the vswitchName as NULL.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Hosts of this cluster will be migrated to the new VDS
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### vswitch_name
    /// Name of the original VSS vSwitch
    ///
    /// ### vds_name
    /// Name of the new VDS. Only required for the creation of
    /// a new VDS
    ///
    /// ### vmnic_devices
    /// Physical NICs that are currently unused to be
    /// attached to the new vDS.
    ///
    /// ### infra_vm
    /// Infrastructure VMs that will be handled specially
    /// during migration.
    /// 
    /// Refers instances of *VirtualMachine*.
    ///
    /// ### vds
    /// The VDS which the hosts are connected to. Set this parameter
    /// when existing VDS can be leveraged.
    /// 
    /// Refers instance of *VmwareDistributedVirtualSwitch*.
    ///
    /// ### hosts
    /// hosts in the cluster to be migrated to the VDS. Leave this
    /// field empty if all the hosts in the cluster needs to be
    /// migrated.
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// Detailed migration plan.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: when specified cluster doesn't exist.
    /// 
    /// ***VsanFault***: Any unexpected runtime error.
    /// 
    /// ***InvalidArgument***: When hosts with invalid or inconsistent
    /// configuration are found.
    pub async fn vsan_vds_get_migration_plan(&self, cluster: &crate::types::structs::ManagedObjectReference, vswitch_name: Option<&str>, vds_name: Option<&str>, vmnic_devices: Option<&[String]>, infra_vm: Option<&[crate::types::structs::ManagedObjectReference]>, vds: Option<&crate::types::structs::ManagedObjectReference>, hosts: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<crate::types::structs::VsanVdsMigrationPlan> {
        let input = VsanVdsGetMigrationPlanRequestType {cluster, vswitch_name, vds_name, vmnic_devices, infra_vm, vds, hosts, };
        let bytes = self.client.invoke("vsan", "VsanVdsSystem", &self.mo_id, "VsanVdsGetMigrationPlan", Some(&input)).await?;
        let result: crate::types::structs::VsanVdsMigrationPlan = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Perform a migration to convert the VSS instances per host in
    /// the cluster to a newly created VDS.
    /// 
    /// The VDS preserves all settings
    /// of the VSS, and all consumers (vmknics, VM vNICs, pNICs) are
    /// considered as part of the migration. The VSS vSwitch and portgroup
    /// instances across the hosts are matched to each other by common name.
    /// It is expected that hosts have a uniform configuration, and issues
    /// will be raised in the migration plan if deviations are found. Any
    /// deviations will still be carried over, but may require renaming of
    /// portgroups and additional VDS portgroups to resolve the conflicts.
    /// The migration plan is defined to be safe, i.e., the effective data
    /// path for all impacted VMs, vmknics and so on are going to be
    /// using the same settings as before the migration. The only change
    /// will be in any additional pNICs added as part of the migration.  
    /// Note: This API can also be used to create a new VDS without any
    /// portgroups, but with the physical NICs attached, and with good
    /// settings for vSAN, by simply passing the vswitchName as NULL.  
    /// If a migration plan is passed in, the API makes sure that no
    /// changes have happened and that the migration plan is still
    /// current. This can be used to make sure that the migration plan
    /// that the user reviewed doesn't deviate from the actual migration
    /// performed.  
    /// If there are any inaccessible or orphaned VMs found the migration
    /// will not be performed. This is a safety measure as such VMs may
    /// loose their networking.  
    /// If there are any inaccessible hosts in the cluster the migration
    /// will fail.  
    /// The migration will fail if there is any infraVm on ESX with
    /// version prior to 6.5.0d, due to the missing of API support on the
    /// hosts.  
    /// A minimal version of 6.0 is required for all hosts to be migrated
    /// for the NetIOC setup task. Also the VDS version will be 6.0.0 if
    /// there is any 6.0 host to be migrated, otherwise 6.5.0.
    /// 
    /// This API by default requires DVSwitch.Create and DVSwitch.Modify on
    /// RootFolder, and System.Read on cluster. Host.Inventory.EditCluster
    /// on hosts is required if hosts parameter is provided.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Hosts of this cluster will be migrated to the new VDS
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### migration_plan
    /// Detailed migration plan
    ///
    /// ### vswitch_name
    /// Name of the original VSS vswitch
    ///
    /// ### vds_name
    /// Name of the new VDS. Only required for the creation of
    /// a new VDS
    ///
    /// ### vmnic_devices
    /// Physical NICs that are currently unused to be
    /// attached to the new VDS.
    ///
    /// ### infra_vm
    /// Infrastructure VMs that will be handled specially
    /// during migration.
    /// 
    /// Refers instances of *VirtualMachine*.
    ///
    /// ### vds
    /// The VDS which the hosts are connected to. Set this parameter
    /// when existing VDS can be leveraged.
    /// 
    /// Refers instance of *VmwareDistributedVirtualSwitch*.
    ///
    /// ### hosts
    /// hosts in the cluster to be migrated to the VDS. Leave this
    /// field empty if all the hosts in the cluster needs to be
    /// migrated.
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ## Returns:
    ///
    /// The task associated with the migration
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: when specified cluster doesn't exist.
    /// 
    /// ***VsanFault***: Any unexpected runtime error.
    /// 
    /// ***InvalidArgument***: When hosts with invalid or inconsistent
    /// configuration are found.
    pub async fn vsan_vds_migrate_vss(&self, cluster: &crate::types::structs::ManagedObjectReference, migration_plan: Option<&crate::types::structs::VsanVdsMigrationPlan>, vswitch_name: Option<&str>, vds_name: Option<&str>, vmnic_devices: Option<&[String]>, infra_vm: Option<&[crate::types::structs::ManagedObjectReference]>, vds: Option<&crate::types::structs::ManagedObjectReference>, hosts: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanVdsMigrateVssRequestType {cluster, migration_plan, vswitch_name, vds_name, vmnic_devices, infra_vm, vds, hosts, };
        let bytes = self.client.invoke("vsan", "VsanVdsSystem", &self.mo_id, "VsanVdsMigrateVss", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Perform a migration to convert the VDS instance in the cluster to a newly
    /// created VSS on each host.
    /// 
    /// The VSS preserves all settings of the VDS, and
    /// all consumers (vmknics, VM vNICs, pNICs) are considered as part of the
    /// migration. The VSS vSwitch and portgroup instances on the VDS are matched
    /// to each other by common name. It is expected that hosts have a uniform
    /// configuration. The VDS will be deleted if no hosts and virtual machines
    /// are connected to this VDS after the migration completes.
    /// If there are any inaccessible or orphaned VMs found, the migration
    /// will not be performed. This is a safety measure as such VMs may
    /// lose their networking. If there are any inaccessible hosts in the cluster
    /// the migration will fail as well.
    /// 
    /// This API requires DVSwitch.Delete and DVSwitch.Modify on vds, extra
    /// privilege check might be needed depends on what is being migrated.
    /// - Host.Inventory.EditCluster on cluster if hosts of the specified cluster to be migrated to the VDS
    /// - Host.Inventory.EditCluster on all the hosts specified in hosts parameter
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Hosts of this cluster will be migrated from the specified
    /// VDS to the new VSS.
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### hosts
    /// Individual hosts to be migrated from the VDS to the VSS.
    /// 
    /// Refers instances of *HostSystem*.
    ///
    /// ### vds
    /// The VDS which the hosts in the cluster are connected to.
    /// 
    /// Refers instance of *VmwareDistributedVirtualSwitch*.
    ///
    /// ### vswitch_name
    /// Name of the new VSS to be migrated to.
    ///
    /// ### vmnic_devices
    /// Physical NICs that are currently unused to be
    /// attached to the new VSS.
    ///
    /// ### infra_vm
    /// Infrastructure VMs that will be handled specially
    /// during migration.
    /// 
    /// Refers instances of *VirtualMachine*.
    ///
    /// ## Returns:
    ///
    /// The task associated with the migration
    /// 
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: when specified cluster doesn't exist.
    /// 
    /// ***VsanFault***: Any unexpected runtime error.
    pub async fn vsan_vss_migrate_vds(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, hosts: Option<&[crate::types::structs::ManagedObjectReference]>, vds: &crate::types::structs::ManagedObjectReference, vswitch_name: Option<&str>, vmnic_devices: Option<&[String]>, infra_vm: Option<&[crate::types::structs::ManagedObjectReference]>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanVssMigrateVdsRequestType {cluster, hosts, vds, vswitch_name, vmnic_devices, infra_vm, };
        let bytes = self.client.invoke("vsan", "VsanVdsSystem", &self.mo_id, "VsanVssMigrateVds", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct VsanRollbackVdsToVssRequestType<'a> {
    task: &'a crate::types::structs::ManagedObjectReference,
}

impl<'a> miniserde::Serialize for VsanRollbackVdsToVssRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanRollbackVdsToVssRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanRollbackVdsToVssRequestTypeSer<'b, 'a> {
    data: &'b VsanRollbackVdsToVssRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanRollbackVdsToVssRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanRollbackVdsToVssRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("task"), &self.data.task as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanVdsGetMigrationPlanRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    vswitch_name: Option<&'a str>,
    vds_name: Option<&'a str>,
    vmnic_devices: Option<&'a [String]>,
    infra_vm: Option<&'a [crate::types::structs::ManagedObjectReference]>,
    vds: Option<&'a crate::types::structs::ManagedObjectReference>,
    hosts: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for VsanVdsGetMigrationPlanRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanVdsGetMigrationPlanRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanVdsGetMigrationPlanRequestTypeSer<'b, 'a> {
    data: &'b VsanVdsGetMigrationPlanRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanVdsGetMigrationPlanRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanVdsGetMigrationPlanRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.vswitch_name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vswitchName"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.vds_name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vdsName"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.vmnic_devices else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vmnicDevices"), val as &dyn miniserde::Serialize));
                }
                5 => {
                    let Some(ref val) = self.data.infra_vm else { continue; };
                    return Some((std::borrow::Cow::Borrowed("infraVm"), val as &dyn miniserde::Serialize));
                }
                6 => {
                    let Some(ref val) = self.data.vds else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vds"), val as &dyn miniserde::Serialize));
                }
                7 => {
                    let Some(ref val) = self.data.hosts else { continue; };
                    return Some((std::borrow::Cow::Borrowed("hosts"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanVdsMigrateVssRequestType<'a> {
    cluster: &'a crate::types::structs::ManagedObjectReference,
    migration_plan: Option<&'a crate::types::structs::VsanVdsMigrationPlan>,
    vswitch_name: Option<&'a str>,
    vds_name: Option<&'a str>,
    vmnic_devices: Option<&'a [String]>,
    infra_vm: Option<&'a [crate::types::structs::ManagedObjectReference]>,
    vds: Option<&'a crate::types::structs::ManagedObjectReference>,
    hosts: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for VsanVdsMigrateVssRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanVdsMigrateVssRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanVdsMigrateVssRequestTypeSer<'b, 'a> {
    data: &'b VsanVdsMigrateVssRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanVdsMigrateVssRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanVdsMigrateVssRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("cluster"), &self.data.cluster as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.migration_plan else { continue; };
                    return Some((std::borrow::Cow::Borrowed("migrationPlan"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.vswitch_name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vswitchName"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.vds_name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vdsName"), val as &dyn miniserde::Serialize));
                }
                5 => {
                    let Some(ref val) = self.data.vmnic_devices else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vmnicDevices"), val as &dyn miniserde::Serialize));
                }
                6 => {
                    let Some(ref val) = self.data.infra_vm else { continue; };
                    return Some((std::borrow::Cow::Borrowed("infraVm"), val as &dyn miniserde::Serialize));
                }
                7 => {
                    let Some(ref val) = self.data.vds else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vds"), val as &dyn miniserde::Serialize));
                }
                8 => {
                    let Some(ref val) = self.data.hosts else { continue; };
                    return Some((std::borrow::Cow::Borrowed("hosts"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanVssMigrateVdsRequestType<'a> {
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    hosts: Option<&'a [crate::types::structs::ManagedObjectReference]>,
    vds: &'a crate::types::structs::ManagedObjectReference,
    vswitch_name: Option<&'a str>,
    vmnic_devices: Option<&'a [String]>,
    infra_vm: Option<&'a [crate::types::structs::ManagedObjectReference]>,
}

impl<'a> miniserde::Serialize for VsanVssMigrateVdsRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanVssMigrateVdsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanVssMigrateVdsRequestTypeSer<'b, 'a> {
    data: &'b VsanVssMigrateVdsRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for VsanVssMigrateVdsRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanVssMigrateVdsRequestType")),
                1 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.hosts else { continue; };
                    return Some((std::borrow::Cow::Borrowed("hosts"), val as &dyn miniserde::Serialize));
                }
                3 => return Some((std::borrow::Cow::Borrowed("vds"), &self.data.vds as &dyn miniserde::Serialize)),
                4 => {
                    let Some(ref val) = self.data.vswitch_name else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vswitchName"), val as &dyn miniserde::Serialize));
                }
                5 => {
                    let Some(ref val) = self.data.vmnic_devices else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vmnicDevices"), val as &dyn miniserde::Serialize));
                }
                6 => {
                    let Some(ref val) = self.data.infra_vm else { continue; };
                    return Some((std::borrow::Cow::Borrowed("infraVm"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
