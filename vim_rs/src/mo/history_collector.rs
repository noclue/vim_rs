use std::sync::Arc;
use crate::core::client::{VimClient, Result};
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
#[derive(Clone)]
pub struct HistoryCollector {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HistoryCollector {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Destroys this collector.
    pub async fn destroy_collector(&self) -> Result<()> {
        self.client.invoke_void("", "HistoryCollector", &self.mo_id, "DestroyCollector", None).await
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
        self.client.invoke_void("", "HistoryCollector", &self.mo_id, "ResetCollector", None).await
    }
    /// Moves the "scrollable view" to the oldest item.
    /// 
    /// If you use
    /// *ReadNextTasks* or
    /// *ReadNextEvents*,
    /// all items are retrieved from the oldest item to the newest item. This
    /// is the default setting when the collector is created.
    pub async fn rewind_collector(&self) -> Result<()> {
        self.client.invoke_void("", "HistoryCollector", &self.mo_id, "RewindCollector", None).await
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
        self.client.invoke_void("", "HistoryCollector", &self.mo_id, "SetCollectorPageSize", Some(&input)).await
    }
    /// The filter used to create this collector.
    /// 
    /// The type of the returned filter is determined by the managed object
    /// for which the collector is created.
    pub async fn filter(&self) -> Result<crate::types::vim_any::VimAny> {
        let bytes_opt = self.client.fetch_property_raw("", "HistoryCollector", &self.mo_id, "filter").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property filter was empty".to_string()))?;
        let result: crate::types::vim_any::VimAny = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
}
struct SetCollectorPageSizeRequestType {
    max_count: i32,
}

impl miniserde::Serialize for SetCollectorPageSizeRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetCollectorPageSizeRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetCollectorPageSizeRequestTypeSer<'b> {
    data: &'b SetCollectorPageSizeRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for SetCollectorPageSizeRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SetCollectorPageSizeRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("maxCount"), &self.data.max_count as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
