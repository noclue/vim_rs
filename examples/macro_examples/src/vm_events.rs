//! # VM Cache Events Example
//!
//! This example demonstrates how to use the vSphere PropertyCollector cache system to
//! efficiently monitor changes to virtual machine properties in real-time.
//!
//! ## Key Concepts
//!
//! **PropertyCollector Cache**: Unlike the `vim_retrievable!` macro which fetches properties
//! once, the cache system uses vSphere's WaitForUpdatesEx to continuously monitor objects
//! and receive incremental updates when properties change.
//!
//! **vim_updatable! macro**: Similar to `vim_retrievable!`, but generates code optimized
//! for the cache system, supporting create/update/delete operations.
//!
//! **ObjectCacheListener**: A trait you implement to receive callbacks when objects are:
//! - Created (`on_new`)
//! - Modified (`on_update`)
//! - Deleted (`on_remove`)
//!
//! ## How It Works
//!
//! 1. Define a struct with `vim_updatable!` specifying which VM properties to track
//! 2. Implement `ObjectCacheListener` to handle property change events
//! 3. Create a `CacheManager` and `ObjectCache` with your listener
//! 4. Add the root folder (or any container) to watch all VMs within it
//! 5. Call `wait_updates()` in a loop to receive and process changes
//!
//! ## Use Cases
//!
//! - Real-time monitoring dashboards
//! - Audit logging of VM changes
//! - Triggering automation based on state changes
//! - Maintaining a synchronized replica of vSphere inventory
//!
//! ## Performance Benefits
//!
//! The cache system is highly efficient because:
//! - Only changed properties are transmitted (not full objects)
//! - A single PropertyCollector filter watches all VMs
//! - The server tracks what data the client already has
//! - Network traffic is minimized through incremental updates
//!
//! In this example:
//! 1. We connect to vSphere and create a cache manager
//! 2. We track VM name and power state changes
//! 3. We print events to the console as VMs are created, updated, or removed
//! 4. The example runs for 60 seconds then exits cleanly

use log::info;
use std::fmt::{Display, Formatter};
use std::time::Instant;
use utils::connect;
use vim_macros::vim_updatable;
use vim_rs::core::pc_cache::{CacheManager, ObjectCache, ObjectCacheListener};

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
