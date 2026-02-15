use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// EventHistoryCollector provides a mechanism for
/// retrieving historical data and updates when the server appends new
/// events.
#[derive(Clone)]
pub struct EventHistoryCollector {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl EventHistoryCollector {
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
    pub async fn read_next_events(&self, max_count: i32) -> Result<Option<Vec<crate::types::structs::Event>>> {
        let input = ReadNextEventsRequestType {max_count, };
        let path = format!("/EventHistoryCollector/{moId}/ReadNextEvents", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::Event>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
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
    pub async fn read_previous_events(&self, max_count: i32) -> Result<Option<Vec<crate::types::structs::Event>>> {
        let input = ReadPreviousEventsRequestType {max_count, };
        let path = format!("/EventHistoryCollector/{moId}/ReadPreviousEvents", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::Event>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
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
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// The filter used to create this collector.
    /// 
    /// The type of the returned filter is determined by the managed object
    /// for which the collector is created.
    pub async fn filter(&self) -> Result<crate::types::vim_any::VimAny> {
        let path = format!("/EventHistoryCollector/{moId}/filter", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::vim_any::VimAny = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Indicates whether the *EventHistoryCollector.latestPage* is initialized.
    /// 
    /// If the
    /// *EventFilterSpec.delayedInit* is `false` or unset this property is always true.
    /// 
    /// ***Since:*** vSphere API Release 8.0.3.0
    pub async fn initialized(&self) -> Result<Option<bool>> {
        let path = format!("/EventHistoryCollector/{moId}/initialized", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<bool>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
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
    pub async fn latest_page(&self) -> Result<Option<Vec<crate::types::structs::Event>>> {
        let path = format!("/EventHistoryCollector/{moId}/latestPage", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::Event>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
}
struct ReadNextEventsRequestType {
    max_count: i32,
}

impl miniserde::Serialize for ReadNextEventsRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReadNextEventsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReadNextEventsRequestTypeSer<'b> {
    data: &'b ReadNextEventsRequestType,
    seq: usize,
}

impl miniserde::ser::Map for ReadNextEventsRequestTypeSer<'_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReadNextEventsRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("maxCount"), &self.data.max_count as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ReadPreviousEventsRequestType {
    max_count: i32,
}

impl miniserde::Serialize for ReadPreviousEventsRequestType {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ReadPreviousEventsRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ReadPreviousEventsRequestTypeSer<'b> {
    data: &'b ReadPreviousEventsRequestType,
    seq: usize,
}

impl miniserde::ser::Map for ReadPreviousEventsRequestTypeSer<'_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ReadPreviousEventsRequestType")),
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

impl miniserde::ser::Map for SetCollectorPageSizeRequestTypeSer<'_> {
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
