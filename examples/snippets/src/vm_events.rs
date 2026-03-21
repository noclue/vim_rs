//! # VM inventory changes via property cache (`vim_updatable!`)
//!
//! Subscribes to virtual machine create/update/remove under the root folder using
//! [`CacheManager`](vim_rs::core::pc_cache::CacheManager), [`ObjectCache`](vim_rs::core::pc_cache::ObjectCache), and
//! [`vim_updatable!`](vim_rs::vim_updatable) for typed VM name and power state. The example runs for 60 seconds,
//! printing each batch of updates from [`Monitor::wait_updates`](vim_rs::core::pc_cache::Monitor::wait_updates).
//!
//! ## Typical uses
//!
//! Live inventory mirrors, audit trails, and automation triggers driven by VM lifecycle changes.

use log::info;
use snippets::connect;
use std::fmt::{Display, Formatter};
use std::time::Instant;
use vim_rs::core::pc_cache::{CacheAction, CacheManager, ObjectCache, ObjectCacheListener};
use vim_rs::vim_updatable;

vim_updatable!(
    struct VmDetails: VirtualMachine {
        name = "name",
        power_state = "runtime.power_state",
    }
);

impl Display for VmDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VM ({}): {} with power state: {:?}",
            self.id.value, self.name, self.power_state
        )
    }
}

struct ChangeListener {}

impl ObjectCacheListener<VmDetails> for ChangeListener {
    fn on_new(&mut self, obj: &VmDetails) -> CacheAction {
        info!("New VM: {}", obj);
        CacheAction::Keep
    }

    fn on_update(&mut self, obj: &VmDetails) -> CacheAction {
        info!("VM updated: {}", obj);
        CacheAction::Keep
    }

    fn on_remove(&mut self, obj: VmDetails) {
        info!("VM removed: {}", obj);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;

    let cache = ObjectCache::new_with_listener(Box::new(ChangeListener {}));
    let mut manager = CacheManager::new(client.clone())?;
    let mut monitor = manager.create_monitor()?;

    manager
        .add_container_cache(Box::new(cache), &client.service_content().root_folder)
        .await?;

    let start = Instant::now();
    loop {
        let updates = monitor.wait_updates(10).await?;
        if let Some(updates) = updates {
            info!("Received {} updates", updates.len());
            manager.apply_updates(updates)?;
        } else {
            info!("No updates received");
        }
        if start.elapsed().as_secs() > 60 {
            break;
        }
    }

    manager.destroy().await?;
    Ok(())
}
