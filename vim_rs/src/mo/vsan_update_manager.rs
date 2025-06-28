use std::sync::Arc;
use crate::core::client::{Client, Result};
/// Generic engine that can install VIBs onto ESX (optimized for vSAN clusters)
/// using either rolling installs or one-shot.
/// 
/// In addition to VIBs, it can also
/// install firmware updates for select hardware. The Managed Entity can be
/// accessed through MOID of vsan-update-manager, through vSAN service at ESXi
/// host side.
#[derive(Clone)]
pub struct VsanUpdateManager {
    client: Arc<Client>,
    mo_id: String,
}
impl VsanUpdateManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Deprecated as of vSphere API 8.0.
    /// 
    /// Takes a list of VIBs, firmwares, and installs those VIBs/FWs on the hosts
    /// referenced in the VIB/FW specs.
    /// 
    /// If a scan has previously been performed,
    /// the results can be passed in. If not provided, a scan is done implicitly.
    /// Install may require ESX maintenance mode, and reboots, depending on the VIB
    /// to be installed and the state of the host. Any firmware update will always
    /// require maintenance mode and a host reboot.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Optional cluster this scan is happening in reference to.
    /// 
    /// ***Required privileges:*** Host.Config.Storage Host.Config.Settings
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### vib_specs
    /// List of VIB specs to installed (context is always a single host)
    ///
    /// ### scan_results
    /// List of VIB scan results obtained from VsanVibScan()
    ///
    /// ### firmware_specs
    /// List of Firmware spec to be installed (context is always
    /// a single host and vmhba)
    ///
    /// ### maintenance_spec
    /// Maintenance spec to pass to EnterMaintenanceMode\_Task()
    /// In case of non-rolling installs, no other vSAN action
    /// than 'noAction' is allowed.
    ///
    /// ### rolling
    /// True (default) means a 1-host-at-a-time rolling install
    /// will be performed.
    ///
    /// ### no_sig_check
    /// Skip signature checking, should not be used outside of
    /// fully trusted test environments. Default: False
    ///
    /// ## Returns:
    ///
    /// Refers instance of *Task*.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vib_install_task(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, vib_specs: Option<&[crate::types::structs::VsanVibSpec]>, scan_results: Option<&[crate::types::structs::VsanVibScanResult]>, firmware_specs: Option<&[crate::types::structs::VsanHclFirmwareUpdateSpec]>, maintenance_spec: Option<&crate::types::structs::HostMaintenanceSpec>, rolling: Option<bool>, no_sig_check: Option<bool>) -> Result<crate::types::structs::ManagedObjectReference> {
        let input = VsanVibInstallRequestType {cluster, vib_specs, scan_results, firmware_specs, maintenance_spec, rolling, no_sig_check, };
        let path = format!("/vsan/VsanUpdateManager/{moId}/VsanVibInstall_Task", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Performs pre-flight checks for a VIB install.
    /// 
    /// Details of checks performed
    /// are documented in the result structure.
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Cluster for which to perform preflight check
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ## Returns:
    ///
    /// Preflight check results
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vib_install_preflight_check(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::VsanVibInstallPreflightStatus> {
        let input = VsanVibInstallPreflightCheckRequestType {cluster, };
        let path = format!("/vsan/VsanUpdateManager/{moId}/VsanVibInstallPreflightCheck", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Takes a list of VIBs and a list of hosts, and determines which VIBs would
    /// be installed, what the requirements are (e.g.
    /// 
    /// maintenance mode, reboot)
    /// and which existing VIBs (if any) they are overriding. Doesn't perform any
    /// actual install, but instead just provides information on "what if".
    ///
    /// ## Parameters:
    ///
    /// ### cluster
    /// Optional cluster this scan is happening in reference to.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ComputeResource*.
    ///
    /// ### vib_specs
    /// List of VIB specs to check (one entry per host)
    ///
    /// ## Returns:
    ///
    /// List of VIBs that would be installed. Each such result is for a
    /// single host, single VIB.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn vsan_vib_scan(&self, cluster: Option<&crate::types::structs::ManagedObjectReference>, vib_specs: &[crate::types::structs::VsanVibSpec]) -> Result<Option<Vec<crate::types::structs::VsanVibScanResult>>> {
        let input = VsanVibScanRequestType {cluster, vib_specs, };
        let path = format!("/vsan/VsanUpdateManager/{moId}/VsanVibScan", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVibInstallRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vibSpecs")]
    vib_specs: Option<&'a [crate::types::structs::VsanVibSpec]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scanResults")]
    scan_results: Option<&'a [crate::types::structs::VsanVibScanResult]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "firmwareSpecs")]
    firmware_specs: Option<&'a [crate::types::structs::VsanHclFirmwareUpdateSpec]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maintenanceSpec")]
    maintenance_spec: Option<&'a crate::types::structs::HostMaintenanceSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    rolling: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "noSigCheck")]
    no_sig_check: Option<bool>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVibInstallPreflightCheckRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct VsanVibScanRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    #[serde(rename = "vibSpecs")]
    vib_specs: &'a [crate::types::structs::VsanVibSpec],
}
