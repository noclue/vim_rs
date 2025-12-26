use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object provides for NTP and date/time related
/// configuration on a host.
/// 
/// Information regarding the running status of the NTP daemon and
/// functionality to start and stop the daemon is provided by the
/// *HostServiceSystem* object.
#[derive(Clone)]
pub struct HostDateTimeSystem {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostDateTimeSystem {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Retrieves the list of available timezones on the host.
    /// 
    /// The API works off the public domain 'tz' timezone database.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// List of available timezones on the host.
    pub async fn query_available_time_zones(&self) -> Result<Option<Vec<crate::types::structs::HostDateTimeSystemTimeZone>>> {
        let path = format!("/HostDateTimeSystem/{moId}/QueryAvailableTimeZones", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<Vec<crate::types::structs::HostDateTimeSystemTimeZone>>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Get the current DateTime on the host.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// Current DateTime on the host.
    pub async fn query_date_time(&self) -> Result<String> {
        let path = format!("/HostDateTimeSystem/{moId}/QueryDateTime", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: String = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
    /// Refresh the DateTime related settings to pick up any changes that might
    /// have occurred.
    /// 
    /// ***Required privileges:*** Host.Config.DateTime
    pub async fn refresh_date_time_system(&self) -> Result<()> {
        let path = format!("/HostDateTimeSystem/{moId}/RefreshDateTimeSystem", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Run a test to validate current time service configuration is functioning
    /// normally.
    /// 
    /// The report will provide a localized diagnostic of any issues.
    /// Only one diagnostic test may be running at a time.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    /// 
    /// ***Required privileges:*** Host.Config.DateTime
    ///
    /// ## Returns:
    ///
    /// The status of the time service on this host based on present time
    /// service configuration.
    pub async fn test_time_service(&self) -> Result<Option<crate::types::structs::HostDateTimeSystemServiceTestResult>> {
        let path = format!("/HostDateTimeSystem/{moId}/TestTimeService", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => Ok(Some(serde_json::from_slice::<crate::types::structs::HostDateTimeSystemServiceTestResult>(bytes.as_ref())?)),
            None => Ok(None),
        }
    }
    /// Update the DateTime configuration of the host.
    /// 
    /// ***Required privileges:*** Host.Config.DateTime
    ///
    /// ## Parameters:
    ///
    /// ### config
    /// The new DateTime configuration information.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if an error occurs.
    pub async fn update_date_time_config(&self, config: &crate::types::structs::HostDateTimeConfig) -> Result<()> {
        let input = UpdateDateTimeConfigRequestType {config, };
        let path = format!("/HostDateTimeSystem/{moId}/UpdateDateTimeConfig", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Update the date/time on the host.
    /// 
    /// This method should be used with caution since network delays, execution
    /// delays can result in time skews.
    /// 
    /// ***Required privileges:*** Host.Config.DateTime
    ///
    /// ## Parameters:
    ///
    /// ### date_time
    /// DateTime to update the host to.
    ///
    /// ## Errors:
    ///
    /// ***HostConfigFault***: if an error occurs.
    pub async fn update_date_time(&self, date_time: &str) -> Result<()> {
        let input = UpdateDateTimeRequestType {date_time, };
        let path = format!("/HostDateTimeSystem/{moId}/UpdateDateTime", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// The DateTime configuration of the host.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// DateTime configuration of the host.
    pub async fn date_time_info(&self) -> Result<crate::types::structs::HostDateTimeInfo> {
        let path = format!("/HostDateTimeSystem/{moId}/dateTimeInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let result: crate::types::structs::HostDateTimeInfo = serde_json::from_slice(bytes.as_ref())?;
        Ok(result)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateDateTimeConfigRequestType<'a> {
    config: &'a crate::types::structs::HostDateTimeConfig,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateDateTimeRequestType<'a> {
    #[serde(rename = "dateTime")]
    date_time: &'a str,
}
