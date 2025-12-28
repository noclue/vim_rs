//! # MAC Address Monitoring Example
//!
//! This example demonstrates how to monitor changes to virtual machine network configurations
//! in a vSphere environment, including MAC addresses and IP address assignments.
//!
//! The code uses the PropertyCollector interface to efficiently detect changes to network
//! properties across all virtual machines without constant polling. It employs an observer
//! pattern to propagate changes through a chain of listeners.
//!
//! In this example:
//! 1. We connect to a vSphere server using credentials from environment variables
//! 2. We create a container view to observe all virtual machines
//! 3. We establish a property filter to watch for specific network configuration changes
//! 4. We implement a caching mechanism to filter out duplicate updates
//! 5. We process detected changes through a listener chain
//!
//! The example demonstrates several design patterns useful in vSphere automation:
//! - Using PropertyCollector and WaitForUpdates to efficiently monitor changes
//! - Filtering detected changes to avoid redundant processing
//! - Handling polymorphic device types using traits
//! - Proper resource cleanup with Drop implementations

// Port of the Python mac_monitor example to Rust
// Original work: https://github.com/vmware/pyvmomi-community-samples/blob/master/samples/monitor_mac_addresses.py

use std::collections::HashMap;
use std::time::Instant;
use std::{env, sync::Arc};
use vim_rs::types::convert::CastInto;
use vim_rs::types::traits::VirtualEthernetCardTrait;

use anyhow::Result;
use log::{debug, info};
use utils::connect;
use vim_macros::vim_updatable;
use vim_rs::core::client::Client;
use vim_rs::core::pc_cache::{CacheAction, CacheManager, Monitor, ObjectCache, ObjectCacheListener};

vim_updatable!(
    struct VirtualMachine: VirtualMachine {
        name = "name",
        device = "config.hardware.device",
        guest_net = "guest.net",
    }
);

/// Physical Networks Address details of a VM. Contains vm name and a map
/// of device key to MAC address.
#[derive(Debug, Clone)]
struct VmDetails {
    /// Name of the VM
    vm_name: String,
    /// Map of device key to MAC address
    vnic: HashMap<i64, String>,
    /// Map of MAC address to IP addresses
    guest_net: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
struct VmChange {
    vm_name: Option<String>,
    vnic: Option<HashMap<i64, String>>,
    guest_net: Option<HashMap<String, Vec<String>>>,
}

/// Listens for changes in VM network details
trait VmChangeListener: Send {
    fn update_vm(&mut self, vm_id: String, vm_details: VmChange);
    fn remove_vm(&mut self, vm_id: String);
}

/// Prints the changes in VM network details
struct VmChangePrinter {}

impl VmChangeListener for VmChangePrinter {
    fn update_vm(&mut self, vm_id: String, vm_details: VmChange) {
        info!("VM {} updated: {:?}", vm_id, vm_details);
    }

    fn remove_vm(&mut self, vm_id: String) {
        info!("VM {} removed", vm_id);
    }
}

/// This cache removes spurious updates to MAC and IP addresses. It listens for changes in
/// the MAC addresses of VMs, updates the cache of VM Mac addresses when a real change
/// occurs and notifies a nested VmChangeListener. The cache is a map of VM id to
/// VmDetails.
struct VMMacCache {
    vm_cache: HashMap<String, VmDetails>,
    listener: Box<dyn VmChangeListener>,
}

impl VMMacCache {
    fn new(listener: Box<dyn VmChangeListener>) -> Self {
        VMMacCache {
            vm_cache: HashMap::new(),
            listener,
        }
    }
}

impl VmChangeListener for VMMacCache {
    fn update_vm(&mut self, vm_id: String, vm_details: VmChange) {
        let mut update = false;
        let (updated_vm_details, update) = if let Some(old_vm_details) = self.vm_cache.get(&vm_id) {
            let mut updated_vm_details = old_vm_details.clone();
            if let Some(vnic) = vm_details.vnic {
                if updated_vm_details.vnic != vnic {
                    updated_vm_details.vnic = vnic;
                    update = true;
                }
            }
            if let Some(vm_name) = vm_details.vm_name {
                if updated_vm_details.vm_name != vm_name {
                    updated_vm_details.vm_name = vm_name;
                    update = true;
                }
            }
            if let Some(guest_net) = vm_details.guest_net {
                if updated_vm_details.guest_net != guest_net {
                    updated_vm_details.guest_net = guest_net;
                    update = true;
                }
            }
            (updated_vm_details, update)
        } else {
            let new_vm_details = VmDetails {
                vm_name: vm_details.vm_name.unwrap_or("".to_string()),
                vnic: vm_details.vnic.unwrap_or_default(),
                guest_net: vm_details.guest_net.unwrap_or_default(),
            };
            (new_vm_details.clone(), true)
        };

        if update {
            self.vm_cache
                .insert(vm_id.clone(), updated_vm_details.clone());
            self.listener.update_vm(
                vm_id,
                VmChange {
                    vm_name: Some(updated_vm_details.vm_name),
                    vnic: Some(updated_vm_details.vnic),
                    guest_net: Some(updated_vm_details.guest_net),
                },
            );
        }
    }

