use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Deployment engine that allows the deploy of VCSA (Virtual Center Service
/// Appliance) onto a vSAN datastore.
/// 
/// The Managed Entity can be accessed through
/// MOID of vsan-vcsa-deployer-system at vCenter server and ESXi host side.
#[derive(Clone)]
pub struct VsanVcsaDeployerSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanVcsaDeployerSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Deployment engine that allows the deploy of VCSA (Virtual Center Service
    /// Appliance) onto a vSAN datastore.
    /// 
    /// vSAN is bootstrapped as part of this
    /// process onto the local host as a single node vSAN cluster, VCSA is deployed
    /// onto the vSAN datastore, then vCenter is configured with a datacenter, a
    /// cluster, and the local host is added to that cluster. This completes the
    /// preliminary setup of a vSAN cluster in Virtual Center.
    /// Note: To fully configure the vSAN cluster(configure network, claim disks,
    /// etc), user can use the cluster QuickStart workflow which is available on the
    /// Virtual Center UI, or invoke corresponding vSphere/vSAN APIs to accomplish
    /// the same. If user choose to invoke APIs for cluster setup, please explicitly
    /// invoke *ClusterComputeResource.AbandonHciWorkflow* API to opt out
    /// the system HCI workflow after the cluster is fully configured.
    /// 
    /// ***Required privileges:*** Host.Inventory.EditCluster
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// Bootstrap specification
    ///
    /// ## Returns:
    ///
    /// Unique Identifier for the background operation that can be used with
    /// VsanVcsaGetBootstrapProgress().
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: when specified cluster doesn't exist.
    /// 
    /// ***VsanFault***: when there are too many tasks running.
    pub async fn vsan_post_config_for_vcsa(&self, spec: &crate::types::structs::VsanVcPostDeployConfigSpec) -> Result<Option<String>> {
        let input = VsanPostConfigForVcsaRequestType {spec, };
        let path = format!("/vsan/VsanVcsaDeployerSystem/{moId}/VsanPostConfigForVcsa", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<String>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Deployment engine that allows the deploy of VCSA (Virtual Center Service
    /// Appliance) onto a vSAN datastore.
    /// 
    /// vSAN is bootstrapped as part of this
    /// process onto the local host as a single node vSAN cluster, VCSA is deployed
    /// onto the vSAN datastore, then vCenter is configured with a datacenter, a
    /// cluster, and the local host is added to that cluster to complete a fully
    /// configured vSAN cluster in Virtual Center.
    /// Note: Must be called on an ESXi host
    /// 
    /// ***Required privileges:*** VirtualMachine.Inventory.Create
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// Bootstrap specification
    ///
    /// ## Returns:
    ///
    /// Unique Identifier for the background operation that can be used with
    /// VsanVcsaGetBootstrapProgress().
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: when specified cluster doesn't exist.
    pub async fn vsan_prepare_vsan_for_vcsa(&self, spec: &crate::types::structs::VsanPrepareVsanForVcsaSpec) -> Result<Option<String>> {
        let input = VsanPrepareVsanForVcsaRequestType {spec, };
        let path = format!("/vsan/VsanVcsaDeployerSystem/{moId}/VsanPrepareVsanForVcsa", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<String>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Fetch the detailed progress of a running or recently completed VCSA
    /// deployment task.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### task_id
    /// List of unique task identifier returned by
    /// VsanPostConfigForVcsa()
    ///
    /// ## Returns:
    ///
    /// Detailed progress report for each requested task
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: when specified cluster doesn't exist.
    pub async fn vsan_vcsa_get_bootstrap_progress(&self, task_id: &[String]) -> Result<Vec<crate::types::structs::VsanVcsaDeploymentProgress>> {
        let input = VsanVcsaGetBootstrapProgressRequestType {task_id, };
        let path = format!("/vsan/VsanVcsaDeployerSystem/{moId}/VsanVcsaGetBootstrapProgress", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: Vec<crate::types::structs::VsanVcsaDeploymentProgress> = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct VsanPostConfigForVcsaRequestType<'a> {
    spec: &'a crate::types::structs::VsanVcPostDeployConfigSpec,
}

impl<'a> miniserde::Serialize for VsanPostConfigForVcsaRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanPostConfigForVcsaRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanPostConfigForVcsaRequestTypeSer<'b, 'a> {
    data: &'b VsanPostConfigForVcsaRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanPostConfigForVcsaRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanPostConfigForVcsaRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanPrepareVsanForVcsaRequestType<'a> {
    spec: &'a crate::types::structs::VsanPrepareVsanForVcsaSpec,
}

impl<'a> miniserde::Serialize for VsanPrepareVsanForVcsaRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanPrepareVsanForVcsaRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanPrepareVsanForVcsaRequestTypeSer<'b, 'a> {
    data: &'b VsanPrepareVsanForVcsaRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanPrepareVsanForVcsaRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanPrepareVsanForVcsaRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct VsanVcsaGetBootstrapProgressRequestType<'a> {
    task_id: &'a [String],
}

impl<'a> miniserde::Serialize for VsanVcsaGetBootstrapProgressRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanVcsaGetBootstrapProgressRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanVcsaGetBootstrapProgressRequestTypeSer<'b, 'a> {
    data: &'b VsanVcsaGetBootstrapProgressRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanVcsaGetBootstrapProgressRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanVcsaGetBootstrapProgressRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("taskId"), &self.data.task_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
