use log::info;
use vim_rs::core::RootObjects;
use utils::connect;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;
    
    let root_objects = RootObjects::new(client.clone());

    info!("VIM About vCenter version : {}", root_objects.vim_service_instance().content().await?.about.version);
    info!("EAM agency list: {:?}", root_objects.esx_agent_manager().agency().await?);
    info!("PBM About version: {}", root_objects.pbm_service_instance().content().await?.about_info.version);
    info!("VSLM About API version: {}", root_objects.vslm_service_instance().content().await?.about_info.api_version);
    info!("SMS About VASA version: {:?}", root_objects.sms_service_instance().query_about_info().await?.vasa_api_version);
    Ok(())
}
