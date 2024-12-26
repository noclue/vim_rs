use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::DatabaseSizeEstimate;
use crate::types::structs::DatabaseSizeParam;
pub struct ResourcePlanningManager {
    client: Arc<Client>,
    mo_id: String,
}
impl ResourcePlanningManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Estimates the database size required to store VirtualCenter data.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### db_size_param
    /// *DatabaseSizeParam*
    /// Contains the summary of an inventory for which the database size
    /// requirements are to be computed. It also contains
    /// the historic interval setting for which the database
    /// computations are to be done. This is an optional parameter and
    /// the current virtual center historical settings are used by default.
    /// There are many other optional fields in the dbSizeParam structure
    /// that are appropriately filled up based on some heuristics.
    ///
    /// ## Returns:
    ///
    /// *DatabaseSizeEstimate*
    /// Returns the size required in MB of the database and the number of database
    /// rows.
    pub async fn estimate_database_size(&self, db_size_param: &DatabaseSizeParam) -> Result<DatabaseSizeEstimate> {
        let input = EstimateDatabaseSizeRequestType {db_size_param, };
        let path = format!("/ResourcePlanningManager/{moId}/EstimateDatabaseSize", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct EstimateDatabaseSizeRequestType<'a> {
    #[serde(rename = "dbSizeParam")]
    db_size_param: &'a DatabaseSizeParam,
}
