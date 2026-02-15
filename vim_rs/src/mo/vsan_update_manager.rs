use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// Generic engine that can install VIBs onto ESX (optimized for vSAN clusters)
/// using either rolling installs or one-shot.
/// 
/// In addition to VIBs, it can also
/// install firmware updates for select hardware. The Managed Entity can be
/// accessed through MOID of vsan-update-manager, through vSAN service at ESXi
/// host side.
#[derive(Clone)]
pub struct VsanUpdateManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VsanUpdateManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::ManagedObjectReference = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::VsanVibInstallPreflightStatus = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::VsanVibScanResult>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
}
struct VsanVibInstallRequestType<'a> {
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    vib_specs: Option<&'a [crate::types::structs::VsanVibSpec]>,
    scan_results: Option<&'a [crate::types::structs::VsanVibScanResult]>,
    firmware_specs: Option<&'a [crate::types::structs::VsanHclFirmwareUpdateSpec]>,
    maintenance_spec: Option<&'a crate::types::structs::HostMaintenanceSpec>,
    rolling: Option<bool>,
    no_sig_check: Option<bool>,
}

impl<'a> miniserde::Serialize for VsanVibInstallRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanVibInstallRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanVibInstallRequestTypeSer<'b, 'a> {
    data: &'b VsanVibInstallRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanVibInstallRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanVibInstallRequestType")),
                1 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.vib_specs else { continue; };
                    return Some((std::borrow::Cow::Borrowed("vibSpecs"), val as &dyn miniserde::Serialize));
                }
                3 => {
                    let Some(ref val) = self.data.scan_results else { continue; };
                    return Some((std::borrow::Cow::Borrowed("scanResults"), val as &dyn miniserde::Serialize));
                }
                4 => {
                    let Some(ref val) = self.data.firmware_specs else { continue; };
                    return Some((std::borrow::Cow::Borrowed("firmwareSpecs"), val as &dyn miniserde::Serialize));
                }
                5 => {
                    let Some(ref val) = self.data.maintenance_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("maintenanceSpec"), val as &dyn miniserde::Serialize));
                }
                6 => {
                    let Some(ref val) = self.data.rolling else { continue; };
                    return Some((std::borrow::Cow::Borrowed("rolling"), val as &dyn miniserde::Serialize));
                }
                7 => {
                    let Some(ref val) = self.data.no_sig_check else { continue; };
                    return Some((std::borrow::Cow::Borrowed("noSigCheck"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanVibInstallPreflightCheckRequestType<'a> {
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for VsanVibInstallPreflightCheckRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanVibInstallPreflightCheckRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanVibInstallPreflightCheckRequestTypeSer<'b, 'a> {
    data: &'b VsanVibInstallPreflightCheckRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanVibInstallPreflightCheckRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanVibInstallPreflightCheckRequestType")),
                1 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct VsanVibScanRequestType<'a> {
    cluster: Option<&'a crate::types::structs::ManagedObjectReference>,
    vib_specs: &'a [crate::types::structs::VsanVibSpec],
}

impl<'a> miniserde::Serialize for VsanVibScanRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VsanVibScanRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VsanVibScanRequestTypeSer<'b, 'a> {
    data: &'b VsanVibScanRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VsanVibScanRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VsanVibScanRequestType")),
                1 => {
                    let Some(ref val) = self.data.cluster else { continue; };
                    return Some((std::borrow::Cow::Borrowed("cluster"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("vibSpecs"), &self.data.vib_specs as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
