use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// FailoverClusterConfigurator provides operations to create and configure
/// a vCenter High Availability Cluster (VCHA Cluster).
/// 
/// A VCHA Cluster consists of three VMs. One is the Active vCenter VM that
/// serves client requests. Second is the Passive VM that is identical to the
/// Active vCenter VM in terms of resources and capabilities. Passive VM
/// constantly receives updates from Active VM and takes over the role of
/// Active vCenter VM in the event of failover. Third is the Witness VM that
/// acts as a quorum VM in a VCHA Cluster. Sole purpose of Witness VM is to
/// avoid classic split-brain problem in a VCHA Cluster.
/// A VCHA Cluster can be deployed in two possible ways -
/// 1\. Automatic deployment - Configuration (described below) inputs for each
/// node in a VCHA Cluster is provided by the user. A Deployment workflow
/// is used that automatically deploys Passive and Witness VM and configures
/// each node to form a VCHA Cluster.
/// 2\. Manual deployment - User provisions and creates Passive and Witness
/// VMs and uses Configuration workflow to configure Active, Passive and
/// Witness VM to form a VCHA Cluster. Passive and Witness VMs must be
/// created using the VM-Clone operation with Active VM as the source.
#[derive(Clone)]
pub struct FailoverClusterConfigurator {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl FailoverClusterConfigurator {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Configure VCHA on the local vCenter Server.
    /// 
    /// This operation configures the VC appliance with VCHA specific inputs
    /// and uses already deployed Passive and Witness nodes to set up the
    /// VCHA cluster. After configuration, the VCHA Cluster is enabled
    /// on a best effort basis, but if this operation does not succeed,
    /// *FailoverClusterManager.setClusterMode_Task* must be called
    /// to enable it. State replication or failover is not allowed
    /// until the VCHA Cluster is enabled. The current vCenter Server
    /// continues to serve client requests during and after the configuration.
    /// 
    /// ***Required privileges:*** Global.VCServer
    ///
    /// ## Parameters:
    ///
    /// ### config_spec
    /// contains the configuration for the cluster
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the progress of the operation.
    /// 
    /// Refers instance of *Task*.
    pub async fn configure_vcha_task(&self, config_spec: &crate::types::structs::VchaClusterConfigSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = ConfigureVchaRequestType {config_spec, };
        let bytes = self.client.invoke("", "FailoverClusterConfigurator", &self.mo_id, "configureVcha_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Creates a Passive node in a degraded VCHA Cluster with node location
    /// information and pre-existing VCHA Cluster configuration from the
    /// Active node.
    /// 
    /// ***Required privileges:*** Global.VCServer
    ///
    /// ## Parameters:
    ///
    /// ### passive_deployment_spec
    /// contains deployment specification for the
    /// passive node
    ///
    /// ### source_vc_spec
    /// contains specification for the source vCenter
    /// server that is used to create Passive node
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the progress of the operation.
    /// 
    /// Refers instance of *Task*.
    pub async fn create_passive_node_task(&self, passive_deployment_spec: &crate::types::structs::PassiveNodeDeploymentSpec, source_vc_spec: &crate::types::structs::SourceNodeSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreatePassiveNodeRequestType {passive_deployment_spec, source_vc_spec, };
        let bytes = self.client.invoke("", "FailoverClusterConfigurator", &self.mo_id, "createPassiveNode_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Creates a Witness node in a degraded VCHA Cluster with node location
    /// information and pre-existing VCHA Cluster configuration from the
    /// Active node.
    /// 
    /// ***Required privileges:*** Global.VCServer
    ///
    /// ## Parameters:
    ///
    /// ### witness_deployment_spec
    /// contains deployment specification for the
    /// witness node
    ///
    /// ### source_vc_spec
    /// contains specification for the source vCenter
    /// server that is used to create Witness node
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the progress of the operation.
    /// 
    /// Refers instance of *Task*.
    pub async fn create_witness_node_task(&self, witness_deployment_spec: &dyn crate::types::traits::NodeDeploymentSpecTrait, source_vc_spec: &crate::types::structs::SourceNodeSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = CreateWitnessNodeRequestType {witness_deployment_spec, source_vc_spec, };
        let bytes = self.client.invoke("", "FailoverClusterConfigurator", &self.mo_id, "createWitnessNode_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Deploys and Configures VCHA on the local vCenter as a single API.
    /// 
    /// This deployment operation automatically provisions and creates a
    /// Passive and a Witness node followed by configuring each node such that
    /// a 3 node VCHA Cluster is formed. After configuration, the VCHA Cluster
    /// is enabled on a best effort basis, but if this operation does not
    /// succeed, *FailoverClusterManager.setClusterMode_Task* must be
    /// called to enable it. State replication or failover is not allowed
    /// until the VCHA Cluster is enabled. The current vCenter Server continues
    /// to serve client requests during and after the deployment.
    /// If the activeVcNetworkConfig spec if filled in, the cluster network
    /// will be created and configured.
    /// No changes will be made if the cluster network is already configured.
    /// 
    /// ***Required privileges:*** Global.VCServer
    ///
    /// ## Parameters:
    ///
    /// ### deployment_spec
    /// contains the information needed to deploy and
    /// configure a VCHA Cluster
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the progress of the operation.
    /// 
    /// Refers instance of *Task*.
    pub async fn deploy_vcha_task(&self, deployment_spec: &crate::types::structs::VchaClusterDeploymentSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = DeployVchaRequestType {deployment_spec, };
        let bytes = self.client.invoke("", "FailoverClusterConfigurator", &self.mo_id, "deployVcha_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Destroys the VCHA cluster setup and removes all VCHA specific
    /// configuration from the VCVA appliance.
    /// 
    /// The active node in the cluster
    /// continues to run as a standalone VCVA appliance after the destroy
    /// operation has been performed.
    /// This operation is allowed under the following circumstances:
    /// \- VCHA cluster is disabled
    /// \- The node is in an isolated state
    /// \- VCHA Deploy/Configure has failed
    /// 
    /// ***Required privileges:*** Global.VCServer
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    pub async fn destroy_vcha_task(&self) -> Result<crate::types::structs::ManagedObjectReference> {
        let bytes = self.client.invoke("", "FailoverClusterConfigurator", &self.mo_id, "destroyVcha_Task", None).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Returns the configuration information for each node that is part of
    /// the VCHA Cluster.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// Returns a data structure specifying configuration for Active,
    /// Passive and Witness node in the Cluster.
    pub async fn get_vcha_config(&self) -> Result<crate::types::structs::VchaClusterConfigInfo> {
        let bytes = self.client.invoke("", "FailoverClusterConfigurator", &self.mo_id, "getVchaConfig", None).await?;
        let result: crate::types::structs::VchaClusterConfigInfo = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Prepares the vCenter appliance for a VCHA cluster deployment.
    /// 
    /// This preparation operation saves the network configuration of the cluster
    /// and configures the Active node to be cloned for a VCHA configuration.
    /// Prepares the VCHA Active node for ssh keys, vpostgres replication
    /// and related configuration file setup needed for a VCHA cluster.
    /// If the Active node Cluster network adapter does not exist
    /// the prepare operation will fail.
    /// No changes will be made if the cluster is already configured.
    /// 
    /// ***Required privileges:*** Global.VCServer
    ///
    /// ## Parameters:
    ///
    /// ### network_spec
    /// contains the information needed to prepare
    /// a VCHA Cluster and configure networking.
    ///
    /// ## Returns:
    ///
    /// This method returns a *Task* object with which to
    /// monitor the progress of the operation.
    /// 
    /// Refers instance of *Task*.
    pub async fn prepare_vcha_task(&self, network_spec: &crate::types::structs::VchaClusterNetworkSpec) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = PrepareVchaRequestType {network_spec, };
        let bytes = self.client.invoke("", "FailoverClusterConfigurator", &self.mo_id, "prepareVcha_Task", Some(&input)).await?;
        let result: crate::types::structs::ManagedObjectReference = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// A list of method names that must not be called and will throw
    /// a fault due to some other method running that the disabled method
    /// can cause side-effects for.
    /// 
    /// This list may include the following methods:
    /// - *FailoverClusterConfigurator.deployVcha_Task*
    /// - *FailoverClusterConfigurator.configureVcha_Task*
    /// - *FailoverClusterConfigurator.createPassiveNode_Task*
    /// - *FailoverClusterConfigurator.createWitnessNode_Task*
    /// - *FailoverClusterConfigurator.destroyVcha_Task*
    ///   
    /// As with other disabled methods there will be no property updates
    /// on this property when called with non-zero property collector versions.
    pub async fn disabled_configure_method(&self) -> Result<Option<Vec<String>>> {
        let pv_opt = self.client.fetch_property_raw("", "FailoverClusterConfigurator", &self.mo_id, "disabledConfigureMethod").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
            None => Ok(None),
        }
    }
}
struct ConfigureVchaRequestType<'a> {
    config_spec: &'a crate::types::structs::VchaClusterConfigSpec,
}

impl<'a> miniserde::Serialize for ConfigureVchaRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ConfigureVchaRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ConfigureVchaRequestTypeSer<'b, 'a> {
    data: &'b ConfigureVchaRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ConfigureVchaRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"configureVchaRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("configSpec"), &self.data.config_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreatePassiveNodeRequestType<'a> {
    passive_deployment_spec: &'a crate::types::structs::PassiveNodeDeploymentSpec,
    source_vc_spec: &'a crate::types::structs::SourceNodeSpec,
}

impl<'a> miniserde::Serialize for CreatePassiveNodeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreatePassiveNodeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreatePassiveNodeRequestTypeSer<'b, 'a> {
    data: &'b CreatePassiveNodeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreatePassiveNodeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"createPassiveNodeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("passiveDeploymentSpec"), &self.data.passive_deployment_spec as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("sourceVcSpec"), &self.data.source_vc_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateWitnessNodeRequestType<'a> {
    witness_deployment_spec: &'a dyn crate::types::traits::NodeDeploymentSpecTrait,
    source_vc_spec: &'a crate::types::structs::SourceNodeSpec,
}

impl<'a> miniserde::Serialize for CreateWitnessNodeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateWitnessNodeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateWitnessNodeRequestTypeSer<'b, 'a> {
    data: &'b CreateWitnessNodeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateWitnessNodeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"createWitnessNodeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("witnessDeploymentSpec"), &self.data.witness_deployment_spec as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("sourceVcSpec"), &self.data.source_vc_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DeployVchaRequestType<'a> {
    deployment_spec: &'a crate::types::structs::VchaClusterDeploymentSpec,
}

impl<'a> miniserde::Serialize for DeployVchaRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DeployVchaRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DeployVchaRequestTypeSer<'b, 'a> {
    data: &'b DeployVchaRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DeployVchaRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"deployVchaRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("deploymentSpec"), &self.data.deployment_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct PrepareVchaRequestType<'a> {
    network_spec: &'a crate::types::structs::VchaClusterNetworkSpec,
}

impl<'a> miniserde::Serialize for PrepareVchaRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(PrepareVchaRequestTypeSer { data: self, seq: 0 }))
    }
}

struct PrepareVchaRequestTypeSer<'b, 'a> {
    data: &'b PrepareVchaRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for PrepareVchaRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"prepareVchaRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("networkSpec"), &self.data.network_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
