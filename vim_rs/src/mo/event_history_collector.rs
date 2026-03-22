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
        let bytes_opt = self.client.invoke_optional("", "EventHistoryCollector", &self.mo_id, "ReadNextEvents", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
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
        let bytes_opt = self.client.invoke_optional("", "EventHistoryCollector", &self.mo_id, "ReadPreviousEvents", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Destroys this collector.
    pub async fn destroy_collector(&self) -> Result<()> {
        self.client.invoke_void("", "EventHistoryCollector", &self.mo_id, "DestroyCollector", None).await
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
        self.client.invoke_void("", "EventHistoryCollector", &self.mo_id, "ResetCollector", None).await
    }
    /// Moves the "scrollable view" to the oldest item.
    /// 
    /// If you use
    /// *ReadNextTasks* or
    /// *ReadNextEvents*,
    /// all items are retrieved from the oldest item to the newest item. This
    /// is the default setting when the collector is created.
    pub async fn rewind_collector(&self) -> Result<()> {
        self.client.invoke_void("", "EventHistoryCollector", &self.mo_id, "RewindCollector", None).await
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
        self.client.invoke_void("", "EventHistoryCollector", &self.mo_id, "SetCollectorPageSize", Some(&input)).await
    }
    /// The filter used to create this collector.
    /// 
    /// The type of the returned filter is determined by the managed object
    /// for which the collector is created.
    pub async fn filter(&self) -> Result<crate::types::vim_any::VimAny> {
        let pv_opt = self.client.fetch_property_raw("", "EventHistoryCollector", &self.mo_id, "filter").await?;
        let pv = pv_opt.ok_or_else(|| crate::core::client::VimError::ParseError("property filter was empty".to_string()))?;
        let result: crate::types::vim_any::VimAny = crate::core::client::extract_property(pv)?;
        Ok(result)
    }
    /// Indicates whether the *EventHistoryCollector.latestPage* is initialized.
    /// 
    /// If the
    /// *EventFilterSpec.delayedInit* is `false` or unset this property is always true.
    /// 
    /// ***Since:*** vSphere API Release 8.0.3.0
    pub async fn initialized(&self) -> Result<Option<bool>> {
        let pv_opt = self.client.fetch_property_raw("", "EventHistoryCollector", &self.mo_id, "initialized").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
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
        let pv_opt = self.client.fetch_property_raw("", "EventHistoryCollector", &self.mo_id, "latestPage").await?;
        match pv_opt {
            Some(pv) => Ok(Some(crate::core::client::extract_property(pv)?)),
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

impl<'b> miniserde::ser::Map for ReadNextEventsRequestTypeSer<'b> {
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

impl<'b> miniserde::ser::Map for ReadPreviousEventsRequestTypeSer<'b> {
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
