use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::TaskInfo;
use crate::types::structs::VimAny;
/// TaskHistoryCollector provides a mechanism for
/// retrieving historical data and updates when the server appends new
/// tasks.
pub struct TaskHistoryCollector {
    client: Arc<Client>,
    mo_id: String,
}
impl TaskHistoryCollector {
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
    pub async fn read_next_tasks(&self, max_count: i32) -> Result<Option<Vec<TaskInfo>>> {
        let input = ReadNextTasksRequestType {max_count, };
        let path = format!("/TaskHistoryCollector/{moId}/ReadNextTasks", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Reads the 'scrollable view' from the current position.
    /// 
    /// The scrollable
    /// position is then moved to the next older page after the read. No item is
    /// returned when the head of the collector is reached.
    ///
    /// ## Parameters:
    ///
    /// ### max_count
    /// The maximum number of items in the page.
    pub async fn read_previous_tasks(&self, max_count: i32) -> Result<Option<Vec<TaskInfo>>> {
        let input = ReadPreviousTasksRequestType {max_count, };
        let path = format!("/TaskHistoryCollector/{moId}/ReadPreviousTasks", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Destroys this collector.
    pub async fn destroy_collector(&self) -> Result<()> {
        let path = format!("/TaskHistoryCollector/{moId}/DestroyCollector", moId = &self.mo_id);
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
        let path = format!("/TaskHistoryCollector/{moId}/ResetCollector", moId = &self.mo_id);
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
        let path = format!("/TaskHistoryCollector/{moId}/RewindCollector", moId = &self.mo_id);
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
        let path = format!("/TaskHistoryCollector/{moId}/SetCollectorPageSize", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// The filter used to create this collector.
    /// 
    /// The type of the returned filter is determined by the managed object
    /// for which the collector is created.
    pub async fn filter(&self) -> Result<VimAny> {
        let path = format!("/TaskHistoryCollector/{moId}/filter", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// The items in the 'viewable latest page'.
    /// 
    /// As new tasks that match the
    /// collector's *TaskFilterSpec* are created, they are added to this
    /// page, and the oldest tasks are removed from the collector to keep the
    /// size of the page to that allowed by
    /// *HistoryCollector.SetCollectorPageSize*.
    /// 
    /// The "oldest task" is the one with the oldest creation time. The
    /// tasks in the returned page are unordered.
    pub async fn latest_page(&self) -> Result<Option<Vec<TaskInfo>>> {
        let path = format!("/TaskHistoryCollector/{moId}/latestPage", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReadNextTasksRequestType {
    #[serde(rename = "maxCount")]
    max_count: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ReadPreviousTasksRequestType {
    #[serde(rename = "maxCount")]
    max_count: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetCollectorPageSizeRequestType {
    #[serde(rename = "maxCount")]
    max_count: i32,
}
