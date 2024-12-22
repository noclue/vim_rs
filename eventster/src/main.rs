use std::{env, sync::Arc};
use vim::{event_manager::EventManager, service_instance::ServiceInstance, types::{EventFilterSpecByTime, ServiceContent}, vim_client::VimClient};
use tokio;
use vim::session_manager::SessionManager;
use log::{debug, info};
use env_logger;
use chrono::{Utc, Duration};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("MethodFault: {0:?}")]
    MethodFault(Box<dyn vim::types::MethodFaultTrait>),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("VimClient error: {0}")]
    VimClientError(#[from] vim::vim_client::Error),
    #[error("Error: {0}")]
    Error(String),
}

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

// Dump the 10 latest events in vCenter
async fn dump_events(event_manager : &EventManager) -> Result<(), Error> {
    let thirty_minutes_ago = Utc::now() - Duration::minutes(30);
    

    let filter = &vim::types::EventFilterSpec {
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
    let events = event_manager.query_events(filter).await?;
    match events {
        Some(events) => {
            for event in events {
                debug!("Event: {ts} - {id} - {msg}",
                    id=event.get_key(),
                    ts=event.get_created_time(),
                    msg=event.get_full_formatted_message().as_ref().unwrap_or(&String::from("No message")));
            }
        },
        None => debug!("No events found"),
    }
    Ok(())
}

async fn create_client(vc_server: String, username: String, pwd: String) -> Result<(Arc<VimClient>, ServiceContent), Error> {
    let http_client = reqwest::ClientBuilder::new()
    .danger_accept_invalid_certs(true)
    .danger_accept_invalid_hostnames(true)
    .user_agent(APP_USER_AGENT)
    .build()?;

    let vim_client = vim::vim_client::VimClient::new(http_client, &vc_server, None);

    let service_instance = ServiceInstance::new(vim_client.clone(), "ServiceInstance");

    let content = service_instance.content().await?;
    debug!("ServiceInstance::content obtained from vCenter {}",
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

    let vc_server = env::var("VC_SERVER")?;
    let username = env::var("VC_USERNAME")?;
    let pwd = env::var("VC_PASSWORD")?;

    let (vim_client, content) = create_client(vc_server, username, pwd).await?;
    let event_manager = vim::event_manager::EventManager::new(vim_client.clone(), 
    &content.event_manager.ok_or(Error::Error("No event manager found".to_string()))?.value);

    dump_events(&event_manager).await?;

    Ok(())
}
