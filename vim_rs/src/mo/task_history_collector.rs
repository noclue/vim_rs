use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// TaskHistoryCollector provides a mechanism for
/// retrieving historical data and updates when the server appends new
/// tasks.
#[derive(Clone)]
pub struct TaskHistoryCollector {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl TaskHistoryCollector {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
    pub async fn read_next_tasks(&self, max_count: i32) -> Result<Option<Vec<crate::types::structs::TaskInfo>>> {
        let input = ReadNextTasksRequestType {max_count, };
        let bytes_opt = self.client.invoke_optional("", "TaskHistoryCollector", &self.mo_id, "ReadNextTasks", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
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
    pub async fn read_previous_tasks(&self, max_count: i32) -> Result<Option<Vec<crate::types::structs::TaskInfo>>> {
        let input = ReadPreviousTasksRequestType {max_count, };
        let bytes_opt = self.client.invoke_optional("", "TaskHistoryCollector", &self.mo_id, "ReadPreviousTasks", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Destroys this collector.
    pub async fn destroy_collector(&self) -> Result<()> {
        self.client.invoke_void("", "TaskHistoryCollector", &self.mo_id, "DestroyCollector", None).await
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
        self.client.invoke_void("", "TaskHistoryCollector", &self.mo_id, "ResetCollector", None).await
    }
    /// Moves the "scrollable view" to the oldest item.
    /// 
    /// If you use
    /// *ReadNextTasks* or
    /// *ReadNextEvents*,
    /// all items are retrieved from the oldest item to the newest item. This
    /// is the default setting when the collector is created.
    pub async fn rewind_collector(&self) -> Result<()> {
        self.client.invoke_void("", "TaskHistoryCollector", &self.mo_id, "RewindCollector", None).await
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
        self.client.invoke_void("", "TaskHistoryCollector", &self.mo_id, "SetCollectorPageSize", Some(&input)).await
    }
    /// The filter used to create this collector.
    /// 
    /// The type of the returned filter is determined by the managed object
    /// for which the collector is created.
    pub async fn filter(&self) -> Result<crate::types::vim_any::VimAny> {
        let bytes_opt = self.client.fetch_property_raw("", "TaskHistoryCollector", &self.mo_id, "filter").await?;
        let bytes = bytes_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property filter was empty".to_string()))?;
        let result: crate::types::vim_any::VimAny = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
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
    pub async fn latest_page(&self) -> Result<Option<Vec<crate::types::structs::TaskInfo>>> {
        let bytes_opt = self.client.fetch_property_raw("", "TaskHistoryCollector", &self.mo_id, "latestPage").await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
}
struct ReadNextTasksRequestType {
    max_count: i32,
}

impl miniserde::Serialize for ReadNextTasksRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReadNextTasksRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReadNextTasksRequestTypeSer<'b> {
    data: &'b ReadNextTasksRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for ReadNextTasksRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReadNextTasksRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("maxCount"), &self.data.max_count as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ReadPreviousTasksRequestType {
    max_count: i32,
}

impl miniserde::Serialize for ReadPreviousTasksRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReadPreviousTasksRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReadPreviousTasksRequestTypeSer<'b> {
    data: &'b ReadPreviousTasksRequestType,
    seq: usize,
}

impl<'b> miniserde::ser::Map for ReadPreviousTasksRequestTypeSer<'b> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReadPreviousTasksRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("maxCount"), &self.data.max_count as &dyn miniserde::Serialize)),
            _ => return None,
        }
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
