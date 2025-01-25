use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::HostDateTimeConfig;
use crate::types::structs::HostDateTimeInfo;
use crate::types::structs::HostDateTimeSystemServiceTestResult;
use crate::types::structs::HostDateTimeSystemTimeZone;
/// This managed object provides for NTP and date/time related
/// configuration on a host.
/// 
/// Information regarding the running status of the NTP daemon and
/// functionality to start and stop the daemon is provided by the
/// *HostServiceSystem* object.
pub struct HostDateTimeSystem {
    client: Arc<Client>,
    mo_id: String,
}
impl HostDateTimeSystem {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
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
    pub async fn query_available_time_zones(&self) -> Result<Option<Vec<HostDateTimeSystemTimeZone>>> {
        let path = format!("/HostDateTimeSystem/{moId}/QueryAvailableTimeZones", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
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
        Ok(self.client.execute(req).await?)
    }
    /// Refresh the DateTime related settings to pick up any changes that might
    /// have occurred.
    /// 
    /// ***Required privileges:*** Host.Config.DateTime
    pub async fn refresh_date_time_system(&self) -> Result<()> {
        let path = format!("/HostDateTimeSystem/{moId}/RefreshDateTimeSystem", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
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
    pub async fn test_time_service(&self) -> Result<Option<HostDateTimeSystemServiceTestResult>> {
        let path = format!("/HostDateTimeSystem/{moId}/TestTimeService", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_option(req).await?)
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
    pub async fn update_date_time_config(&self, config: &HostDateTimeConfig) -> Result<()> {
        let input = UpdateDateTimeConfigRequestType {config, };
        let path = format!("/HostDateTimeSystem/{moId}/UpdateDateTimeConfig", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// The DateTime configuration of the host.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Returns:
    ///
    /// DateTime configuration of the host.
    pub async fn date_time_info(&self) -> Result<HostDateTimeInfo> {
        let path = format!("/HostDateTimeSystem/{moId}/dateTimeInfo", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateDateTimeConfigRequestType<'a> {
    config: &'a HostDateTimeConfig,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateDateTimeRequestType<'a> {
    #[serde(rename = "dateTime")]
    date_time: &'a str,
}