    fn remove_vm(&mut self, vm_id: String) {
        self.vm_cache.remove(&vm_id);
        self.listener.remove_vm(vm_id);
    }
}

struct VmChangeDetector {
    monitor: Monitor,
    cache_manager: CacheManager,
}

impl VmChangeDetector {
    async fn new(listener: Box<dyn VmChangeListener>, client: Arc<Client>) -> Result<Self> {
        let mut cache_manager = CacheManager::new(client.clone())?;
        let monitor = cache_manager.create_monitor()?;

        let listener_adapter = VmChangeListenerAdapter::new(listener);
        let cache: ObjectCache<VirtualMachine> =
            ObjectCache::new_with_listener(Box::new(listener_adapter));
        cache_manager
            .add_container_cache(Box::new(cache), &client.service_content().root_folder)
            .await?;

        Ok(VmChangeDetector {
            monitor,
            cache_manager,
        })
    }

    /// Monitor for changes in VM network details.
    /// seconds: The interval in seconds to monitor for changes.
    async fn monitor(&mut self, seconds: u64) -> Result<()> {
        let start = Instant::now();

        loop {
            let res = self.monitor.wait_updates(10).await?;
            if let Some(res) = res {
                self.cache_manager.apply_updates(res)?;
            };
            let elapsed = start.elapsed().as_secs();
            debug!("Elapsed time: {}; set seconds {}", elapsed, seconds);
            if elapsed >= seconds {
                break;
            }
        }
        Ok(())
    }
}

struct VmChangeListenerAdapter {
    listener: Box<dyn VmChangeListener>,
}

impl VmChangeListenerAdapter {
    fn new(listener: Box<dyn VmChangeListener>) -> Self {
        VmChangeListenerAdapter { listener }
    }
}

impl ObjectCacheListener<VirtualMachine> for VmChangeListenerAdapter {
    fn on_new(&mut self, obj: &VirtualMachine) -> CacheAction {
        self.listener
            .update_vm(obj.id.value.clone(), vm_vm_change(obj));
        CacheAction::Keep
    }

    fn on_update(&mut self, obj: &VirtualMachine) -> CacheAction {
        self.listener
            .update_vm(obj.id.value.clone(), vm_vm_change(obj));
        CacheAction::Keep
    }

    fn on_remove(&mut self, obj: VirtualMachine) {
        self.listener.remove_vm(obj.id.value.clone());
    }
}

fn vm_vm_change(vm: &VirtualMachine) -> VmChange {
    let mut vnic_map: HashMap<i64, String> = HashMap::new();
    if let Some(ref vnic) = vm.device {
        for device in vnic {
            let key = device.get_key();
            let Some(eth): Option<&dyn VirtualEthernetCardTrait> = device.as_ref().into_ref()
            else {
                continue;
            };
            if let Some(mac) = eth.get_mac_address() {
                vnic_map.insert(key.into(), mac.clone());
            }
        }
    }
    let mut guest_net_map: HashMap<String, Vec<String>> = HashMap::new();
    if let Some(ref guest_net) = vm.guest_net {
        for net in guest_net {
            if let Some(mac_address) = net.mac_address.clone() {
                if let Some(ip) = net.ip_address.clone() {
                    guest_net_map.insert(mac_address.clone(), ip);
                }
            }
        }
    }
    VmChange {
        vm_name: Some(vm.name.clone()),
        vnic: Some(vnic_map),
        guest_net: Some(guest_net_map),
    }
}

impl Drop for VmChangeDetector {
    fn drop(&mut self) {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async move {
                let _ = self.cache_manager.destroy().await;
            })
        });
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;

    let listener = Box::new(VmChangePrinter {});
    let listener = Box::new(VMMacCache::new(listener));
    let mut detector = VmChangeDetector::new(listener, client).await?;
    detector.monitor(30).await?;

    Ok(())
}
