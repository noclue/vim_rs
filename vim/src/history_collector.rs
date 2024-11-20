use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::VimAny;
/// This managed object type enables clients to retrieve historical data and
/// receive updates when the server appends new data to a collection.
/// 
/// This is a base type for item-specific types related to event or task history.
/// Historical data is inherently append-only,
/// although a server administrator may periodically purge old data.
/// 
/// Typically, a client creates a history collector by using a filter on a
/// potentially large set, such as all the events in a datacenter.
/// The collector provides access to the items that match the filter,
/// which could also be a relatively large set.
/// 
/// The items in a collector are always ordered by date and time of creation.
/// Item properties normally include this time stamp.
/// 
/// The client may set the "viewable latest page" for the collector,
/// which is the contiguous subset of the items that are of
/// immediate interest. These items are available as the "latestPage"
/// property, which the client may retrieve and monitor by using the
/// *PropertyCollector* managed object.
/// 
/// Clients can change the page size of the "latestPage" by using
/// *setLatestPageSize()*.
/// 
/// The client may use the following features of the history collector.
/// - *rewind* - Moves the "scrollable view" to
///   the oldest item (the default setting).
/// - readNext - Retrieves all the items in the collector, from the oldest
///   item to the newest item. Retrieves either
///   *tasks* or
///   *events* depending on the operation.
/// - readPrev - Retrieves all items (excluding the "viewable latest page") in
///   the collector, from the newest item to the oldest item. Retrieves either
///   *tasks* or
///   *events* depending on the operation.
/// - *reset* - Moves the "scrollable view" to
///   the item immediately preceding the "viewable latest page".
pub struct HistoryCollector {
    client: Arc<VimClient>,
    mo_id: String,
}
impl HistoryCollector {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Destroys this collector.
    pub async fn destroy_collector(&self) -> Result<()> {
        let path = format!("/HistoryCollector/{moId}/DestroyCollector", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Moves the "scrollable view" to the item immediately preceding the
    /// "viewable latest page".
    /// 
    /// If you use "readPrev",
    /// *ReadPreviousTasks* or
    /// *ReadPreviousEvents*,
    /// all items
    /// are retrieved from the newest item to the oldest item.
    pub async fn reset_collector(&self) -> Result<()> {
        let path = format!("/HistoryCollector/{moId}/ResetCollector", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Moves the "scrollable view" to the oldest item.
    /// 
    /// If you use
    /// *ReadNextTasks* or
    /// *ReadNextEvents*,
    /// all items are retrieved from the oldest item to the newest item. This
    /// is the default setting when the collector is created.
    pub async fn rewind_collector(&self) -> Result<()> {
        let path = format!("/HistoryCollector/{moId}/RewindCollector", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Sets the "viewable latest page" size to contain at most the
    /// number of items specified by the maxCount parameter).
    ///
    /// ## Parameters:
    ///
    /// ### max_count
    /// The maximum number of items in the page.
    pub async fn set_collector_page_size(&self, max_count: i32) -> Result<()> {
        let input = SetCollectorPageSizeRequestType {max_count, };
        let path = format!("/HistoryCollector/{moId}/SetCollectorPageSize", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// The filter used to create this collector.
    /// 
    /// The type of the returned filter is determined by the managed object
    /// for which the collector is created.
    pub async fn filter(&self) -> Result<VimAny> {
        let path = format!("/HistoryCollector/{moId}/filter", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetCollectorPageSizeRequestType {
    #[serde(rename = "maxCount")]
    max_count: i32,
}
