use std::{env, sync::Arc};
use tokio::time::sleep;
use vim::mo::{EventManager, ServiceInstance, SessionManager};
use vim::types::structs::{EventFilterSpecByTime, ServiceContent, ExtendedEvent, EventEx};
use vim::types::traits::EventTrait;
use vim::types::traits::MethodFaultTrait;
use vim::core::client::Client;
use tokio;
use log::info;
use env_logger;
use chrono::{Utc, Duration as ChronoDuration};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("MethodFault: {0:?}")]
    MethodFault(Box<dyn MethodFaultTrait>),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("VimClient error: {0}")]
    VimClientError(#[from] vim::core::client::Error),
    #[error("Error: {0}")]
    Error(String),
}

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Get the event type ID from an event
/// The semantics of how eventTypeId matching is done is as follows:
/// - If the event is of type EventEx return event_type_id member of the EventEx
/// - If the event is of type ExtendedEvent return event_id member of the ExtendedEvent
/// - Otherwise, return the type name of the Event itself.
fn get_event_type_id(event: &dyn EventTrait) -> String {
    let any_ref = event.as_any_ref();
    if let Some(event_ex) = any_ref.downcast_ref::<EventEx>() {
        return event_ex.event_type_id.clone();
    }
    if let Some(extended_event) = any_ref.downcast_ref::<ExtendedEvent>() {
        return extended_event.event_type_id.clone();
    }
    <&'static str>::from(event.data_type()).to_string()
}

// Dump the last 30 minutes of events in vCenter
async fn dump_events(client: Arc<Client>, event_manager: &EventManager) -> Result<(), Error> {
    let thirty_minutes_ago = Utc::now() - ChronoDuration::minutes(30);
    

    let filter = &vim::types::structs::EventFilterSpec {
        entity: None,
        time: Some(EventFilterSpecByTime {
            begin_time: Some(String::from(thirty_minutes_ago.to_rfc3339())),
            end_time: None,
        }),
        user_name: None,
        event_chain_id: None,
        alarm: None,
        scheduled_task: None,
        disable_full_message: Some(true),
        category: None,
        r#type: None,
        tag: None,
        event_type_id: None,
        max_count: None,
    };

    let collector = event_manager.create_collector_for_events(filter).await?;

    let collector = vim::mo::EventHistoryCollector::new(client.clone(), &collector.value);
    //let events = event_manager.query_events(filter).await?;
    for _ in 0..50 {
        let events = collector.read_next_events(10).await?;
        match events {
            Some(events) => {
                for event in events {
                    info!("{event_type}: {ts} - {id} - {msg}",
                        event_type=get_event_type_id(event.as_ref()),
                        id=event.get_key(),
                        ts=event.get_created_time(),
                        msg=event.get_full_formatted_message().as_ref().unwrap_or(&String::from("No message")));
                }
            },
            None => info!("No events found"),
        }
        sleep(std::time::Duration::from_secs(5)).await;
    }
    collector.destroy_collector().await?;
    Ok(())
}



async fn create_client(vc_server: String, username: String, pwd: String) -> Result<(Arc<Client>, ServiceContent), Error> {
    let http_client = reqwest::ClientBuilder::new()
    .danger_accept_invalid_certs(true)
    .danger_accept_invalid_hostnames(true)
    .build()?;

    let vim_client = Client::new(http_client, &vc_server, None);
    vim_client.set_app_details(APP_NAME, APP_VERSION).await;

    let service_instance = ServiceInstance::new(vim_client.clone(), "ServiceInstance");

    let content = service_instance.content().await?;
    info!("ServiceInstance::content obtained from vCenter {}",
            content.about.full_name);

    let Some(ref session_mgr_moref) = content.session_manager else {
        return Err(Error::Error("No session manager found".to_string()));
    };

    let sm = SessionManager::new(vim_client.clone(), &session_mgr_moref.value.clone());
    let us = sm.login(&username, &pwd, Some("en")).await?;

    info!("Session created for: {:?}", us.user_name);
    Ok((vim_client, content))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("Starting up!");

    let vc_server = env::var("VC_SERVER").map_err(|_| Error::Error(String::from("VC_SERVER env var not set")))?;
    let username = env::var("VC_USERNAME").map_err(|_| Error::Error(String::from("VC_USERNAME env var not set")))?;
    let pwd = env::var("VC_PASSWORD").map_err(|_| Error::Error(String::from("VC_PASSWORD env var not set")))?;

    let (vim_client, content) = create_client(vc_server, username, pwd).await?;
    let event_manager = EventManager::new(vim_client.clone(), 
    &content.event_manager.ok_or(Error::Error("No event manager found".to_string()))?.value);

    dump_events(vim_client.clone(), &event_manager).await?;

    Ok(())
}
