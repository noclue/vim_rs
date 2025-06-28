//! # vSphere Event Monitoring Example
//!
//! This example demonstrates how to retrieve and process events from a vSphere environment
//! using the EventManager interface.
//!
//! The vSphere event system provides a comprehensive record of actions and state changes
//! that occur within the virtual infrastructure. Monitoring these events programmatically
//! allows for auditing, alerting, and tracking system changes over time.
//!
//! In this example:
//! 1. We connect to a vSphere server using credentials from environment variables
//! 2. We access the EventManager service from the ServiceContent
//! 3. We create an event filter to retrieve events from the last 30 minutes
//! 4. We create an EventHistoryCollector to efficiently retrieve batches of events
//! 5. We process each event, extracting its type ID, timestamp, and message
//!
//! The example includes a utility function to determine the event type ID based on
//! different event class structures, demonstrating how to handle the polymorphic
//! nature of vSphere events.

use anyhow::{Error, Result};
use chrono::{Duration as ChronoDuration, Utc};
use log::info;
use std::{env, sync::Arc};
use tokio::time::sleep;
use utils::connect;
use vim_rs::core::client::Client;
use vim_rs::mo::EventManager;
use vim_rs::types::struct_enum::StructType;
use vim_rs::types::structs::{Event, EventFilterSpec, EventFilterSpecByTime};

/// Get the event type ID from an event
/// The semantics of how eventTypeId matching is done is as follows:
/// - If the event is of type EventEx return event_type_id member of the EventEx
/// - If the event is of type ExtendedEvent return event_type_id member of the ExtendedEvent
/// - Otherwise, return the type name of the Event itself.
fn get_event_type_id(event: &Event) -> String {
    let Some(type_) = event.type_ else {
        return "Event".to_string();
    };
    if type_.child_of(StructType::EventEx) || type_.child_of(StructType::ExtendedEvent) {
        if let Some(event_type_id) = event.extra_fields_["eventTypeId"].as_str() {
            return event_type_id.to_string();
        }
    }
    let s: &'static str = type_.into();
    s.to_string()
}

// Dump the last 30 minutes of events in vCenter
async fn dump_events(client: Arc<Client>, event_manager: &EventManager) -> Result<()> {
    let thirty_minutes_ago = Utc::now() - ChronoDuration::minutes(30);

    let filter = &EventFilterSpec {
        entity: None,
        time: Some(EventFilterSpecByTime {
            begin_time: Some(thirty_minutes_ago.to_rfc3339()),
            end_time: None,
        }),
        user_name: None,
        event_chain_id: None,
        alarm: None,
        scheduled_task: None,
        disable_full_message: Some(false),
        category: None,
        r#type: None,
        tag: None,
        event_type_id: None,
        max_count: None,
        delayed_init: None,
    };

    let collector = event_manager.create_collector_for_events(filter).await?;

    let collector = vim_rs::mo::EventHistoryCollector::new(client.clone(), &collector.value);
    for _ in 0..5 {
        let events = collector.read_next_events(50).await?;
        match events {
            Some(events) => {
                for event in &events {
                    info!(
                        "{event_type}: {ts} - {id} - {msg}",
                        event_type = get_event_type_id(event),
                        id = event.key,
                        ts = event.created_time,
                        msg = event
                            .full_formatted_message
                            .as_deref()
                            .unwrap_or("No message")
                    );
                }
                if !events.is_empty() {
                    continue; // dump events with no delay
                }
            }
            None => {
                info!("No events found")
            }
        }
        sleep(std::time::Duration::from_secs(5)).await;
    }
    collector.destroy_collector().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;

    let Some(event_manager_moref) = client.service_content().event_manager.clone() else {
        return Err(Error::msg("No event manager found"));
    };
    let event_manager = EventManager::new(client.clone(), &event_manager_moref.value);

    dump_events(client.clone(), &event_manager).await?;

    Ok(())
}
