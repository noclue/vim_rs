// Port of the Python mac_monitor example to Rust
// Original work: https://github.com/vmware/pyvmomi-community-samples/blob/master/samples/monitor_mac_addresses.py

use std::collections::HashMap;
use std::time::Instant;
use std::{env, sync::Arc};
use vim::mo::{PropertyCollector, PropertyFilter, View, ViewManager};
use vim::types::vim_any::VimAny;
use vim::types::traits::VirtualEthernetCardTrait;
use vim::types::boxed_types::ValueElements;
use vim::types::convert::CastInto;
use vim::types::enums::{self, MoTypesEnum};
use vim::types::structs;

use vim::core::client::{Client, ClientBuilder};
use tokio;
use log::{debug, error, info, trace};
use env_logger;
use utils::{Result, Error};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

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
trait VmChangeListener {
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
                vnic: vm_details.vnic.unwrap_or(HashMap::new()),
                guest_net: vm_details.guest_net.unwrap_or(HashMap::new()),
            };
            (new_vm_details.clone(), true)
        };

        if update {
            self.vm_cache.insert(vm_id.clone(), updated_vm_details.clone());
            self.listener.update_vm(vm_id, VmChange {
                vm_name: Some(updated_vm_details.vm_name),
                vnic: Some(updated_vm_details.vnic),
                guest_net: Some(updated_vm_details.guest_net),
            });
        }
    }

    fn remove_vm(&mut self, vm_id: String) {
        self.vm_cache.remove(&vm_id);
        self.listener.remove_vm(vm_id);
    }
}

struct VmChangeDetector {
    listener: Box<dyn VmChangeListener>,
    client: Arc<Client>,
    property_collector: PropertyCollector,
    view: String,
    filter: String,
    version: String,
}

impl VmChangeDetector {
    async fn new(listener: Box<dyn VmChangeListener>, client: Arc<Client>) -> Result<Self> {
        let pc_id = client.service_content().property_collector.value.clone();
        // TODO : Create own property collector instance PropertyCollector::CreatePropertyCollector
        let property_collector = PropertyCollector::new(client.clone(), &pc_id);
        let Some(ref view_mgr_id) = client.service_content().view_manager else {
            return Err(Error::Error("No view manager found".to_string()));
        };
        let view_mgr_id = view_mgr_id.value.clone();
        let view_mgr = ViewManager::new(client.clone(), &view_mgr_id);
        let view_ref = view_mgr.create_container_view(&client.service_content().root_folder, Some(&vec!["VirtualMachine".to_string()]), true).await?;
        let spec = vim::types::structs::PropertyFilterSpec {
            object_set: vec![structs::ObjectSpec {
                obj: view_ref.clone(),
                skip: Some(false),
                select_set: Some(vec![Box::new(structs::TraversalSpec {
                    name: Some("traverseEntities".to_string()), 
                    r#type: Into::<&str>::into(MoTypesEnum::ContainerView).to_string(), 
                    path: "view".to_string(), 
                    skip: Some(false), 
                    select_set: None,
                 })]),
            }],
            prop_set: vec![structs::PropertySpec {
                all: Some(false),
                path_set: Some(vec![
                    "name".to_string(),
                    "config.hardware.device".to_string(),
                    "guest.net".to_string(),
                    ]),
                r#type: Into::<&str>::into(MoTypesEnum::VirtualMachine).to_string(),
            }],
            report_missing_objects_in_results: None,
        };
        let filter = property_collector.create_filter(&spec, false).await?;

        Ok(VmChangeDetector {
            listener,
            client: client.clone(),
            property_collector,
            view: view_ref.value,
            filter: filter.value,
            version: "".to_string(),
        })
    }

    /// Monitor for changes in VM network details.
    /// seconds: The interval in seconds to monitor for changes.
    async fn monitor(&mut self, seconds: u64) -> Result<()> {
        let wait_opts = structs::WaitOptions {
            max_wait_seconds: Some(10),
            max_object_updates: Some(100),
        };
        let start = Instant::now();
        
        loop {
            let res = self.property_collector.wait_for_updates_ex(Some(&self.version), Some(&wait_opts)).await?;
            if let Some(res) = res {
                self.version = res.version.clone();
                let Some(ref filter_set) = res.filter_set else {
                    debug!("No filter set received");
                    continue;
                };

                for update in filter_set {
                    if self.filter != update.filter.value {
                        trace!("Ignoring update for filter {}", update.filter.value);
                        continue;
                    }
                    let Some(ref object_set) = update.object_set else {
                        debug!("No object set received");
                        continue;
                    };
                    for change in object_set {
                        match change.kind {
                            enums::ObjectUpdateKindEnum::Enter => {
                                self.process_change(change);
                            }
                            enums::ObjectUpdateKindEnum::Modify => {
                                self.process_change(change);
                            }
                            enums::ObjectUpdateKindEnum::Leave => {
                                self.listener.remove_vm(change.obj.value.clone());
                            }
                            enums::ObjectUpdateKindEnum::Other_(ref other) => {
                                debug!("Unknown change kind {}", other);
                            }
                        }
                    }
                }
                let elapsed = start.elapsed().as_secs();
                if elapsed >= seconds {
                    break;
                }
            }
        }

        Ok(())
    }

fn process_change(&mut self, change: &structs::ObjectUpdate) {
        let Some(ref change_set) = change.change_set else {
            error!("No change set received on Enter of vm: {}", change.obj.value);
            return;
        };
        let vm_change = create_vm_change(change_set);
        self.listener.update_vm(change.obj.value.clone(), vm_change);
    }
}

