use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::Event;
use crate::types::vim_any::VimAny;
/// EventHistoryCollector provides a mechanism for
/// retrieving historical data and updates when the server appends new
/// events.
pub struct EventHistoryCollector {
    client: Arc<Client>,
    mo_id: String,
}
impl EventHistoryCollector {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Reads the 'scrollable view' from the current position.
    /// 
    /// The scrollable
    /// position is moved to the next newer page after the read. No item is
    /// returned when the end of the collector is reached.
    ///
    /// ## Parameters:
    ///
    /// ### max_count
    /// The maximum number of items in the page.
    pub async fn read_next_events(&self, max_count: i32) -> Result<Option<Vec<Event>>> {
        let input = ReadNextEventsRequestType {max_count, };
        let path = format!("/EventHistoryCollector/{moId}/ReadNextEvents", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Reads the 'scrollable view' from the current position.
    /// 
    /// The scrollable
    /// position is moved to the next older page after the read. No item is
    /// returned when the head of the collector is reached.
    ///
    /// ## Parameters:
    ///
    /// ### max_count
    /// The maximum number of items in the page.
    pub async fn read_previous_events(&self, max_count: i32) -> Result<Option<Vec<Event>>> {
        let input = ReadPreviousEventsRequestType {max_count, };
        let path = format!("/EventHistoryCollector/{moId}/ReadPreviousEvents", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Destroys this collector.
    pub async fn destroy_collector(&self) -> Result<()> {
        let path = format!("/EventHistoryCollector/{moId}/DestroyCollector", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
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
        let path = format!("/EventHistoryCollector/{moId}/ResetCollector", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Moves the "scrollable view" to the oldest item.
    /// 
    /// If you use
    /// *ReadNextTasks* or
    /// *ReadNextEvents*,
    /// all items are retrieved from the oldest item to the newest item. This
    /// is the default setting when the collector is created.
    pub async fn rewind_collector(&self) -> Result<()> {
        let path = format!("/EventHistoryCollector/{moId}/RewindCollector", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
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
        let path = format!("/EventHistoryCollector/{moId}/SetCollectorPageSize", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// The filter used to create this collector.
    /// 
    /// The type of the returned filter is determined by the managed object
    /// for which the collector is created.
    pub async fn filter(&self) -> Result<VimAny> {
        let path = format!("/EventHistoryCollector/{moId}/filter", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// The items in the 'viewable latest page'.
    /// 
    /// As new events that match the
    /// collector's *EventFilterSpec* are created, they are added to this
    /// page, and the oldest events are removed from the collector to keep the
    /// size of the page to that allowed by
    /// *HistoryCollector.SetCollectorPageSize*.
    /// 
    /// The "oldest event" is the one with the smallest key (event ID). The
    /// events in the returned page are unordered.
    /// While `initialized` is `false` this property will remain empty and once the Collector is initialized it will be populated.
    /// While `initialized` is `true` this property is populated immediately.
    pub async fn latest_page(&self) -> Result<Option<Vec<Event>>> {
        let path = format!("/EventHistoryCollector/{moId}/latestPage", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReadNextEventsRequestType {
    #[serde(rename = "maxCount")]
    max_count: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReadPreviousEventsRequestType {
    #[serde(rename = "maxCount")]
    max_count: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetCollectorPageSizeRequestType {
    #[serde(rename = "maxCount")]
    max_count: i32,
}
