use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object type controls entitlements for a given VMware
/// platform.
/// 
/// VMware platforms include VirtualCenter, ESX Server, VMware Server,
/// Workstation and Player. Entitlements define what software capabilities
/// this host may use.
/// 
/// Entitlements are identified by a short string 'key'. Keys can represent either
/// a particular edition (Full, Starter) or a particular feature/function (featureKey)
/// (backup, nas). An edition implies zero one or more functions which are express,
/// denied or optional. For example a 'Full' edition includes 'iscsi' function but a
/// Starter edition might disallow it.
/// 
/// Which edition a given VMware platform uses can be defined at any time. Generally this
/// is done right after first install and boot as installation software may not set it.
/// For editions that are similar in nature, any future changes to edition
/// type will only impact future requests for functionality.
/// Current functionality is left unaffected. The same is true for optional
/// functions enabled/disabled after some period of time. For dissimilar editions,
/// such transitions may require entering maintenance mode first else an exception of
/// InvalidState will be thrown.
/// 
/// To specify the edition type and any optional functions, use updateLicense for
/// ESX Server and addLicense follow by LicenseAssingmentManager.updateAssignedLicense
/// for VirtualCenter.
/// 
/// When an edition is specified for a given host, the cost of that edition
/// (how many licenses are needed) is determined. The cost is computed
/// using the license's CostUnit value multiplied by the number of units activated.
/// For example, when a VMware platform is set to an edition which uses a 'cpuPackage'
/// on a two socket server, two licenses would be needed to successfully
/// install that edition.
/// 
/// Here is a diagram of the unit costs supported by this API and their relationships.
/// 
///        +------------------------------+   +--------+      +-------+
///        | +-----------+ +-----------+  |   | Server |      |  Host |
///        | |           | |           |  |   +--------+      +-------+
///        | |  cpuCore  | |   cpuCore |  |                   +-------+
///        | +-----------+ +-----------+  |   +--------+      |  Host |
///        |                  cpuPackage  |   |  VM    |      +-------+
///        +------------------------------+   +--------+
#[derive(Clone)]
pub struct LicenseManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl LicenseManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Adds a license to the inventory of available licenses.
    /// 
    /// ***Required privileges:*** Global.Licenses
    ///
    /// ## Parameters:
    ///
    /// ### license_key
    /// A license. E.g. a serial license.
    ///
    /// ### labels
    /// array of key-value labels. Ignored by ESX Server.
    ///
    /// ## Returns:
    ///
    /// Returns information about the license specified in licenseKey.
    pub async fn add_license(&self, license_key: &str, labels: Option<&[crate::types::structs::KeyValue]>) -> Result<crate::types::structs::LicenseManagerLicenseInfo> {
        let input = AddLicenseRequestType {license_key, labels, };
        let path = format!("/LicenseManager/{moId}/AddLicense", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::LicenseManagerLicenseInfo = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of vSphere API 4.0, use
    /// *LicenseAssignmentManager.QueryAssignedLicenses* instead.
    /// 
    /// Returns whether or not a given feature is enabled.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Host to act on if LicenseManager is not on a host.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### feature_key
    /// Name of the feature to enable.
    ///
    /// ## Returns:
    ///
    /// Returns true if the feature is enabled and false if it is not.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If a feature name is not found.
    /// 
    /// ***InvalidState***: If the feature cannot be supported on the platform,
    /// potentially because the hardware configuration does not support it.
    pub async fn check_license_feature(&self, host: Option<&crate::types::structs::ManagedObjectReference>, feature_key: &str) -> Result<bool> {
        let input = CheckLicenseFeatureRequestType {host, feature_key, };
        let path = format!("/LicenseManager/{moId}/CheckLicenseFeature", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: bool = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of vSphere API 4.0, use *LicenseManager.UpdateLicense*
    /// instead.
    /// 
    /// Allows for reconfiguration of the License Manager license source.
    /// 
    /// This changes the licensing source to be either served or local. Before changing
    /// the license source location, the API checks the number of licenses available at
    /// the new potential source to ensure there are at least as many licenses there as
    /// have been issued by the current source. If there are an equal or greater number of
    /// licenses at the new source, all licenses on the current source are released and
    /// then reacquired from the new source. If there are not enough licenses available on
    /// the new source to reissue all licenses, the operation fails.
    /// 
    /// This is used to configure the license source on an individual host.
    /// 
    /// **PLATFORM Specific Notes:**
    /// VirtualCenter - only supports a served source.
    /// the host parameter is mandatory.
    /// ESX Server - the host parameter is optional.
    /// 
    /// ***Required privileges:*** Global.Licenses
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Host for which the license manager should be reconfigured.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### license_source
    /// ServedSource or LocalSource.
    ///
    /// ## Errors:
    ///
    /// ***LicenseServerUnavailable***: if the license server is unreachable.
    /// 
    /// ***CannotAccessLocalSource***: if the local source cannot be accessed.
    /// 
    /// ***InvalidLicense***: if the new license source is LocalLicenseSource and the
    /// license file is not valid.
    /// 
    /// ***NotEnoughLicenses***: if the new license source does not have enough licenses.
    pub async fn configure_license_source(&self, host: Option<&crate::types::structs::ManagedObjectReference>, license_source: &dyn crate::types::traits::LicenseSourceTrait) -> Result<()> {
        let input = ConfigureLicenseSourceRequestType {host, license_source, };
        let path = format!("/LicenseManager/{moId}/ConfigureLicenseSource", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Decodes licensing information on the license specified.
    /// 
    /// ***Required privileges:*** Global.Licenses
    ///
    /// ## Parameters:
    ///
    /// ### license_key
    /// A license. E.g. a serial license.
    ///
    /// ## Returns:
    ///
    /// Returns information about the license specified in licenseKey.
    pub async fn decode_license(&self, license_key: &str) -> Result<crate::types::structs::LicenseManagerLicenseInfo> {
        let input = DecodeLicenseRequestType {license_key, };
        let path = format!("/LicenseManager/{moId}/DecodeLicense", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::LicenseManagerLicenseInfo = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of vSphere API 4.0, use
    /// *LicenseAssignmentManager.RemoveAssignedLicense* instead.
    /// 
    /// Release licenses for an optional feature.
    /// 
    /// ***Required privileges:*** Global.Licenses
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Host to act on if LicenseManager is not on a host.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### feature_key
    /// key of the feature to disable.
    ///
    /// ## Returns:
    ///
    /// Returns true if the feature was disabled and false if not.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If a feature name is not found or it is not optional.
    /// 
    /// ***LicenseServerUnavailable***: If the license server is unavailable.
    /// 
    /// ***InvalidState***: If the feature is in use.
    pub async fn disable_feature(&self, host: Option<&crate::types::structs::ManagedObjectReference>, feature_key: &str) -> Result<bool> {
        let input = DisableFeatureRequestType {host, feature_key, };
        let path = format!("/LicenseManager/{moId}/DisableFeature", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: bool = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of vSphere API 4.0, use
    /// *LicenseAssignmentManager.UpdateAssignedLicense* instead.
    /// 
    /// Enable a feature that has an optional state.
    /// 
    /// ***Required privileges:*** Global.Licenses
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Host to act on if LicenseManager is not on a host.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### feature_key
    /// Name of the feature to enable.
    ///
    /// ## Returns:
    ///
    /// Returns true if enabling the feature was successful, false otherwise.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If a feature name is not found or it is not optional.
    /// 
    /// ***LicenseServerUnavailable***: If the license server is unavailable.
    /// 
    /// ***InvalidState***: If the feature cannot be supported on the platform,
    /// potentially because the hardware configuration does not support it.
    pub async fn enable_feature(&self, host: Option<&crate::types::structs::ManagedObjectReference>, feature_key: &str) -> Result<bool> {
        let input = EnableFeatureRequestType {host, feature_key, };
        let path = format!("/LicenseManager/{moId}/EnableFeature", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: bool = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of vSphere API 4.0, use
    /// *LicenseAssignmentManager.QueryAssignedLicenses* instead.
    /// 
    /// Queries the current license source for total and available licenses available for
    /// each feature known to this system.
    /// 
    /// ***Required privileges:*** Global.Licenses
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Use the license source of the specified host.
    /// 
    /// Refers instance of *HostSystem*.
    pub async fn query_license_source_availability(&self, host: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<crate::types::structs::LicenseAvailabilityInfo>>> {
        let input = QueryLicenseSourceAvailabilityRequestType {host, };
        let path = format!("/LicenseManager/{moId}/QueryLicenseSourceAvailability", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::LicenseAvailabilityInfo>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Deprecated as of vSphere API 4.0, use
    /// *LicenseAssignmentManager.QueryAssignedLicenses* instead.
    /// 
    /// Queries the current license source for a list of available licenses that can be
    /// licensed from this system.
    /// 
    /// ***Required privileges:*** Global.Licenses
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Use the license source of the specified host.
    /// 
    /// Refers instance of *HostSystem*.
    pub async fn query_supported_features(&self, host: Option<&crate::types::structs::ManagedObjectReference>) -> Result<Option<Vec<crate::types::structs::LicenseFeatureInfo>>> {
        let input = QuerySupportedFeaturesRequestType {host, };
        let path = format!("/LicenseManager/{moId}/QuerySupportedFeatures", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::LicenseFeatureInfo>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Deprecated as of vSphere API 4.0, use
    /// *LicenseAssignmentManager.QueryAssignedLicenses* instead.
    /// 
    /// Returns the license usage.
    /// 
    /// The license usage is a list of supported features and
    /// the number of licenses that have been reserved.
    /// 
    /// **PLATFORM Specific Notes:**
    /// VirtualCenter - Empty string returns the usage of non-host specific features.
    /// Must specify managed host to query.
    /// ESX Server - Host argument ignored.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Host to query for usage. If missing, query the server the
    /// *LicenseManager* is on.
    /// 
    /// Refers instance of *HostSystem*.
    pub async fn query_license_usage(&self, host: Option<&crate::types::structs::ManagedObjectReference>) -> Result<crate::types::structs::LicenseUsageInfo> {
        let input = QueryLicenseUsageRequestType {host, };
        let path = format!("/LicenseManager/{moId}/QueryLicenseUsage", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::LicenseUsageInfo = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Removed a license's label.
    /// 
    /// ***Required privileges:*** Global.Licenses
    ///
    /// ## Parameters:
    ///
    /// ### license_key
    /// A license.
    ///
    /// ### label_key
    /// A label key.
    pub async fn remove_license_label(&self, license_key: &str, label_key: &str) -> Result<()> {
        let input = RemoveLicenseLabelRequestType {license_key, label_key, };
        let path = format!("/LicenseManager/{moId}/RemoveLicenseLabel", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Remove license from the available set.
    /// 
    /// ***Required privileges:*** Global.Licenses
    ///
    /// ## Parameters:
    ///
    /// ### license_key
    /// A licenses. E.g. a serial license.
    pub async fn remove_license(&self, license_key: &str) -> Result<()> {
        let input = RemoveLicenseRequestType {license_key, };
        let path = format!("/LicenseManager/{moId}/RemoveLicense", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Deprecated as of vSphere API 4.0, use
    /// *LicenseAssignmentManager.QueryAssignedLicenses* instead.
    /// 
    /// Defines the product's license edition.
    /// 
    /// The edition defines which product license
    /// the server requires. This, in turn, determines the core set of functionality
    /// provided by the product and the additional features that can be licensed.
    /// 
    /// To determine what featureKey the current platform will accept, use
    /// querySourceAvailablity() at runtime, or consult the documentation for the
    /// current platform.
    /// 
    /// ***Required privileges:*** Global.Licenses
    ///
    /// ## Parameters:
    ///
    /// ### host
    /// Host to act on if LicenseManager is not on a host.
    /// 
    /// Refers instance of *HostSystem*.
    ///
    /// ### feature_key
    /// Name of edition feature to select. If featureKey is not
    /// set or set to empty string, the product becomes unlicensed.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If a feature key is not an edition feature key.
    /// 
    /// ***LicenseServerUnavailable***: If the license server is unavailable.
    /// 
    /// ***InvalidState***: If the feature cannot be supported on the platform,
    /// potentially because the hardware configuration does not support it.
    pub async fn set_license_edition(&self, host: Option<&crate::types::structs::ManagedObjectReference>, feature_key: Option<&str>) -> Result<()> {
        let input = SetLicenseEditionRequestType {host, feature_key, };
        let path = format!("/LicenseManager/{moId}/SetLicenseEdition", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Update a license's label.
    /// 
    /// It creates a label entry if the labelKey doesn't already exist
    /// 
    /// ***Required privileges:*** Global.Licenses
    ///
    /// ## Parameters:
    ///
    /// ### license_key
    /// A license.
    ///
    /// ### label_key
    /// A label key.
    ///
    /// ### label_value
    /// Value for the label.
    pub async fn update_license_label(&self, license_key: &str, label_key: &str, label_value: &str) -> Result<()> {
        let input = UpdateLicenseLabelRequestType {license_key, label_key, label_value, };
        let path = format!("/LicenseManager/{moId}/UpdateLicenseLabel", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Updates the available licenses to the one provided in licenseKey.
    /// 
    /// This is the same as removing all the licenses using
    /// *LicenseManager.RemoveLicense* and adding licenseKey using
    /// *LicenseManager.AddLicense*
    /// If the optional parameter labels is specify this is the same as calling
    /// updateLicense without the optioal parameter and calling updateLabel for each pair
    /// in the labels array.
    /// 
    /// ***Required privileges:*** Global.Licenses
    ///
    /// ## Parameters:
    ///
    /// ### license_key
    /// A license. E.g. a serial license.
    ///
    /// ### labels
    /// array of key-value labels.
    ///
    /// ## Returns:
    ///
    /// Returns information about the license specified in licenseKey.
    pub async fn update_license(&self, license_key: &str, labels: Option<&[crate::types::structs::KeyValue]>) -> Result<crate::types::structs::LicenseManagerLicenseInfo> {
        let input = UpdateLicenseRequestType {license_key, labels, };
        let path = format!("/LicenseManager/{moId}/UpdateLicense", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::LicenseManagerLicenseInfo = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of vSphere API 4.0, this property is not used by the system.
    /// 
    /// Return current diagnostic information.
    pub async fn diagnostics(&self) -> Result<Option<crate::types::structs::LicenseDiagnostics>> {
        let path = format!("/LicenseManager/{moId}/diagnostics", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::LicenseDiagnostics>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// ***Required privileges:*** System.Read
    pub async fn evaluation(&self) -> Result<crate::types::structs::LicenseManagerEvaluationInfo> {
        let path = format!("/LicenseManager/{moId}/evaluation", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::LicenseManagerEvaluationInfo = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of VI API 2.5, use *LicenseManager.QuerySupportedFeatures*
    /// instead.
    /// 
    /// The list of features that can be licensed.
    pub async fn feature_info(&self) -> Result<Option<Vec<crate::types::structs::LicenseFeatureInfo>>> {
        let path = format!("/LicenseManager/{moId}/featureInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::LicenseFeatureInfo>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// License Assignment Manager
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instance of *LicenseAssignmentManager*.
    pub async fn license_assignment_manager(&self) -> Result<Option<crate::types::structs::ManagedObjectReference>> {
        let path = format!("/LicenseManager/{moId}/licenseAssignmentManager", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::ManagedObjectReference>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Deprecated as of vSphere API 4.0, use
    /// *LicenseAssignmentManager.QueryAssignedLicenses* instead.
    /// 
    /// The product's license edition.
    /// 
    /// The edition defines which product license
    /// the server requires. This, in turn, determines the core set of functionalities
    /// provided by the product and the additional features that can be licensed. If
    /// no edition is set the property is set to the empty string ("").
    /// To set the edition use *LicenseManager.SetLicenseEdition*.
    pub async fn licensed_edition(&self) -> Result<String> {
        let path = format!("/LicenseManager/{moId}/licensedEdition", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: String = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Get information about all the licenses available.
    pub async fn licenses(&self) -> Result<Vec<crate::types::structs::LicenseManagerLicenseInfo>> {
        let path = format!("/LicenseManager/{moId}/licenses", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: Vec<crate::types::structs::LicenseManagerLicenseInfo> = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of vSphere API 4.0, use
    /// *LicenseAssignmentManager.QueryAssignedLicenses* to get evaluation information.
    /// 
    /// Set or return a data object type of LocalLicense or LicenseServer.
    pub async fn source(&self) -> Result<Box<dyn crate::types::traits::LicenseSourceTrait>> {
        let path = format!("/LicenseManager/{moId}/source", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: Box<dyn crate::types::traits::LicenseSourceTrait> = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of vSphere API 4.0, this property is not used.
    /// 
    /// Current state of the license source.
    /// 
    /// License sources that are LocalSource
    /// are always available.
    pub async fn source_available(&self) -> Result<bool> {
        let path = format!("/LicenseManager/{moId}/sourceAvailable", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: bool = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct AddLicenseRequestType<'a> {
    license_key: &'a str,
    labels: Option<&'a [crate::types::structs::KeyValue]>,
}

impl<'a> miniserde::Serialize for AddLicenseRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AddLicenseRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AddLicenseRequestTypeSer<'b, 'a> {
    data: &'b AddLicenseRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AddLicenseRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AddLicenseRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("licenseKey"), &self.data.license_key as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.labels else { continue; };
                    return Some((std::borrow::Cow::Borrowed("labels"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct CheckLicenseFeatureRequestType<'a> {
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
    feature_key: &'a str,
}

impl<'a> miniserde::Serialize for CheckLicenseFeatureRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CheckLicenseFeatureRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CheckLicenseFeatureRequestTypeSer<'b, 'a> {
    data: &'b CheckLicenseFeatureRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CheckLicenseFeatureRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CheckLicenseFeatureRequestType")),
                1 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("featureKey"), &self.data.feature_key as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct ConfigureLicenseSourceRequestType<'a> {
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
    license_source: &'a dyn crate::types::traits::LicenseSourceTrait,
}

impl<'a> miniserde::Serialize for ConfigureLicenseSourceRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ConfigureLicenseSourceRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ConfigureLicenseSourceRequestTypeSer<'b, 'a> {
    data: &'b ConfigureLicenseSourceRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ConfigureLicenseSourceRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ConfigureLicenseSourceRequestType")),
                1 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("licenseSource"), &self.data.license_source as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct DecodeLicenseRequestType<'a> {
    license_key: &'a str,
}

impl<'a> miniserde::Serialize for DecodeLicenseRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DecodeLicenseRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DecodeLicenseRequestTypeSer<'b, 'a> {
    data: &'b DecodeLicenseRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DecodeLicenseRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DecodeLicenseRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("licenseKey"), &self.data.license_key as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DisableFeatureRequestType<'a> {
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
    feature_key: &'a str,
}

impl<'a> miniserde::Serialize for DisableFeatureRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DisableFeatureRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DisableFeatureRequestTypeSer<'b, 'a> {
    data: &'b DisableFeatureRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DisableFeatureRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DisableFeatureRequestType")),
                1 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("featureKey"), &self.data.feature_key as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct EnableFeatureRequestType<'a> {
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
    feature_key: &'a str,
}

impl<'a> miniserde::Serialize for EnableFeatureRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(EnableFeatureRequestTypeSer { data: self, seq: 0 }))
    }
}

struct EnableFeatureRequestTypeSer<'b, 'a> {
    data: &'b EnableFeatureRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for EnableFeatureRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"EnableFeatureRequestType")),
                1 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                2 => return Some((std::borrow::Cow::Borrowed("featureKey"), &self.data.feature_key as &dyn miniserde::Serialize)),
                _ => return None,
            }
        }
    }
}
struct QueryLicenseSourceAvailabilityRequestType<'a> {
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for QueryLicenseSourceAvailabilityRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryLicenseSourceAvailabilityRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryLicenseSourceAvailabilityRequestTypeSer<'b, 'a> {
    data: &'b QueryLicenseSourceAvailabilityRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryLicenseSourceAvailabilityRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryLicenseSourceAvailabilityRequestType")),
                1 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QuerySupportedFeaturesRequestType<'a> {
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for QuerySupportedFeaturesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QuerySupportedFeaturesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QuerySupportedFeaturesRequestTypeSer<'b, 'a> {
    data: &'b QuerySupportedFeaturesRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QuerySupportedFeaturesRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QuerySupportedFeaturesRequestType")),
                1 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct QueryLicenseUsageRequestType<'a> {
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
}

impl<'a> miniserde::Serialize for QueryLicenseUsageRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(QueryLicenseUsageRequestTypeSer { data: self, seq: 0 }))
    }
}

struct QueryLicenseUsageRequestTypeSer<'b, 'a> {
    data: &'b QueryLicenseUsageRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for QueryLicenseUsageRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"QueryLicenseUsageRequestType")),
                1 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct RemoveLicenseLabelRequestType<'a> {
    license_key: &'a str,
    label_key: &'a str,
}

impl<'a> miniserde::Serialize for RemoveLicenseLabelRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveLicenseLabelRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveLicenseLabelRequestTypeSer<'b, 'a> {
    data: &'b RemoveLicenseLabelRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveLicenseLabelRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveLicenseLabelRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("licenseKey"), &self.data.license_key as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("labelKey"), &self.data.label_key as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RemoveLicenseRequestType<'a> {
    license_key: &'a str,
}

impl<'a> miniserde::Serialize for RemoveLicenseRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveLicenseRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveLicenseRequestTypeSer<'b, 'a> {
    data: &'b RemoveLicenseRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveLicenseRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveLicenseRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("licenseKey"), &self.data.license_key as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct SetLicenseEditionRequestType<'a> {
    host: Option<&'a crate::types::structs::ManagedObjectReference>,
    feature_key: Option<&'a str>,
}

impl<'a> miniserde::Serialize for SetLicenseEditionRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetLicenseEditionRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetLicenseEditionRequestTypeSer<'b, 'a> {
    data: &'b SetLicenseEditionRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for SetLicenseEditionRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SetLicenseEditionRequestType")),
                1 => {
                    let Some(ref val) = self.data.host else { continue; };
                    return Some((std::borrow::Cow::Borrowed("host"), val as &dyn miniserde::Serialize));
                }
                2 => {
                    let Some(ref val) = self.data.feature_key else { continue; };
                    return Some((std::borrow::Cow::Borrowed("featureKey"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct UpdateLicenseLabelRequestType<'a> {
    license_key: &'a str,
    label_key: &'a str,
    label_value: &'a str,
}

impl<'a> miniserde::Serialize for UpdateLicenseLabelRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateLicenseLabelRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateLicenseLabelRequestTypeSer<'b, 'a> {
    data: &'b UpdateLicenseLabelRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateLicenseLabelRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateLicenseLabelRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("licenseKey"), &self.data.license_key as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("labelKey"), &self.data.label_key as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("labelValue"), &self.data.label_value as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateLicenseRequestType<'a> {
    license_key: &'a str,
    labels: Option<&'a [crate::types::structs::KeyValue]>,
}

impl<'a> miniserde::Serialize for UpdateLicenseRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateLicenseRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateLicenseRequestTypeSer<'b, 'a> {
    data: &'b UpdateLicenseRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateLicenseRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateLicenseRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("licenseKey"), &self.data.license_key as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.labels else { continue; };
                    return Some((std::borrow::Cow::Borrowed("labels"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
