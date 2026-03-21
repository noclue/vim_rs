use std::sync::Arc;
use crate::core::client::{VimClient, Result};
#[derive(Clone)]
pub struct ResourcePlanningManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl ResourcePlanningManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn estimate_database_size(&self, db_size_param: &crate::types::structs::DatabaseSizeParam) -> Result<crate::types::structs::DatabaseSizeEstimate> {
        let input = EstimateDatabaseSizeRequestType {db_size_param, };
        let bytes = self.client.invoke("", "ResourcePlanningManager", &self.mo_id, "EstimateDatabaseSize", Some(&input)).await?;
        let result: crate::types::structs::DatabaseSizeEstimate = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct EstimateDatabaseSizeRequestType<'a> {
    db_size_param: &'a crate::types::structs::DatabaseSizeParam,
}

impl<'a> miniserde::Serialize for EstimateDatabaseSizeRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(EstimateDatabaseSizeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct EstimateDatabaseSizeRequestTypeSer<'b, 'a> {
    data: &'b EstimateDatabaseSizeRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for EstimateDatabaseSizeRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"EstimateDatabaseSizeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("dbSizeParam"), &self.data.db_size_param as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
