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
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::HostDateTimeSystemTimeZone>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: String = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
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
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::HostDateTimeSystemServiceTestResult>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
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
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::HostDateTimeInfo = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
}
struct UpdateDateTimeConfigRequestType<'a> {
    config: &'a crate::types::structs::HostDateTimeConfig,
}

impl<'a> miniserde::Serialize for UpdateDateTimeConfigRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateDateTimeConfigRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateDateTimeConfigRequestTypeSer<'b, 'a> {
    data: &'b UpdateDateTimeConfigRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateDateTimeConfigRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateDateTimeConfigRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("config"), &self.data.config as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateDateTimeRequestType<'a> {
    date_time: &'a str,
}

impl<'a> miniserde::Serialize for UpdateDateTimeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateDateTimeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateDateTimeRequestTypeSer<'b, 'a> {
    data: &'b UpdateDateTimeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateDateTimeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateDateTimeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("dateTime"), &self.data.date_time as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
