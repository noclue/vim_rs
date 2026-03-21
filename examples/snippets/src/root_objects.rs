//! # Multi-service root handles
//!
//! Uses [`RootObjects`](vim_rs::core::RootObjects) to obtain typed entry points for VIM and several auxiliary
//! vSphere services (EAM, PBM, VSLM, SMS) from a single authenticated client, then logs a small
//! slice of each service’s “about” metadata.
//!
//! ## Requirements
//!
//! - A vCenter endpoint where these services are available; some calls may fail or return empty
//!   results on minimal lab setups.
//!
//! ## Typical uses
//!
//! Smoke-testing connectivity across APIs and discovering service versions after login.

use log::info;
use snippets::connect;
use vim_rs::core::RootObjects;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;

    let root_objects = RootObjects::new(client.clone());

    info!(
        "VIM About vCenter version : {}",
        root_objects
            .vim_service_instance()
            .content()
            .await?
            .about
            .version
    );
    info!(
        "EAM agency list: {:?}",
        root_objects.esx_agent_manager().agency().await?
    );
    info!(
        "PBM About version: {}",
        root_objects
            .pbm_service_instance()
            .content()
            .await?
            .about_info
            .version
    );
    info!(
        "VSLM About API version: {}",
        root_objects
            .vslm_service_instance()
            .content()
            .await?
            .about_info
            .api_version
    );
    info!(
        "SMS About VASA version: {:?}",
        root_objects
            .sms_service_instance()
            .query_about_info()
            .await?
            .vasa_api_version
    );
    Ok(())
}
