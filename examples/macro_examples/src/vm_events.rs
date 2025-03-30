//! # VM Cache Events Example
//! This example demonstrates how to use the `vim_macros` library to efficiently replicate specific
//! properties from virtual machines in a VMware vSphere environment. It monitors for changes in VM
//! properties and acknowledges these on the console.

use log::info;
use std::fmt::{Display, Formatter};
use std::time::Instant;
use utils::connect;
use vim_macros::vim_updatable;
use vim_rs::core::pc_helpers::{CacheManager, ObjectCache, ObjectCacheListener};

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
    fn on_new(&mut self, obj: &VmDetails) {
        info!("New VM: {}", obj);
    }

    fn on_update(&mut self, obj: &VmDetails) {
        info!("VM updated: {}", obj);
    }

    fn on_remove(&mut self, obj: VmDetails) {
        info!("VM removed: {}", obj);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
