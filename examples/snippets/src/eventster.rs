use std::{env, sync::Arc};
use tokio::time::sleep;
use vim_rs::mo::EventManager;
use vim_rs::types::structs::{EventFilterSpecByTime, EventFilterSpec, Event};
use vim_rs::core::client::{Client, ClientBuilder};
use log::info;
use chrono::{Utc, Duration as ChronoDuration};
use anyhow::{Result, Error, Context};
use vim_rs::types::struct_enum::StructType;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

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
    };

    let collector = event_manager.create_collector_for_events(filter).await?;

    let collector = vim_rs::mo::EventHistoryCollector::new(client.clone(), &collector.value);
    for _ in 0..5 {
        let events = collector.read_next_events(50).await?;
        match events {
            Some(events) => {
                for event in &events {
                    info!("{event_type}: {ts} - {id} - {msg}",
                        event_type=get_event_type_id(event),
                        id=event.key,
                        ts=event.created_time,
                        msg=event.full_formatted_message.as_deref().unwrap_or("No message"));
                }
                if !events.is_empty() {
                    continue; // dump events with no delay
                }
            },
            None => {
                info!("No events found")
            },
        }
        sleep(std::time::Duration::from_secs(5)).await;
    }
    collector.destroy_collector().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    info!("Starting up!");

    let vc_server = env::var("VIM_SERVER").with_context(||"VIM_SERVER env var not set")?;
    let username = env::var("VIM_USERNAME").with_context(||"VIM_USERNAME env var not set")?;
    let pwd = env::var("VIM_PASSWORD").with_context(||"VIM_PASSWORD env var not set")?;

    let vim_client = ClientBuilder::new(&vc_server)
        .insecure(true)
        .basic_authn(&username, &pwd)
        .app_details(APP_NAME, APP_VERSION)
        .build().await?;

    let Some(event_manager_moref) = vim_client.service_content().event_manager.clone() else {
        return Err(Error::msg("No event manager found"));
    };
    let event_manager = EventManager::new(vim_client.clone(), &event_manager_moref.value);

    dump_events(vim_client.clone(), &event_manager).await?;

    Ok(())
}