fn create_vm_change(change_set: &Vec<structs::PropertyChange>) -> VmChange {
    let mut vm_change = VmChange {
        vm_name: None,
        vnic: None,
        guest_net: None,
    };
    for change in change_set {
        match change.name.as_str() {
            "name" => {
                let Some(ref val) = change.val else {
                    error!("No value received for name change");
                    continue;
                };
                let name: &str = match val {
                    VimAny::Value(ValueElements::PrimitiveString(s)) => &s,
                    _ => "Unexpected name type",
                };
                vm_change.vm_name = Some(name.to_string());
            }
            "config.hardware.device" => {
                // Vec<Box<dyn VirtualDeviceTrait>>
                let Some(ref devices) = change.val else {
                    error!("No value received for device change");
                    continue;
                };
                let devices  = match devices {
                    VimAny::Value(ValueElements::ArrayOfVirtualDevice(v)) => v,
                    _ => continue,
                };
                let mut vnic: HashMap<i64, String> = HashMap::new();
                for device in devices {
                    let key =device.get_key();
                    let Some(eth): Option<&dyn VirtualEthernetCardTrait> = device.as_ref().into_ref() else {
                        continue;
                    };
                    if let Some(ref mac) = eth.get_mac_address() {
                        vnic.insert(key.into(), mac.clone());
                    }
                }
                vm_change.vnic = Some(vnic);
            }
            "guest.net" => {
                // Vec<GuestNicInfo>
                let Some(ref nets) = change.val else {
                    error!("No value received for guest.net change");
                    continue;
                };
                let nets = match nets {
                    VimAny::Value(ValueElements::ArrayOfGuestNicInfo(v)) => v,
                    _ => continue,
                };

                let mut guest_net: HashMap<String, Vec<String>> = HashMap::new();
                for net in nets {
                    let Some(ref mac_address) = net.mac_address else {
                        error!("No MAC received for guest.net change");
                        continue;
                    };
                    if let Some(ip) = net.ip_address.clone() {
                        guest_net.insert(mac_address.clone().into(), ip);
                    }
                }
                vm_change.guest_net = Some(guest_net);
            }
            _ => {
                debug!("Ignoring change for property {}", change.name.clone());
            }
        }
    }
    vm_change
}


impl Drop for VmChangeDetector {
    fn drop(&mut self) {
        let view_id = self.view.clone();
        let filter_id = self.filter.clone();
        let client = self.client.clone();
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async move {
                let view = View::new(client.clone(), &self.view);
                if let Err(e) = view.destroy_view().await {
                    error!("Error destroying view {}: {:?}", view_id, e);
                };
                let filter = PropertyFilter::new(client.clone(), &filter_id);
                if let Err(e) = filter.destroy_property_filter().await {
                    error!("Error destroying property filter {}: {:?}", filter_id, e);
                };
            })
        });
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    info!("Starting up!");

    let vc_server = env::var("VC_SERVER").map_err(|_| Error::Error(String::from("VC_SERVER env var not set")))?;
    let username = env::var("VC_USERNAME").map_err(|_| Error::Error(String::from("VC_USERNAME env var not set")))?;
    let pwd = env::var("VC_PASSWORD").map_err(|_| Error::Error(String::from("VC_PASSWORD env var not set")))?;

    let vim_client = ClientBuilder::new(&vc_server)
        .insecure(true)
        .basic_authn(&username, &pwd)
        .app_details(APP_NAME, APP_VERSION)
        .build().await?;

    let listener = Box::new(VmChangePrinter {});
    let listener = Box::new(VMMacCache::new(listener));
    let mut detector = VmChangeDetector::new(listener, vim_client).await?;
    detector.monitor(30).await?;

    Ok(())
}